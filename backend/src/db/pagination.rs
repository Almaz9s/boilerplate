use crate::models::{PaginatedResponse, PaginationMeta, PaginationParams};

const MAX_PER_PAGE: i64 = 100;

/// Helper function to create pagination parameters
pub fn paginate_params(params: &PaginationParams) -> (i64, i64) {
    let page = params.page.max(1);
    let per_page = params.per_page.clamp(1, MAX_PER_PAGE);
    (page, per_page)
}

/// Calculate offset for pagination
pub fn calculate_offset(page: i64, per_page: i64) -> i64 {
    (page - 1) * per_page
}

/// Create a paginated response from data and total count
pub fn create_paginated_response<T>(
    data: Vec<T>,
    page: i64,
    per_page: i64,
    total: i64,
) -> PaginatedResponse<T> {
    let total_pages = if per_page > 0 {
        (total as f64 / per_page as f64).ceil() as i64
    } else {
        0
    };

    PaginatedResponse {
        data,
        pagination: PaginationMeta {
            page,
            per_page,
            total,
            total_pages,
        },
    }
}

/// Example usage in a handler:
/// ```rust
/// use diesel::prelude::*;
/// use diesel_async::RunQueryDsl;
///
/// // Get pagination params
/// let (page, per_page) = paginate_params(&params);
/// let offset = calculate_offset(page, per_page);
///
/// // Count total
/// let total: i64 = users::table
///     .count()
///     .get_result(&mut conn)
///     .await?;
///
/// // Get paginated results
/// let data = users::table
///     .limit(per_page)
///     .offset(offset)
///     .load::<User>(&mut conn)
///     .await?;
///
/// // Create response
/// let response = create_paginated_response(data, page, per_page, total);
/// ```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paginate_params() {
        let params = PaginationParams {
            page: 1,
            per_page: 20,
        };
        let (page, per_page) = paginate_params(&params);
        assert_eq!(page, 1);
        assert_eq!(per_page, 20);
    }

    #[test]
    fn test_paginate_params_max_per_page() {
        let params = PaginationParams {
            page: 1,
            per_page: 200, // Over max
        };
        let (_, per_page) = paginate_params(&params);
        assert_eq!(per_page, MAX_PER_PAGE);
    }

    #[test]
    fn test_paginate_params_min_page() {
        let params = PaginationParams {
            page: -1, // Invalid
            per_page: 20,
        };
        let (page, _) = paginate_params(&params);
        assert_eq!(page, 1);
    }
}
