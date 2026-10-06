# train-booking-api

REST API for booking train seats. A train has 4 wagons with 50 seats each. Seat availability is persisted to a JSON file.

## Architecture

```
handlers → service → repository → models
```

## Run

```bash
cargo run
```

Server listens on `http://127.0.0.1:3001`.

## Book a seat

```bash
curl -X POST http://127.0.0.1:3001/book \
  -H "Content-Type: application/json" \
  -d '{
    "username": "Noa",
    "email": "noa@example.com",
    "requested_seats": [{"wagon_number": 1, "seat_number": 2}]
  }'
```

## Error responses

| Status | Meaning |
|--------|---------|
| 409 | Seat already booked |
| 400 | Invalid seat or wagon number |
| 500 | Internal / I/O error |

## Test

```bash
cargo test
```
