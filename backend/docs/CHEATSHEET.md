# Quick Reference Cheat Sheet

## 🚀 Getting Started

```bash
just fresh-start    # Zero-config start with fresh DB
just dev-watch      # Watch mode with clean logs
```

## 📝 Common Commands

```bash
# Development
just dev-watch              # Auto-reload with simple logs
just dev-reset              # Fresh DB + restart server
just quick-check            # Fast validation (format, check, test)

# Database
just db-fresh               # Drop, migrate, seed
just db-migrate             # Run migrations
just db-gen create_users    # Create migration

# Testing
just test                   # Run all tests
just test-watch-quiet       # Watch tests
just smoke                  # Quick smoke test

# Quality
just fmt                    # Format code
just lint                   # Run clippy
just check                  # Format + lint + test
```

## 🛠️ Dev Endpoints (Debug Only)

```bash
curl localhost:2999/dev/state              # View app state
curl -X POST localhost:2999/dev/token      # Generate test JWT
curl localhost:2999/dev/error/not_found    # Test error handling
```

## 📋 API Testing

Use `requests.http` with REST Client extension in VS Code.

Common workflow:
1. Send register request → copy token
2. Send login request → token auto-extracted
3. Send authenticated requests with `{{authToken}}`

## 🔧 Error Handling

```rust
use backend::{bail_not_found, internal_error};

// Quick bail
bail_not_found!("User not found");

// With context
Err(internal_error!("Failed to process", err))
```

## 🧪 Testing

```rust
use backend::tests::common::*;

let state = setup_test_state();
let client = TestClient::new(app);
let resp = client.get("/health").await;
resp.assert_status(StatusCode::OK);
```

## 📸 Snapshot Testing

```rust
assert_json_snapshot!(response, {
    ".timestamp" => "[timestamp]"
});
```

Update: `cargo test -- --update-snapshots && cargo insta accept`

## 🐛 Debugging

```rust
use backend::dev::*;

debug_json("Data", &my_struct);
let _timer = Timer::new("operation");
```

## ⚙️ Configuration

```rust
// Zero-config
let config = Config::dev();

// From environment
let config = Config::from_env()?;
```

## 🔍 Logging

```bash
# Simple mode (clean output)
SIMPLE_LOGS=1 cargo run

# Verbose mode
RUST_LOG=trace cargo run

# Specific module
RUST_LOG=backend::handlers=debug cargo run
```

## 📊 Performance

```bash
just profile-build    # Build timing analysis
just bench            # Run benchmarks (if added)
```

## 🔒 Services

```rust
// Access via services struct
state.services.auth.login(req).await?;
state.services.user_repo.find_by_id(id).await?;
```

## 📚 Documentation

- `docs/DEV_GUIDE.md` - Complete developer guide
- `docs/IMPROVEMENTS.md` - What changed and why
- `requests.http` - API examples
- `http://localhost:2999/swagger-ui` - Interactive API docs

## 💡 Pro Tips

1. Keep `requests.http` open for quick API tests
2. Use `just dev-watch` for fastest iteration
3. Run `just smoke` before committing
4. Use `/dev/token` to generate test tokens
5. Set `SIMPLE_LOGS=1` for cleaner logs
6. Use error macros everywhere
7. Take snapshots of API responses

## 🆘 Troubleshooting

```bash
just show-env       # View current configuration
just db-info        # Check database connection
just clean          # Clean rebuild
just docker-up      # Ensure database is running
```

## 🎯 Quick Examples

### Add New Handler

```rust
// src/handlers/my_feature.rs
#[tracing::instrument(skip(state))]
pub async fn my_handler(
    State(state): State<AppState>,
) -> Result<Json<Response>, AppError> {
    Ok(Json(response))
}

// src/routes.rs
.route("/my-feature", get(handlers::my_feature::my_handler))
```

### Add New Service

```rust
// src/services/my_service.rs
pub struct MyService { /* ... */ }

// src/lib.rs - Services
pub struct Services {
    pub auth: Arc<AuthService>,
    pub my_service: Arc<MyService>,
}
```

### Write Snapshot Test

```rust
#[test]
fn test_api_response() {
    let response = json!({"status": "ok"});
    assert_json_snapshot!(response);
}
```

## 🔗 External Resources

- Axum docs: https://docs.rs/axum
- Diesel docs: https://diesel.rs/guides/
- Tokio docs: https://tokio.rs/
- Tracing docs: https://docs.rs/tracing
