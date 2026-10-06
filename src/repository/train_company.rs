use std::path::Path;
use std::fs;
use std::io::{self, Result, Write};

use crate::models::{Train, Wagon, Seat, SeatStatus};

#[derive(Clone)]
pub struct TrainCompanyFileRepository {
    path: String
}

const WAGON_NUMBER: u32 = 4;
const NUMBER_OF_SEATS_PER_WAGON: u32  = 50;

impl TrainCompanyFileRepository {
    pub fn new(path: String) -> Self {
        Self{path}
    } 

    pub fn load_train(&self) -> Result<Vec<Train>> {
        if !Path::new(&self.path).exists(){
            self.init_file()?;
        }

        let contents = fs::read_to_string(&self.path)?;

        serde_json::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn save_train(&self, train: &[Train]) -> Result<()> {
        let json = serde_json::to_string_pretty(&train)?;

        fs::write(&self.path, json)?;
        Ok(())
    }

    fn initialise_train(&self) -> Result<Train> {
        let mut wagons: Vec<Wagon> = vec![];
        let mut seats: Vec<Seat> = vec![];

        for wagon_idx in 1..=WAGON_NUMBER {
            seats.clear();
            for seat_idx in 1..=NUMBER_OF_SEATS_PER_WAGON{
                seats.push(Seat{number : seat_idx, status: SeatStatus::Available})
            }
            wagons.push(Wagon{wagon_number: wagon_idx, seats: seats.clone()});
        }
        Ok(Train{wagons})

    }
    pub fn init_file(&self) -> Result<()> {
        let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&self.path)?;

        let train = self.initialise_train()?;
        let json  = serde_json::to_string_pretty(&[train])?;
        file.write_all(b"[]")?;
        fs::write(&self.path, json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn valid_train_creation(){
        let repo = TrainCompanyFileRepository::new("train-test.json".to_string());
        repo.init_file().unwrap();
        assert!(Path::new("train-test.json").exists())

    }
}