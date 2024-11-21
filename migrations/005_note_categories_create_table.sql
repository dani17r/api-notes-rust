CREATE TABLE note_categories (
    id BIGSERIAL PRIMARY KEY,
    note_id BIGINT NOT NULL,
    category_id BIGINT NOT NULL,
    FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);