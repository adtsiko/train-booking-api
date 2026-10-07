use axum::{routing::{post}, Router};

use crate::handlers::train_system::{booking};
use crate::service::TrainService;
use crate::repository::train_repository::TrainRepository;

pub fn create_router<R: TrainRepository + Clone + Send + Sync + 'static>(service: TrainService<R>) -> Router {
    Router::new()
        .route("/book", post(booking))
        .with_state(service)
}