use axum::{extract::State, http::StatusCode, Json};
use validator::Validate;

use crate::{
    error::{AppError, JsonResult},
    middleware::auth::AuthUser,
    models::{
        dto::{AuthResponseDto, LoginRequestDto, RegisterRequestDto, UserResponseDto},
        user::{LoginRequest, RegisterRequest},
    },
    AppState,
};

/// Register a new user
///
/// POST /api/v1/auth/register
/// Body: { "email": "user@example.com", "username": "username", "password": "password123" }
#[tracing::instrument(name = "register_handler", skip(state, dto), fields(email = %dto.email, username = %dto.username))]
pub async fn register(
    State(state): State<AppState>,
    Json(dto): Json<RegisterRequestDto>,
) -> Result<(StatusCode, Json<AuthResponseDto>), AppError> {
    tracing::info!("Registration request received");

    // Validate request
    dto.validate()?;
    tracing::debug!("Request validation passed");

    // Register user using service from AppState
    let request: RegisterRequest = dto.into();
    let response = state.auth().register(request).await?;
    let response_dto: AuthResponseDto = response.into();

    tracing::info!("User registered successfully");
    Ok((StatusCode::CREATED, Json(response_dto)))
}

/// Login with email and password
///
/// POST /api/v1/auth/login
/// Body: { "email": "user@example.com", "password": "password123" }
#[tracing::instrument(name = "login_handler", skip(state, dto), fields(email = %dto.email))]
pub async fn login(
    State(state): State<AppState>,
    Json(dto): Json<LoginRequestDto>,
) -> JsonResult<AuthResponseDto> {
    tracing::info!("Login request received");

    // Validate request
    dto.validate()?;
    tracing::debug!("Request validation passed");

    // Login user using service from AppState
    let request: LoginRequest = dto.into();
    let response = state.auth().login(request).await?;
    let response_dto: AuthResponseDto = response.into();

    tracing::info!("User logged in successfully");
    Ok(Json(response_dto))
}

/// Get current user information
///
/// GET /api/v1/auth/me
/// Headers: { "Authorization": "Bearer <token>" }
#[tracing::instrument(name = "get_current_user", skip(state, auth_user), fields(user_id = %auth_user.user_id, email = %auth_user.email))]
pub async fn me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> JsonResult<UserResponseDto> {
    tracing::info!("Fetching current user information");

    // Get user by ID from token using service from AppState
    let user = state.auth().get_user_by_id(&auth_user.user_id).await?;
    let user_dto: UserResponseDto = user.into();

    tracing::debug!("User information retrieved successfully");
    Ok(Json(user_dto))
}
