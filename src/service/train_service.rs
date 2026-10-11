use crate::models::{
    Booking, BookingError, BookingRecord, BookingRequest, RepositoryError, Seat,
    SeatInventoryRecord, SeatSelection, SeatStatus, Train, User,
};
use crate::repository::{DbBooking, DbInventory};
use chrono::Utc;
use uuid::Uuid;

const NUMBER_OF_SEATS_PER_WAGON: u32 = 50;
const WAGON_NUMBER: u32 = 4;

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

        let mut inventory_ids: Vec<SeatInventoryRecord> = vec![];

        for seat in &request.requested_seats {
            let inventory = self
                .inventory_repo
                .find_seat_inventory_for_update(
                    request.journey_id,
                    seat.wagon_number,
                    seat.seat_number,
                )
                .await?;

            todo!(); //check if seat is available
        }

        let record: BookingRecord = self
            .booking_repo
            .create_booking(
                &request.journey_id,
                &request.username,
                &request.email,
                &inventory_ids,
            )
            .await?;

        let booking = Self::create_booking(request, record)?;
        tracing::info!("Booking {} created successfully", &booking.booking_id);
        Ok(booking)
    }

    fn create_booking(
        request: BookingRequest,
        record: BookingRecord,
    ) -> Result<Booking, BookingError> {
        let user = User {
            username: request.username.clone(),
            email: request.email.clone(),
        };

        let booking = Booking {
            user: user,
            booking_id: record.id,
            booked_seats: request.requested_seats.clone(),
            created_at: Utc::now(),
        };
        Ok(booking)
    }

    fn book_seats(train: &mut Train, seats_required: &[SeatSelection]) -> Result<(), BookingError> {
        for seat_to_book in seats_required {
            let seat = Self::find_seat_mut(train, seat_to_book);
            if let Some(seat) = seat {
                seat.status = SeatStatus::Booked
            } else {
                return Err(BookingError::InvalidSeatNumber);
            }
        }
        Ok(())
    }

    fn find_seat_mut<'a>(train: &'a mut Train, sel: &SeatSelection) -> Option<&'a mut Seat> {
        let wagon = train
            .wagons
            .iter_mut()
            .find(|w| w.wagon_number == sel.wagon_number);
        wagon.and_then(|w| w.seats.iter_mut().find(|s| s.number == sel.seat_number))
    }

    fn check_available_seats(
        train: &mut Train,
        seats_required: &[SeatSelection],
    ) -> Result<(), BookingError> {
        for seat_to_book in seats_required {
            let seat = Self::find_seat_mut(train, seat_to_book);

            if let Some(seat) = seat {
                if seat.status == SeatStatus::Booked {
                    tracing::error!(
                        "seat {} in wagon {} unavailable",
                        seat_to_book.seat_number,
                        seat_to_book.wagon_number
                    );
                    return Err(BookingError::SeatTaken);
                }
            } else {
                return Err(BookingError::InvalidSeatNumber);
            }
        }

        Ok(())
    }

    fn validate_booking(booking_request: &BookingRequest) -> Result<(), BookingError> {
        if booking_request.requested_seats.is_empty() {
            return Err(BookingError::NoSeatsRequested);
        }

        for seat in &booking_request.requested_seats {
            if seat.seat_number > NUMBER_OF_SEATS_PER_WAGON {
                return Err(BookingError::InvalidSeatNumber);
            }
            if seat.wagon_number > WAGON_NUMBER || seat.wagon_number < 1 {
                return Err(BookingError::InvalidWagonNumber);
            }
        }

        Ok(())
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
    use crate::repository::postgres::{BookingRepository, InventoryRepository};

    #[test]
    fn invalid_seat_returns_error() {
        let request = BookingRequest::new(
            Uuid::new_v4(),
            "Anesu",
            "a@live.com",
            vec![SeatSelection {
                wagon_number: 1,
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
