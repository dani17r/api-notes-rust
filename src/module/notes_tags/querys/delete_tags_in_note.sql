DELETE FROM note_tags 
WHERE note_id = $1 AND tag_id IN (SELECT unnest($2::bigint[])) 
RETURNING *;