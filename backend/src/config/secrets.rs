use crate::error::AppError;
use std::env;

/// Secret provider trait for abstracting secret retrieval
#[async_trait::async_trait]
pub trait SecretProvider: Send + Sync {
    async fn get_secret(&self, key: &str) -> Result<String, AppError>;
}

/// Environment variable secret provider (default)
pub struct EnvSecretProvider;

#[async_trait::async_trait]
impl SecretProvider for EnvSecretProvider {
    async fn get_secret(&self, key: &str) -> Result<String, AppError> {
        env::var(key).map_err(|_| AppError::ConfigError(format!("Secret '{}' not found in environment", key)))
    }
}

/// AWS Secrets Manager provider
#[cfg(feature = "aws-secrets")]
pub struct AwsSecretsProvider {
    client: aws_sdk_secretsmanager::Client,
    prefix: Option<String>,
}

#[cfg(feature = "aws-secrets")]
impl AwsSecretsProvider {
    pub async fn new(prefix: Option<String>) -> Result<Self, AppError> {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let client = aws_sdk_secretsmanager::Client::new(&config);
        Ok(Self { client, prefix })
    }

    fn with_prefix(&self, key: &str) -> String {
        match &self.prefix {
            Some(prefix) => format!("{}/{}", prefix, key),
            None => key.to_string(),
        }
    }
}

#[cfg(feature = "aws-secrets")]
#[async_trait::async_trait]
impl SecretProvider for AwsSecretsProvider {
    async fn get_secret(&self, key: &str) -> Result<String, AppError> {
        let secret_id = self.with_prefix(key);

        let result = self
            .client
            .get_secret_value()
            .secret_id(&secret_id)
            .send()
            .await
            .map_err(|e| AppError::external_service("AWS Secrets Manager", e))?;

        result
            .secret_string()
            .map(|s| s.to_string())
            .ok_or_else(|| AppError::ConfigError(format!("Secret '{}' has no string value", secret_id)))
    }
}

/// HashiCorp Vault provider
#[cfg(feature = "vault-secrets")]
pub struct VaultSecretProvider {
    client: vaultrs::client::VaultClient,
    mount: String,
    path: String,
}

#[cfg(feature = "vault-secrets")]
impl VaultSecretProvider {
    pub fn new(addr: String, token: String, mount: String, path: String) -> Result<Self, AppError> {
        let client = vaultrs::client::VaultClient::new(
            vaultrs::client::VaultClientSettingsBuilder::default()
                .address(addr)
                .token(token)
                .build()
                .map_err(|e| AppError::ConfigError(format!("Failed to create Vault client: {}", e)))?,
        )
        .map_err(|e| AppError::external_service("Vault", e))?;

        Ok(Self { client, mount, path })
    }
}

#[cfg(feature = "vault-secrets")]
#[async_trait::async_trait]
impl SecretProvider for VaultSecretProvider {
    async fn get_secret(&self, key: &str) -> Result<String, AppError> {
        use vaultrs::kv2;

        let secret_path = format!("{}/{}", self.path, key);
        let secret: std::collections::HashMap<String, String> = kv2::read(&self.client, &self.mount, &secret_path)
            .await
            .map_err(|e| AppError::external_service("Vault", e))?;

        secret
            .get("value")
            .cloned()
            .or_else(|| secret.get(key).cloned())
            .ok_or_else(|| AppError::ConfigError(format!("Secret '{}' not found in Vault", key)))
    }
}

/// Secret manager that can use different providers
pub struct SecretManager {
    provider: Box<dyn SecretProvider>,
}

impl SecretManager {
    /// Create a new secret manager with environment variable provider (default)
    pub fn new_env() -> Self {
        Self {
            provider: Box::new(EnvSecretProvider),
        }
    }

    /// Create a new secret manager with custom provider
    pub fn new(provider: Box<dyn SecretProvider>) -> Self {
        Self { provider }
    }

    /// Initialize secret manager based on configuration
    pub async fn from_config() -> Result<Self, AppError> {
        // Check which secret provider to use based on env vars
        let provider_type = env::var("SECRET_PROVIDER").unwrap_or_else(|_| "env".to_string());

        match provider_type.as_str() {
            "env" => Ok(Self::new_env()),

            #[cfg(feature = "aws-secrets")]
            "aws" => {
                let prefix = env::var("AWS_SECRET_PREFIX").ok();
                let provider = AwsSecretsProvider::new(prefix).await?;
                Ok(Self::new(Box::new(provider)))
            }

            #[cfg(feature = "vault-secrets")]
            "vault" => {
                let addr = env::var("VAULT_ADDR")
                    .map_err(|_| AppError::ConfigError("VAULT_ADDR must be set".to_string()))?;
                let token = env::var("VAULT_TOKEN")
                    .map_err(|_| AppError::ConfigError("VAULT_TOKEN must be set".to_string()))?;
                let mount = env::var("VAULT_MOUNT").unwrap_or_else(|_| "secret".to_string());
                let path = env::var("VAULT_PATH").unwrap_or_else(|_| "backend".to_string());

                let provider = VaultSecretProvider::new(addr, token, mount, path)?;
                Ok(Self::new(Box::new(provider)))
            }

            _ => Err(AppError::ConfigError(format!(
                "Unknown secret provider: {}. Available: env{}{}",
                provider_type,
                if cfg!(feature = "aws-secrets") { ", aws" } else { "" },
                if cfg!(feature = "vault-secrets") { ", vault" } else { "" }
            ))),
        }
    }

    /// Get a secret by key
    pub async fn get_secret(&self, key: &str) -> Result<String, AppError> {
        self.provider.get_secret(key).await
    }

    /// Get a secret with a fallback to environment variable
    pub async fn get_secret_or_env(&self, key: &str, env_key: Option<&str>) -> Result<String, AppError> {
        // Try secret manager first
        match self.provider.get_secret(key).await {
            Ok(value) => Ok(value),
            Err(_) => {
                // Fallback to environment variable
                let env_var = env_key.unwrap_or(key);
                env::var(env_var).map_err(|_| {
                    AppError::ConfigError(format!(
                        "Secret '{}' not found in secret provider or environment variable '{}'",
                        key, env_var
                    ))
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_env_secret_provider() {
        env::set_var("TEST_SECRET", "test_value");

        let provider = EnvSecretProvider;
        let result = provider.get_secret("TEST_SECRET").await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_value");

        env::remove_var("TEST_SECRET");
    }

    #[tokio::test]
    async fn test_env_secret_provider_not_found() {
        let provider = EnvSecretProvider;
        let result = provider.get_secret("NONEXISTENT_SECRET").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_secret_manager_env() {
        env::set_var("TEST_SECRET_2", "test_value_2");

        let manager = SecretManager::new_env();
        let result = manager.get_secret("TEST_SECRET_2").await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_value_2");

        env::remove_var("TEST_SECRET_2");
    }
}
