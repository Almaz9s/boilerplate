use axum::{body::Body, extract::MatchedPath, http::Request, middleware::Next, response::Response};
use metrics::{counter, histogram};
use std::time::Instant;

/// Middleware to track HTTP metrics
pub async fn track_metrics(req: Request<Body>, next: Next) -> Response {
    let start = Instant::now();
    let path = req
        .extensions()
        .get::<MatchedPath>()
        .map(|p| p.as_str().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let method = req.method().to_string();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    // Record metrics
    counter!("http_requests_total", "method" => method.clone(), "path" => path.clone(), "status" => status.clone()).increment(1);
    histogram!("http_request_duration_seconds", "method" => method, "path" => path).record(latency);

    response
}

/// Initialize metrics recorder
pub fn init_metrics() {
    match metrics_exporter_prometheus::PrometheusBuilder::new().install() {
        Ok(_) => {
            tracing::info!("Prometheus metrics exporter initialized");
        }
        Err(e) => {
            tracing::warn!("Failed to install Prometheus metrics exporter: {}. Metrics will not be available.", e);
        }
    }
}

/// Get metrics handler - returns Prometheus formatted metrics
pub async fn metrics_handler() -> String {
    // Get the handle that was created during init
    match metrics_exporter_prometheus::PrometheusBuilder::new().install_recorder() {
        Ok(handle) => handle.render(),
        Err(e) => {
            tracing::error!("Failed to get Prometheus handle: {}", e);
            format!("# Metrics unavailable: {}", e)
        }
    }
}
