use backend::{db, AppState, config::Config};
use testcontainers::{clients::Cli, Container};
use testcontainers_modules::postgres::Postgres;

/// Test database helper that manages a PostgreSQL container
#[allow(dead_code)]
pub struct TestDb<'a> {
    container: Container<'a, Postgres>,
    pub database_url: String,
}

impl<'a> TestDb<'a> {
    /// Create a new test database using testcontainers
    #[allow(dead_code)]
    pub fn new(docker: &'a Cli) -> Self {
        let postgres = Postgres::default();
        let container = docker.run(postgres);

        let port = container.get_host_port_ipv4(5432);
        let database_url = format!(
            "postgres://postgres:postgres@localhost:{}/postgres",
            port
        );

        Self {
            container,
            database_url,
        }
    }

    /// Create application state with test database
    #[allow(dead_code)]
    pub async fn create_app_state(&self) -> AppState {
        let mut config = Config::from_env()
            .unwrap_or_else(|_| Config::default_test_config());

        // Override database URL to use container
        config.database.url = self.database_url.clone();

        let db_pool = db::create_pool(&config.database.url, 5)
            .expect("Failed to create test database pool");

        // Run migrations
        // In production code, you'd use diesel_migrations::run_migrations here

        // Use AppState::new() to ensure consistency with production code
        AppState::new(config, db_pool)
    }
}

/// Create a minimal test state without database (for unit tests)
#[allow(dead_code)]
pub fn create_mock_state() -> AppState {
    let config = Config::default_test_config();

    // This will fail if actually used, but works for unit tests that mock DB
    let db_pool = db::create_pool("postgres://invalid:invalid@localhost/test", 1)
        .unwrap_or_else(|_| panic!("Mock state should not connect to real DB"));

    // Use AppState::new() to ensure consistency with production code
    AppState::new(config, db_pool)
}

/// Run a test with an isolated database transaction that auto-rolls back
///
/// This is perfect for integration tests where you want to:
/// 1. Make database changes
/// 2. Test those changes
/// 3. Automatically clean up without affecting other tests
///
/// # Example
/// ```
/// #[tokio::test]
/// async fn test_user_creation() {
///     with_test_transaction(|mut conn| async move {
///         let new_user = NewUser { /* ... */ };
///         let user = diesel::insert_into(users::table)
///             .values(&new_user)
///             .get_result::<User>(&mut conn)
///             .await
///             .unwrap();
///
///         assert_eq!(user.email, "test@example.com");
///         // Transaction automatically rolled back
///     }).await;
/// }
/// ```
#[allow(dead_code)]
pub async fn with_test_transaction<F, Fut, R>(f: F) -> R
where
    F: FnOnce(backend::db::DbConnection) -> Fut,
    Fut: std::future::Future<Output = R>,
{
    use diesel_async::AsyncConnection;

    // Get test database URL from env or use default
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/backend_test".to_string());

    let pool = db::create_pool(&database_url, 1)
        .expect("Failed to create test pool");

    let mut conn = pool.get()
        .await
        .expect("Failed to get connection from test pool");

    // Start a test transaction that will be automatically rolled back
    conn.begin_test_transaction()
        .await
        .expect("Failed to begin test transaction");

    // Run the test
    let result = f(conn).await;

    // Transaction is automatically rolled back when connection is dropped
    result
}

/// Helper for tests that need access to the pool but still want transaction rollback
///
/// # Example
/// ```
/// #[tokio::test]
/// async fn test_with_pool() {
///     let pool = get_test_pool();
///     let mut conn = pool.get().await.unwrap();
///     conn.begin_test_transaction().await.unwrap();
///
///     // Your test code here
///
///     // Auto-rollback on drop
/// }
/// ```
#[allow(dead_code)]
pub fn get_test_pool() -> backend::db::DbPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/backend_test".to_string());

    db::create_pool(&database_url, 5)
        .expect("Failed to create test pool")
}
