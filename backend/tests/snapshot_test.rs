//! Snapshot testing examples using insta
//!
//! Snapshot tests help catch unintended changes to API responses
//! and data structures over time.

use backend::models::user::UserResponse;
use chrono::Utc;
use insta::{assert_json_snapshot, assert_yaml_snapshot};
use uuid::Uuid;

#[test]
fn test_user_response_snapshot() {
    let user = UserResponse {
        id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
        email: "test@example.com".to_string(),
        username: "testuser".to_string(),
        created_at: Utc::now().naive_utc(),
    };

    // Snapshot the JSON representation
    assert_json_snapshot!(user, {
        // Ignore the timestamp as it changes
        ".created_at" => "[timestamp]"
    });
}

#[test]
fn test_multiple_users_snapshot() {
    let users = vec![
        UserResponse {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            email: "user1@example.com".to_string(),
            username: "user1".to_string(),
            created_at: Utc::now().naive_utc(),
        },
        UserResponse {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
            email: "user2@example.com".to_string(),
            username: "user2".to_string(),
            created_at: Utc::now().naive_utc(),
        },
    ];

    assert_json_snapshot!(users, {
        "[].created_at" => "[timestamp]"
    });
}

#[test]
fn test_error_response_format() {
    use serde_json::json;

    let error_response = json!({
        "error_id": "123e4567-e89b-12d3-a456-426614174000",
        "error_code": "NOT_FOUND",
        "error": "User not found",
    });

    // Use YAML snapshot for better readability
    assert_yaml_snapshot!(error_response, {
        ".error_id" => "[uuid]"
    });
}

// Example: Snapshot testing for API endpoint responses
// This would typically be in an integration test
#[cfg(test)]
mod api_snapshots {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_health_check_response_format() {
        // Simulate a health check response
        let health_response = json!({
            "status": "healthy",
            "version": "0.1.0",
            "checks": {
                "database": {
                    "status": "healthy",
                    "details": {
                        "available_connections": 8,
                        "max_connections": 10
                    }
                },
                "memory": {
                    "status": "healthy",
                    "details": {
                        "memory_usage_mb": 45
                    }
                }
            }
        });

        assert_json_snapshot!(health_response, {
            ".checks.database.details.available_connections" => "[dynamic]",
            ".checks.memory.details.memory_usage_mb" => "[dynamic]"
        });
    }

    #[test]
    fn test_auth_response_format() {
        let auth_response = json!({
            "user": {
                "id": "00000000-0000-0000-0000-000000000001",
                "email": "test@example.com",
                "username": "testuser",
                "created_at": "2024-01-01T00:00:00Z"
            },
            "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
        });

        assert_json_snapshot!(auth_response, {
            ".user.created_at" => "[timestamp]",
            ".token" => "[jwt-token]"
        });
    }
}

// To use these snapshot tests:
//
// 1. Run tests: cargo test
// 2. Review snapshots: cat tests/snapshots/*.snap
// 3. Update snapshots: cargo test -- --update-snapshots
// 4. Accept new snapshots: cargo insta accept
//
// Snapshot files are stored in tests/snapshots/ and should be committed to git.
// They serve as documentation and regression tests for your API contracts.
