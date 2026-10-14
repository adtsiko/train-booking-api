SELECT
    si.id,
    si.journey_id,
    si.seat_id,
    si.status::text AS "status!"
FROM seat_inventory si
INNER JOIN seats s ON s.id = si.seat_id
INNER JOIN coaches c ON c.id = s.coach_id
WHERE si.journey_id = $1
  AND c.coach_number = $2
  AND s.seat_number = $3
FOR UPDATE OF si