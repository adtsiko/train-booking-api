use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;
use train_booking_api::config::Config;
use train_booking_api::repository::TrainCompanyPostgresRepository;
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
    let repo = TrainCompanyPostgresRepository::new(&config.database_url)
        .await
        .expect("failed to connect to postgres");
    let service = TrainService::new(repo);
    let app = routes::create_router(service);

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
