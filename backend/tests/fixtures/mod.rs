// Test fixtures for creating test data
// Makes it easy to create consistent test data across test files

use backend::models::user::{NewUser, User};
use chrono::Utc;
use fake::{Fake, Faker};
use uuid::Uuid;

/// Generate a random test user with fake data
#[allow(dead_code)]
pub fn create_fake_user() -> User {
    User {
        id: Uuid::new_v4(),
        email: Faker.fake::<String>() + "@example.com",
        username: Faker.fake::<String>(),
        password_hash: "$argon2id$v=19$m=19456,t=2,p=1$test$test".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    }
}

/// Create a new user fixture with custom values
pub fn create_new_user(email: &str, username: &str, password_hash: &str) -> NewUser {
    NewUser {
        email: email.to_string(),
        username: username.to_string(),
        password_hash: password_hash.to_string(),
    }
}

/// Create a test user fixture with predefined values
pub fn create_test_user() -> User {
    User {
        id: Uuid::new_v4(),
        email: "test@example.com".to_string(),
        username: "testuser".to_string(),
        password_hash: "$argon2id$v=19$m=19456,t=2,p=1$test$test".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    }
}

/// Create an admin user fixture
#[allow(dead_code)]
pub fn create_admin_user() -> User {
    User {
        id: Uuid::new_v4(),
        email: "admin@example.com".to_string(),
        username: "admin".to_string(),
        password_hash: "$argon2id$v=19$m=19456,t=2,p=1$test$test".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    }
}

/// Create multiple test users
pub fn create_test_users(count: usize) -> Vec<User> {
    (0..count)
        .map(|i| User {
            id: Uuid::new_v4(),
            email: format!("user{}@example.com", i),
            username: format!("user{}", i),
            password_hash: "$argon2id$v=19$m=19456,t=2,p=1$test$test".to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        })
        .collect()
}

/// Builder pattern for creating custom user fixtures
pub struct UserFixtureBuilder {
    email: Option<String>,
    username: Option<String>,
    password_hash: Option<String>,
}

impl UserFixtureBuilder {
    pub fn new() -> Self {
        Self {
            email: None,
            username: None,
            password_hash: None,
        }
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    #[allow(dead_code)]
    pub fn password_hash(mut self, hash: impl Into<String>) -> Self {
        self.password_hash = Some(hash.into());
        self
    }

    pub fn build(self) -> User {
        User {
            id: Uuid::new_v4(),
            email: self.email.unwrap_or_else(|| "default@example.com".to_string()),
            username: self.username.unwrap_or_else(|| "defaultuser".to_string()),
            password_hash: self
                .password_hash
                .unwrap_or_else(|| "$argon2id$v=19$m=19456,t=2,p=1$test$test".to_string()),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

impl Default for UserFixtureBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_user() {
        let user = create_test_user();
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.username, "testuser");
    }

    #[test]
    fn test_user_fixture_builder() {
        let user = UserFixtureBuilder::new()
            .email("custom@example.com")
            .username("customuser")
            .build();

        assert_eq!(user.email, "custom@example.com");
        assert_eq!(user.username, "customuser");
    }

    #[test]
    fn test_create_multiple_users() {
        let users = create_test_users(5);
        assert_eq!(users.len(), 5);
        assert_eq!(users[0].email, "user0@example.com");
        assert_eq!(users[4].email, "user4@example.com");
    }
}
