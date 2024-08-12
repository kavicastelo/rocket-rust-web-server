CREATE TABLE users (
                       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                       name TEXT NOT NULL,
                       email TEXT NOT NULL UNIQUE,
                       created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
