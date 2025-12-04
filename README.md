# Boilerplate Application

Full-stack web application boilerplate with modern architecture and best practices.

## Tech Stack

### Backend
- **Language**: Rust 1.80+
- **Web Framework**: Axum 0.7
- **Runtime**: Tokio (async)
- **Database**: PostgreSQL with Diesel 2.1 (async via diesel-async)
- **Connection Pool**: Deadpool
- **API Documentation**: utoipa + Swagger UI
- **Authentication**: JWT (jsonwebtoken) + Argon2 password hashing
- **Validation**: validator
- **Serialization**: serde + serde_json
- **Logging & Tracing**: tracing, tracing-subscriber, opentelemetry
- **Metrics**: Prometheus exporter
- **Rate Limiting**: tower_governor
- **Background Jobs**: tokio-cron-scheduler
- **HTTP Client**: reqwest
- **Testing**: mockall, rstest, fake, testcontainers, insta

### Frontend
- **Language**: TypeScript 5.9
- **Framework**: React 19
- **Build Tool**: Vite 7
- **State Management**: Effector 23
- **Routing**: Atomic Router + React Router DOM 7
- **UI Components**: Radix UI (comprehensive set)
- **Styling**: Tailwind CSS 4 + class-variance-authority
- **Forms**: React Hook Form + Zod 4 validation
- **HTTP**: Generated types from OpenAPI
- **Date Handling**: date-fns
- **Icons**: lucide-react
- **Notifications**: sonner
- **Theme**: next-themes
- **Code Quality**: ESLint 9, Prettier

### Architecture
- **Pattern**: Feature-Sliced Design (FSD)
- **API**: RESTful with OpenAPI specification
- **Type Safety**: End-to-end type generation from backend to frontend
- **Secrets Management**: Configurable (AWS Secrets Manager, Vault)

### Infrastructure
- **Database**: PostgreSQL (via docker-compose)
- **Migrations**: Diesel migrations
- **Development**: Docker Compose

## Project Structure
```
├── backend/          # Rust/Axum API server
├── frontend/         # React/TypeScript SPA
└── docs/            # Project documentation
```

## Port Configuration

- **Frontend**: `17102`
- **Backend**: `17202`
- **Postgres**: `17302`

## Getting Started

See individual README files in `backend/` and `frontend/` directories for detailed setup instructions.
