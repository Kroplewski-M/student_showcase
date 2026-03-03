-- Add up migration script here
CREATE INDEX ON users USING hnsw (embedding vector_cosine_ops);
CREATE INDEX ON projects USING hnsw (embedding vector_cosine_ops);
