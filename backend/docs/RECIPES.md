# Code Recipes

Quick copy-paste solutions for common tasks.

## 🎯 Adding a New Entity

### 1. Create Migration

```bash
just db-gen create_products
```

### 2. Edit Migration Files

```rust
// migrations/xxx_create_products/up.sql
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    price INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

// migrations/xxx_create_products/down.sql
DROP TABLE products;
```

### 3. Run Migration and Update Schema

```bash
just db-migrate
```

### 4. Create Model

```rust
// src/models/product.rs
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

use crate::db::schema::products;

#[derive(Debug, Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub price: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(range(min = 0))]
    pub price: i32,
}
```

### 5. Create Repository

```rust
// src/repositories/product_repository.rs
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    db::{schema::products, DbPool},
    error::AppError,
    models::product::{NewProduct, Product},
};

#[derive(Clone)]
pub struct ProductRepository {
    pool: DbPool,
}

impl ProductRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(name = "repo_create_product", skip(self))]
    pub async fn create(&self, new_product: NewProduct) -> Result<Product, AppError> {
        let mut conn = crate::db::get_connection(&self.pool).await?;

        diesel::insert_into(products::table)
            .values(&new_product)
            .returning(Product::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(|e| AppError::database("Failed to create product", e))
    }

    #[tracing::instrument(name = "repo_find_product_by_id", skip(self))]
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>, AppError> {
        let mut conn = crate::db::get_connection(&self.pool).await?;

        products::table
            .find(id)
            .select(Product::as_select())
            .first(&mut conn)
            .await
            .optional()
            .map_err(|e| AppError::database("Failed to find product", e))
    }

    #[tracing::instrument(name = "repo_list_products", skip(self))]
    pub async fn list(&self) -> Result<Vec<Product>, AppError> {
        let mut conn = crate::db::get_connection(&self.pool).await?;

        products::table
            .select(Product::as_select())
            .load(&mut conn)
            .await
            .map_err(|e| AppError::database("Failed to list products", e))
    }
}
```

### 6. Add Service (if needed)

```rust
// src/services/product.rs
use crate::{
    error::AppError,
    models::product::{CreateProductRequest, Product},
    repositories::product_repository::ProductRepository,
};

pub struct ProductService {
    product_repo: ProductRepository,
}

impl ProductService {
    pub fn new(product_repo: ProductRepository) -> Self {
        Self { product_repo }
    }

    #[tracing::instrument(name = "service_create_product", skip(self))]
    pub async fn create(&self, req: CreateProductRequest) -> Result<Product, AppError> {
        // Business logic here
        let new_product = NewProduct {
            name: req.name,
            price: req.price,
        };

        self.product_repo.create(new_product).await
    }
}
```

### 7. Add to Services Struct

```rust
// src/lib.rs
pub struct Services {
    pub auth: Arc<AuthService>,
    pub user_repo: Arc<UserRepository>,
    pub product: Arc<ProductService>,  // Add here
}

impl Services {
    pub fn new(db_pool: DbPool, config: &Config) -> Self {
        // ... existing services
        let product_repo = ProductRepository::new(db_pool.clone());
        let product_service = ProductService::new(product_repo);

        Self {
            // ... existing services
            product: Arc::new(product_service),
        }
    }
}
```

### 8. Create Handler

```rust
// src/handlers/product.rs
use axum::{extract::{Path, State}, http::StatusCode, Json};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    models::product::CreateProductRequest,
    AppState,
};

#[utoipa::path(
    post,
    path = "/api/v1/products",
    request_body = CreateProductRequest,
    responses(
        (status = 201, description = "Product created", body = Product),
        (status = 400, description = "Bad request"),
    )
)]
#[tracing::instrument(name = "create_product", skip(state))]
pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<Product>), AppError> {
    req.validate()?;

    let product = state.services.product.create(req).await?;

    Ok((StatusCode::CREATED, Json(product)))
}

#[utoipa::path(
    get,
    path = "/api/v1/products/{id}",
    responses(
        (status = 200, description = "Product found", body = Product),
        (status = 404, description = "Product not found"),
    )
)]
#[tracing::instrument(name = "get_product", skip(state))]
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Product>, AppError> {
    let product = state.services.product_repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

    Ok(Json(product))
}
```

### 9. Add Routes

```rust
// src/routes.rs
let product_routes = Router::new()
    .route("/products", post(handlers::product::create))
    .route("/products/:id", get(handlers::product::get));

let api_routes = Router::new()
    .route("/health", get(handlers::health_check))
    .nest("/auth", auth_routes)
    .merge(product_routes);  // Add here
```

---

## 🔍 Debugging

### Debug Slow Queries

```rust
use std::time::Instant;

let start = Instant::now();
let result = query.load(&mut conn).await?;
let elapsed = start.elapsed();

if elapsed.as_millis() > 100 {
    tracing::warn!("Slow query detected: {:?}", elapsed);
}
```

