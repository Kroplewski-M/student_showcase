-- Add down migration script here
ALTER TABLE user_links ALTER COLUMN id DROP DEFAULT;
