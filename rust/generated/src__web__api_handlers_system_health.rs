unsafe extern "C" {
    unsafe fn system_health_scope_name(scope: system_health_scope_t) -> *const ::core::ffi::c_char;
    unsafe fn system_health_capability_name(
        capability: system_health_capability_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn system_health_unit_name(unit: system_health_unit_t) -> *const ::core::ffi::c_char;
    unsafe fn system_health_severity_name(
        severity: system_health_severity_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn system_health_condition_code(
        condition: system_health_condition_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn system_health_policy_snapshot(
        snapshot: *mut system_health_policy_t,
    ) -> ::core::ffi::c_int;
    static mut g_config: config_t;
    unsafe fn http_request_get_query_param(
        req: *const http_request_t,
        name: *const ::core::ffi::c_char,
        value: *mut ::core::ffi::c_char,
        value_len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_set_json(
        res: *mut http_response_t,
        status_code: ::core::ffi::c_int,
        json_str: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_set_json_error(
        res: *mut http_response_t,
        status_code: ::core::ffi::c_int,
        error_message: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn cJSON_ParseWithOpts(
        value: *const ::core::ffi::c_char,
        return_parse_end: *mut *const ::core::ffi::c_char,
        require_null_terminated: cJSON_bool,
    ) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateArray() -> *mut cJSON;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_AddItemToObject(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        item: *mut cJSON,
    ) -> cJSON_bool;
    unsafe fn cJSON_AddNullToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddBoolToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        boolean: cJSON_bool,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddNumberToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        number: ::core::ffi::c_double,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddStringToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddObjectToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_free(object: *mut ::core::ffi::c_void);
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn event_bus_get_stats(stats: *mut event_bus_stats_t);
    unsafe fn db_event_destination_list(
        destinations: *mut event_destination_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_destination_make_key(
        uuid: *const ::core::ffi::c_char,
        key: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn event_router_get_stats(stats: *mut event_router_stats_t);
    unsafe fn db_event_outbox_get_stats(
        destination: *const ::core::ffi::c_char,
        now: int64_t,
        stats: *mut event_outbox_stats_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mqtt_delivery_worker_get_stats(stats: *mut mqtt_delivery_worker_stats_t);
    unsafe fn mqtt_destination_client_list_stats(
        stats: *mut mqtt_destination_client_stats_t,
        capacity: size_t,
    ) -> size_t;
    unsafe fn mqtt_presence_get_stats(stats: *mut mqtt_presence_stats_t);
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn db_system_health_incident_list(
        include_closed: bool,
        cursor: *const system_health_incident_cursor_t,
        incidents: *mut system_health_incident_record_t,
        max_count: ::core::ffi::c_int,
        next_cursor: *mut system_health_incident_cursor_t,
    ) -> ::core::ffi::c_int;
    unsafe fn system_health_snapshot_copy(snapshot: *mut system_health_snapshot_t) -> bool;
    unsafe fn system_health_summary_copy(
        summaries: *mut system_health_summary_t,
        capacity: size_t,
    ) -> size_t;
    unsafe fn system_health_get_stats(stats: *mut system_health_stats_t);
    unsafe fn system_health_collector_stats_copy(
        stats: *mut system_health_collector_stats_t,
        capacity: size_t,
    ) -> size_t;
    unsafe fn system_health_evaluator_service_active_copy(
        incidents: *mut system_health_incident_view_t,
        capacity: size_t,
    ) -> size_t;
    unsafe fn system_health_evaluator_service_get_stats(
        stats: *mut system_health_evaluator_stats_t,
    );
    unsafe fn httpd_authorize_global_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
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
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_scope_t(pub ::core::ffi::c_uint);
impl system_health_scope_t {
    pub const SYSTEM_HEALTH_SCOPE_PROCESS: Self = Self(0);
    pub const SYSTEM_HEALTH_SCOPE_CONTAINER: Self = Self(1);
    pub const SYSTEM_HEALTH_SCOPE_HOST: Self = Self(2);
    pub const SYSTEM_HEALTH_SCOPE_FILESYSTEM: Self = Self(3);
    pub const SYSTEM_HEALTH_SCOPE_DEVICE: Self = Self(4);
    pub const SYSTEM_HEALTH_SCOPE_COUNT: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_capability_t(pub ::core::ffi::c_uint);
impl system_health_capability_t {
    pub const SYSTEM_HEALTH_CAPABILITY_AVAILABLE: Self = Self(0);
    pub const SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED: Self = Self(1);
    pub const SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED: Self = Self(2);
    pub const SYSTEM_HEALTH_CAPABILITY_STALE: Self = Self(3);
    pub const SYSTEM_HEALTH_CAPABILITY_ERROR: Self = Self(4);
    pub const SYSTEM_HEALTH_CAPABILITY_COUNT: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_freshness_t(pub ::core::ffi::c_uint);
impl system_health_freshness_t {
    pub const SYSTEM_HEALTH_FRESHNESS_UNKNOWN: Self = Self(0);
    pub const SYSTEM_HEALTH_FRESHNESS_FRESH: Self = Self(1);
    pub const SYSTEM_HEALTH_FRESHNESS_STALE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_unit_t(pub ::core::ffi::c_uint);
impl system_health_unit_t {
    pub const SYSTEM_HEALTH_UNIT_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_UNIT_RATIO: Self = Self(1);
    pub const SYSTEM_HEALTH_UNIT_BYTES: Self = Self(2);
    pub const SYSTEM_HEALTH_UNIT_COUNT: Self = Self(3);
    pub const SYSTEM_HEALTH_UNIT_SECONDS: Self = Self(4);
    pub const SYSTEM_HEALTH_UNIT_CELSIUS: Self = Self(5);
    pub const SYSTEM_HEALTH_UNIT_HERTZ: Self = Self(6);
    pub const SYSTEM_HEALTH_UNIT_BOOLEAN: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_severity_t(pub ::core::ffi::c_uint);
impl system_health_severity_t {
    pub const SYSTEM_HEALTH_SEVERITY_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_SEVERITY_WARNING: Self = Self(1);
    pub const SYSTEM_HEALTH_SEVERITY_ERROR: Self = Self(2);
    pub const SYSTEM_HEALTH_SEVERITY_CRITICAL: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_state_t(pub ::core::ffi::c_uint);
impl system_health_state_t {
    pub const SYSTEM_HEALTH_STATE_UNKNOWN: Self = Self(0);
    pub const SYSTEM_HEALTH_STATE_HEALTHY: Self = Self(1);
    pub const SYSTEM_HEALTH_STATE_PENDING: Self = Self(2);
    pub const SYSTEM_HEALTH_STATE_OPEN: Self = Self(3);
    pub const SYSTEM_HEALTH_STATE_RECOVERING: Self = Self(4);
    pub const SYSTEM_HEALTH_STATE_CLOSED: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_condition_t(pub ::core::ffi::c_uint);
impl system_health_condition_t {
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW: Self = Self(0);
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_OOM_KILL: Self = Self(1);
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_SWAP_THRASH: Self = Self(2);
    pub const SYSTEM_HEALTH_CONDITION_CPU_SATURATION: Self = Self(3);
    pub const SYSTEM_HEALTH_CONDITION_CPU_THROTTLED: Self = Self(4);
    pub const SYSTEM_HEALTH_CONDITION_IO_PRESSURE: Self = Self(5);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_BYTES_LOW: Self = Self(6);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_INODES_LOW: Self = Self(7);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY: Self = Self(8);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED: Self = Self(9);
    pub const SYSTEM_HEALTH_CONDITION_THERMAL_HIGH: Self = Self(10);
    pub const SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN: Self = Self(11);
    pub const SYSTEM_HEALTH_CONDITION_NETWORK_ERROR_RATE: Self = Self(12);
    pub const SYSTEM_HEALTH_CONDITION_CLOCK_UNSYNCHRONIZED: Self = Self(13);
    pub const SYSTEM_HEALTH_CONDITION_CLOCK_JUMP: Self = Self(14);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION: Self = Self(15);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_PID_EXHAUSTION: Self = Self(16);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_ALLOCATION_FAILED: Self = Self(17);
    pub const SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL: Self = Self(18);
    pub const SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL: Self = Self(19);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_CORRECTED: Self = Self(20);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_UNCORRECTABLE: Self = Self(21);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_FAN_FAILED: Self = Self(22);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_POWER_UNSTABLE: Self = Self(23);
    pub const SYSTEM_HEALTH_CONDITION_COLLECTOR_STALE: Self = Self(24);
    pub const SYSTEM_HEALTH_CONDITION_UNEXPECTED_RESTART: Self = Self(25);
    pub const SYSTEM_HEALTH_CONDITION_EVENT_DELIVERY_DEGRADED: Self = Self(26);
    pub const SYSTEM_HEALTH_CONDITION_COUNT: Self = Self(27);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_observation_t {
    pub metric: [::core::ffi::c_char; 64],
    pub resource_id: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub capability: system_health_capability_t,
    pub freshness: system_health_freshness_t,
    pub unit: system_health_unit_t,
    pub value_valid: bool,
    pub value: ::core::ffi::c_double,
    pub sampled_monotonic_ms: uint64_t,
    pub observed_wall_time_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_snapshot_t {
    pub observations: [system_health_observation_t; 256],
    pub observation_count: size_t,
    pub observations_dropped: size_t,
    pub sequence: uint64_t,
    pub completed_monotonic_ms: uint64_t,
    pub completed_wall_time_ms: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_profile_t(pub ::core::ffi::c_uint);
impl system_health_profile_t {
    pub const SYSTEM_HEALTH_PROFILE_BALANCED: Self = Self(0);
    pub const SYSTEM_HEALTH_PROFILE_CONSERVATIVE: Self = Self(1);
    pub const SYSTEM_HEALTH_PROFILE_DISABLED: Self = Self(2);
    pub const SYSTEM_HEALTH_PROFILE_CUSTOM: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_threshold_direction_t(pub ::core::ffi::c_uint);
impl system_health_threshold_direction_t {
    pub const SYSTEM_HEALTH_THRESHOLD_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE: Self = Self(1);
    pub const SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE: Self = Self(2);
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_condition_policy_t {
    pub condition: system_health_condition_t,
    pub profile: system_health_profile_t,
    pub enabled: bool,
    pub overridden: bool,
    pub direction: system_health_threshold_direction_t,
    pub unit: system_health_unit_t,
    pub warning_threshold: ::core::ffi::c_double,
    pub critical_threshold: ::core::ffi::c_double,
    pub recovery_threshold: ::core::ffi::c_double,
    pub warning_for_seconds: uint32_t,
    pub critical_for_seconds: uint32_t,
    pub recovery_for_seconds: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_policy_t {
    pub generation: uint64_t,
    pub settings: system_health_policy_settings_t,
    pub conditions: [system_health_condition_policy_t; 27],
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
pub struct http_method_t(pub ::core::ffi::c_uint);
impl http_method_t {
    pub const HTTP_METHOD_GET: Self = Self(0);
    pub const HTTP_METHOD_POST: Self = Self(1);
    pub const HTTP_METHOD_PUT: Self = Self(2);
    pub const HTTP_METHOD_DELETE: Self = Self(3);
    pub const HTTP_METHOD_OPTIONS: Self = Self(4);
    pub const HTTP_METHOD_HEAD: Self = Self(5);
    pub const HTTP_METHOD_PATCH: Self = Self(6);
    pub const HTTP_METHOD_UNKNOWN: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_header_t {
    pub name: [::core::ffi::c_char; 128],
    pub value: [::core::ffi::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_request_t {
    pub method: http_method_t,
    pub method_str: [::core::ffi::c_char; 16],
    pub path: [::core::ffi::c_char; 512],
    pub uri: [::core::ffi::c_char; 1536],
    pub query_string: [::core::ffi::c_char; 1024],
    pub content_type: [::core::ffi::c_char; 128],
    pub content_length: uint64_t,
    pub user_agent: [::core::ffi::c_char; 256],
    pub body: *mut ::core::ffi::c_void,
    pub body_len: size_t,
    pub headers: [http_header_t; 50],
    pub num_headers: ::core::ffi::c_int,
    pub client_ip: [::core::ffi::c_char; 64],
    pub request_id: [::core::ffi::c_char; 65],
    pub user_data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_response_t {
    pub status_code: ::core::ffi::c_int,
    pub content_type: [::core::ffi::c_char; 128],
    pub body_length: size_t,
    pub body: *mut ::core::ffi::c_void,
    pub body_allocated: bool,
    pub headers: [http_header_t; 50],
    pub num_headers: ::core::ffi::c_int,
    pub user_data: *mut ::core::ffi::c_void,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_incident_view_t {
    pub incident_id: [::core::ffi::c_char; 37],
    pub condition: system_health_condition_t,
    pub subject: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub state: system_health_state_t,
    pub severity: system_health_severity_t,
    pub first_observed_at_ms: int64_t,
    pub last_observed_at_ms: int64_t,
    pub observation: system_health_observation_t,
    pub persistence_pending: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_evaluator_stats_t {
    pub tracked_conditions: size_t,
    pub active_incidents: size_t,
    pub pending_persistence: size_t,
    pub transitions: uint64_t,
    pub persistence_failures: uint64_t,
    pub persistence_retries: uint64_t,
    pub oldest_pending_wall_time_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_stats_t {
    pub initialized: bool,
    pub enabled: bool,
    pub ring_allocated: bool,
    pub worker_threads: uint32_t,
    pub generations_completed: uint64_t,
    pub collections_completed: uint64_t,
    pub collection_errors: uint64_t,
    pub collection_timeouts: uint64_t,
    pub overlap_skips: uint64_t,
    pub observations_dropped: uint64_t,
    pub coverage_overflows: uint64_t,
    pub abandoned_helpers: uint32_t,
}
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mqtt_presence_stats_t {
    pub configured: bool,
    pub connected: bool,
    pub sequence: uint64_t,
    pub connections: uint64_t,
    pub reconnects: uint64_t,
    pub disconnects: uint64_t,
    pub publish_attempts: uint64_t,
    pub publish_successes: uint64_t,
    pub publish_failures: uint64_t,
    pub last_publish_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_outbox_stats_t {
    pub total_rows: int64_t,
    pub total_bytes: int64_t,
    pub pending_rows: int64_t,
    pub delivering_rows: int64_t,
    pub delivered_rows: int64_t,
    pub dead_rows: int64_t,
    pub due_rows: int64_t,
    pub oldest_pending_at: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mqtt_destination_client_stats_t {
    pub destination_uuid: [::core::ffi::c_char; 37],
    pub presence_configured: bool,
    pub connected: bool,
    pub connections: uint64_t,
    pub reconnects: uint64_t,
    pub disconnects: uint64_t,
    pub connection_failures: uint64_t,
    pub publish_successes: uint64_t,
    pub publish_failures: uint64_t,
    pub last_success_at_ms: int64_t,
    pub last_failure_at_ms: int64_t,
    pub last_failure: mqtt_destination_failure_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mqtt_destination_failure_t(pub ::core::ffi::c_uint);
impl mqtt_destination_failure_t {
    pub const MQTT_DESTINATION_FAILURE_NONE: Self = Self(0);
    pub const MQTT_DESTINATION_FAILURE_CONFIGURATION: Self = Self(1);
    pub const MQTT_DESTINATION_FAILURE_CONNECTION: Self = Self(2);
    pub const MQTT_DESTINATION_FAILURE_PUBLICATION: Self = Self(3);
    pub const MQTT_DESTINATION_FAILURE_COUNT: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_destination_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub description: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub destination_type: [::core::ffi::c_char; 16],
    pub broker_host: [::core::ffi::c_char; 256],
    pub broker_port: ::core::ffi::c_int,
    pub client_id: [::core::ffi::c_char; 128],
    pub topic_template: [::core::ffi::c_char; 512],
    pub status_topic_template: [::core::ffi::c_char; 512],
    pub username: [::core::ffi::c_char; 128],
    pub password_configured: bool,
    pub tls_mode: [::core::ffi::c_char; 16],
    pub ca_file: [::core::ffi::c_char; 512],
    pub cert_file: [::core::ffi::c_char; 512],
    pub key_file: [::core::ffi::c_char; 512],
    pub keepalive_seconds: ::core::ffi::c_int,
    pub qos: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mqtt_delivery_worker_stats_t {
    pub enqueued: uint64_t,
    pub duplicates: uint64_t,
    pub rejected_full: uint64_t,
    pub enqueue_errors: uint64_t,
    pub priority_shed: uint64_t,
    pub expired: uint64_t,
    pub attempted: uint64_t,
    pub delivered: uint64_t,
    pub retried: uint64_t,
    pub dead: uint64_t,
    pub outcome_errors: uint64_t,
    pub disconnected_polls: uint64_t,
    pub managed_profiles: uint64_t,
    pub managed_connected: uint64_t,
    pub profile_reloads: uint64_t,
    pub profile_errors: uint64_t,
    pub pruned: uint64_t,
    pub running: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_router_stats_t {
    pub cache_reloads: uint64_t,
    pub events_evaluated: uint64_t,
    pub default_events: uint64_t,
    pub matched_events: uint64_t,
    pub unmatched_events: uint64_t,
    pub evaluation_errors: uint64_t,
    pub routes_considered: uint64_t,
    pub type_rejections: uint64_t,
    pub scope_rejections: uint64_t,
    pub predicate_rejections: uint64_t,
    pub schedule_rejections: uint64_t,
    pub destination_disabled_rejections: uint64_t,
    pub debounce_suppressions: uint64_t,
    pub cooldown_suppressions: uint64_t,
    pub grouping_suppressions: uint64_t,
    pub rate_suppressions: uint64_t,
    pub suppression_errors: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_bus_stats_t {
    pub accepted_events: uint64_t,
    pub dispatched_events: uint64_t,
    pub callback_deliveries: uint64_t,
    pub handler_failures: uint64_t,
    pub dropped_events: uint64_t,
    pub priority_shed_events: uint64_t,
    pub rejected_events: uint64_t,
    pub queued_events: size_t,
    pub queued_bytes: size_t,
    pub active_dispatches: size_t,
    pub max_events: size_t,
    pub max_bytes: size_t,
    pub subscriber_count: ::core::ffi::c_int,
    pub running: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_collector_stats_t {
    pub name: [::core::ffi::c_char; 32],
    pub scope: system_health_scope_t,
    pub tier: system_health_sampling_tier_t,
    pub interval_seconds: uint32_t,
    pub stale_after_seconds: uint32_t,
    pub attempts: uint64_t,
    pub completions: uint64_t,
    pub failures: uint64_t,
    pub timeouts: uint64_t,
    pub overlap_skips: uint64_t,
    pub last_duration_ms: uint64_t,
    pub maximum_duration_ms: uint64_t,
    pub last_attempt_monotonic_ms: uint64_t,
    pub last_success_monotonic_ms: uint64_t,
    pub busy: bool,
    pub stale: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_sampling_tier_t(pub ::core::ffi::c_uint);
impl system_health_sampling_tier_t {
    pub const SYSTEM_HEALTH_TIER_FAST: Self = Self(0);
    pub const SYSTEM_HEALTH_TIER_NORMAL: Self = Self(1);
    pub const SYSTEM_HEALTH_TIER_SLOW: Self = Self(2);
    pub const SYSTEM_HEALTH_TIER_DEVICE: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_summary_t {
    pub sequence: uint64_t,
    pub completed_monotonic_ms: uint64_t,
    pub completed_wall_time_ms: int64_t,
    pub observation_count: uint32_t,
    pub observations_dropped: uint32_t,
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
pub struct system_health_incident_cursor_t {
    pub valid: bool,
    pub last_seen_at_ms: int64_t,
    pub uuid: [::core::ffi::c_char; 37],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_incident_record_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub condition_code: [::core::ffi::c_char; 64],
    pub subject: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub state: system_health_state_t,
    pub severity: system_health_severity_t,
    pub first_seen_at_ms: int64_t,
    pub last_seen_at_ms: int64_t,
    pub closed_at_ms: int64_t,
    pub observation_json: [::core::ffi::c_char; 2049],
    pub alert_event_id: [::core::ffi::c_char; 37],
    pub recovery_event_id: [::core::ffi::c_char; 37],
    pub reconciliation: system_health_reconciliation_t,
    pub boot_id: [::core::ffi::c_char; 64],
    pub run_id: [::core::ffi::c_char; 37],
    pub revision: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_reconciliation_t(pub ::core::ffi::c_uint);
impl system_health_reconciliation_t {
    pub const SYSTEM_HEALTH_RECONCILIATION_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_RECONCILIATION_ALERT_PENDING: Self = Self(1);
    pub const SYSTEM_HEALTH_RECONCILIATION_RECOVERY_PENDING: Self = Self(2);
    pub const SYSTEM_HEALTH_RECONCILIATION_RECONCILED: Self = Self(3);
    pub const SYSTEM_HEALTH_RECONCILIATION_DELIVERY_FAILED: Self = Self(4);
}
pub const SYSTEM_HEALTH_MAX_INCIDENTS: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_RING_SAMPLES: ::core::ffi::c_uint = 120 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_MAX_COLLECTORS: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_METRIC_LENGTH: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EVENT_DESTINATION_MAX_COUNT: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const EVENT_OUTBOX_DEFAULT_MAX_ROWS: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const EVENT_OUTBOX_DEFAULT_MAX_BYTES: ::core::ffi::c_longlong = 64 as ::core::ffi::c_longlong
    * 1024 as ::core::ffi::c_longlong
    * 1024 as ::core::ffi::c_longlong;
pub const MQTT_EVENT_OUTBOX_DESTINATION: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"mqtt:default\0") };
pub const SYSTEM_HEALTH_INCIDENT_PAGE_MAX: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const SYSTEM_HEALTH_API_SCHEMA_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SYSTEM_HEALTH_INCIDENT_DEFAULT_LIMIT: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
unsafe extern "C" fn freshness_name(
    mut freshness: system_health_freshness_t,
) -> *const ::core::ffi::c_char {
    match freshness {
        system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_FRESH => {
            return b"fresh\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_STALE => {
            return b"stale\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_UNKNOWN => {
            return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn state_name(mut state: system_health_state_t) -> *const ::core::ffi::c_char {
    match state {
        system_health_state_t::SYSTEM_HEALTH_STATE_HEALTHY => {
            return b"healthy\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_state_t::SYSTEM_HEALTH_STATE_PENDING => {
            return b"pending\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_state_t::SYSTEM_HEALTH_STATE_OPEN => {
            return b"open\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_state_t::SYSTEM_HEALTH_STATE_RECOVERING => {
            return b"recovering\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_state_t::SYSTEM_HEALTH_STATE_CLOSED => {
            return b"closed\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN => {
            return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn direction_name(
    mut direction: system_health_threshold_direction_t,
) -> *const ::core::ffi::c_char {
    match direction {
        system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE => {
            return b"lower_is_worse\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE => {
            return b"higher_is_worse\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE => {
            return b"none\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"none\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn reconciliation_name(
    mut state: system_health_reconciliation_t,
) -> *const ::core::ffi::c_char {
    match state {
        system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_NONE => {
            return b"none\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_ALERT_PENDING => {
            return b"alert_pending\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_RECOVERY_PENDING => {
            return b"recovery_pending\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_RECONCILED => {
            return b"reconciled\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_reconciliation_t::SYSTEM_HEALTH_RECONCILIATION_DELIVERY_FAILED => {
            return b"delivery_failed\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn monotonic_ms() -> uint64_t {
    let mut now: timespec = timespec {
        tv_sec: 0 as __time_t,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_MONOTONIC, &raw mut now) != 0 as ::core::ffi::c_int {
        return 0 as uint64_t;
    }
    return (now.tv_sec as uint64_t)
        .wrapping_mul(1000 as uint64_t)
        .wrapping_add((now.tv_nsec as uint64_t).wrapping_div(1000000 as uint64_t));
}
unsafe extern "C" fn wall_time_seconds() -> int64_t {
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    return if now > 0 as time_t {
        now as int64_t
    } else {
        0 as int64_t
    };
}
unsafe extern "C" fn destination_failure_name(
    mut failure: mqtt_destination_failure_t,
) -> *const ::core::ffi::c_char {
    match failure {
        mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_NONE => {
            return b"none\0".as_ptr() as *const ::core::ffi::c_char;
        }
        mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_CONFIGURATION => {
            return b"configuration\0".as_ptr() as *const ::core::ffi::c_char;
        }
        mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_CONNECTION => {
            return b"connection\0".as_ptr() as *const ::core::ffi::c_char;
        }
        mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_PUBLICATION => {
            return b"publication\0".as_ptr() as *const ::core::ffi::c_char;
        }
        mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_COUNT | _ => {}
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn find_destination_stats(
    mut stats: *const mqtt_destination_client_stats_t,
    mut count: size_t,
    mut uuid: *const ::core::ffi::c_char,
) -> *const mqtt_destination_client_stats_t {
    let mut index: size_t = 0 as size_t;
    while index < count {
        if strcmp(
            &raw const (*stats.offset(index as isize)).destination_uuid
                as *const ::core::ffi::c_char,
            uuid,
        ) == 0 as ::core::ffi::c_int
        {
            return stats.offset(index as isize);
        }
        index = index.wrapping_add(1);
    }
    return ::core::ptr::null::<mqtt_destination_client_stats_t>();
}
unsafe extern "C" fn add_outbox_stats(
    mut parent: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut destination: *const ::core::ffi::c_char,
    mut now: int64_t,
) {
    let mut item: *mut cJSON = cJSON_CreateObject();
    if item.is_null() {
        return;
    }
    let mut stats: event_outbox_stats_t = event_outbox_stats_t {
        total_rows: 0,
        total_bytes: 0,
        pending_rows: 0,
        delivering_rows: 0,
        delivered_rows: 0,
        dead_rows: 0,
        due_rows: 0,
        oldest_pending_at: 0,
    };
    let mut available: bool =
        db_event_outbox_get_stats(destination, now, &raw mut stats) == 0 as ::core::ffi::c_int;
    cJSON_AddBoolToObject(
        item,
        b"available\0".as_ptr() as *const ::core::ffi::c_char,
        available as cJSON_bool,
    );
    if available {
        cJSON_AddNumberToObject(
            item,
            b"rows\0".as_ptr() as *const ::core::ffi::c_char,
            stats.total_rows as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"bytes\0".as_ptr() as *const ::core::ffi::c_char,
            stats.total_bytes as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"pending\0".as_ptr() as *const ::core::ffi::c_char,
            stats.pending_rows as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"delivering\0".as_ptr() as *const ::core::ffi::c_char,
            stats.delivering_rows as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"delivered\0".as_ptr() as *const ::core::ffi::c_char,
            stats.delivered_rows as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"dead\0".as_ptr() as *const ::core::ffi::c_char,
            stats.dead_rows as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"due\0".as_ptr() as *const ::core::ffi::c_char,
            stats.due_rows as ::core::ffi::c_double,
        );
        let mut has_oldest: bool = stats.pending_rows > 0 as int64_t
            && stats.oldest_pending_at > 0 as int64_t
            && now >= stats.oldest_pending_at;
        add_nullable_number(
            item,
            b"oldest_pending_age_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            has_oldest,
            if has_oldest as ::core::ffi::c_int != 0 {
                (now - stats.oldest_pending_at) as ::core::ffi::c_double
            } else {
                0.0f64
            },
        );
    }
    cJSON_AddItemToObject(parent, name, item);
}
unsafe extern "C" fn append_self_observability(
    mut root: *mut cJSON,
    mut sampler_stats: *const system_health_stats_t,
    mut evaluator_stats: *const system_health_evaluator_stats_t,
    mut incidents: *const system_health_incident_view_t,
    mut incident_count: size_t,
) {
    let mut self_0: *mut cJSON = cJSON_CreateObject();
    let mut collectors: *mut cJSON = cJSON_CreateArray();
    let mut delivery: *mut cJSON = cJSON_CreateObject();
    if self_0.is_null() || collectors.is_null() || delivery.is_null() {
        cJSON_Delete(self_0);
        cJSON_Delete(collectors);
        cJSON_Delete(delivery);
        return;
    }
    let mut collector_stats: [system_health_collector_stats_t; 33] =
        [system_health_collector_stats_t {
            name: [0; 32],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            tier: system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST,
            interval_seconds: 0,
            stale_after_seconds: 0,
            attempts: 0,
            completions: 0,
            failures: 0,
            timeouts: 0,
            overlap_skips: 0,
            last_duration_ms: 0,
            maximum_duration_ms: 0,
            last_attempt_monotonic_ms: 0,
            last_success_monotonic_ms: 0,
            busy: false,
            stale: false,
        }; 33];
    let mut collector_count: size_t = system_health_collector_stats_copy(
        &raw mut collector_stats as *mut system_health_collector_stats_t,
        SYSTEM_HEALTH_MAX_COLLECTORS.wrapping_add(1 as ::core::ffi::c_uint) as size_t,
    );
    let mut index: size_t = 0 as size_t;
    while index < collector_count {
        let mut stats: *const system_health_collector_stats_t = (&raw mut collector_stats
            as *mut system_health_collector_stats_t)
            .offset(index as isize);
        let mut item: *mut cJSON = cJSON_CreateObject();
        if !item.is_null() {
            cJSON_AddStringToObject(
                item,
                b"collector\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*stats).name as *const ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                item,
                b"scope\0".as_ptr() as *const ::core::ffi::c_char,
                system_health_scope_name((*stats).scope),
            );
            cJSON_AddNumberToObject(
                item,
                b"tier\0".as_ptr() as *const ::core::ffi::c_char,
                (*stats).tier.0 as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                item,
                b"attempts\0".as_ptr() as *const ::core::ffi::c_char,
                (*stats).attempts as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                item,
                b"completions\0".as_ptr() as *const ::core::ffi::c_char,
                (*stats).completions as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                item,
                b"failures\0".as_ptr() as *const ::core::ffi::c_char,
                (*stats).failures as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                item,
                b"timeouts\0".as_ptr() as *const ::core::ffi::c_char,
                (*stats).timeouts as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                item,
                b"overlap_skips\0".as_ptr() as *const ::core::ffi::c_char,
                (*stats).overlap_skips as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                item,
                b"last_duration_ms\0".as_ptr() as *const ::core::ffi::c_char,
                (*stats).last_duration_ms as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                item,
                b"maximum_duration_ms\0".as_ptr() as *const ::core::ffi::c_char,
                (*stats).maximum_duration_ms as ::core::ffi::c_double,
            );
            cJSON_AddBoolToObject(
                item,
                b"busy\0".as_ptr() as *const ::core::ffi::c_char,
                (*stats).busy as cJSON_bool,
            );
            cJSON_AddBoolToObject(
                item,
                b"stale\0".as_ptr() as *const ::core::ffi::c_char,
                (*stats).stale as cJSON_bool,
            );
            cJSON_AddItemToArray(collectors, item);
        }
        index = index.wrapping_add(1);
    }
    cJSON_AddItemToObject(
        self_0,
        b"collectors\0".as_ptr() as *const ::core::ffi::c_char,
        collectors,
    );
    cJSON_AddNumberToObject(
        self_0,
        b"coverage_overflows\0".as_ptr() as *const ::core::ffi::c_char,
        (*sampler_stats).coverage_overflows as ::core::ffi::c_double,
    );
    let mut persistence: *mut cJSON = cJSON_CreateObject();
    if !persistence.is_null() {
        cJSON_AddNumberToObject(
            persistence,
            b"pending\0".as_ptr() as *const ::core::ffi::c_char,
            (*evaluator_stats).pending_persistence as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            persistence,
            b"failures\0".as_ptr() as *const ::core::ffi::c_char,
            (*evaluator_stats).persistence_failures as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            persistence,
            b"retries\0".as_ptr() as *const ::core::ffi::c_char,
            (*evaluator_stats).persistence_retries as ::core::ffi::c_double,
        );
        cJSON_AddItemToObject(
            self_0,
            b"incident_persistence\0".as_ptr() as *const ::core::ffi::c_char,
            persistence,
        );
    }
    let mut bus: event_bus_stats_t = event_bus_stats_t {
        accepted_events: 0,
        dispatched_events: 0,
        callback_deliveries: 0,
        handler_failures: 0,
        dropped_events: 0,
        priority_shed_events: 0,
        rejected_events: 0,
        queued_events: 0,
        queued_bytes: 0,
        active_dispatches: 0,
        max_events: 0,
        max_bytes: 0,
        subscriber_count: 0,
        running: false,
    };
    let mut router: event_router_stats_t = event_router_stats_t {
        cache_reloads: 0,
        events_evaluated: 0,
        default_events: 0,
        matched_events: 0,
        unmatched_events: 0,
        evaluation_errors: 0,
        routes_considered: 0,
        type_rejections: 0,
        scope_rejections: 0,
        predicate_rejections: 0,
        schedule_rejections: 0,
        destination_disabled_rejections: 0,
        debounce_suppressions: 0,
        cooldown_suppressions: 0,
        grouping_suppressions: 0,
        rate_suppressions: 0,
        suppression_errors: 0,
    };
    let mut worker: mqtt_delivery_worker_stats_t = mqtt_delivery_worker_stats_t {
        enqueued: 0,
        duplicates: 0,
        rejected_full: 0,
        enqueue_errors: 0,
        priority_shed: 0,
        expired: 0,
        attempted: 0,
        delivered: 0,
        retried: 0,
        dead: 0,
        outcome_errors: 0,
        disconnected_polls: 0,
        managed_profiles: 0,
        managed_connected: 0,
        profile_reloads: 0,
        profile_errors: 0,
        pruned: 0,
        running: false,
    };
    let mut default_presence: mqtt_presence_stats_t = mqtt_presence_stats_t {
        configured: false,
        connected: false,
        sequence: 0,
        connections: 0,
        reconnects: 0,
        disconnects: 0,
        publish_attempts: 0,
        publish_successes: 0,
        publish_failures: 0,
        last_publish_at_ms: 0,
    };
    event_bus_get_stats(&raw mut bus);
    event_router_get_stats(&raw mut router);
    mqtt_delivery_worker_get_stats(&raw mut worker);
    mqtt_presence_get_stats(&raw mut default_presence);
    let mut managed: [mqtt_destination_client_stats_t; 64] = [mqtt_destination_client_stats_t {
        destination_uuid: [0; 37],
        presence_configured: false,
        connected: false,
        connections: 0,
        reconnects: 0,
        disconnects: 0,
        connection_failures: 0,
        publish_successes: 0,
        publish_failures: 0,
        last_success_at_ms: 0,
        last_failure_at_ms: 0,
        last_failure: mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_NONE,
    }; 64];
    let mut managed_count: size_t = mqtt_destination_client_list_stats(
        &raw mut managed as *mut mqtt_destination_client_stats_t,
        EVENT_DESTINATION_MAX_COUNT as size_t,
    );
    let mut profiles: [event_destination_t; 64] = [event_destination_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        enabled: false,
        destination_type: [0; 16],
        broker_host: [0; 256],
        broker_port: 0,
        client_id: [0; 128],
        topic_template: [0; 512],
        status_topic_template: [0; 512],
        username: [0; 128],
        password_configured: false,
        tls_mode: [0; 16],
        ca_file: [0; 512],
        cert_file: [0; 512],
        key_file: [0; 512],
        keepalive_seconds: 0,
        qos: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    }; 64];
    let mut profile_count: ::core::ffi::c_int = db_event_destination_list(
        &raw mut profiles as *mut event_destination_t,
        EVENT_DESTINATION_MAX_COUNT,
    );
    let mut profiles_available: bool = profile_count >= 0 as ::core::ffi::c_int;
    if profile_count < 0 as ::core::ffi::c_int {
        profile_count = 0 as ::core::ffi::c_int;
    }
    let mut now: int64_t = wall_time_seconds();
    let mut aggregate_outbox: event_outbox_stats_t = event_outbox_stats_t {
        total_rows: 0,
        total_bytes: 0,
        pending_rows: 0,
        delivering_rows: 0,
        delivered_rows: 0,
        dead_rows: 0,
        due_rows: 0,
        oldest_pending_at: 0,
    };
    let mut outbox_available: bool = db_event_outbox_get_stats(
        ::core::ptr::null::<::core::ffi::c_char>(),
        now,
        &raw mut aggregate_outbox,
    ) == 0 as ::core::ffi::c_int;
    let mut bus_json: *mut cJSON = cJSON_CreateObject();
    if !bus_json.is_null() {
        cJSON_AddBoolToObject(
            bus_json,
            b"running\0".as_ptr() as *const ::core::ffi::c_char,
            bus.running as cJSON_bool,
        );
        cJSON_AddNumberToObject(
            bus_json,
            b"accepted\0".as_ptr() as *const ::core::ffi::c_char,
            bus.accepted_events as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            bus_json,
            b"dispatched\0".as_ptr() as *const ::core::ffi::c_char,
            bus.dispatched_events as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            bus_json,
            b"dropped\0".as_ptr() as *const ::core::ffi::c_char,
            bus.dropped_events as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            bus_json,
            b"priority_shed\0".as_ptr() as *const ::core::ffi::c_char,
            bus.priority_shed_events as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            bus_json,
            b"rejected\0".as_ptr() as *const ::core::ffi::c_char,
            bus.rejected_events as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            bus_json,
            b"handler_failures\0".as_ptr() as *const ::core::ffi::c_char,
            bus.handler_failures as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            bus_json,
            b"queued_events\0".as_ptr() as *const ::core::ffi::c_char,
            bus.queued_events as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            bus_json,
            b"queued_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            bus.queued_bytes as ::core::ffi::c_double,
        );
        cJSON_AddItemToObject(
            delivery,
            b"bus\0".as_ptr() as *const ::core::ffi::c_char,
            bus_json,
        );
    }
    let mut router_json: *mut cJSON = cJSON_CreateObject();
    if !router_json.is_null() {
        cJSON_AddNumberToObject(
            router_json,
            b"events_evaluated\0".as_ptr() as *const ::core::ffi::c_char,
            router.events_evaluated as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            router_json,
            b"matched\0".as_ptr() as *const ::core::ffi::c_char,
            router.matched_events as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            router_json,
            b"unmatched\0".as_ptr() as *const ::core::ffi::c_char,
            router.unmatched_events as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            router_json,
            b"errors\0".as_ptr() as *const ::core::ffi::c_char,
            router.evaluation_errors as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            router_json,
            b"rate_suppressions\0".as_ptr() as *const ::core::ffi::c_char,
            router.rate_suppressions as ::core::ffi::c_double,
        );
        cJSON_AddItemToObject(
            delivery,
            b"router\0".as_ptr() as *const ::core::ffi::c_char,
            router_json,
        );
    }
    let mut worker_json: *mut cJSON = cJSON_CreateObject();
    if !worker_json.is_null() {
        cJSON_AddBoolToObject(
            worker_json,
            b"running\0".as_ptr() as *const ::core::ffi::c_char,
            worker.running as cJSON_bool,
        );
        cJSON_AddNumberToObject(
            worker_json,
            b"enqueued\0".as_ptr() as *const ::core::ffi::c_char,
            worker.enqueued as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            worker_json,
            b"rejected_full\0".as_ptr() as *const ::core::ffi::c_char,
            worker.rejected_full as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            worker_json,
            b"enqueue_errors\0".as_ptr() as *const ::core::ffi::c_char,
            worker.enqueue_errors as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            worker_json,
            b"attempted\0".as_ptr() as *const ::core::ffi::c_char,
            worker.attempted as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            worker_json,
            b"delivered\0".as_ptr() as *const ::core::ffi::c_char,
            worker.delivered as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            worker_json,
            b"retried\0".as_ptr() as *const ::core::ffi::c_char,
            worker.retried as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            worker_json,
            b"dead\0".as_ptr() as *const ::core::ffi::c_char,
            worker.dead as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            worker_json,
            b"profile_errors\0".as_ptr() as *const ::core::ffi::c_char,
            worker.profile_errors as ::core::ffi::c_double,
        );
        cJSON_AddItemToObject(
            delivery,
            b"worker\0".as_ptr() as *const ::core::ffi::c_char,
            worker_json,
        );
    }
    add_outbox_stats(
        delivery,
        b"outbox\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        now,
    );
    let mut destinations: *mut cJSON = cJSON_CreateArray();
    let mut configured: size_t = (if default_presence.configured as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_uint
    } else {
        0 as ::core::ffi::c_uint
    }) as size_t;
    let mut connected: size_t = (if default_presence.configured as ::core::ffi::c_int != 0
        && default_presence.connected as ::core::ffi::c_int != 0
    {
        1 as ::core::ffi::c_uint
    } else {
        0 as ::core::ffi::c_uint
    }) as size_t;
    let mut only_destination_failed: bool = r#false != 0;
    if !destinations.is_null() {
        if default_presence.configured {
            let mut item_0: *mut cJSON = cJSON_CreateObject();
            if !item_0.is_null() {
                cJSON_AddStringToObject(
                    item_0,
                    b"destination\0".as_ptr() as *const ::core::ffi::c_char,
                    b"default\0".as_ptr() as *const ::core::ffi::c_char,
                );
                cJSON_AddBoolToObject(
                    item_0,
                    b"connected\0".as_ptr() as *const ::core::ffi::c_char,
                    default_presence.connected as cJSON_bool,
                );
                cJSON_AddNumberToObject(
                    item_0,
                    b"connections\0".as_ptr() as *const ::core::ffi::c_char,
                    default_presence.connections as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    item_0,
                    b"reconnects\0".as_ptr() as *const ::core::ffi::c_char,
                    default_presence.reconnects as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    item_0,
                    b"publish_failures\0".as_ptr() as *const ::core::ffi::c_char,
                    default_presence.publish_failures as ::core::ffi::c_double,
                );
                add_outbox_stats(
                    item_0,
                    b"outbox\0".as_ptr() as *const ::core::ffi::c_char,
                    MQTT_EVENT_OUTBOX_DESTINATION.as_ptr(),
                    now,
                );
                cJSON_AddItemToArray(destinations, item_0);
            }
        }
        let mut index_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while index_0 < profile_count {
            if profiles[index_0 as usize].enabled {
                configured = configured.wrapping_add(1);
                let mut stats_0: *const mqtt_destination_client_stats_t = find_destination_stats(
                    &raw mut managed as *mut mqtt_destination_client_stats_t,
                    managed_count,
                    &raw mut (*(&raw mut profiles as *mut event_destination_t)
                        .offset(index_0 as isize))
                    .uuid as *mut ::core::ffi::c_char,
                );
                if !stats_0.is_null() && (*stats_0).connected as ::core::ffi::c_int != 0 {
                    connected = connected.wrapping_add(1);
                }
                let mut item_1: *mut cJSON = cJSON_CreateObject();
                if !item_1.is_null() {
                    cJSON_AddStringToObject(
                        item_1,
                        b"destination\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*(&raw mut profiles as *mut event_destination_t)
                            .offset(index_0 as isize))
                        .uuid as *mut ::core::ffi::c_char,
                    );
                    cJSON_AddBoolToObject(
                        item_1,
                        b"runtime_available\0".as_ptr() as *const ::core::ffi::c_char,
                        !stats_0.is_null() as ::core::ffi::c_int,
                    );
                    cJSON_AddBoolToObject(
                        item_1,
                        b"connected\0".as_ptr() as *const ::core::ffi::c_char,
                        (!stats_0.is_null() && (*stats_0).connected as ::core::ffi::c_int != 0)
                            as ::core::ffi::c_int,
                    );
                    cJSON_AddNumberToObject(
                        item_1,
                        b"connections\0".as_ptr() as *const ::core::ffi::c_char,
                        if !stats_0.is_null() {
                            (*stats_0).connections as ::core::ffi::c_double
                        } else {
                            0.0f64
                        },
                    );
                    cJSON_AddNumberToObject(
                        item_1,
                        b"reconnects\0".as_ptr() as *const ::core::ffi::c_char,
                        if !stats_0.is_null() {
                            (*stats_0).reconnects as ::core::ffi::c_double
                        } else {
                            0.0f64
                        },
                    );
                    cJSON_AddNumberToObject(
                        item_1,
                        b"connection_failures\0".as_ptr() as *const ::core::ffi::c_char,
                        if !stats_0.is_null() {
                            (*stats_0).connection_failures as ::core::ffi::c_double
                        } else {
                            0.0f64
                        },
                    );
                    cJSON_AddNumberToObject(
                        item_1,
                        b"publish_failures\0".as_ptr() as *const ::core::ffi::c_char,
                        if !stats_0.is_null() {
                            (*stats_0).publish_failures as ::core::ffi::c_double
                        } else {
                            0.0f64
                        },
                    );
                    cJSON_AddStringToObject(
                        item_1,
                        b"last_failure\0".as_ptr() as *const ::core::ffi::c_char,
                        if !stats_0.is_null() {
                            destination_failure_name((*stats_0).last_failure)
                        } else {
                            b"configuration\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                    let mut destination_key: [::core::ffi::c_char; 64] = [0; 64];
                    if db_event_destination_make_key(
                        &raw mut (*(&raw mut profiles as *mut event_destination_t)
                            .offset(index_0 as isize))
                        .uuid as *mut ::core::ffi::c_char,
                        &raw mut destination_key as *mut ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        add_outbox_stats(
                            item_1,
                            b"outbox\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut destination_key as *mut ::core::ffi::c_char,
                            now,
                        );
                    }
                    cJSON_AddItemToArray(destinations, item_1);
                }
            }
            index_0 += 1;
        }
        cJSON_AddItemToObject(
            delivery,
            b"destinations\0".as_ptr() as *const ::core::ffi::c_char,
            destinations,
        );
    }
    if configured == 1 as size_t {
        if default_presence.configured {
            only_destination_failed =
                !default_presence.connected && default_presence.publish_failures > 0 as uint64_t;
        } else {
            let mut index_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while index_1 < profile_count {
                if !profiles[index_1 as usize].enabled {
                    index_1 += 1;
                } else {
                    let mut stats_1: *const mqtt_destination_client_stats_t =
                        find_destination_stats(
                            &raw mut managed as *mut mqtt_destination_client_stats_t,
                            managed_count,
                            &raw mut (*(&raw mut profiles as *mut event_destination_t)
                                .offset(index_1 as isize))
                            .uuid as *mut ::core::ffi::c_char,
                        );
                    only_destination_failed = stats_1.is_null()
                        || !(*stats_1).connected
                            && (*stats_1).last_failure.0
                                != mqtt_destination_failure_t::MQTT_DESTINATION_FAILURE_NONE.0;
                    break;
                }
            }
        }
    }
    let mut outbox_full: bool = outbox_available as ::core::ffi::c_int != 0
        && (aggregate_outbox.total_rows >= EVENT_OUTBOX_DEFAULT_MAX_ROWS as int64_t
            || aggregate_outbox.total_bytes as ::core::ffi::c_longlong
                >= EVENT_OUTBOX_DEFAULT_MAX_BYTES);
    let mut undeliverable: bool = outbox_available as ::core::ffi::c_int != 0
        && aggregate_outbox.pending_rows > 0 as int64_t
        && connected == 0 as size_t;
    let mut degraded: bool = outbox_full as ::core::ffi::c_int != 0
        || only_destination_failed as ::core::ffi::c_int != 0
        || undeliverable as ::core::ffi::c_int != 0
        || outbox_available as ::core::ffi::c_int != 0 && aggregate_outbox.dead_rows > 0 as int64_t
        || !profiles_available && managed_count > 0 as size_t;
    cJSON_AddBoolToObject(
        delivery,
        b"degraded\0".as_ptr() as *const ::core::ffi::c_char,
        degraded as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        delivery,
        b"circular_report_path\0".as_ptr() as *const ::core::ffi::c_char,
        (configured == 1 as size_t
            && (only_destination_failed as ::core::ffi::c_int != 0
                || outbox_full as ::core::ffi::c_int != 0
                || undeliverable as ::core::ffi::c_int != 0)) as ::core::ffi::c_int,
    );
    cJSON_AddBoolToObject(
        delivery,
        b"outbox_full\0".as_ptr() as *const ::core::ffi::c_char,
        outbox_full as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        delivery,
        b"all_destinations_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        undeliverable as cJSON_bool,
    );
    cJSON_AddItemToObject(
        self_0,
        b"event_delivery\0".as_ptr() as *const ::core::ffi::c_char,
        delivery,
    );
    let mut now_ms: int64_t = if now > 0 as int64_t {
        now * 1000 as int64_t
    } else {
        0 as int64_t
    };
    let mut oldest_pending_ms: int64_t = (*evaluator_stats).oldest_pending_wall_time_ms;
    add_nullable_number(
        self_0,
        b"incident_persistence_retry_age_ms\0".as_ptr() as *const ::core::ffi::c_char,
        oldest_pending_ms > 0 as int64_t && now_ms >= oldest_pending_ms,
        if oldest_pending_ms > 0 as int64_t && now_ms >= oldest_pending_ms {
            (now_ms - oldest_pending_ms) as ::core::ffi::c_double
        } else {
            0.0f64
        },
    );
    cJSON_AddItemToObject(
        root,
        b"self_observability\0".as_ptr() as *const ::core::ffi::c_char,
        self_0,
    );
}
unsafe extern "C" fn add_nullable_number(
    mut object: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut valid: bool,
    mut value: ::core::ffi::c_double,
) {
    if valid as ::core::ffi::c_int != 0 && value.is_finite() as ::core::ffi::c_int != 0 {
        cJSON_AddNumberToObject(object, name, value);
    } else {
        cJSON_AddNullToObject(object, name);
    };
}
unsafe extern "C" fn observation_to_json(
    mut value: *const system_health_observation_t,
) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_CreateObject();
    if item.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        item,
        b"metric\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*value).metric as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        item,
        b"resource\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*value).resource_id as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        item,
        b"scope\0".as_ptr() as *const ::core::ffi::c_char,
        system_health_scope_name((*value).scope),
    );
    cJSON_AddStringToObject(
        item,
        b"capability\0".as_ptr() as *const ::core::ffi::c_char,
        system_health_capability_name((*value).capability),
    );
    cJSON_AddStringToObject(
        item,
        b"freshness\0".as_ptr() as *const ::core::ffi::c_char,
        freshness_name((*value).freshness),
    );
    cJSON_AddStringToObject(
        item,
        b"unit\0".as_ptr() as *const ::core::ffi::c_char,
        system_health_unit_name((*value).unit),
    );
    add_nullable_number(
        item,
        b"value\0".as_ptr() as *const ::core::ffi::c_char,
        (*value).value_valid,
        (*value).value,
    );
    cJSON_AddNumberToObject(
        item,
        b"sampled_monotonic_ms\0".as_ptr() as *const ::core::ffi::c_char,
        (*value).sampled_monotonic_ms as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        item,
        b"observed_at_ms\0".as_ptr() as *const ::core::ffi::c_char,
        (*value).observed_wall_time_ms as ::core::ffi::c_double,
    );
    return item;
}
unsafe extern "C" fn policy_to_json(
    mut rule: *const system_health_condition_policy_t,
) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_CreateObject();
    if item.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        item,
        b"condition\0".as_ptr() as *const ::core::ffi::c_char,
        system_health_condition_code((*rule).condition),
    );
    cJSON_AddBoolToObject(
        item,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        (*rule).enabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        item,
        b"overridden\0".as_ptr() as *const ::core::ffi::c_char,
        (*rule).overridden as cJSON_bool,
    );
    cJSON_AddStringToObject(
        item,
        b"direction\0".as_ptr() as *const ::core::ffi::c_char,
        direction_name((*rule).direction),
    );
    cJSON_AddStringToObject(
        item,
        b"unit\0".as_ptr() as *const ::core::ffi::c_char,
        system_health_unit_name((*rule).unit),
    );
    cJSON_AddNumberToObject(
        item,
        b"warning_threshold\0".as_ptr() as *const ::core::ffi::c_char,
        (*rule).warning_threshold,
    );
    cJSON_AddNumberToObject(
        item,
        b"critical_threshold\0".as_ptr() as *const ::core::ffi::c_char,
        (*rule).critical_threshold,
    );
    cJSON_AddNumberToObject(
        item,
        b"recovery_threshold\0".as_ptr() as *const ::core::ffi::c_char,
        (*rule).recovery_threshold,
    );
    cJSON_AddNumberToObject(
        item,
        b"warning_for_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        (*rule).warning_for_seconds as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        item,
        b"critical_for_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        (*rule).critical_for_seconds as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        item,
        b"recovery_for_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        (*rule).recovery_for_seconds as ::core::ffi::c_double,
    );
    return item;
}
unsafe extern "C" fn incident_view_to_json(
    mut incident: *const system_health_incident_view_t,
    mut policy: *const system_health_policy_t,
) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_CreateObject();
    let mut observation: *mut cJSON = observation_to_json(&raw const (*incident).observation);
    if item.is_null() || observation.is_null() {
        cJSON_Delete(item);
        cJSON_Delete(observation);
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut code: *const ::core::ffi::c_char = system_health_condition_code((*incident).condition);
    cJSON_AddStringToObject(
        item,
        b"incident_id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*incident).incident_id as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        item,
        b"condition\0".as_ptr() as *const ::core::ffi::c_char,
        if !code.is_null() {
            code
        } else {
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddStringToObject(
        item,
        b"subject\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*incident).subject as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        item,
        b"scope\0".as_ptr() as *const ::core::ffi::c_char,
        system_health_scope_name((*incident).scope),
    );
    cJSON_AddStringToObject(
        item,
        b"state\0".as_ptr() as *const ::core::ffi::c_char,
        state_name((*incident).state),
    );
    cJSON_AddStringToObject(
        item,
        b"severity\0".as_ptr() as *const ::core::ffi::c_char,
        system_health_severity_name((*incident).severity),
    );
    cJSON_AddNumberToObject(
        item,
        b"first_observed_at_ms\0".as_ptr() as *const ::core::ffi::c_char,
        (*incident).first_observed_at_ms as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        item,
        b"last_observed_at_ms\0".as_ptr() as *const ::core::ffi::c_char,
        (*incident).last_observed_at_ms as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        item,
        b"persistence_pending\0".as_ptr() as *const ::core::ffi::c_char,
        (*incident).persistence_pending as cJSON_bool,
    );
    cJSON_AddItemToObject(
        item,
        b"observation\0".as_ptr() as *const ::core::ffi::c_char,
        observation,
    );
    if (*incident).condition.0 >= 0 as ::core::ffi::c_uint
        && (*incident).condition.0 < system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0
    {
        let mut thresholds: *mut cJSON = policy_to_json(
            (&raw const (*policy).conditions as *const system_health_condition_policy_t)
                .offset((*incident).condition.0 as isize),
        );
        if !thresholds.is_null() {
            cJSON_AddItemToObject(
                item,
                b"thresholds\0".as_ptr() as *const ::core::ffi::c_char,
                thresholds,
            );
        }
    }
    return item;
}
unsafe extern "C" fn add_count_object(
    mut parent: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut names: *const *const ::core::ffi::c_char,
    mut counts: *const size_t,
    mut count: size_t,
) {
    let mut object: *mut cJSON = cJSON_CreateObject();
    if object.is_null() {
        return;
    }
    let mut index: size_t = 0 as size_t;
    while index < count {
        cJSON_AddNumberToObject(
            object,
            *names.offset(index as isize),
            *counts.offset(index as isize) as ::core::ffi::c_double,
        );
        index = index.wrapping_add(1);
    }
    cJSON_AddItemToObject(parent, name, object);
}
unsafe extern "C" fn effective_visibility(
    mut scope_counts: *const size_t,
) -> *const ::core::ffi::c_char {
    if *scope_counts.offset(
        system_health_scope_t::SYSTEM_HEALTH_SCOPE_CONTAINER.0 as ::core::ffi::c_int as isize,
    ) > 0 as size_t
    {
        return b"container\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if *scope_counts
        .offset(system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST.0 as ::core::ffi::c_int as isize)
        > 0 as size_t
    {
        return b"host\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if *scope_counts
        .offset(system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS.0 as ::core::ffi::c_int as isize)
        > 0 as size_t
    {
        return b"process\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn send_json(mut res: *mut http_response_t, mut root: *mut cJSON) -> bool {
    let mut encoded: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if encoded.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize system health\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut result: ::core::ffi::c_int =
        http_response_set_json(res, 200 as ::core::ffi::c_int, encoded);
    cJSON_free(encoded as *mut ::core::ffi::c_void);
    return result == 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_system_health(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_SYSTEM_ADMIN) == 0 {
        return;
    }
    let mut snapshot: system_health_snapshot_t = system_health_snapshot_t {
        observations: [system_health_observation_t {
            metric: [0; 64],
            resource_id: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            freshness: system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_UNKNOWN,
            unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
            value_valid: false,
            value: 0.,
            sampled_monotonic_ms: 0,
            observed_wall_time_ms: 0,
        }; 256],
        observation_count: 0,
        observations_dropped: 0,
        sequence: 0,
        completed_monotonic_ms: 0,
        completed_wall_time_ms: 0,
    };
    memset(
        &raw mut snapshot as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_snapshot_t>(),
    );
    let mut has_snapshot: bool = system_health_snapshot_copy(&raw mut snapshot);
    let mut policy: system_health_policy_t = system_health_policy_t {
        generation: 0,
        settings: system_health_policy_settings_t {
            enabled: false,
            profile: [0; 16],
            fast_interval_seconds: 0,
            normal_interval_seconds: 0,
            slow_interval_seconds: 0,
            device_interval_seconds: 0,
            write_probe_enabled: false,
            hardware_provider: [0; 16],
            presence_interval_seconds: 0,
            incident_retention_days: 0,
        },
        conditions: [system_health_condition_policy_t {
            condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
            profile: system_health_profile_t::SYSTEM_HEALTH_PROFILE_BALANCED,
            enabled: false,
            overridden: false,
            direction: system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE,
            unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
            warning_threshold: 0.,
            critical_threshold: 0.,
            recovery_threshold: 0.,
            warning_for_seconds: 0,
            critical_for_seconds: 0,
            recovery_for_seconds: 0,
        }; 27],
    };
    memset(
        &raw mut policy as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_policy_t>(),
    );
    system_health_policy_snapshot(&raw mut policy);
    let mut sampler_stats: system_health_stats_t = system_health_stats_t {
        initialized: false,
        enabled: false,
        ring_allocated: false,
        worker_threads: 0,
        generations_completed: 0,
        collections_completed: 0,
        collection_errors: 0,
        collection_timeouts: 0,
        overlap_skips: 0,
        observations_dropped: 0,
        coverage_overflows: 0,
        abandoned_helpers: 0,
    };
    system_health_get_stats(&raw mut sampler_stats);
    let mut evaluator_stats: system_health_evaluator_stats_t = system_health_evaluator_stats_t {
        tracked_conditions: 0,
        active_incidents: 0,
        pending_persistence: 0,
        transitions: 0,
        persistence_failures: 0,
        persistence_retries: 0,
        oldest_pending_wall_time_ms: 0,
    };
    system_health_evaluator_service_get_stats(&raw mut evaluator_stats);
    let mut incidents: [system_health_incident_view_t; 64] = [system_health_incident_view_t {
        incident_id: [0; 37],
        condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
        subject: [0; 64],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
        state: system_health_state_t::SYSTEM_HEALTH_STATE_UNKNOWN,
        severity: system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE,
        first_observed_at_ms: 0,
        last_observed_at_ms: 0,
        observation: system_health_observation_t {
            metric: [0; 64],
            resource_id: [0; 64],
            scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
            capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            freshness: system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_UNKNOWN,
            unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
            value_valid: false,
            value: 0.,
            sampled_monotonic_ms: 0,
            observed_wall_time_ms: 0,
        },
        persistence_pending: false,
    }; 64];
    let mut incident_count: size_t = system_health_evaluator_service_active_copy(
        &raw mut incidents as *mut system_health_incident_view_t,
        SYSTEM_HEALTH_MAX_INCIDENTS as size_t,
    );
    let mut summaries: [system_health_summary_t; 120] = [system_health_summary_t {
        sequence: 0,
        completed_monotonic_ms: 0,
        completed_wall_time_ms: 0,
        observation_count: 0,
        observations_dropped: 0,
    }; 120];
    let mut summary_count: size_t = system_health_summary_copy(
        &raw mut summaries as *mut system_health_summary_t,
        SYSTEM_HEALTH_RING_SAMPLES as size_t,
    );
    let mut capability_counts: [size_t; 5] = [0 as size_t, 0, 0, 0, 0];
    let mut scope_counts: [size_t; 5] = [0 as size_t, 0, 0, 0, 0];
    let mut unavailable_count: size_t = 0 as size_t;
    if has_snapshot {
        let mut index: size_t = 0 as size_t;
        while index < snapshot.observation_count {
            let mut observation: *const system_health_observation_t =
                (&raw mut snapshot.observations as *mut system_health_observation_t)
                    .offset(index as isize);
            if (*observation).capability.0 >= 0 as ::core::ffi::c_uint
                && (*observation).capability.0
                    < system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_COUNT.0
            {
                capability_counts[(*observation).capability.0 as usize] =
                    capability_counts[(*observation).capability.0 as usize].wrapping_add(1);
            }
            if (*observation).scope.0 >= 0 as ::core::ffi::c_uint
                && (*observation).scope.0 < system_health_scope_t::SYSTEM_HEALTH_SCOPE_COUNT.0
            {
                scope_counts[(*observation).scope.0 as usize] =
                    scope_counts[(*observation).scope.0 as usize].wrapping_add(1);
            }
            if !(*observation).value_valid {
                unavailable_count = unavailable_count.wrapping_add(1);
            }
            index = index.wrapping_add(1);
        }
    }
    let mut overall: system_health_severity_t =
        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE;
    if has_snapshot {
        let mut index_0: size_t = 0 as size_t;
        while index_0 < incident_count {
            if incidents[index_0].severity.0 > overall.0 {
                overall = incidents[index_0].severity;
            }
            index_0 = index_0.wrapping_add(1);
        }
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut visibility: *mut cJSON = cJSON_CreateObject();
    let mut generation: *mut cJSON = cJSON_CreateObject();
    let mut coverage: *mut cJSON = cJSON_CreateObject();
    let mut observations: *mut cJSON = cJSON_CreateArray();
    let mut thresholds: *mut cJSON = cJSON_CreateArray();
    let mut active: *mut cJSON = cJSON_CreateArray();
    let mut recent_samples: *mut cJSON = cJSON_CreateArray();
    if root.is_null()
        || visibility.is_null()
        || generation.is_null()
        || coverage.is_null()
        || observations.is_null()
        || thresholds.is_null()
        || active.is_null()
        || recent_samples.is_null()
    {
        cJSON_Delete(root);
        cJSON_Delete(visibility);
        cJSON_Delete(generation);
        cJSON_Delete(coverage);
        cJSON_Delete(observations);
        cJSON_Delete(thresholds);
        cJSON_Delete(active);
        cJSON_Delete(recent_samples);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build system health response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddNumberToObject(
        root,
        b"schema_version\0".as_ptr() as *const ::core::ffi::c_char,
        SYSTEM_HEALTH_API_SCHEMA_VERSION as ::core::ffi::c_double,
    );
    let mut monitoring: *mut cJSON =
        cJSON_AddObjectToObject(root, b"monitoring\0".as_ptr() as *const ::core::ffi::c_char);
    if !monitoring.is_null() {
        let mut configured_enabled: bool = g_config.health.enabled as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut g_config.health.profile as *mut ::core::ffi::c_char,
                b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int;
        cJSON_AddBoolToObject(
            monitoring,
            b"configured_enabled\0".as_ptr() as *const ::core::ffi::c_char,
            configured_enabled as cJSON_bool,
        );
        cJSON_AddBoolToObject(
            monitoring,
            b"initialized\0".as_ptr() as *const ::core::ffi::c_char,
            sampler_stats.initialized as cJSON_bool,
        );
        cJSON_AddBoolToObject(
            monitoring,
            b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
            sampler_stats.enabled as cJSON_bool,
        );
        cJSON_AddNumberToObject(
            monitoring,
            b"worker_threads\0".as_ptr() as *const ::core::ffi::c_char,
            sampler_stats.worker_threads as ::core::ffi::c_double,
        );
        cJSON_AddBoolToObject(
            monitoring,
            b"restart_required\0".as_ptr() as *const ::core::ffi::c_char,
            (sampler_stats.initialized as ::core::ffi::c_int != 0
                && configured_enabled as ::core::ffi::c_int
                    != sampler_stats.enabled as ::core::ffi::c_int)
                as ::core::ffi::c_int,
        );
    }
    cJSON_AddStringToObject(
        root,
        b"overall_state\0".as_ptr() as *const ::core::ffi::c_char,
        if !has_snapshot {
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char
        } else if overall.0 == system_health_severity_t::SYSTEM_HEALTH_SEVERITY_NONE.0 {
            b"healthy\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            system_health_severity_name(overall)
        },
    );
    cJSON_AddStringToObject(
        visibility,
        b"effective_scope\0".as_ptr() as *const ::core::ffi::c_char,
        effective_visibility(&raw mut scope_counts as *mut size_t),
    );
    cJSON_AddBoolToObject(
        visibility,
        b"host_hardware_visible\0".as_ptr() as *const ::core::ffi::c_char,
        (scope_counts
            [system_health_scope_t::SYSTEM_HEALTH_SCOPE_DEVICE.0 as ::core::ffi::c_int as usize]
            > 0 as size_t) as ::core::ffi::c_int,
    );
    cJSON_AddStringToObject(
        visibility,
        b"coverage_boundary\0".as_ptr() as *const ::core::ffi::c_char,
        if scope_counts
            [system_health_scope_t::SYSTEM_HEALTH_SCOPE_CONTAINER.0 as ::core::ffi::c_int as usize]
            > 0 as size_t
        {
            b"container_and_visible_mounts\0".as_ptr() as *const ::core::ffi::c_char
        } else if scope_counts
            [system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST.0 as ::core::ffi::c_int as usize]
            > 0 as size_t
        {
            b"host_and_visible_mounts\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddItemToObject(
        root,
        b"visibility\0".as_ptr() as *const ::core::ffi::c_char,
        visibility,
    );
    let mut now: uint64_t = monotonic_ms();
    let mut age_valid: bool =
        has_snapshot as ::core::ffi::c_int != 0 && now >= snapshot.completed_monotonic_ms;
    let mut age: uint64_t = if age_valid as ::core::ffi::c_int != 0 {
        now.wrapping_sub(snapshot.completed_monotonic_ms)
    } else {
        0 as uint64_t
    };
    let mut stale_after: uint64_t =
        (policy.settings.fast_interval_seconds as uint64_t).wrapping_mul(3000 as uint64_t);
    cJSON_AddBoolToObject(
        generation,
        b"available\0".as_ptr() as *const ::core::ffi::c_char,
        has_snapshot as cJSON_bool,
    );
    add_nullable_number(
        generation,
        b"sequence\0".as_ptr() as *const ::core::ffi::c_char,
        has_snapshot,
        snapshot.sequence as ::core::ffi::c_double,
    );
    add_nullable_number(
        generation,
        b"completed_at_ms\0".as_ptr() as *const ::core::ffi::c_char,
        has_snapshot,
        snapshot.completed_wall_time_ms as ::core::ffi::c_double,
    );
    add_nullable_number(
        generation,
        b"age_ms\0".as_ptr() as *const ::core::ffi::c_char,
        age_valid,
        age as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        generation,
        b"freshness\0".as_ptr() as *const ::core::ffi::c_char,
        if !has_snapshot || !age_valid {
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char
        } else if stale_after > 0 as uint64_t && age > stale_after {
            b"stale\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"fresh\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddItemToObject(
        root,
        b"snapshot\0".as_ptr() as *const ::core::ffi::c_char,
        generation,
    );
    static mut capability_names: [*const ::core::ffi::c_char; 5] = [
        b"available\0".as_ptr() as *const ::core::ffi::c_char,
        b"unsupported\0".as_ptr() as *const ::core::ffi::c_char,
        b"permission_denied\0".as_ptr() as *const ::core::ffi::c_char,
        b"stale\0".as_ptr() as *const ::core::ffi::c_char,
        b"error\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut scope_names: [*const ::core::ffi::c_char; 5] = [
        b"process\0".as_ptr() as *const ::core::ffi::c_char,
        b"container\0".as_ptr() as *const ::core::ffi::c_char,
        b"host\0".as_ptr() as *const ::core::ffi::c_char,
        b"filesystem\0".as_ptr() as *const ::core::ffi::c_char,
        b"device\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    add_count_object(
        coverage,
        b"capabilities\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const capability_names as *const *const ::core::ffi::c_char,
        &raw mut capability_counts as *mut size_t,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_COUNT.0 as ::core::ffi::c_int
            as size_t,
    );
    add_count_object(
        coverage,
        b"scopes\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const scope_names as *const *const ::core::ffi::c_char,
        &raw mut scope_counts as *mut size_t,
        system_health_scope_t::SYSTEM_HEALTH_SCOPE_COUNT.0 as ::core::ffi::c_int as size_t,
    );
    let mut incomplete: uint64_t = (unavailable_count as uint64_t)
        .wrapping_add(snapshot.observations_dropped as uint64_t)
        .wrapping_add(sampler_stats.collection_errors)
        .wrapping_add(sampler_stats.collection_timeouts)
        .wrapping_add(sampler_stats.abandoned_helpers as uint64_t)
        .wrapping_add(sampler_stats.coverage_overflows);
    cJSON_AddBoolToObject(
        coverage,
        b"complete\0".as_ptr() as *const ::core::ffi::c_char,
        (has_snapshot as ::core::ffi::c_int != 0
            && incomplete == 0 as uint64_t
            && sampler_stats.collection_errors == 0 as uint64_t
            && sampler_stats.collection_timeouts == 0 as uint64_t) as ::core::ffi::c_int,
    );
    cJSON_AddNumberToObject(
        coverage,
        b"incomplete_count\0".as_ptr() as *const ::core::ffi::c_char,
        incomplete as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        coverage,
        b"unavailable_observations\0".as_ptr() as *const ::core::ffi::c_char,
        unavailable_count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        coverage,
        b"observations_dropped\0".as_ptr() as *const ::core::ffi::c_char,
        snapshot.observations_dropped as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        coverage,
        b"collection_errors\0".as_ptr() as *const ::core::ffi::c_char,
        sampler_stats.collection_errors as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        coverage,
        b"collection_timeouts\0".as_ptr() as *const ::core::ffi::c_char,
        sampler_stats.collection_timeouts as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        coverage,
        b"abandoned_helpers\0".as_ptr() as *const ::core::ffi::c_char,
        sampler_stats.abandoned_helpers as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        coverage,
        b"coverage_overflows\0".as_ptr() as *const ::core::ffi::c_char,
        sampler_stats.coverage_overflows as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"coverage\0".as_ptr() as *const ::core::ffi::c_char,
        coverage,
    );
    let mut index_1: size_t = 0 as size_t;
    while index_1 < snapshot.observation_count {
        let mut item: *mut cJSON = observation_to_json(
            (&raw mut snapshot.observations as *mut system_health_observation_t)
                .offset(index_1 as isize),
        );
        if !item.is_null() {
            cJSON_AddItemToArray(observations, item);
        }
        index_1 = index_1.wrapping_add(1);
    }
    cJSON_AddItemToObject(
        root,
        b"observations\0".as_ptr() as *const ::core::ffi::c_char,
        observations,
    );
    let mut index_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index_2 < system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0 as ::core::ffi::c_int
    {
        let mut item_0: *mut cJSON = policy_to_json(
            (&raw mut policy.conditions as *mut system_health_condition_policy_t)
                .offset(index_2 as isize),
        );
        if !item_0.is_null() {
            cJSON_AddItemToArray(thresholds, item_0);
        }
        index_2 += 1;
    }
    cJSON_AddItemToObject(
        root,
        b"thresholds\0".as_ptr() as *const ::core::ffi::c_char,
        thresholds,
    );
    let mut index_3: size_t = 0 as size_t;
    while index_3 < incident_count {
        let mut item_1: *mut cJSON = incident_view_to_json(
            (&raw mut incidents as *mut system_health_incident_view_t).offset(index_3 as isize),
            &raw mut policy,
        );
        if !item_1.is_null() {
            cJSON_AddItemToArray(active, item_1);
        }
        index_3 = index_3.wrapping_add(1);
    }
    cJSON_AddItemToObject(
        root,
        b"active_incidents\0".as_ptr() as *const ::core::ffi::c_char,
        active,
    );
    let mut index_4: size_t = 0 as size_t;
    while index_4 < summary_count {
        let mut item_2: *mut cJSON = cJSON_CreateObject();
        if !item_2.is_null() {
            cJSON_AddNumberToObject(
                item_2,
                b"sequence\0".as_ptr() as *const ::core::ffi::c_char,
                summaries[index_4].sequence as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                item_2,
                b"completed_at_ms\0".as_ptr() as *const ::core::ffi::c_char,
                summaries[index_4].completed_wall_time_ms as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                item_2,
                b"observation_count\0".as_ptr() as *const ::core::ffi::c_char,
                summaries[index_4].observation_count as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                item_2,
                b"observations_dropped\0".as_ptr() as *const ::core::ffi::c_char,
                summaries[index_4].observations_dropped as ::core::ffi::c_double,
            );
            cJSON_AddItemToArray(recent_samples, item_2);
        }
        index_4 = index_4.wrapping_add(1);
    }
    cJSON_AddItemToObject(
        root,
        b"recent_samples\0".as_ptr() as *const ::core::ffi::c_char,
        recent_samples,
    );
    let mut evaluator: *mut cJSON = cJSON_CreateObject();
    if !evaluator.is_null() {
        cJSON_AddNumberToObject(
            evaluator,
            b"tracked_conditions\0".as_ptr() as *const ::core::ffi::c_char,
            evaluator_stats.tracked_conditions as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            evaluator,
            b"active_incidents\0".as_ptr() as *const ::core::ffi::c_char,
            evaluator_stats.active_incidents as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            evaluator,
            b"pending_persistence\0".as_ptr() as *const ::core::ffi::c_char,
            evaluator_stats.pending_persistence as ::core::ffi::c_double,
        );
        cJSON_AddItemToObject(
            root,
            b"evaluator\0".as_ptr() as *const ::core::ffi::c_char,
            evaluator,
        );
    }
    append_self_observability(
        root,
        &raw mut sampler_stats,
        &raw mut evaluator_stats,
        &raw mut incidents as *mut system_health_incident_view_t,
        incident_count,
    );
    send_json(res, root);
}
unsafe extern "C" fn parse_limit(
    mut req: *const http_request_t,
    mut limit: *mut ::core::ffi::c_int,
    mut res: *mut http_response_t,
) -> bool {
    let mut value: [::core::ffi::c_char; 32] = [0; 32];
    if http_request_get_query_param(
        req,
        b"limit\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    ) < 0 as ::core::ffi::c_int
    {
        *limit = SYSTEM_HEALTH_INCIDENT_DEFAULT_LIMIT;
        return r#true != 0;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parsed: ::core::ffi::c_long = strtol(
        &raw mut value as *mut ::core::ffi::c_char,
        &raw mut end,
        10 as ::core::ffi::c_int,
    );
    if *__errno_location() == ERANGE
        || end.is_null()
        || end == &raw mut value as *mut ::core::ffi::c_char
        || *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || parsed < 1 as ::core::ffi::c_long
        || parsed > SYSTEM_HEALTH_INCIDENT_PAGE_MAX as ::core::ffi::c_long
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"limit must be between 1 and 100\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *limit = parsed as ::core::ffi::c_int;
    return r#true != 0;
}
unsafe extern "C" fn parse_include_closed(
    mut req: *const http_request_t,
    mut value: *mut bool,
    mut res: *mut http_response_t,
) -> bool {
    let mut text: [::core::ffi::c_char; 16] = [0; 16];
    if http_request_get_query_param(
        req,
        b"include_closed\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut text as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
    ) < 0 as ::core::ffi::c_int
    {
        *value = r#true != 0;
        return r#true != 0;
    }
    if strcmp(
        &raw mut text as *mut ::core::ffi::c_char,
        b"true\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            &raw mut text as *mut ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        *value = r#true != 0;
        return r#true != 0;
    }
    if strcmp(
        &raw mut text as *mut ::core::ffi::c_char,
        b"false\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            &raw mut text as *mut ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        *value = r#false != 0;
        return r#true != 0;
    }
    http_response_set_json_error(
        res,
        400 as ::core::ffi::c_int,
        b"include_closed must be true or false\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return r#false != 0;
}
unsafe extern "C" fn parse_cursor(
    mut req: *const http_request_t,
    mut cursor: *mut system_health_incident_cursor_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut uuid: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut timestamp: ::core::ffi::c_longlong = 0;
    let mut value: [::core::ffi::c_char; 96] = [0; 96];
    memset(
        cursor as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_incident_cursor_t>(),
    );
    if http_request_get_query_param(
        req,
        b"cursor\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
    ) < 0 as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    if strncmp(
        &raw mut value as *mut ::core::ffi::c_char,
        b"v1:\0".as_ptr() as *const ::core::ffi::c_char,
        3 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        uuid = strchr(
            (&raw mut value as *mut ::core::ffi::c_char).offset(3 as ::core::ffi::c_int as isize),
            ':' as ::core::ffi::c_int,
        );
        if !uuid.is_null() {
            let c2rust_fresh0 = uuid;
            uuid = uuid.offset(1);
            *c2rust_fresh0 = '\0' as ::core::ffi::c_char;
            *__errno_location() = 0 as ::core::ffi::c_int;
            end = ::core::ptr::null_mut::<::core::ffi::c_char>();
            timestamp = strtoll(
                (&raw mut value as *mut ::core::ffi::c_char)
                    .offset(3 as ::core::ffi::c_int as isize),
                &raw mut end,
                10 as ::core::ffi::c_int,
            );
            if !(*__errno_location() == ERANGE
                || end.is_null()
                || *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                || timestamp <= 0 as ::core::ffi::c_longlong
                || !lightnvr_uuid_is_valid(uuid))
            {
                (*cursor).valid = r#true != 0;
                (*cursor).last_seen_at_ms = timestamp as int64_t;
                snprintf(
                    &raw mut (*cursor).uuid as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    uuid,
                );
                return r#true != 0;
            }
        }
    }
    http_response_set_json_error(
        res,
        400 as ::core::ffi::c_int,
        b"Invalid incident cursor\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return r#false != 0;
}
unsafe extern "C" fn safe_stored_observation(
    mut incident: *const system_health_incident_record_t,
) -> *mut cJSON {
    let mut safe: *mut cJSON = cJSON_CreateObject();
    let mut stored: *mut cJSON = cJSON_ParseWithOpts(
        &raw const (*incident).observation_json as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        r#true,
    );
    if safe.is_null() {
        cJSON_Delete(stored);
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut metric: *const cJSON = if cJSON_IsObject(stored) != 0 {
        cJSON_GetObjectItemCaseSensitive(stored, b"metric\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut value: *const cJSON = if cJSON_IsObject(stored) != 0 {
        cJSON_GetObjectItemCaseSensitive(stored, b"value\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut unit: *const cJSON = if cJSON_IsObject(stored) != 0 {
        cJSON_GetObjectItemCaseSensitive(stored, b"unit\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut capability: *const cJSON = if cJSON_IsObject(stored) != 0 {
        cJSON_GetObjectItemCaseSensitive(
            stored,
            b"capability\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut safe_metric: *const ::core::ffi::c_char =
        &raw const (*incident).condition_code as *const ::core::ffi::c_char;
    if cJSON_IsString(metric) != 0 {
        let mut length: size_t =
            strnlen((*metric).valuestring, SYSTEM_HEALTH_METRIC_LENGTH as size_t);
        let mut logical: bool =
            length > 0 as size_t && length < SYSTEM_HEALTH_METRIC_LENGTH as size_t;
        let mut index: size_t = 0 as size_t;
        while logical as ::core::ffi::c_int != 0 && index < length {
            let mut current: ::core::ffi::c_uchar =
                *(*metric).valuestring.offset(index as isize) as ::core::ffi::c_uchar;
            logical = current as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                && current as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
                || current as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && current as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
                || current as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && current as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                || current as ::core::ffi::c_int == '_' as ::core::ffi::c_int
                || current as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                || current as ::core::ffi::c_int == ':' as ::core::ffi::c_int
                || current as ::core::ffi::c_int == '-' as ::core::ffi::c_int;
            index = index.wrapping_add(1);
        }
        if logical {
            safe_metric = (*metric).valuestring;
        }
    }
    let mut safe_unit: *const ::core::ffi::c_char =
        b"none\0".as_ptr() as *const ::core::ffi::c_char;
    if cJSON_IsString(unit) != 0 {
        let mut candidate: ::core::ffi::c_int =
            system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE.0 as ::core::ffi::c_int;
        while candidate <= system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN.0 as ::core::ffi::c_int
        {
            let mut name: *const ::core::ffi::c_char =
                system_health_unit_name(system_health_unit_t(candidate as ::core::ffi::c_uint));
            if strcmp(name, (*unit).valuestring) == 0 as ::core::ffi::c_int {
                safe_unit = name;
                break;
            } else {
                candidate += 1;
            }
        }
    }
    let mut safe_capability: *const ::core::ffi::c_char = if cJSON_IsNumber(value) != 0 {
        b"available\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"error\0".as_ptr() as *const ::core::ffi::c_char
    };
    if cJSON_IsString(capability) != 0 {
        let mut candidate_0: ::core::ffi::c_int =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0 as ::core::ffi::c_int;
        while candidate_0
            < system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_COUNT.0 as ::core::ffi::c_int
        {
            let mut name_0: *const ::core::ffi::c_char = system_health_capability_name(
                system_health_capability_t(candidate_0 as ::core::ffi::c_uint),
            );
            if strcmp(name_0, (*capability).valuestring) == 0 as ::core::ffi::c_int {
                safe_capability = name_0;
                break;
            } else {
                candidate_0 += 1;
            }
        }
    }
    cJSON_AddStringToObject(
        safe,
        b"metric\0".as_ptr() as *const ::core::ffi::c_char,
        safe_metric,
    );
    cJSON_AddStringToObject(
        safe,
        b"resource\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*incident).subject as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        safe,
        b"scope\0".as_ptr() as *const ::core::ffi::c_char,
        system_health_scope_name((*incident).scope),
    );
    if cJSON_IsNumber(value) != 0 && (*value).valuedouble.is_finite() as ::core::ffi::c_int != 0 {
        cJSON_AddNumberToObject(
            safe,
            b"value\0".as_ptr() as *const ::core::ffi::c_char,
            (*value).valuedouble,
        );
    } else {
        cJSON_AddNullToObject(safe, b"value\0".as_ptr() as *const ::core::ffi::c_char);
    }
    cJSON_AddStringToObject(
        safe,
        b"unit\0".as_ptr() as *const ::core::ffi::c_char,
        safe_unit,
    );
    cJSON_AddStringToObject(
        safe,
        b"capability\0".as_ptr() as *const ::core::ffi::c_char,
        safe_capability,
    );
    cJSON_Delete(stored);
    return safe;
}
unsafe extern "C" fn stored_incident_to_json(
    mut incident: *const system_health_incident_record_t,
) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_CreateObject();
    let mut observation: *mut cJSON = safe_stored_observation(incident);
    if item.is_null() || observation.is_null() {
        cJSON_Delete(item);
        cJSON_Delete(observation);
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        item,
        b"incident_id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*incident).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        item,
        b"condition\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*incident).condition_code as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        item,
        b"subject\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*incident).subject as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        item,
        b"scope\0".as_ptr() as *const ::core::ffi::c_char,
        system_health_scope_name((*incident).scope),
    );
    cJSON_AddStringToObject(
        item,
        b"state\0".as_ptr() as *const ::core::ffi::c_char,
        state_name((*incident).state),
    );
    cJSON_AddStringToObject(
        item,
        b"severity\0".as_ptr() as *const ::core::ffi::c_char,
        system_health_severity_name((*incident).severity),
    );
    cJSON_AddNumberToObject(
        item,
        b"first_observed_at_ms\0".as_ptr() as *const ::core::ffi::c_char,
        (*incident).first_seen_at_ms as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        item,
        b"last_observed_at_ms\0".as_ptr() as *const ::core::ffi::c_char,
        (*incident).last_seen_at_ms as ::core::ffi::c_double,
    );
    if (*incident).closed_at_ms > 0 as int64_t {
        cJSON_AddNumberToObject(
            item,
            b"closed_at_ms\0".as_ptr() as *const ::core::ffi::c_char,
            (*incident).closed_at_ms as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(
            item,
            b"closed_at_ms\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddStringToObject(
        item,
        b"reconciliation\0".as_ptr() as *const ::core::ffi::c_char,
        reconciliation_name((*incident).reconciliation),
    );
    cJSON_AddNumberToObject(
        item,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        (*incident).revision as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        item,
        b"observation\0".as_ptr() as *const ::core::ffi::c_char,
        observation,
    );
    return item;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_system_health_incidents(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_SYSTEM_ADMIN) == 0 {
        return;
    }
    let mut limit: ::core::ffi::c_int = 0;
    let mut include_closed: bool = false;
    let mut cursor: system_health_incident_cursor_t = system_health_incident_cursor_t {
        valid: false,
        last_seen_at_ms: 0,
        uuid: [0; 37],
    };
    if !parse_limit(req, &raw mut limit, res)
        || !parse_include_closed(req, &raw mut include_closed, res)
        || !parse_cursor(req, &raw mut cursor, res)
    {
        return;
    }
    let mut records: *mut system_health_incident_record_t = calloc(
        limit as size_t,
        ::core::mem::size_of::<system_health_incident_record_t>(),
    )
        as *mut system_health_incident_record_t;
    if records.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut next: system_health_incident_cursor_t = system_health_incident_cursor_t {
        valid: false,
        last_seen_at_ms: 0,
        uuid: [0; 37],
    };
    let mut count: ::core::ffi::c_int = db_system_health_incident_list(
        include_closed,
        if cursor.valid as ::core::ffi::c_int != 0 {
            &raw mut cursor
        } else {
            ::core::ptr::null_mut::<system_health_incident_cursor_t>()
        },
        records,
        limit,
        &raw mut next,
    );
    if count < 0 as ::core::ffi::c_int {
        free(records as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to read incident history\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || items.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(records as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build incident history\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddNumberToObject(
        root,
        b"schema_version\0".as_ptr() as *const ::core::ffi::c_char,
        SYSTEM_HEALTH_API_SCHEMA_VERSION as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        count as ::core::ffi::c_double,
    );
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut item: *mut cJSON = stored_incident_to_json(records.offset(index as isize));
        if !item.is_null() {
            cJSON_AddItemToArray(items, item);
        }
        index += 1;
    }
    free(records as *mut ::core::ffi::c_void);
    cJSON_AddItemToObject(
        root,
        b"incidents\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    if next.valid {
        let mut encoded: [::core::ffi::c_char; 96] = [0; 96];
        snprintf(
            &raw mut encoded as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
            b"v1:%lld:%s\0".as_ptr() as *const ::core::ffi::c_char,
            next.last_seen_at_ms as ::core::ffi::c_longlong,
            &raw mut next.uuid as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            root,
            b"next_cursor\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut encoded as *mut ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            root,
            b"next_cursor\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    send_json(res, root);
}
