/**
 * Types for the System page and the stream Health page. Field lists come from
 * the JSON the C handlers in src/web/ build; the handler is named on each type.
 *
 * A required field is one its handler always emits. A field is optional when
 * the handler emits it conditionally or when the UI's own placeholder state
 * omits it. Metrics the sampler may not have are `number | null`.
 */
import type { t as translate } from '../i18n.ts';

/** The module-level or useI18n() translator, passed as a prop. */
export type Translate = typeof translate;

/** A formatter passed as a prop; system/SystemUtils.ts formatBytes satisfies it. */
export type FormatBytes = (bytes: number) => string;

// ---------------------------------------------------------------------------
// GET /api/system/info: handle_get_system_info in api_handlers_system.c
// ---------------------------------------------------------------------------

export interface SystemCpu {
  model?: string;
  cores?: number;
  usage?: number | null;
  /** system_health_capability_name(): 'available', 'unsupported', and so on. */
  usageCapability?: string;
}

/** memory, go2rtcMemory, detectorMemory, systemMemory, disk, and systemDisk. */
export interface CapacitySection {
  total?: number | null;
  used?: number | null;
  free?: number | null;
  /** Absent means the figures carry no capability verdict; see metricKnown. */
  capability?: string;
}

export interface SystemNetworkInterface {
  name: string;
  address?: string | null;
  ipv6?: string;
  mac?: string;
  up?: boolean;
  capability?: string;
}

/** Item of `streamStorage`: add_cached_stream_storage_usage_to_json in storage_manager_streams_cache.c. */
export interface StreamStorageEntry {
  name: string;
  size: number;
  count: number;
  /** -1 inherits the global value; 0 is unlimited. */
  retention_days: number;
  detection_retention_days: number;
  max_storage_mb: number;
}

export interface SystemVersionItem {
  name: string;
  category: string;
  version: string;
  details?: string;
}

export interface SystemVersions {
  items?: SystemVersionItem[];
}

export interface SystemInfoData {
  version?: string;
  git_commit?: string;
  /** Seconds. The empty string is SystemView's placeholder before the first response. */
  uptime?: number | string;
  cpu?: SystemCpu;
  memory?: CapacitySection;
  go2rtcMemory?: CapacitySection;
  detectorMemory?: CapacitySection;
  systemMemory?: CapacitySection;
  disk?: CapacitySection;
  systemDisk?: CapacitySection;
  threads?: number | null;
  webThreadPoolSize?: number;
  global_retention_days?: number;
  network?: { interfaces?: SystemNetworkInterface[] };
  streams?: { active?: number; total?: number };
  recordings?: { count?: number; size?: number };
  streamStorage?: StreamStorageEntry[];
  versions?: SystemVersions;
}

// ---------------------------------------------------------------------------
// GET /api/system/logs: api_handlers_system_logs.c, core/logger_json.c
// ---------------------------------------------------------------------------

export interface SystemLogEntry {
  timestamp: string;
  level: string;
  message: string;
}

export interface SystemLogsResponse {
  logs?: Partial<SystemLogEntry>[];
  file?: string;
  level?: string;
}

// ---------------------------------------------------------------------------
// GET /api/storage/health: api_handlers_storage.c
// ---------------------------------------------------------------------------

export interface StorageHealthResponse {
  /** disk_pressure_level_str(): NORMAL, WARNING, CRITICAL, EMERGENCY. */
  pressure_level: string;
  pressure_level_num: number;
  free_space_pct: number;
  free_space_bytes: number;
  total_space_bytes: number;
  used_space_bytes: number;
  last_check_time: number;
  last_cleanup_time: number;
  last_deep_time: number;
  last_cleanup_deleted: number;
  last_cleanup_freed: number;
  capacity_target_free_pct: number;
  fill_rate_bytes_per_day: number;
  /** Below zero means unknown; zero means already at or over the target. */
  projected_seconds_to_target: number;
}

