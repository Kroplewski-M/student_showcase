-- Add down migration script here
ALTER TABLE projects
DROP COLUMN featured_image_id; 
