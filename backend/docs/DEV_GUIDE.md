# Developer Guide

Quick reference for solo developers working with this boilerplate.

## 🚀 Quick Start

```bash
# Zero config start (uses sensible defaults)
just fresh-start

# Or with environment variables
cp .env.example .env
just setup
just run
```

## 📝 Common Workflows

### Daily Development

```bash
# Clean, fast dev mode with simple logs
just dev-watch

# Quick check before commit
just quick-check

# Full check (format, lint, test)
just check
```

### Database Operations

```bash
# Fresh start with seed data
just db-fresh

# Create migration
just db-gen create_my_table

# Reset everything and restart server
just dev-reset
```

### Testing

```bash
# Run all tests
just test

# Watch tests on file changes
just test-watch-quiet

# Quick smoke test
just smoke

# Update snapshots
cargo test -- --update-snapshots
cargo insta accept
```

## 🛠️ Development Features

### Simple Logging Mode

Set `SIMPLE_LOGS=1` for cleaner output without OpenTelemetry:

```bash
SIMPLE_LOGS=1 cargo run
# or
just dev-watch
```

### Dev Endpoints (Debug Builds Only)

These endpoints are only available in `--debug` builds:

```http
GET /dev/state          # View app state and config
GET /dev/health         # Simple health check
POST /dev/echo          # Echo request body
POST /dev/token         # Generate test JWT
GET /dev/error/:type    # Simulate errors
```

### API Testing with REST Client

Use `requests.http` with VS Code REST Client extension:

1. Install REST Client extension
2. Open `requests.http`
3. Click "Send Request" above any request
4. Variables like `@authToken` are extracted automatically

## 🎯 Error Handling Shortcuts

Use convenience macros for cleaner error handling:

```rust
use backend::{bail_not_found, internal_error, db_error};

// Quick bail with custom error
if user.is_none() {
    bail_not_found!("User not found");
}

// Create errors with context
Err(internal_error!("Failed to process", err))
Err(db_error!("Query failed", err))
```

## 🧪 Test Helpers

```rust
use backend::tests::common::*;

// Complete test state with all services
let state = setup_test_state();

// Test HTTP client
let client = TestClient::new(app);
let response = client.get("/api/v1/health").await;
response.assert_status(StatusCode::OK);

// Generate test JWT
let token = create_test_jwt("user-id", "secret");
```

## 📊 Debugging

### Dev Utilities

```rust
use backend::dev::*;

// Pretty print JSON
debug_json("My Data", &some_struct);

// Time operations
let timer = Timer::new("database query");
// ... operation ...
// Timer prints on drop

// Generate test data
use backend::dev::fixtures::*;
let email = test_email(1);  // test.user1@example.com
let uuid = test_uuid();     // Deterministic UUID
```

### Profile Build Times

```bash
just profile-build
# Opens HTML report with build timing breakdown
```

### View Environment

```bash
just show-env
```

## 🏗️ Architecture

### AppState Structure

```rust
AppState {
    config: Arc<Config>,
    db_pool: DbPool,
    services: Services {
        auth: Arc<AuthService>,
        user_repo: Arc<UserRepository>,
    }
}
```

Access services via `state.services.auth` or legacy `state.auth_service`.

### Configuration

```rust
// Zero-config dev mode
let config = Config::dev();

// From environment variables
let config = Config::from_env()?;

// With secrets manager (optional features)
let config = Config::from_secrets().await?;
```

## 📦 Adding New Features

### Add a New Handler

```rust
// src/handlers/my_feature.rs
#[tracing::instrument(skip(state))]
pub async fn my_handler(
    State(state): State<AppState>,
) -> Result<Json<Response>, AppError> {
    // Implementation
}
```

### Add to Routes

```rust
// src/routes.rs
.route("/my-feature", get(handlers::my_feature::my_handler))
```

### Add a Service

```rust
// src/services/my_service.rs
pub struct MyService {
    repo: MyRepository,
}

// src/lib.rs - Services struct
pub struct Services {
    pub auth: Arc<AuthService>,
    pub my_service: Arc<MyService>,  // Add here
}
```

## 📸 Snapshot Testing

Use `insta` for API response regression testing:

```rust
#[test]
fn test_api_response() {
    let response = json!({
        "data": "value",
        "timestamp": "2024-01-01"
    });

    assert_json_snapshot!(response, {
        ".timestamp" => "[timestamp]"  // Ignore dynamic fields
    });
}
```

Update snapshots:
```bash
cargo test -- --update-snapshots
cargo insta accept
```

## 🔍 Common Issues

### Build Errors

```bash
# Clean rebuild
just clean
just build
```

### Database Connection Issues

```bash
# Check database is running
just docker-up

# Test connection
just db-info
```

### Port Already in Use

Change port in `.env`:
```bash
PORT=8081
```

## 📚 Additional Resources

- OpenAPI docs: http://localhost:2999/swagger-ui
- Metrics: http://localhost:2999/metrics
- Health check: http://localhost:2999/api/v1/health
- Dev state: http://localhost:2999/dev/state (debug builds only)

## 💡 Tips

1. Use `just dev-watch` for fastest development loop
2. Keep `requests.http` open for quick API testing
3. Use snapshot tests for API contract testing
4. Profile builds with `just profile-build` if slow
5. Use dev endpoints to generate test tokens
6. Set `SIMPLE_LOGS=1` for cleaner console output
7. Use error macros to reduce boilerplate
8. Run `just smoke` for quick validation

## 🎨 Code Style

```bash
# Format code (automatic)
just fmt

# Check without modifying
just fmt-check

# Lint
just lint
```

All code is formatted with `rustfmt` and linted with `clippy`.
