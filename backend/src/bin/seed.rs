use backend::{config::Config, db};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize basic logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    tracing::info!("Database seeding tool");

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Loaded configuration");

    // Create database pool
    let pool = db::create_pool(&config.database.url, 1)?;
    tracing::info!("Connected to database");

    // Check for --clear flag
    let args: Vec<String> = env::args().collect();
    let clear_existing = args.contains(&"--clear".to_string());

    if clear_existing {
        tracing::warn!("⚠️  --clear flag detected: This will DELETE all existing data!");
        tracing::warn!("Press Ctrl+C within 3 seconds to cancel...");
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }

    let seed_config = db::seed::SeedConfig { clear_existing };

    // Run seeding
    db::seed::seed_database(&pool, seed_config).await?;

    tracing::info!("✓ Database seeded successfully");
    tracing::info!("");
    tracing::info!("Test accounts created:");
    tracing::info!("  Email: admin@example.com    | Password: 123");
    tracing::info!("  Email: user@example.com     | Password: 123");
    tracing::info!("  Email: test@example.com     | Password: 123");
    tracing::info!("");
    tracing::info!("Run with --clear to delete existing data before seeding");

    Ok(())
}
