use backend::{
    config::{Config, CorsConfig, DatabaseConfig, JwtConfig, ServerConfig},
    db, AppState,
};

/// Builder for creating test AppState with custom configuration
///
/// # Examples
///
/// ```ignore
/// let state = TestStateBuilder::new()
///     .with_jwt_secret("test-secret")
///     .with_db_url("postgres://localhost/test")
///     .build()
///     .await;
/// ```
pub struct TestStateBuilder {
    config: Config,
}

impl TestStateBuilder {
    /// Create a new builder with default test configuration
    pub fn new() -> Self {
        Self {
            config: Config {
                server: ServerConfig {
                    host: "127.0.0.1".to_string(),
                    port: 0,
                    environment: "test".to_string(),
                    request_timeout: 10,
                },
                database: DatabaseConfig {
                    url: std::env::var("TEST_DATABASE_URL")
                        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string()),
                    pool_size: 2,
                },
                jwt: JwtConfig {
                    secret: "test-secret-key-for-testing-only".to_string(),
                    expiration_hours: 1,
                },
                cors: CorsConfig {
                    allowed_origins: vec!["http://localhost:3000".to_string()],
                },
            },
        }
    }

    /// Set custom database URL
    #[allow(dead_code)]
    pub fn with_db_url(mut self, url: impl Into<String>) -> Self {
        self.config.database.url = url.into();
        self
    }

    /// Set custom JWT secret
    pub fn with_jwt_secret(mut self, secret: impl Into<String>) -> Self {
        self.config.jwt.secret = secret.into();
        self
    }

    /// Set custom JWT expiration hours
    #[allow(dead_code)]
    pub fn with_jwt_expiration(mut self, hours: i64) -> Self {
        self.config.jwt.expiration_hours = hours;
        self
    }

    /// Set custom database pool size
    #[allow(dead_code)]
    pub fn with_pool_size(mut self, size: usize) -> Self {
        self.config.database.pool_size = size;
        self
    }

    /// Set custom environment
    pub fn with_environment(mut self, env: impl Into<String>) -> Self {
        self.config.server.environment = env.into();
        self
    }

    /// Build the AppState
    pub async fn build(self) -> Result<AppState, Box<dyn std::error::Error>> {
        let db_pool = db::create_pool(&self.config.database.url, self.config.database.pool_size)?;

        // Test connection
        db::test_connection(&db_pool).await?;

        // Use AppState::new() to ensure consistency with production code
        Ok(AppState::new(self.config, db_pool))
    }

    /// Build the AppState, panicking on error (for test convenience)
    #[allow(dead_code)]
    pub async fn build_unwrap(self) -> AppState {
        self.build().await.expect("Failed to build test state")
    }
}

impl Default for TestStateBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires database
    async fn test_builder_default() {
        let state = TestStateBuilder::new().build().await;
        assert!(state.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires database
    async fn test_builder_custom() {
        let state = TestStateBuilder::new()
            .with_jwt_secret("custom-secret")
            .with_environment("testing")
            .build()
            .await;

        assert!(state.is_ok());
        let state = state.unwrap();
        assert_eq!(state.config.server.environment, "testing");
    }
}
