use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, error, warn};
use uuid::Uuid;

pub mod google;
pub mod microsoft;
pub mod email;
pub mod calendar;
pub mod storage;

/// External Service Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub id: Uuid,
    pub service_type: ServiceType,
    pub name: String,
    pub enabled: bool,
    pub credentials: ServiceCredentials,
    pub settings: HashMap<String, serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    GoogleDrive,
    GoogleCalendar,
    GoogleContacts,
    Gmail,
    OneDrive,
    OutlookCalendar,
    OutlookContacts,
    Exchange,
    Imap,
    Pop3,
    Smtp,
    Dropbox,
    AwsS3,
    AzureBlob,
    Monday,
    Custom { provider: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceCredentials {
    OAuth2 {
        client_id: String,
        client_secret: String,
        access_token: String,
        refresh_token: String,
        expires_at: chrono::DateTime<chrono::Utc>,
    },
    ApiKey {
        key: String,
        secret: Option<String>,
    },
    BasicAuth {
        username: String,
        password: String,
    },
    Certificate {
        cert_path: String,
        key_path: String,
        password: Option<String>,
    },
}

/// External Services Manager
pub struct ExternalServicesManager {
    services: tokio::sync::RwLock<HashMap<Uuid, ServiceConfig>>,
    google_service: google::GoogleService,
    microsoft_service: microsoft::MicrosoftService,
    email_service: email::EmailService,
    calendar_service: calendar::CalendarService,
    storage_service: storage::StorageService,
}

impl ExternalServicesManager {
    pub fn new() -> Self {
        Self {
            services: tokio::sync::RwLock::new(HashMap::new()),
            google_service: google::GoogleService::new(),
            microsoft_service: microsoft::MicrosoftService::new(),
            email_service: email::EmailService::new(),
            calendar_service: calendar::CalendarService::new(),
            storage_service: storage::StorageService::new(),
        }
    }

    /// Register a new external service
    pub async fn register_service(&self, mut config: ServiceConfig) -> Result<Uuid> {
        config.id = Uuid::new_v4();
        config.created_at = chrono::Utc::now();
        config.updated_at = chrono::Utc::now();

        let service_id = config.id;
        
        // Initialize the service based on type
        match config.service_type {
            ServiceType::GoogleDrive | ServiceType::GoogleCalendar | 
            ServiceType::GoogleContacts | ServiceType::Gmail => {
                self.google_service.register_service(&config).await?;
            }
            ServiceType::OneDrive | ServiceType::OutlookCalendar | 
            ServiceType::OutlookContacts | ServiceType::Exchange => {
                self.microsoft_service.register_service(&config).await?;
            }
            ServiceType::Imap | ServiceType::Pop3 | ServiceType::Smtp => {
                self.email_service.register_service(&config).await?;
            }
            _ => {
                info!("Service type {:?} registered without specific initialization", config.service_type);
            }
        }

        {
            let mut services = self.services.write().await;
            services.insert(service_id, config);
        }

        info!("Registered external service: {}", service_id);
        Ok(service_id)
    }

    /// List all registered services
    pub async fn list_services(&self) -> Result<Vec<ServiceConfig>> {
        let services = self.services.read().await;
        Ok(services.values().cloned().collect())
    }

    /// Execute service operation
    pub async fn execute_operation(&self, service_id: Uuid, operation: ServiceOperation) -> Result<ServiceResult> {
        let config = {
            let services = self.services.read().await;
            services.get(&service_id).cloned()
                .ok_or_else(|| anyhow::anyhow!("Service not found: {}", service_id))?
        };

        if !config.enabled {
            return Err(anyhow::anyhow!("Service is disabled: {}", service_id));
        }

        match config.service_type {
            ServiceType::GoogleDrive => {
                self.google_service.execute_drive_operation(&config, operation).await
            }
            ServiceType::GoogleCalendar => {
                self.google_service.execute_calendar_operation(&config, operation).await  
            }
            ServiceType::Gmail => {
                self.google_service.execute_gmail_operation(&config, operation).await
            }
            ServiceType::OneDrive => {
                self.microsoft_service.execute_onedrive_operation(&config, operation).await
            }
            ServiceType::OutlookCalendar => {
                self.microsoft_service.execute_calendar_operation(&config, operation).await
            }
            ServiceType::Imap | ServiceType::Pop3 | ServiceType::Smtp => {
                self.email_service.execute_operation(&config, operation).await
            }
            _ => {
                Err(anyhow::anyhow!("Operation not supported for service type: {:?}", config.service_type))
            }
        }
    }

    /// Sync all enabled services
    pub async fn sync_all_services(&self) -> Result<Vec<SyncResult>> {
        let services = {
            let services = self.services.read().await;
            services.values().filter(|s| s.enabled).cloned().collect::<Vec<_>>()
        };

        let mut results = Vec::new();
        
        for service in services {
            match self.sync_service(service.id).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    error!("Failed to sync service {}: {}", service.id, e);
                    results.push(SyncResult {
                        service_id: service.id,
                        service_name: service.name,
                        success: false,
                        synced_items: 0,
                        errors: vec![e.to_string()],
                        duration_ms: 0,
                        last_sync: chrono::Utc::now(),
                    });
                }
            }
        }

        Ok(results)
    }

    async fn sync_service(&self, service_id: Uuid) -> Result<SyncResult> {
        let start_time = std::time::Instant::now();
        
        let config = {
            let services = self.services.read().await;
            services.get(&service_id).cloned()
                .ok_or_else(|| anyhow::anyhow!("Service not found: {}", service_id))?
        };

        info!("Syncing service: {} ({})", config.name, service_id);

        // Execute sync based on service type
        let sync_operation = ServiceOperation::Sync {
            full_sync: false,
            since: None,
        };

        let result = self.execute_operation(service_id, sync_operation).await?;
        
        let duration = start_time.elapsed().as_millis() as u64;

        Ok(SyncResult {
            service_id,
            service_name: config.name,
            success: result.success,
            synced_items: result.data.get("synced_count")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize,
            errors: if result.success { Vec::new() } else { vec![result.error.unwrap_or_default()] },
            duration_ms: duration,
            last_sync: chrono::Utc::now(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceOperation {
    List {
        resource_type: String,
        limit: Option<usize>,
        filters: HashMap<String, String>,
    },
    Get {
        resource_type: String,
        resource_id: String,
    },
    Create {
        resource_type: String,
        data: serde_json::Value,
    },
    Update {
        resource_type: String,
        resource_id: String,
        data: serde_json::Value,
    },
    Delete {
        resource_type: String,
        resource_id: String,
    },
    Search {
        resource_type: String,
        query: String,
        limit: Option<usize>,
    },
    Sync {
        full_sync: bool,
        since: Option<chrono::DateTime<chrono::Utc>>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceResult {
    pub success: bool,
    pub data: serde_json::Value,
    pub error: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResult {
    pub service_id: Uuid,
    pub service_name: String,
    pub success: bool,
    pub synced_items: usize,
    pub errors: Vec<String>,
    pub duration_ms: u64,
    pub last_sync: chrono::DateTime<chrono::Utc>,
} 