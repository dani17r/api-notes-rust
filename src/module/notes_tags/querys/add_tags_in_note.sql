INSERT INTO note_tags(note_id, tag_id)
SELECT $1 AS note_id, unnest($2::bigint[]) AS tag_id
RETURNING *;
