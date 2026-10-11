use super::{DbBooking, DbInventory, DbRepository};
use crate::models::{BookingRecord, RepositoryError, SeatInventoryRecord, Train};
use async_trait::async_trait;
use sqlx::PgPool;
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
        journey_id: Uuid,
        coach: u32,
        seat: u32,
    ) -> Result<(), RepositoryError> {
        todo!();
    }
    async fn mark_seat_booked(
        &self,
        inventory_id: Uuid,
    ) -> Result<SeatInventoryRecord, RepositoryError> {
        todo!();
    }
}

impl InventoryRepository {
    pub async fn new(database_url: &str) -> std::result::Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
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
        journey_id: &Uuid,
        username: &String,
        email: &String,
        seat_inventory_ids: &Vec<SeatInventoryRecord>,
    ) -> Result<BookingRecord, RepositoryError> {
        todo!();
    }
}

impl BookingRepository {
    pub async fn new(database_url: &str) -> std::result::Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }
}
