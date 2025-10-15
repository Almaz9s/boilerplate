use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Backend API",
        version = "0.1.0",
        description = "Production-ready Rust backend API with Axum and Diesel",
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    ),
    paths(
        crate::handlers::health::health_check,
        // TODO: Add utoipa::path annotations to auth handlers
        // crate::handlers::auth::register,
        // crate::handlers::auth::login,
        // crate::handlers::auth::me,
        // Add more paths here as you create them
    ),
    components(
        schemas(
            crate::models::HealthResponse,
            crate::models::HealthChecks,
            crate::models::SubsystemHealth,
            crate::models::PaginationParams,
            crate::models::PaginationMeta,
            crate::models::dto::RegisterRequestDto,
            crate::models::dto::LoginRequestDto,
            crate::models::dto::UserResponseDto,
            crate::models::dto::AuthResponseDto,
            // Add more schemas here
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
    )
)]
pub struct ApiDoc;
