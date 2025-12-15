-- Add record audio column

-- migrate:up

ALTER TABLE streams ADD COLUMN record_audio INTEGER DEFAULT 1;

-- migrate:down

SELECT 1;

