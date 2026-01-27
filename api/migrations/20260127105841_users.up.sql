-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "vector";

CREATE TABLE users(
    id VARCHAR(7) NOT NULL PRIMARY KEY,
    first_name VARCHAR(50) NULL,
    last_name VARCHAR(50) NULL,
    password TEXT NULL,
    personal_email VARCHAR(250) UNIQUE,
    verified BOOLEAN NOT NULL DEFAULT false,
    embedding vector(384),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
