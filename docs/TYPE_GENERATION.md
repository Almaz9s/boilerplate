# TypeScript Type Generation from OpenAPI

This project uses **OpenAPI → TypeScript** workflow to automatically generate TypeScript types from the Rust backend API specification.

## Overview

The backend uses `utoipa` to generate OpenAPI specs, and the frontend uses `openapi-typescript` to convert those specs into TypeScript types.

### Architecture

```
Backend (Rust)           →    OpenAPI Spec    →    Frontend (TypeScript)
[utoipa annotations]     →    [openapi.json]  →    [api.generated.ts]
```

## Setup

### Backend

The backend has a dedicated binary (`src/bin/openapi.rs`) that exports the OpenAPI specification:

```rust
use backend::docs::ApiDoc;
use utoipa::OpenApi;

fn main() {
    let openapi = ApiDoc::openapi();
    let json = openapi.to_pretty_json().expect("Failed to serialize OpenAPI spec");
    println!("{}", json);
}
```

### Frontend

The frontend has `openapi-typescript` installed as a dev dependency and npm scripts configured:

```json
{
  "scripts": {
    "openapi:generate": "openapi-typescript openapi.json -o src/shared/types/api.generated.ts",
    "types:generate": "pnpm openapi:generate"
  }
}
```

## Usage

### Generate Types - Full Workflow

From the **backend** directory:

```bash
# Generate OpenAPI spec and save to frontend directory
just openapi-frontend
```

From the **frontend** directory:

```bash
# Generate TypeScript types from OpenAPI spec
pnpm types:generate
```

### Using Generated Types

The generated types are available at `src/shared/types/api.generated.ts`:

```typescript
import type { paths, components, operations } from '@/shared/types/api.generated';

// Access path types
type HealthCheckResponse = paths['/api/v1/health']['get']['responses']['200']['content']['application/json'];

// Access component schemas
type HealthResponse = components['schemas']['HealthResponse'];
type HealthChecks = components['schemas']['HealthChecks'];

// Access operations
type HealthCheckOperation = operations['health_check'];
```

### Example: Type-safe API Client

```typescript
import type { paths } from '@/shared/types/api.generated';

// Type-safe fetch wrapper
async function apiGet<P extends keyof paths>(
  path: P,
  options?: RequestInit
): Promise<paths[P]['get']['responses']['200']['content']['application/json']> {
  const response = await fetch(`http://localhost:2999${path}`, options);
  return response.json();
}

// Usage with full type safety
const health = await apiGet('/api/v1/health');
// health is typed as HealthResponse
console.log(health.status, health.version);
```

## Backend: Adding New Endpoints

When adding new endpoints to the backend, make sure to:

1. **Add `ToSchema` derive** to your DTOs:
   ```rust
   use utoipa::ToSchema;

   #[derive(Serialize, Deserialize, ToSchema)]
   pub struct MyRequestDto {
       pub field: String,
   }
   ```

2. **Document your handler** with utoipa annotations:
   ```rust
   #[utoipa::path(
       post,
       path = "/api/v1/my-endpoint",
       request_body = MyRequestDto,
       responses(
           (status = 200, description = "Success", body = MyResponseDto),
           (status = 400, description = "Bad Request")
       ),
       tag = "my-tag"
   )]
   pub async fn my_handler(/* ... */) -> impl IntoResponse {
       // handler implementation
   }
   ```

3. **Register in `docs.rs`**:
   ```rust
   #[derive(OpenApi)]
   #[openapi(
       paths(
           crate::handlers::health::health_check,
           crate::handlers::my_handler,  // Add your handler here
       ),
       components(
           schemas(
               crate::models::MyRequestDto,
               crate::models::MyResponseDto,  // Add your schemas here
           )
       )
   )]
   pub struct ApiDoc;
   ```

4. **Regenerate types**:
   ```bash
   cd backend && just openapi-frontend
   cd ../frontend && pnpm types:generate
   ```

## Available Commands

### Backend (using just)

```bash
# Generate OpenAPI spec to backend/openapi.json
just openapi

# Generate OpenAPI spec to frontend/openapi.json
just openapi-frontend
```

### Frontend (using pnpm)

```bash
# Generate TypeScript types from openapi.json
pnpm types:generate

# Alias for the same command
pnpm openapi:generate
```

## File Locations

- **Backend OpenAPI binary**: `backend/src/bin/openapi.rs`
- **Backend OpenAPI config**: `backend/src/docs.rs`
- **OpenAPI spec** (generated): `frontend/openapi.json` (gitignored)
- **Generated TS types**: `frontend/src/shared/types/api.generated.ts` (gitignored)

## Best Practices

1. **Don't edit generated files**: The `api.generated.ts` file is auto-generated and should never be manually edited.

2. **Regenerate after backend changes**: Whenever you add/modify backend endpoints or DTOs, regenerate the types.

3. **Commit OpenAPI annotations**: Make sure your backend code includes proper utoipa annotations for API documentation.

4. **Type-safe API layer**: Use the generated types in your API client code for compile-time safety.

5. **Version control**: The generated files (`openapi.json` and `api.generated.ts`) are gitignored by default. Each developer generates them locally.

## Troubleshooting

### Types not updating

```bash
# Clean and regenerate
cd frontend
rm -f openapi.json src/shared/types/api.generated.ts
cd ../backend
just openapi-frontend
cd ../frontend
pnpm types:generate
```

### Missing schemas in generated types

Make sure your Rust structs:
- Have `#[derive(ToSchema)]`
- Are registered in `docs.rs` under `components(schemas(...))`

### Missing endpoints in generated types

Make sure your handlers:
- Have `#[utoipa::path(...)]` annotations
- Are registered in `docs.rs` under `paths(...)`

## Additional Resources

- [utoipa documentation](https://github.com/juhaku/utoipa)
- [openapi-typescript documentation](https://github.com/drwpow/openapi-typescript)
- Backend OpenAPI spec available at: `http://localhost:2999/swagger-ui` (when server is running)
