# Authentication Guide

This backend now includes a complete JWT-based authentication system with user registration, login, and protected routes.

## Features

- User registration with email, username, and password
- Secure password hashing using Argon2
- JWT token generation and validation
- Protected routes requiring authentication
- Email and username validation
- Password strength requirements (minimum 8 characters)

## API Endpoints

### 1. Register a New User

**Endpoint:** `POST /api/v1/auth/register`

**Request Body:**
```json
{
  "email": "user@example.com",
  "username": "johndoe",
  "password": "securepassword123"
}
```

**Response (201 Created):**
```json
{
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "username": "johndoe",
    "created_at": "2025-10-13T12:34:56"
  },
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Validation Rules:**
- Email must be a valid email address
- Username must be 3-100 characters
- Password must be at least 8 characters
- Email and username must be unique

### 2. Login

**Endpoint:** `POST /api/v1/auth/login`

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

**Response (200 OK):**
```json
{
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "username": "johndoe",
    "created_at": "2025-10-13T12:34:56"
  },
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### 3. Get Current User (Protected)

**Endpoint:** `GET /api/v1/auth/me`

**Headers:**
```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "username": "johndoe",
  "created_at": "2025-10-13T12:34:56"
}
```

## Using Authentication in Your Handlers

### Required Authentication

To protect a route and require authentication, use the `AuthUser` extractor:

```rust
use crate::middleware::auth::AuthUser;

pub async fn protected_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,  // This will extract the authenticated user
) -> Result<Json<SomeResponse>, AppError> {
    // Access user information
    let user_id = auth_user.user_id;
    let email = auth_user.email;
    let username = auth_user.username;

    // Your handler logic here
    Ok(Json(response))
}
```

### Optional Authentication

For routes where authentication is optional, use `OptionalAuthUser`:

```rust
use crate::middleware::auth::OptionalAuthUser;

pub async fn optional_auth_handler(
    State(state): State<AppState>,
    OptionalAuthUser(maybe_user): OptionalAuthUser,
) -> Result<Json<SomeResponse>, AppError> {
    match maybe_user {
        Some(user) => {
            // User is authenticated
            println!("Authenticated user: {}", user.username);
        }
        None => {
            // User is not authenticated
            println!("Anonymous user");
        }
    }

    Ok(Json(response))
}
```

## Example Usage with cURL

### Register a new user:
```bash
curl -X POST http://localhost:2999/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "username": "testuser",
    "password": "password123"
  }'
```

### Login:
```bash
curl -X POST http://localhost:2999/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }'
```

### Get current user (using token from login response):
```bash
curl -X GET http://localhost:2999/api/v1/auth/me \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"
```

## Error Responses

### 400 Bad Request
```json
{
  "error": "User with this email or username already exists"
}
```

### 401 Unauthorized
```json
{
  "error": "Invalid email or password"
}
```

### 422 Unprocessable Entity (Validation Error)
```json
{
  "error": "email: Invalid email address"
}
```

## Database Setup

Before using the authentication system, run the migrations:

```bash
# Make sure PostgreSQL is running (via docker-compose or locally)
make docker-up

# Run migrations
diesel migration run --database-url="postgres://postgres:postgres@localhost:5432/backend_db"

# Or if you have .env file set up:
make migrate
```

## Security Considerations

1. **JWT Secret**: Change `JWT_SECRET` in `.env` to a strong, random value in production
2. **Password Hashing**: Uses Argon2, the winner of the Password Hashing Competition
3. **Token Expiration**: Tokens expire after 24 hours by default (configurable via `JWT_EXPIRATION_HOURS`)
4. **HTTPS**: Always use HTTPS in production to protect tokens in transit
5. **Token Storage**: Store tokens securely on the client side (not in localStorage for sensitive apps)

## Configuration

All auth configuration is in `.env`:

```env
# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
JWT_EXPIRATION_HOURS=24
```

## Implementation Details

### Architecture

- **Models** (`src/models/user.rs`): User data structures and DTOs
- **Services** (`src/services/`):
  - `jwt.rs`: JWT token generation and validation
  - `auth.rs`: Authentication business logic (register, login, user lookup)
- **Handlers** (`src/handlers/auth.rs`): HTTP request handlers
- **Middleware** (`src/middleware/auth.rs`): JWT authentication extractors
- **Routes** (`src/routes.rs`): Route configuration

### Database Schema

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### JWT Claims

```rust
{
  "sub": "user-uuid",           // Subject (user ID)
  "email": "user@example.com",  // User email
  "username": "johndoe",        // Username
  "exp": 1234567890,            // Expiration time
  "iat": 1234567890             // Issued at time
}
```

## Testing

Run the tests:

```bash
make test
```

## Next Steps

1. Add password reset functionality
2. Add email verification
3. Add refresh tokens for long-lived sessions
4. Add rate limiting for login attempts
5. Add two-factor authentication (2FA)
6. Add OAuth2 providers (Google, GitHub, etc.)
