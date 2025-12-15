-- Add per-stream detection API URL override

-- migrate:up

ALTER TABLE streams ADD COLUMN detection_api_url TEXT DEFAULT '';

-- migrate:down

SELECT 1;

