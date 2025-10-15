# Backend Boilerplate

A production-ready Rust backend boilerplate using Axum web framework and Diesel ORM with full async support.

**✨ Optimized for solo developers** - Zero-config start, minimal boilerplate, maximum velocity.

## 📚 Documentation

- **[Quick Start](#quick-start)** - Get running in 2 minutes
- **[docs/RECIPES.md](docs/RECIPES.md)** - Copy-paste solutions for common tasks
- **[docs/CHEATSHEET.md](docs/CHEATSHEET.md)** - Quick reference for daily use
- **[docs/DEV_GUIDE.md](docs/DEV_GUIDE.md)** - Complete developer guide
- **[docs/MIGRATION_GUIDE.md](docs/MIGRATION_GUIDE.md)** - Updating existing code

## Features

### Core Framework
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum) - Fast, ergonomic async web framework
- **ORM**: [Diesel](https://diesel.rs/) with full async support via `diesel-async` - Safe, extensible ORM and query builder
- **Database**: PostgreSQL with async connection pooling (deadpool)
- **Authentication**: JWT support with argon2 password hashing
- **Validation**: Request validation using `validator`

### Production-Ready Features
- **Error Handling**: Comprehensive error types with structured error codes, error IDs, and proper HTTP responses
- **API Documentation**: OpenAPI/Swagger UI with utoipa - accessible at `/swagger-ui`
- **Pagination**: Built-in pagination utilities with Diesel query integration
- **Transactions**: Database transaction helpers for atomic operations
- **Background Jobs**: Cron-based job scheduler with tokio-cron-scheduler

### Middleware & Security
- **Request ID Tracing**: Unique IDs for all requests with correlation support
- **CORS**: Configurable CORS support
- **Compression**: Gzip compression for responses
- **Timeouts**: Configurable request timeouts

### Observability
- **Distributed Tracing**: Full OpenTelemetry integration with span-based request tracing
  - Automatic span creation for all HTTP requests
  - Method-level instrumentation for handlers, services, and database operations
  - Span context propagation through the entire call stack
  - Request correlation with unique trace IDs
  - Detailed timing and performance metrics per operation
- **Logging**: Structured logging with environment-aware formatting
  - JSON format in production for log aggregation
  - Pretty format in development for readability
  - Automatic span context inclusion in all logs
  - Hierarchical logging with trace/debug/info/warn/error levels
- **Metrics**: Prometheus metrics export with HTTP request tracking at `/metrics`
- **Health Checks**: Comprehensive health monitoring at `/api/v1/health`
  - Database connectivity with pool statistics
  - Memory usage monitoring
  - Subsystem status reporting

### Developer Experience
- **Configuration**: Environment variable configuration with optional secrets management
  - AWS Secrets Manager support (optional feature)
  - HashiCorp Vault support (optional feature)
  - Automatic fallback to environment variables
- **Graceful Shutdown**: Proper signal handling for clean shutdowns
- **Testing**: Comprehensive test infrastructure
  - Test helpers and utilities
  - HTTP client for integration tests
  - Database container support with testcontainers
  - Mock state helpers
- **Code Quality**: rustfmt, clippy, cargo-deny configurations

## Prerequisites

- Rust 1.80 or later (stable)
- PostgreSQL 14 or later
- Docker and Docker Compose (optional)

## Quick Start

### Zero-Config Start (Fastest)

```bash
# Uses sensible defaults - no .env needed!
just fresh-start
```

Server starts at `http://localhost:8080` with development defaults.

### Traditional Start

1. **Clone and navigate to the project**:
   ```bash
   cd 2003/boilerplate/backend
   ```

2. **Install development tools** (optional):
   ```bash
   just install-tools
   ```

3. **Set up environment variables** (optional):
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

4. **Start PostgreSQL** (if using Docker):
   ```bash
   just docker-up
   ```

5. **Set up the database**:
   ```bash
   just db-setup
   ```

6. **Run the application**:
   ```bash
   just run
   ```

The server will start at `http://localhost:8080`.

## Development

### Available Commands

See full list with `just --list`. Common commands:

```bash
# Development workflow
just dev-watch         # Watch and auto-restart with clean logs
just dev-reset         # Fresh database + restart server
just quick-check       # Fast check (format, compile, test)

# Building and testing
just build            # Build the project
just test             # Run tests
just test-watch-quiet # Watch tests on file changes
just smoke            # Quick smoke test

# Database operations
just db-fresh         # Fresh database with seed data
just db-migrate       # Run migrations
just db-gen name      # Create new migration

# Code quality
just fmt              # Format code
just lint             # Run clippy
just check            # Run all checks (fmt, clippy, test)
just audit            # Security audit
```

💡 **Tip**: Use `just dev-watch` for fastest development iteration.

### Project Structure

```
backend/
├── src/
│   ├── config/          # Configuration management (no dotenv)
│   ├── db/              # Database layer
│   │   ├── mod.rs       # Pool creation and connection helpers
│   │   ├── pagination.rs# Pagination utilities for Diesel queries
│   │   ├── schema.rs    # Generated schema from Diesel
│   │   └── transaction.rs# Transaction helpers
│   ├── docs.rs          # OpenAPI/Swagger documentation definitions
│   ├── error.rs         # Error types with codes and IDs
│   ├── handlers/        # HTTP request handlers with utoipa annotations
│   ├── jobs/            # Background job system
│   │   ├── mod.rs       # Scheduler initialization
│   │   └── tasks.rs     # Background task implementations
│   ├── middleware/      # Custom middleware
│   │   ├── auth.rs      # JWT authentication
│   │   ├── logging.rs   # Request/response logging
│   │   ├── rate_limit.rs# Rate limiting configuration
│   │   └── request_id.rs# Request ID generation and tracing
│   ├── metrics.rs       # Prometheus metrics
│   ├── models/          # Data models with ToSchema for OpenAPI
│   ├── routes.rs        # Route definitions with middleware stack
│   ├── services/        # Business logic (async)
│   └── main.rs          # Application entry point with graceful shutdown
├── migrations/          # Database migrations
├── tests/               # Integration tests
│   ├── common/          # Test utilities
│   │   ├── mod.rs       # Re-exports
│   │   ├── test_db.rs   # Test database helpers with testcontainers
│   │   └── test_helpers.rs# HTTP test client and JWT helpers
│   └── *.rs             # Test files
├── Cargo.toml           # Dependencies and project metadata
├── .env.example         # Example environment variables
├── diesel.toml          # Diesel configuration
├── clippy.toml          # Clippy linting rules
├── deny.toml            # Cargo-deny configuration
└── Makefile             # Build automation
```

## Database Migrations

### Create a new migration:
```bash
make migrate-generate name=create_users_table
```

### Run migrations:
```bash
make migrate
```

### Revert last migration:
```bash
make migrate-revert
```

### Reset database:
```bash
make db-reset
```

## API Endpoints

### API Documentation
```
GET /swagger-ui
```

Interactive Swagger UI for exploring and testing the API. The OpenAPI spec is available at `/api-docs/openapi.json`.

### Health Check
```
GET /api/v1/health
```

Returns comprehensive health status including:
- Overall service status (healthy/degraded/unhealthy)
- Database connectivity and pool statistics
- Memory usage metrics
- Version information

Example response:
```json
{
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
}
```

### Metrics
```
GET /metrics
```

Returns Prometheus-formatted metrics including:
- HTTP request counts by method, path, and status
- Request duration histograms
- Custom business metrics

### Authentication
```
POST /api/v1/auth/register
POST /api/v1/auth/login
GET /api/v1/auth/me
```

All endpoints include request ID tracing via `x-request-id` header for correlation.

## Configuration

Configuration is managed through environment variables. See `.env.example` for all available options:

- `HOST`: Server host (default: 0.0.0.0)
- `PORT`: Server port (default: 8080)
- `DATABASE_URL`: PostgreSQL connection string
- `DATABASE_POOL_SIZE`: Connection pool size (default: 10)
- `JWT_SECRET`: Secret key for JWT signing
- `JWT_EXPIRATION_HOURS`: Token expiration time (default: 24)
- `CORS_ALLOWED_ORIGINS`: Comma-separated list of allowed origins
- `REQUEST_TIMEOUT`: Request timeout in seconds (default: 30)
- `RUST_LOG`: Logging level configuration

### Secrets Management

For production deployments, sensitive configuration values can be loaded from external secret providers:

- **AWS Secrets Manager**: Enable with `--features aws-secrets`
- **HashiCorp Vault**: Enable with `--features vault-secrets`

See [info/SECRETS_MANAGEMENT.md](info/SECRETS_MANAGEMENT.md) for detailed configuration and usage instructions.

## Testing

### Run all tests:
```bash
make test
```

### Run tests with coverage:
```bash
make test-coverage
```

Coverage reports will be generated in the `coverage/` directory.

## Code Quality

### Format code:
```bash
make fmt
```

### Run linter:
```bash
make lint
```

### Run all checks:
```bash
make check
```

### Security audit:
```bash
make audit
```

### Dependency checks:
```bash
make deny
```

## Production Deployment

1. Build the release binary:
   ```bash
   make build
   ```

2. The optimized binary will be in `target/release/backend`

3. Set production environment variables:
   - Set `ENVIRONMENT=production`
   - Use strong `JWT_SECRET`
   - Configure appropriate `CORS_ALLOWED_ORIGINS`
   - Set proper database credentials

4. Run migrations on production database:
   ```bash
   diesel migration run --database-url=$PRODUCTION_DATABASE_URL
   ```

## Solo Developer Features

This boilerplate includes several enhancements specifically for solo developers:

### 🚀 Zero-Config Development

```rust
// Start without any .env setup
let config = Config::dev();  // Sensible defaults
```

No need to configure anything to start coding - just run `just fresh-start`!

### 🔧 Smart Logging Defaults

Development mode uses clean, simple logging by default. Production uses full OpenTelemetry:

```bash
# Development (simple logs by default)
cargo run

# Enable full OpenTelemetry tracing in dev
OTEL=1 cargo run
# or
just dev-watch-otel
```

### 🛠️ Dev-Only Endpoints

Debug builds include helpful endpoints at `/dev/*`:

- `GET /dev/state` - View app state, pool stats with utilization monitoring
- `POST /dev/token` - Generate test JWT tokens
- `POST /dev/echo` - Test request/response
- `GET /dev/error/:type` - Simulate error scenarios
- `GET /dev/health` - Simple dev health check

These are **automatically removed** in release builds.

### 📝 Error Handling Shortcuts

Convenience macros reduce boilerplate:

```rust
use backend::{bail_not_found, internal_error, db_error};

// Instead of verbose error construction:
if user.is_none() {
    bail_not_found!("User not found");
}

// Quick error creation with context:
Err(internal_error!("Operation failed", source_error))
```

### 🧪 Complete Test Helpers

```rust
use backend::tests::common::*;

// Fully configured test state
let state = setup_test_state();

// Easy HTTP testing with rich assertions
let client = TestClient::new(app);
let resp = client.get("/api/v1/health").await;
resp.assert_status(StatusCode::OK)
    .assert_contains("healthy");

// Debug responses during test development
resp.dump();  // Prints status and body

// Auth requests
let token = create_test_jwt("user123", "secret");
let resp = client.get_with_auth("/api/v1/auth/me", &token).await;
```

### 📸 Snapshot Testing

Built-in snapshot testing with `insta`:

```rust
assert_json_snapshot!(response, {
    ".timestamp" => "[timestamp]",  // Ignore dynamic fields
});
```

Update snapshots: `cargo test -- --update-snapshots && cargo insta accept`

### 📋 REST Client Integration

Use `requests.http` for quick API testing in VS Code:

1. Install REST Client extension
2. Open `requests.http`
3. Click "Send Request" to test endpoints
4. Variables auto-extracted from responses

### 🏗️ Clean Services Architecture

Simplified state management with no duplicate fields:

```rust
AppState {
    config: Arc<Config>,
    db_pool: DbPool,
    services: Services {
        auth: Arc<AuthService>,
        user_repo: Arc<UserRepository>,
    }
}

// Access services cleanly
state.services.auth.login(request).await?;
```

### 📚 Comprehensive Dev Guide

See [`docs/DEV_GUIDE.md`](docs/DEV_GUIDE.md) for:
- Common workflows
- Debugging tips
- Architecture overview
- Code examples
- Troubleshooting

## Key Improvements Over Standard Boilerplates

### 1. Fully Async Database Layer
- Uses `diesel-async` with `deadpool` for async connection pooling
- Transaction helpers for atomic operations
- No blocking operations in async context
- Better performance under high load
- Built-in pagination utilities with query integration

### 2. Production-Grade Observability
- **OpenTelemetry Tracing**: Full distributed tracing with automatic span instrumentation
  - Span-based request tracing for every API call
  - Automatic context propagation across async boundaries
  - Method-level spans with detailed attributes
  - Integration-ready with Jaeger, Tempo, or any OTLP-compatible backend
- **Structured Logging**: Environment-aware (JSON in prod, pretty in dev)
  - Logs automatically include span context
  - Trace/span IDs for correlation
  - File, line numbers, and thread IDs
- **Prometheus Metrics**: Full HTTP request tracking at `/metrics`
- **Enhanced Health Checks**: Multi-subsystem monitoring with pool stats
- Ready for production observability stack (Grafana, Loki, Tempo, Prometheus)

### 3. Comprehensive Error Handling
- Structured error codes (e.g., `DATABASE_ERROR`, `NOT_FOUND`)
- Unique error IDs (UUID) for debugging and tracking
- Proper error context logging with tracing
- User-friendly error messages vs internal logging
- Prevents information leakage in responses

### 4. Security
- **JWT Authentication**: Industry-standard tokens with configurable expiration
- **Password Security**: Argon2 hashing with salt
- **Secrets Management**: Optional AWS Secrets Manager and Vault integration
- **CORS**: Configurable origin restrictions

### 5. Developer Experience
- **OpenAPI/Swagger**: Auto-generated API docs at `/swagger-ui`
- **Test Infrastructure**: Helpers, mocks, and testcontainers integration
- **Background Jobs**: Cron scheduler for recurring tasks
- **Configuration**: Pure env vars, no runtime file loading
- **Code Quality**: Pre-configured clippy, rustfmt, cargo-deny

### 6. Operational Excellence
- **Graceful Shutdown**: Handles SIGTERM/SIGINT with job cleanup
- **Database Migrations**: Diesel CLI integration
- **Production Builds**: Optimized with LTO and strip
- **Containerization-Ready**: No file dependencies at runtime

## Security Considerations

- Always use HTTPS in production
- Rotate JWT secrets regularly
- Use strong database passwords
- Keep dependencies updated (run `make audit` regularly)
- Review security advisories with `cargo deny check advisories`
- Never commit `.env` files
- Use prepared statements (Diesel does this by default)
- Error responses don't leak internal details
- Request IDs help with security audit trails

## Performance Tips

- Async connection pooling is configured by default
- Enable compression for responses (configured by default)
- Monitor database query performance via logs
- Use indexes appropriately in migrations
- Profile with `cargo flamegraph` if needed
- Health check validates DB connectivity for load balancer integration

## Usage Examples

### Using Pagination in Queries

```rust
use crate::db::pagination::Paginate;
use crate::models::{PaginationParams, PaginatedResponse};

// In your handler
async fn list_users(
    Query(params): Query<PaginationParams>,
    State(state): State<AppState>,
) -> Result<Json<PaginatedResponse<User>>, AppError> {
    let mut conn = db::get_connection(&state.db_pool).await?;

    let result = users::table
        .paginate(params.page)
        .per_page(params.per_page)
        .load_and_count(&mut conn)
        .await?;

    Ok(Json(result))
}
```

### Using Database Transactions

```rust
use crate::db::transaction::with_transaction;

async fn transfer_funds(
    from_id: i32,
    to_id: i32,
    amount: i32,
) -> Result<(), AppError> {
    let mut conn = db::get_connection(&pool).await?;

    with_transaction(&mut conn, |conn| {
        Box::pin(async move {
            // Deduct from sender
            diesel::update(accounts::table.find(from_id))
                .set(balance.eq(balance - amount))
                .execute(conn)
                .await?;

            // Add to receiver
            diesel::update(accounts::table.find(to_id))
                .set(balance.eq(balance + amount))
                .execute(conn)
                .await?;

            Ok(())
        })
    }).await
}
```

### Adding Custom Background Jobs

```rust
// In src/jobs/mod.rs
let custom_job = Job::new_async("0 0 2 * * *", move |_uuid, _lock| {
    let state = state_clone.clone();
    Box::pin(async move {
        tracing::info!("Running custom nightly job");
        if let Err(e) = tasks::custom_task(state).await {
            tracing::error!("Custom task failed: {}", e);
        }
    })
})?;
scheduler.add(custom_job).await?;
```

### Distributed Tracing

The boilerplate includes full OpenTelemetry tracing with automatic instrumentation:

#### How Tracing Works

Every HTTP request automatically creates a parent span with:
- HTTP method, route, version, user agent
- Request and response timing
- Status codes and error states
- Unique trace and span IDs

All handlers, services, and database operations create child spans that inherit context:

```rust
// Handlers are automatically instrumented
#[tracing::instrument(name = "my_handler", skip(state), fields(user_id = %user.id))]
pub async fn my_handler(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<Response>, AppError> {
    tracing::info!("Processing request");

    // Service calls create nested spans
    let result = my_service.process(&user.id).await?;

    tracing::debug!(result_count = result.len(), "Request completed");
    Ok(Json(result))
}
```

#### Viewing Traces

By default, traces are exported to stdout for development. To integrate with a tracing backend:

**For Jaeger:**
```toml
# In Cargo.toml, replace opentelemetry-stdout with:
opentelemetry-jaeger = "0.21"
```

```rust
// In src/tracing_config.rs, replace SpanExporter with:
use opentelemetry_jaeger::new_agent_pipeline;

let tracer = new_agent_pipeline()
    .with_service_name("backend")
    .install_simple()?;
```

**For OTLP (Tempo, etc.):**
```toml
opentelemetry-otlp = "0.15"
```

#### Log Levels

Control tracing verbosity via `RUST_LOG`:
```bash
# Show all traces
RUST_LOG=trace cargo run

# Show only info and above, with debug for your app
RUST_LOG=info,backend=debug cargo run

# Production setting
RUST_LOG=info,backend=info cargo run
```

### Custom Metrics

The metrics system supports custom business metrics:
```rust
use metrics::{counter, histogram, gauge};

// Count events
counter!("user_registration_total").increment(1);

// Track durations
histogram!("api_call_duration_seconds").record(duration);

// Monitor values
gauge!("active_connections").set(count as f64);
```

### Testing with Test Helpers

```rust
use crate::tests::common::{TestClient, create_test_jwt};

#[tokio::test]
async fn test_protected_endpoint() {
    let state = setup_test_state();
    let app = routes::create_router(state);
    let client = TestClient::new(app);

    let token = create_test_jwt("user123", "secret");

    let response = client
        .get("/api/v1/auth/me")
        .header("authorization", format!("Bearer {}", token))
        .await;

    response.assert_status(StatusCode::OK);
    let user: User = response.json();
    assert_eq!(user.id, "user123");
}
```

## Contributing

1. Format your code: `make fmt`
2. Run linter: `make lint`
3. Run tests: `make test`
4. Ensure all checks pass: `make check`

## License

MIT OR Apache-2.0
