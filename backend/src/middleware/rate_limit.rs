//! Simple rate limiter for authentication endpoints
//!
//! Tracks request counts per IP address with a sliding window.
//! Automatically cleans up old entries to prevent memory leaks.
use axum::{
    body::Body,
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Rate limiter state shared across requests
#[derive(Clone)]
pub struct RateLimiter {
    state: Arc<RwLock<RateLimiterState>>,
    max_requests: usize,
    window: Duration,
    trust_proxy: bool,
}

struct RateLimiterState {
    requests: HashMap<String, Vec<Instant>>,
    last_cleanup: Instant,
}

impl RateLimiter {
    /// Create a new rate limiter
    ///
    /// # Arguments
    /// * `max_requests` - Maximum number of requests allowed in the time window
    /// * `window` - Time window duration (e.g., Duration::from_secs(60) for 1 minute)
    /// * `trust_proxy` - Whether to trust X-Forwarded-For/X-Real-IP headers
    pub fn new(max_requests: usize, window: Duration, trust_proxy: bool) -> Self {
        Self {
            state: Arc::new(RwLock::new(RateLimiterState {
                requests: HashMap::new(),
                last_cleanup: Instant::now(),
            })),
            max_requests,
            window,
            trust_proxy,
        }
    }

    /// Create a rate limiter for auth endpoints: 10 requests per minute
    pub fn auth(trust_proxy: bool) -> Self {
        Self::new(10, Duration::from_secs(60), trust_proxy)
    }

    /// Check if a request from the given IP should be allowed
    pub async fn check(&self, ip: &str) -> bool {
        let mut state = self.state.write().await;
        let now = Instant::now();

        // Cleanup old entries every 5 minutes
        if now.duration_since(state.last_cleanup) > Duration::from_secs(300) {
            state.requests.retain(|_, timestamps| {
                timestamps.retain(|ts| now.duration_since(*ts) < self.window);
                !timestamps.is_empty()
            });
            state.last_cleanup = now;
        }

        // Get or create request history for this IP
        let requests = state.requests.entry(ip.to_string()).or_insert_with(Vec::new);

        // Remove expired requests
        requests.retain(|ts| now.duration_since(*ts) < self.window);

        // Check if under limit
        if requests.len() < self.max_requests {
            requests.push(now);
            true
        } else {
            false
        }
    }
}

/// Extract IP address from request
///
/// # Arguments
/// * `req` - The HTTP request
/// * `trust_proxy` - Whether to trust X-Forwarded-For/X-Real-IP headers
///                   Should only be true when behind a trusted reverse proxy
///
/// # Security
/// When `trust_proxy` is false, proxy headers are ignored to prevent IP spoofing
fn extract_ip(req: &Request, trust_proxy: bool) -> String {
    // Only trust proxy headers if explicitly configured
    if trust_proxy {
        // Try to get IP from X-Forwarded-For header (for proxies/load balancers)
        if let Some(forwarded_for) = req.headers().get("x-forwarded-for") {
            if let Ok(forwarded_str) = forwarded_for.to_str() {
                // Take the first IP in the chain (the client IP)
                if let Some(first_ip) = forwarded_str.split(',').next() {
                    return first_ip.trim().to_string();
                }
            }
        }

        // Try to get IP from X-Real-IP header
        if let Some(real_ip) = req.headers().get("x-real-ip") {
            if let Ok(ip_str) = real_ip.to_str() {
                return ip_str.to_string();
            }
        }
    }

    // Fall back to ConnectInfo (actual connection IP)
    if let Some(ConnectInfo(addr)) = req.extensions().get::<ConnectInfo<SocketAddr>>() {
        return addr.ip().to_string();
    }

    // Last resort: use unknown
    "unknown".to_string()
}

/// Create a rate limiting middleware closure
///
/// Returns a closure that can be used with axum::middleware::from_fn
pub fn rate_limit_layer(
    limiter: RateLimiter,
) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response<Body>> + Send>> + Clone {
    move |req: Request, next: Next| {
        let limiter = limiter.clone();
        let trust_proxy = limiter.trust_proxy;
        Box::pin(async move {
            // Extract IP address from headers or connection info
            let ip = extract_ip(&req, trust_proxy);

            // Check rate limit
            if !limiter.check(&ip).await {
                tracing::warn!(ip = %ip, "Rate limit exceeded");

                let error_response = serde_json::json!({
                    "error": "Too many requests",
                    "message": "Rate limit exceeded. Please try again later.",
                });

                return (StatusCode::TOO_MANY_REQUESTS, Json(error_response)).into_response();
            }

            next.run(req).await
        })
            as std::pin::Pin<Box<dyn std::future::Future<Output = Response<Body>> + Send>>
    }
}
