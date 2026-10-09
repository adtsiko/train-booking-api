use crate::models::{RepositoryError, Train};
use async_trait::async_trait;

#[async_trait]
pub trait TrainRepository: Send + Sync + Clone {
    async fn load_train(&self) -> Result<Vec<Train>, RepositoryError>;
    async fn save_train(&self, train: &[Train]) -> Result<(), RepositoryError>;
    async fn ready(&self) -> Result<(), RepositoryError>;
}
