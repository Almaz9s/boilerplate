pub mod builder;
pub mod macros;
pub mod test_db;
pub mod test_helpers;

use backend::{config::Config, db, AppState};

// Re-export for easier use
#[allow(unused_imports)]
pub use builder::TestStateBuilder;
#[allow(unused_imports)]
pub use macros::*;

/// Setup test database and application state for integration tests
/// This is a simple version that expects DATABASE_URL env var
pub fn setup_test_state() -> AppState {
    let mut config = Config::from_env()
        .unwrap_or_else(|_| Config::default_test_config());

    // Override database URL from environment if set (for CI/CD or local testing)
    if let Ok(db_url) = std::env::var("DATABASE_URL") {
        config.database.url = db_url;
    }

    let db_pool = db::create_pool(&config.database.url, 1)
        .expect("Failed to create test database pool");

    // Use the AppState constructor to ensure consistency with production code
    AppState::new(config, db_pool)
}

/// Clean up test data from the database (async version)
/// Removes users created by integration tests
/// Call this at the start of each test to ensure isolation
#[allow(dead_code)]
pub async fn cleanup_test_data(pool: &db::DbPool) {
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let test_emails = vec![
        "newuser@example.com",
        "duplicate@example.com",
        "logintest@example.com",
        "wrongpass@example.com",
        "currentuser@example.com",
    ];

    let mut conn = pool.get().await.expect("Failed to get connection for cleanup");

    for email in test_emails {
        let _ = diesel::delete(backend::db::schema::users::table)
            .filter(backend::db::schema::users::email.eq(email))
            .execute(&mut conn)
            .await;
    }
}

#[allow(unused_imports)]
pub use test_db::{TestDb, create_mock_state};
#[allow(unused_imports)]
pub use test_helpers::{TestClient, TestResponse, create_test_jwt};
