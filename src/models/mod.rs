pub mod db_models;
pub mod http_models;
pub use db_models::{BookingRecord, SeatInventoryRecord};
pub use http_models::{
    Booking, BookingError, BookingRequest, ErrorResponse, HealthResponse, RepositoryError, Seat,
    SeatSelection, SeatStatus, Train, User, Wagon,
};
