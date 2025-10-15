use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use password_hash::rand_core::OsRng;

use crate::{
    error::AppError,
    models::user::{AuthResponse, LoginRequest, NewUser, RegisterRequest, UserResponse},
    repositories::user_repository::{UserRepository, UserRepositoryTrait},
    services::jwt::JwtService,
};

pub struct AuthService<R: UserRepositoryTrait = UserRepository> {
    user_repository: R,
    jwt_service: JwtService,
}

impl<R: UserRepositoryTrait + Clone> Clone for AuthService<R> {
    fn clone(&self) -> Self {
        Self {
            user_repository: self.user_repository.clone(),
            jwt_service: self.jwt_service.clone(),
        }
    }
}

impl<R: UserRepositoryTrait> AuthService<R> {
    pub fn new(user_repository: R, jwt_service: JwtService) -> Self {
        Self {
            user_repository,
            jwt_service,
        }
    }

    #[tracing::instrument(name = "auth_register", skip(self, req), fields(email = %req.email, username = %req.username))]
    pub async fn register(&self, req: RegisterRequest) -> Result<AuthResponse, AppError> {
        tracing::debug!("Starting user registration");

        // Check if user already exists
        let existing_user = self
            .user_repository
            .find_by_email_or_username(&req.email, &req.username)
            .await?;

        if existing_user.is_some() {
            tracing::warn!("Registration failed: user already exists");
            return Err(AppError::BadRequest(
                "User with this email or username already exists".to_string(),
            ));
        }

        tracing::debug!("User does not exist, proceeding with registration");

        // Hash password
        let password_hash = self.hash_password(&req.password)?;
        tracing::trace!("Password hashed successfully");

        // Create new user
        let new_user = NewUser {
            email: req.email,
            username: req.username,
            password_hash,
        };

        let user = self.user_repository.create(new_user).await?;
        tracing::info!(user_id = %user.id, "User created successfully");

        // Generate JWT token
        let token = self
            .jwt_service
            .generate_token(user.id, user.email.clone(), user.username.clone())?;
        tracing::debug!("JWT token generated");

        Ok(AuthResponse {
            user: user.into(),
            token,
        })
    }

    #[tracing::instrument(name = "auth_login", skip(self, req), fields(email = %req.email))]
    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse, AppError> {
        tracing::debug!("Starting user login");

        // Find user by email
        let user = self
            .user_repository
            .find_by_email(&req.email)
            .await?
            .ok_or_else(|| {
                tracing::warn!("Login failed: user not found");
                AppError::Unauthorized("Invalid email or password".to_string())
            })?;

        tracing::debug!(user_id = %user.id, "User found");

        // Verify password
        self.verify_password(&req.password, &user.password_hash)?;
        tracing::trace!("Password verified successfully");

        // Generate JWT token
        let token = self
            .jwt_service
            .generate_token(user.id, user.email.clone(), user.username.clone())?;
        tracing::debug!("JWT token generated");

        tracing::info!(user_id = %user.id, "User logged in successfully");

        Ok(AuthResponse {
            user: user.into(),
            token,
        })
    }

    #[tracing::instrument(name = "auth_get_user_by_id", skip(self), fields(user_id = %user_id))]
    pub async fn get_user_by_id(&self, user_id: &str) -> Result<UserResponse, AppError> {
        tracing::debug!("Fetching user by ID");

        let uuid = uuid::Uuid::parse_str(user_id)
            .map_err(|_| {
                tracing::warn!("Invalid user ID format");
                AppError::BadRequest("Invalid user ID".to_string())
            })?;

        let user = self
            .user_repository
            .find_by_id(uuid)
            .await?
            .ok_or_else(|| {
                tracing::warn!("User not found");
                AppError::NotFound("User not found".to_string())
            })?;

        tracing::debug!("User found successfully");
        Ok(user.into())
    }

    fn hash_password(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| AppError::InternalServerError {
                message: "Failed to hash password".to_string(),
                source: Some(Box::new(std::io::Error::other(
                    e.to_string(),
                ))),
            })
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<(), AppError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| AppError::InternalServerError {
            message: "Invalid password hash".to_string(),
            source: Some(Box::new(std::io::Error::other(
                e.to_string(),
            ))),
        })?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::Unauthorized("Invalid email or password".to_string()))
    }
}
