-- Durable host-health incident episodes, transitions, and process-run markers.

-- migrate:up

CREATE TABLE system_health_process_runs (
    run_id TEXT PRIMARY KEY
        CHECK (length(run_id) = 36 AND substr(run_id, 9, 1) = '-'
            AND substr(run_id, 14, 1) = '-' AND substr(run_id, 19, 1) = '-'
            AND substr(run_id, 24, 1) = '-'),
    boot_id TEXT NOT NULL
        CHECK (length(boot_id) BETWEEN 1 AND 63
            AND boot_id NOT GLOB '*[^A-Za-z0-9_.:-]*'),
    started_at_ms INTEGER NOT NULL CHECK (started_at_ms > 0),
    closed_at_ms INTEGER CHECK (
        closed_at_ms IS NULL OR closed_at_ms >= started_at_ms),
    clean_close INTEGER NOT NULL DEFAULT 0 CHECK (clean_close IN (0, 1)),
    CHECK ((clean_close = 0 AND closed_at_ms IS NULL)
        OR (clean_close = 1 AND closed_at_ms IS NOT NULL))
);

CREATE INDEX idx_system_health_runs_latest
ON system_health_process_runs(started_at_ms DESC, run_id DESC);

CREATE TABLE system_health_incidents (
    uuid TEXT PRIMARY KEY
        CHECK (length(uuid) = 36 AND substr(uuid, 9, 1) = '-'
            AND substr(uuid, 14, 1) = '-' AND substr(uuid, 19, 1) = '-'
            AND substr(uuid, 24, 1) = '-'),
    condition_code TEXT NOT NULL CHECK (condition_code IN (
        'memory.available_low', 'memory.oom_kill', 'memory.swap_thrash',
        'cpu.saturation', 'cpu.throttled', 'io.pressure',
        'filesystem.bytes_low', 'filesystem.inodes_low',
        'filesystem.read_only', 'filesystem.write_failed', 'thermal.high',
        'network.link_down', 'network.error_rate',
        'clock.unsynchronized', 'clock.jump', 'process.fd_exhaustion',
        'process.pid_exhaustion', 'process.allocation_failed',
        'storage.device_prefail', 'storage.device_critical',
        'hardware.ecc_corrected', 'hardware.ecc_uncorrectable',
        'hardware.fan_failed', 'hardware.power_unstable',
        'health.collector_stale', 'system.unexpected_restart',
        'event.delivery_degraded')),
    subject TEXT NOT NULL CHECK (length(subject) BETWEEN 1 AND 63
        AND subject NOT GLOB '*[^A-Za-z0-9_.:-]*'),
    scope TEXT NOT NULL CHECK (
        scope IN ('process', 'container', 'host', 'filesystem', 'device')),
    state TEXT NOT NULL CHECK (state IN ('open', 'recovering', 'closed')),
    severity TEXT NOT NULL CHECK (severity IN ('warning', 'error', 'critical')),
    first_seen_at_ms INTEGER NOT NULL CHECK (first_seen_at_ms > 0),
    last_seen_at_ms INTEGER NOT NULL CHECK (last_seen_at_ms >= first_seen_at_ms),
    closed_at_ms INTEGER,
    last_observation_json TEXT NOT NULL
        CHECK (length(last_observation_json) BETWEEN 2 AND 2048),
    alert_event_id TEXT CHECK (alert_event_id IS NULL OR
        (length(alert_event_id) = 36 AND substr(alert_event_id, 9, 1) = '-'
            AND substr(alert_event_id, 14, 1) = '-'
            AND substr(alert_event_id, 19, 1) = '-'
            AND substr(alert_event_id, 24, 1) = '-')),
    recovery_event_id TEXT CHECK (recovery_event_id IS NULL OR
        (length(recovery_event_id) = 36 AND substr(recovery_event_id, 9, 1) = '-'
            AND substr(recovery_event_id, 14, 1) = '-'
            AND substr(recovery_event_id, 19, 1) = '-'
            AND substr(recovery_event_id, 24, 1) = '-')),
    reconciliation_state TEXT NOT NULL DEFAULT 'none' CHECK (
        reconciliation_state IN ('none', 'alert_pending', 'recovery_pending',
                                 'reconciled', 'delivery_failed')),
    boot_id TEXT NOT NULL CHECK (length(boot_id) BETWEEN 1 AND 63
        AND boot_id NOT GLOB '*[^A-Za-z0-9_.:-]*'),
    run_id TEXT NOT NULL CHECK (length(run_id) = 36
        AND substr(run_id, 9, 1) = '-' AND substr(run_id, 14, 1) = '-'
        AND substr(run_id, 19, 1) = '-' AND substr(run_id, 24, 1) = '-'),
    revision INTEGER NOT NULL DEFAULT 1 CHECK (revision >= 1),
    CHECK ((state = 'closed' AND closed_at_ms IS NOT NULL
            AND closed_at_ms >= last_seen_at_ms)
        OR (state <> 'closed' AND closed_at_ms IS NULL))
);

