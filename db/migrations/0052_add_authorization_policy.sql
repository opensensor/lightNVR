-- Action-level authorization roles and selector-backed user grants.

-- migrate:up

ALTER TABLE users ADD COLUMN authorization_mode TEXT NOT NULL DEFAULT 'legacy'
CHECK (authorization_mode IN ('legacy', 'policy'));

CREATE TABLE authz_actions (
    action_key TEXT PRIMARY KEY,
    category TEXT NOT NULL,
    description TEXT NOT NULL,
    camera_scoped INTEGER NOT NULL DEFAULT 0,
    destructive INTEGER NOT NULL DEFAULT 0,
    sort_order INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE authz_roles (
    uuid TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE COLLATE NOCASE,
    description TEXT NOT NULL DEFAULT '',
    is_builtin INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE authz_role_actions (
    role_uuid TEXT NOT NULL REFERENCES authz_roles(uuid) ON DELETE CASCADE,
    action_key TEXT NOT NULL REFERENCES authz_actions(action_key) ON DELETE RESTRICT,
    PRIMARY KEY (role_uuid, action_key)
);

CREATE TABLE authz_grants (
    uuid TEXT PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_uuid TEXT NOT NULL REFERENCES authz_roles(uuid) ON DELETE RESTRICT,
    scope_type TEXT NOT NULL DEFAULT 'all'
        CHECK (scope_type IN ('all', 'selector')),
    selector_json TEXT,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    CHECK (
        (scope_type = 'all' AND selector_json IS NULL) OR
        (scope_type = 'selector' AND selector_json IS NOT NULL)
    )
);

CREATE INDEX idx_authz_grants_user
ON authz_grants(user_id, enabled);

CREATE INDEX idx_authz_grants_role
ON authz_grants(role_uuid);

CREATE TABLE authz_policy_state (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    version INTEGER NOT NULL DEFAULT 1,
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

INSERT INTO authz_policy_state (id, version) VALUES (1, 1);

INSERT INTO authz_actions
    (action_key, category, description, camera_scoped, destructive, sort_order)
VALUES
    ('live.view', 'Live video', 'View a camera live stream', 1, 0, 10),
    ('audio.listen', 'Live video', 'Listen to camera audio', 1, 0, 20),
    ('audio.talk', 'Live video', 'Transmit audio to a camera', 1, 0, 30),
    ('recordings.replay', 'Recordings', 'Replay recorded video', 1, 0, 40),
    ('recordings.export', 'Recordings', 'Download or export recorded video', 1, 0, 50),
    ('snapshot.create', 'Recordings', 'Create or download a camera snapshot', 1, 0, 60),
    ('ptz.control', 'Camera operation', 'Move PTZ cameras and manage presets', 1, 0, 70),
    ('evidence.protect', 'Recordings', 'Protect or release recordings from retention', 1, 1, 80),
    ('recording.delete', 'Recordings', 'Permanently delete recordings', 1, 1, 90),
    ('camera.configure', 'Camera administration', 'Add, change, or remove camera configuration', 1, 1, 100),
    ('fleet.execute_job', 'Camera administration', 'Execute a bulk fleet operation', 1, 1, 110),
    ('storage.configure', 'System administration', 'Change storage and retention configuration', 0, 1, 120),
    ('events.configure', 'System administration', 'Change event routes and destinations', 0, 1, 130),
    ('users.manage', 'System administration', 'Manage users, roles, and grants', 0, 1, 140),
    ('system.admin', 'System administration', 'Change or control the lightNVR system', 0, 1, 150);

INSERT INTO authz_roles (uuid, name, description, is_builtin) VALUES
    ('00000000-0000-4000-8000-000000000001', 'Administrator', 'All lightNVR actions over every resource', 1),
    ('00000000-0000-4000-8000-000000000002', 'Operator', 'Day-to-day camera and recording operation', 1),
    ('00000000-0000-4000-8000-000000000003', 'Viewer', 'Read-only live and recording access', 1),
    ('00000000-0000-4000-8000-000000000004', 'API Operator', 'Programmatic camera and recording operation', 1);

INSERT INTO authz_role_actions (role_uuid, action_key)
SELECT '00000000-0000-4000-8000-000000000001', action_key
FROM authz_actions;

INSERT INTO authz_role_actions (role_uuid, action_key) VALUES
    ('00000000-0000-4000-8000-000000000002', 'live.view'),
    ('00000000-0000-4000-8000-000000000002', 'audio.listen'),
    ('00000000-0000-4000-8000-000000000002', 'audio.talk'),
    ('00000000-0000-4000-8000-000000000002', 'recordings.replay'),
    ('00000000-0000-4000-8000-000000000002', 'recordings.export'),
    ('00000000-0000-4000-8000-000000000002', 'snapshot.create'),
    ('00000000-0000-4000-8000-000000000002', 'ptz.control'),
    ('00000000-0000-4000-8000-000000000002', 'evidence.protect'),
    ('00000000-0000-4000-8000-000000000002', 'recording.delete'),
    ('00000000-0000-4000-8000-000000000002', 'camera.configure'),
    ('00000000-0000-4000-8000-000000000003', 'live.view'),
    ('00000000-0000-4000-8000-000000000003', 'audio.listen'),
    ('00000000-0000-4000-8000-000000000003', 'recordings.replay'),
    ('00000000-0000-4000-8000-000000000003', 'recordings.export'),
    ('00000000-0000-4000-8000-000000000003', 'snapshot.create'),
    ('00000000-0000-4000-8000-000000000004', 'live.view'),
    ('00000000-0000-4000-8000-000000000004', 'audio.listen'),
    ('00000000-0000-4000-8000-000000000004', 'audio.talk'),
    ('00000000-0000-4000-8000-000000000004', 'recordings.replay'),
    ('00000000-0000-4000-8000-000000000004', 'recordings.export'),
    ('00000000-0000-4000-8000-000000000004', 'snapshot.create'),
    ('00000000-0000-4000-8000-000000000004', 'ptz.control'),
    ('00000000-0000-4000-8000-000000000004', 'evidence.protect'),
    ('00000000-0000-4000-8000-000000000004', 'recording.delete'),
    ('00000000-0000-4000-8000-000000000004', 'camera.configure');

-- migrate:down

DROP TABLE IF EXISTS authz_policy_state;
DROP INDEX IF EXISTS idx_authz_grants_role;
DROP INDEX IF EXISTS idx_authz_grants_user;
DROP TABLE IF EXISTS authz_grants;
DROP TABLE IF EXISTS authz_role_actions;
DROP TABLE IF EXISTS authz_roles;
DROP TABLE IF EXISTS authz_actions;
SELECT 1;
