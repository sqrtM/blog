CREATE TABLE IF NOT EXISTS board
(
    board_id              UUID         NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    board_name            VARCHAR(255) NOT NULL,
    board_description     TEXT         NOT NULL,
    board_authorized_only BOOLEAN      NOT NULL DEFAULT FALSE
);

ALTER TABLE thread
    ADD COLUMN board_id UUID REFERENCES board (board_id);
