//! Developer convenience macros for solo development
//!
//! These macros make common development patterns more ergonomic.
//! Most are only active in debug builds to avoid overhead in production.

/// Pretty-print JSON values in logs (debug builds only)
///
/// # Example
/// ```
/// let user = get_user().await?;
/// dbg_json!(user);
/// // Logs: user = { "id": "123", "email": "test@example.com" }
/// ```
#[macro_export]
macro_rules! dbg_json {
    ($val:expr) => {
        #[cfg(debug_assertions)]
        {
            match serde_json::to_string_pretty(&$val) {
                Ok(json) => tracing::debug!("{} = {}", stringify!($val), json),
                Err(e) => tracing::debug!("{} = <serialization failed: {}>", stringify!($val), e),
            }
        }
    };
}

/// Enhanced todo! with automatic logging
///
/// # Example
/// ```
/// todo_with_msg!("Need to implement caching here");
/// // Logs a warning then panics with the message
/// ```
#[macro_export]
macro_rules! todo_with_msg {
    ($msg:expr) => {{
        #[cfg(debug_assertions)]
        tracing::warn!("TODO: {} ({}:{})", $msg, file!(), line!());
        todo!($msg)
    }};
}

/// Quick endpoint stub for prototyping
///
/// # Example
/// ```
/// stub_handler!(get_analytics);
/// // Creates: pub async fn get_analytics() -> Result<Json<Value>, AppError>
/// ```
#[macro_export]
macro_rules! stub_handler {
    ($name:ident) => {
        #[allow(dead_code)]
        pub async fn $name() -> Result<axum::Json<serde_json::Value>, $crate::error::AppError> {
            #[cfg(debug_assertions)]
            tracing::warn!("🚧 Stub handler called: {}", stringify!($name));
            Ok(axum::Json(serde_json::json!({
                "status": "not_implemented",
                "handler": stringify!($name)
            })))
        }
    };
}

/// Time a code block and log the duration
///
/// # Example
/// ```
/// time_block!("database query", {
///     let users = fetch_users().await?;
///     process(users)
/// });
/// // Logs: ⏱️  database query took: 45ms
/// ```
#[macro_export]
macro_rules! time_block {
    ($label:expr, $block:block) => {{
        let _timer = $crate::dev::Timer::new($label);
        $block
    }};
}

/// Log entry and exit of a function (debug builds only)
///
/// # Example
/// ```
/// fn process_data(id: &str) {
///     trace_fn!("process_data", id);
///     // ... function body
/// }
/// // Logs: → process_data(id="123")
/// // Logs: ← process_data [42ms]
/// ```
#[macro_export]
macro_rules! trace_fn {
    ($name:expr $(, $param:expr)*) => {
        #[cfg(debug_assertions)]
        {
            tracing::debug!("→ {}({})", $name, format!($("{:?} ",)* $(, $param)*).trim());
            let _guard = $crate::dev_macros::FnTracer::new($name);
        }
    };
}

/// Assert that a value matches a pattern, with helpful error message
///
/// # Example
/// ```
/// assert_matches!(result, Ok(_), "Expected successful result");
/// ```
#[macro_export]
macro_rules! assert_matches {
    ($expr:expr, $pat:pat, $msg:expr) => {
        match $expr {
            $pat => {},
            ref e => panic!("{}: got {:?}", $msg, e),
        }
    };
}

/// Quick database error with context
///
/// # Example
/// ```
/// let user = users.find(id)
///     .first(&mut conn)
///     .await
///     .map_err(db_err!("Failed to find user {}", id))?;
/// ```
#[macro_export]
macro_rules! db_err {
    ($msg:expr $(, $arg:expr)*) => {
        |e| $crate::error::AppError::database(format!($msg $(, $arg)*), e)
    };
}

/// Quick internal error with context
///
/// # Example
/// ```
/// parse_config().map_err(internal_err!("Config parsing failed"))?;
/// ```
#[macro_export]
macro_rules! internal_err {
    ($msg:expr $(, $arg:expr)*) => {
        |e| $crate::error::AppError::internal(format!($msg $(, $arg)*), e)
    };
}

#[cfg(debug_assertions)]
pub struct FnTracer {
    name: &'static str,
    start: std::time::Instant,
}

#[cfg(debug_assertions)]
impl FnTracer {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            start: std::time::Instant::now(),
        }
    }
}

#[cfg(debug_assertions)]
impl Drop for FnTracer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        tracing::debug!("← {} [{:?}]", self.name, elapsed);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_macros_compile() {
        // Just ensure macros compile
        #[cfg(debug_assertions)]
        {
            let data = serde_json::json!({"test": "value"});
            dbg_json!(data);
        }
    }
}
