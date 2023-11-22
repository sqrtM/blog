ALTER TABLE thread
    DROP CONSTRAINT IF EXISTS thread_board_id_fkey;

ALTER TABLE thread
    DROP COLUMN IF EXISTS board_id;

DROP TABLE IF EXISTS board;