### Debug Request/Response Bodies

```bash
# Set environment variable
export LOG_BODIES=1

# Run with debug logging
RUST_LOG=debug cargo run
```

### View Database Pool Status

```bash
# In dev mode
curl http://localhost:8080/dev/state | jq '.database'
```

### Test Error Responses

```bash
# Simulate different errors
curl http://localhost:8080/dev/error/not_found
curl http://localhost:8080/dev/error/unauthorized
curl http://localhost:8080/dev/error/database
```

---

## 🧪 Testing

### Integration Test Template

```rust
// tests/product_integration_test.rs
use axum::http::StatusCode;
use backend::routes::create_router;
use backend::tests::common::*;

#[tokio::test]
async fn test_create_product() {
    let state = setup_test_state();
    let app = create_router(state);
    let client = TestClient::new(app);

    let body = json!({
        "name": "Test Product",
        "price": 1000
    });

    let response = client.post("/api/v1/products", &body).await;
    response.assert_status(StatusCode::CREATED);

    let product: Product = response.json();
    assert_eq!(product.name, "Test Product");
    assert_eq!(product.price, 1000);
}
```

### Snapshot Testing

```rust
use insta::assert_json_snapshot;

#[tokio::test]
async fn test_product_response_format() {
    let response = client.get("/api/v1/products/123").await;

    assert_json_snapshot!(response.json::<Value>(), {
        ".id" => "[uuid]",
        ".created_at" => "[timestamp]",
        ".updated_at" => "[timestamp]",
    });
}

// Update snapshots
// cargo test -- --update-snapshots
// cargo insta accept
```

---

## 🛠️ Common Patterns

### Using Error Macros

```rust
use backend::{bail_not_found, bail_bad_request, db_error, internal_error};

// Quick bail
if product.is_none() {
    bail_not_found!("Product not found");
}

// With validation
if price < 0 {
    bail_bad_request!("Price must be positive");
}

// Database errors with context
Err(db_error!("Failed to insert product", err))

// Internal errors
Err(internal_error!("Unexpected state", err))
```

### Pagination

```rust
use crate::db::pagination::Paginate;
use crate::models::PaginationParams;

async fn list_products(
    Query(params): Query<PaginationParams>,
    State(state): State<AppState>,
) -> Result<Json<PaginatedResponse<Product>>, AppError> {
    let mut conn = get_connection(&state.db_pool).await?;

    let result = products::table
        .order(products::created_at.desc())
        .paginate(params.page)
        .per_page(params.per_page)
        .load_and_count(&mut conn)
        .await?;

    Ok(Json(result))
}
```

### Transactions

```rust
use crate::db::transaction::with_transaction;

async fn transfer_operation() -> Result<(), AppError> {
    let mut conn = get_connection(&pool).await?;

    with_transaction(&mut conn, |conn| {
        Box::pin(async move {
            // Multiple operations that must succeed together
            operation1(conn).await?;
            operation2(conn).await?;
            Ok(())
        })
    }).await
}
```

### Background Jobs

```rust
// src/jobs/tasks.rs
use tokio_cron_scheduler::Job;

pub async fn cleanup_old_records(state: Arc<AppState>) -> Result<(), AppError> {
    tracing::info!("Running cleanup job");

    let mut conn = get_connection(&state.db_pool).await?;

    diesel::delete(
        products::table
            .filter(products::created_at.lt(
                Utc::now() - chrono::Duration::days(30)
            ))
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

// src/jobs/mod.rs - Add to scheduler
let cleanup_job = Job::new_async("0 0 2 * * *", move |_uuid, _lock| {
    let state = state_clone.clone();
    Box::pin(async move {
        if let Err(e) = tasks::cleanup_old_records(state).await {
            tracing::error!("Cleanup job failed: {}", e);
        }
    })
})?;
scheduler.add(cleanup_job).await?;
```

---

## 📚 Quick Reference

### Common Just Commands

```bash
just dev-watch          # Development with auto-reload
just quick             # Fast check (fmt, check, test)
just db-fresh          # Fresh database with seed data
just watch-test auth   # Watch specific test module
just install-hooks     # Set up pre-commit hooks
just db-test-migrations # Verify migrations
```

### Environment Variables

```bash
# Simple development logging (default)
cargo run

# Full OpenTelemetry tracing
OTEL=1 cargo run

# Debug level logging
RUST_LOG=debug cargo run

# Trace specific modules
RUST_LOG=backend::services=debug cargo run
```

### REST Client Testing

```http
# requests.http
@baseUrl = http://localhost:8080/api/v1

### Create Product
POST {{baseUrl}}/products
Content-Type: application/json

{
  "name": "New Product",
  "price": 2500
}

### Get Product (use ID from previous request)
GET {{baseUrl}}/products/{{productId}}
```
