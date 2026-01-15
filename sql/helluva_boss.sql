CREATE TABLE helluva_boss (
	id SERIAL PRIMARY KEY,
	name VARCHAR(100) NOT NULL,
	data JSONB NOT NULL,
	specie VARCHAR(50),
	class VARCHAR(50)
);
-- inline
-- CREATE TABLE helluva_boss ( id SERIAL PRIMARY KEY, name VARCHAR(50) NOT NULL, data JSONB NOT NULL, specie VARCHAR(50), class VARCHAR(50) );

-- Insert Example
-- INSERT INTO helluva_boss ( name, data, specie, class ) VALUES ( 'alastor', '{  "goal": "unknow",  "role": "radio_demon",  "family": [   "unknow"  ],  "powers": [   "reality_distortion",   "shadow_control"  ],  "status": "alive",  "personality": [   "sadistic",   "charismatic"  ],  "first_appearance": "hazbin_hotel_pilot" }', 'demon', 'overlord' );


