-- Add migration script here
CREATE TABLE briefly(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    url TEXT NOT NULL,
    extension TEXT NOT NULL UNIQUE
);
