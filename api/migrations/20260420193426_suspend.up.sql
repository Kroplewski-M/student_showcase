-- Add up migration script here
ALTER TABLE users
ADD suspended BOOLEAN NOT NULL DEFAULT false;
