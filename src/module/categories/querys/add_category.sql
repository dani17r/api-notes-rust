INSERT INTO categories(title, description)
VALUES ($1, $2)
RETURNING *;
