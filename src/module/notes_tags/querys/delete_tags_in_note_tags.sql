DELETE FROM note_tags
WHERE tag_id IN ($ids)
RETURNING *;