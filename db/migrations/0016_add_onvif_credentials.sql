-- Add ONVIF credential columns for storing username, password, and profile

-- migrate:up

ALTER TABLE streams ADD COLUMN onvif_username TEXT DEFAULT '';
ALTER TABLE streams ADD COLUMN onvif_password TEXT DEFAULT '';
ALTER TABLE streams ADD COLUMN onvif_profile TEXT DEFAULT '';

-- migrate:down

SELECT 1;

