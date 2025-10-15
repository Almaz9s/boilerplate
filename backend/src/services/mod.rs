pub mod auth;
pub mod jwt;

use crate::db::DbPool;

/// Example service trait showing how to structure services
#[async_trait::async_trait]
#[allow(dead_code)]
pub trait Service: Send + Sync {
    /// Initialize the service with dependencies
    fn new(pool: DbPool) -> Self where Self: Sized;
}
