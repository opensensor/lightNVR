-- Add schedule provenance to recordings and a separate weekly schedule for
-- detection-triggered recording.

-- migrate:up

-- schedule_restricted is a tri-state flag:
--   NULL = unset/unknown (for legacy rows or when provenance is unavailable)
--   0    = recording was not schedule-restricted
--   1    = recording was schedule-restricted
ALTER TABLE recordings ADD COLUMN schedule_restricted INTEGER DEFAULT NULL;
ALTER TABLE streams ADD COLUMN detection_record_on_schedule INTEGER NOT NULL DEFAULT 0;
ALTER TABLE streams ADD COLUMN detection_recording_schedule TEXT DEFAULT NULL;

-- migrate:down

ALTER TABLE recordings DROP COLUMN schedule_restricted;
ALTER TABLE streams DROP COLUMN detection_record_on_schedule;
ALTER TABLE streams DROP COLUMN detection_recording_schedule;
