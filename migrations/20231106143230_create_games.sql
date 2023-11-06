-- Add migration script here
CREATE TABLE games (
    game_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);
