use super::{DbBooking, DbInventory, DbRepository};
use crate::models::{
    Booking, BookingRecord, BookingRow, RepositoryError, SeatInventoryRecord, SeatInventoryRow,
    SeatSelection, SeatStatus,
};
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[derive(Clone)]
pub struct InventoryRepository {
    pool: PgPool,
}

#[derive(Clone)]
pub struct BookingRepository {
    pool: PgPool,
}

#[async_trait]
impl DbRepository for InventoryRepository {
    fn pool(&self) -> &PgPool {
        &self.pool
    }
}

#[async_trait]
impl DbInventory for InventoryRepository {
    async fn find_seat_inventory_for_update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        journey_id: Uuid,
        coach: u32,
        seat: u32,
    ) -> Result<SeatInventoryRecord, RepositoryError> {
        let row = sqlx::query_file_as!(
            SeatInventoryRow,
            "sql/inventory/find_seat_inventory_for_update.sql",
            journey_id,
            coach as i32,
            seat as i32
        )
        .fetch_optional(&mut **tx)
        .await
        .map_err(RepositoryError::Database)?
        .ok_or(RepositoryError::Database(sqlx::Error::RowNotFound))?;

        Ok(SeatInventoryRecord {
            id: row.id,
            journey_id: row.journey_id,
            seat_id: row.seat_id,
            status: SeatStatus::from_db(&row.status)?,
        })
    }
    async fn mark_seat_booked(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        inventory_id: Uuid,
    ) -> Result<SeatInventoryRecord, RepositoryError> {
        let row = sqlx::query_file_as!(
            SeatInventoryRow,
            "sql/inventory/mark_seat_booked_update.sql",
            inventory_id
        )
        .fetch_optional(&mut **tx)
        .await
        .map_err(RepositoryError::Database)?
        .ok_or(RepositoryError::Database(sqlx::Error::RowNotFound))?;

        Ok(SeatInventoryRecord {
            id: row.id,
            journey_id: row.journey_id,
            seat_id: row.seat_id,
            status: SeatStatus::from_db(&row.status)?,
        })
    }
}

impl InventoryRepository {
    pub async fn new(pool: PgPool) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self { pool })
    }
}

#[async_trait]
impl DbRepository for BookingRepository {
    fn pool(&self) -> &PgPool {
        &self.pool
    }
}

#[async_trait]
impl DbBooking for BookingRepository {
    async fn create_booking(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        journey_id: &Uuid,
        username: &String,
        email: &String,
        seat_inventory_ids: &Vec<SeatInventoryRecord>,
    ) -> Result<BookingRecord, RepositoryError> {
        let booking =
            sqlx::query_file_as!(BookingRecord, "sql/booking/create_booking.sql", journey_id)
                .fetch_one(&mut **tx)
                .await
                .map_err(RepositoryError::Database)?;

        sqlx::query_file!(
            "sql/booking/create_passanger.sql",
            booking.id,
            username,
            email
        )
        .execute(&mut **tx)
        .await
        .map_err(RepositoryError::Database)?;

        for seat in seat_inventory_ids {
            sqlx::query_file!("sql/booking/create_booking_seat.sql", booking.id, seat.id)
                .execute(&mut **tx)
                .await
                .map_err(RepositoryError::Database)?;
        }

        Ok(booking)
    }

    async fn get_bookings_by_email(&self, email: &str) -> Result<Vec<Booking>, RepositoryError> {
        let rows = sqlx::query_file_as!(BookingRow, "sql/booking/get_booking.sql", email)
            .fetch_all(&self.pool)
            .await
            .map_err(RepositoryError::Database)?;

        let bookings = rows
            .into_iter()
            .map(|row| {
                let booked_seats: Vec<SeatSelection> = serde_json::from_value(row.booked_seats)
                    .map_err(|e| RepositoryError::Database(sqlx::Error::Decode(e.into())))
                    .expect("booked seats");
                Booking {
                    id: row.id,
                    journey_id: row.journey_id,
                    created_at: row.created_at,
                    booked_seats,
                }
            })
            .collect();

        Ok(bookings)
    }
}

impl BookingRepository {
    pub async fn new(pool: PgPool) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self { pool })
    }
}
