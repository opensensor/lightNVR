-- Hashed, expiring API tokens that narrow their issuing user's authorization.

-- migrate:up

CREATE TABLE authz_api_tokens (
    uuid TEXT PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_by_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    description TEXT NOT NULL,
    token_prefix TEXT NOT NULL,
    token_hash TEXT NOT NULL UNIQUE,
    action_mask INTEGER NOT NULL CHECK (action_mask > 0),
    scope_type TEXT NOT NULL
        CHECK (scope_type IN ('all', 'selector', 'collection')),
    selector_json TEXT,
    collection_uuid TEXT
        REFERENCES camera_collections(uuid) ON DELETE RESTRICT,
    expires_at INTEGER NOT NULL,
    revoked_at INTEGER,
    last_used_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    CHECK (
        (scope_type = 'all' AND selector_json IS NULL AND collection_uuid IS NULL) OR
        (scope_type = 'selector' AND selector_json IS NOT NULL AND collection_uuid IS NULL) OR
        (scope_type = 'collection' AND selector_json IS NULL AND collection_uuid IS NOT NULL)
    ),
    CHECK (expires_at > created_at)
);

CREATE INDEX idx_authz_api_tokens_user
ON authz_api_tokens(user_id, created_at DESC);

CREATE INDEX idx_authz_api_tokens_active
ON authz_api_tokens(token_hash, expires_at)
WHERE revoked_at IS NULL;

-- migrate:down

DROP INDEX IF EXISTS idx_authz_api_tokens_active;
DROP INDEX IF EXISTS idx_authz_api_tokens_user;
DROP TABLE IF EXISTS authz_api_tokens;
SELECT 1;
