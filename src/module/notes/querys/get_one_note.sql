SELECT notes.id $table_fields $relationship
FROM notes
LEFT JOIN note_tags ON notes.id = note_tags.note_id
LEFT JOIN tags ON note_tags.tag_id = tags.id
WHERE notes.id = $id_note
GROUP BY notes.id $table_fields;