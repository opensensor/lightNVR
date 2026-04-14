-- Add go2rtc_source_override and sub_stream_url columns to streams table
--
-- go2rtc_source_override: when non-empty, written directly into go2rtc.yaml
-- streams section instead of auto-constructing the source URL.
--
-- sub_stream_url: optional low-resolution stream URL used for the dashboard
-- grid view while the main URL is used for recording and fullscreen viewing.

-- migrate:up
ALTER TABLE streams ADD COLUMN go2rtc_source_override TEXT DEFAULT '';
ALTER TABLE streams ADD COLUMN sub_stream_url TEXT DEFAULT '';

-- migrate:down
-- SQLite does not support DROP COLUMN in older versions; migration is left intentionally empty.
