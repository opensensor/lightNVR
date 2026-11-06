/**
 * Types for the settings view and its tabs.
 *
 * API shapes come from src/web/api_handlers_settings.c and the storage, auth,
 * and go2rtc handlers named on each section. Fields a handler omits in some
 * branches are optional.
 */

import type { Dispatch, StateUpdater } from 'preact/hooks';
import type { t as translate } from '../i18n.ts';
import type { FleetSelector, LocationLike, TagLike } from './fleet.ts';
import type { FormNumber } from './streams.ts';

export type Translate = typeof translate;

// ---------------------------------------------------------------------------
// Health settings: api_handlers_settings.c
// ---------------------------------------------------------------------------

export interface HealthCondition {
  code: string;
  unit?: string;
  warning?: number;
  critical?: number;
  recovery?: number;
  warning_for_seconds?: number;
  critical_for_seconds?: number;
  recovery_for_seconds?: number;
}

/** A condition override: either `{ code, profile }` or custom thresholds. */
export interface HealthConditionOverride extends HealthCondition {
  profile?: string;
}

export interface HealthConditionOverrides {
  version?: number;
  conditions?: HealthConditionOverride[];
}

export interface HealthEffectivePolicy {
  conditions?: HealthCondition[];
}

export interface HealthSettingsResponse {
  health_enabled?: boolean;
  health_profile?: string;
  health_fast_interval_seconds?: number;
  health_normal_interval_seconds?: number;
  health_slow_interval_seconds?: number;
  health_device_interval_seconds?: number;
  health_write_probe_enabled?: boolean;
  health_hardware_provider?: string;
  health_presence_interval_seconds?: number;
  health_incident_retention_days?: number;
  health_condition_overrides?: HealthConditionOverrides | null;
  health_effective_policy?: HealthEffectivePolicy;
}

export type HealthNumericField =
  | 'healthFastIntervalSeconds'
  | 'healthNormalIntervalSeconds'
  | 'healthSlowIntervalSeconds'
  | 'healthDeviceIntervalSeconds'
  | 'healthPresenceIntervalSeconds'
  | 'healthIncidentRetentionDays';

export type HealthSettingsState = Record<HealthNumericField, string> & {
  healthEnabled: boolean;
  healthProfile: string;
  healthWriteProbeEnabled: boolean;
  healthHardwareProvider: string;
  healthConditionOverrides: HealthConditionOverrides;
  healthEffectivePolicy: HealthEffectivePolicy;
};

// ---------------------------------------------------------------------------
// Settings: api_handlers_settings.c
// ---------------------------------------------------------------------------

