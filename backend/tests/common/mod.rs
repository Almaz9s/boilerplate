pub mod builder;
pub mod macros;
pub mod test_db;
pub mod test_helpers;

use std::sync::OnceLock;

use backend::{config::Config, db, AppState};

// Re-export for easier use
#[allow(unused_imports)]
pub use builder::TestStateBuilder;
#[allow(unused_imports)]
pub use macros::*;

/// Hardcoded test database URL.
///
/// This **must never** point at the real dev DB (`backend_db`). Integration
/// tests freely INSERT/UPDATE/DELETE rows, so they get their own dedicated
/// database (`backend_db_test`) on the same Postgres instance that
/// `docker-compose.yml` manages (port 17302).
///
/// To point tests at a different host/port/name (e.g. in CI) set the
/// `TEST_DATABASE_URL` env var. We deliberately **never** fall back to
/// `DATABASE_URL` — that was the exact footgun that let integration tests
/// pollute the dev DB.
pub(super) const DEFAULT_TEST_DATABASE_URL: &str =
    "postgres://postgres:postgres@localhost:17302/backend_db_test";

/// Embedded migrations — baked into the test binary at compile time so tests
/// don't depend on the diesel CLI being installed.
const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("migrations");

/// Tracks whether the test DB has been bootstrapped (created + migrated) in this
/// process. We only do this work once per `cargo test` invocation.
static TEST_DB_INIT: OnceLock<()> = OnceLock::new();

/// Resolve the test database URL.
///
/// Reads `TEST_DATABASE_URL` if set, otherwise the hardcoded default. **Never**
/// reads `DATABASE_URL`, and hard-fails if the resolved URL would collide with
/// the real dev DB (`DATABASE_URL`) or doesn't look like a test database.
pub(super) fn test_database_url() -> String {
    let url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| DEFAULT_TEST_DATABASE_URL.to_string());

    let db_name = extract_db_name(&url);

    // Guard 1: the database name must obviously be a test database.
    assert!(
        db_name.contains("test"),
        "Refusing to use test database: name `{}` does not contain `test`. \
         Set TEST_DATABASE_URL to something like `{}`.",
        db_name,
        DEFAULT_TEST_DATABASE_URL
    );

    // Guard 2: never equal the real dev DB, even if someone sets
    // TEST_DATABASE_URL == DATABASE_URL by mistake.
    if let Ok(real_url) = std::env::var("DATABASE_URL") {
        assert_ne!(
            normalize_url(&url),
            normalize_url(&real_url),
            "Refusing to run tests: TEST_DATABASE_URL equals DATABASE_URL ({}). \
             Tests would clobber real data.",
            real_url
        );
    }

    url
}

/// Extract the database name from a Postgres URL, e.g.
/// `postgres://u:p@host:5432/backend_db_test?x=1` -> `backend_db_test`.
fn extract_db_name(url: &str) -> String {
    url.rsplit('/')
        .next()
        .and_then(|s| s.split('?').next())
        .map(|s| s.to_string())
        .unwrap_or_default()
}

/// Strip credentials/query so the equality guard compares host+port+db only.
fn normalize_url(url: &str) -> String {
    // Drop everything up to and including the last `@` (userinfo), and any
    // trailing query string. Cheap and dependency-free.
    let after_auth = url.rsplit_once('@').map(|(_, rest)| rest).unwrap_or(url);
    after_auth.split('?').next().unwrap_or(after_auth).to_string()
}

/// Build the maintenance DSN (same host/port/credentials, `postgres` database)
/// used only to bootstrap the test database via `CREATE DATABASE`.
fn maintenance_url(test_db_url: &str) -> String {
    match test_db_url.rsplit_once('/') {
        Some((prefix, _db_and_query)) => format!("{}/postgres", prefix),
        None => test_db_url.to_string(),
    }
}

/// Ensure the test database exists, creating it via the maintenance connection
/// if missing. Safe to call repeatedly.
fn ensure_test_database_exists(test_db_url: &str) {
    use diesel::prelude::*;
    use diesel::sql_query;

    let db_name = extract_db_name(test_db_url);

    // Belt-and-braces guard (also enforced in `test_database_url`): refuse to
    // bootstrap anything that isn't obviously a test DB.
    if !db_name.contains("test") {
        panic!(
            "Refusing to bootstrap test database: name `{}` does not contain `test`.",
            db_name
        );
    }

    let maint_url = maintenance_url(test_db_url);
    let mut maintenance_conn = PgConnection::establish(&maint_url).unwrap_or_else(|e| {
        panic!(
            "Failed to connect to maintenance DB at {}: {}. \
             Is the `boilerplate_postgres` container running? \
             (docker compose up -d in backend/)",
            maint_url, e
        )
    });

    #[derive(QueryableByName)]
    struct Exists {
        #[diesel(sql_type = diesel::sql_types::Bool)]
        exists: bool,
    }

    let rows: Vec<Exists> = sql_query(format!(
        "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = '{}') AS exists",
        db_name.replace('\'', "''")
    ))
    .load(&mut maintenance_conn)
    .expect("Failed to query pg_database");

    let already_exists = rows.first().map(|r| r.exists).unwrap_or(false);

    if !already_exists {
        // CREATE DATABASE cannot run inside a transaction block; diesel's
        // sql_query runs statements outside an implicit transaction.
        sql_query(format!(
            "CREATE DATABASE \"{}\"",
            db_name.replace('"', "\"\"")
        ))
        .execute(&mut maintenance_conn)
        .expect("Failed to CREATE DATABASE for tests");
    }
}

/// Run embedded diesel migrations against the test database.
fn run_test_migrations(test_db_url: &str) {
    use diesel::prelude::*;
    use diesel_migrations::MigrationHarness;

    let mut conn = PgConnection::establish(test_db_url)
        .unwrap_or_else(|e| panic!("Failed to connect to test DB {}: {}", test_db_url, e));

    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations on test database");
}

/// Bootstrap the test database exactly once per process: create it if missing,
/// then run all pending migrations.
pub(super) fn bootstrap_test_db_once(test_db_url: &str) {
    TEST_DB_INIT.get_or_init(|| {
        ensure_test_database_exists(test_db_url);
        run_test_migrations(test_db_url);
    });
}

/// Setup test database and application state for integration tests.
///
/// This **always** connects to an isolated test database (`backend_db_test` by
/// default), never to the real dev DB. On first call per process it also
/// creates the DB if missing and runs embedded migrations, so tests are
/// self-bootstrapping on a fresh machine as long as the Postgres container is
/// up.
pub fn setup_test_state() -> AppState {
    let test_db_url = test_database_url();

    bootstrap_test_db_once(&test_db_url);

    let mut config = Config::default_test_config();
    config.database.url = test_db_url;

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
