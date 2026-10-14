use crate::models::{
    Booking, BookingError, BookingQuery, BookingRequest, ErrorResponse, HealthResponse,
};
use crate::repository::{DbBooking, DbInventory};
use crate::service::TrainService;
use axum::extract::{Query, State};
use axum::{Json, http::StatusCode};

pub async fn booking<I: DbInventory + Clone, B: DbBooking + Clone>(
    State(service): State<TrainService<I, B>>,
    Json(body): Json<BookingRequest>,
) -> Result<Json<Booking>, (StatusCode, Json<ErrorResponse>)> {
    service.book_trip(body).await.map(Json).map_err(|e| {
        let (status, message) = match e {
            BookingError::SeatTaken => (StatusCode::CONFLICT, "seat already taken"),
            BookingError::InvalidSeatNumber => (StatusCode::BAD_REQUEST, "invalid seat"),
            BookingError::NoSeatsRequested => (StatusCode::BAD_REQUEST, "no seat(s) requested"),
            BookingError::InvalidCoachNumber => (StatusCode::BAD_REQUEST, "invalid wagon"),
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

pub async fn fetch_bookings<I, B>(
    State(service): State<TrainService<I, B>>,
    Query(params): Query<BookingQuery>,
) -> Result<Json<Vec<Booking>>, (StatusCode, Json<ErrorResponse>)>
where
    I: DbInventory + Clone + Send + Sync + 'static,
    B: DbBooking + Clone + Send + Sync + 'static,
{
    let bookings = service
        .get_user_bookings(&params.email)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal error".to_string(),
                }),
            )
        })?;

    Ok(Json(bookings))
}

pub async fn health() -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "ok".to_string(),
        }),
    )
}

pub async fn readiness<I: DbInventory + Clone, B: DbBooking + Clone>(
    axum::extract::State(service): axum::extract::State<TrainService<I, B>>,
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