/** GET /api/settings. */
export interface SettingsResponse extends HealthSettingsResponse {
  log_level?: number;
  syslog_enabled?: boolean;
  syslog_ident?: string;
  syslog_facility?: string;
  storage_path?: string;
  storage_path_hls?: string;
  mp4_directory_format?: string;
  max_storage_size?: number;
  retention_days?: number;
  auto_delete_oldest?: boolean;
  storage_min_free_pct?: number;
  storage_pressure_warning_pct?: number;
  storage_pressure_critical_pct?: number;
  storage_pressure_emergency_pct?: number;
  generate_thumbnails?: boolean;
  thumbnails_per_recording?: number;
  db_path?: string;
  db_backup_interval_minutes?: number;
  db_backup_retention_count?: number;
  db_backup_verify?: string;
  db_post_backup_script?: string;
  web_port?: number;
  web_bind_ip?: string;
  web_thread_pool_size?: number;
  max_streams?: number;
  max_streams_ceiling?: number;
  web_auth_enabled?: boolean;
  demo_mode?: boolean;
  audio_disabled?: boolean;
  auto_disabled?: boolean;
  webrtc_disabled?: boolean;
  hls_disabled?: boolean;
  mse_disabled?: boolean;
  auth_timeout_hours?: number;
  auth_absolute_timeout_hours?: number;
  trusted_device_days?: number;
  trusted_proxy_cidrs?: string;
  force_mfa_on_login?: boolean;
  login_rate_limit_enabled?: boolean;
  login_rate_limit_max_attempts?: number;
  login_rate_limit_window_seconds?: number;
  buffer_size?: number;
  use_swap?: boolean;
  swap_size?: number;
  models_path?: string;
  api_detection_url?: string;
  api_detection_backend?: string;
  api_detection_format?: string;
  api_detection_detector_name?: string;
  default_detection_threshold?: number;
  pre_detection_buffer?: number;
  post_detection_buffer?: number;
  default_playback_transport?: string;
  detection_grace_period?: number;
  detection_engine_enabled?: boolean;
  detection_engine_threads?: number;
  detection_engine_delegate?: string;
  go2rtc_enabled?: boolean;
  go2rtc_binary_path?: string;
  go2rtc_config_dir?: string;
  go2rtc_api_port?: number;
  go2rtc_rtsp_port?: number;
  go2rtc_webrtc_enabled?: boolean;
  go2rtc_webrtc_listen_port?: number;
  go2rtc_stun_enabled?: boolean;
  go2rtc_stun_server?: string;
  go2rtc_external_ip?: string;
  go2rtc_ice_servers?: string;
  go2rtc_force_native_hls?: boolean;
  go2rtc_config_override?: string;
  go2rtc_config_override_disabled_reason?: string;
  go2rtc_config_override_quarantined?: string;
  mqtt_enabled?: boolean;
  mqtt_broker_host?: string;
  mqtt_broker_port?: number;
  mqtt_username?: string;
  mqtt_password?: string;
  mqtt_client_id?: string;
  mqtt_topic_prefix?: string;
  mqtt_tls_enabled?: boolean;
  mqtt_keepalive?: number;
  mqtt_qos?: number;
  mqtt_retain?: boolean;
  mqtt_ha_discovery?: boolean;
  mqtt_ha_discovery_prefix?: string;
  mqtt_ha_snapshot_interval?: number;
  turn_enabled?: boolean;
  turn_server_url?: string;
  turn_username?: string;
  turn_password?: string;
  onvif_discovery_enabled?: boolean;
  onvif_discovery_interval?: number;
  onvif_discovery_network?: string;
}

/** POST /api/settings. */
export interface SaveSettingsResponse {
  success?: boolean;
  restart_required?: boolean;
  restart_required_message?: string;
}

/**
 * The settings form. Text and number inputs hold strings once loaded. Fields
 * that start as numbers and can be edited are FormNumber. The optional fields
 * are absent from the initial state and appear once the settings load.
 */
export interface SettingsFormState extends HealthSettingsState {
  logLevel: string;
  syslogEnabled: boolean;
  syslogIdent: string;
  syslogFacility: string;
  storagePath: string;
  storagePathHls: string;
  mp4DirectoryFormat?: string;
  maxStorage: string;
  retention: string;
  autoDelete: boolean;
  minFreePct?: string;
  pressureWarningPct?: string;
  pressureCriticalPct?: string;
  pressureEmergencyPct?: string;
  generateThumbnails: boolean;
  thumbnailsPerRecording: FormNumber;
  dbPath: string;
  dbBackupIntervalMinutes: string;
  dbBackupRetentionCount: string;
  dbBackupVerify: string;
  dbPostBackupScript: string;
  webPort: string;
  webBindIp: string;
  webThreadPoolSize: string;
  maxStreams: string;
  maxStreamsCeiling: number;
  authEnabled: boolean;
  demoMode: boolean;
  audioDisabled: boolean;
  autoDisabled: boolean;
  webrtcDisabled: boolean;
  hlsDisabled: boolean;
  mseDisabled: boolean;
  authTimeoutHours: string;
  authAbsoluteTimeoutHours: string;
  trustedDeviceDays: string;
  trustedProxyCidrs: string;
  forceMfaOnLogin: boolean;
  loginRateLimitEnabled: boolean;
  loginRateLimitMaxAttempts: string;
  loginRateLimitWindowSeconds: string;
  bufferSize: string;
  useSwap: boolean;
  swapSize: string;
  detectionModelsPath: string;
  apiDetectionUrl: string;
  apiDetectionBackend: string;
  apiDetectionFormat: string;
  apiDetectionDetectorName: string;
  defaultDetectionThreshold: number;
  defaultPreBuffer: FormNumber;
  defaultPostBuffer: FormNumber;
  defaultPlaybackTransport?: string;
  detectionGracePeriod: FormNumber;
  detectionEngineEnabled: boolean;
  detectionEngineThreads: FormNumber;
  detectionEngineDelegate: string;
  go2rtcEnabled: boolean;
  go2rtcBinaryPath: string;
  go2rtcConfigDir: string;
  go2rtcApiPort: string;
  go2rtcRtspPort: string;
  go2rtcWebrtcEnabled: boolean;
  go2rtcWebrtcListenPort: string;
  go2rtcStunEnabled: boolean;
  go2rtcStunServer: string;
  go2rtcExternalIp: string;
  go2rtcIceServers: string;
  go2rtcForceNativeHls: boolean;
  go2rtcConfigOverride: string;
  go2rtcOverrideDisabledReason?: string;
  go2rtcOverrideQuarantined?: string;
  mqttEnabled: boolean;
  mqttBrokerHost: string;
  mqttBrokerPort: string;
  mqttUsername: string;
  mqttPassword: string;
  mqttClientId: string;
  mqttTopicPrefix: string;
  mqttTlsEnabled: boolean;
  mqttKeepalive: string;
  mqttQos: string;
  mqttRetain: boolean;
  mqttHaDiscovery: boolean;
  mqttHaDiscoveryPrefix: string;
  mqttHaSnapshotInterval: string;
  turnEnabled: boolean;
  turnServerUrl: string;
  turnUsername: string;
  turnPassword: string;
  onvifDiscoveryEnabled: boolean;
  onvifDiscoveryInterval: string;
  onvifDiscoveryNetwork: string;
}

