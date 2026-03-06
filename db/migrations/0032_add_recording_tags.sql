-- Add recording tags support
-- Junction table for many-to-many relationship between recordings and tags

-- migrate:up

CREATE TABLE IF NOT EXISTS recording_tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    recording_id INTEGER NOT NULL,
    tag TEXT NOT NULL,
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (recording_id) REFERENCES recordings(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_recording_tags_unique ON recording_tags(recording_id, tag);
CREATE INDEX IF NOT EXISTS idx_recording_tags_tag ON recording_tags(tag);
CREATE INDEX IF NOT EXISTS idx_recording_tags_recording ON recording_tags(recording_id);

-- migrate:down

DROP INDEX IF EXISTS idx_recording_tags_recording;
DROP INDEX IF EXISTS idx_recording_tags_tag;
DROP INDEX IF EXISTS idx_recording_tags_unique;
DROP TABLE IF EXISTS recording_tags;

