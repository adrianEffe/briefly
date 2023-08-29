-- Add up migration script here

CREATE TABLE IF NOT EXISTS briefly(
    id TEXT PRIMARY KEY NOT NULL, 
    url TEXT NOT NULL,
    created_at TIMESTAMPTZ
  );
