-- Add migration script here
CREATE TABLE IF NOT EXISTS meetings (
    id UUID PRIMARY KEY NOT NULL,
    studio_id UUID NOT NULL,
    name TEXT NOT NULL,
    date TIMESTAMPTZ NOT NULL
);