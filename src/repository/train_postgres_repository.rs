use crate::models::{RepositoryError, Seat, SeatStatus, Train};
use crate::repository::train_repository::TrainRepository;
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Clone)]
pub struct TrainCompanyPostgresRepository {
    pool: PgPool,
}

#[async_trait]
impl TrainRepository for TrainCompanyPostgresRepository {
    async fn load_train(&self) -> Result<Vec<Train>, RepositoryError> {
        todo!();
    }
    async fn save_train(&self, train: &[Train]) -> Result<(), RepositoryError> {
        todo!();
    }
    async fn ready(&self) -> Result<(), RepositoryError> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(RepositoryError::Database)?;

        Ok(())
    }
}

impl TrainCompanyPostgresRepository {
    pub async fn new(database_url: &str) -> std::result::Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }
}
