pub mod pagination;
pub mod query_log;
pub mod schema;
pub mod seed;
pub mod transaction;

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

use crate::error::AppError;

pub type DbPool = Pool<AsyncPgConnection>;
pub type DbConnection = diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection>;

pub fn create_pool(database_url: &str, max_size: usize) -> Result<DbPool, AppError> {
    tracing::debug!(
        max_size = max_size,
        "Creating database connection pool"
    );
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder(config)
        .max_size(max_size)
        .build()
        .map_err(|e| AppError::database("Failed to create connection pool", e))
}

#[tracing::instrument(name = "db_test_connection", skip(pool))]
pub async fn test_connection(pool: &DbPool) -> Result<(), AppError> {
    tracing::debug!("Testing database connection");
    let _conn = pool.get()
        .await
        .map_err(|e| AppError::database("Failed to test database connection", e))?;
    tracing::debug!("Database connection test successful");
    Ok(())
}

/// Get a database connection from the pool
///
/// Logs a warning if connection acquisition takes longer than 100ms,
/// which can indicate pool exhaustion or database issues.
///
/// In debug builds with QUERY_LOG=1, also logs all SQL queries executed on the connection.
#[tracing::instrument(name = "db_get_connection", skip(pool))]
pub async fn get_connection(pool: &DbPool) -> Result<DbConnection, AppError> {
    let start = std::time::Instant::now();
    tracing::trace!("Acquiring connection from pool");

    let conn = pool.get()
        .await
        .map_err(|e| AppError::database("Failed to get connection from pool", e))?;

    let elapsed = start.elapsed();
    if elapsed.as_millis() > 100 {
        let pool_status = pool.status();
        tracing::warn!(
            duration_ms = elapsed.as_millis(),
            available = pool_status.available as usize,
            size = pool_status.size,
            max_size = pool_status.max_size,
            "Slow connection acquisition - possible pool exhaustion"
        );
    } else {
        tracing::trace!(duration_ms = elapsed.as_millis(), "Connection acquired from pool");
    }

    Ok(conn)
}

/// Log SQL query execution (only in debug builds with QUERY_LOG=1)
#[cfg(debug_assertions)]
pub fn log_query(sql: &str, duration: std::time::Duration) {
    if std::env::var("QUERY_LOG").is_ok() {
        let duration_ms = duration.as_millis();
        if duration_ms > 100 {
            tracing::warn!(
                duration_ms = duration_ms,
                sql = sql,
                "🐌 Slow query"
            );
        } else {
            tracing::debug!(
                duration_ms = duration_ms,
                sql = sql,
                "🗄️  Query"
            );
        }
    }
}

/// Log SQL query execution with result count (only in debug builds with QUERY_LOG=1)
#[cfg(debug_assertions)]
pub fn log_query_with_count(sql: &str, duration: std::time::Duration, count: Option<usize>) {
    if std::env::var("QUERY_LOG").is_ok() {
        let duration_ms = duration.as_millis();
        if duration_ms > 100 {
            tracing::warn!(
                duration_ms = duration_ms,
                sql = sql,
                count = count,
                "🐌 Slow query"
            );
        } else {
            tracing::debug!(
                duration_ms = duration_ms,
                sql = sql,
                count = count,
                "🗄️  Query"
            );
        }
    }
}

#[cfg(not(debug_assertions))]
pub fn log_query(_sql: &str, _duration: std::time::Duration) {
    // No-op in release builds
}

#[cfg(not(debug_assertions))]
pub fn log_query_with_count(_sql: &str, _duration: std::time::Duration, _count: Option<usize>) {
    // No-op in release builds
}

/// Database pool statistics for monitoring
#[derive(Debug, serde::Serialize)]
pub struct PoolStats {
    pub size: usize,
    pub available: usize,
    pub max_size: usize,
}

/// Get current pool statistics
pub fn pool_stats(pool: &DbPool) -> PoolStats {
    let status = pool.status();
    PoolStats {
        size: status.size,
        available: status.available as usize,
        max_size: status.max_size,
    }
}
