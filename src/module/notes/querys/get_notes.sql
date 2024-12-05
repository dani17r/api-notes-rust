SELECT notes.id $table_fields $relationship
FROM notes
LEFT JOIN note_tags ON notes.id = note_tags.note_id
LEFT JOIN tags ON note_tags.tag_id = tags.id
$_CONDITIONALS_ $_SEARCH_
GROUP BY notes.id $table_fields
ORDER BY notes.id $sort_order
LIMIT $limit_pag OFFSET $offset_pag;