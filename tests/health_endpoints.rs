use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use train_booking_api::models::HealthResponse;
mod common;
use common::test_app;

#[tokio::test]
#[ignore = "Implementation not complete"]
async fn health_returns_200() {
    let app = test_app("test-health.json").await;

    let response = app
        .oneshot(Request::get("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: HealthResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(json.status, "ok");
}
