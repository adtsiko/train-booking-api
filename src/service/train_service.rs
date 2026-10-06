use uuid::Uuid;
use crate::repository::TrainCompanyFileRepository;
use crate::models::{Train, BookingRequest, Booking, User, Seat, SeatSelection, BookingError, SeatStatus};

const NUMBER_OF_SEATS_PER_WAGON: u32  = 50;
const WAGON_NUMBER: u32 = 4;

#[derive(Clone)]
pub struct TrainService {
    repo: TrainCompanyFileRepository
}

impl TrainService {
    pub fn new(repo: TrainCompanyFileRepository) -> Self {
        Self{repo}
    }

    pub fn book_trip(&self, booking_request: BookingRequest) -> Result<Booking, BookingError>  {
        TrainService::validate_booking(&booking_request)?;
        let mut trains  = self.repo.load_train()?;
        let train = &mut trains[0];
        TrainService::check_available_seats(train, &booking_request.requested_seats)?;
        TrainService::book_seats(train, &booking_request.requested_seats)?;
        let booking = TrainService::create_booking(booking_request)?;
        self.repo.save_train(&trains)?;
        Ok(booking)
    }

    fn create_booking(booking_request: BookingRequest) -> Result<Booking, BookingError> {
        let user = User{
            username: booking_request.username.clone(),
            email: booking_request.email.clone(),
        };
       
        let booking = Booking{
            user: user,
            booking_id: Uuid::new_v4(),
            booked_seats: booking_request.requested_seats.clone()
        };
        Ok(booking)

    }

    fn book_seats(train: &mut Train, seats_required: &[SeatSelection]) -> Result<(), BookingError> {
        for seat_to_book in seats_required {
            let seat = TrainService::find_seat_mut(train, seat_to_book);
            if let Some(seat) = seat{
                seat.status = SeatStatus::Booked
            }
            else{
                return Err(BookingError::InvalidSeatNumber)
            }
        }
        Ok(())
    }

    fn find_seat_mut<'a>(train: &'a mut Train, sel: &SeatSelection) -> Option<&'a mut Seat> {
        let wagon = train.wagons.iter_mut().find(|w| w.wagon_number == sel.wagon_number);
        wagon
                .and_then(|w| w.seats.iter_mut().find(|s| s.number == sel.seat_number))
    }

    fn check_available_seats(train: &mut Train, seats_required: &[SeatSelection] ) -> Result<(), BookingError> {
        
        for seat_to_book in seats_required{
            let seat = TrainService::find_seat_mut(train, seat_to_book);

            if let Some(seat) = seat{
                if seat.status == SeatStatus::Booked{
                    return Err(BookingError::SeatTaken);
                }
            }
            else{
                return Err(BookingError::InvalidSeatNumber);
            }
        }

        Ok(())
    }

    fn validate_booking(booking_request: &BookingRequest) -> Result<(), BookingError> {
        for seat in &booking_request.requested_seats {
            if  seat.seat_number > NUMBER_OF_SEATS_PER_WAGON {
                return Err(BookingError::InvalidSeatNumber);
            }
            if seat.wagon_number > WAGON_NUMBER || seat.wagon_number < 1 {
                return Err(BookingError::InvalidWagonNumber)
            }
        }

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_seat_returns_error() {
        let request = BookingRequest::new("Anesu",
             "a@live.com", 
             vec![SeatSelection{wagon_number: 1,seat_number: 51}]);
        let result = TrainService::validate_booking(&request);
        assert!(result.is_err());
    }

}