use axum::{Json, http::StatusCode};

use crate::models::{Booking, BookingError, BookingRequest, ErrorResponse, HealthResponse};
use crate::repository::train_repository::TrainRepository;
use crate::service::TrainService;

pub async fn booking<R: TrainRepository + Clone>(
    axum::extract::State(service): axum::extract::State<TrainService<R>>,
    Json(body): Json<BookingRequest>,
) -> Result<Json<Booking>, (StatusCode, Json<ErrorResponse>)> {
    service.book_trip(body).await.map(Json).map_err(|e| {
        let (status, message) = match e {
            BookingError::SeatTaken => (StatusCode::CONFLICT, "seat already taken"),
            BookingError::InvalidSeatNumber => (StatusCode::BAD_REQUEST, "invalid seat"),
            BookingError::NoSeatsRequested => (StatusCode::BAD_REQUEST, "no seat(s) requested"),
            BookingError::InvalidWagonNumber => (StatusCode::BAD_REQUEST, "invalid wagon"),
            BookingError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal error"),
            BookingError::Repository(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal error"),
        };

        (
            status,
            Json(ErrorResponse {
                error: message.to_string(),
            }),
        )
    })
}

pub async fn health() -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "ok".to_string(),
        }),
    )
}

pub async fn readiness<R: TrainRepository + Clone>(
    axum::extract::State(service): axum::extract::State<TrainService<R>>,
) -> (StatusCode, Json<HealthResponse>) {
    match service.check_readiness().await {
        Ok(()) => (
            StatusCode::OK,
            Json(HealthResponse {
                status: "ok".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthResponse {
                status: "Not ready".to_string(),
            }),
        ),
    }
}
