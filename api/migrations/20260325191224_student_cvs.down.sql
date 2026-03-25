-- Add down migration script here
ALTER TABLE users
DROP COLUMN cv_file_id;
