pub mod postgres;
pub use postgres::{BookingRepository, InventoryRepository};

use crate::models::RepositoryError;
use crate::models::{
    Booking,
    db_models::{BookingRecord, SeatInventoryRecord},
};
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[async_trait]
pub trait DbRepository: Send + Sync + Clone {
    fn pool(&self) -> &PgPool;

    async fn ready(&self) -> Result<(), RepositoryError> {
        sqlx::query("SELECT 1")
            .execute(self.pool())
            .await
            .map_err(RepositoryError::Database)?;

        Ok(())
    }
}

#[async_trait]
pub trait DbInventory: DbRepository {
    async fn find_seat_inventory_for_update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        journey_id: Uuid,
        coach: u32,
        seat: u32,
    ) -> Result<SeatInventoryRecord, RepositoryError>;
    async fn mark_seat_booked(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        inventory_id: Uuid,
    ) -> Result<SeatInventoryRecord, RepositoryError>;
}

#[async_trait]
pub trait DbBooking: DbRepository {
    async fn create_booking(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        journey_id: &Uuid,
        username: &String,
        email: &String,
        seat_inventory_ids: &Vec<SeatInventoryRecord>,
    ) -> Result<BookingRecord, RepositoryError>;

    async fn get_bookings_by_email(&self, email: &str) -> Result<Vec<Booking>, RepositoryError>;
}
