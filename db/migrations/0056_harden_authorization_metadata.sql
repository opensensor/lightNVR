-- Make the persisted action-mask layout explicit, stop empty camera UUIDs at
-- the source, and let the legacy tag backfill run once instead of every boot.

-- migrate:up

-- authz_api_tokens.action_mask stores one bit per action, positioned by the
-- authorization_action_t ordinal. Recording that position makes the coupling
-- visible to operators and lets the daemon detect catalog drift at startup
-- instead of silently re-mapping every issued token.
ALTER TABLE authz_actions ADD COLUMN bit_index INTEGER NOT NULL DEFAULT -1;

UPDATE authz_actions SET bit_index = CASE action_key
    WHEN 'live.view'          THEN 0
    WHEN 'audio.listen'       THEN 1
    WHEN 'audio.talk'         THEN 2
    WHEN 'recordings.replay'  THEN 3
    WHEN 'recordings.export'  THEN 4
    WHEN 'snapshot.create'    THEN 5
    WHEN 'ptz.control'        THEN 6
    WHEN 'evidence.protect'   THEN 7
    WHEN 'recording.delete'   THEN 8
    WHEN 'camera.configure'   THEN 9
    WHEN 'fleet.execute_job'  THEN 10
    WHEN 'storage.configure'  THEN 11
    WHEN 'events.configure'   THEN 12
    WHEN 'users.manage'       THEN 13
    WHEN 'system.admin'       THEN 14
    ELSE -1
END;

CREATE UNIQUE INDEX idx_authz_actions_bit_index
ON authz_actions(bit_index);

-- streams.camera_uuid defaults to '' so the 0048 backfill could run, but the
-- unique index then lets exactly one row hold '' and fails every later insert
-- with a confusing constraint error. Reject the empty value outright so any
-- code path that forgets to generate a UUID fails loudly on the first row.
CREATE TRIGGER trg_streams_camera_uuid_insert
BEFORE INSERT ON streams
FOR EACH ROW WHEN NEW.camera_uuid IS NULL OR NEW.camera_uuid = ''
BEGIN
    SELECT RAISE(ABORT, 'streams.camera_uuid must be a generated UUID');
END;

CREATE TRIGGER trg_streams_camera_uuid_update
BEFORE UPDATE OF camera_uuid ON streams
FOR EACH ROW WHEN NEW.camera_uuid IS NULL OR NEW.camera_uuid = ''
BEGIN
    SELECT RAISE(ABORT, 'streams.camera_uuid must be a generated UUID');
END;

-- The 0050 legacy tag backfill is idempotent but rewrites every assignment on
-- every startup. Record completion so it becomes a one-time migration step.
INSERT OR IGNORE INTO system_settings (key, value)
VALUES ('camera_tags_backfill_completed', '0');

-- migrate:down

DROP TRIGGER IF EXISTS trg_streams_camera_uuid_update;
DROP TRIGGER IF EXISTS trg_streams_camera_uuid_insert;
DROP INDEX IF EXISTS idx_authz_actions_bit_index;
DELETE FROM system_settings WHERE key = 'camera_tags_backfill_completed';
SELECT 1;
