import type { components } from '../../generated/api/schema';
/**
 * Shared domain types for the LightNVR web UI. Field lists come from the JSON
 * the C handlers in src/web/ build; the handler is named on each type.
 *
 * A required field is one its handler always emits. A field is optional when
 * the handler emits it conditionally, when only some surfaces emit it, or when
 * it could not be verified. Domain-specific shapes live in js/types/<domain>.ts.
 */

/** Error body shape used by the API error responses the UI reads. */
export interface ApiErrorBody {
  error?: string;
  message?: string;
}

// ---------------------------------------------------------------------------
// Playback transports
// ---------------------------------------------------------------------------

export type LiveTransport = 'webrtc' | 'mse' | 'hls';

/** Per-stream profiles with a fixed chain; see PLAYBACK_TRANSPORT_CHAINS. */
export type PlaybackTransportProfile =
  | 'webrtc_only'
  | 'mse_only'
  | 'hls_only'
  | 'webrtc_then_mse'
  | 'mse_then_hls';

export type PlaybackTransportPreference = 'auto' | PlaybackTransportProfile;

/** A transport is unavailable only when its entry is explicitly false. */
export type TransportOfferings = Partial<Record<LiveTransport, boolean>>;

// ---------------------------------------------------------------------------
// Streams: api_handlers_streams_get.c
// ---------------------------------------------------------------------------

/** summary_status() plus the stream_status_t names in handle_get_streams. */
export type StreamStatus =
  | 'Running'
  | 'Degraded'
  | 'Error'
  | 'Stopped'
  | 'Starting'
  | 'Stopping'
  | 'Reconnecting'
  | 'Unknown';

/** fleet_availability_state_name() in core/camera_selector.c. */
export type StreamAvailability = 'live' | 'offline' | 'never_connected' | 'disabled';

/** The `availability` query parameter of GET /api/streams?summary=true. */
export type StreamAvailabilityFilter = StreamAvailability | 'all';

/** The `surface` query parameter of GET /api/streams?summary=true. */
export type StreamSurface = 'admin' | 'live' | 'lpr';

/**
 * Item of GET /api/streams?summary=true (handle_get_stream_summaries). The lpr
 * surface returns only camera_uuid and name, so everything else is optional.
 */
export interface StreamSummary {
  camera_uuid: string;
  name: string;
  location_uuid?: string;
  location_name?: string;
  location_path?: string;
  enabled?: boolean;
  width?: number;
  height?: number;
  fps?: number;
  codec?: string;
  protocol?: number;
  record?: boolean;
  record_on_schedule?: boolean;
  detection_based_recording?: boolean;
  isOnvif?: boolean;
  /** Comma-separated tag string. */
  tags?: string;
  status?: StreamStatus;
  availability?: StreamAvailability;
  first_video_at?: number;
  last_frame_ts?: number;
  current_fps?: number;
  recording_active?: boolean;
  last_recording_at?: number;
  can_configure?: boolean;
  /** admin surface only, and only when the caller asks for it. */
  admin_url?: string;
  // live surface only:
  streaming_enabled?: boolean;
  playback_transport?: PlaybackTransportPreference;
  eptz_config?: string;
  detection_model?: string;
  privacy_mode?: boolean;
  can_control_privacy?: boolean;
  has_sub_stream?: boolean;
  ptz_enabled?: boolean;
  backchannel_enabled?: boolean;
  go2rtc_hls_available?: boolean;
}

export interface StreamSummariesResponse {
  streams?: StreamSummary[];
  total?: number;
  page?: number;
  page_size?: number;
  total_pages?: number;
}

/**
 * Full stream configuration from GET /api/streams and GET /api/streams/:name
 * (handle_get_streams, handle_get_stream). Credential-bearing fields are blanked
 * by the server for callers without configuration rights.
 */
