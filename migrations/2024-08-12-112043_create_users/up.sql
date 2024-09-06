CREATE TABLE users (
                       id UUID PRIMARY KEY,
                       name TEXT NOT NULL,
                       email TEXT NOT NULL,
                       created_at TIMESTAMPTZ NULL
);
