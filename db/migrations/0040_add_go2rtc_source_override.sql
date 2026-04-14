-- Add go2rtc_source_override column to streams table
-- When non-empty, this value is written directly into go2rtc.yaml streams
-- section instead of auto-constructing the source URL from the stream URL.
-- Supports single source URLs or multi-source YAML lists for advanced
-- go2rtc features like failover, transcoding, and hardware acceleration.

-- migrate:up
ALTER TABLE streams ADD COLUMN go2rtc_source_override TEXT DEFAULT '';

-- migrate:down
-- SQLite does not support DROP COLUMN in older versions; migration is left intentionally empty.
