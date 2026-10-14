use axum::http::StatusCode;
use serial_test::serial;
use train_booking_api::models::{Booking, ErrorResponse, SeatSelection};
mod common;
use common::{create_booking, get_request, post_request, test_app};

#[tokio::test]
#[serial]
async fn book_happy_path_200() {
    let app = test_app().await;
    let requested_seats = vec![
        SeatSelection {
            coach_number: 1,
            seat_number: 1,
        },
        SeatSelection {
            coach_number: 2,
            seat_number: 2,
        },
    ];
    let booking_request = create_booking("1", requested_seats);
    let (response_body, status_code) = post_request(booking_request, &app).await;
    let body_str = String::from_utf8_lossy(&response_body);
    assert_eq!(
        status_code,
        StatusCode::OK,
        "unexpected response: {body_str}"
    );
    let booking: Booking = serde_json::from_slice(&response_body).unwrap();

    assert_eq!(booking.booked_seats.len(), 2);
    assert!(!booking.id.to_string().is_empty());
}

#[tokio::test]
#[serial]
async fn seat_already_booked() {
    let app = test_app().await;
    let requested_seats = vec![
        SeatSelection {
            coach_number: 1,
            seat_number: 10,
        },
        SeatSelection {
            coach_number: 2,
            seat_number: 11,
        },
    ];
    let booking_request = create_booking("2", requested_seats.clone());

    let (first_body, first_status) = post_request(booking_request, &app).await;
    assert_eq!(
        first_status,
        StatusCode::OK,
        "first booking failed: {}",
        String::from_utf8_lossy(&first_body)
    );

    let booking_request1 = create_booking("2", requested_seats);
    let (response_body1, status_code) = post_request(booking_request1, &app).await;
    let booking_err: ErrorResponse = serde_json::from_slice(&response_body1).unwrap();

    assert_eq!(booking_err.error, "seat already taken".to_string());
    assert_eq!(status_code, StatusCode::CONFLICT);
}

#[tokio::test]
#[serial]
async fn invalid_booking_no_seats() {
    let app = test_app().await;
    let requested_seats: Vec<SeatSelection> = vec![];

    let booking_request = create_booking("3", requested_seats);
    let (response_body, status_code) = post_request(booking_request, &app).await;
    let booking_err: ErrorResponse = serde_json::from_slice(&response_body).unwrap();

    assert_eq!(booking_err.error, "no seat(s) requested".to_string());
    assert_eq!(status_code, StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[serial]
async fn fetch_bookings_returns_all_bookings_for_email() {
    let app = test_app().await;
    let requested_seats1 = vec![
        SeatSelection {
            coach_number: 3,
            seat_number: 30,
        },
        SeatSelection {
            coach_number: 3,
            seat_number: 31,
        },
    ];

    let requested_seats2 = vec![
        SeatSelection {
            coach_number: 4,
            seat_number: 32,
        },
        SeatSelection {
            coach_number: 4,
            seat_number: 33,
        },
    ];

    let booking_request1 = create_booking("4", requested_seats1.clone());

    let (response_body1, status_code1) = post_request(booking_request1, &app).await;
    assert_eq!(
        status_code1,
        StatusCode::OK,
        "first booking failed: {}",
        String::from_utf8_lossy(&response_body1)
    );
    let booking1: Booking = serde_json::from_slice(&response_body1).unwrap();

    let booking_request2 = create_booking("4", requested_seats2.clone());
    let (response_body2, status_code2) = post_request(booking_request2, &app).await;
    assert_eq!(
        status_code2,
        StatusCode::OK,
        "second booking failed: {}",
        String::from_utf8_lossy(&response_body2)
    );
    let booking2: Booking = serde_json::from_slice(&response_body2).unwrap();

    let (response_all_bookings, status_code3) = get_request("test4@user.com", &app).await;
    assert_eq!(status_code3, StatusCode::OK);
    let all_bookings: Vec<Booking> = serde_json::from_slice(&response_all_bookings).unwrap();
    assert_eq!(all_bookings.len(), 2);

    let ids = all_bookings.iter().map(|b| b.id).collect::<Vec<_>>();

    assert!(ids.contains(&booking1.id));
    assert!(ids.contains(&booking2.id));

    let b1 = all_bookings.iter().find(|b| b.id == booking1.id).unwrap();
    let b2 = all_bookings.iter().find(|b| b.id == booking2.id).unwrap();

    assert_eq!(b1.booked_seats, requested_seats1);
    assert_eq!(b2.booked_seats, requested_seats2);
}
