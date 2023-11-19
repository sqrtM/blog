CREATE TABLE IF NOT EXISTS reply
(
    reply_id         UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    reply_author_id  UUID REFERENCES users (user_id),
    reply_content    TEXT NOT NULL,
    reply_created_at TIMESTAMPTZ   DEFAULT NOW(),
    reply_updated_at TIMESTAMPTZ   DEFAULT NOW(),
    reply_post_id    UUID NOT NULL REFERENCES thread (thread_id)
);

CREATE TABLE IF NOT EXISTS reply_relation
(
    parent_reply_id UUID REFERENCES reply (reply_id),
    child_reply_id  UUID REFERENCES reply (reply_id),
    PRIMARY KEY (parent_reply_id, child_reply_id)
);
