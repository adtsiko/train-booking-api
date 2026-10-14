pub mod db_models;
pub mod http_models;
pub use db_models::{BookingRecord, BookingRow, SeatInventoryRecord, SeatInventoryRow};
pub use http_models::{
    Booking, BookingError, BookingQuery, BookingRequest, Coach, ErrorResponse, HealthResponse,
    RepositoryError, Seat, SeatSelection, SeatStatus, Train, User,
};
