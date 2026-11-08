unsafe extern "C" {
    pub type sqlite3;
    pub type fleet_selector;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut g_config: config_t;
    unsafe fn fleet_selector_parse(
        json: *const cJSON,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> *mut fleet_selector_t;
    unsafe fn fleet_selector_free(selector: *mut fleet_selector_t);
    unsafe fn fleet_selector_matches(
        selector: *const fleet_selector_t,
        camera: *const fleet_camera_t,
        explanation: *mut fleet_selector_explanation_t,
    ) -> bool;
    unsafe fn db_auth_get_role_name(role: user_role_t) -> *const ::core::ffi::c_char;
    unsafe fn camera_collection_filter_load_for_authorization(
        collection_uuid: *const ::core::ffi::c_char,
        filter: *mut camera_collection_filter_t,
    ) -> camera_collection_filter_result_t;
    unsafe fn camera_collection_filter_load_on_connection(
        connection: *mut sqlite3,
        collection_uuid: *const ::core::ffi::c_char,
        filter: *mut camera_collection_filter_t,
    ) -> camera_collection_filter_result_t;
    unsafe fn camera_collection_filter_free(filter: *mut camera_collection_filter_t);
    unsafe fn camera_collection_filter_matches(
        filter: *const camera_collection_filter_t,
        camera: *const fleet_camera_t,
    ) -> bool;
    unsafe fn db_api_token_get_active(
        token_uuid: *const ::core::ffi::c_char,
        token: *mut api_token_t,
    ) -> db_api_token_result_t;
    unsafe fn db_api_token_get_active_on_connection(
        db: *mut sqlite3,
        observed_at: int64_t,
        token_uuid: *const ::core::ffi::c_char,
        token: *mut api_token_t,
    ) -> db_api_token_result_t;
    unsafe fn db_authorization_role_get(
        uuid: *const ::core::ffi::c_char,
        role: *mut authorization_role_t,
    ) -> db_authorization_result_t;
    unsafe fn db_authorization_get_user_policy(
        user_id: int64_t,
        mode: *mut ::core::ffi::c_char,
        grants: *mut *mut authorization_grant_t,
        grant_count: *mut ::core::ffi::c_int,
        policy_version: *mut int64_t,
    ) -> db_authorization_result_t;
    unsafe fn db_authorization_load_user_grants(
        user_id: int64_t,
        action_key: *const ::core::ffi::c_char,
        grants: *mut *mut authorization_grant_t,
        grant_count: *mut ::core::ffi::c_int,
        policy_version: *mut int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_authorization_get_policy_version_readonly(
        version: *mut int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_authorization_get_policy_version_on_connection(
        db: *mut sqlite3,
        version: *mut int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_authorization_load_user_grants_on_connection(
        db: *mut sqlite3,
        user_id: int64_t,
        action_key: *const ::core::ffi::c_char,
        grants: *mut *mut authorization_grant_t,
        grant_count: *mut ::core::ffi::c_int,
        policy_version: *mut int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcat(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cJSON {
    pub next: *mut cJSON,
    pub prev: *mut cJSON,
    pub child: *mut cJSON,
    pub r#type: ::core::ffi::c_int,
    pub valuestring: *mut ::core::ffi::c_char,
    pub valueint: ::core::ffi::c_int,
    pub valuedouble: ::core::ffi::c_double,
    pub string: *mut ::core::ffi::c_char,
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_policy_settings_t {
    pub enabled: bool,
    pub profile: [::core::ffi::c_char; 16],
    pub fast_interval_seconds: uint32_t,
    pub normal_interval_seconds: uint32_t,
    pub slow_interval_seconds: uint32_t,
    pub device_interval_seconds: uint32_t,
    pub write_probe_enabled: bool,
    pub hardware_provider: [::core::ffi::c_char; 16],
    pub presence_interval_seconds: uint32_t,
    pub incident_retention_days: uint32_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_protocol_t(pub ::core::ffi::c_uint);
impl stream_protocol_t {
    pub const STREAM_PROTOCOL_TCP: Self = Self(0);
    pub const STREAM_PROTOCOL_UDP: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_config_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub location_uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 256],
    pub url: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub fps: ::core::ffi::c_int,
    pub codec: [::core::ffi::c_char; 16],
    pub priority: ::core::ffi::c_int,
    pub record: bool,
    pub segment_duration: ::core::ffi::c_int,
    pub detection_based_recording: bool,
    pub detection_model: [::core::ffi::c_char; 512],
    pub detection_interval: ::core::ffi::c_int,
    pub detection_threshold: ::core::ffi::c_float,
    pub pre_detection_buffer: ::core::ffi::c_int,
    pub post_detection_buffer: ::core::ffi::c_int,
    pub detection_api_url: [::core::ffi::c_char; 512],
    pub detection_object_filter: [::core::ffi::c_char; 16],
    pub detection_object_filter_list: [::core::ffi::c_char; 256],
    pub streaming_enabled: bool,
    pub protocol: stream_protocol_t,
    pub record_audio: bool,
    pub audio_voice_enhancement: bool,
    pub onvif_username: [::core::ffi::c_char; 64],
    pub onvif_password: [::core::ffi::c_char; 64],
    pub onvif_profile: [::core::ffi::c_char; 64],
    pub onvif_port: ::core::ffi::c_int,
    pub onvif_discovery_enabled: bool,
    pub is_onvif: bool,
    pub backchannel_enabled: bool,
    pub retention_days: ::core::ffi::c_int,
    pub detection_retention_days: ::core::ffi::c_int,
    pub max_storage_mb: ::core::ffi::c_int,
    pub tier_critical_multiplier: ::core::ffi::c_double,
    pub tier_important_multiplier: ::core::ffi::c_double,
    pub tier_ephemeral_multiplier: ::core::ffi::c_double,
    pub storage_priority: ::core::ffi::c_int,
    pub ptz_enabled: bool,
    pub ptz_max_x: ::core::ffi::c_int,
    pub ptz_max_y: ::core::ffi::c_int,
    pub ptz_max_z: ::core::ffi::c_int,
    pub ptz_has_home: bool,
    pub record_on_schedule: bool,
    pub recording_schedule: [uint8_t; 168],
    pub detection_record_on_schedule: bool,
    pub detection_recording_schedule: [uint8_t; 168],
    pub tags: [::core::ffi::c_char; 256],
    pub admin_url: [::core::ffi::c_char; 512],
    pub privacy_mode: bool,
    pub motion_trigger_source: [::core::ffi::c_char; 256],
    pub go2rtc_source_override: [::core::ffi::c_char; 8192],
    pub sub_stream_url: [::core::ffi::c_char; 512],
    pub detection_url: [::core::ffi::c_char; 512],
    pub publish_url: [::core::ffi::c_char; 512],
    pub playback_transport: [::core::ffi::c_char; 24],
    pub eptz_config: [::core::ffi::c_char; 1024],
    pub observation_evidence_enabled: bool,
    pub observation_labels: [::core::ffi::c_char; 512],
    pub observation_max_age_days: ::core::ffi::c_int,
    pub observation_max_bytes: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_engine_config_t {
    pub enabled: bool,
    pub num_threads: ::core::ffi::c_int,
    pub delegate: [::core::ffi::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_t {
    pub pid_file: [::core::ffi::c_char; 512],
    pub log_file: [::core::ffi::c_char; 512],
    pub log_level: ::core::ffi::c_int,
    pub syslog_enabled: bool,
    pub syslog_ident: [::core::ffi::c_char; 64],
    pub syslog_facility: ::core::ffi::c_int,
    pub storage_path: [::core::ffi::c_char; 512],
    pub storage_path_hls: [::core::ffi::c_char; 512],
    pub max_storage_size: uint64_t,
    pub retention_days: ::core::ffi::c_int,
    pub auto_delete_oldest: bool,
    pub mp4_faststart: bool,
    pub storage_min_free_pct: ::core::ffi::c_int,
    pub storage_pressure_warning_pct: ::core::ffi::c_double,
    pub storage_pressure_critical_pct: ::core::ffi::c_double,
    pub storage_pressure_emergency_pct: ::core::ffi::c_double,
    pub generate_thumbnails: bool,
    pub thumbnails_per_recording: ::core::ffi::c_int,
    pub record_mp4_directly: bool,
    pub mp4_storage_path: [::core::ffi::c_char; 512],
    pub mp4_directory_format: [::core::ffi::c_char; 32],
    pub mp4_segment_duration: ::core::ffi::c_int,
    pub mp4_retention_days: ::core::ffi::c_int,
    pub models_path: [::core::ffi::c_char; 512],
    pub api_detection_url: [::core::ffi::c_char; 512],
    pub api_detection_backend: [::core::ffi::c_char; 32],
    pub api_detection_format: [::core::ffi::c_char; 32],
    pub api_detection_detector_name: [::core::ffi::c_char; 64],
    pub default_detection_threshold: ::core::ffi::c_int,
    pub default_pre_detection_buffer: ::core::ffi::c_int,
    pub default_post_detection_buffer: ::core::ffi::c_int,
    pub default_playback_transport: [::core::ffi::c_char; 24],
    pub detection_grace_period: ::core::ffi::c_int,
    pub db_path: [::core::ffi::c_char; 512],
    pub db_backup_interval_minutes: ::core::ffi::c_int,
    pub db_backup_retention_count: ::core::ffi::c_int,
    pub db_post_backup_script: [::core::ffi::c_char; 512],
    pub db_startup_check: ::core::ffi::c_int,
    pub db_backup_verify: ::core::ffi::c_int,
    pub web_thread_pool_size: ::core::ffi::c_int,
    pub web_port: ::core::ffi::c_int,
    pub web_bind_ip: [::core::ffi::c_char; 32],
    pub web_root: [::core::ffi::c_char; 512],
    pub web_ssl_enabled: bool,
    pub web_ssl_cert_path: [::core::ffi::c_char; 512],
    pub web_ssl_key_path: [::core::ffi::c_char; 512],
    pub web_auth_enabled: bool,
    pub web_username: [::core::ffi::c_char; 32],
    pub web_password: [::core::ffi::c_char; 32],
    pub audio_disabled: bool,
    pub auto_disabled: bool,
    pub webrtc_disabled: bool,
    pub hls_disabled: bool,
    pub mse_disabled: bool,
    pub auth_timeout_hours: ::core::ffi::c_int,
    pub auth_absolute_timeout_hours: ::core::ffi::c_int,
    pub trusted_device_days: ::core::ffi::c_int,
    pub trusted_proxy_cidrs: [::core::ffi::c_char; 1024],
    pub demo_mode: bool,
    pub force_mfa_on_login: bool,
    pub login_rate_limit_enabled: bool,
    pub login_rate_limit_max_attempts: ::core::ffi::c_int,
    pub login_rate_limit_window_seconds: ::core::ffi::c_int,
    pub web_compression_enabled: bool,
    pub web_use_minified_assets: bool,
    pub web_cache_max_age_html: ::core::ffi::c_int,
    pub web_cache_max_age_css: ::core::ffi::c_int,
    pub web_cache_max_age_js: ::core::ffi::c_int,
    pub web_cache_max_age_images: ::core::ffi::c_int,
    pub web_cache_max_age_fonts: ::core::ffi::c_int,
    pub web_cache_max_age_default: ::core::ffi::c_int,
    pub onvif_discovery_enabled: bool,
    pub onvif_discovery_interval: ::core::ffi::c_int,
    pub onvif_discovery_network: [::core::ffi::c_char; 64],
    pub max_streams: ::core::ffi::c_int,
    pub streams: *mut stream_config_t,
    pub buffer_size: ::core::ffi::c_int,
    pub use_swap: bool,
    pub swap_file: [::core::ffi::c_char; 512],
    pub swap_size: uint64_t,
    pub memory_constrained: bool,
    pub hw_accel_enabled: bool,
    pub hw_accel_device: [::core::ffi::c_char; 32],
    pub go2rtc_enabled: bool,
    pub go2rtc_binary_path: [::core::ffi::c_char; 512],
    pub go2rtc_config_dir: [::core::ffi::c_char; 512],
    pub go2rtc_api_port: ::core::ffi::c_int,
    pub go2rtc_rtsp_port: ::core::ffi::c_int,
    pub go2rtc_force_native_hls: bool,
    pub go2rtc_proxy_max_inflight: ::core::ffi::c_int,
    pub go2rtc_webrtc_enabled: bool,
    pub go2rtc_webrtc_listen_port: ::core::ffi::c_int,
    pub go2rtc_stun_enabled: bool,
    pub go2rtc_stun_server: [::core::ffi::c_char; 256],
    pub go2rtc_external_ip: [::core::ffi::c_char; 64],
    pub go2rtc_ice_servers: [::core::ffi::c_char; 512],
    pub webrtc_connection_timeout_ms: ::core::ffi::c_int,
    pub webrtc_ice_recovery_timeout_ms: ::core::ffi::c_int,
    pub turn_enabled: bool,
    pub turn_server_url: [::core::ffi::c_char; 256],
    pub turn_username: [::core::ffi::c_char; 64],
    pub turn_password: [::core::ffi::c_char; 64],
    pub mqtt_enabled: bool,
    pub mqtt_broker_host: [::core::ffi::c_char; 256],
    pub mqtt_broker_port: ::core::ffi::c_int,
    pub mqtt_username: [::core::ffi::c_char; 64],
    pub mqtt_password: [::core::ffi::c_char; 128],
    pub mqtt_client_id: [::core::ffi::c_char; 64],
    pub mqtt_topic_prefix: [::core::ffi::c_char; 128],
    pub mqtt_tls_enabled: bool,
    pub mqtt_keepalive: ::core::ffi::c_int,
    pub mqtt_qos: ::core::ffi::c_int,
    pub mqtt_retain: bool,
    pub mqtt_ha_discovery: bool,
    pub mqtt_ha_discovery_prefix: [::core::ffi::c_char; 128],
    pub mqtt_ha_snapshot_interval: ::core::ffi::c_int,
    pub health: system_health_policy_settings_t,
    pub detection_engine: detection_engine_config_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct fleet_health_state_t(pub ::core::ffi::c_uint);
impl fleet_health_state_t {
    pub const FLEET_HEALTH_UNKNOWN: Self = Self(0);
    pub const FLEET_HEALTH_UP: Self = Self(1);
    pub const FLEET_HEALTH_DEGRADED: Self = Self(2);
    pub const FLEET_HEALTH_DOWN: Self = Self(3);
    pub const FLEET_HEALTH_DISABLED: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct fleet_availability_state_t(pub ::core::ffi::c_uint);
impl fleet_availability_state_t {
    pub const FLEET_AVAILABILITY_LIVE: Self = Self(0);
    pub const FLEET_AVAILABILITY_OFFLINE: Self = Self(1);
    pub const FLEET_AVAILABILITY_NEVER_CONNECTED: Self = Self(2);
    pub const FLEET_AVAILABILITY_DISABLED: Self = Self(3);
    pub const FLEET_AVAILABILITY_COUNT: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_camera_tag_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub label: [::core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_camera_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 256],
    pub address: [::core::ffi::c_char; 512],
    pub legacy_tags: [::core::ffi::c_char; 256],
    pub location_uuid: [::core::ffi::c_char; 37],
    pub location_name: [::core::ffi::c_char; 128],
    pub location_path: [::core::ffi::c_char; 1024],
    pub location_ancestor_uuids: [[::core::ffi::c_char; 37]; 32],
    pub location_depth: ::core::ffi::c_int,
    pub tags: [fleet_camera_tag_t; 64],
    pub tag_count: ::core::ffi::c_int,
    pub manufacturer: [::core::ffi::c_char; 128],
    pub model: [::core::ffi::c_char; 128],
    pub enabled: bool,
    pub streaming_enabled: bool,
    pub record: bool,
    pub detection_based_recording: bool,
    pub is_onvif: bool,
    pub ptz_enabled: bool,
    pub backchannel_enabled: bool,
    pub health: fleet_health_state_t,
    pub availability: fleet_availability_state_t,
    pub health_changed_at: int64_t,
    pub first_video_at: int64_t,
    pub last_frame_ts: int64_t,
    pub last_recording_at: int64_t,
    pub current_fps: ::core::ffi::c_double,
    pub recording_active: bool,
}
pub type fleet_selector_t = fleet_selector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_selector_explanation_t {
    pub clauses: [[::core::ffi::c_char; 160]; 64],
    pub clause_count: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct user_role_t(pub ::core::ffi::c_uint);
impl user_role_t {
    pub const USER_ROLE_ADMIN: Self = Self(0);
    pub const USER_ROLE_USER: Self = Self(1);
    pub const USER_ROLE_VIEWER: Self = Self(2);
    pub const USER_ROLE_API: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_t {
    pub id: int64_t,
    pub username: [::core::ffi::c_char; 64],
    pub email: [::core::ffi::c_char; 128],
    pub role: user_role_t,
    pub api_key: [::core::ffi::c_char; 64],
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub last_login: int64_t,
    pub is_active: bool,
    pub password_change_locked: bool,
    pub must_change_password: bool,
    pub totp_enabled: bool,
    pub allowed_login_cidrs: [::core::ffi::c_char; 1024],
    pub has_login_cidr_restriction: bool,
    pub authorization_mode: [::core::ffi::c_char; 16],
    pub authenticated_via_scoped_token: bool,
    pub api_token_uuid: [::core::ffi::c_char; 37],
    pub authentication_method: [::core::ffi::c_char; 24],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct authorization_action_t(pub ::core::ffi::c_int);
impl authorization_action_t {
    pub const AUTHZ_ACTION_INVALID: Self = Self(-1);
    pub const AUTHZ_LIVE_VIEW: Self = Self(0);
    pub const AUTHZ_AUDIO_LISTEN: Self = Self(1);
    pub const AUTHZ_AUDIO_TALK: Self = Self(2);
    pub const AUTHZ_RECORDINGS_REPLAY: Self = Self(3);
    pub const AUTHZ_RECORDINGS_EXPORT: Self = Self(4);
    pub const AUTHZ_SNAPSHOT_CREATE: Self = Self(5);
    pub const AUTHZ_PTZ_CONTROL: Self = Self(6);
    pub const AUTHZ_EVIDENCE_PROTECT: Self = Self(7);
    pub const AUTHZ_RECORDING_DELETE: Self = Self(8);
    pub const AUTHZ_CAMERA_CONFIGURE: Self = Self(9);
    pub const AUTHZ_FLEET_EXECUTE_JOB: Self = Self(10);
    pub const AUTHZ_STORAGE_CONFIGURE: Self = Self(11);
    pub const AUTHZ_EVENTS_CONFIGURE: Self = Self(12);
    pub const AUTHZ_USERS_MANAGE: Self = Self(13);
    pub const AUTHZ_SYSTEM_ADMIN: Self = Self(14);
    pub const AUTHZ_LPR_READ: Self = Self(15);
    pub const AUTHZ_LPR_SEARCH: Self = Self(16);
    pub const AUTHZ_LPR_EXPORT: Self = Self(17);
    pub const AUTHZ_LPR_DELETE: Self = Self(18);
    pub const AUTHZ_OBSERVATIONS_ANNOTATE: Self = Self(19);
    pub const AUTHZ_ACTION_COUNT: Self = Self(20);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct authorization_action_metadata_t {
    pub action: authorization_action_t,
    pub key: *const ::core::ffi::c_char,
    pub category: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub camera_scoped: bool,
    pub destructive: bool,
    pub enforced: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct authorization_decision_t(pub ::core::ffi::c_uint);
impl authorization_decision_t {
    pub const AUTHZ_DECISION_DENY: Self = Self(0);
    pub const AUTHZ_DECISION_ALLOW: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct authorization_decision_source_t(pub ::core::ffi::c_uint);
impl authorization_decision_source_t {
    pub const AUTHZ_SOURCE_NONE: Self = Self(0);
    pub const AUTHZ_SOURCE_LEGACY_ROLE: Self = Self(1);
    pub const AUTHZ_SOURCE_POLICY_GRANT: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct authorization_evaluation_t {
    pub decision: authorization_decision_t,
    pub source: authorization_decision_source_t,
    pub policy_version: int64_t,
    pub grant_uuid: [::core::ffi::c_char; 37],
    pub token_uuid: [::core::ffi::c_char; 37],
    pub role_uuid: [::core::ffi::c_char; 37],
    pub role_name: [::core::ffi::c_char; 128],
    pub explanation: [::core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct api_token_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub user_id: int64_t,
    pub created_by_user_id: int64_t,
    pub description: [::core::ffi::c_char; 128],
    pub token_prefix: [::core::ffi::c_char; 16],
    pub action_mask: uint64_t,
    pub scope_type: [::core::ffi::c_char; 16],
    pub selector_json: [::core::ffi::c_char; 8192],
    pub collection_uuid: [::core::ffi::c_char; 37],
    pub expires_at: int64_t,
    pub revoked_at: int64_t,
    pub last_used_at: int64_t,
    pub created_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_api_token_result_t(pub ::core::ffi::c_int);
impl db_api_token_result_t {
    pub const DB_API_TOKEN_OK: Self = Self(0);
    pub const DB_API_TOKEN_ERROR: Self = Self(-1);
    pub const DB_API_TOKEN_NOT_FOUND: Self = Self(-2);
    pub const DB_API_TOKEN_INVALID: Self = Self(-3);
    pub const DB_API_TOKEN_LIMIT: Self = Self(-4);
    pub const DB_API_TOKEN_EXPIRED: Self = Self(-5);
    pub const DB_API_TOKEN_REVOKED: Self = Self(-6);
    pub const DB_API_TOKEN_INACTIVE_OWNER: Self = Self(-7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct authorization_grant_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub user_id: int64_t,
    pub role_uuid: [::core::ffi::c_char; 37],
    pub role_name: [::core::ffi::c_char; 128],
    pub scope_type: [::core::ffi::c_char; 16],
    pub selector_json: [::core::ffi::c_char; 8192],
    pub collection_uuid: [::core::ffi::c_char; 37],
    pub enabled: bool,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct authorization_role_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub description: [::core::ffi::c_char; 256],
    pub is_builtin: bool,
    pub action_mask: uint64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_authorization_result_t(pub ::core::ffi::c_int);
impl db_authorization_result_t {
    pub const DB_AUTHORIZATION_OK: Self = Self(0);
    pub const DB_AUTHORIZATION_ERROR: Self = Self(-1);
    pub const DB_AUTHORIZATION_NOT_FOUND: Self = Self(-2);
    pub const DB_AUTHORIZATION_CONFLICT: Self = Self(-3);
    pub const DB_AUTHORIZATION_IMMUTABLE: Self = Self(-4);
    pub const DB_AUTHORIZATION_IN_USE: Self = Self(-5);
    pub const DB_AUTHORIZATION_STALE: Self = Self(-6);
    pub const DB_AUTHORIZATION_INVALID: Self = Self(-7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct authorization_context {
    pub connection: *mut sqlite3,
    pub observed_at: int64_t,
    pub legacy_policy_version_loaded: bool,
    pub legacy_policy_version: int64_t,
    pub grants: *mut grant_cache_entry_t,
    pub grant_count: ::core::ffi::c_int,
    pub grant_capacity: ::core::ffi::c_int,
    pub collections: *mut collection_cache_entry_t,
    pub collection_count: ::core::ffi::c_int,
    pub collection_capacity: ::core::ffi::c_int,
    pub token_loaded: bool,
    pub token_valid: bool,
    pub token_failed: bool,
    pub token: api_token_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct collection_cache_entry_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub filter: camera_collection_filter_t,
    pub failed: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camera_collection_filter_t {
    pub active: bool,
    pub smart_selector: *mut fleet_selector_t,
    pub member_uuids: *mut [::core::ffi::c_char; 37],
    pub member_count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grant_cache_entry_t {
    pub user_id: int64_t,
    pub action: authorization_action_t,
    pub grants: *mut authorization_grant_t,
    pub selectors: *mut *mut fleet_selector_t,
    pub grant_count: ::core::ffi::c_int,
    pub policy_version: int64_t,
    pub failed: bool,
}
pub type authorization_context_t = authorization_context;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct camera_collection_filter_result_t(pub ::core::ffi::c_int);
impl camera_collection_filter_result_t {
    pub const CAMERA_COLLECTION_FILTER_OK: Self = Self(0);
    pub const CAMERA_COLLECTION_FILTER_NOT_FOUND: Self = Self(-1);
    pub const CAMERA_COLLECTION_FILTER_INVALID_SELECTOR: Self = Self(-2);
    pub const CAMERA_COLLECTION_FILTER_OUT_OF_MEMORY: Self = Self(-3);
    pub const CAMERA_COLLECTION_FILTER_DATABASE_ERROR: Self = Self(-4);
}
static mut action_catalog: [authorization_action_metadata_t; 20] = [
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_LIVE_VIEW,
        key: b"live.view\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"Live video\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"View a camera live stream\0".as_ptr() as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#false != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_AUDIO_LISTEN,
        key: b"audio.listen\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"Live video\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Listen to camera audio\0".as_ptr() as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#false != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_AUDIO_TALK,
        key: b"audio.talk\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"Live video\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Transmit audio to a camera\0".as_ptr() as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#false != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_RECORDINGS_REPLAY,
        key: b"recordings.replay\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"Recordings\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Replay recorded video\0".as_ptr() as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#false != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_RECORDINGS_EXPORT,
        key: b"recordings.export\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"Recordings\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Download or export recorded video\0".as_ptr() as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#false != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_SNAPSHOT_CREATE,
        key: b"snapshot.create\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"Recordings\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Create or download a camera snapshot\0".as_ptr()
            as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#false != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_PTZ_CONTROL,
        key: b"ptz.control\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"Camera operation\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Move PTZ cameras and manage presets\0".as_ptr()
            as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#false != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_EVIDENCE_PROTECT,
        key: b"evidence.protect\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"Recordings\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Protect or release recordings from retention\0".as_ptr()
            as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#true != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_RECORDING_DELETE,
        key: b"recording.delete\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"Recordings\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Permanently delete recordings\0".as_ptr() as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#true != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        key: b"camera.configure\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"Camera administration\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Add, change, or remove camera configuration\0".as_ptr()
            as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#true != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_FLEET_EXECUTE_JOB,
        key: b"fleet.execute_job\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"Camera administration\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Execute a bulk fleet operation\0".as_ptr() as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#true != 0,
        enforced: r#false != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_STORAGE_CONFIGURE,
        key: b"storage.configure\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"System administration\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Change storage and retention configuration\0".as_ptr()
            as *const ::core::ffi::c_char,
        camera_scoped: r#false != 0,
        destructive: r#true != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_EVENTS_CONFIGURE,
        key: b"events.configure\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"System administration\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Change event routes and destinations\0".as_ptr()
            as *const ::core::ffi::c_char,
        camera_scoped: r#false != 0,
        destructive: r#true != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_USERS_MANAGE,
        key: b"users.manage\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"System administration\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Manage users, roles, and grants\0".as_ptr() as *const ::core::ffi::c_char,
        camera_scoped: r#false != 0,
        destructive: r#true != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_SYSTEM_ADMIN,
        key: b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"System administration\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Change or control the lightNVR system\0".as_ptr()
            as *const ::core::ffi::c_char,
        camera_scoped: r#false != 0,
        destructive: r#true != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_LPR_READ,
        key: b"lpr.read\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"License plates\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"View protected plate values\0".as_ptr() as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#false != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_LPR_SEARCH,
        key: b"lpr.search\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"License plates\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Search protected plate reads\0".as_ptr() as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#false != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_LPR_EXPORT,
        key: b"lpr.export\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"License plates\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Export protected plate reads\0".as_ptr() as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#false != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_LPR_DELETE,
        key: b"lpr.delete\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"License plates\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Permanently delete protected plate reads\0".as_ptr()
            as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#true != 0,
        enforced: r#true != 0,
    },
    authorization_action_metadata_t {
        action: authorization_action_t::AUTHZ_OBSERVATIONS_ANNOTATE,
        key: b"observations.annotate\0".as_ptr() as *const ::core::ffi::c_char,
        category: b"Integrations\0".as_ptr() as *const ::core::ffi::c_char,
        description: b"Annotate detection observations with classifier results\0".as_ptr()
            as *const ::core::ffi::c_char,
        camera_scoped: r#true != 0,
        destructive: r#false != 0,
        enforced: r#true != 0,
    },
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_action_catalog(
    mut action_count: *mut ::core::ffi::c_int,
) -> *const authorization_action_metadata_t {
    if !action_count.is_null() {
        *action_count = authorization_action_t::AUTHZ_ACTION_COUNT.0;
    }
    return &raw const action_catalog as *const authorization_action_metadata_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_action_metadata(
    mut action: authorization_action_t,
) -> *const authorization_action_metadata_t {
    if action.0 < 0 as ::core::ffi::c_int
        || action.0 >= authorization_action_t::AUTHZ_ACTION_COUNT.0
    {
        return ::core::ptr::null::<authorization_action_metadata_t>();
    }
    return (&raw const action_catalog as *const authorization_action_metadata_t)
        .offset(action.0 as isize);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_action_from_key(
    mut key: *const ::core::ffi::c_char,
) -> authorization_action_t {
    if key.is_null() {
        return authorization_action_t::AUTHZ_ACTION_INVALID;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < authorization_action_t::AUTHZ_ACTION_COUNT.0 {
        if strcmp(action_catalog[i as usize].key, key) == 0 as ::core::ffi::c_int {
            return action_catalog[i as usize].action;
        }
        i += 1;
    }
    return authorization_action_t::AUTHZ_ACTION_INVALID;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_action_bit(mut action: authorization_action_t) -> uint64_t {
    if action.0 < 0 as ::core::ffi::c_int
        || action.0 >= authorization_action_t::AUTHZ_ACTION_COUNT.0
    {
        return 0 as uint64_t;
    }
    return (1 as uint64_t) << action.0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_decision_source_name(
    mut source: authorization_decision_source_t,
) -> *const ::core::ffi::c_char {
    match source {
        authorization_decision_source_t::AUTHZ_SOURCE_LEGACY_ROLE => {
            return b"legacy_role\0".as_ptr() as *const ::core::ffi::c_char;
        }
        authorization_decision_source_t::AUTHZ_SOURCE_POLICY_GRANT => {
            return b"policy_grant\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return b"none\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
unsafe extern "C" fn legacy_role_allows(
    mut role: user_role_t,
    mut action: authorization_action_t,
) -> bool {
    if role.0 == user_role_t::USER_ROLE_ADMIN.0 {
        return r#true != 0;
    }
    match action {
        authorization_action_t::AUTHZ_LIVE_VIEW
        | authorization_action_t::AUTHZ_AUDIO_LISTEN
        | authorization_action_t::AUTHZ_RECORDINGS_REPLAY
        | authorization_action_t::AUTHZ_RECORDINGS_EXPORT
        | authorization_action_t::AUTHZ_SNAPSHOT_CREATE => {
            return role.0 == user_role_t::USER_ROLE_USER.0
                || role.0 == user_role_t::USER_ROLE_VIEWER.0
                || role.0 == user_role_t::USER_ROLE_API.0;
        }
        authorization_action_t::AUTHZ_AUDIO_TALK
        | authorization_action_t::AUTHZ_PTZ_CONTROL
        | authorization_action_t::AUTHZ_EVIDENCE_PROTECT
        | authorization_action_t::AUTHZ_RECORDING_DELETE
        | authorization_action_t::AUTHZ_CAMERA_CONFIGURE => {
            return role.0 == user_role_t::USER_ROLE_USER.0
                || role.0 == user_role_t::USER_ROLE_API.0;
        }
        _ => return r#false != 0,
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_context_create() -> *mut authorization_context_t {
    return calloc(
        1 as size_t,
        ::core::mem::size_of::<authorization_context_t>(),
    ) as *mut authorization_context_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_context_create_on_connection(
    mut connection: *mut sqlite3,
    mut observed_at: int64_t,
) -> *mut authorization_context_t {
    if connection.is_null() || observed_at < 0 as int64_t {
        return ::core::ptr::null_mut::<authorization_context_t>();
    }
    let mut context: *mut authorization_context_t = authorization_context_create();
    if context.is_null() {
        return ::core::ptr::null_mut::<authorization_context_t>();
    }
    (*context).connection = connection;
    (*context).observed_at = observed_at;
    if db_authorization_get_policy_version_on_connection(
        connection,
        &raw mut (*context).legacy_policy_version,
    ) != 0 as ::core::ffi::c_int
    {
        free(context as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<authorization_context_t>();
    }
    (*context).legacy_policy_version_loaded = r#true != 0;
    return context;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_context_free(mut context: *mut authorization_context_t) {
    if context.is_null() {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*context).grant_count {
        let mut entry: *mut grant_cache_entry_t = (*context).grants.offset(i as isize);
        if !(*entry).selectors.is_null() {
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < (*entry).grant_count {
                fleet_selector_free(*(*entry).selectors.offset(j as isize));
                j += 1;
            }
            free((*entry).selectors as *mut ::core::ffi::c_void);
        }
        free((*entry).grants as *mut ::core::ffi::c_void);
        i += 1;
    }
    free((*context).grants as *mut ::core::ffi::c_void);
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*context).collection_count {
        camera_collection_filter_free(
            &raw mut (*(*context).collections.offset(i_0 as isize)).filter,
        );
        i_0 += 1;
    }
    free((*context).collections as *mut ::core::ffi::c_void);
    free(context as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn grow_array(
    mut items: *mut ::core::ffi::c_void,
    mut capacity: *mut ::core::ffi::c_int,
    mut item_size: size_t,
) -> *mut ::core::ffi::c_void {
    let mut next_capacity: ::core::ffi::c_int = if *capacity == 0 as ::core::ffi::c_int {
        4 as ::core::ffi::c_int
    } else {
        *capacity * 2 as ::core::ffi::c_int
    };
    let mut resized: *mut ::core::ffi::c_void =
        realloc(items, (next_capacity as size_t).wrapping_mul(item_size));
    if resized.is_null() {
        return NULL;
    }
    *capacity = next_capacity;
    return resized;
}
unsafe extern "C" fn context_collection_filter(
    mut context: *mut authorization_context_t,
    mut collection_uuid: *const ::core::ffi::c_char,
) -> *const camera_collection_filter_t {
    if collection_uuid.is_null()
        || *collection_uuid.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return ::core::ptr::null::<camera_collection_filter_t>();
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*context).collection_count {
        if strcmp(
            &raw mut (*(*context).collections.offset(i as isize)).uuid as *mut ::core::ffi::c_char,
            collection_uuid,
        ) == 0 as ::core::ffi::c_int
        {
            return if (*(*context).collections.offset(i as isize)).failed as ::core::ffi::c_int != 0
            {
                ::core::ptr::null_mut::<camera_collection_filter_t>()
            } else {
                &raw mut (*(*context).collections.offset(i as isize)).filter
            };
        }
        i += 1;
    }
    if (*context).collection_count == (*context).collection_capacity {
        let mut resized: *mut ::core::ffi::c_void = grow_array(
            (*context).collections as *mut ::core::ffi::c_void,
            &raw mut (*context).collection_capacity,
            ::core::mem::size_of::<collection_cache_entry_t>(),
        );
        if resized.is_null() {
            return ::core::ptr::null::<camera_collection_filter_t>();
        }
        (*context).collections = resized as *mut collection_cache_entry_t;
    }
    let c2rust_fresh0 = (*context).collection_count;
    (*context).collection_count += 1;
    let mut entry: *mut collection_cache_entry_t =
        (*context).collections.offset(c2rust_fresh0 as isize);
    memset(
        entry as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<collection_cache_entry_t>(),
    );
    safe_strcpy(
        &raw mut (*entry).uuid as *mut ::core::ffi::c_char,
        collection_uuid,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    let mut result: camera_collection_filter_result_t =
        camera_collection_filter_result_t(if !(*context).connection.is_null() {
            camera_collection_filter_load_on_connection(
                (*context).connection,
                collection_uuid,
                &raw mut (*entry).filter,
            )
            .0
        } else {
            camera_collection_filter_load_for_authorization(
                collection_uuid,
                &raw mut (*entry).filter,
            )
            .0
        });
    (*entry).failed = result.0 != camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_OK.0;
    return if (*entry).failed as ::core::ffi::c_int != 0 {
        ::core::ptr::null_mut::<camera_collection_filter_t>()
    } else {
        &raw mut (*entry).filter
    };
}
unsafe extern "C" fn parse_selector(
    mut serialized: *const ::core::ffi::c_char,
) -> *mut fleet_selector_t {
    if serialized.is_null()
        || *serialized.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<fleet_selector_t>();
    }
    let mut json: *mut cJSON = cJSON_Parse(serialized);
    if json.is_null() {
        return ::core::ptr::null_mut::<fleet_selector_t>();
    }
    let mut error: [::core::ffi::c_char; 256] = [
        0 as ::core::ffi::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut selector: *mut fleet_selector_t = fleet_selector_parse(
        json,
        &raw mut error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    cJSON_Delete(json);
    return selector;
}
unsafe extern "C" fn context_grants(
    mut context: *mut authorization_context_t,
    mut user_id: int64_t,
    mut action: authorization_action_t,
    mut action_key: *const ::core::ffi::c_char,
) -> *const grant_cache_entry_t {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*context).grant_count {
        if (*(*context).grants.offset(i as isize)).user_id == user_id
            && (*(*context).grants.offset(i as isize)).action.0 == action.0
        {
            return if (*(*context).grants.offset(i as isize)).failed as ::core::ffi::c_int != 0 {
                ::core::ptr::null_mut::<grant_cache_entry_t>()
            } else {
                (*context).grants.offset(i as isize)
            };
        }
        i += 1;
    }
    if (*context).grant_count == (*context).grant_capacity {
        let mut resized: *mut ::core::ffi::c_void = grow_array(
            (*context).grants as *mut ::core::ffi::c_void,
            &raw mut (*context).grant_capacity,
            ::core::mem::size_of::<grant_cache_entry_t>(),
        );
        if resized.is_null() {
            return ::core::ptr::null::<grant_cache_entry_t>();
        }
        (*context).grants = resized as *mut grant_cache_entry_t;
    }
    let c2rust_fresh1 = (*context).grant_count;
    (*context).grant_count += 1;
    let mut entry: *mut grant_cache_entry_t = (*context).grants.offset(c2rust_fresh1 as isize);
    memset(
        entry as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<grant_cache_entry_t>(),
    );
    (*entry).user_id = user_id;
    (*entry).action = action;
    let mut result: ::core::ffi::c_int = if !(*context).connection.is_null() {
        db_authorization_load_user_grants_on_connection(
            (*context).connection,
            user_id,
            action_key,
            &raw mut (*entry).grants,
            &raw mut (*entry).grant_count,
            &raw mut (*entry).policy_version,
        )
    } else {
        db_authorization_load_user_grants(
            user_id,
            action_key,
            &raw mut (*entry).grants,
            &raw mut (*entry).grant_count,
            &raw mut (*entry).policy_version,
        )
    };
    if result != 0 as ::core::ffi::c_int {
        (*entry).failed = r#true != 0;
        (*entry).grants = ::core::ptr::null_mut::<authorization_grant_t>();
        (*entry).grant_count = 0 as ::core::ffi::c_int;
        return ::core::ptr::null::<grant_cache_entry_t>();
    }
    if (*entry).grant_count > 0 as ::core::ffi::c_int {
        (*entry).selectors = calloc(
            (*entry).grant_count as size_t,
            ::core::mem::size_of::<*mut fleet_selector_t>(),
        ) as *mut *mut fleet_selector_t;
        if (*entry).selectors.is_null() {
            (*entry).failed = r#true != 0;
            return ::core::ptr::null::<grant_cache_entry_t>();
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*entry).grant_count {
            if strcmp(
                &raw mut (*(*entry).grants.offset(i_0 as isize)).scope_type
                    as *mut ::core::ffi::c_char,
                b"selector\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                *(*entry).selectors.offset(i_0 as isize) = parse_selector(
                    &raw mut (*(*entry).grants.offset(i_0 as isize)).selector_json
                        as *mut ::core::ffi::c_char,
                );
                if (*(*entry).selectors.offset(i_0 as isize)).is_null() {
                    (*entry).failed = r#true != 0;
                    return ::core::ptr::null::<grant_cache_entry_t>();
                }
            }
            i_0 += 1;
        }
    }
    return entry;
}
unsafe extern "C" fn grant_matches(
    mut context: *mut authorization_context_t,
    mut grant: *const authorization_grant_t,
    mut selector: *mut fleet_selector_t,
    mut camera: *const fleet_camera_t,
    mut matches: *mut bool,
) -> ::core::ffi::c_int {
    *matches = r#false != 0;
    if strcmp(
        &raw const (*grant).scope_type as *const ::core::ffi::c_char,
        b"all\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        *matches = r#true != 0;
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw const (*grant).scope_type as *const ::core::ffi::c_char,
        b"collection\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if camera.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        let mut filter: *const camera_collection_filter_t = context_collection_filter(
            context,
            &raw const (*grant).collection_uuid as *const ::core::ffi::c_char,
        );
        if filter.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        *matches = camera_collection_filter_matches(filter, camera);
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw const (*grant).scope_type as *const ::core::ffi::c_char,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if camera.is_null() || selector.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    *matches = fleet_selector_matches(
        selector,
        camera,
        ::core::ptr::null_mut::<fleet_selector_explanation_t>(),
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn evaluate_legacy(
    mut user: *const user_t,
    mut metadata: *const authorization_action_metadata_t,
    mut camera: *const fleet_camera_t,
    mut evaluation: *mut authorization_evaluation_t,
) -> ::core::ffi::c_int {
    (*evaluation).source = authorization_decision_source_t::AUTHZ_SOURCE_LEGACY_ROLE;
    safe_strcpy(
        &raw mut (*evaluation).role_name as *mut ::core::ffi::c_char,
        db_auth_get_role_name((*user).role),
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    if !legacy_role_allows((*user).role, (*metadata).action) {
        snprintf(
            &raw mut (*evaluation).explanation as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"Legacy %s role does not include %s\0".as_ptr() as *const ::core::ffi::c_char,
            db_auth_get_role_name((*user).role),
            (*metadata).key,
        );
        return 0 as ::core::ffi::c_int;
    }
    (*evaluation).decision = authorization_decision_t::AUTHZ_DECISION_ALLOW;
    snprintf(
        &raw mut (*evaluation).explanation as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"Allowed by compatibility mapping for legacy %s role\0".as_ptr()
            as *const ::core::ffi::c_char,
        db_auth_get_role_name((*user).role),
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn evaluate_principal(
    mut context: *mut authorization_context_t,
    mut user: *const user_t,
    mut action: authorization_action_t,
    mut camera: *const fleet_camera_t,
    mut evaluation: *mut authorization_evaluation_t,
) -> ::core::ffi::c_int {
    if user.is_null() || evaluation.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut metadata: *const authorization_action_metadata_t =
        authorization_action_metadata(action);
    if metadata.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        evaluation as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<authorization_evaluation_t>(),
    );
    if !(*user).is_active {
        safe_strcpy(
            &raw mut (*evaluation).explanation as *mut ::core::ffi::c_char,
            b"User is inactive\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw const (*user).authorization_mode as *const ::core::ffi::c_char,
        b"policy\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        if !(*context).legacy_policy_version_loaded {
            db_authorization_get_policy_version_readonly(&raw mut (*context).legacy_policy_version);
            (*context).legacy_policy_version_loaded = r#true != 0;
        }
        (*evaluation).policy_version = (*context).legacy_policy_version;
        return evaluate_legacy(user, metadata, camera, evaluation);
    }
    let mut entry: *const grant_cache_entry_t =
        context_grants(context, (*user).id, action, (*metadata).key);
    if entry.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    (*evaluation).policy_version = (*entry).policy_version;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*entry).grant_count {
        let mut matches: bool = r#false != 0;
        if grant_matches(
            context,
            (*entry).grants.offset(i as isize),
            if !(*entry).selectors.is_null() {
                *(*entry).selectors.offset(i as isize)
            } else {
                ::core::ptr::null_mut::<fleet_selector_t>()
            },
            camera,
            &raw mut matches,
        ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if !matches {
            i += 1;
        } else {
            (*evaluation).decision = authorization_decision_t::AUTHZ_DECISION_ALLOW;
            (*evaluation).source = authorization_decision_source_t::AUTHZ_SOURCE_POLICY_GRANT;
            safe_strcpy(
                &raw mut (*evaluation).grant_uuid as *mut ::core::ffi::c_char,
                &raw mut (*(*entry).grants.offset(i as isize)).uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*evaluation).role_uuid as *mut ::core::ffi::c_char,
                &raw mut (*(*entry).grants.offset(i as isize)).role_uuid
                    as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*evaluation).role_name as *mut ::core::ffi::c_char,
                &raw mut (*(*entry).grants.offset(i as isize)).role_name
                    as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                0 as size_t,
            );
            snprintf(
                &raw mut (*evaluation).explanation as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                b"Allowed by %s role grant\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*(*entry).grants.offset(i as isize)).role_name
                    as *mut ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
    }
    safe_strcpy(
        &raw mut (*evaluation).explanation as *mut ::core::ffi::c_char,
        if (*metadata).camera_scoped as ::core::ffi::c_int != 0 && camera.is_null() {
            b"No all-fleet grant allows this action\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"No matching grant allows this action\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn token_scope_matches(
    mut context: *mut authorization_context_t,
    mut token: *const api_token_t,
    mut camera: *const fleet_camera_t,
    mut matches: *mut bool,
) -> ::core::ffi::c_int {
    *matches = r#false != 0;
    if strcmp(
        &raw const (*token).scope_type as *const ::core::ffi::c_char,
        b"all\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        *matches = r#true != 0;
        return 0 as ::core::ffi::c_int;
    }
    if camera.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw const (*token).scope_type as *const ::core::ffi::c_char,
        b"collection\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut filter: *const camera_collection_filter_t = context_collection_filter(
            context,
            &raw const (*token).collection_uuid as *const ::core::ffi::c_char,
        );
        if filter.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        *matches = camera_collection_filter_matches(filter, camera);
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw const (*token).scope_type as *const ::core::ffi::c_char,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut selector: *mut fleet_selector_t =
        parse_selector(&raw const (*token).selector_json as *const ::core::ffi::c_char);
    if selector.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *matches = fleet_selector_matches(
        selector,
        camera,
        ::core::ptr::null_mut::<fleet_selector_explanation_t>(),
    );
    fleet_selector_free(selector);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn context_token(
    mut context: *mut authorization_context_t,
    mut user: *const user_t,
) -> *const api_token_t {
    if (*context).token_loaded {
        return if (*context).token_valid as ::core::ffi::c_int != 0 {
            &raw mut (*context).token
        } else {
            ::core::ptr::null_mut::<api_token_t>()
        };
    }
    (*context).token_loaded = r#true != 0;
    let mut result: db_api_token_result_t =
        db_api_token_result_t(if !(*context).connection.is_null() {
            db_api_token_get_active_on_connection(
                (*context).connection,
                (*context).observed_at,
                &raw const (*user).api_token_uuid as *const ::core::ffi::c_char,
                &raw mut (*context).token,
            )
            .0
        } else {
            db_api_token_get_active(
                &raw const (*user).api_token_uuid as *const ::core::ffi::c_char,
                &raw mut (*context).token,
            )
            .0
        });
    (*context).token_failed = result.0 == db_api_token_result_t::DB_API_TOKEN_ERROR.0;
    (*context).token_valid = result.0 == db_api_token_result_t::DB_API_TOKEN_OK.0
        && (*context).token.user_id == (*user).id;
    return if (*context).token_valid as ::core::ffi::c_int != 0 {
        &raw mut (*context).token
    } else {
        ::core::ptr::null_mut::<api_token_t>()
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_evaluate_in_context(
    mut context: *mut authorization_context_t,
    mut user: *const user_t,
    mut action: authorization_action_t,
    mut camera: *const fleet_camera_t,
    mut evaluation: *mut authorization_evaluation_t,
) -> ::core::ffi::c_int {
    let mut owned: *mut authorization_context_t =
        ::core::ptr::null_mut::<authorization_context_t>();
    if context.is_null() {
        owned = authorization_context_create();
        if owned.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        context = owned;
    }
    let mut result: ::core::ffi::c_int =
        evaluate_principal(context, user, action, camera, evaluation);
    if result != 0 as ::core::ffi::c_int
        || user.is_null()
        || evaluation.is_null()
        || (*evaluation).decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0
        || !(*user).authenticated_via_scoped_token
    {
        authorization_context_free(owned);
        return result;
    }
    let mut token: *const api_token_t = context_token(context, user);
    if token.is_null() {
        if !(*context).connection.is_null() && (*context).token_failed as ::core::ffi::c_int != 0 {
            authorization_context_free(owned);
            return -1 as ::core::ffi::c_int;
        }
        (*evaluation).decision = authorization_decision_t::AUTHZ_DECISION_DENY;
        (*evaluation).source = authorization_decision_source_t::AUTHZ_SOURCE_NONE;
        safe_strcpy(
            &raw mut (*evaluation).explanation as *mut ::core::ffi::c_char,
            b"Scoped API token is expired, revoked, or unavailable\0".as_ptr()
                as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        authorization_context_free(owned);
        return 0 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut (*evaluation).token_uuid as *mut ::core::ffi::c_char,
        &raw const (*token).uuid as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    if (*token).action_mask & authorization_action_bit(action) == 0 as uint64_t {
        (*evaluation).decision = authorization_decision_t::AUTHZ_DECISION_DENY;
        (*evaluation).source = authorization_decision_source_t::AUTHZ_SOURCE_NONE;
        safe_strcpy(
            &raw mut (*evaluation).explanation as *mut ::core::ffi::c_char,
            b"Scoped API token does not include this action\0".as_ptr()
                as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        authorization_context_free(owned);
        return 0 as ::core::ffi::c_int;
    }
    let mut matches: bool = r#false != 0;
    if token_scope_matches(context, token, camera, &raw mut matches) != 0 as ::core::ffi::c_int {
        authorization_context_free(owned);
        return -1 as ::core::ffi::c_int;
    }
    if !matches {
        (*evaluation).decision = authorization_decision_t::AUTHZ_DECISION_DENY;
        (*evaluation).source = authorization_decision_source_t::AUTHZ_SOURCE_NONE;
        safe_strcpy(
            &raw mut (*evaluation).explanation as *mut ::core::ffi::c_char,
            if !camera.is_null() {
                b"Camera is outside the scoped API token\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"Scoped API token is not valid for global actions\0".as_ptr()
                    as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        authorization_context_free(owned);
        return 0 as ::core::ffi::c_int;
    }
    safe_strcat(
        &raw mut (*evaluation).explanation as *mut ::core::ffi::c_char,
        b"; narrowed by scoped API token\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    authorization_context_free(owned);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_evaluate(
    mut user: *const user_t,
    mut action: authorization_action_t,
    mut camera: *const fleet_camera_t,
    mut evaluation: *mut authorization_evaluation_t,
) -> ::core::ffi::c_int {
    return authorization_evaluate_in_context(
        ::core::ptr::null_mut::<authorization_context_t>(),
        user,
        action,
        camera,
        evaluation,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_filter_cameras(
    mut user: *const user_t,
    mut action: authorization_action_t,
    mut cameras: *mut fleet_camera_t,
    mut count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if user.is_null() || count.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut metadata: *const authorization_action_metadata_t =
        authorization_action_metadata(action);
    if metadata.is_null() || !(*metadata).camera_scoped {
        return -1 as ::core::ffi::c_int;
    }
    if cameras.is_null() || *count <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if !g_config.web_auth_enabled {
        return 0 as ::core::ffi::c_int;
    }
    let mut context: *mut authorization_context_t = authorization_context_create();
    if context.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut visible: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < *count {
        let mut evaluation: authorization_evaluation_t = authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        };
        if authorization_evaluate_in_context(
            context,
            user,
            action,
            cameras.offset(i as isize),
            &raw mut evaluation,
        ) != 0 as ::core::ffi::c_int
        {
            authorization_context_free(context);
            return -1 as ::core::ffi::c_int;
        }
        if evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0 {
            if visible != i {
                *cameras.offset(visible as isize) = *cameras.offset(i as isize);
            }
            visible += 1;
        }
        i += 1;
    }
    authorization_context_free(context);
    *count = visible;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_filter_visible_cameras(
    mut user: *const user_t,
    mut cameras: *mut fleet_camera_t,
    mut count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return authorization_filter_cameras(
        user,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        cameras,
        count,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn authorization_effective_action_mask(
    mut user: *const user_t,
    mut mask: *mut uint64_t,
) -> ::core::ffi::c_int {
    if user.is_null() || mask.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *mask = 0 as uint64_t;
    if !(*user).is_active {
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw const (*user).authorization_mode as *const ::core::ffi::c_char,
        b"policy\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < authorization_action_t::AUTHZ_ACTION_COUNT.0 {
            if legacy_role_allows((*user).role, action_catalog[i as usize].action) {
                *mask |= authorization_action_bit(action_catalog[i as usize].action);
            }
            i += 1;
        }
    } else {
        let mut mode: [::core::ffi::c_char; 16] = [0; 16];
        let mut grants: *mut authorization_grant_t =
            ::core::ptr::null_mut::<authorization_grant_t>();
        let mut grant_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut policy_version: int64_t = 0 as int64_t;
        if db_authorization_get_user_policy(
            (*user).id,
            &raw mut mode as *mut ::core::ffi::c_char,
            &raw mut grants,
            &raw mut grant_count,
            &raw mut policy_version,
        )
        .0 != db_authorization_result_t::DB_AUTHORIZATION_OK.0
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < grant_count {
            if (*grants.offset(i_0 as isize)).enabled {
                let mut role: authorization_role_t = authorization_role_t {
                    uuid: [0; 37],
                    name: [0; 128],
                    description: [0; 256],
                    is_builtin: false,
                    action_mask: 0,
                    created_at: 0,
                    updated_at: 0,
                };
                if db_authorization_role_get(
                    &raw mut (*grants.offset(i_0 as isize)).role_uuid as *mut ::core::ffi::c_char,
                    &raw mut role,
                )
                .0 != db_authorization_result_t::DB_AUTHORIZATION_OK.0
                {
                    free(grants as *mut ::core::ffi::c_void);
                    return -1 as ::core::ffi::c_int;
                }
                *mask |= role.action_mask;
            }
            i_0 += 1;
        }
        free(grants as *mut ::core::ffi::c_void);
    }
    if (*user).authenticated_via_scoped_token {
        let mut token: api_token_t = api_token_t {
            uuid: [0; 37],
            user_id: 0,
            created_by_user_id: 0,
            description: [0; 128],
            token_prefix: [0; 16],
            action_mask: 0,
            scope_type: [0; 16],
            selector_json: [0; 8192],
            collection_uuid: [0; 37],
            expires_at: 0,
            revoked_at: 0,
            last_used_at: 0,
            created_at: 0,
        };
        let mut result: db_api_token_result_t = db_api_token_get_active(
            &raw const (*user).api_token_uuid as *const ::core::ffi::c_char,
            &raw mut token,
        );
        if result.0 == db_api_token_result_t::DB_API_TOKEN_ERROR.0 {
            return -1 as ::core::ffi::c_int;
        }
        *mask = if result.0 == db_api_token_result_t::DB_API_TOKEN_OK.0
            && token.user_id == (*user).id
        {
            *mask & token.action_mask
        } else {
            0 as uint64_t
        };
    }
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
