use crate::handlers::train_system::{booking, health, readiness};
use crate::openapi::ApiDoc;
use crate::repository::{DbBooking, DbInventory};
use crate::service::TrainService;
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn create_router<
    I: DbInventory + Clone + Send + Sync + 'static,
    B: DbBooking + Clone + Send + Sync + 'static,
>(
    service: TrainService<I, B>,
) -> Router {
    Router::new()
        .route("/book", post(booking))
        .route("/health", get(health))
        .route("/ready", get(readiness))
        .with_state(service)
        .layer(TraceLayer::new_for_http())
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
