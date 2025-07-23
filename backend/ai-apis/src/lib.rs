use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, error};
use uuid::Uuid;

pub mod anthropic;
pub mod grok;
pub mod monday;

/// AI API Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub id: Uuid,
    pub provider: ApiProvider,
    pub name: String,
    pub api_key: String,
    pub base_url: Option<String>,
    pub rate_limit: RateLimitConfig,
    pub enabled: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiProvider {
    Anthropic,
    Grok3,
    Monday,
    Custom { name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub tokens_per_minute: Option<u32>,
}

/// AI API Manager
pub struct AiApiManager {
    configs: tokio::sync::RwLock<HashMap<Uuid, ApiConfig>>,
    anthropic_client: anthropic::AnthropicClient,
    grok_client: grok::GrokClient,
    monday_client: monday::MondayClient,
}

impl AiApiManager {
    pub fn new() -> Self {
        Self {
            configs: tokio::sync::RwLock::new(HashMap::new()),
            anthropic_client: anthropic::AnthropicClient::new(),
            grok_client: grok::GrokClient::new(),
            monday_client: monday::MondayClient::new(),
        }
    }

    pub async fn register_api(&self, mut config: ApiConfig) -> Result<Uuid> {
        config.id = Uuid::new_v4();
        config.created_at = chrono::Utc::now();

        let api_id = config.id;

        // Initialize client based on provider
        match config.provider {
            ApiProvider::Anthropic => {
                self.anthropic_client.initialize(&config).await?;
            }
            ApiProvider::Grok3 => {
                self.grok_client.initialize(&config).await?;
            }
            ApiProvider::Monday => {
                self.monday_client.initialize(&config).await?;
            }
            _ => {}
        }

        {
            let mut configs = self.configs.write().await;
            configs.insert(api_id, config);
        }

        info!("Registered AI API: {}", api_id);
        Ok(api_id)
    }

    pub async fn execute_request(&self, api_id: Uuid, request: ApiRequest) -> Result<ApiResponse> {
        let config = {
            let configs = self.configs.read().await;
            configs.get(&api_id).cloned()
                .ok_or_else(|| anyhow::anyhow!("API not found: {}", api_id))?
        };

        if !config.enabled {
            return Err(anyhow::anyhow!("API is disabled: {}", api_id));
        }

        match config.provider {
            ApiProvider::Anthropic => {
                self.anthropic_client.execute_request(&config, request).await
            }
            ApiProvider::Grok3 => {
                self.grok_client.execute_request(&config, request).await
            }
            ApiProvider::Monday => {
                self.monday_client.execute_request(&config, request).await
            }
            _ => {
                Err(anyhow::anyhow!("Provider not supported: {:?}", config.provider))
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiRequest {
    ChatCompletion {
        messages: Vec<ChatMessage>,
        model: String,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    },
    TextCompletion {
        prompt: String,
        model: String,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    },
    Embedding {
        text: String,
        model: String,
    },
    Custom {
        endpoint: String,
        method: String,
        headers: HashMap<String, String>,
        body: serde_json::Value,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub data: serde_json::Value,
    pub usage: Option<UsageInfo>,
    pub error: Option<String>,
    pub latency_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageInfo {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
} 