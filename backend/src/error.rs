use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

/// Convenient type alias for Results with AppError
pub type AppResult<T> = Result<T, AppError>;

/// Convenient type alias for handler Results that return JSON
pub type JsonResult<T> = Result<Json<T>, AppError>;

/// Trait for adding context to Result types
/// Provides anyhow-style context chaining for better error messages
pub trait ResultExt<T> {
    /// Add context to an error
    fn context(self, msg: impl Into<String>) -> Result<T, AppError>;

    /// Add context using a closure (for lazy evaluation)
    fn with_context<F>(self, f: F) -> Result<T, AppError>
    where
        F: FnOnce() -> String;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context(self, msg: impl Into<String>) -> Result<T, AppError> {
        self.map_err(|e| AppError::internal(msg, e))
    }

    fn with_context<F>(self, f: F) -> Result<T, AppError>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| AppError::internal(f(), e))
    }
}

/// Trait for adding database context to Result types
pub trait DatabaseResultExt<T> {
    /// Add database context to an error
    fn db_context(self, msg: impl Into<String>) -> Result<T, AppError>;

    /// Add database context using a closure
    fn with_db_context<F>(self, f: F) -> Result<T, AppError>
    where
        F: FnOnce() -> String;
}

impl<T, E> DatabaseResultExt<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn db_context(self, msg: impl Into<String>) -> Result<T, AppError> {
        self.map_err(|e| AppError::database(msg, e))
    }

    fn with_db_context<F>(self, f: F) -> Result<T, AppError>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| AppError::database(f(), e))
    }
}

/// Convenience macro for creating database errors
#[macro_export]
macro_rules! db_error {
    ($msg:expr) => {
        $crate::error::AppError::DatabaseError {
            message: $msg.to_string(),
            source: None,
        }
    };
    ($msg:expr, $err:expr) => {
        $crate::error::AppError::database($msg, $err)
    };
}

/// Convenience macro for creating internal server errors
#[macro_export]
macro_rules! internal_error {
    ($msg:expr) => {
        $crate::error::AppError::InternalServerError {
            message: $msg.to_string(),
            source: None,
        }
    };
    ($msg:expr, $err:expr) => {
        $crate::error::AppError::internal($msg, $err)
    };
}

/// Convenience macro for creating external service errors
#[macro_export]
macro_rules! external_error {
    ($service:expr, $err:expr) => {
        $crate::error::AppError::external_service($service, $err)
    };
}

/// Quick bail with error
#[macro_export]
macro_rules! bail {
    ($err:expr) => {
        return Err($err)
    };
}

/// Quick bail with custom AppError variant
#[macro_export]
macro_rules! bail_bad_request {
    ($msg:expr) => {
        return Err($crate::error::AppError::BadRequest($msg.to_string()))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::error::AppError::BadRequest(format!($fmt, $($arg)*)))
    };
}

#[macro_export]
macro_rules! bail_not_found {
    ($msg:expr) => {
        return Err($crate::error::AppError::NotFound($msg.to_string()))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::error::AppError::NotFound(format!($fmt, $($arg)*)))
    };
}

#[macro_export]
macro_rules! bail_unauthorized {
    ($msg:expr) => {
        return Err($crate::error::AppError::Unauthorized($msg.to_string()))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::error::AppError::Unauthorized(format!($fmt, $($arg)*)))
    };
}

