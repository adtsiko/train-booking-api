use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
pub enum SeatStatus {
    Booked,
    Available,
    Held,
}

impl SeatStatus {
    pub fn from_db(value: &str) -> Result<Self, RepositoryError> {
        match value {
            "available" => Ok(SeatStatus::Available),
            "held" => Ok(SeatStatus::Held),
            "booked" => Ok(SeatStatus::Booked),
            other => Err(RepositoryError::Database(sqlx::Error::Protocol(
                format!("unknown seat_status: {other}").into(),
            ))),
        }
    }
}

#[derive(Debug)]
pub enum RepositoryError {
    Io(std::io::Error),
    Database(sqlx::Error),
}

#[derive(Debug)]
pub enum BookingError {
    SeatTaken,
    InvalidSeatNumber,
    NoSeatsRequested,
    InvalidCoachNumber,
    Io(std::io::Error),
    Repository(RepositoryError),
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
}

impl From<std::io::Error> for BookingError {
    fn from(err: std::io::Error) -> Self {
        BookingError::Io(err)
    }
}

impl From<RepositoryError> for BookingError {
    fn from(err: RepositoryError) -> Self {
        BookingError::Repository(err)
    }
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Seat {
    pub number: u32,
    pub status: SeatStatus,
}

#[derive(Debug, Deserialize, Serialize, Copy, PartialEq, Clone, ToSchema)]
pub struct SeatSelection {
    pub coach_number: u32,
    pub seat_number: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Coach {
    pub coach_number: u32,
    pub seats: Vec<Seat>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Train {
    pub coaches: Vec<Coach>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct User {
    pub email: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Booking {
    pub id: Uuid,
    pub journey_id: Uuid,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub booked_seats: Vec<SeatSelection>,
}
#[derive(Debug, Deserialize, IntoParams)]
pub struct BookingQuery {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct BookingRequest {
    pub journey_id: Uuid,
    pub username: String,
    pub email: String,
    pub requested_seats: Vec<SeatSelection>,
}

impl BookingRequest {
    pub fn new(
        journey_id: Uuid,
        username: &str,
        email: &str,
        requested_seats: Vec<SeatSelection>,
    ) -> Self {
        Self {
            journey_id,
            username: String::from(username),
            email: String::from(email),
            requested_seats,
        }
    }
}
