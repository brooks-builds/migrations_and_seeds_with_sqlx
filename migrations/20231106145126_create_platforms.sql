-- Add up migration script here
CREATE TABLE platforms (
    platform_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

