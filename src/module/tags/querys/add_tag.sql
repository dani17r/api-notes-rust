INSERT INTO tags(name, description, color)
VALUES ($1, $2, $3)
RETURNING *;
