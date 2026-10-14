UPDATE seat_inventory
SET status = 'booked'
WHERE id = $1 AND status = 'available'
RETURNING id, journey_id, seat_id, status::text AS "status!"