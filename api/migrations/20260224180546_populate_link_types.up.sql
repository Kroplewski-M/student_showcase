-- Add up migration script here
INSERT INTO link_types
(id, name)
VALUES
(uuid_generate_v4(), 'GitHub'),
(uuid_generate_v4(), 'YouTube'),
(uuid_generate_v4(), 'LinkedIn'),
(uuid_generate_v4(), 'GitLab'),
(uuid_generate_v4(), 'Bitbucket'),
(uuid_generate_v4(), 'Stack Overflow'),
(uuid_generate_v4(), 'Figma'),
(uuid_generate_v4(), 'Live Preview'),
(uuid_generate_v4(), 'Other')

