WITH tag AS (SELECT unnest($2:: bigint[]) AS tag_id)
UPDATE note_tags
SET tag_id = tag.tag_id
FROM tag WHERE note_tags.note_id = $1
AND note_tags.tag_id IN (SELECT tag_id FROM tag);