CREATE UNIQUE INDEX idx_system_health_incidents_active
ON system_health_incidents(condition_code, subject)
WHERE state <> 'closed';

CREATE INDEX idx_system_health_incidents_list
ON system_health_incidents(last_seen_at_ms DESC, uuid DESC);

CREATE TABLE system_health_incident_transitions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transition_uuid TEXT NOT NULL UNIQUE CHECK (length(transition_uuid) = 36
        AND substr(transition_uuid, 9, 1) = '-'
        AND substr(transition_uuid, 14, 1) = '-'
        AND substr(transition_uuid, 19, 1) = '-'
        AND substr(transition_uuid, 24, 1) = '-'),
    incident_uuid TEXT NOT NULL REFERENCES system_health_incidents(uuid)
        ON DELETE CASCADE,
    kind TEXT NOT NULL CHECK (
        kind IN ('open', 'escalation', 'material_change', 'recovery', 'one_shot')),
    from_state TEXT CHECK (
        from_state IS NULL OR from_state IN ('open', 'recovering', 'closed')),
    to_state TEXT NOT NULL CHECK (to_state IN ('open', 'recovering', 'closed')),
    severity TEXT NOT NULL CHECK (severity IN ('warning', 'error', 'critical')),
    observed_at_ms INTEGER NOT NULL CHECK (observed_at_ms > 0),
    safe_observation_json TEXT NOT NULL
        CHECK (length(safe_observation_json) BETWEEN 2 AND 2048),
    event_id TEXT CHECK (event_id IS NULL OR (length(event_id) = 36
        AND substr(event_id, 9, 1) = '-' AND substr(event_id, 14, 1) = '-'
        AND substr(event_id, 19, 1) = '-' AND substr(event_id, 24, 1) = '-')),
    reconciliation_state TEXT NOT NULL CHECK (
        reconciliation_state IN ('none', 'alert_pending', 'recovery_pending',
                                 'reconciled', 'delivery_failed')),
    boot_id TEXT NOT NULL CHECK (length(boot_id) BETWEEN 1 AND 63
        AND boot_id NOT GLOB '*[^A-Za-z0-9_.:-]*'),
    run_id TEXT NOT NULL CHECK (length(run_id) = 36
        AND substr(run_id, 9, 1) = '-' AND substr(run_id, 14, 1) = '-'
        AND substr(run_id, 19, 1) = '-' AND substr(run_id, 24, 1) = '-')
);

CREATE INDEX idx_system_health_transitions_incident
ON system_health_incident_transitions(
    incident_uuid, observed_at_ms DESC, id DESC);

CREATE INDEX idx_system_health_transitions_retention
ON system_health_incident_transitions(observed_at_ms, id);

-- migrate:down

DROP INDEX IF EXISTS idx_system_health_transitions_retention;
DROP INDEX IF EXISTS idx_system_health_transitions_incident;
DROP TABLE IF EXISTS system_health_incident_transitions;
DROP INDEX IF EXISTS idx_system_health_incidents_list;
DROP INDEX IF EXISTS idx_system_health_incidents_active;
DROP TABLE IF EXISTS system_health_incidents;
DROP INDEX IF EXISTS idx_system_health_runs_latest;
DROP TABLE IF EXISTS system_health_process_runs;
