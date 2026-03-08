-- Add optional per-stream camera admin page URL

-- migrate:up

ALTER TABLE streams ADD COLUMN admin_url TEXT DEFAULT '';

-- migrate:down

SELECT 1;