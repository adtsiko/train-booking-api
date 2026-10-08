use crate::models::{Booking, BookingRequest, ErrorResponse, SeatSelection, User};
use utoipa::OpenApi;

#[utoipa::path(
    post,
    path = "/book",
    request_body = BookingRequest,
    responses(
        (status = 200, body = Booking),
        (status = 400, body = ErrorResponse),
        (status = 409, body = ErrorResponse),
        (status = 500, body = ErrorResponse),

    )
)]
fn book_doc() {}

#[derive(OpenApi)]
#[openapi(
    paths(book_doc),
    components(schemas(BookingRequest, Booking, User, SeatSelection, ErrorResponse)),
    tags((name = "train-booking-api", description = "Train booking API"))
)]
pub struct ApiDoc;
