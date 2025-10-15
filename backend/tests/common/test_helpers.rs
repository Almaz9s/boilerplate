use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use http_body_util::BodyExt;
use serde::de::DeserializeOwned;
use tower::ServiceExt;

/// Helper to make HTTP requests in tests
#[allow(dead_code)]
pub struct TestClient {
    app: axum::Router,
}

#[allow(dead_code)]
impl TestClient {
    pub fn new(app: axum::Router) -> Self {
        Self { app }
    }

    /// Make a GET request
    pub async fn get(&self, uri: &str) -> TestResponse {
        self.request(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
    }

    /// Make a GET request with authorization header
    pub async fn get_with_auth(&self, uri: &str, token: &str) -> TestResponse {
        self.request(
            Request::builder()
                .uri(uri)
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
    }

    /// Make a POST request with JSON body
    pub async fn post<T: serde::Serialize>(&self, uri: &str, body: &T) -> TestResponse {
        let body_bytes = serde_json::to_vec(body).unwrap();
        self.request(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(body_bytes))
                .unwrap(),
        )
        .await
    }

    /// Make a PUT request with JSON body
    pub async fn put<T: serde::Serialize>(&self, uri: &str, body: &T) -> TestResponse {
        let body_bytes = serde_json::to_vec(body).unwrap();
        self.request(
            Request::builder()
                .method("PUT")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(body_bytes))
                .unwrap(),
        )
        .await
    }

    /// Make a DELETE request
    pub async fn delete(&self, uri: &str) -> TestResponse {
        self.request(
            Request::builder()
                .method("DELETE")
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
    }

    /// Make a request with custom headers
    pub async fn request(&self, request: Request<Body>) -> TestResponse {
        let response = self
            .app
            .clone()
            .oneshot(request)
            .await
            .expect("Failed to execute request");

        TestResponse::new(response).await
    }
}

/// Test response wrapper with convenient assertion methods
#[allow(dead_code)]
pub struct TestResponse {
    pub status: StatusCode,
    pub body: Vec<u8>,
}

#[allow(dead_code)]
impl TestResponse {
    async fn new(response: Response<Body>) -> Self {
        let status = response.status();
        let body = response
            .into_body()
            .collect()
            .await
            .expect("Failed to read body")
            .to_bytes()
            .to_vec();

        Self { status, body }
    }

    /// Assert response status code
    pub fn assert_status(&self, expected: StatusCode) -> &Self {
        assert_eq!(
            self.status, expected,
            "Expected status {}, got {}. Body: {}",
            expected,
            self.status,
            String::from_utf8_lossy(&self.body)
        );
        self
    }

    /// Deserialize JSON response body
    pub fn json<T: DeserializeOwned>(&self) -> T {
        serde_json::from_slice(&self.body)
            .unwrap_or_else(|e| panic!("Failed to deserialize JSON: {}. Body: {:?}", e, String::from_utf8_lossy(&self.body)))
    }

    /// Get response body as string
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }

    /// Assert response contains text
    pub fn assert_contains(&self, text: &str) -> &Self {
        let body = self.text();
        assert!(
            body.contains(text),
            "Expected body to contain '{}', got: {}",
            text,
            body
        );
        self
    }

    /// Assert JSON response equals expected value
    pub fn assert_json_eq<T: DeserializeOwned + PartialEq + std::fmt::Debug>(&self, expected: T) -> &Self {
        let actual: T = self.json();
        assert_eq!(actual, expected, "JSON response did not match");
        self
    }

    /// Print response for debugging (useful during test development)
    #[allow(dead_code)]
    pub fn dump(&self) {
        eprintln!("=== Response Debug ===");
        eprintln!("Status: {}", self.status);
        eprintln!("Body: {}", String::from_utf8_lossy(&self.body));
        eprintln!("====================");
    }

    /// Assert response body is empty
    pub fn assert_empty(&self) -> &Self {
        assert!(
            self.body.is_empty(),
            "Expected empty body, got: {}",
            String::from_utf8_lossy(&self.body)
        );
        self
    }

    /// Get JSON value at path (useful for partial assertions)
    #[allow(dead_code)]
    pub fn json_path(&self, path: &str) -> serde_json::Value {
        let json: serde_json::Value = self.json();
        path.split('.')
            .fold(json, |acc, key| acc.get(key).unwrap_or(&serde_json::Value::Null).clone())
    }
}

/// Create a test JWT token for authentication tests
#[allow(dead_code)]
pub fn create_test_jwt(user_id: &str, secret: &str) -> String {
    use chrono::{Duration, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        exp: i64,
    }

    let claims = Claims {
        sub: user_id.to_string(),
        exp: (Utc::now() + Duration::hours(1)).timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .expect("Failed to create test JWT")
}

// ============================================================================
// Test Fixtures
// ============================================================================

use backend::models::user::User;
use chrono::Utc;
use fake::Fake;
use uuid::Uuid;

/// Generate a fake user for testing
///
/// # Example
/// ```
/// let user = fake_user();
/// assert!(!user.email.is_empty());
/// ```
#[allow(dead_code)]
pub fn fake_user() -> User {
    User {
        id: Uuid::new_v4(),
        email: fake::faker::internet::en::SafeEmail().fake(),
        username: fake::faker::internet::en::Username().fake(),
        password_hash: "$argon2id$v=19$m=19456,t=2,p=1$test$hash".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    }
}

/// Generate multiple fake users for testing
///
/// # Example
/// ```
/// let users = fake_users(5);
/// assert_eq!(users.len(), 5);
/// ```
#[allow(dead_code)]
pub fn fake_users(count: usize) -> Vec<User> {
    (0..count).map(|_| fake_user()).collect()
}

/// Generate a fake user with specific email
#[allow(dead_code)]
pub fn fake_user_with_email(email: &str) -> User {
    User {
        email: email.to_string(),
        ..fake_user()
    }
}

/// Generate a fake user with specific username
#[allow(dead_code)]
pub fn fake_user_with_username(username: &str) -> User {
    User {
        username: username.to_string(),
        ..fake_user()
    }
}
