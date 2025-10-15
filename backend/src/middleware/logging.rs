use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};
use std::time::Instant;

/// Request logging middleware with tracing spans
/// This creates a span for each request with detailed context
///
/// Set VERBOSE_HTTP=1 to log request headers and body
pub async fn log_request(
    req: Request<Body>,
    next: Next,
) -> Response<Body> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let path = uri.path().to_string();
    let query = uri.query().map(|q| q.to_string());
    let version = format!("{:?}", req.version());
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    // Check if verbose logging is enabled
    let verbose = std::env::var("VERBOSE_HTTP").is_ok();

    // Create a span for this request
    let span = tracing::info_span!(
        "http_request",
        http.method = %method,
        http.route = %path,
        http.version = %version,
        http.user_agent = %user_agent,
        http.status_code = tracing::field::Empty,
        http.response_time_ms = tracing::field::Empty,
        otel.kind = "server",
        otel.status_code = tracing::field::Empty,
    );

    let _guard = span.enter();

    if let Some(q) = query {
        tracing::debug!(http.query = %q, "Request query parameters");
    }

    // Log headers in verbose mode
    if verbose {
        tracing::debug!("📨 Request headers:");
        for (name, value) in req.headers() {
            if let Ok(val) = value.to_str() {
                // Don't log sensitive headers
                if name != "authorization" && name != "cookie" {
                    tracing::debug!("  {}: {}", name, val);
                }
            }
        }
    }

    tracing::info!("Request started");

    let start = Instant::now();

    // Drop the guard before calling next to avoid holding the span across await
    drop(_guard);

    let response = async {
        next.run(req).await
    }
    .instrument(span.clone())
    .await;

    let elapsed = start.elapsed();
    let status = response.status();

    // Record response information in the span
    span.record("http.status_code", status.as_u16());
    span.record("http.response_time_ms", elapsed.as_millis() as u64);

    // Set OpenTelemetry status code based on HTTP status
    if status.is_server_error() || status.is_client_error() {
        span.record("otel.status_code", "ERROR");
    } else {
        span.record("otel.status_code", "OK");
    }

    let _guard = span.enter();
    tracing::info!(
        status = %status,
        elapsed_ms = %elapsed.as_millis(),
        "Request completed"
    );

    response
}

// Re-export for convenience
use tracing::Instrument;
