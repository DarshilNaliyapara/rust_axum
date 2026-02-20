use tracing::error;
use crate::error::AppError;

/// Executes any CPU-heavy, synchronous task on a background thread.
pub async fn async_task<F, R>(task: F) -> Result<R, AppError>
where
    F: FnOnce() -> R + Send + 'static, // The task must be a function that can be sent across threads
    R: Send + 'static,                 // The result must also be safe to send back across threads
{
    tokio::task::spawn_blocking(task)
        .await
        .map_err(|e| {
            error!(error = %e, "A background CPU task panicked or was cancelled");
            AppError::InternalServerError
        })
}
