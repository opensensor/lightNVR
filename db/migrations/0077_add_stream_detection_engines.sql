-- Ordered, independently configurable detection engines for each stream.

-- migrate:up

CREATE TABLE stream_detection_engines (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    stream_id INTEGER NOT NULL REFERENCES streams(id) ON DELETE CASCADE,
    engine_key TEXT NOT NULL,
    engine_type TEXT NOT NULL CHECK (engine_type IN (
        'motion', 'object', 'onvif', 'api', 'external'
    )),
    model_path TEXT NOT NULL DEFAULT '',
    enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
    threshold REAL NOT NULL DEFAULT 0.5 CHECK (threshold >= 0.0 AND threshold <= 1.0),
    interval_seconds INTEGER NOT NULL DEFAULT 5 CHECK (interval_seconds BETWEEN 1 AND 86400),
    sort_order INTEGER NOT NULL DEFAULT 0,
    config_json TEXT NOT NULL DEFAULT '{}',
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    UNIQUE(stream_id, engine_key)
);

CREATE INDEX idx_stream_detection_engines_stream_order
ON stream_detection_engines(stream_id, enabled DESC, sort_order, id);

-- Preserve the current configuration as the compatibility engine. An empty
-- model remains the existing external-trigger-only mode and is deliberately
-- not represented as a local inference engine.
INSERT INTO stream_detection_engines (
    stream_id, engine_key, engine_type, model_path, enabled,
    threshold, interval_seconds, sort_order, config_json
)
SELECT id,
       'legacy-primary',
       CASE
           WHEN detection_model = 'motion' THEN 'motion'
           WHEN detection_model = 'onvif' THEN 'onvif'
           WHEN detection_model = 'api-detection'
             OR detection_model LIKE 'http://%'
             OR detection_model LIKE 'https://%' THEN 'api'
           ELSE 'object'
       END,
       detection_model,
       1,
       CASE
           WHEN detection_threshold BETWEEN 0.0 AND 1.0
               THEN detection_threshold
           ELSE 0.5
       END,
       CASE
           WHEN detection_interval BETWEEN 1 AND 86400
               THEN detection_interval
           ELSE 5
       END,
       0,
       '{}'
FROM streams
WHERE detection_model IS NOT NULL AND trim(detection_model) <> '';

-- Existing stream APIs continue to write the legacy columns. Keep their
-- compatibility engine synchronized without touching additional engines.
CREATE TRIGGER trg_stream_detection_engines_after_insert
AFTER INSERT ON streams
WHEN NEW.detection_model IS NOT NULL AND trim(NEW.detection_model) <> ''
BEGIN
    INSERT INTO stream_detection_engines (
        stream_id, engine_key, engine_type, model_path, enabled,
        threshold, interval_seconds, sort_order, config_json
    ) VALUES (
        NEW.id,
        'legacy-primary',
        CASE
            WHEN NEW.detection_model = 'motion' THEN 'motion'
            WHEN NEW.detection_model = 'onvif' THEN 'onvif'
            WHEN NEW.detection_model = 'api-detection'
              OR NEW.detection_model LIKE 'http://%'
              OR NEW.detection_model LIKE 'https://%' THEN 'api'
            ELSE 'object'
        END,
        NEW.detection_model,
        1,
        CASE WHEN NEW.detection_threshold BETWEEN 0.0 AND 1.0
             THEN NEW.detection_threshold ELSE 0.5 END,
        CASE WHEN NEW.detection_interval BETWEEN 1 AND 86400
             THEN NEW.detection_interval ELSE 5 END,
        0,
        '{}'
    );
END;

CREATE TRIGGER trg_stream_detection_engines_after_update
AFTER UPDATE OF detection_model, detection_threshold, detection_interval ON streams
BEGIN
    DELETE FROM stream_detection_engines
     WHERE stream_id = NEW.id
       AND engine_key = 'legacy-primary'
       AND (NEW.detection_model IS NULL OR trim(NEW.detection_model) = '');

    INSERT INTO stream_detection_engines (
        stream_id, engine_key, engine_type, model_path, enabled,
        threshold, interval_seconds, sort_order, config_json
    )
    SELECT NEW.id,
           'legacy-primary',
           CASE
               WHEN NEW.detection_model = 'motion' THEN 'motion'
               WHEN NEW.detection_model = 'onvif' THEN 'onvif'
               WHEN NEW.detection_model = 'api-detection'
                 OR NEW.detection_model LIKE 'http://%'
                 OR NEW.detection_model LIKE 'https://%' THEN 'api'
               ELSE 'object'
           END,
           NEW.detection_model,
           1,
           CASE WHEN NEW.detection_threshold BETWEEN 0.0 AND 1.0
                THEN NEW.detection_threshold ELSE 0.5 END,
           CASE WHEN NEW.detection_interval BETWEEN 1 AND 86400
                THEN NEW.detection_interval ELSE 5 END,
           0,
           '{}'
    WHERE NEW.detection_model IS NOT NULL AND trim(NEW.detection_model) <> ''
    ON CONFLICT(stream_id, engine_key) DO UPDATE SET
        engine_type = excluded.engine_type,
        model_path = excluded.model_path,
        enabled = excluded.enabled,
        threshold = excluded.threshold,
        interval_seconds = excluded.interval_seconds,
        updated_at = strftime('%s', 'now');
END;

-- migrate:down

DROP TRIGGER IF EXISTS trg_stream_detection_engines_after_update;
DROP TRIGGER IF EXISTS trg_stream_detection_engines_after_insert;
DROP INDEX IF EXISTS idx_stream_detection_engines_stream_order;
DROP TABLE IF EXISTS stream_detection_engines;
