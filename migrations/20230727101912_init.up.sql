-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE briefly(
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    url TEXT NOT NULL,
    extension TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
    );