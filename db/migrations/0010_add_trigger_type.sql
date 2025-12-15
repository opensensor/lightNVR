-- Add trigger type column to recordings

-- migrate:up

ALTER TABLE recordings ADD COLUMN trigger_type TEXT DEFAULT 'scheduled';

-- migrate:down

SELECT 1;

