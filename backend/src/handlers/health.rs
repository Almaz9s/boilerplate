use axum::{extract::State, http::StatusCode, Json};
use crate::{
    db,
    models::{HealthChecks, HealthResponse, SubsystemHealth},
    AppState,
};

/// Health check endpoint with comprehensive subsystem monitoring
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 503, description = "Service is degraded or unhealthy", body = HealthResponse)
    ),
    tag = "health"
)]
#[tracing::instrument(name = "health_check", skip(state))]
pub async fn health_check(
    State(state): State<AppState>,
) -> (StatusCode, Json<HealthResponse>) {
    tracing::debug!("Starting health check");

    // Check database
    let database_health = match db::test_connection(&state.db_pool).await {
        Ok(_) => {
            let pool_status = state.db_pool.status();
            SubsystemHealth {
                status: "healthy".to_string(),
                message: None,
                details: Some(serde_json::json!({
                    "available_connections": pool_status.available,
                    "max_connections": pool_status.max_size,
                })),
            }
        }
        Err(e) => SubsystemHealth {
            status: "unhealthy".to_string(),
            message: Some(format!("Database connection failed: {}", e)),
            details: None,
        },
    };

    // Check memory usage
    let memory_health = check_memory_health();

    let checks = HealthChecks {
        database: database_health,
        memory: memory_health,
    };

    // Determine overall status
    let overall_status = if checks.database.status == "healthy" && checks.memory.status == "healthy"
    {
        "healthy"
    } else if checks.database.status == "unhealthy" {
        "unhealthy"
    } else {
        "degraded"
    };

    let status_code = if overall_status == "healthy" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    tracing::info!(
        overall_status = %overall_status,
        database_status = %checks.database.status,
        memory_status = %checks.memory.status,
        "Health check completed"
    );

    (
        status_code,
        Json(HealthResponse {
            status: overall_status.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            checks,
        }),
    )
}

fn check_memory_health() -> SubsystemHealth {
    // Get process memory info (basic check)
    // NOTE: Reading /proc on every request has minimal overhead, but for high-traffic
    // systems, consider caching this value or using a dedicated monitoring solution
    #[cfg(target_os = "linux")]
    {
        // Use a thread-local cache to avoid reading /proc on every single health check
        use std::cell::RefCell;
        use std::time::{Duration, Instant};

        thread_local! {
            static CACHED_MEMORY: RefCell<Option<(Instant, SubsystemHealth)>> = const { RefCell::new(None) };
        }

        const CACHE_DURATION: Duration = Duration::from_secs(5);

        CACHED_MEMORY.with(|cache| {
            let mut cached = cache.borrow_mut();

            // Check if cache is valid
            if let Some((timestamp, health)) = cached.as_ref() {
                if timestamp.elapsed() < CACHE_DURATION {
                    return health.clone();
                }
            }

            // Read new value
            let health = read_memory_status_linux();
            *cached = Some((Instant::now(), health.clone()));
            health
        })
    }

    #[cfg(not(target_os = "linux"))]
    {
        SubsystemHealth {
            status: "healthy".to_string(),
            message: Some("Memory check not available on this platform".to_string()),
            details: None,
        }
    }
}

#[cfg(target_os = "linux")]
fn read_memory_status_linux() -> SubsystemHealth {
    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
        if let Some(line) = status.lines().find(|l| l.starts_with("VmRSS:")) {
            if let Some(kb_str) = line.split_whitespace().nth(1) {
                if let Ok(kb) = kb_str.parse::<u64>() {
                    let mb = kb / 1024;
                    let status = if mb > 1024 { "degraded" } else { "healthy" };
                    return SubsystemHealth {
                        status: status.to_string(),
                        message: None,
                        details: Some(serde_json::json!({
                            "memory_usage_mb": mb,
                        })),
                    };
                }
            }
        }
    }

    SubsystemHealth {
        status: "healthy".to_string(),
        message: Some("Could not read memory status".to_string()),
        details: None,
    }
}
