# Migration Guide

This guide helps you update existing code to use the new solo developer improvements.

## Overview

All improvements are **backward compatible**. You can adopt them gradually or all at once.

## 1. Update Error Handling

### Old Way

```rust
use crate::error::AppError;

pub async fn my_handler() -> Result<Response, AppError> {
    if user.is_none() {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    let result = some_operation().map_err(|e| AppError::InternalServerError {
        message: "Operation failed".to_string(),
        source: Some(Box::new(e)),
    })?;

    Ok(result)
}
```

### New Way

```rust
use crate::{error::AppError, bail_not_found, internal_error};

pub async fn my_handler() -> Result<Response, AppError> {
    if user.is_none() {
        bail_not_found!("User not found");
    }

    let result = some_operation()
        .map_err(|e| internal_error!("Operation failed", e))?;

    Ok(result)
}
```

### Available Macros

```rust
// Quick error creation
internal_error!("message")
internal_error!("message", source_error)

db_error!("message")
db_error!("message", source_error)

external_error!("service_name", source_error)

// Quick bail (early return)
bail!(error)
bail_not_found!("message")
bail_bad_request!("message")
bail_unauthorized!("message")
```

---

## 2. Update Service Access

### Old Way

```rust
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = state.auth_service.login(req).await?;
    Ok(Json(response))
}
```

### New Way (Recommended)

```rust
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Use services struct instead
    let response = state.services.auth.login(req).await?;
    Ok(Json(response))
}
```

**Note**: The old way still works! The new way is just cleaner.

### Service Access Pattern

```rust
// Old (still works)
state.auth_service
state.user_repository

// New (recommended)
state.services.auth
state.services.user_repo
```

---

## 3. Update Test Setup

### Old Way

```rust
#[tokio::test]
async fn test_my_feature() {
    let config = Config::default_test_config();
    let db_pool = db::create_pool(&config.database.url, 1).unwrap();

    // Missing services initialization!
    let state = AppState {
        config: Arc::new(config),
        db_pool,
    };

    // Test...
}
```

### New Way

```rust
use backend::tests::common::*;

#[tokio::test]
async fn test_my_feature() {
    // Complete state with all services
    let state = setup_test_state();

    // Test...
}
```

### Test Client Usage

```rust
use backend::tests::common::*;

#[tokio::test]
async fn test_api_endpoint() {
    let state = setup_test_state();
    let app = routes::create_router(state);
    let client = TestClient::new(app);

    let response = client
        .get("/api/v1/health")
        .await;

    response.assert_status(StatusCode::OK);

    let body: HealthResponse = response.json();
    assert_eq!(body.status, "healthy");
}
```

---

## 4. Add Snapshot Tests

### For Existing Tests

```rust
// Add this to Cargo.toml dev-dependencies
// insta = { version = "1.34", features = ["json", "yaml"] }

use insta::assert_json_snapshot;

#[test]
fn test_user_response_format() {
    let user = create_test_user();

    // Snapshot the response
    assert_json_snapshot!(user, {
        ".id" => "[uuid]",
        ".created_at" => "[timestamp]",
    });
}
```

### Update Snapshots

```bash
cargo test -- --update-snapshots
cargo insta accept
```

---

## 5. Use Dev Utilities

### For Debugging

```rust
#[cfg(debug_assertions)]
use backend::dev::*;

pub async fn debug_handler() {
    #[cfg(debug_assertions)]
    {
        // Pretty print JSON
        debug_json("Request", &request_data);

        // Time operations
        let timer = Timer::new("database query");
        let result = expensive_operation().await;
        // Timer prints duration on drop

        // Generate test data
        let email = fixtures::test_email(1);
        let uuid = fixtures::test_uuid();
    }
}
```

### For Testing

```rust
#[cfg(test)]
mod tests {
    use backend::dev::fixtures::*;

    #[test]
    fn test_with_fixtures() {
        let email = test_email(1);  // test.user1@example.com
        let username = test_username(1);  // testuser1
        let password = test_password();  // TestPassword123!
    }
}
```

---

## 6. Update Configuration Loading

### Old Way

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Requires .env file
    let config = Config::from_env()?;

    // ...
}
```

### New Way - Development

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Zero-config for development
    let config = if cfg!(debug_assertions) {
        Config::dev()  // Sensible defaults
    } else {
        Config::from_env()?  // Require config in production
    };

    // ...
}
```

### New Way - Testing

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_with_config() {
        // Use test config
        let config = Config::default_test_config();

        // Or dev config
        let config = Config::dev();
    }
}
```

---

## 7. Use Simple Logging Mode

### In Development

```bash
# Set environment variable
SIMPLE_LOGS=1 cargo run

