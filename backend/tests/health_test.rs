mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use backend::routes;
use tower::ServiceExt;

#[tokio::test]
async fn test_health_check() {
    let state = common::setup_test_state();
    let app = routes::create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
