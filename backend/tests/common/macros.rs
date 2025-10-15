/// Macro to quickly create test requests
///
/// # Examples
///
/// ```ignore
/// let response = test_request!(client, post, "/api/v1/auth/register", json_body);
/// let response = test_request!(client, get, "/api/v1/health");
/// ```
#[macro_export]
macro_rules! test_request {
    ($client:expr, post, $path:expr, $body:expr) => {{
        $client.post($path).json($body).await
    }};
    ($client:expr, post, $path:expr, $body:expr, $token:expr) => {{
        $client
            .post($path)
            .header("authorization", format!("Bearer {}", $token))
            .json($body)
            .await
    }};
    ($client:expr, get, $path:expr) => {{
        $client.get($path).await
    }};
    ($client:expr, get, $path:expr, $token:expr) => {{
        $client
            .get($path)
            .header("authorization", format!("Bearer {}", $token))
            .await
    }};
    ($client:expr, put, $path:expr, $body:expr, $token:expr) => {{
        $client
            .put($path)
            .header("authorization", format!("Bearer {}", $token))
            .json($body)
            .await
    }};
    ($client:expr, delete, $path:expr, $token:expr) => {{
        $client
            .delete($path)
            .header("authorization", format!("Bearer {}", $token))
            .await
    }};
}

/// Macro to assert response status codes with better error messages
///
/// # Examples
///
/// ```ignore
/// assert_status!(response, StatusCode::OK);
/// assert_status!(response, StatusCode::CREATED);
/// ```
#[macro_export]
macro_rules! assert_status {
    ($response:expr, $expected:expr) => {{
        let status = $response.status();
        if status != $expected {
            let body = $response.text().await.unwrap_or_default();
            panic!(
                "Expected status {}, got {}. Response body: {}",
                $expected, status, body
            );
        }
    }};
}

/// Macro to extract JSON from response with type inference
///
/// # Examples
///
/// ```ignore
/// let user: UserResponse = json_response!(response);
/// ```
#[macro_export]
macro_rules! json_response {
    ($response:expr) => {{
        $response.json().await.expect("Failed to parse JSON response")
    }};
}

/// Macro to create test users with defaults
///
/// # Examples
///
/// ```ignore
/// let user = test_user!("test@example.com", "testuser", "password123");
/// let user = test_user!(email: "custom@example.com"); // Uses defaults for username and password
/// ```
#[macro_export]
macro_rules! test_user {
    ($email:expr, $username:expr, $password:expr) => {{
        backend::models::dto::RegisterRequestDto {
            email: $email.to_string(),
            username: $username.to_string(),
            password: $password.to_string(),
        }
    }};
    (email: $email:expr) => {{
        backend::models::dto::RegisterRequestDto {
            email: $email.to_string(),
            username: "testuser".to_string(),
            password: "TestPass123!".to_string(),
        }
    }};
    () => {{
        backend::models::dto::RegisterRequestDto {
            email: format!("test{}@example.com", uuid::Uuid::new_v4()),
            username: format!("user{}", uuid::Uuid::new_v4()),
            password: "TestPass123!".to_string(),
        }
    }};
}
