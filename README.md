# train-booking-api

REST API for booking train seats. A train has 4 wagons with 50 seats each. Seat availability is persisted to a JSON file.

## Architecture

```
handlers → service → repository → models
```

## Configuration

All settings are optional. Set them as environment variables when starting the server.

| Variable   | Default      | Description                          |
|------------|--------------|--------------------------------------|
| `APP_PORT` | `3001`       | TCP port the HTTP server binds to    |
| `DATA_PATH`| `train.json` | Path to the JSON file for seat data  |

Example:

```bash
APP_PORT=3001 DATA_PATH=./data/train.json cargo run
```

No secrets are required to run the API locally.

## Run

```bash
cargo run
```

By default the server listens on `http://127.0.0.1:3001`.

## API documentation

OpenAPI 3 spec is generated at runtime with [utoipa](https://github.com/juhaku/utoipa) from `src/openapi.rs`.

| Resource | URL |
|----------|-----|
| Swagger UI | http://127.0.0.1:3001/docs |
| OpenAPI JSON | http://127.0.0.1:3001/api-docs/openapi.json |

`POST /book` is documented with request/response schemas and error responses (`400`, `409`, `500`).

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

## Development

### Formatting

```bash
cargo fmt --all
```

CI runs `cargo fmt --all -- --check` on pull requests and pushes to `main`.

### Pre-commit hook

Enable the local hook so formatting is checked before each commit:

```bash
git config core.hooksPath .githooks
chmod +x .githooks/pre-commit
```

### Continuous integration

GitHub Actions (`.github/workflows/ci.yml`) runs formatting checks and tests on pull requests and pushes to `main`.

## Test

```bash
cargo test
```
