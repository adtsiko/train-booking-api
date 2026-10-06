use axum::{http::StatusCode, Json};

use crate::models::{BookingRequest, Booking, BookingError, ErrorResponse};
use crate::service::TrainService;

pub async fn booking(
    axum::extract::State(service): axum::extract::State<TrainService>,
    Json(body): Json<BookingRequest>,
) -> Result<Json<Booking>, (StatusCode, Json<ErrorResponse>)> {
    service.
        book_trip(body)
        .map(Json)
        .map_err(|e| {
            let (status, message) = match e {
                BookingError::SeatTaken => (StatusCode::CONFLICT, "seat already taken"),
                BookingError::InvalidSeatNumber  => (StatusCode::BAD_REQUEST, "invalid seat"),
                BookingError::NoSeatsAvailable => (StatusCode::BAD_REQUEST, "no seats available"),
                BookingError::InvalidWagonNumber => (StatusCode::BAD_REQUEST, "invalid wagon"),
                BookingError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal error"),
            };

            (status, Json(ErrorResponse {error: message.to_string() }))
})

}