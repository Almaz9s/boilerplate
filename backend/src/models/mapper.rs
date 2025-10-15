// Mapper functions to convert between DTOs and domain models
// This keeps the API contract separate from internal domain logic

use crate::models::{
    dto::{AuthResponseDto, LoginRequestDto, RegisterRequestDto, UserResponseDto},
    user::{AuthResponse, LoginRequest, RegisterRequest, User, UserResponse},
};

// ===== User Mappers =====

impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        UserResponseDto {
            id: user.id,
            email: user.email,
            username: user.username,
            created_at: user.created_at,
        }
    }
}

impl From<UserResponse> for UserResponseDto {
    fn from(response: UserResponse) -> Self {
        UserResponseDto {
            id: response.id,
            email: response.email,
            username: response.username,
            created_at: response.created_at,
        }
    }
}

// ===== Auth Mappers =====

impl From<RegisterRequestDto> for RegisterRequest {
    fn from(dto: RegisterRequestDto) -> Self {
        RegisterRequest {
            email: dto.email,
            username: dto.username,
            password: dto.password,
        }
    }
}

impl From<LoginRequestDto> for LoginRequest {
    fn from(dto: LoginRequestDto) -> Self {
        LoginRequest {
            email: dto.email,
            password: dto.password,
        }
    }
}

impl From<AuthResponse> for AuthResponseDto {
    fn from(response: AuthResponse) -> Self {
        AuthResponseDto {
            user: response.user.into(),
            token: response.token,
        }
    }
}