export interface Stream {
  camera_uuid: string;
  location_uuid: string;
  name: string;
  url: string;
  enabled: boolean;
  streaming_enabled: boolean;
  playback_transport: PlaybackTransportPreference;
  eptz_config: string;
  width: number;
  height: number;
  fps: number;
  codec: string;
  priority: number;
  record: boolean;
  segment_duration: number;
  detection_based_recording: boolean;
  detection_model: string;
  /** Percent (0 to 100); the server stores a fraction. */
  detection_threshold: number;
  detection_interval: number;
  pre_detection_buffer: number;
  post_detection_buffer: number;
  detection_object_filter: string;
  detection_object_filter_list: string;
  protocol: number;
  record_audio: boolean;
  audio_voice_enhancement: boolean;
  isOnvif: boolean;
  backchannel_enabled: boolean;
  retention_days: number;
  detection_retention_days: number;
  max_storage_mb: number;
  tier_critical_multiplier: number;
  tier_important_multiplier: number;
  tier_ephemeral_multiplier: number;
  storage_priority: number;
  ptz_enabled: boolean;
  ptz_max_x: number;
  ptz_max_y: number;
  ptz_max_z: number;
  ptz_has_home: boolean;
  onvif_username: string;
  onvif_password: string;
  onvif_profile: string;
  onvif_port: number;
  record_on_schedule: boolean;
  /** 168 hourly flags. Omitted if the server could not allocate the array. */
  recording_schedule?: boolean[];
  detection_record_on_schedule: boolean;
  /** 168 hourly flags. Omitted if the server could not allocate the array. */
  detection_recording_schedule?: boolean[];
  /** Comma-separated tag string. */
  tags: string;
  admin_url: string;
  privacy_mode: boolean;
  can_control_privacy: boolean;
  motion_trigger_source: string;
  go2rtc_source_override: string;
  sub_stream_url: string;
  has_sub_stream: boolean;
  detection_url: string;
  publish_url: string;
  status: StreamStatus;
  /** Only present when status is 'Error' and a cause is known. */
  error_message?: string;
  go2rtc_hls_available: boolean;
}

// ---------------------------------------------------------------------------
// Recordings: api_handlers_recordings_list.c
// ---------------------------------------------------------------------------

export interface DetectionLabelCount {
  label: string;
  count: number;
}

/** Item of GET /api/recordings. */
export interface Recording {
  id: number;
  stream: string;
  camera_uuid: string | null;
  file_path: string;
  start_time: string;
  /** Null while the recording is in progress. */
  end_time: string | null;
  start_time_unix: number;
  end_time_unix: number | null;
  in_progress: boolean;
  /** Saved check result for an interrupted recording; empty means no known issue. */
  playback_issue?: string;
  duration: number;
  /** Formatted for display, for example "12.3 MB". */
  size: string;
  /** recording_capture_method(): 'scheduled', 'continuous', or the trigger type. */
  capture_method: string;
  schedule_restricted: boolean | null;
  has_detection: boolean;
  protected: boolean;
  // recording_source_add_status() in recording_source.c; omitted if its query fails:
  storage_state?: 'deletion_pending' | 'preparing' | 'unavailable' | 'archiving' | 'archived' | 'hot_and_archive' | 'hot';
  deletion_pending?: boolean;
  external_source?: boolean;
  storage_target_uuid?: string;
  durable_copy_count?: number;
  storage_policy_uuid?: string;
  storage_policy_version?: number;
  retention_mode?: string;
  /** Only present when the recording has detection labels. */
  detection_labels?: DetectionLabelCount[];
  tags: string[];
}

export interface RecordingsPagination {
  page: number;
  pages: number;
  total: number;
  limit: number;
}

export interface RecordingsResponse {
  recordings: Recording[];
  pagination: RecordingsPagination;
}

/** Counts from GET /api/recordings/batch-delete/progress/:token. */
export interface BatchDeleteCounts {
  succeeded?: number;
  failed?: number;
  pending_deletions?: number;
}