export type SetSettings = Dispatch<StateUpdater<SettingsFormState>>;

/** The event target fields handleInputChange reads. */
export interface SettingsFieldTarget {
  name: string;
  type?: string;
  value?: string;
  checked?: boolean;
}

/** AuthTab calls handleInputChange with this instead of a DOM event. */
export interface SettingsFieldChange {
  target: SettingsFieldTarget;
}

export type SettingsInputHandler = (e: Event | SettingsFieldChange) => void;

/** Props shared by the tabs that edit `settings`. */
export interface SettingsTabProps {
  settings: SettingsFormState;
  handleInputChange: SettingsInputHandler;
  canModifySettings: boolean;
  t: Translate;
}

// ---------------------------------------------------------------------------
// Auth sessions and trusted devices: api_handlers_auth_backend_agnostic.c
// ---------------------------------------------------------------------------

export type AuthSession = import('../../generated/api/schema').components['schemas']['AuthSession'];
export type AuthSessionsResponse = import('../../generated/api/schema').components['schemas']['AuthSessionsResponse'];
export type TrustedDevice = import('../../generated/api/schema').components['schemas']['TrustedDevice'];
export type TrustedDevicesResponse = import('../../generated/api/schema').components['schemas']['TrustedDevicesResponse'];

/** The part of a revoke mutation AuthTab uses. */
export interface RevokeMutation {
  mutateAsync: (id: number) => Promise<unknown>;
}

// ---------------------------------------------------------------------------
// go2rtc override: api_handlers_settings.c, api_handlers_system_go2rtc.c
// ---------------------------------------------------------------------------

/** POST /api/settings/go2rtc/validate. */
export interface Go2rtcOverrideValidation {
  valid?: boolean;
  libyaml_available?: boolean;
  skipped?: boolean;
  error?: {
    line?: number;
    column?: number;
    message?: string;
  } | null;
  warnings?: string[];
}

/** GET /api/system/go2rtc/effective-config, or a local `{ error }` on failure. */
export interface Go2rtcEffectiveConfig {
  error?: string;
  redaction_available?: boolean;
  base?: string;
  override?: string;
  warnings?: string[];
}

/** GET /api/system/go2rtc/override-status. */
export interface Go2rtcOverrideStatus {
  expected_file_path?: string;
  db_bytes: number;
  file_present?: boolean;
  file_bytes?: number;
  content_matches?: boolean;
  quarantine_active?: boolean;
  quarantine_reason?: string;
  process_state_known?: boolean;
  process_running?: boolean;
  process_pid?: number;
}

// ---------------------------------------------------------------------------
// Storage targets: api_handlers_storage_targets.c
// ---------------------------------------------------------------------------

export interface StorageTargetHealth {
  status?: string;
  capacity_bytes?: number;
  available_bytes?: number;
  used_pct?: number;
  pressure?: string;
  duplicate_filesystem?: boolean;
  cleanup_target_bytes?: number;
  last_error?: string;
}

