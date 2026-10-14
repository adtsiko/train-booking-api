use crate::models::{
    Booking, BookingError, BookingRecord, BookingRequest, RepositoryError, SeatInventoryRecord,
    SeatStatus,
};
use crate::repository::{DbBooking, DbInventory};

const NUMBER_OF_SEATS_PER_COACH: u32 = 50;
const COACH_NUMBER: u32 = 4;

#[derive(Clone)]
pub struct TrainService<I: DbInventory, B: DbBooking> {
    inventory_repo: I,
    booking_repo: B,
}

impl<I: DbInventory, B: DbBooking> TrainService<I, B> {
    pub fn new(inventory_repo: I, booking_repo: B) -> Self {
        Self {
            inventory_repo,
            booking_repo,
        }
    }

    pub async fn book_trip(&self, request: BookingRequest) -> Result<Booking, BookingError> {
        Self::validate_booking(&request)?;
        let mut tx = self
            .inventory_repo
            .pool()
            .begin()
            .await
            .map_err(RepositoryError::Database)?;
        let mut inventory_ids: Vec<SeatInventoryRecord> = vec![];

        for seat in &request.requested_seats {
            let inventory = self
                .inventory_repo
                .find_seat_inventory_for_update(
                    &mut tx,
                    request.journey_id,
                    seat.coach_number,
                    seat.seat_number,
                )
                .await?;

            if inventory.status != SeatStatus::Available {
                return Err(BookingError::SeatTaken);
            }

            let booked = self
                .inventory_repo
                .mark_seat_booked(&mut tx, inventory.id)
                .await?;

            inventory_ids.push(booked);
        }

        let record: BookingRecord = self
            .booking_repo
            .create_booking(
                &mut tx,
                &request.journey_id,
                &request.username,
                &request.email,
                &inventory_ids,
            )
            .await?;

        tx.commit().await.map_err(RepositoryError::Database)?;
        let booking = Self::map_booking(request, record)?;
        tracing::info!("Booking {} created successfully", &booking.id);
        Ok(booking)
    }

    fn map_booking(
        request: BookingRequest,
        record: BookingRecord,
    ) -> Result<Booking, BookingError> {
        Ok(Booking {
            id: record.id,
            journey_id: request.journey_id,
            booked_seats: request.requested_seats.clone(),
            created_at: record.created_at,
        })
    }

    fn validate_booking(booking_request: &BookingRequest) -> Result<(), BookingError> {
        if booking_request.requested_seats.is_empty() {
            return Err(BookingError::NoSeatsRequested);
        }

        for seat in &booking_request.requested_seats {
            if seat.seat_number > NUMBER_OF_SEATS_PER_COACH {
                return Err(BookingError::InvalidSeatNumber);
            }
            if seat.coach_number > COACH_NUMBER || seat.coach_number < 1 {
                return Err(BookingError::InvalidCoachNumber);
            }
        }

        Ok(())
    }

    pub async fn get_user_bookings(&self, email: &str) -> Result<Vec<Booking>, BookingError> {
        let bookings = self.booking_repo.get_bookings_by_email(email).await?;

        Ok(bookings)
    }

    pub async fn check_readiness(&self) -> Result<(), RepositoryError> {
        let (inventory, booking) =
            tokio::join! {self.inventory_repo.ready(), self.booking_repo.ready()};

        inventory?;
        booking?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::SeatSelection;
    use crate::repository::postgres::{BookingRepository, InventoryRepository};
    use uuid::Uuid;

    #[test]
    fn invalid_seat_returns_error() {
        let request = BookingRequest::new(
            Uuid::new_v4(),
            "Anesu",
            "a@live.com",
            vec![SeatSelection {
                coach_number: 1,
                seat_number: 51,
            }],
        );

        let result =
            TrainService::<InventoryRepository, BookingRepository>::validate_booking(&request);
        assert!(result.is_err());
    }

    #[test]
    fn no_seats_booked() {
        let request = BookingRequest::new(Uuid::new_v4(), "Anesu", "a@live.com", vec![]);

        let result =
            TrainService::<InventoryRepository, BookingRepository>::validate_booking(&request);
        assert!(result.is_err());
    }
}
