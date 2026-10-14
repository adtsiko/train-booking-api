use crate::models::http_models::SeatStatus;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct TrainServiceRecord {
    pub id: Uuid,
    pub code: String,
    pub name: String,
}

pub struct JourneyRecord {
    pub id: Uuid,
    pub service_id: Uuid,
    pub departure_at: DateTime<Utc>,
    pub origin: String,
    pub destination: String,
}

pub struct CoachRecord {
    pub id: Uuid,
    pub journey_id: Uuid,
    pub coach_number: u32,
}

pub struct SeatRecord {
    pub id: Uuid,
    pub coach_id: Uuid,
    pub seat_number: u32,
}

pub struct SeatInventoryRecord {
    pub id: Uuid,
    pub journey_id: Uuid,
    pub seat_id: Uuid,
    pub status: SeatStatus,
}

#[derive(sqlx::FromRow)]
pub struct SeatInventoryRow {
    pub id: Uuid,
    pub journey_id: Uuid,
    pub seat_id: Uuid,
    pub status: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct BookingRecord {
    pub id: Uuid,
    pub journey_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
pub struct BookingRow {
    pub id: Uuid,
    pub journey_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub booked_seats: serde_json::Value,
}

pub struct PassengerRecord {
    pub id: Uuid,
    pub booking_id: Uuid,
    pub full_name: String,
    pub email: String,
}

pub struct BookingSeatRecord {
    pub booking_id: Uuid,
    pub seat_inventory_id: Uuid,
}
