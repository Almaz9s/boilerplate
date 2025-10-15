mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use backend::{models::dto::*, routes};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_user_registration_flow() {
    let state = common::setup_test_state();
    common::cleanup_test_data(&state.db_pool).await;
    let app = routes::create_router(state);

    // Register a new user
    let register_payload = json!({
        "email": "newuser@example.com",
        "username": "newuser",
        "password": "SecurePass123!"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let auth_response: AuthResponseDto = serde_json::from_slice(&body).unwrap();

    assert_eq!(auth_response.user.email, "newuser@example.com");
    assert_eq!(auth_response.user.username, "newuser");
    assert!(!auth_response.token.is_empty());
}

#[tokio::test]
async fn test_duplicate_user_registration() {
    let state = common::setup_test_state();
    common::cleanup_test_data(&state.db_pool).await;
    let app = routes::create_router(state);

    let register_payload = json!({
        "email": "duplicate@example.com",
        "username": "duplicate",
        "password": "SecurePass123!"
    });

    // First registration should succeed
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // Second registration with same email should fail
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_user_login_flow() {
    let state = common::setup_test_state();
    common::cleanup_test_data(&state.db_pool).await;
    let app = routes::create_router(state);

    // Register user first
    let register_payload = json!({
        "email": "logintest@example.com",
        "username": "logintest",
        "password": "SecurePass123!"
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Now login
    let login_payload = json!({
        "email": "logintest@example.com",
        "password": "SecurePass123!"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let auth_response: AuthResponseDto = serde_json::from_slice(&body).unwrap();

    assert_eq!(auth_response.user.email, "logintest@example.com");
    assert!(!auth_response.token.is_empty());
}

#[tokio::test]
async fn test_login_with_wrong_password() {
    let state = common::setup_test_state();
    common::cleanup_test_data(&state.db_pool).await;
    let app = routes::create_router(state);

    // Register user
    let register_payload = json!({
        "email": "wrongpass@example.com",
        "username": "wrongpass",
        "password": "CorrectPass123!"
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Login with wrong password
    let login_payload = json!({
        "email": "wrongpass@example.com",
        "password": "WrongPass123!"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_current_user() {
    let state = common::setup_test_state();
    common::cleanup_test_data(&state.db_pool).await;
    let app = routes::create_router(state);

    // Register and get token
    let register_payload = json!({
        "email": "currentuser@example.com",
        "username": "currentuser",
        "password": "SecurePass123!"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let auth_response: AuthResponseDto = serde_json::from_slice(&body).unwrap();
    let token = auth_response.token;

    // Get current user info
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/auth/me")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let user: UserResponseDto = serde_json::from_slice(&body).unwrap();

    assert_eq!(user.email, "currentuser@example.com");
    assert_eq!(user.username, "currentuser");
}

#[tokio::test]
async fn test_protected_endpoint_without_token() {
    let state = common::setup_test_state();
    let app = routes::create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/auth/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_protected_endpoint_with_invalid_token() {
    let state = common::setup_test_state();
    let app = routes::create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/auth/me")
                .header("authorization", "Bearer invalid_token_here")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_invalid_email_validation() {
    let state = common::setup_test_state();
    let app = routes::create_router(state);

    let register_payload = json!({
        "email": "not-an-email",
        "username": "testuser",
        "password": "SecurePass123!"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_short_password_validation() {
    let state = common::setup_test_state();
    let app = routes::create_router(state);

    let register_payload = json!({
        "email": "test@example.com",
        "username": "testuser",
        "password": "short"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