// ---------------------------------------------------------------------------
// GET /api/storage-targets: api_handlers_storage_targets.c. Only what
// system/storageTiers.ts reads; the Settings page owns the full shape.
// ---------------------------------------------------------------------------

export type StorageClass = 'hot' | 'warm' | 'cold';

export interface StorageTierTargetHealth {
  status?: string;
  capacity_known?: boolean;
  capacity_bytes?: number;
  used_bytes?: number;
  pressure?: string;
  duplicate_filesystem?: boolean;
}

export interface StorageTierTarget {
  uuid?: string;
  name?: string;
  target_type?: string;
  storage_class?: string;
  enabled?: boolean;
  is_default?: boolean;
  recording_count?: number;
  recording_bytes?: number;
  replica_bytes?: number;
  archive_budget_bytes?: number;
  health?: StorageTierTargetHealth;
}

export interface StorageTargetsResponse {
  count?: number;
  targets?: StorageTierTarget[];
}

export interface StorageMeter {
  kind: 'filesystem' | 'budget';
  used: number;
  total: number;
}

/** What storageTierRow() returns. */
export interface StorageTierRow {
  uuid: string;
  name: string;
  kind: 'bucket' | 'disk';
  storageClass: StorageClass;
  isDefault: boolean;
  recordingCount: number;
  recordingBytes: number;
  replicaBytes: number;
  storedBytes: number;
  meter: StorageMeter | null;
  pressure: string;
  status: string;
  duplicateFilesystem: boolean;
}

/** What summarizeStorageTiers() returns. */
export interface StorageTierSummary {
  rows: StorageTierRow[];
  byClass: Record<StorageClass, number>;
  storedBytes: number;
  tiered: boolean;
}

// ---------------------------------------------------------------------------
// GET /api/system/health and /incidents: api_handlers_system_health.c
// ---------------------------------------------------------------------------

export interface HealthObservation {
  metric: string;
  resource: string;
  scope: string;
  capability: string;
  freshness: string;
  /** system_health_unit_name(): 'ratio', 'bytes', 'seconds', and so on. */
  unit: string;
  value: number | null;
  sampled_monotonic_ms?: number;
  observed_at_ms?: number;
}

/** policy_to_json(). */
export interface HealthThresholdPolicy {
  condition: string;
  enabled: boolean;
  overridden: boolean;
  direction: string;
  unit: string;
  warning_threshold: number;
  critical_threshold: number;
  recovery_threshold: number;
  warning_for_seconds: number;
  critical_for_seconds: number;
  recovery_for_seconds: number;
}

/** incident_view_to_json(): the active incidents of /api/system/health. */
export interface HealthActiveIncident {
  incident_id: string;
  condition: string;
  subject: string;
  scope: string;
  state: string;
  severity: string;
  first_observed_at_ms: number;
  last_observed_at_ms: number;
  persistence_pending: boolean;
  observation: HealthObservation;
  thresholds?: HealthThresholdPolicy;
}

/** safe_stored_observation(): historical values have no live freshness. */
export type HealthStoredObservation = Pick<HealthObservation,
  'metric' | 'resource' | 'scope' | 'capability' | 'unit' | 'value'>;

/** stored_incident_to_json(): the items of /api/system/health/incidents. */
export interface HealthStoredIncident {
  incident_id: string;
  condition: string;
  subject: string;
  scope: string;
  state: string;
  severity: string;
  first_observed_at_ms: number;
  last_observed_at_ms: number;
  closed_at_ms: number | null;
  reconciliation: string;
  revision: number;
  observation: HealthStoredObservation;
}

export interface HealthIncidentsResponse {
  schema_version?: number;
  count?: number;
  incidents?: HealthStoredIncident[];
  next_cursor?: string | null;
}

export interface HealthCollectorStats {
  collector: string;
  scope: string;
  tier: number;
  attempts: number;
  completions: number;
  failures: number;
  timeouts: number;
  overlap_skips: number;
  last_duration_ms: number;
  maximum_duration_ms: number;
  busy: boolean;
  stale: boolean;
}

