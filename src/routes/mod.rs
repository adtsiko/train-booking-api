use axum::{routing::{get, post}, Router};

use crate::handlers::train_system::{booking, health, readiness};
use crate::service::TrainService;
use crate::repository::train_repository::TrainRepository;

pub fn create_router<R: TrainRepository + Clone + Send + Sync + 'static>(service: TrainService<R>) -> Router {
    Router::new()
        .route("/book", post(booking))
        .route("/health", get(health))
        .route("/ready", get(readiness))
        .with_state(service)
}