use axum::http::StatusCode;
use train_booking_api::models::{Booking, ErrorResponse, SeatSelection};
mod common;
use common::{create_booking, post_request, test_app};

#[tokio::test]
async fn book_happy_path_200() {
    let path = "test-book-happy.json";
    let app = test_app(path);
    let requested_seats = vec![
        SeatSelection {
            wagon_number: 1,
            seat_number: 1,
        },
        SeatSelection {
            wagon_number: 2,
            seat_number: 2,
        },
    ];
    let booking_request = create_booking("1", requested_seats);
    let (response_body, status_code) = post_request(booking_request, &app).await;
    let booking: Booking = serde_json::from_slice(&response_body).unwrap();

    assert_eq!(booking.user.username, "test1");
    assert_eq!(booking.booked_seats.len(), 2);
    assert!(!booking.booking_id.to_string().is_empty());
    assert_eq!(status_code, StatusCode::OK);
}

#[tokio::test]
async fn seat_already_booked() {
    let path = "test-book-booked_already.json";
    let app = test_app(path);
    let requested_seats = vec![
        SeatSelection {
            wagon_number: 1,
            seat_number: 1,
        },
        SeatSelection {
            wagon_number: 2,
            seat_number: 2,
        },
    ];
    let booking_request = create_booking("1", requested_seats.clone());

    let _ = post_request(booking_request, &app).await;

    let booking_request1 = create_booking("1", requested_seats);
    let (response_body1, status_code) = post_request(booking_request1, &app).await;
    let booking_err: ErrorResponse = serde_json::from_slice(&response_body1).unwrap();

    assert_eq!(booking_err.error, "seat already taken".to_string());
    assert_eq!(status_code, StatusCode::CONFLICT);
}
