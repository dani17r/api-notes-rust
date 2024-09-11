UPDATE notes
SET $set_clause
WHERE id = $1
RETURNING *;
