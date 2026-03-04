-- Add ONVIF port column to streams table
-- Allows specifying a separate ONVIF service port from the stream URL port

-- migrate:up

ALTER TABLE streams ADD COLUMN onvif_port INTEGER DEFAULT 0;

-- migrate:down

SELECT 1;

