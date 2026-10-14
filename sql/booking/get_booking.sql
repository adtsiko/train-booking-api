SELECT 
     b.id,
     b.journey_id,
     b.created_at,
     COALESCE(
          json_agg(
               json_build_object(
                    'coach_number', c.coach_number,
                    'seat_number', s.seat_number
          )
          ORDER BY c.coach_number, s.seat_number
     ) FILTER (WHERE si.id IS NOT NULL),
     '[]'::json
     ) AS "booked_seats!"
FROM bookings b
INNER JOIN passengers ON b.id = passengers.booking_id
LEFT JOIN booking_seats bs ON b.id = bs.booking_id
LEFT JOIN seat_inventory si ON bs.seat_inventory_id = si.id
LEFT JOIN seats s ON si.seat_id = s.id
LEFT JOIN coaches c ON s.coach_id = c.id
WHERE passengers.email = $1
GROUP BY b.id, b.journey_id, b.created_at