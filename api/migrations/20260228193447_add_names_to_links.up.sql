-- Add up migration script here
ALTER TABLE user_links
ADD COLUMN name VARCHAR(50) NULL;

ALTER TABLE project_links
ADD COLUMN name VARCHAR(50) NULL;
