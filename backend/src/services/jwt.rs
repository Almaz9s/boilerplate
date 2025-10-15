use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // subject (user id)
    pub email: String,
    pub username: String,
    pub exp: i64,     // expiration time
    pub iat: i64,     // issued at
}

#[derive(Clone)]
pub struct JwtService {
    secret: String,
    expiration_hours: i64,
}

impl JwtService {
    pub fn new(secret: String, expiration_hours: i64) -> Self {
        Self {
            secret,
            expiration_hours,
        }
    }

    pub fn generate_token(
        &self,
        user_id: Uuid,
        email: String,
        username: String,
    ) -> Result<String, AppError> {
        if self.expiration_hours <= 0 {
            return Err(AppError::ConfigError(
                "JWT expiration hours must be positive".to_string(),
            ));
        }

        let now = Utc::now();
        let expires_at = now + Duration::hours(self.expiration_hours);

        let claims = Claims {
            sub: user_id.to_string(),
            email,
            username,
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AppError::internal("Failed to generate JWT token", e))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_verify_token() {
        let jwt_service = JwtService::new("test_secret_key".to_string(), 24);
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let username = "testuser".to_string();

        let token = jwt_service
            .generate_token(user_id, email.clone(), username.clone())
            .unwrap();

        let claims = jwt_service.verify_token(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
        assert_eq!(claims.username, username);
    }

    #[test]
    fn test_verify_invalid_token() {
        let jwt_service = JwtService::new("test_secret_key".to_string(), 24);
        let result = jwt_service.verify_token("invalid_token");

        assert!(result.is_err());
    }
}
