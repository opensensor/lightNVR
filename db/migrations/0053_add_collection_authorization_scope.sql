-- Reference shared camera collections directly from authorization grants.

-- migrate:up

CREATE TABLE authz_grants_new (
    uuid TEXT PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_uuid TEXT NOT NULL REFERENCES authz_roles(uuid) ON DELETE RESTRICT,
    scope_type TEXT NOT NULL DEFAULT 'all'
        CHECK (scope_type IN ('all', 'selector', 'collection')),
    selector_json TEXT,
    collection_uuid TEXT
        REFERENCES camera_collections(uuid) ON DELETE RESTRICT,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    CHECK (
        (scope_type = 'all' AND selector_json IS NULL AND collection_uuid IS NULL) OR
        (scope_type = 'selector' AND selector_json IS NOT NULL AND collection_uuid IS NULL) OR
        (scope_type = 'collection' AND selector_json IS NULL AND collection_uuid IS NOT NULL)
    )
);

INSERT INTO authz_grants_new
    (uuid, user_id, role_uuid, scope_type, selector_json, enabled,
     created_at, updated_at)
SELECT uuid, user_id, role_uuid, scope_type, selector_json, enabled,
       created_at, updated_at
FROM authz_grants;

DROP INDEX idx_authz_grants_role;
DROP INDEX idx_authz_grants_user;
DROP TABLE authz_grants;
ALTER TABLE authz_grants_new RENAME TO authz_grants;

CREATE INDEX idx_authz_grants_user
ON authz_grants(user_id, enabled);

CREATE INDEX idx_authz_grants_role
ON authz_grants(role_uuid);

CREATE INDEX idx_authz_grants_collection
ON authz_grants(collection_uuid)
WHERE collection_uuid IS NOT NULL;

-- migrate:down

CREATE TABLE authz_grants_old (
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

INSERT INTO authz_grants_old
    (uuid, user_id, role_uuid, scope_type, selector_json, enabled,
     created_at, updated_at)
SELECT uuid, user_id, role_uuid, scope_type, selector_json, enabled,
       created_at, updated_at
FROM authz_grants
WHERE scope_type != 'collection';

DROP INDEX idx_authz_grants_collection;
DROP INDEX idx_authz_grants_role;
DROP INDEX idx_authz_grants_user;
DROP TABLE authz_grants;
ALTER TABLE authz_grants_old RENAME TO authz_grants;

CREATE INDEX idx_authz_grants_user
ON authz_grants(user_id, enabled);

CREATE INDEX idx_authz_grants_role
ON authz_grants(role_uuid);

SELECT 1;
