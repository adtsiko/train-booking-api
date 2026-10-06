use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
pub enum SeatStatus {
    Booked,
    Available
}

pub enum BookingError {
    SeatTaken,
    InvalidSeatNumber,
    NoSeatsAvailable,
    InvalidWagonNumber,
    Io(std::io::Error),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl From<std::io::Error> for BookingError {
    fn from(err: std::io::Error) -> Self {
        BookingError::Io(err)
    }
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Seat {
    pub number: u32,
    pub status: SeatStatus 
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct SeatSelection {
    pub wagon_number: u32,
    pub seat_number: u32
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Wagon {
    pub wagon_number: u32,
    pub seats: Vec<Seat>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Train {
    pub wagons: Vec<Wagon>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub username: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Booking {
    pub user: User,
    pub booking_id: Uuid,
    pub booked_seats: Vec<SeatSelection> 
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BookingRequest {
    pub username: String,
    pub email: String,
    pub requested_seats: Vec<SeatSelection>
}

impl BookingRequest {

    pub fn new(username: &str, email: &str, requested_seats: Vec<SeatSelection>) -> Self {
        Self{
            username : String::from(username),
            email : String::from(email),
            requested_seats
        }
    }
}