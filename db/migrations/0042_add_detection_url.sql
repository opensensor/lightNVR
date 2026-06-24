-- Add detection_url to streams table
--
-- Optional secondary stream URL used exclusively for object detection
-- (e.g. a low-resolution MJPEG sub-stream).  When non-empty the Unified
-- Detection Thread opens this URL in a dedicated background thread and runs
-- the configured detection model against its frames, while the main stream
-- URL is used only for buffering and MP4 recording.
--
-- MJPEG streams are ideal because every frame is a full JPEG keyframe,
-- removing the large inter-keyframe gap that limits detection accuracy on
-- the primary H.264/H.265 RTSP stream.

-- migrate:up

ALTER TABLE streams ADD COLUMN detection_url TEXT DEFAULT '';

-- migrate:down

SELECT 1;
