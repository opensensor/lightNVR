-- Shared, lightweight building plans for the operator Live workspace.

-- migrate:up

CREATE TABLE operator_floor_plans (
    uuid TEXT PRIMARY KEY,
    name TEXT NOT NULL COLLATE NOCASE,
    location_uuid TEXT REFERENCES camera_locations(uuid) ON DELETE SET NULL,
    parent_plan_uuid TEXT REFERENCES operator_floor_plans(uuid) ON DELETE SET NULL,
    canvas_width INTEGER NOT NULL DEFAULT 1200 CHECK (canvas_width BETWEEN 400 AND 4000),
    canvas_height INTEGER NOT NULL DEFAULT 800 CHECK (canvas_height BETWEEN 300 AND 4000),
    revision INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE UNIQUE INDEX idx_operator_floor_plans_name
ON operator_floor_plans(name COLLATE NOCASE);

CREATE INDEX idx_operator_floor_plans_location
ON operator_floor_plans(location_uuid, name COLLATE NOCASE);

CREATE TABLE operator_floor_plan_cameras (
    plan_uuid TEXT NOT NULL REFERENCES operator_floor_plans(uuid) ON DELETE CASCADE,
    camera_uuid TEXT NOT NULL REFERENCES streams(camera_uuid) ON DELETE CASCADE,
    x REAL NOT NULL CHECK (x >= 0.0 AND x <= 1.0),
    y REAL NOT NULL CHECK (y >= 0.0 AND y <= 1.0),
    rotation REAL NOT NULL DEFAULT 0.0 CHECK (rotation >= -180.0 AND rotation <= 180.0),
    fov REAL NOT NULL DEFAULT 65.0 CHECK (fov >= 1.0 AND fov <= 180.0),
    PRIMARY KEY (plan_uuid, camera_uuid)
);

CREATE INDEX idx_operator_floor_plan_cameras_camera
ON operator_floor_plan_cameras(camera_uuid, plan_uuid);

-- migrate:down

DROP INDEX IF EXISTS idx_operator_floor_plan_cameras_camera;
DROP TABLE IF EXISTS operator_floor_plan_cameras;
DROP INDEX IF EXISTS idx_operator_floor_plans_location;
DROP INDEX IF EXISTS idx_operator_floor_plans_name;
DROP TABLE IF EXISTS operator_floor_plans;
