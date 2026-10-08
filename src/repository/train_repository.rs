use crate::models::Train;
use std::io::Result;
pub trait TrainRepository {
    fn load_train(&self) -> Result<Vec<Train>>;
    fn save_train(&self, train: &[Train]) -> Result<()>;
    fn ready(&self) -> Result<()>;
}
