use std::io::Result;
use crate::models::{Train};
pub trait TrainRepository {
    fn load_train(&self) -> Result<Vec<Train>>;
    fn save_train(&self, train: &[Train]) -> Result<()>;
}

