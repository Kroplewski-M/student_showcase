-- Add down migration script here
DROP INDEX IF EXISTS users_embedding_idx;
DROP INDEX IF EXISTS projects_embedding_idx;

