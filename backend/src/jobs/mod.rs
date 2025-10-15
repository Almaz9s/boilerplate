use crate::AppState;
use tokio_cron_scheduler::{Job, JobScheduler};
use std::sync::Arc;

pub mod tasks;

/// Initialize and start the job scheduler
pub async fn init_scheduler(state: Arc<AppState>) -> Result<JobScheduler, Box<dyn std::error::Error>> {
    let scheduler = JobScheduler::new().await?;

    // Example: Run a task every hour
    let state_clone = state.clone();
    let cleanup_job = Job::new_async("0 0 * * * *", move |_uuid, _lock| {
        let state = state_clone.clone();
        Box::pin(async move {
            tracing::info!("Running scheduled cleanup task");
            if let Err(e) = tasks::cleanup_old_records(state).await {
                tracing::error!("Cleanup task failed: {}", e);
            }
        })
    })?;
    scheduler.add(cleanup_job).await?;

    // Example: Run a task every 5 minutes
    let state_clone = state.clone();
    let health_check_job = Job::new_async("0 */5 * * * *", move |_uuid, _lock| {
        let state = state_clone.clone();
        Box::pin(async move {
            tracing::debug!("Running periodic health check");
            if let Err(e) = tasks::periodic_health_check(state).await {
                tracing::error!("Health check task failed: {}", e);
            }
        })
    })?;
    scheduler.add(health_check_job).await?;

    scheduler.start().await?;
    tracing::info!("Job scheduler started successfully");

    Ok(scheduler)
}

/// Gracefully shutdown the scheduler
pub async fn shutdown_scheduler(mut scheduler: JobScheduler) {
    if let Err(e) = scheduler.shutdown().await {
        tracing::error!("Error shutting down scheduler: {}", e);
    } else {
        tracing::info!("Job scheduler shutdown complete");
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_scheduler_initialization() {
        // Test that scheduler can be initialized
        // In real tests, you'd use a test database and config
    }
}
