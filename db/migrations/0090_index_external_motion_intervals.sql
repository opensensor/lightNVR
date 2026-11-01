-- Investigation search derives a lower time bound from external-motion
-- intervals that start before the search window and are still open or end
-- inside it. Without a bound, the window predicate cannot use a timestamp
-- range and walks every historical detection of the selected cameras.
-- External-motion rows are a small fraction of detections, so the partial
-- index stays small.
-- migrate:up
CREATE INDEX IF NOT EXISTS idx_detections_external_motion_camera_end
    ON detections(camera_uuid, event_end_time, timestamp)
    WHERE source = 'external_motion' AND camera_uuid IS NOT NULL;

-- migrate:down
DROP INDEX IF EXISTS idx_detections_external_motion_camera_end;
