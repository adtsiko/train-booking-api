use axum::{routing::{post}, Router};

use crate::handlers::train_system::{booking};
use crate::service::TrainService;

pub fn create_router(service: TrainService) -> Router {
    Router::new()
        .route("/book", post(booking))
        .with_state(service)
}