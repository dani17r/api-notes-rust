-- WITH note AS (
  INSERT INTO notes(title, details, done, rank)
  VALUES ($1, $2, $3, $4) 
  RETURNING *
-- )
-- SELECT note.id, note.title, note.details, note.done, note.rank, COALESCE(
--     json_agg(json_build_object('id', tags.id, 'name', tags.name)) 
--     FILTER ( WHERE tags.id IS NOT NULL), '[]' :: json
--   ) AS tags
-- FROM note
-- LEFT JOIN note_tags ON note.id = note_tags.note_id
-- LEFT JOIN tags ON note_tags.tag_id = tags.id
-- GROUP BY note.id, note.title, note.details, note.done, note.rank;