# Or use dev-watch (automatically sets it)
just dev-watch
```

### Programmatically

```rust
// In main.rs
if std::env::var("SIMPLE_LOGS").is_ok() {
    tracing_config::init_simple_tracing()?;
} else {
    tracing_config::init_tracing("backend", &environment)?;
}
```

---

## 8. Update Justfile Usage

### Old Commands → New Commands

```bash
# Old
make run
# New
just run
# Better
just dev-watch

# Old
make test
# New
just test
# Better (for TDD)
just test-watch-quiet

# Old
make db-setup && make migrate && make seed && make run
# New
just fresh-start

# Old
make fmt && make lint && make test
# New
just check
# Better (faster)
just quick-check
```

---

## 9. API Testing Migration

### From Postman/Insomnia

1. Open `requests.http`
2. Copy your endpoints to this file:

```http
### My Endpoint
POST {{baseUrl}}/api/v1/my-endpoint
Content-Type: application/json
Authorization: Bearer {{authToken}}

{
  "data": "value"
}
```

3. Install REST Client extension in VS Code
4. Click "Send Request"

### From curl Scripts

```bash
# Old: curl script
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"pass123"}'

# New: Add to requests.http
### Login
POST {{baseUrl}}/api/v1/auth/login
Content-Type: application/json

{
  "email": "test@example.com",
  "password": "pass123"
}
```

---

## 10. Adding New Services

### Old Pattern

```rust
// In main.rs
let my_service = MyService::new(db_pool.clone());

let state = AppState {
    config: Arc::new(config),
    db_pool,
    auth_service: Arc::new(auth_service),
    my_service: Arc::new(my_service),  // Add here
};
```

### New Pattern

```rust
// In src/lib.rs - Services struct
pub struct Services {
    pub auth: Arc<AuthService>,
    pub user_repo: Arc<UserRepository>,
    pub my_service: Arc<MyService>,  // Add here
}

impl Services {
    pub fn new(db_pool: DbPool, config: &Config) -> Self {
        // Initialize all services
        let my_service = MyService::new(db_pool.clone());

        Self {
            auth: Arc::new(auth_service),
            user_repo: Arc::new(user_repository),
            my_service: Arc::new(my_service),  // Add here
        }
    }
}
```

Now accessible via `state.services.my_service` everywhere!

---

## Quick Migration Checklist

- [ ] Replace error creation with macros (`internal_error!`, etc.)
- [ ] Replace early returns with `bail_*!` macros
- [ ] Update service access to use `state.services.*`
- [ ] Update tests to use `setup_test_state()`
- [ ] Add snapshot tests for API endpoints
- [ ] Create `requests.http` file for API testing
- [ ] Add dev utilities for debugging
- [ ] Use `just dev-watch` for development
- [ ] Add new services to `Services` struct
- [ ] Use `Config::dev()` for zero-config start

---

## Step-by-Step Example

Let's migrate a complete handler:

### Before

```rust
// src/handlers/user.rs
use crate::{error::AppError, AppState};
use axum::{extract::State, Json};

pub async fn get_user(
    State(state): State<AppState>,
    user_id: String,
) -> Result<Json<User>, AppError> {
    let uuid = uuid::Uuid::parse_str(&user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let user = state.user_repository
        .find_by_id(uuid)
        .await
        .map_err(|e| AppError::DatabaseError {
            message: "Failed to fetch user".to_string(),
            source: Some(Box::new(e)),
        })?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(user))
}
```

### After

```rust
// src/handlers/user.rs
use crate::{error::AppError, AppState, bail_bad_request, bail_not_found, db_error};
use axum::{extract::State, Json};

#[tracing::instrument(skip(state))]
pub async fn get_user(
    State(state): State<AppState>,
    user_id: String,
) -> Result<Json<User>, AppError> {
    let uuid = uuid::Uuid::parse_str(&user_id)
        .map_err(|_| bail_bad_request!("Invalid user ID"))?;

    let user = state.services.user_repo
        .find_by_id(uuid)
        .await
        .map_err(|e| db_error!("Failed to fetch user", e))?
        .ok_or_else(|| bail_not_found!("User not found"))?;

    Ok(Json(user))
}
```

**Changes**:
- ✅ Added tracing instrumentation
- ✅ Used `bail_bad_request!` macro
- ✅ Used `db_error!` macro
- ✅ Used `bail_not_found!` macro
- ✅ Accessed service via `state.services.user_repo`

---

## Need Help?

- Check `docs/DEV_GUIDE.md` for examples
- See `CHEATSHEET.md` for quick reference
- Look at existing handlers for patterns
- Try the examples in `requests.http`

Happy migrating! 🚀
