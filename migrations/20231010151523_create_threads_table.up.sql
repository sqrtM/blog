CREATE TABLE IF NOT EXISTS thread
(
    thread_id         UUID         NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    thread_author_id  UUID REFERENCES users (user_id),
    thread_title      VARCHAR(255) NOT NULL,
    thread_content    TEXT         NOT NULL,
    thread_created_at TIMESTAMPTZ           DEFAULT NOW(),
    thread_updated_at TIMESTAMPTZ           DEFAULT NOW()
);
