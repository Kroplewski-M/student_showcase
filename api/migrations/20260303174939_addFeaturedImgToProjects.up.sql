-- Add up migration script here
ALTER TABLE projects
ADD COLUMN featured_image_id UUID REFERENCES files(id) DEFAULT NULL;
