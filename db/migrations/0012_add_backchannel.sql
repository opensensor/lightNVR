-- Add backchannel (two-way audio) support

-- migrate:up

ALTER TABLE streams ADD COLUMN backchannel_enabled INTEGER DEFAULT 0;

-- migrate:down

SELECT 1;

