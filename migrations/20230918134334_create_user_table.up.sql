CREATE TABLE IF NOT EXISTS users
(
    user_id              UUID        NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    user_username        VARCHAR(36) NOT NULL UNIQUE,
    user_password        VARCHAR(60) NOT NULL,
    user_recovery_key    VARCHAR(60) NOT NULL,
    user_created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    user_last_connection TIMESTAMPTZ NOT NULL DEFAULT now()
)
