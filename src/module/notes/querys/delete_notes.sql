DELETE FROM notes
WHERE notes.id IN ($ids) 
RETURNING *;