use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use train_booking_api::models::{HealthResponse};

fn test_app(path: &str) -> axum::Router {
    let repo = train_booking_api::repository::TrainCompanyFileRepository::new(path.to_string());
    let service = train_booking_api::service::TrainService::new(repo);
    train_booking_api::routes::create_router(service)
}

#[tokio::test]
async fn healt_returns_200() {
    let app = test_app("test-health.json");

    let response = app
        .oneshot(Request::get("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: HealthResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(json.status, "ok");


}