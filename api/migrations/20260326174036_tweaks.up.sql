-- Add up migration script here
DELETE FROM courses WHERE name LIKE '%(Distance Learning)%';

INSERT INTO software_tools (id, name)
VALUES
(uuid_generate_v4(), 'Prototyping'),
(uuid_generate_v4(), 'Simulation')
