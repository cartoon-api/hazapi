-- All Characters
CREATE TABLE characters (
	id SERIAL PRIMARY KEY,
	name VARCHAR(100) NOT NULL,
	data JSONB NOT NULL,
	specie VARCHAR(50),
	class VARCHAR(50)
);

-- Hazbin Hotel Characters 
CREATE TABLE hazbin_hotel (
	id SERIAL PRIMARY KEY,
	name VARCHAR(100) NOT NULL,
	data JSONB NOT NULL,
	specie VARCHAR(50),
	class VARCHAR(50)
);

-- Helluva Boss Characters
CREATE TABLE helluva_boss (
	id SERIAL PRIMARY KEY,
	name VARCHAR(100) NOT NULL,
	data JSONB NOT NULL,
	specie VARCHAR(50),
	class VARCHAR(50)
);

-- All Weapons - Hazbin Hotel + Helluva Boss
CREATE TABLE weapons (
	id SERIAL PRIMARY KEY,
	name VARCHAR(100) NOT NULL,
	data JSONB NOT NULL,
	universe VARCHAR(50) NOT NULL,
	owner VARCHAR(50)
);

-- Table of Tests
CREATE TABLE tests (
	id SERIAL PRIMARY KEY,
	description VARCHAR(200) NOT NULL,
	status VARCHAR(50) NOT NULL
);
