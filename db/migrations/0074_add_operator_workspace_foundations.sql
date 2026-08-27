-- Per-user workspace visibility and durable camera video observations.

-- migrate:up

CREATE TABLE user_workspace_preferences (
    owner_user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    workspace_key TEXT NOT NULL CHECK (workspace_key IN (
        'live.navigator',
        'investigation'
    )),
    is_visible INTEGER NOT NULL CHECK (is_visible IN (0, 1)),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE UNIQUE INDEX idx_user_workspace_preferences_owner_key
ON user_workspace_preferences(COALESCE(owner_user_id, 0), workspace_key);

CREATE TABLE camera_observations (
    camera_uuid TEXT PRIMARY KEY
        REFERENCES streams(camera_uuid) ON DELETE CASCADE,
    first_video_at INTEGER NOT NULL DEFAULT 0,
    last_video_at INTEGER NOT NULL DEFAULT 0,
    last_recording_at INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX idx_camera_observations_last_video
ON camera_observations(last_video_at DESC, camera_uuid);

-- Existing completed recordings prove that a camera successfully reported in
-- before this migration, even if no in-process metrics survived the restart.
INSERT INTO camera_observations(
    camera_uuid, first_video_at, last_video_at, last_recording_at
)
SELECT r.camera_uuid,
       MIN(r.start_time),
       MAX(COALESCE(r.end_time, r.start_time)),
       MAX(COALESCE(r.end_time, r.start_time))
FROM recordings r
JOIN streams s ON s.camera_uuid = r.camera_uuid
WHERE r.camera_uuid IS NOT NULL
  AND r.camera_uuid <> ''
  AND r.is_complete = 1
GROUP BY r.camera_uuid;

-- migrate:down

DROP INDEX IF EXISTS idx_camera_observations_last_video;
DROP TABLE IF EXISTS camera_observations;
DROP INDEX IF EXISTS idx_user_workspace_preferences_owner_key;
DROP TABLE IF EXISTS user_workspace_preferences;
