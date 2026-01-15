CREATE TABLE weapons (
	id SERIAL PRIMARY KEY,
	name VARCHAR(100) NOT NULL,
	data JSONB NOT NULL,
	universe VARCHAR(50) NOT NULL,
	owner VARCHAR(100)
);
-- inline
-- CREATE TABLE weapons (id SERIAL PRIMARY KEY, name VARCHAR(100) NOT NULL, data JSONB NOT NULL, universe VARCHAR(20) NOT NULL, owner VARCHAR(100));

-- Insert Example
-- INSERT INTO weapons ( name, data, universe, owner ) VALUES ( 'cherri_bomb_explosives', '{  "type": "explosive",  "effect": "area_damage",  "status": "active",  "material": "hell_made" }', 'hazbin_hotel', 'cherri_bomb' );