/// Application error type with context chaining support
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {message}")]
    DatabaseError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Internal server error: {message}")]
    InternalServerError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("External service error: {service}")]
    ExternalServiceError {
        service: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl AppError {
    /// Create a database error with context
    pub fn database<E>(message: impl Into<String>, error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::DatabaseError {
            message: message.into(),
            source: Some(Box::new(error)),
        }
    }

    /// Create an internal server error with context
    pub fn internal<E>(message: impl Into<String>, error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::InternalServerError {
            message: message.into(),
            source: Some(Box::new(error)),
        }
    }

    /// Create an external service error with context
    pub fn external_service<E>(service: impl Into<String>, error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::ExternalServiceError {
            service: service.into(),
            source: Some(Box::new(error)),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error_id: String,
    error_code: String,
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
    #[cfg(debug_assertions)]
    #[serde(skip_serializing_if = "Option::is_none")]
    debug_info: Option<DebugInfo>,
    #[cfg(debug_assertions)]
    #[serde(skip_serializing_if = "Option::is_none")]
    operation: Option<String>,
}

#[cfg(debug_assertions)]
#[derive(Serialize)]
struct DebugInfo {
    error_chain: Vec<String>,
    backtrace: Option<String>,
}

impl AppError {
    fn error_code(&self) -> &str {
        match self {
            AppError::DatabaseError { .. } => "DATABASE_ERROR",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::InternalServerError { .. } => "INTERNAL_SERVER_ERROR",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::ConfigError(_) => "CONFIG_ERROR",
            AppError::ExternalServiceError { .. } => "EXTERNAL_SERVICE_ERROR",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ExternalServiceError { .. } => StatusCode::BAD_GATEWAY,
        }
    }

    fn user_message(&self) -> String {
        match self {
            AppError::DatabaseError { .. } => "A database error occurred".to_string(),
            AppError::NotFound(msg) => msg.clone(),
            AppError::BadRequest(msg) => msg.clone(),
            AppError::Unauthorized(msg) => msg.clone(),
            AppError::InternalServerError { .. } => "An internal server error occurred".to_string(),
            AppError::ValidationError(msg) => msg.clone(),
            AppError::ConfigError(_) => "A configuration error occurred".to_string(),
            AppError::ExternalServiceError { service, .. } => {
                format!("External service '{}' is unavailable", service)
            }
        }
    }

    /// Log the error with full context chain
    fn log_with_context(&self, error_id: &str) {
        let error_code = self.error_code();

        // Log the main error
        match self {
            AppError::DatabaseError { message, source } |
            AppError::InternalServerError { message, source } => {
                tracing::error!(
                    error_id = %error_id,
                    error_code = %error_code,
                    message = %message,
                    "Error occurred"
                );

                // Log the error chain
                if let Some(src) = source {
                    let mut current: Option<&(dyn std::error::Error + 'static)> = Some(src.as_ref());
                    let mut depth = 1;
                    while let Some(err) = current {
                        tracing::error!(
                            error_id = %error_id,
                            depth = depth,
                            "Caused by: {}",
                            err
                        );
                        current = err.source();
                        depth += 1;
                    }
                }
            }
            AppError::ExternalServiceError { service, source } => {
                tracing::error!(
                    error_id = %error_id,
                    error_code = %error_code,
                    service = %service,
                    "External service error"
                );

                if let Some(src) = source {
                    let mut current: Option<&(dyn std::error::Error + 'static)> = Some(src.as_ref());
                    let mut depth = 1;
                    while let Some(err) = current {
                        tracing::error!(
                            error_id = %error_id,
                            depth = depth,
                            "Caused by: {}",
                            err
                        );
                        current = err.source();
                        depth += 1;
                    }
                }
            }
            _ => {
                tracing::debug!(
                    error_id = %error_id,
                    error_code = %error_code,
                    "Error: {}",
                    self
                );
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_id = Uuid::new_v4().to_string();
        let error_code = self.error_code().to_string();
        let status = self.status_code();

        // Log with full error context chain
        self.log_with_context(&error_id);

        #[cfg(debug_assertions)]
        let debug_info = {
            let mut error_chain = vec![format!("{}", self)];

            // Collect error chain
            let source: Option<&(dyn std::error::Error + 'static)> = match &self {
                AppError::DatabaseError { source, .. }
                | AppError::InternalServerError { source, .. }
                | AppError::ExternalServiceError { source, .. } => {
                    source.as_ref().map(|s| s.as_ref() as &(dyn std::error::Error + 'static))
                }
                _ => None,
            };

            if let Some(mut current) = source {
                loop {
                    error_chain.push(format!("{}", current));
                    match current.source() {
                        Some(src) => current = src,
                        None => break,
                    }
                }
            }

            Some(DebugInfo {
                error_chain,
                backtrace: std::env::var("RUST_BACKTRACE")
                    .ok()
                    .filter(|v| v == "1" || v == "full")
                    .map(|_| "Backtrace available in logs".to_string()),
            })
        };

        #[cfg(debug_assertions)]
        let operation = {
            // Try to extract operation from current span
            let span = tracing::Span::current();
            if span.is_none() {
                None
            } else {
                // Get the span name as operation
                Some(span.metadata().map(|m| m.name()).unwrap_or("unknown").to_string())
            }
        };

        let body = Json(ErrorResponse {
            error_id,
            error_code,
            error: self.user_message(),
            details: None,
            #[cfg(debug_assertions)]
            debug_info,
            #[cfg(debug_assertions)]
            operation,
        });

        (status, body).into_response()
    }
}

// Implement From traits for common error types
impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => AppError::NotFound("Resource not found".to_string()),
            _ => AppError::DatabaseError {
                message: "Database operation failed".to_string(),
                source: Some(Box::new(err)),
            },
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::ValidationError(err.to_string())
    }
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::ConfigError(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::ExternalServiceError {
            service: "HTTP client".to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::Unauthorized(format!("JWT error: {}", err))
    }
}
