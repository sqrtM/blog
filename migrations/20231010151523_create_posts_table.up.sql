CREATE TABLE IF NOT EXISTS posts
(
    post_id         UUID         NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    post_author_id  UUID REFERENCES users (user_id),
    post_title      VARCHAR(255) NOT NULL,
    post_content    TEXT         NOT NULL,
    post_created_at TIMESTAMPTZ           DEFAULT NOW(),
    post_updated_at TIMESTAMPTZ           DEFAULT NOW()
);
