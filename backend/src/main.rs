use backend::{config::Config, db, jobs, metrics, routes, tracing_config, AppState};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Determine environment
    let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

    // Initialize tracing - check if file logging is requested in dev mode
    if environment != "production" && std::env::var("LOG_TO_FILE").is_ok() {
        tracing_config::init_dev_with_file_logging("logs/dev.log")?;
    } else {
        tracing_config::init_tracing("backend", &environment)?;
    }

    // Setup better panic messages in development
    #[cfg(debug_assertions)]
    backend::setup_dev_panic_handler();

    tracing::info!(
        environment = %environment,
        version = %env!("CARGO_PKG_VERSION"),
        "Starting backend service"
    );

    // Initialize metrics
    metrics::init_metrics();
    tracing::info!("Metrics initialized");

    // Load configuration (with smart defaults in dev mode)
    let config = Config::load()?;
    tracing::info!("Configuration loaded successfully");
    tracing::info!("Environment: {}", config.server.environment);

    // Create database pool
    let db_pool = db::create_pool(&config.database.url, config.database.pool_size)?;
    tracing::info!("Database pool created successfully");

    // Test database connection
    db::test_connection(&db_pool).await?;
    tracing::info!("Database connection validated");

    // Create application state (services are initialized inside)
    let state = AppState::new(config.clone(), db_pool);

    // Initialize background job scheduler
    let scheduler = jobs::init_scheduler(Arc::new(state.clone())).await?;
    tracing::info!("Background job scheduler initialized");

    // Create router
    let app = routes::create_router(state);

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server listening on {}", addr);

    // Setup graceful shutdown
    let graceful_shutdown = shutdown_signal();

    tokio::select! {
        result = axum::serve(listener, app) => {
            if let Err(e) = result {
                tracing::error!("Server error: {}", e);
            }
        }
        _ = graceful_shutdown => {
            tracing::info!("Graceful shutdown initiated");
        }
    }

    // Shutdown scheduler
    jobs::shutdown_scheduler(scheduler).await;

    tracing::info!("Server shutdown complete");

    // Shutdown tracing and flush spans (do this last to ensure all logs are flushed)
    tracing_config::shutdown_tracing().await;

    Ok(())
}

/// Handle graceful shutdown signals
async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C signal, starting graceful shutdown");
        },
        _ = terminate => {
            tracing::info!("Received terminate signal, starting graceful shutdown");
        },
    }
}
