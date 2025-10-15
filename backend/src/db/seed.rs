// Database seeding utilities for development and testing
// Provides a way to populate the database with initial data

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    db::{schema::users, DbPool},
    error::AppError,
    models::user::{NewUser, User},
};

/// Seed data configuration
#[derive(Default)]
pub struct SeedConfig {
    pub clear_existing: bool,
}

/// Main seeding function
pub async fn seed_database(pool: &DbPool, config: SeedConfig) -> Result<(), AppError> {
    tracing::info!("Starting database seeding");

    if config.clear_existing {
        clear_users(pool).await?;
    }

    seed_users(pool).await?;

    tracing::info!("Database seeding completed");
    Ok(())
}

/// Clear all users (use with caution!)
async fn clear_users(pool: &DbPool) -> Result<(), AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| AppError::database("Failed to get connection", e))?;

    diesel::delete(users::table)
        .execute(&mut conn)
        .await
        .map_err(|e| AppError::database("Failed to clear users", e))?;

    tracing::info!("Cleared existing users");
    Ok(())
}

/// Seed example users
async fn seed_users(pool: &DbPool) -> Result<(), AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| AppError::database("Failed to get connection", e))?;

    // Note: In production, use proper password hashing!
    // For seeding, these are example hashed passwords using argon2
    let test_users = vec![
        NewUser {
            email: "admin@example.com".to_string(),
            username: "admin".to_string(),
            // Hash of "123"
            password_hash: "$argon2id$v=19$m=19456,t=2,p=1$tJap/+mRGuIZP8fT3qgVxg$qLT+8k+D6eGlhtxqKRnTW1QjppC9Uh/wuCQyKd9nOSA".to_string(),
        },
        NewUser {
            email: "user@example.com".to_string(),
            username: "user".to_string(),
            // Hash of "123"
            password_hash: "$argon2id$v=19$m=19456,t=2,p=1$tJap/+mRGuIZP8fT3qgVxg$qLT+8k+D6eGlhtxqKRnTW1QjppC9Uh/wuCQyKd9nOSA".to_string(),
        },
        NewUser {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            // Hash of "123"
            password_hash: "$argon2id$v=19$m=19456,t=2,p=1$tJap/+mRGuIZP8fT3qgVxg$qLT+8k+D6eGlhtxqKRnTW1QjppC9Uh/wuCQyKd9nOSA".to_string(),
        },
    ];

    for new_user in test_users {
        // Check if user already exists
        let existing = users::table
            .filter(users::email.eq(&new_user.email))
            .first::<User>(&mut conn)
            .await
            .optional()
            .map_err(|e| AppError::database("Failed to check existing user", e))?;

        if existing.is_none() {
            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&mut conn)
                .await
                .map_err(|e| AppError::database("Failed to insert user", e))?;

            tracing::info!("Seeded user: {}", new_user.username);
        } else {
            tracing::info!("User already exists, skipping: {}", new_user.username);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    /// Example of how to use seeding in tests
    #[tokio::test]
    #[ignore] // Ignore by default as it requires a database
    async fn test_seed_database() {
        // This would use a test database pool
        // let pool = create_test_pool().await;
        //
        // let config = SeedConfig {
        //     clear_existing: true,
        // };
        //
        // let result = seed_database(&pool, config).await;
        // assert!(result.is_ok());
    }
}
