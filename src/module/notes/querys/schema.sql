DROP SCHEMA IF EXISTS notes CASCADE;
CREATE SCHEMA notes;

CREATE TABLE notes (
	id  BIGSERIAL PRIMARY KEY,
	title       VARCHAR(120) NOT NULL,
	details     VARCHAR(250) NOT NULL,
	done        BOOLEAN NOT NULL
);