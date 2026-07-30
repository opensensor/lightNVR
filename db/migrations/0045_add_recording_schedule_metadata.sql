ALTER TABLE recordings ADD COLUMN schedule_restricted INTEGER DEFAULT NULL;
ALTER TABLE streams ADD COLUMN detection_record_on_schedule INTEGER NOT NULL DEFAULT 0;
ALTER TABLE streams ADD COLUMN detection_recording_schedule TEXT DEFAULT NULL;
