use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: Option<String>,
    pub redis_url: Option<String>,
    pub jwt_secret: String,
    pub cors_origins: Vec<String>,
    pub rate_limit: RateLimitConfig,
    pub auth: AuthConfig,
    pub observability: ObservabilityConfig,
    pub services: ServicesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u64,
    pub burst_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub session_timeout_hours: u64,
    pub max_sessions_per_user: u32,
    pub password_min_length: u32,
    pub require_mfa: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    pub jaeger_endpoint: Option<String>,
    pub metrics_enabled: bool,
    pub log_level: String,
    pub structured_logging: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicesConfig {
    pub anthropic_api_url: String,
    pub openai_api_url: String,
    pub grok_api_url: String,
    pub monday_api_url: String,
    pub vault_addr: Option<String>,
    pub vault_role: String,
}

impl Config {
    /// Load configuration from environment variables and config files
    pub fn load() -> Result<Self> {
        dotenvy::dotenv().ok(); // Load .env file if present

        let config = Config {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            
            database_url: env::var("DATABASE_URL").ok(),
            redis_url: env::var("REDIS_URL").ok(),
            
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-in-production".to_string()),
            
            cors_origins: env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000,http://localhost:3001".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            
            rate_limit: RateLimitConfig {
                requests_per_minute: env::var("RATE_LIMIT_RPM")
                    .unwrap_or_else(|_| "100".to_string())
                    .parse()
                    .unwrap_or(100),
                burst_size: env::var("RATE_LIMIT_BURST")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
            },
            
            auth: AuthConfig {
                session_timeout_hours: env::var("SESSION_TIMEOUT_HOURS")
                    .unwrap_or_else(|_| "24".to_string())
                    .parse()
                    .unwrap_or(24),
                max_sessions_per_user: env::var("MAX_SESSIONS_PER_USER")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .unwrap_or(5),
                password_min_length: env::var("PASSWORD_MIN_LENGTH")
                    .unwrap_or_else(|_| "12".to_string())
                    .parse()
                    .unwrap_or(12),
                require_mfa: env::var("REQUIRE_MFA")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .unwrap_or(false),
            },
            
            observability: ObservabilityConfig {
                jaeger_endpoint: env::var("JAEGER_ENDPOINT").ok(),
                metrics_enabled: env::var("METRICS_ENABLED")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                log_level: env::var("RUST_LOG")
                    .unwrap_or_else(|_| "info".to_string()),
                structured_logging: env::var("STRUCTURED_LOGGING")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
            },
            
            services: ServicesConfig {
                anthropic_api_url: env::var("ANTHROPIC_API_URL")
                    .unwrap_or_else(|_| "https://api.anthropic.com".to_string()),
                openai_api_url: env::var("OPENAI_API_URL")
                    .unwrap_or_else(|_| "https://api.openai.com".to_string()),
                grok_api_url: env::var("GROK_API_URL")
                    .unwrap_or_else(|_| "https://api.x.ai".to_string()),
                monday_api_url: env::var("MONDAY_API_URL")
                    .unwrap_or_else(|_| "https://api.monday.com".to_string()),
                vault_addr: env::var("VAULT_ADDR").ok(),
                vault_role: env::var("VAULT_ROLE")
                    .unwrap_or_else(|_| "talk-plus-plus".to_string()),
            },
        };

        // Validate required configuration
        config.validate()?;

        Ok(config)
    }

    /// Validate configuration
    fn validate(&self) -> Result<()> {
        if self.database_url.is_none() && env::var("APP_ENV").unwrap_or_default() != "test" {
            return Err(anyhow::anyhow!("DATABASE_URL is required"));
        }

        if self.redis_url.is_none() && env::var("APP_ENV").unwrap_or_default() != "test" {
            return Err(anyhow::anyhow!("REDIS_URL is required"));
        }

        if self.jwt_secret == "dev-secret-change-in-production" 
            && env::var("APP_ENV").unwrap_or_default() == "production" {
            return Err(anyhow::anyhow!("JWT_SECRET must be set in production"));
        }

        Ok(())
    }

    /// Get configuration as environment-specific values
    pub fn for_environment(&self, env: &str) -> Self {
        let mut config = self.clone();
        
        match env {
            "development" => {
                config.observability.log_level = "debug".to_string();
                config.rate_limit.requests_per_minute = 1000; // More lenient for dev
            },
            "production" => {
                config.observability.log_level = "info".to_string();
                config.auth.require_mfa = true; // Enforce MFA in prod
            },
            "test" => {
                config.observability.log_level = "warn".to_string();
                config.database_url = Some("postgres://test:test@localhost/test".to_string());
                config.redis_url = Some("redis://localhost:6379/1".to_string());
            },
            _ => {}
        }
        
        config
    }
} 