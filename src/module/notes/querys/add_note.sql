INSERT INTO notes(title, details, done)
VALUES ($1, $2, $3)
RETURNING *;
