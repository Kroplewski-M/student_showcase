-- Add down migration script here
ALTER TABLE user_links DROP COLUMN name;
ALTER TABLE project_links DROP COLUMN name;