export interface StorageTarget {
  uuid: string;
  name: string;
  target_type?: string;
  root_path: string;
  endpoint?: string;
  region?: string;
  bucket?: string;
  credential_ref?: string;
  archive_budget_bytes?: number;
  enabled: boolean;
  is_default?: boolean;
  mount_required: boolean;
  mount_guard_path?: string | null;
  storage_class: string;
  reserve_bytes?: number;
  high_watermark_pct: number;
  low_watermark_pct: number;
  recording_count?: number;
  recording_bytes?: number;
  replica_count?: number;
  replica_bytes?: number;
  migration_bandwidth_bps?: number;
  archival_window_start_minute?: number;
  archival_window_end_minute?: number;
  revision: number;
  health?: StorageTargetHealth;
}

export interface StorageTargetsResponse {
  targets?: StorageTarget[];
}

export interface StorageTargetProbeResponse {
  health?: StorageTargetHealth;
}

export interface StorageTargetEditorState {
  uuid?: string;
  revision?: number;
  is_default?: boolean;
  recording_count?: number;
  replica_count?: number;
  target_type: string;
  endpoint: string;
  region: string;
  bucket: string;
  credential_ref: string;
  archive_budget_gb: string;
  name: string;
  root_path: string;
  enabled: boolean;
  mount_required: boolean;
  storage_class: string;
  reserve_gb: string;
  high_watermark_pct: string;
  low_watermark_pct: string;
  migration_mbps: string;
  archival_window_start: string;
  archival_window_end: string;
}

export interface StorageTargetPayload {
  target_type: string;
  endpoint: string;
  region: string;
  bucket: string;
  credential_ref: string;
  archive_budget_bytes: number;
  name: string;
  root_path: string;
  enabled: boolean;
  mount_required: boolean;
  storage_class: string;
  reserve_bytes: number;
  low_watermark_pct: number;
  high_watermark_pct: number;
  migration_bandwidth_bps: number;
  archival_window_start_minute: number;
  archival_window_end_minute: number;
  revision?: number;
}

// ---------------------------------------------------------------------------
// Storage pools: api_handlers_storage_pools.c
// ---------------------------------------------------------------------------

export interface StoragePoolMember {
  target_uuid: string;
  position?: number;
  weight?: number;
}

export interface StoragePool {
  uuid: string;
  name: string;
  strategy: string;
  enabled: boolean;
  revision: number;
  members?: StoragePoolMember[];
}

export interface StoragePoolsResponse {
  pools?: StoragePool[];
}

export interface StoragePoolEditorState {
  uuid?: string;
  revision?: number;
  name: string;
  strategy: string;
  enabled: boolean;
  /** Target UUIDs, in order. */
  members: string[];
}

export interface StoragePoolPayload {
  name: string;
  strategy: string;
  enabled: boolean;
  members: Required<StoragePoolMember>[];
  revision?: number;
}

// ---------------------------------------------------------------------------
// Storage policies: api_handlers_storage_policies.c
// ---------------------------------------------------------------------------

export interface StoragePolicy {
  uuid: string;
  name: string;
  enabled: boolean;
  priority: number;
  selector?: FleetSelector;
  primary_target_uuid: string;
  primary_pool_uuid?: string | null;
  fallback_mode: string;
  fallback_target_uuid?: string | null;
  minimum_retention_days?: number;
  desired_retention_days?: number;
  maximum_retention_days?: number;
  required_copy_count?: number;
  replication_pool_uuid?: string | null;
  migration_after_days?: number;
  migration_target_uuid?: string | null;
  pressure_priority?: number;
  archive_after_seconds?: number;
  hot_residency_seconds?: number;
  archive_protected?: boolean;
  archive_on_pressure?: boolean;
  revision: number;
}

export interface StoragePoliciesResponse {
  policies?: StoragePolicy[];
}

export interface StoragePolicyConflict {
  policy_uuid: string;
  policy_name: string;
  priority: number;
  overlap_camera_count: number;
  draft_precedes: boolean;
}

/** POST /api/storage-policies/preview. */
export interface StoragePolicyPreview {
  matched_camera_count: number;
  effective_camera_count: number;
  shadowed_camera_count: number;
  conflict_policy_count: number;
  conflicts: StoragePolicyConflict[];
}

