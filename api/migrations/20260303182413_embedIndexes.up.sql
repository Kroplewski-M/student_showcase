-- Add up migration script here
CREATE INDEX IF NOT EXISTS users_embedding_idx ON users USING hnsw (embedding vector_cosine_ops);
CREATE INDEX IF NOT EXISTS projects_embedding_idx ON projects USING hnsw (embedding vector_cosine_ops);
