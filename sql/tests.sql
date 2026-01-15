CREATE TABLE tests (
	id SERIAL PRIMARY KEY,
	description VARCHAR(50) NOT NULL,
	status VARCHAR(50) NOT NULL
);
-- inline
-- CREATE TABLE tests ( id SERIAL PRIMARY KEY, description VARCHAR(50) NOT NULL, status VARCHAR(50) NOT NULL );

-- Insert Example
-- INSERT INTO tests (description, status) VALUES ('Teste de Conexão', 'ok');
