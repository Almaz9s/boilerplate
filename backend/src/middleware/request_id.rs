use axum::{
    extract::Request,
    http::HeaderValue,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Middleware to add request ID to all requests
pub async fn request_id_middleware(
    mut req: Request,
    next: Next,
) -> Response {
    // Check if request already has an ID, otherwise generate one
    let request_id = req
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // Add request ID to request extensions for use in handlers
    req.extensions_mut().insert(RequestId(request_id.clone()));

    // Call the next middleware/handler
    let mut response = next.run(req).await;

    // Add request ID to response headers
    if let Ok(header_value) = HeaderValue::from_str(&request_id) {
        response.headers_mut().insert(REQUEST_ID_HEADER, header_value);
    }

    response
}

/// Request ID extension for extracting in handlers
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct RequestId(pub String);

#[allow(dead_code)]
impl RequestId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
