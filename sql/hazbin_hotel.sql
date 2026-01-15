CREATE TABLE hazbin_hotel (
	id SERIAL PRIMARY KEY,
	name VARCHAR(100) NOT NULL,
	data JSONB NOT NULL,
	specie VARCHAR(50),
	class VARCHAR(50)
);
-- inline
-- CREATE TABLE hazbin_hotel ( id SERIAL PRIMARY KEY, name VARCHAR(100) NOT NULL, data JSONB NOT NULL, specie VARCHAR(50), class VARCHAR(50) );

-- Insert Example
-- INSERT INTO hazbin_hotel ( name, data, specie, class ) VALUES ( 'alastor', '{  "goal": "unknow",  "role": "radio_demon",  "family": [   "unknow"  ],  "powers": [   "reality_distortion",   "shadow_control"  ],  "status": "alive",  "personality": [   "sadistic",   "charismatic"  ],  "first_appearance": "hazbin_hotel_pilot" }', 'demon', 'overlord' );

