use crate::models::{RepositoryError, Seat, SeatStatus, Train, Wagon};
use crate::repository::train_repository::TrainRepository;
use async_trait::async_trait;
use std::fs;
use std::io::{self, Result as IoResult, Write};
use std::path::Path;

#[derive(Clone)]
pub struct TrainCompanyFileRepository {
    path: String,
}

const WAGON_NUMBER: u32 = 4;
const NUMBER_OF_SEATS_PER_WAGON: u32 = 50;

#[async_trait]
impl TrainRepository for TrainCompanyFileRepository {
    async fn load_train(&self) -> Result<Vec<Train>, RepositoryError> {
        if !Path::new(&self.path).exists() {
            self.init_file().map_err(RepositoryError::Io)?;
        }
        let contents = fs::read_to_string(&self.path).map_err(RepositoryError::Io)?;
        serde_json::from_str(&contents)
            .map_err(|e| RepositoryError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))
    }

    async fn save_train(&self, train: &[Train]) -> Result<(), RepositoryError> {
        let json = serde_json::to_string_pretty(&train)
            .map_err(|e| RepositoryError::Io(io::Error::new(io::ErrorKind::InvalidData, e)))?;

        fs::write(&self.path, json).map_err(RepositoryError::Io)?;
        Ok(())
    }

    async fn ready(&self) -> Result<(), RepositoryError> {
        match self.path.rsplit('.').next() {
            Some("json") => Ok(()),
            _ => Err(RepositoryError::Io(io::Error::new(
                io::ErrorKind::InvalidInput,
                "storage layer not ready",
            ))),
        }
    }
}

impl TrainCompanyFileRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    fn initialise_train(&self) -> IoResult<Train> {
        let mut wagons: Vec<Wagon> = vec![];
        let mut seats: Vec<Seat> = vec![];

        for wagon_idx in 1..=WAGON_NUMBER {
            seats.clear();
            for seat_idx in 1..=NUMBER_OF_SEATS_PER_WAGON {
                seats.push(Seat {
                    number: seat_idx,
                    status: SeatStatus::Available,
                })
            }
            wagons.push(Wagon {
                wagon_number: wagon_idx,
                seats: seats.clone(),
            });
        }
        Ok(Train { wagons })
    }
    pub fn init_file(&self) -> IoResult<()> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.path)?;

        let train = self.initialise_train()?;
        let json = serde_json::to_string_pretty(&[train])?;
        fs::write(&self.path, json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn valid_train_creation() {
        let repo = TrainCompanyFileRepository::new("train-test.json".to_string());
        repo.init_file().unwrap();
        assert!(Path::new("train-test.json").exists())
    }
}
