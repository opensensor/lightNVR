-- Add PTZ (Pan-Tilt-Zoom) support

-- migrate:up

ALTER TABLE streams ADD COLUMN ptz_enabled INTEGER DEFAULT 0;
ALTER TABLE streams ADD COLUMN ptz_max_x INTEGER DEFAULT 0;
ALTER TABLE streams ADD COLUMN ptz_max_y INTEGER DEFAULT 0;
ALTER TABLE streams ADD COLUMN ptz_max_z INTEGER DEFAULT 0;
ALTER TABLE streams ADD COLUMN ptz_has_home INTEGER DEFAULT 0;

-- migrate:down

SELECT 1;

