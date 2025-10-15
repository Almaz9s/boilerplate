//! Query logging utilities for development
//!
//! Provides macros to log SQL queries in debug builds when QUERY_LOG=1 is set.
//! This helps identify slow queries and debug N+1 query problems.

/// Execute a query and log it (only in debug builds with QUERY_LOG=1)
///
/// Automatically logs query execution time and result count when applicable.
///
/// Usage:
/// ```rust
/// use crate::db::query_log::logged_query;
///
/// let user = logged_query!(
///     "SELECT * FROM users WHERE id = $1",
///     users::table.find(id).first::<User>(&mut conn).await
/// )?;
///
/// let users = logged_query!(
///     "SELECT * FROM users LIMIT 10",
///     users::table.limit(10).load::<User>(&mut conn).await
/// )?;
/// ```
#[macro_export]
macro_rules! logged_query {
    ($description:expr, $query:expr) => {{
        #[cfg(debug_assertions)]
        {
            let start = std::time::Instant::now();
            let result = $query;
            let duration = start.elapsed();

            // Log the query execution
            $crate::db::log_query($description, duration);

            result
        }
        #[cfg(not(debug_assertions))]
        {
            $query
        }
    }};
}

/// Convenience re-export for use without macro_use
pub use crate::logged_query;
