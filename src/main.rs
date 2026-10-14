use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;
use train_booking_api::config::Config;
use train_booking_api::repository::{BookingRepository, InventoryRepository};
use train_booking_api::routes;
use train_booking_api::service::TrainService;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info, tower_http=debug")),
        )
        .init();

    let config = Config::from_env();
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("failed to connect to postgres");
    let booking_repo = BookingRepository::new(pool.clone())
        .await
        .expect("failed to create booking");
    let inventory_repo = InventoryRepository::new(pool)
        .await
        .expect("failed to create inventory repo");
    let service = TrainService::new(inventory_repo, booking_repo);
    let app = routes::create_router(service);

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
