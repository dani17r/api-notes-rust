DELETE FROM categories 
WHERE id IN ($ids) 
RETURNING *;