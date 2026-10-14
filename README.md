# train-booking-api

REST API for booking train seats. A train has 4 wagons with 50 seats each. The app uses PostgreSQL for persistence and readiness checks; integration tests use a file-based repository.

## Architecture

```
handlers → service → repository → models
```

## Configuration

Copy `.env.example` to `.env` and adjust as needed. All settings have defaults for local development.

| Variable       | Default                                                    | Description                              |
|----------------|------------------------------------------------------------|------------------------------------------|
| `DATABASE_URL` | `postgres://train:train@localhost:5432/train_booking`        | PostgreSQL connection string             |
| `APP_PORT`     | `3001`                                                     | TCP port the HTTP server binds to        |
| `DATA_PATH`    | `train.json`                                               | JSON file path (used by integration tests) |

Example:

```bash
cp .env.example .env
cargo run
```

## Local development

### 1. Start PostgreSQL

Requires [Docker Desktop](https://www.docker.com/products/docker-desktop/). Start the daemon, then:

```bash
docker compose up -d
```

Check the database is healthy:

```bash
docker compose ps
```

### 2. Run migrations

Migrations live in `migrations/` and are applied with [sqlx-cli](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli).

Install sqlx-cli once:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

Set `DATABASE_URL` (or copy `.env.example` to `.env` and export it), then apply all pending migrations:

```bash
export DATABASE_URL=postgres://train:train@localhost:5432/train_booking
sqlx migrate run
```

Check migration status:

```bash
sqlx migrate info
```

Add a new migration:

```bash
sqlx migrate add <name>
```

This creates a timestamped file under `migrations/`. Edit the SQL, then run `sqlx migrate run` again.

### 3. Run the API

```bash
cargo run
```

By default the server listens on `http://127.0.0.1:3001`.

`GET /ready` returns `200` when PostgreSQL is reachable and `503` when it is not.

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
    "requested_seats": [{"coach_number": 1, "seat_number": 2}]
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
