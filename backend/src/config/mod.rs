pub mod secrets;

use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub environment: String,
    pub request_timeout: u64,
    /// Whether to trust X-Forwarded-For/X-Real-IP headers for IP extraction
    /// Should only be true when behind a trusted reverse proxy/load balancer
    pub trust_proxy: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

impl Config {
    /// Load configuration with smart defaults for development
    ///
    /// In development (debug builds), this will:
    /// 1. Try to load from environment variables
    /// 2. Fall back to dev defaults if required vars are missing
    ///
    /// In production (release builds), this will:
    /// 1. Require all environment variables (fail if missing)
    pub fn load() -> Result<Self, config::ConfigError> {
        match Self::from_env() {
            Ok(config) => Ok(config),
            Err(e) => {
                #[cfg(debug_assertions)]
                {
                    // Provide detailed feedback about what's missing
                    let mut missing = Vec::new();
                    if std::env::var("DATABASE_URL").is_err() {
                        missing.push("DATABASE_URL");
                    }
                    if std::env::var("JWT_SECRET").is_err() {
                        missing.push("JWT_SECRET");
                    }

                    if !missing.is_empty() {
                        tracing::warn!(
                            "⚠️  Using dev defaults. Missing env vars: {}",
                            missing.join(", ")
                        );
                    } else {
                        tracing::warn!("Config loading failed: {}. Using dev defaults", e);
                    }

                    Ok(Self::dev())
                }
                #[cfg(not(debug_assertions))]
                {
                    return Err(e);
                }
            }
        }
    }

    /// Helper to parse environment variables with proper error handling
    fn env_or<T: std::str::FromStr>(key: &str, default: T) -> Result<T, config::ConfigError>
    where
        T::Err: std::fmt::Display,
    {
        match env::var(key) {
            Ok(val) => val.parse::<T>().map_err(|e| {
                config::ConfigError::Message(format!(
                    "Invalid value for {}: '{}' - {}",
                    key, val, e
                ))
            }),
            Err(_) => Ok(default),
        }
    }

    /// Helper to get required environment variable
    fn env_required(key: &str) -> Result<String, config::ConfigError> {
        env::var(key).map_err(|_| {
            config::ConfigError::NotFound(format!("{} must be set", key))
        })
    }

    pub fn from_env() -> Result<Self, config::ConfigError> {
        // Build config from environment variables directly
        let server = ServerConfig {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: Self::env_or("PORT", 8080)?,
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            request_timeout: Self::env_or("REQUEST_TIMEOUT", 30)?,
            trust_proxy: Self::env_or("TRUST_PROXY", false)?,
        };

        let database = DatabaseConfig {
            url: Self::env_required("DATABASE_URL")?,
            pool_size: Self::env_or("DATABASE_POOL_SIZE", 10)?,
        };

        let jwt = JwtConfig {
            secret: Self::env_required("JWT_SECRET")?,
            expiration_hours: Self::env_or("JWT_EXPIRATION_HOURS", 24)?,
        };

        let cors = CorsConfig {
            allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
        };

        Ok(Config {
            server,
            database,
            jwt,
            cors,
        })
    }

    /// Load configuration with secrets from a secret manager
    ///
    /// Supports loading sensitive configuration values from external secret providers
    /// instead of environment variables. Use SECRET_PROVIDER env var to choose provider:
    /// - "env" (default): Load from environment variables
    /// - "aws": Use AWS Secrets Manager (requires aws-secrets feature)
    /// - "vault": Use HashiCorp Vault (requires vault-secrets feature)
    pub async fn from_secrets() -> Result<Self, Box<dyn std::error::Error>> {
        

        // Initialize secret manager
        let secret_manager = secrets::SecretManager::from_config().await?;

        // Non-sensitive config from env vars
        let server = ServerConfig {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            request_timeout: env::var("REQUEST_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            trust_proxy: env::var("TRUST_PROXY")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        };

        // Sensitive config from secret manager with env var fallback
        let database_url = secret_manager
            .get_secret_or_env("DATABASE_URL", None)
            .await
            .map_err(|e| -> Box<dyn std::error::Error> {
                Box::new(config::ConfigError::NotFound(e.to_string()))
            })?;

        let jwt_secret = secret_manager
            .get_secret_or_env("JWT_SECRET", None)
            .await
            .map_err(|e| -> Box<dyn std::error::Error> {
                Box::new(config::ConfigError::NotFound(e.to_string()))
            })?;

        let database = DatabaseConfig {
            url: database_url,
            pool_size: env::var("DATABASE_POOL_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
        };

        let jwt = JwtConfig {
            secret: jwt_secret,
            expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),
        };

        let cors = CorsConfig {
            allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        };

        Ok(Config {
            server,
            database,
            jwt,
            cors,
        })
    }

    pub fn is_production(&self) -> bool {
        self.server.environment == "production"
    }

    /// Quick development config with sensible defaults
    /// Perfect for getting started without setting up .env
    ///
    /// WARNING: Never use this in production! This provides insecure defaults.
    pub fn dev() -> Self {
        Config {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                environment: "development".to_string(),
                request_timeout: 30,
                trust_proxy: false,
            },
            database: DatabaseConfig {
                url: "postgres://postgres:postgres@localhost:5432/backend_dev".to_string(),
                pool_size: 5,
            },
            jwt: JwtConfig {
                secret: "dev-secret-not-for-production".to_string(),
                expiration_hours: 24,
            },
            cors: CorsConfig {
                allowed_origins: vec![
                    "http://localhost:3000".to_string(),
                    "http://localhost:8080".to_string(),
                ],
            },
        }
    }

    /// Create a default test configuration
    /// Available for integration tests
    pub fn default_test_config() -> Self {
        // Try to read database URL from environment, fallback to default
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5431/backend_db".to_string());

        Config {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 0, // Random port
                environment: "test".to_string(),
                request_timeout: 10,
                trust_proxy: false,
            },
            database: DatabaseConfig {
                url: database_url,
                pool_size: 2,
            },
            jwt: JwtConfig {
                secret: "test-secret-key-for-testing-only".to_string(),
                expiration_hours: 1,
            },
            cors: CorsConfig {
                allowed_origins: vec!["http://localhost:3000".to_string()],
            },
        }
    }
}
