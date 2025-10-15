// DTOs (Data Transfer Objects) - Request and Response types
// These are separate from domain models to allow independent evolution

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

// ===== Auth DTOs =====

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequestDto {
    #[validate(email(message = "Invalid email address"))]
    #[schema(example = "user@example.com")]
    pub email: String,

    #[validate(length(min = 3, max = 100, message = "Username must be between 3 and 100 characters"))]
    #[schema(example = "johndoe")]
    pub username: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[schema(example = "SecurePass123!")]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequestDto {
    #[validate(email(message = "Invalid email address"))]
    #[schema(example = "user@example.com")]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    #[schema(example = "SecurePass123!")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponseDto {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,

    #[schema(example = "user@example.com")]
    pub email: String,

    #[schema(example = "johndoe")]
    pub username: String,

    #[schema(example = "2024-01-15T10:30:00")]
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthResponseDto {
    pub user: UserResponseDto,

    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub token: String,
}

// ===== User DTOs =====

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequestDto {
    #[validate(email(message = "Invalid email address"))]
    #[schema(example = "newemail@example.com")]
    pub email: Option<String>,

    #[validate(length(min = 3, max = 100, message = "Username must be between 3 and 100 characters"))]
    #[schema(example = "newusername")]
    pub username: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordRequestDto {
    #[validate(length(min = 1, message = "Current password is required"))]
    pub current_password: String,

    #[validate(length(min = 8, message = "New password must be at least 8 characters"))]
    pub new_password: String,
}

// ===== List/Pagination DTOs =====

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ListUsersRequestDto {
    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100"))]
    #[schema(example = 20)]
    pub limit: Option<i64>,

    #[validate(range(min = 0, message = "Offset must be non-negative"))]
    #[schema(example = 0)]
    pub offset: Option<i64>,
}

impl Default for ListUsersRequestDto {
    fn default() -> Self {
        Self {
            limit: Some(20),
            offset: Some(0),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ListUsersResponseDto {
    pub users: Vec<UserResponseDto>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}
