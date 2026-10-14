use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header::CONTENT_TYPE};
use bytes::Bytes;
use http_body_util::BodyExt;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, ImageExt};
use testcontainers_modules::postgres::Postgres;
use tokio::sync::OnceCell;
use tower::ServiceExt;
use train_booking_api::models::{BookingRequest, SeatSelection};
use train_booking_api::repository::{BookingRepository, InventoryRepository};
use uuid::Uuid;

struct TestDb {
    url: String,
    _container: ContainerAsync<Postgres>,
}

static TEST_DB: OnceCell<TestDb> = OnceCell::const_new();

async fn database_url() -> &'static str {
    &TEST_DB
        .get_or_init(|| async {
            let container = Postgres::default()
                .with_tag("16-alpine")
                .start()
                .await
                .expect("start postgres");
            let port = container.get_host_port_ipv4(5432).await.expect("port");
            let url = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");

            let pool = PgPoolOptions::new()
                .max_connections(5)
                .acquire_timeout(Duration::from_secs(30))
                .connect(&url)
                .await
                .expect("connect");

            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .expect("migrate");

            pool.close().await;

            TestDb {
                url,
                _container: container,
            }
        })
        .await
        .url
}

async fn new_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url().await)
        .await
        .expect("connect")
}

pub async fn test_app() -> axum::Router {
    let pool = new_pool().await;

    let booking_repo = BookingRepository::new(pool.clone())
        .await
        .expect("failed to connect to postgres");
    let inventory_repo = InventoryRepository::new(pool)
        .await
        .expect("failed to connect to postgres");
    let service = train_booking_api::service::TrainService::new(inventory_repo, booking_repo);

    train_booking_api::routes::create_router(service)
}

#[allow(dead_code)]
pub fn create_booking(user_id: &str, requested_seats: Vec<SeatSelection>) -> BookingRequest {
    BookingRequest {
        journey_id: Uuid::parse_str("22222222-2222-2222-2222-222222222222")
            .expect("valid seeded journey id"),
        username: format!("test{}", user_id),
        email: format!("test{}@user.com", user_id),
        requested_seats,
    }
}

#[allow(dead_code)]
pub async fn post_request<R: serde::Serialize>(body: R, app: &Router) -> (Bytes, StatusCode) {
    let body = serde_json::to_string(&body).unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/bookings")
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

#[allow(dead_code)]
pub async fn get_request(email: &str, app: &Router) -> (Bytes, StatusCode) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/bookings?email={email}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let response_bytes = response.into_body().collect().await.unwrap().to_bytes();
    (response_bytes, status)
}
