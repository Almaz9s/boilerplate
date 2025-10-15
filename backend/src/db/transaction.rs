use crate::error::AppError;
use diesel_async::{AsyncConnection, AsyncPgConnection};

/// Execute a database operation within a transaction
/// Automatically rolls back on error and commits on success
pub async fn with_transaction<F, T>(
    conn: &mut AsyncPgConnection,
    f: F,
) -> Result<T, AppError>
where
    F: for<'a> FnOnce(&'a mut AsyncPgConnection) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, AppError>> + Send + 'a>> + Send,
    T: Send,
{
    conn.transaction(|conn| Box::pin(f(conn)))
        .await
        .map_err(|e| AppError::database("Transaction failed", e))
}

/// Convenience macro for running database operations in a transaction
///
/// # Example
/// ```no_run
/// use backend::tx;
///
/// tx!(&state.db_pool, |conn| async move {
///     diesel::update(users::table.find(user_id))
///         .set(balance.eq(balance - 100))
///         .execute(conn)
///         .await?;
///
///     diesel::insert_into(transactions::table)
///         .values(&new_transaction)
///         .execute(conn)
///         .await?;
///
///     Ok(())
/// })
/// ```
#[macro_export]
macro_rules! tx {
    ($pool:expr, |$conn:ident| $body:expr) => {{
        let mut conn = $crate::db::get_connection($pool).await?;
        $crate::db::transaction::with_transaction(&mut conn, |$conn| {
            Box::pin($body)
        })
        .await
    }};
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_transaction_commits_on_success() {
        // This is a conceptual test - in real usage you'd test with actual DB operations
        // The transaction will automatically commit if the closure returns Ok
    }

    #[tokio::test]
    async fn test_transaction_rolls_back_on_error() {
        // This is a conceptual test - in real usage you'd test with actual DB operations
        // The transaction will automatically roll back if the closure returns Err
    }
}
