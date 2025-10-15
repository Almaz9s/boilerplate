// Integration tests for repository layer
// Tests use fixtures for consistent test data

mod common;
mod fixtures;

use backend::repositories::{UserRepository, UserRepositoryTrait};
use fixtures::*;
use uuid::Uuid;

#[tokio::test]
async fn test_create_user() {
    let state = common::setup_test_state();
    let repository = UserRepository::new(state.db_pool.clone());

    let unique_id = Uuid::new_v4();
    let email = format!("repo_test_{}@example.com", unique_id);
    let username = format!("repotest_{}", unique_id);

    let new_user = create_new_user(
        &email,
        &username,
        "$argon2id$v=19$m=19456,t=2,p=1$test$test",
    );

    let result = repository.create(new_user).await;
    assert!(result.is_ok());

    let user = result.unwrap();
    assert_eq!(user.email, email);
    assert_eq!(user.username, username);
}

#[tokio::test]
async fn test_find_user_by_id() {
    let state = common::setup_test_state();
    let repository = UserRepository::new(state.db_pool.clone());

    // Create user first
    let unique_id = Uuid::new_v4();
    let email = format!("findbyid_{}@example.com", unique_id);
    let username = format!("findbyid_{}", unique_id);

    let new_user = create_new_user(
        &email,
        &username,
        "$argon2id$v=19$m=19456,t=2,p=1$test$test",
    );

    let created_user = repository.create(new_user).await.unwrap();

    // Find by ID
    let result = repository.find_by_id(created_user.id).await;
    assert!(result.is_ok());

    let found_user = result.unwrap();
    assert!(found_user.is_some());
    assert_eq!(found_user.unwrap().id, created_user.id);
}

#[tokio::test]
async fn test_find_user_by_email() {
    let state = common::setup_test_state();
    let repository = UserRepository::new(state.db_pool.clone());

    // Create user
    let unique_id = Uuid::new_v4();
    let email = format!("findbyemail_{}@example.com", unique_id);
    let username = format!("findbyemail_{}", unique_id);

    let new_user = create_new_user(
        &email,
        &username,
        "$argon2id$v=19$m=19456,t=2,p=1$test$test",
    );

    repository.create(new_user).await.unwrap();

    // Find by email
    let result = repository.find_by_email(&email).await;
    assert!(result.is_ok());

    let found_user = result.unwrap();
    assert!(found_user.is_some());
    assert_eq!(found_user.unwrap().email, email);
}

#[tokio::test]
async fn test_find_user_by_username() {
    let state = common::setup_test_state();
    let repository = UserRepository::new(state.db_pool.clone());

    // Create user
    let unique_id = Uuid::new_v4();
    let email = format!("findbyusername_{}@example.com", unique_id);
    let username = format!("findbyusername_{}", unique_id);

    let new_user = create_new_user(
        &email,
        &username,
        "$argon2id$v=19$m=19456,t=2,p=1$test$test",
    );

    repository.create(new_user).await.unwrap();

    // Find by username
    let result = repository.find_by_username(&username).await;
    assert!(result.is_ok());

    let found_user = result.unwrap();
    assert!(found_user.is_some());
    assert_eq!(found_user.unwrap().username, username);
}

#[tokio::test]
async fn test_find_nonexistent_user() {
    let state = common::setup_test_state();
    let repository = UserRepository::new(state.db_pool.clone());

    let result = repository.find_by_email("nonexistent@example.com").await;
    assert!(result.is_ok());

    let found_user = result.unwrap();
    assert!(found_user.is_none());
}

#[tokio::test]
async fn test_update_user_password() {
    let state = common::setup_test_state();
    let repository = UserRepository::new(state.db_pool.clone());

    // Create user
    let unique_id = Uuid::new_v4();
    let email = format!("updatepass_{}@example.com", unique_id);
    let username = format!("updatepass_{}", unique_id);

    let new_user = create_new_user(
        &email,
        &username,
        "$argon2id$v=19$m=19456,t=2,p=1$test$old",
    );

    let created_user = repository.create(new_user).await.unwrap();

    // Update password
    let new_hash = "$argon2id$v=19$m=19456,t=2,p=1$test$new";
    let result = repository.update_password(created_user.id, new_hash.to_string()).await;
    assert!(result.is_ok());

    let updated_user = result.unwrap();
    assert_eq!(updated_user.password_hash, new_hash);
}

#[tokio::test]
async fn test_delete_user() {
    let state = common::setup_test_state();
    let repository = UserRepository::new(state.db_pool.clone());

    // Create user
    let unique_id = Uuid::new_v4();
    let email = format!("deleteuser_{}@example.com", unique_id);
    let username = format!("deleteuser_{}", unique_id);

    let new_user = create_new_user(
        &email,
        &username,
        "$argon2id$v=19$m=19456,t=2,p=1$test$test",
    );

    let created_user = repository.create(new_user).await.unwrap();

    // Delete user
    let result = repository.delete(created_user.id).await;
    assert!(result.is_ok());

    // Verify user is deleted
    let found = repository.find_by_id(created_user.id).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_list_users() {
    let state = common::setup_test_state();
    let repository = UserRepository::new(state.db_pool.clone());

    // Create multiple users with unique IDs per test run
    let test_id = Uuid::new_v4();
    for i in 0..5 {
        let new_user = create_new_user(
            &format!("listuser_{}_{}@example.com", test_id, i),
            &format!("listuser_{}_{}", test_id, i),
            "$argon2id$v=19$m=19456,t=2,p=1$test$test",
        );
        repository.create(new_user).await.unwrap();
    }

    // List users
    let result = repository.list(10, 0).await;
    assert!(result.is_ok());

    let users = result.unwrap();
    assert!(users.len() >= 5);
}

#[tokio::test]
async fn test_list_users_with_pagination() {
    let state = common::setup_test_state();
    let repository = UserRepository::new(state.db_pool.clone());

    // Create users with unique IDs per test run
    let test_id = Uuid::new_v4();
    for i in 0..10 {
        let new_user = create_new_user(
            &format!("pageuser_{}_{}@example.com", test_id, i),
            &format!("pageuser_{}_{}", test_id, i),
            "$argon2id$v=19$m=19456,t=2,p=1$test$test",
        );
        repository.create(new_user).await.unwrap();
    }

    // Get first page
    let page1 = repository.list(5, 0).await.unwrap();
    assert!(page1.len() <= 5);

    // Get second page
    let page2 = repository.list(5, 5).await.unwrap();
    assert!(page2.len() <= 5);

    // Verify different users
    if !page1.is_empty() && !page2.is_empty() {
        assert_ne!(page1[0].id, page2[0].id);
    }
}
