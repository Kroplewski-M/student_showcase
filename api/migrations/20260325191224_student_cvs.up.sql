-- Add up migration script here
ALTER TABLE users
ADD cv_file_id UUID REFERENCES files(id) NULL;
