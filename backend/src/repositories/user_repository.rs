use async_trait::async_trait;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    db::{schema::users, DbPool},
    error::{AppError, DatabaseResultExt},
    logged_query,
    models::user::{NewUser, User},
};

/// Repository trait for user data access operations
/// Allows for easy mocking and testing
#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    async fn find_by_email_or_username(
        &self,
        email: &str,
        username: &str,
    ) -> Result<Option<User>, AppError>;
    async fn create(&self, new_user: NewUser) -> Result<User, AppError>;
    async fn update_password(&self, id: Uuid, password_hash: String) -> Result<User, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, AppError>;
}

/// Concrete implementation of UserRepository
#[derive(Clone)]
pub struct UserRepository {
    db_pool: DbPool,
}

impl UserRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    async fn get_connection(&self) -> Result<crate::db::DbConnection, AppError> {
        crate::db::get_connection(&self.db_pool).await
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let mut conn = self.get_connection().await?;

        logged_query!(
            "SELECT * FROM users WHERE id = $1",
            users::table
                .find(id)
                .first::<User>(&mut conn)
                .await
        )
        .optional()
        .with_db_context(|| format!("Failed to query user by id: {}", id))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let mut conn = self.get_connection().await?;

        logged_query!(
            "SELECT * FROM users WHERE email = $1",
            users::table
                .filter(users::email.eq(email))
                .first::<User>(&mut conn)
                .await
        )
        .optional()
        .with_db_context(|| format!("Failed to query user by email: {}", email))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let mut conn = self.get_connection().await?;

        users::table
            .filter(users::username.eq(username))
            .first::<User>(&mut conn)
            .await
            .optional()
            .with_db_context(|| format!("Failed to query user by username: {}", username))
    }

    async fn find_by_email_or_username(
        &self,
        email: &str,
        username: &str,
    ) -> Result<Option<User>, AppError> {
        let mut conn = self.get_connection().await?;

        users::table
            .filter(users::email.eq(email).or(users::username.eq(username)))
            .first::<User>(&mut conn)
            .await
            .optional()
            .with_db_context(|| format!("Failed to query user by email '{}' or username '{}'", email, username))
    }

    async fn create(&self, new_user: NewUser) -> Result<User, AppError> {
        let mut conn = self.get_connection().await?;

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut conn)
            .await
            .with_db_context(|| format!("Failed to create user with email: {}", new_user.email))
    }

    async fn update_password(&self, id: Uuid, password_hash: String) -> Result<User, AppError> {
        let mut conn = self.get_connection().await?;

        diesel::update(users::table.find(id))
            .set(users::password_hash.eq(password_hash))
            .get_result::<User>(&mut conn)
            .await
            .with_db_context(|| format!("Failed to update password for user id: {}", id))
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let mut conn = self.get_connection().await?;

        diesel::delete(users::table.find(id))
            .execute(&mut conn)
            .await
            .with_db_context(|| format!("Failed to delete user id: {}", id))?;

        Ok(())
    }

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, AppError> {
        let mut conn = self.get_connection().await?;

        users::table
            .limit(limit)
            .offset(offset)
            .load::<User>(&mut conn)
            .await
            .with_db_context(|| format!("Failed to list users (limit: {}, offset: {})", limit, offset))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementation for testing
    pub struct MockUserRepository {
        pub users: std::sync::Arc<tokio::sync::Mutex<Vec<User>>>,
    }

    impl MockUserRepository {
        #[allow(dead_code)]
        pub fn new() -> Self {
            Self {
                users: std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new())),
            }
        }

        #[allow(dead_code)]
        pub async fn add_user(&self, user: User) {
            self.users.lock().await.push(user);
        }
    }

    #[async_trait]
    impl UserRepositoryTrait for MockUserRepository {
        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
            let users = self.users.lock().await;
            Ok(users.iter().find(|u| u.id == id).cloned())
        }

        async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
            let users = self.users.lock().await;
            Ok(users.iter().find(|u| u.email == email).cloned())
        }

        async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
            let users = self.users.lock().await;
            Ok(users.iter().find(|u| u.username == username).cloned())
        }

        async fn find_by_email_or_username(
            &self,
            email: &str,
            username: &str,
        ) -> Result<Option<User>, AppError> {
            let users = self.users.lock().await;
            Ok(users
                .iter()
                .find(|u| u.email == email || u.username == username)
                .cloned())
        }

        async fn create(&self, new_user: NewUser) -> Result<User, AppError> {
            let user = User {
                id: Uuid::new_v4(),
                email: new_user.email,
                username: new_user.username,
                password_hash: new_user.password_hash,
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            };
            self.users.lock().await.push(user.clone());
            Ok(user)
        }

        async fn update_password(&self, id: Uuid, password_hash: String) -> Result<User, AppError> {
            let mut users = self.users.lock().await;
            let user = users
                .iter_mut()
                .find(|u| u.id == id)
                .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
            user.password_hash = password_hash;
            Ok(user.clone())
        }

        async fn delete(&self, id: Uuid) -> Result<(), AppError> {
            let mut users = self.users.lock().await;
            users.retain(|u| u.id != id);
            Ok(())
        }

        async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, AppError> {
            let users = self.users.lock().await;
            Ok(users
                .iter()
                .skip(offset as usize)
                .take(limit as usize)
                .cloned()
                .collect())
        }
    }
}
