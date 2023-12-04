CREATE TABLE IF NOT EXISTS blog_post
(
    blog_id          UUID         NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    blog_title       VARCHAR(255) NOT NULL,
    blog_description TEXT         NOT NULL,
    blog_content     TEXT         NOT NULL,
    blog_created_at  TIMESTAMPTZ           DEFAULT NOW()
);
