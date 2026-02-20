-- Add up migration script here
ALTER TABLE files
ADD COLUMN extension VARCHAR(10) NOT NULL;
