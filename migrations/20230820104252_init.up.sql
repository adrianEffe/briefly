-- Add up migration script here

CREATE TABLE briefly(
    id TEXT PRIMARY KEY NOT NULL, 
    url TEXT NOT NULL,
    created_at TIMESTAMPTZ
  );
