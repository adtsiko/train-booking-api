use crate::models::{
    Booking, BookingQuery, BookingRequest, ErrorResponse, HealthResponse, SeatSelection, User,
};
use utoipa::OpenApi;

#[allow(dead_code)]
#[utoipa::path(
    post,
    path = "/bookings",
    request_body = BookingRequest,
    responses(
        (status = 200, body = Booking),
        (status = 400, body = ErrorResponse),
        (status = 409, body = ErrorResponse),
        (status = 500, body = ErrorResponse),

    )
)]
fn book_doc() {}

#[allow(dead_code)]
#[utoipa::path(
    get,
    path = "/bookings",
    params(BookingQuery),
    responses(
        (status = 200, body = Vec<Booking>),
        (status = 500, body = ErrorResponse)
    )
)]
fn fetch_bookings() {}

#[allow(dead_code)]
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, body = HealthResponse),
        (status = 500, body = ErrorResponse)
    )
)]
fn app_health() {}

#[allow(dead_code)]
#[utoipa::path(
    get,
    path = "/ready",
    responses(
        (status = 200, body = HealthResponse),
        (status = 500, body = ErrorResponse)
    )
)]
fn app_readiness() {}

#[derive(OpenApi)]
#[openapi(
    paths(book_doc, fetch_bookings, app_health, app_readiness),
    components(schemas(BookingRequest, Booking, User, SeatSelection, ErrorResponse)),
    tags((name = "train-booking-api", description = "Train booking API"))
)]
pub struct ApiDoc;
