CREATE TABLE IF NOT EXISTS users
(
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    username VARCHAR(36) NOT NULL UNIQUE,
    password VARCHAR(60) NOT NULL,
    email VARCHAR(60) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_connection TIMESTAMPTZ NOT NULL DEFAULT now()
)