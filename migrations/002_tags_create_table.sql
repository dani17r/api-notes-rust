DROP SCHEMA IF EXISTS tags CASCADE;
CREATE SCHEMA tags;

CREATE TABLE tags (
	id  BIGSERIAL PRIMARY KEY,
	name         VARCHAR(120) NOT NULL,
	description  VARCHAR(250) NOT NULL,
	color        VARCHAR(120)
);