/** POST /api/storage-policies/<uuid>/apply. */
export interface StoragePolicyApplyResult {
  recordings: number;
  bytes: number;
  past_new_retention_limit: number;
  unfinished_transfers: number;
}

export interface StoragePolicyApplyPreview extends StoragePolicyApplyResult {
  policy: StoragePolicy;
}

export interface StoragePolicyEditorState {
  uuid?: string;
  revision?: number;
  name: string;
  enabled: boolean;
  priority: string;
  /** CollectionSelectorBuilder reports null while its draft is invalid. */
  selector?: FleetSelector | null;
  selector_error?: string;
  primary_target_uuid: string;
  primary_pool_uuid: string;
  fallback_mode: string;
  fallback_target_uuid: string;
  minimum_retention_days: string;
  desired_retention_days: string;
  maximum_retention_days: string;
  required_copy_count: string;
  replication_pool_uuid: string;
  migration_after_days: string;
  migration_target_uuid: string;
  /** A number from the API until edited. */
  archive_after_seconds?: FormNumber;
  /** A number from the API until edited. */
  hot_residency_seconds?: FormNumber;
  archive_protected?: boolean;
  archive_on_pressure?: boolean;
  pressure_priority: string;
  preview?: StoragePolicyPreview | null;
  preview_key?: string;
  preview_request_key?: string;
}

export interface StoragePolicyPayload {
  uuid?: string;
  revision?: number;
  name: string;
  enabled: boolean;
  priority: number;
  selector?: FleetSelector | null;
  primary_target_uuid: string;
  primary_pool_uuid: string | null;
  fallback_mode: string;
  fallback_target_uuid: string | null;
  minimum_retention_days: number;
  desired_retention_days: number;
  maximum_retention_days: number;
  required_copy_count: number;
  replication_pool_uuid: string | null;
  migration_after_days: number;
  migration_target_uuid: string | null;
  archive_after_seconds: number;
  hot_residency_seconds: number;
  archive_protected: boolean;
  archive_on_pressure: boolean;
  pressure_priority: number;
}

// ---------------------------------------------------------------------------
// Migrations and archive: api_handlers_storage_migrations.c
// ---------------------------------------------------------------------------

export interface StorageMigrationJob {
  uuid: string;
  recording_id: number;
  operation?: string;
  source_target_uuid: string;
  destination_target_uuid: string;
  state: string;
  bytes_total?: number;
  bytes_copied?: number;
  progress?: number;
  attempt_count?: number;
  max_attempts?: number;
  last_error?: string;
  cancel_requested?: boolean;
}

export interface StorageMigrationsResponse {
  jobs?: StorageMigrationJob[];
}

export interface StorageArchiveDeletion {
  uuid: string;
  recording_id: number;
  remaining_objects?: number;
  last_error?: string;
}

/** GET /api/storage-archive. */
export interface StorageArchiveResponse {
  s3_enabled?: boolean;
  recovery_read_only?: boolean;
  archive_bytes?: number;
  transfer_backlog_bytes?: number;
  retrieval_cache_bytes?: number;
  retrieval_jobs?: number;
  pending_deletion_count?: number;
  pending_deletions?: StorageArchiveDeletion[];
}

// ---------------------------------------------------------------------------
// Compliance: api_handlers_storage_compliance.c
// ---------------------------------------------------------------------------

export interface StorageComplianceTarget {
  target_uuid: string;
  target_name: string;
  observed_daily_bytes?: number;
  days_to_high_watermark?: number | null;
  expected_retention_days?: number | null;
  confidence?: string;
}

export interface StorageCompliancePolicy {
  policy_uuid: string;
  policy_name: string;
  achieved_retention_days?: number | null;
  expected_retention_days?: number | null;
  desired_retention_days?: number;
  copy_deficit_recordings: number;
  active_violation_count: number;
}

export interface StorageComplianceViolation {
  uuid: string;
  policy_name: string;
  type: string;
  details?: string;
}

export interface StorageComplianceResponse {
  forecast_note?: string;
  targets?: StorageComplianceTarget[];
  policies?: StorageCompliancePolicy[];
  active_violations?: StorageComplianceViolation[];
}

/** GET /api/locations and /api/camera-tags; the fleet builder reads the entries. */
export interface FleetLocationsResponse {
  locations?: LocationLike[];
}

export interface FleetTagsResponse {
  tags?: TagLike[];
}
