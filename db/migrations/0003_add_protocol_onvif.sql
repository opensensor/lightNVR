-- Add protocol and ONVIF support columns

-- migrate:up

ALTER TABLE streams ADD COLUMN protocol INTEGER DEFAULT 0;
ALTER TABLE streams ADD COLUMN is_onvif INTEGER DEFAULT 0;

-- migrate:down

SELECT 1;

