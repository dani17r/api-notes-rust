SELECT categories.id, $table_fields
FROM categories $_SEARCH_
ORDER by $sort_field $sort_order
LIMIT $limit_pag
OFFSET $offset_pag;