export interface HealthEventDelivery {
  degraded?: boolean;
  circular_report_path?: boolean;
  outbox_full?: boolean;
  all_destinations_unavailable?: boolean;
  /** add_outbox_stats(); the counts are only present when available is true. */
  outbox?: { available?: boolean };
}

export interface HealthIncidentPersistence {
  pending?: number;
  failures?: number;
  retries?: number;
}

export interface HealthSelfObservability {
  collectors?: HealthCollectorStats[];
  coverage_overflows?: number;
  incident_persistence?: HealthIncidentPersistence;
  event_delivery?: HealthEventDelivery;
}

export interface HealthRecentSample {
  sequence: number;
  completed_at_ms: number;
  observation_count: number;
  observations_dropped: number;
}

export interface SystemHealthResponse {
  schema_version?: number;
  monitoring?: {
    configured_enabled?: boolean;
    initialized?: boolean;
    enabled?: boolean;
    worker_threads?: number;
    restart_required?: boolean;
  };
  overall_state?: string;
  visibility?: {
    effective_scope?: string;
    host_hardware_visible?: boolean;
    coverage_boundary?: string;
  };
  snapshot?: {
    available?: boolean;
    sequence?: number | null;
    completed_at_ms?: number | null;
    age_ms?: number | null;
    freshness?: string;
  };
  coverage?: {
    complete?: boolean;
    incomplete_count?: number;
    unavailable_observations?: number;
    observations_dropped?: number;
    collection_errors?: number;
    collection_timeouts?: number;
    abandoned_helpers?: number;
    coverage_overflows?: number;
  };
  observations?: HealthObservation[];
  thresholds?: HealthThresholdPolicy[];
  active_incidents?: HealthActiveIncident[];
  recent_samples?: HealthRecentSample[];
  self_observability?: HealthSelfObservability;
}

/** What healthCoverageMessage() returns: two i18n keys and a tone. */
export interface HealthCoverageMessage {
  title: string;
  message: string;
  warning: boolean;
}

// ---------------------------------------------------------------------------
// GET /api/health: handle_get_health in api_handlers_health.c
// ---------------------------------------------------------------------------

export interface StreamHealthCounts {
  total: number;
  up: number;
  degraded: number;
  down: number;
}

/** Item of `streams_detail` in the full response. */
export interface StreamHealthDetail {
  name: string;
  status: string;
  observed_fps: number;
  current_fps: number;
  expected_fps: number;
  fps: number;
  configured_fps: number;
  bitrate_bps: number;
  uptime_seconds: number;
  reconnects: number;
  last_packet_ts: number;
  last_frame_ts: number;
  last_completed_segment_ts: number;
  frames_total: number;
  frames_dropped: number;
  connection_latency_ms: number;
  source_connected: boolean;
  video_advancing: boolean;
  recorder_writing: boolean;
  detector_operating: boolean;
  recorder_session_active: boolean;
  recording_active: boolean;
  recording_gaps: number;
  /** Only with ?sparklines=true. */
  sparkline_fps?: number[];
  sparkline_bitrate?: number[];
}

/** Item of `streams_detail` with ?history_only=true. */
export type StreamHealthHistoryItem = Pick<StreamHealthDetail, 'name' | 'sparkline_fps' | 'sparkline_bitrate'>;

/** Full response. The liveness and history forms omit the fleet fields. */
export interface StreamHealthResponse {
  healthy?: boolean;
  status?: string;
  uptime?: number;
  totalRequests?: number;
  failedRequests?: number;
  timestamp?: string;
  streams?: StreamHealthCounts;
  storage?: { used_bytes?: number; available_bytes?: number };
  streams_detail?: StreamHealthDetail[];
}

export interface StreamHealthHistoryResponse {
  streams_detail?: StreamHealthHistoryItem[];
}
