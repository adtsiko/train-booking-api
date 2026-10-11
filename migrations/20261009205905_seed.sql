-- Add migration script here
-- =============================================================================
-- Seed: 1 service, 1 journey, 4 coaches × 50 seats, 200 inventory rows (available)
-- =============================================================================

INSERT INTO train_services (id, code, name)
VALUES (
    '11111111-1111-1111-1111-111111111111',
    'PADBHM1',
    'Paddington to Birmingham'
);
INSERT INTO journeys (id, service_id, departure_at, origin, destination)
VALUES (
    '22222222-2222-2222-2222-222222222222',
    '11111111-1111-1111-1111-111111111111',
    '2026-03-15 08:00:00+00',
    'London Paddington',
    'Birmingham'
);

-- 4 coaches (wagon 1–4)
INSERT INTO coaches (journey_id, coach_number)
SELECT
    '22222222-2222-2222-2222-222222222222',
    coach_number
FROM generate_series(1, 4) AS coach_number;

-- 50 seats per coach
INSERT INTO seats (coach_id, seat_number)
SELECT
    c.id,
    seat_number
FROM coaches c
CROSS JOIN generate_series(1, 50) AS seat_number
WHERE c.journey_id = '22222222-2222-2222-2222-222222222222';

-- One inventory row per seat on this journey (all available)
INSERT INTO seat_inventory (journey_id, seat_id, status)
SELECT
    '22222222-2222-2222-2222-222222222222',
    s.id,
    'available'
FROM seats s
INNER JOIN coaches c ON s.coach_id = c.id
WHERE c.journey_id = '22222222-2222-2222-2222-222222222222';