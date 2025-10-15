// Library exports for binary crates and tests

pub mod config;
pub mod db;
pub mod dev_macros;
pub mod docs;
pub mod error;
pub mod handlers;
pub mod jobs;
pub mod metrics;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod tracing_config;
pub mod types;

// Development-only utilities (compiled out in release builds)
#[cfg(debug_assertions)]
pub mod dev;

// Re-export dev panic handler setup
#[cfg(debug_assertions)]
pub use dev::setup_dev_panic_handler;

use config::Config;
use db::DbPool;
use repositories::UserRepository;
use services::{auth::AuthService, jwt::JwtService};
use std::sync::Arc;

/// Application services layer
/// Groups all business logic services together
#[derive(Clone)]
pub struct Services {
    pub auth: Arc<AuthService>,
    pub user_repo: Arc<UserRepository>,
    pub jwt: Arc<JwtService>,
}

impl Services {
    pub fn new(db_pool: DbPool, config: &Config) -> Self {
        let jwt_service = JwtService::new(
            config.jwt.secret.clone(),
            config.jwt.expiration_hours,
        );
        let user_repository = UserRepository::new(db_pool);
        let auth_service = AuthService::new(user_repository.clone(), jwt_service.clone());

        Self {
            auth: Arc::new(auth_service),
            user_repo: Arc::new(user_repository),
            jwt: Arc::new(jwt_service),
        }
    }
}

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db_pool: DbPool,
    pub services: Services,
}

impl AppState {
    pub fn new(config: Config, db_pool: DbPool) -> Self {
        let config = Arc::new(config);
        let services = Services::new(db_pool.clone(), &config);

        Self {
            config,
            db_pool,
            services,
        }
    }

    /// Convenient access to auth service
    #[inline]
    pub fn auth(&self) -> &AuthService {
        &self.services.auth
    }

    /// Convenient access to user repository
    #[inline]
    pub fn user_repo(&self) -> &UserRepository {
        &self.services.user_repo
    }

    /// Convenient access to JWT service
    #[inline]
    pub fn jwt(&self) -> &JwtService {
        &self.services.jwt
    }
}
