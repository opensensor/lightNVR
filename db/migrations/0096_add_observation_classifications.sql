-- INT_01 P2 classification annotations on object observations and the
-- observations.annotate authorization action. Normative contract:
-- docs/integrations/OBSERVATIONS_CONTRACT.md (sections 10, 12.1).
--
-- One row per (observation, "<writer_namespace>/<name>") key. Write authority
-- is pinned to writer_user_id, stored when the key is created (D3): user ids
-- are never reused, so a later user with the same username cannot overwrite.
-- There is no foreign key to users, so annotations survive user deletion and
-- become read-only. Times are epoch milliseconds. The original detector
-- fields on object_observations are never modified by an annotation.

-- migrate:up

CREATE TABLE observation_classifications (
    observation_uuid TEXT NOT NULL REFERENCES object_observations(uuid) ON DELETE CASCADE,
    key TEXT NOT NULL CHECK (length(key) BETWEEN 3 AND 65),
    writer_namespace TEXT NOT NULL CHECK (length(writer_namespace) BETWEEN 1 AND 32),
    writer_user_id INTEGER NOT NULL CHECK (writer_user_id > 0),
    classifier TEXT NOT NULL CHECK (length(classifier) BETWEEN 1 AND 64),
    model_version TEXT CHECK (model_version IS NULL OR length(model_version) BETWEEN 1 AND 64),
    label TEXT NOT NULL CHECK (length(CAST(label AS BLOB)) BETWEEN 1 AND 128),
    taxonomy_id TEXT CHECK (taxonomy_id IS NULL OR length(taxonomy_id) BETWEEN 1 AND 64),
    confidence REAL NOT NULL CHECK (confidence >= 0.0 AND confidence <= 1.0),
    classified_at_ms INTEGER NOT NULL CHECK (classified_at_ms >= 0),
    input_asset_uuid TEXT NOT NULL CHECK (length(input_asset_uuid) = 36),
    input_revision INTEGER NOT NULL CHECK (input_revision >= 1),
    kind TEXT NOT NULL CHECK (kind IN ('automated', 'owner_confirmed')),
    annotation_revision INTEGER NOT NULL DEFAULT 1 CHECK (annotation_revision >= 1),
    created_at_ms INTEGER NOT NULL,
    updated_at_ms INTEGER NOT NULL,
    PRIMARY KEY (observation_uuid, key),
    -- The key's namespace segment is the writer's namespace.
    CHECK (substr(key, 1, length(writer_namespace) + 1) = writer_namespace || '/')
);

-- New persisted action-mask positions are appended; existing positions never
-- move. Built-in roles fail closed: only Administrator receives the action.
INSERT INTO authz_actions
    (action_key, category, description, camera_scoped, destructive, sort_order, bit_index)
VALUES
    ('observations.annotate', 'Integrations',
     'Annotate detection observations with classifier results', 1, 0, 200, 19);

INSERT INTO authz_role_actions (role_uuid, action_key)
SELECT '00000000-0000-4000-8000-000000000001', action_key
FROM authz_actions
WHERE action_key = 'observations.annotate';

-- migrate:down

-- Drops all classification annotations; take a backup first.
DELETE FROM authz_role_actions WHERE action_key = 'observations.annotate';
DELETE FROM authz_actions WHERE action_key = 'observations.annotate';
DROP TABLE IF EXISTS observation_classifications;
