use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::trace::{Config, TracerProvider};
use opentelemetry_sdk::Resource;
use opentelemetry_stdout::SpanExporter;
use tracing_subscriber::{fmt::writer::MakeWriterExt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use std::io;

/// Initialize simple tracing without OpenTelemetry
/// Perfect for development when you want cleaner, less verbose output
pub fn init_simple_tracing() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,backend=debug,tower_http=debug".into());

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .compact()
        .init();

    tracing::info!("Simple tracing initialized (no OpenTelemetry)");
    Ok(())
}

/// Initialize development tracing with both console and file output
/// Logs are written to both stdout and a file for easier debugging
pub fn init_dev_with_file_logging(log_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create log directory if it doesn't exist
    if let Some(parent) = std::path::Path::new(log_file_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_file_path)?;

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,backend=debug,tower_http=debug".into());

    // Write to both stdout and file
    let writer = io::stdout.and(file);

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_writer(writer)
        .compact()
        .init();

    tracing::info!("Development tracing initialized with file logging: {}", log_file_path);
    Ok(())
}

/// Initialize tracing with OpenTelemetry support
/// Returns a guard that must be kept alive for the duration of the program
pub fn init_tracing(
    service_name: &str,
    environment: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // In development, use simple logs by default for better DX
    // Set OTEL=1 to enable full OpenTelemetry in development
    // In production, always use full OpenTelemetry tracing
    let use_simple = environment != "production" && std::env::var("OTEL").is_err();

    if use_simple {
        tracing::info!("💡 Using simple tracing (set OTEL=1 for full OpenTelemetry, or ENVIRONMENT=production)");
        return init_simple_tracing();
    }
    // Create a resource with service information
    let resource = Resource::new(vec![
        KeyValue::new("service.name", service_name.to_string()),
        KeyValue::new("service.version", env!("CARGO_PKG_VERSION").to_string()),
        KeyValue::new("deployment.environment", environment.to_string()),
    ]);

    // Create a stdout exporter for spans (for development)
    // In production, you would use OTLP exporter to send to Jaeger/Tempo/etc
    let exporter = SpanExporter::default();

    // Build tracer provider with the exporter
    let provider = TracerProvider::builder()
        .with_config(Config::default().with_resource(resource))
        .with_simple_exporter(exporter)
        .build();

    // Get a tracer from the provider
    let tracer = provider.tracer("backend");

    // Set the global tracer provider
    global::set_tracer_provider(provider);

    // Set up environment filter for logs
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,backend=debug,tower_http=debug".into());

    // Create OpenTelemetry layer
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // Determine log format based on environment
    if environment == "production" {
        // JSON formatting for production (structured logging)
        tracing_subscriber::registry()
            .with(env_filter)
            .with(telemetry_layer)
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true),
            )
            .init();
    } else {
        // Pretty formatting for development
        tracing_subscriber::registry()
            .with(env_filter)
            .with(telemetry_layer)
            .with(
                tracing_subscriber::fmt::layer()
                    .pretty()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true),
            )
            .init();
    }

    Ok(())
}

/// Shutdown tracing and flush any pending spans
pub async fn shutdown_tracing() {
    tracing::info!("Shutting down tracing");
    global::shutdown_tracer_provider();
}

/// Helper macro for creating instrumented async functions
/// Usage: instrument_async!(function_name, param1, param2)
#[macro_export]
macro_rules! instrument_async {
    ($name:expr) => {
        tracing::info_span!($name)
    };
    ($name:expr, $($key:expr => $value:expr),* $(,)?) => {
        tracing::info_span!($name, $($key = %$value),*)
    };
}

/// Helper for adding context to the current span
pub fn add_span_context(key: &str, value: &str) {
    tracing::Span::current().record(key, value);
}

/// Helper for recording errors in spans
#[allow(dead_code)]
pub fn record_error(error: &dyn std::error::Error) {
    let span = tracing::Span::current();
    span.record("error", true);
    span.record("error.message", &error.to_string() as &str);

    // Record error chain if available
    let mut current = error.source();
    let mut depth = 1;
    while let Some(source) = current {
        let key = format!("error.cause.{}", depth);
        span.record(&key as &str, &source.to_string() as &str);
        current = source.source();
        depth += 1;
    }
}
