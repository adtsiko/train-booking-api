INSERT INTO bookings ( journey_id)
VALUES ($1)
RETURNING id, journey_id, created_at