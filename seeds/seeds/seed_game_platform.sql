INSERT INTO game_platform (game_id, platform_id) VALUES
	(
		(SELECT game_id FROM games WHERE name = 'Starfield'), 
		(SELECT platform_id FROM platforms WHERE name = 'Steam')
	),
	(
		(SELECT game_id FROM games WHERE name = 'Baldur''s Gate 3'), 
		(SELECT platform_id FROM platforms WHERE name = 'Steam')
	),
	(
		(SELECT game_id FROM games WHERE name = 'Stardew Valley'), 
		(SELECT platform_id FROM platforms WHERE name = 'Switch')
	);
