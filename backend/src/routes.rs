use axum::{
    extract::DefaultBodyLimit,
    routing::get,
    Router,
};
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use std::time::Duration;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{docs::ApiDoc, handlers, metrics, middleware, AppState};

/// Default request body size limit: 2MB
/// This prevents memory exhaustion attacks and oversized uploads
const DEFAULT_BODY_LIMIT: usize = 2 * 1024 * 1024; // 2MB

pub fn create_router(state: AppState) -> Router {
    let cors_origins: Vec<_> = state
        .config
        .cors
        .allowed_origins
        .iter()
        .filter_map(|origin| {
            origin.parse().map_err(|e| {
                tracing::warn!("Invalid CORS origin '{}': {}", origin, e);
                e
            }).ok()
        })
        .collect();

    if cors_origins.is_empty() {
        tracing::warn!("No valid CORS origins configured, CORS will be restrictive");
    }

    let cors = CorsLayer::new()
        .allow_origin(cors_origins)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_credentials(false); // Explicitly set - JWT in headers, not cookies

    // Rate limiter for auth endpoints: 10 requests per minute per IP
    // Protects against brute force attacks on login/register
    // DISABLED in development builds for easier testing
    let auth_routes = Router::new()
        .route("/register", axum::routing::post(handlers::auth::register))
        .route("/login", axum::routing::post(handlers::auth::login))
        .route("/me", get(handlers::auth::me));

    // Only apply rate limiting in production builds
    #[cfg(not(debug_assertions))]
    let auth_routes = {
        let auth_rate_limiter = middleware::rate_limit::RateLimiter::auth(state.config.server.trust_proxy);
        auth_routes.layer(axum::middleware::from_fn(
            middleware::rate_limit::rate_limit_layer(auth_rate_limiter)
        ))
    };

    let api_routes = Router::new()
        .route("/health", get(handlers::health_check))
        .nest("/auth", auth_routes);
        // Add more routes here
        // .route("/users", get(handlers::user::list_users).post(handlers::user::create_user))
        // .route("/users/:id", get(handlers::user::get_user).put(handlers::user::update_user).delete(handlers::user::delete_user))

    let router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/metrics", get(metrics::metrics_handler))
        .nest("/api/v1", api_routes);

    // Add dev routes only in debug builds
    #[cfg(debug_assertions)]
    let router = {
        let dev_routes = Router::new()
            .route("/", get(handlers::dev::dashboard))
            .route("/state", get(handlers::dev::debug_state))
            .route("/health", get(handlers::dev::dev_health))
            .route("/echo", axum::routing::post(handlers::dev::echo))
            .route("/error/:type", get(handlers::dev::simulate_error))
            .route("/token", axum::routing::post(handlers::dev::generate_test_token))
            .route("/db-info", get(handlers::dev::db_info));

        tracing::info!("Development endpoints enabled at /dev/* (visit /dev for dashboard)");
        router.nest("/dev", dev_routes)
    };

    router
        .layer(
            tower::ServiceBuilder::new()
                // Middleware execution order (outer → inner):
                // 1. TraceLayer - Creates spans for distributed tracing
                // 2. RequestID - Adds unique request ID to headers & logs
                // 3. SecurityHeaders - Adds security headers to responses
                // 4. Metrics - Tracks request counts and latencies
                // 5. Compression - Compresses response bodies (gzip)
                // 6. CORS - Handles cross-origin requests
                // 7. Timeout - Enforces request timeout limits
                // 8. Logging - Logs request/response details
                // 9. BodyLimit - Enforces max body size (prevents DoS)
                // → Handler executes here
                .layer(TraceLayer::new_for_http())
                .layer(axum::middleware::from_fn(middleware::request_id_middleware))
                .layer(axum::middleware::from_fn(middleware::security_headers))
                .layer(axum::middleware::from_fn(metrics::track_metrics))
                .layer(CompressionLayer::new())
                .layer(cors)
                .layer(TimeoutLayer::new(Duration::from_secs(state.config.server.request_timeout)))
                .layer(axum::middleware::from_fn(middleware::log_request))
                .layer(DefaultBodyLimit::max(DEFAULT_BODY_LIMIT))
        )
        .with_state(state)
}
