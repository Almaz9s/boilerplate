//! Security headers middleware
//!
//! Adds common security headers to all responses to protect against
//! common web vulnerabilities.
use axum::{
    body::Body,
    extract::Request,
    http::header,
    middleware::Next,
    response::Response,
};

/// Middleware to add security headers to responses
///
/// Adds the following headers:
/// - X-Content-Type-Options: nosniff (prevents MIME sniffing)
/// - X-Frame-Options: DENY (prevents clickjacking)
/// - X-XSS-Protection: 1; mode=block (XSS protection for older browsers)
/// - Strict-Transport-Security: enforces HTTPS (in production only)
/// - Referrer-Policy: strict-origin-when-cross-origin (controls referrer info)
pub async fn security_headers(request: Request, next: Next) -> Response<Body> {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    // Prevent MIME type sniffing
    headers.insert(
        header::HeaderName::from_static("x-content-type-options"),
        header::HeaderValue::from_static("nosniff"),
    );

    // Prevent clickjacking - don't allow embedding in iframes
    headers.insert(
        header::HeaderName::from_static("x-frame-options"),
        header::HeaderValue::from_static("DENY"),
    );

    // Enable XSS protection in older browsers
    headers.insert(
        header::HeaderName::from_static("x-xss-protection"),
        header::HeaderValue::from_static("1; mode=block"),
    );

    // Control referrer information sent with requests
    headers.insert(
        header::REFERRER_POLICY,
        header::HeaderValue::from_static("strict-origin-when-cross-origin"),
    );

    // HSTS: Force HTTPS for 1 year (only in production)
    // Note: Only enable this if you're serving over HTTPS in production!
    #[cfg(not(debug_assertions))]
    {
        headers.insert(
            header::STRICT_TRANSPORT_SECURITY,
            header::HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        );
    }

    // CSP: Basic content security policy (you should customize this for your needs)
    // This is a restrictive default - adjust based on your application requirements
    headers.insert(
        header::HeaderName::from_static("content-security-policy"),
        header::HeaderValue::from_static(
            "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self'; connect-src 'self'; frame-ancestors 'none';",
        ),
    );

    response
}
