-- Enable pgcrypto extension for UUID generation
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Product definition (optional but realistic)
CREATE TABLE train_services (
    id   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code TEXT NOT NULL UNIQUE,          -- e.g. 'IC101'
    name TEXT NOT NULL
);

-- A specific run customers can book
CREATE TABLE journeys (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_id   UUID NOT NULL REFERENCES train_services(id),
    departure_at TIMESTAMPTZ NOT NULL,
    origin       TEXT NOT NULL,
    destination  TEXT NOT NULL,
    UNIQUE (service_id, departure_at)
);

-- Coach layout for a journey (or template copied per journey)
CREATE TABLE coaches (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    journey_id   UUID NOT NULL REFERENCES journeys(id),
    coach_number INT NOT NULL,
    UNIQUE (journey_id, coach_number)
);

CREATE TYPE seat_status AS ENUM ('available', 'held', 'booked');

-- Sellable inventory unit
CREATE TABLE seats (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    coach_id    UUID NOT NULL REFERENCES coaches(id),
    seat_number INT NOT NULL,
    UNIQUE (coach_id, seat_number)
);

-- Better: status lives on inventory row, not only on seat
CREATE TABLE seat_inventory (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    journey_id UUID NOT NULL REFERENCES journeys(id),
    seat_id    UUID NOT NULL REFERENCES seats(id),
    status     seat_status NOT NULL DEFAULT 'available',
    UNIQUE (journey_id, seat_id)
);

CREATE TABLE bookings (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    journey_id UUID NOT NULL REFERENCES journeys(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE passengers (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    booking_id  UUID NOT NULL REFERENCES bookings(id),
    full_name   TEXT NOT NULL,
    email       TEXT NOT NULL
);

CREATE TABLE booking_seats (
    booking_id       UUID NOT NULL REFERENCES bookings(id),
    seat_inventory_id UUID NOT NULL REFERENCES seat_inventory(id),
    PRIMARY KEY (booking_id, seat_inventory_id),
    UNIQUE (seat_inventory_id)   -- one active booking per seat on that journey
);