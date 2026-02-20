-- Add down migration script here
ALTER TABLE files
DROP COLUMN extension;
