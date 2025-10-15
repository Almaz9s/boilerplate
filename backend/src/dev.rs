//! Development-only utilities and debugging helpers
//!
//! This module is only compiled in debug builds and provides useful tools
//! for solo development, debugging, and testing.

use axum::http::Request;

/// Setup a better panic handler for development mode
///
/// This provides clearer panic messages with helpful hints for solo developers.
/// Call this early in main() after initializing tracing.
pub fn setup_dev_panic_handler() {
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("\n");
        eprintln!("╔═══════════════════════════════════════════════════════════════╗");
        eprintln!("║              💥 PANIC IN DEVELOPMENT MODE 💥                 ║");
        eprintln!("╚═══════════════════════════════════════════════════════════════╝");
        eprintln!();

        // Print panic location
        if let Some(location) = panic_info.location() {
            eprintln!("📍 Location: {}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            );
        }

        // Print panic message
        if let Some(msg) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("📝 Message: {}", msg);
        } else if let Some(msg) = panic_info.payload().downcast_ref::<String>() {
            eprintln!("📝 Message: {}", msg);
        } else {
            eprintln!("📝 Message: <non-string panic payload>");
        }

        eprintln!();
        eprintln!("💡 Debugging tips:");
        eprintln!("   • Check logs: tracing_test_output.log");
        eprintln!("   • Run with: RUST_BACKTRACE=1 cargo run");
        eprintln!("   • For full backtrace: RUST_BACKTRACE=full cargo run");
        eprintln!("   • Use /dev/state endpoint to check app state");
        eprintln!();
        eprintln!("────────────────────────────────────────────────────────────────");
        eprintln!();
    }));
}

/// Print detailed request information for debugging
pub fn print_request_details<B>(req: &Request<B>) {
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("📥 Request Details");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("Method: {}", req.method());
    eprintln!("URI: {}", req.uri());
    eprintln!("Version: {:?}", req.version());
    eprintln!("\nHeaders:");
    for (name, value) in req.headers() {
        if let Ok(val) = value.to_str() {
            eprintln!("  {}: {}", name, val);
        }
    }
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

/// Print formatted JSON for debugging
pub fn debug_json<T: serde::Serialize>(label: &str, data: &T) {
    if let Ok(json) = serde_json::to_string_pretty(data) {
        eprintln!("🔍 {} =>\n{}\n", label, json);
    }
}

/// Measure execution time of a block
pub struct Timer {
    label: String,
    start: std::time::Instant,
}

impl Timer {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            start: std::time::Instant::now(),
        }
    }

    pub fn lap(&self) -> std::time::Duration {
        self.start.elapsed()
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        eprintln!("⏱️  {} took: {:?}", self.label, elapsed);
    }
}

/// Pretty-print SQL queries for debugging (mock for demonstration)
pub fn log_query(query: &str, params: Option<Vec<String>>) {
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("💾 SQL Query");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("{}", query);
    if let Some(p) = params {
        eprintln!("\nParams: {:?}", p);
    }
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

/// Generate test data helpers
pub mod fixtures {
    use uuid::Uuid;

    pub fn test_uuid() -> Uuid {
        Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
    }

    pub fn test_email(index: usize) -> String {
        format!("test.user{}@example.com", index)
    }

    pub fn test_username(index: usize) -> String {
        format!("testuser{}", index)
    }

    pub fn test_password() -> String {
        "TestPassword123!".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer() {
        let timer = Timer::new("test operation");
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(timer.lap() >= std::time::Duration::from_millis(10));
    }

    #[test]
    fn test_fixtures() {
        assert_eq!(fixtures::test_email(1), "test.user1@example.com");
        assert_eq!(fixtures::test_username(1), "testuser1");
        assert!(!fixtures::test_password().is_empty());
    }
}
