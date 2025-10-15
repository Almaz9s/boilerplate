//! Development-only endpoints for debugging and testing
//!
//! These endpoints are only available in debug builds and should NEVER
//! be compiled into production releases.

use axum::{extract::State, Json, response::Html};
use serde_json::{json, Value};

use crate::{error::AppError, AppState};

/// Development dashboard with links to all dev tools
///
/// GET /dev
pub async fn dashboard() -> Html<String> {
    Html(format!(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Development Dashboard</title>
    <style>
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            line-height: 1.6;
            color: #333;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }}
        .container {{
            max-width: 900px;
            margin: 0 auto;
            background: white;
            border-radius: 12px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            overflow: hidden;
        }}
        .header {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 40px;
            text-align: center;
        }}
        .header h1 {{
            font-size: 2.5em;
            margin-bottom: 10px;
            font-weight: 700;
        }}
        .header p {{
            opacity: 0.9;
            font-size: 1.1em;
        }}
        .content {{
            padding: 40px;
        }}
        .section {{
            margin-bottom: 40px;
        }}
        .section h2 {{
            color: #667eea;
            margin-bottom: 20px;
            font-size: 1.5em;
            border-bottom: 2px solid #667eea;
            padding-bottom: 10px;
        }}
        .links {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 15px;
        }}
        .link-card {{
            background: #f7fafc;
            border: 2px solid #e2e8f0;
            border-radius: 8px;
            padding: 20px;
            text-decoration: none;
            color: #333;
            transition: all 0.3s ease;
            display: block;
        }}
        .link-card:hover {{
            border-color: #667eea;
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(102, 126, 234, 0.2);
        }}
        .link-card h3 {{
            color: #667eea;
            margin-bottom: 8px;
            font-size: 1.2em;
        }}
        .link-card p {{
            color: #718096;
            font-size: 0.9em;
        }}
        .badge {{
            display: inline-block;
            background: #48bb78;
            color: white;
            padding: 4px 12px;
            border-radius: 20px;
            font-size: 0.85em;
            font-weight: 600;
            margin-left: 10px;
        }}
        .code {{
            background: #2d3748;
            color: #68d391;
            padding: 15px;
            border-radius: 6px;
            font-family: 'Monaco', 'Menlo', monospace;
            font-size: 0.9em;
            margin-top: 10px;
            overflow-x: auto;
        }}
        .footer {{
            background: #f7fafc;
            padding: 20px 40px;
            text-align: center;
            color: #718096;
            border-top: 1px solid #e2e8f0;
        }}
        .status-indicator {{
            display: inline-block;
            width: 8px;
            height: 8px;
            background: #48bb78;
            border-radius: 50%;
            margin-right: 8px;
            animation: pulse 2s infinite;
        }}
        @keyframes pulse {{
            0%, 100% {{ opacity: 1; }}
            50% {{ opacity: 0.5; }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🛠️ Dev Dashboard</h1>
            <p><span class="status-indicator"></span>Development mode active</p>
        </div>

        <div class="content">
            <div class="section">
                <h2>🔍 Debugging Tools</h2>
                <div class="links">
                    <a href="/dev/state" class="link-card">
                        <h3>App State</h3>
                        <p>View current application state, database pool stats, and configuration</p>
                    </a>
                    <a href="/dev/health" class="link-card">
                        <h3>Dev Health Check</h3>
                        <p>Quick health status for development environment</p>
                    </a>
                    <a href="/dev/db-info" class="link-card">
                        <h3>Database Info</h3>
                        <p>Detailed database connection and migration status</p>
                    </a>
                </div>
            </div>

            <div class="section">
                <h2>🧪 Testing Utilities</h2>
                <div class="links">
                    <a href="/dev/token" class="link-card">
                        <h3>JWT Token Generator</h3>
                        <p>Generate test JWT tokens for API authentication</p>
                    </a>
                    <a href="/dev/echo" class="link-card">
                        <h3>Echo Endpoint</h3>
                        <p>Test request/response with JSON echo</p>
                    </a>
                    <a href="/dev/error/not_found" class="link-card">
                        <h3>Error Simulator</h3>
                        <p>Trigger various error types for testing error handling</p>
                    </a>
                </div>
            </div>

            <div class="section">
                <h2>📚 Documentation</h2>
                <div class="links">
                    <a href="/swagger-ui" class="link-card">
                        <h3>Swagger UI <span class="badge">API Docs</span></h3>
                        <p>Interactive API documentation with request testing</p>
                    </a>
                    <a href="/metrics" class="link-card">
                        <h3>Prometheus Metrics</h3>
                        <p>Application metrics in Prometheus format</p>
                    </a>
                    <a href="/api/v1/health" class="link-card">
                        <h3>Production Health</h3>
                        <p>Full health check endpoint with subsystem status</p>
                    </a>
                </div>
            </div>

            <div class="section">
                <h2>⚡ Quick Commands</h2>
                <div class="code">
# Watch mode with auto-reload<br>
just dev-watch<br>
<br>
# Fresh database + watch<br>
just dev-fresh<br>
<br>
# View SQL queries<br>
QUERY_LOG=1 just dev-watch<br>
<br>
# Run tests with nextest<br>
just test
                </div>
            </div>
        </div>

        <div class="footer">
            <p>Backend v{} • Development Mode • Debug Assertions Enabled</p>
        </div>
    </div>
</body>
</html>
    "#, env!("CARGO_PKG_VERSION")))
}

/// Debug endpoint to view application state
///
/// GET /dev/state
#[tracing::instrument(name = "dev_state", skip(state))]
pub async fn debug_state(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    let pool_stats = crate::db::pool_stats(&state.db_pool);

    // Calculate pool utilization percentage
    let utilization = if pool_stats.max_size > 0 {
        (pool_stats.max_size - pool_stats.available) as f64 / pool_stats.max_size as f64 * 100.0
    } else {
        0.0
    };

    Ok(Json(json!({
        "environment": state.config.server.environment,
        "database": {
            "pool_size": pool_stats.size,
            "available_connections": pool_stats.available,
            "max_connections": pool_stats.max_size,
            "utilization_percent": format!("{:.1}%", utilization),
            "status": if utilization > 80.0 { "high_load" } else if utilization > 50.0 { "moderate" } else { "healthy" }
        },
        "server": {
            "host": state.config.server.host,
            "port": state.config.server.port,
            "request_timeout": state.config.server.request_timeout,
        },
        "jwt": {
            "expiration_hours": state.config.jwt.expiration_hours,
        }
    })))
}

/// Health check specifically for development
///
/// GET /dev/health
pub async fn dev_health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "mode": "development",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

/// Echo endpoint for testing requests
///
/// POST /dev/echo
pub async fn echo(Json(body): Json<Value>) -> Json<Value> {
    Json(json!({
        "echo": body,
        "received_at": chrono::Utc::now().to_rfc3339(),
    }))
}

/// Simulate various error responses for testing error handling
///
/// GET /dev/error/:type
pub async fn simulate_error(
    axum::extract::Path(error_type): axum::extract::Path<String>,
) -> Result<Json<Value>, AppError> {
    match error_type.as_str() {
        "not_found" => Err(AppError::NotFound("Test resource not found".to_string())),
        "bad_request" => Err(AppError::BadRequest("Test bad request".to_string())),
        "unauthorized" => Err(AppError::Unauthorized("Test unauthorized".to_string())),
        "internal" => Err(AppError::InternalServerError {
            message: "Test internal error".to_string(),
            source: None,
        }),
        "database" => Err(AppError::DatabaseError {
            message: "Test database error".to_string(),
            source: None,
        }),
        _ => Ok(Json(json!({
            "available_error_types": [
                "not_found",
                "bad_request",
                "unauthorized",
                "internal",
                "database"
            ]
        }))),
    }
}

/// Generate a test JWT token for manual testing
///
/// POST /dev/token
pub async fn generate_test_token(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    use crate::services::jwt::JwtService;

    let jwt_service = JwtService::new(
        state.config.jwt.secret.clone(),
        state.config.jwt.expiration_hours,
    );

    let token = jwt_service.generate_token(
        uuid::Uuid::new_v4(),
        "dev@example.com".to_string(),
        "devuser".to_string(),
    )?;

    Ok(Json(json!({
        "token": token,
        "expires_in_hours": state.config.jwt.expiration_hours,
        "test_user": {
            "email": "dev@example.com",
            "username": "devuser"
        }
    })))
}

/// Quick database info endpoint
///
/// GET /dev/db-info
///
/// Provides quick insights into database state for debugging
pub async fn db_info(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    use diesel::dsl::sql;
    use diesel::sql_types::BigInt;
    use diesel_async::RunQueryDsl;

    let mut conn = crate::db::get_connection(&state.db_pool).await?;

    // Get table counts
    let user_count: Result<i64, _> = diesel::select(sql::<BigInt>("COUNT(*) FROM users"))
        .first(&mut conn)
        .await;

    Ok(Json(json!({
        "database": {
            "url_masked": mask_db_url(&state.config.database.url),
            "pool_size": state.config.database.pool_size,
        },
        "tables": {
            "users": {
                "count": user_count.unwrap_or(-1),
            }
        },
        "hint": "For complex queries, use: just repl (launches psql)",
        "quick_commands": {
            "list_tables": "SELECT tablename FROM pg_tables WHERE schemaname = 'public'",
            "list_users": "SELECT id, email, username FROM users LIMIT 10",
        }
    })))
}

/// Mask sensitive parts of database URL
fn mask_db_url(url: &str) -> String {
    if let Some(at_pos) = url.find('@') {
        if let Some(proto_end) = url.find("://") {
            let proto = &url[..proto_end + 3];
            let rest = &url[at_pos..];
            return format!("{}***{}", proto, rest);
        }
    }
    "***".to_string()
}
