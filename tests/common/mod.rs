use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header::CONTENT_TYPE};
use bytes::Bytes;
use http_body_util::BodyExt;
use std::fs;
use std::path::Path;
use tower::ServiceExt;
use train_booking_api::models::{BookingRequest, SeatSelection};

pub fn test_app(path: &str) -> axum::Router {
    let repo = train_booking_api::repository::TrainCompanyFileRepository::new(path.to_string());
    let service = train_booking_api::service::TrainService::new(repo);
    clear_test_files(path);
    train_booking_api::routes::create_router(service)
}

fn clear_test_files(path: &str) {
    if Path::new(path).exists() {
        fs::remove_file(path).unwrap();
    }
}

pub fn create_booking(user_id: &str, requested_seats: Vec<SeatSelection>) -> BookingRequest {
    BookingRequest {
        username: format!("test{}", user_id),
        email: format!("test{}@user.com", user_id),
        requested_seats,
    }
}

pub async fn post_request<R: serde::Serialize>(body: R, app: &Router) -> (Bytes, StatusCode) {
    let body = serde_json::to_string(&body).unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/book")
                .header(CONTENT_TYPE, "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let response_bytes = response.into_body().collect().await.unwrap().to_bytes();
    (response_bytes, status)
}
