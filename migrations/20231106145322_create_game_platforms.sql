-- Add up migration script here
CREATE TABLE game_platforms (
    game_id INT REFERENCES games (game_id) ON DELETE CASCADE,
    platform_id INT REFERENCES platforms (platform_id) ON DELETE CASCADE,
    PRIMARY KEY (game_id, platform_id)
);