// ---------------------------------------------------------------------------
// Users and sessions: api_handlers_users_backend_agnostic.c,
// api_handlers_auth_backend_agnostic.c
// ---------------------------------------------------------------------------

/** role_names in database/db_auth.c, plus its out-of-range fallback. */
export type UserRoleName = 'admin' | 'user' | 'viewer' | 'api' | 'unknown';

/** Item of GET /api/auth/users. `role` is numeric here; see role_name. */
export type User = components['schemas']['AdminUser'];

/**
 * GET /api/auth/verify (handle_auth_verify). `role` is a role name here, unlike
 * User.role. Demo and auth-disabled responses omit the account fields.
 */
export interface AuthVerifyResponse {
  authenticated?: boolean;
  demo_mode?: boolean;
  id?: number;
  username?: string;
  email?: string;
  role?: UserRoleName;
  role_id?: number;
  is_active?: boolean;
  password_change_locked?: boolean;
  must_change_password?: boolean;
  auth_enabled?: boolean;
  auth_timeout_hours?: number;
  auth_absolute_timeout_hours?: number;
  trusted_device_days?: number;
}

/** What validateSession() in utils/auth-utils.ts resolves with. */
export interface SessionInfo {
  /** Unavailable is not rejection; callers must not clear credentials for it. */
  status: 'valid' | 'invalid' | 'unavailable';
  valid: boolean;
  authenticated?: boolean;
  demo_mode?: boolean;
  id?: number;
  username?: string;
  email?: string;
  role?: UserRoleName;
  role_id?: number;
  is_active?: boolean;
  password_change_locked?: boolean;
  must_change_password?: boolean;
  auth_enabled?: boolean;
}

/** What getUserInfo() in utils/auth-utils.ts resolves with. */
export type SessionUser = Omit<SessionInfo, 'status' | 'valid' | 'authenticated' | 'demo_mode' | 'auth_enabled'>;

// ---------------------------------------------------------------------------
// Client configuration: api_handlers_settings.c
// ---------------------------------------------------------------------------

/**
 * GET /api/client-config, or the local fallback in utils/settings-utils.ts. All
 * fields are optional because older servers and the fallback omit some of them.
 */
export interface ClientConfig {
  go2rtc_enabled?: boolean;
  go2rtc_available?: boolean;
  go2rtc_api_port?: number;
  go2rtc_base_path?: string;
  go2rtc_force_native_hls?: boolean;
  web_auth_enabled?: boolean;
  demo_mode?: boolean;
  audio_disabled?: boolean;
  auto_disabled?: boolean;
  webrtc_disabled?: boolean;
  hls_disabled?: boolean;
  mse_disabled?: boolean;
  default_playback_transport?: string;
  webrtc_connection_timeout_ms?: number;
  webrtc_ice_recovery_timeout_ms?: number;
  generate_thumbnails?: boolean;
  thumbnails_per_recording?: number;
  // Only in the local fallback:
  web_port?: number;
  web_bind_ip?: string;
}

// ---------------------------------------------------------------------------
// Workspaces: api_handlers_workspaces.c
// ---------------------------------------------------------------------------

export interface WorkspaceEntry {
  key?: string;
  label?: string;
  description?: string;
  visible?: boolean;
  configurable?: boolean;
  authorization_unchanged?: boolean;
}

/** GET and PUT /api/ui/workspaces. */
export interface WorkspacesResponse {
  workspaces?: WorkspaceEntry[];
}

// ---------------------------------------------------------------------------
// Detections: api_handlers_detection.c
// ---------------------------------------------------------------------------

export interface Detection {
  label: string;
  confidence: number;
  x: number;
  y: number;
  width: number;
  height: number;
  timestamp?: number;
  end_timestamp?: number;
}

/** GET /api/detection/results/:stream. */
export interface DetectionResultsResponse {
  detections?: Detection[];
  timestamp?: string;
  stream_status?: string;
}
