unsafe extern "C" {
    unsafe fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    unsafe fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_AddItemToObject(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        item: *mut cJSON,
    ) -> cJSON_bool;
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
    unsafe fn cJSON_AddArrayToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn event_registry_find(
        r#type: *const ::core::ffi::c_char,
    ) -> *const event_type_definition_t;
    unsafe fn event_envelope_create_with_severity_and_id(
        event: *mut event_envelope_t,
        r#type: *const ::core::ffi::c_char,
        source: *const ::core::ffi::c_char,
        subject: *const ::core::ffi::c_char,
        occurred_at: time_t,
        data: *const cJSON,
        severity: event_severity_t,
        event_id: *const ::core::ffi::c_char,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn event_envelope_clear(event: *mut event_envelope_t);
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
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut g_config: config_t;
    unsafe fn event_bus_publish(
        event: *const event_envelope_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> event_bus_result_t;
    unsafe fn event_identity_get_source(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn event_router_evaluate_delivery(
        event: *const event_envelope_t,
        plan: *mut event_route_delivery_plan_t,
    ) -> event_router_result_t;
    unsafe fn event_router_record_destination_enqueued(
        event: *const event_envelope_t,
        plan: *const event_route_delivery_plan_t,
        destination_key: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn event_route_delivery_plan_clear(plan: *mut event_route_delivery_plan_t);
    unsafe fn mqtt_delivery_worker_enqueue(
        event: *const event_envelope_t,
        topic_prefix: *const ::core::ffi::c_char,
        row_id: *mut int64_t,
    ) -> event_outbox_enqueue_result_t;
    unsafe fn mqtt_delivery_worker_enqueue_destination(
        event: *const event_envelope_t,
        destination: *const ::core::ffi::c_char,
        topic_template: *const ::core::ffi::c_char,
        row_id: *mut int64_t,
    ) -> event_outbox_enqueue_result_t;
    unsafe fn get_stream_config_by_name(
        name: *const ::core::ffi::c_char,
        stream: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_severity_t(pub ::core::ffi::c_uint);
impl event_severity_t {
    pub const EVENT_SEVERITY_INFO: Self = Self(0);
    pub const EVENT_SEVERITY_WARNING: Self = Self(1);
    pub const EVENT_SEVERITY_ERROR: Self = Self(2);
    pub const EVENT_SEVERITY_CRITICAL: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_sensitivity_t(pub ::core::ffi::c_uint);
impl event_sensitivity_t {
    pub const EVENT_SENSITIVITY_OPERATIONAL: Self = Self(0);
    pub const EVENT_SENSITIVITY_INTERNAL: Self = Self(1);
    pub const EVENT_SENSITIVITY_RESTRICTED: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_media_policy_t(pub ::core::ffi::c_uint);
impl event_media_policy_t {
    pub const EVENT_MEDIA_FORBIDDEN: Self = Self(0);
    pub const EVENT_MEDIA_REFERENCE_ALLOWED: Self = Self(1);
    pub const EVENT_MEDIA_BINARY_OPT_IN: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_expected_rate_t(pub ::core::ffi::c_uint);
impl event_expected_rate_t {
    pub const EVENT_RATE_LOW: Self = Self(0);
    pub const EVENT_RATE_MEDIUM: Self = Self(1);
    pub const EVENT_RATE_HIGH: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_subject_kind_t(pub ::core::ffi::c_uint);
impl event_subject_kind_t {
    pub const EVENT_SUBJECT_CAMERA: Self = Self(0);
    pub const EVENT_SUBJECT_STORAGE: Self = Self(1);
    pub const EVENT_SUBJECT_SYSTEM: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_type_definition_t {
    pub r#type: *const ::core::ffi::c_char,
    pub family: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub severity: event_severity_t,
    pub dynamic_severity: bool,
    pub minimum_severity: event_severity_t,
    pub maximum_severity: event_severity_t,
    pub sensitivity: event_sensitivity_t,
    pub media_policy: event_media_policy_t,
    pub expected_rate: event_expected_rate_t,
    pub subject_kind: event_subject_kind_t,
    pub default_expiry_seconds: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_envelope_t {
    pub specversion: [::core::ffi::c_char; 4],
    pub id: [::core::ffi::c_char; 37],
    pub r#type: [::core::ffi::c_char; 96],
    pub source: [::core::ffi::c_char; 128],
    pub subject: [::core::ffi::c_char; 96],
    pub time: [::core::ffi::c_char; 32],
    pub datacontenttype: [::core::ffi::c_char; 32],
    pub occurred_at: time_t,
    pub expires_at: time_t,
    pub severity: event_severity_t,
    pub data: *mut cJSON,
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_incident_action_t(pub ::core::ffi::c_uint);
impl system_health_incident_action_t {
    pub const SYSTEM_HEALTH_INCIDENT_OPEN: Self = Self(0);
    pub const SYSTEM_HEALTH_INCIDENT_ESCALATE: Self = Self(1);
    pub const SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE: Self = Self(2);
    pub const SYSTEM_HEALTH_INCIDENT_RECOVER: Self = Self(3);
    pub const SYSTEM_HEALTH_INCIDENT_ONE_SHOT: Self = Self(4);
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
pub struct system_health_transition_t {
    pub action: system_health_incident_action_t,
    pub incident_id: [::core::ffi::c_char; 37],
    pub event_id: [::core::ffi::c_char; 37],
    pub condition: system_health_condition_t,
    pub subject: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub previous_state: system_health_state_t,
    pub state: system_health_state_t,
    pub previous_severity: system_health_severity_t,
    pub severity: system_health_severity_t,
    pub observation: system_health_observation_t,
    pub threshold_direction: system_health_threshold_direction_t,
    pub threshold_value: ::core::ffi::c_double,
    pub threshold_for_ms: uint32_t,
    pub first_observed_at_ms: int64_t,
    pub incident_duration_ms: uint64_t,
    pub observed_at_ms: int64_t,
    pub persisted: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_t {
    pub label: [::core::ffi::c_char; 32],
    pub confidence: ::core::ffi::c_float,
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub width: ::core::ffi::c_float,
    pub height: ::core::ffi::c_float,
    pub track_id: ::core::ffi::c_int,
    pub zone_id: [::core::ffi::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_result_t {
    pub count: ::core::ffi::c_int,
    pub detections: [detection_t; 20],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_producer_enqueue_result_t(pub ::core::ffi::c_int);
impl event_producer_enqueue_result_t {
    pub const EVENT_PRODUCER_ENQUEUE_FAILED: Self = Self(-1);
    pub const EVENT_PRODUCER_ENQUEUE_ENQUEUED: Self = Self(0);
    pub const EVENT_PRODUCER_ENQUEUE_DUPLICATE: Self = Self(1);
    pub const EVENT_PRODUCER_ENQUEUE_NO_ROUTE: Self = Self(2);
    pub const EVENT_PRODUCER_ENQUEUE_MQTT_DISABLED: Self = Self(3);
    pub const EVENT_PRODUCER_ENQUEUE_FULL: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_route_delivery_plan_t {
    pub event_id: [::core::ffi::c_char; 37],
    pub event_type: [::core::ffi::c_char; 96],
    pub subject: [::core::ffi::c_char; 96],
    pub entries: *mut event_route_delivery_plan_entry_t,
    pub count: size_t,
    pub capacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_route_delivery_plan_entry_t {
    pub route_uuid: [::core::ffi::c_char; 37],
    pub route_revision: int64_t,
    pub destination_key: [::core::ffi::c_char; 128],
    pub topic_template: [::core::ffi::c_char; 512],
    pub suppression_pending: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_outbox_enqueue_result_t(pub ::core::ffi::c_int);
impl event_outbox_enqueue_result_t {
    pub const EVENT_OUTBOX_ENQUEUED: Self = Self(0);
    pub const EVENT_OUTBOX_DUPLICATE: Self = Self(1);
    pub const EVENT_OUTBOX_FULL: Self = Self(2);
    pub const EVENT_OUTBOX_ERROR: Self = Self(-1);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_engine_config_t {
    pub enabled: bool,
    pub num_threads: ::core::ffi::c_int,
    pub delegate: [::core::ffi::c_char; 16],
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_protocol_t(pub ::core::ffi::c_uint);
impl stream_protocol_t {
    pub const STREAM_PROTOCOL_TCP: Self = Self(0);
    pub const STREAM_PROTOCOL_UDP: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_router_result_t(pub ::core::ffi::c_int);
impl event_router_result_t {
    pub const EVENT_ROUTER_ERROR: Self = Self(-1);
    pub const EVENT_ROUTER_NO_MATCH: Self = Self(0);
    pub const EVENT_ROUTER_MATCH: Self = Self(1);
    pub const EVENT_ROUTER_DEFAULT: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_bus_result_t(pub ::core::ffi::c_int);
impl event_bus_result_t {
    pub const EVENT_BUS_OK: Self = Self(0);
    pub const EVENT_BUS_NOT_RUNNING: Self = Self(-1);
    pub const EVENT_BUS_INVALID_EVENT: Self = Self(-2);
    pub const EVENT_BUS_QUEUE_FULL: Self = Self(-3);
    pub const EVENT_BUS_ALLOCATION_FAILED: Self = Self(-4);
}
pub const EVENT_SUBJECT_MAX: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const EVENT_TIME_MAX: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const SYSTEM_HEALTH_ID_LENGTH: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const MAX_DETECTIONS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const EVENT_ROUTE_DEFAULT_DESTINATION: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"mqtt:default\0") };
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DETECTION_EVENT_TYPE: [::core::ffi::c_char; 32] = unsafe {
    ::core::mem::transmute::<[u8; 32], [::core::ffi::c_char; 32]>(
        *b"io.lightnvr.detection.object.v1\0",
    )
};
pub const LPR_READ_EVENT_TYPE: [::core::ffi::c_char; 41] = unsafe {
    ::core::mem::transmute::<[u8; 41], [::core::ffi::c_char; 41]>(
        *b"io.lightnvr.recognition.license_plate.v1\0",
    )
};
pub const CAMERA_OFFLINE_EVENT_TYPE: [::core::ffi::c_char; 30] = unsafe {
    ::core::mem::transmute::<[u8; 30], [::core::ffi::c_char; 30]>(
        *b"io.lightnvr.camera.offline.v1\0",
    )
};
pub const CAMERA_RECOVERED_EVENT_TYPE: [::core::ffi::c_char; 32] = unsafe {
    ::core::mem::transmute::<[u8; 32], [::core::ffi::c_char; 32]>(
        *b"io.lightnvr.camera.recovered.v1\0",
    )
};
pub const STREAM_DEGRADED_EVENT_TYPE: [::core::ffi::c_char; 31] = unsafe {
    ::core::mem::transmute::<[u8; 31], [::core::ffi::c_char; 31]>(
        *b"io.lightnvr.stream.degraded.v1\0",
    )
};
pub const STREAM_RECOVERED_EVENT_TYPE: [::core::ffi::c_char; 32] = unsafe {
    ::core::mem::transmute::<[u8; 32], [::core::ffi::c_char; 32]>(
        *b"io.lightnvr.stream.recovered.v1\0",
    )
};
pub const RECORDING_GAP_EVENT_TYPE: [::core::ffi::c_char; 36] = unsafe {
    ::core::mem::transmute::<[u8; 36], [::core::ffi::c_char; 36]>(
        *b"io.lightnvr.stream.recording_gap.v1\0",
    )
};
pub const STORAGE_PRESSURE_EVENT_TYPE: [::core::ffi::c_char; 32] = unsafe {
    ::core::mem::transmute::<[u8; 32], [::core::ffi::c_char; 32]>(
        *b"io.lightnvr.storage.pressure.v1\0",
    )
};
pub const STORAGE_RECOVERED_EVENT_TYPE: [::core::ffi::c_char; 33] = unsafe {
    ::core::mem::transmute::<[u8; 33], [::core::ffi::c_char; 33]>(
        *b"io.lightnvr.storage.recovered.v1\0",
    )
};
pub const STORAGE_TARGET_UNAVAILABLE_EVENT_TYPE: [::core::ffi::c_char; 42] = unsafe {
    ::core::mem::transmute::<[u8; 42], [::core::ffi::c_char; 42]>(
        *b"io.lightnvr.storage.target_unavailable.v1\0",
    )
};
pub const STORAGE_TARGET_RECOVERED_EVENT_TYPE: [::core::ffi::c_char; 40] = unsafe {
    ::core::mem::transmute::<[u8; 40], [::core::ffi::c_char; 40]>(
        *b"io.lightnvr.storage.target_recovered.v1\0",
    )
};
pub const SYSTEM_HEALTH_ALERT_EVENT_TYPE: [::core::ffi::c_char; 35] = unsafe {
    ::core::mem::transmute::<[u8; 35], [::core::ffi::c_char; 35]>(
        *b"io.lightnvr.system.health_alert.v1\0",
    )
};
pub const SYSTEM_HEALTH_RECOVERED_EVENT_TYPE: [::core::ffi::c_char; 39] = unsafe {
    ::core::mem::transmute::<[u8; 39], [::core::ffi::c_char; 39]>(
        *b"io.lightnvr.system.health_recovered.v1\0",
    )
};
unsafe extern "C" fn set_error(
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
    mut message: *const ::core::ffi::c_char,
) {
    if error.is_null() || error_size == 0 as size_t {
        return;
    }
    snprintf(
        error,
        error_size,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        if !message.is_null() {
            message
        } else {
            b"event producer error\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
}
unsafe extern "C" fn valid_text(
    mut value: *const ::core::ffi::c_char,
    mut maximum: size_t,
) -> bool {
    return !value.is_null()
        && *value.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && strnlen(value, maximum) < maximum;
}
unsafe extern "C" fn publish_event_with_severity_and_id(
    mut r#type: *const ::core::ffi::c_char,
    mut subject: *const ::core::ffi::c_char,
    mut occurred_at: time_t,
    mut data: *const cJSON,
    mut severity: event_severity_t,
    mut event_id: *const ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    let mut source: [::core::ffi::c_char; 128] = [0; 128];
    if event_identity_get_source(
        &raw mut source as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    ) != 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"event identity is not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut event: event_envelope_t = event_envelope_t {
        specversion: [0; 4],
        id: [0; 37],
        r#type: [0; 96],
        source: [0; 128],
        subject: [0; 96],
        time: [0; 32],
        datacontenttype: [0; 32],
        occurred_at: 0,
        expires_at: 0,
        severity: event_severity_t::EVENT_SEVERITY_INFO,
        data: ::core::ptr::null_mut::<cJSON>(),
    };
    if event_envelope_create_with_severity_and_id(
        &raw mut event,
        r#type,
        &raw mut source as *mut ::core::ffi::c_char,
        subject,
        occurred_at,
        data,
        severity,
        event_id,
        error,
        error_size,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut publish_result: event_bus_result_t =
        event_bus_publish(&raw mut event, error, error_size);
    event_envelope_clear(&raw mut event);
    return if publish_result.0 == event_bus_result_t::EVENT_BUS_OK.0 {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn publish_event(
    mut r#type: *const ::core::ffi::c_char,
    mut subject: *const ::core::ffi::c_char,
    mut occurred_at: time_t,
    mut data: *const cJSON,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    let mut definition: *const event_type_definition_t = event_registry_find(r#type);
    if definition.is_null() {
        set_error(
            error,
            error_size,
            b"event type is not registered\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return publish_event_with_severity_and_id(
        r#type,
        subject,
        occurred_at,
        data,
        (*definition).severity,
        ::core::ptr::null::<::core::ffi::c_char>(),
        error,
        error_size,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_producer_enqueue_result_name(
    mut result: event_producer_enqueue_result_t,
) -> *const ::core::ffi::c_char {
    match result {
        event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_FAILED => {
            return b"failed\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_ENQUEUED => {
            return b"enqueued\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_DUPLICATE => {
            return b"duplicate\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_NO_ROUTE => {
            return b"no_route\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_MQTT_DISABLED => {
            return b"mqtt_disabled\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_FULL => {
            return b"full\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_producer_enqueue_durable(
    mut event: *const event_envelope_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> event_producer_enqueue_result_t {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if event.is_null() {
        set_error(
            error,
            error_size,
            b"event is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_FAILED;
    }
    let mut plan: event_route_delivery_plan_t = event_route_delivery_plan_t {
        event_id: [
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
        ],
        event_type: [0; 96],
        subject: [0; 96],
        entries: ::core::ptr::null_mut::<event_route_delivery_plan_entry_t>(),
        count: 0,
        capacity: 0,
    };
    let mut route_result: event_router_result_t =
        event_router_evaluate_delivery(event, &raw mut plan);
    if route_result.0 == event_router_result_t::EVENT_ROUTER_ERROR.0 {
        event_route_delivery_plan_clear(&raw mut plan);
        set_error(
            error,
            error_size,
            b"event route evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_FAILED;
    }
    if route_result.0 == event_router_result_t::EVENT_ROUTER_DEFAULT.0 {
        if !g_config.mqtt_enabled {
            event_route_delivery_plan_clear(&raw mut plan);
            return event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_MQTT_DISABLED;
        }
        let mut result: event_outbox_enqueue_result_t = mqtt_delivery_worker_enqueue(
            event,
            &raw mut g_config.mqtt_topic_prefix as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<int64_t>(),
        );
        event_route_delivery_plan_clear(&raw mut plan);
        if result.0 == event_outbox_enqueue_result_t::EVENT_OUTBOX_ENQUEUED.0 {
            return event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_ENQUEUED;
        }
        if result.0 == event_outbox_enqueue_result_t::EVENT_OUTBOX_DUPLICATE.0 {
            return event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_DUPLICATE;
        }
        set_error(
            error,
            error_size,
            b"durable event outbox enqueue failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return event_producer_enqueue_result_t(
            if result.0 == event_outbox_enqueue_result_t::EVENT_OUTBOX_FULL.0 {
                event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_FULL.0
            } else {
                event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_FAILED.0
            },
        );
    }
    if route_result.0 == event_router_result_t::EVENT_ROUTER_NO_MATCH.0 {
        event_route_delivery_plan_clear(&raw mut plan);
        return event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_NO_ROUTE;
    }
    let mut failed: bool = r#false != 0;
    let mut any_error: bool = r#false != 0;
    let mut any_enqueued: bool = r#false != 0;
    let mut any_duplicate: bool = r#false != 0;
    let mut destination_accepted: [bool; 512] = [
        r#false != 0,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    ];
    let mut index: size_t = 0 as size_t;
    while index < plan.count {
        let mut entry: *const event_route_delivery_plan_entry_t =
            plan.entries.offset(index as isize);
        let mut duplicate_destination: bool = r#false != 0;
        let mut previous: size_t = 0 as size_t;
        while previous < index {
            if strcmp(
                &raw mut (*plan.entries.offset(previous as isize)).destination_key
                    as *mut ::core::ffi::c_char,
                &raw const (*entry).destination_key as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                duplicate_destination = r#true != 0;
                break;
            } else {
                previous = previous.wrapping_add(1);
            }
        }
        's_113: {
            if !duplicate_destination {
                let mut result_0: event_outbox_enqueue_result_t =
                    event_outbox_enqueue_result_t::EVENT_OUTBOX_ENQUEUED;
                if strcmp(
                    &raw const (*entry).destination_key as *const ::core::ffi::c_char,
                    EVENT_ROUTE_DEFAULT_DESTINATION.as_ptr(),
                ) == 0 as ::core::ffi::c_int
                {
                    if !g_config.mqtt_enabled {
                        break 's_113;
                    } else {
                        result_0 = mqtt_delivery_worker_enqueue(
                            event,
                            &raw mut g_config.mqtt_topic_prefix as *mut ::core::ffi::c_char,
                            ::core::ptr::null_mut::<int64_t>(),
                        );
                    }
                } else {
                    result_0 = mqtt_delivery_worker_enqueue_destination(
                        event,
                        &raw const (*entry).destination_key as *const ::core::ffi::c_char,
                        &raw const (*entry).topic_template as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<int64_t>(),
                    );
                }
                if result_0.0 != event_outbox_enqueue_result_t::EVENT_OUTBOX_ENQUEUED.0
                    && result_0.0 != event_outbox_enqueue_result_t::EVENT_OUTBOX_DUPLICATE.0
                {
                    failed = r#true != 0;
                    if result_0.0 != event_outbox_enqueue_result_t::EVENT_OUTBOX_FULL.0 {
                        any_error = r#true != 0;
                    }
                } else {
                    destination_accepted[index] = r#true != 0;
                    if result_0.0 == event_outbox_enqueue_result_t::EVENT_OUTBOX_ENQUEUED.0 {
                        any_enqueued = r#true != 0;
                    } else {
                        any_duplicate = r#true != 0;
                    }
                }
            }
        }
        index = index.wrapping_add(1);
    }
    if !failed {
        let mut index_0: size_t = 0 as size_t;
        while index_0 < plan.count {
            let mut entry_0: *const event_route_delivery_plan_entry_t =
                plan.entries.offset(index_0 as isize);
            let mut duplicate_destination_0: bool = r#false != 0;
            let mut previous_0: size_t = 0 as size_t;
            while previous_0 < index_0 {
                if strcmp(
                    &raw mut (*plan.entries.offset(previous_0 as isize)).destination_key
                        as *mut ::core::ffi::c_char,
                    &raw const (*entry_0).destination_key as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    duplicate_destination_0 = r#true != 0;
                    break;
                } else {
                    previous_0 = previous_0.wrapping_add(1);
                }
            }
            if !duplicate_destination_0
                && destination_accepted[index_0] as ::core::ffi::c_int != 0
                && event_router_record_destination_enqueued(
                    event,
                    &raw mut plan,
                    &raw const (*entry_0).destination_key as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
            {
                failed = r#true != 0;
                any_error = r#true != 0;
            }
            index_0 = index_0.wrapping_add(1);
        }
    }
    event_route_delivery_plan_clear(&raw mut plan);
    if failed {
        set_error(
            error,
            error_size,
            b"durable event route enqueue failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return event_producer_enqueue_result_t(if any_error as ::core::ffi::c_int != 0 {
            event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_FAILED.0
        } else {
            event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_FULL.0
        });
    }
    if any_enqueued {
        return event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_ENQUEUED;
    }
    if any_duplicate {
        return event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_DUPLICATE;
    }
    return event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_MQTT_DISABLED;
}
unsafe extern "C" fn durably_enqueue_event(
    mut event: *const event_envelope_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    let mut result: event_producer_enqueue_result_t =
        event_producer_enqueue_durable(event, error, error_size);
    return if result.0 == event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_FAILED.0
        || result.0 == event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_FULL.0
    {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn camera_subject_for_stream(
    mut stream_name: *const ::core::ffi::c_char,
    mut subject: *mut ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !valid_text(stream_name, MAX_STREAM_NAME as size_t) {
        set_error(
            error,
            error_size,
            b"stream name is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: stream_config_t = stream_config_t {
        camera_uuid: [0; 37],
        location_uuid: [0; 37],
        name: [0; 256],
        url: [0; 512],
        enabled: false,
        width: 0,
        height: 0,
        fps: 0,
        codec: [0; 16],
        priority: 0,
        record: false,
        segment_duration: 0,
        detection_based_recording: false,
        detection_model: [0; 512],
        detection_interval: 0,
        detection_threshold: 0.,
        pre_detection_buffer: 0,
        post_detection_buffer: 0,
        detection_api_url: [0; 512],
        detection_object_filter: [0; 16],
        detection_object_filter_list: [0; 256],
        streaming_enabled: false,
        protocol: stream_protocol_t::STREAM_PROTOCOL_TCP,
        record_audio: false,
        audio_voice_enhancement: false,
        onvif_username: [0; 64],
        onvif_password: [0; 64],
        onvif_profile: [0; 64],
        onvif_port: 0,
        onvif_discovery_enabled: false,
        is_onvif: false,
        backchannel_enabled: false,
        retention_days: 0,
        detection_retention_days: 0,
        max_storage_mb: 0,
        tier_critical_multiplier: 0.,
        tier_important_multiplier: 0.,
        tier_ephemeral_multiplier: 0.,
        storage_priority: 0,
        ptz_enabled: false,
        ptz_max_x: 0,
        ptz_max_y: 0,
        ptz_max_z: 0,
        ptz_has_home: false,
        record_on_schedule: false,
        recording_schedule: [0; 168],
        detection_record_on_schedule: false,
        detection_recording_schedule: [0; 168],
        tags: [0; 256],
        admin_url: [0; 512],
        privacy_mode: false,
        motion_trigger_source: [0; 256],
        go2rtc_source_override: [0; 8192],
        sub_stream_url: [0; 512],
        detection_url: [0; 512],
        publish_url: [0; 512],
        playback_transport: [0; 24],
        eptz_config: [0; 1024],
        observation_evidence_enabled: false,
        observation_labels: [0; 512],
        observation_max_age_days: 0,
        observation_max_bytes: 0,
    };
    if get_stream_config_by_name(stream_name, &raw mut stream) != 0 as ::core::ffi::c_int
        || !lightnvr_uuid_is_valid(&raw mut stream.camera_uuid as *mut ::core::ffi::c_char)
    {
        set_error(
            error,
            error_size,
            b"stream camera identity is unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut written: ::core::ffi::c_int = snprintf(
        subject,
        EVENT_SUBJECT_MAX as size_t,
        b"camera/%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream.camera_uuid as *mut ::core::ffi::c_char,
    );
    if written < 0 as ::core::ffi::c_int || written >= EVENT_SUBJECT_MAX {
        set_error(
            error,
            error_size,
            b"camera event subject is too long\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn publish_camera_event_for_stream(
    mut r#type: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut occurred_at: time_t,
    mut data: *const cJSON,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    let mut subject: [::core::ffi::c_char; 96] = [0; 96];
    if camera_subject_for_stream(
        stream_name,
        &raw mut subject as *mut ::core::ffi::c_char,
        error,
        error_size,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    return publish_event(
        r#type,
        &raw mut subject as *mut ::core::ffi::c_char,
        occurred_at,
        data,
        error,
        error_size,
    );
}
unsafe extern "C" fn format_event_time(
    mut value: time_t,
    mut output: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut utc: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    if value <= 0 as time_t || gmtime_r(&raw mut value, &raw mut utc).is_null() {
        return -1 as ::core::ffi::c_int;
    }
    return if strftime(
        output,
        EVENT_TIME_MAX as size_t,
        b"%Y-%m-%dT%H:%M:%SZ\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut utc,
    ) > 0 as size_t
    {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn valid_pressure_level(mut level: *const ::core::ffi::c_char) -> bool {
    return !level.is_null()
        && (strcmp(level, b"warning\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcmp(level, b"critical\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || strcmp(level, b"emergency\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn valid_storage_target_previous_state(
    mut state: *const ::core::ffi::c_char,
) -> bool {
    return !state.is_null()
        && (strcmp(state, b"unknown\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcmp(state, b"healthy\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || strcmp(state, b"degraded\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn valid_storage_target_current_state(
    mut state: *const ::core::ffi::c_char,
) -> bool {
    return !state.is_null()
        && (strcmp(state, b"healthy\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcmp(state, b"degraded\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn valid_storage_target_reason(mut reason: *const ::core::ffi::c_char) -> bool {
    return !reason.is_null()
        && (strcmp(
            reason,
            b"mount_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                reason,
                b"directory_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                reason,
                b"capacity_probe_failed\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                reason,
                b"not_writable\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                reason,
                b"write_probe_failed\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                reason,
                b"probe_cleanup_failed\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(reason, b"unknown\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn add_detection_data(
    mut data: *mut cJSON,
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *const detection_result_t,
) -> bool {
    if cJSON_AddStringToObject(
        data,
        b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    )
    .is_null()
        || cJSON_AddNumberToObject(
            data,
            b"count\0".as_ptr() as *const ::core::ffi::c_char,
            (*result).count as ::core::ffi::c_double,
        )
        .is_null()
    {
        return r#false != 0;
    }
    let mut detections: *mut cJSON =
        cJSON_AddArrayToObject(data, b"detections\0".as_ptr() as *const ::core::ffi::c_char);
    if detections.is_null() {
        return r#false != 0;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < (*result).count {
        let mut source: *const detection_t =
            (&raw const (*result).detections as *const detection_t).offset(index as isize);
        let mut detection: *mut cJSON = cJSON_CreateObject();
        if detection.is_null()
            || cJSON_AddStringToObject(
                detection,
                b"label\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*source).label as *const ::core::ffi::c_char,
            )
            .is_null()
            || cJSON_AddNumberToObject(
                detection,
                b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
                (*source).confidence as ::core::ffi::c_double,
            )
            .is_null()
        {
            cJSON_Delete(detection);
            return r#false != 0;
        }
        if (*source).width > 0.0f32 && (*source).height > 0.0f32 {
            if cJSON_AddNumberToObject(
                detection,
                b"x\0".as_ptr() as *const ::core::ffi::c_char,
                (*source).x as ::core::ffi::c_double,
            )
            .is_null()
                || cJSON_AddNumberToObject(
                    detection,
                    b"y\0".as_ptr() as *const ::core::ffi::c_char,
                    (*source).y as ::core::ffi::c_double,
                )
                .is_null()
                || cJSON_AddNumberToObject(
                    detection,
                    b"width\0".as_ptr() as *const ::core::ffi::c_char,
                    (*source).width as ::core::ffi::c_double,
                )
                .is_null()
                || cJSON_AddNumberToObject(
                    detection,
                    b"height\0".as_ptr() as *const ::core::ffi::c_char,
                    (*source).height as ::core::ffi::c_double,
                )
                .is_null()
            {
                cJSON_Delete(detection);
                return r#false != 0;
            }
        }
        if (*source).track_id >= 0 as ::core::ffi::c_int
            && cJSON_AddNumberToObject(
                detection,
                b"track_id\0".as_ptr() as *const ::core::ffi::c_char,
                (*source).track_id as ::core::ffi::c_double,
            )
            .is_null()
        {
            cJSON_Delete(detection);
            return r#false != 0;
        }
        if (*source).zone_id[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && cJSON_AddStringToObject(
                detection,
                b"zone_id\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*source).zone_id as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            cJSON_Delete(detection);
            return r#false != 0;
        }
        if cJSON_AddItemToArray(detections, detection) == 0 {
            cJSON_Delete(detection);
            return r#false != 0;
        }
        index += 1;
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_producer_publish_detection(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *const detection_result_t,
    mut occurred_at: time_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if !lightnvr_uuid_is_valid(camera_uuid)
        || stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strlen(stream_name) >= MAX_STREAM_NAME as size_t
        || result.is_null()
        || (*result).count <= 0 as ::core::ffi::c_int
        || (*result).count > MAX_DETECTIONS
    {
        set_error(
            error,
            error_size,
            b"detection event input is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut subject: [::core::ffi::c_char; 96] = [0; 96];
    let mut subject_length: ::core::ffi::c_int = snprintf(
        &raw mut subject as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        b"camera/%s\0".as_ptr() as *const ::core::ffi::c_char,
        camera_uuid,
    );
    if subject_length < 0 as ::core::ffi::c_int
        || subject_length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 96]>()
    {
        set_error(
            error,
            error_size,
            b"camera event subject is too long\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut data: *mut cJSON = cJSON_CreateObject();
    if data.is_null() || !add_detection_data(data, stream_name, result) {
        cJSON_Delete(data);
        set_error(
            error,
            error_size,
            b"detection event allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut create_result: ::core::ffi::c_int = publish_event(
        DETECTION_EVENT_TYPE.as_ptr(),
        &raw mut subject as *mut ::core::ffi::c_char,
        occurred_at,
        data,
        error,
        error_size,
    );
    cJSON_Delete(data);
    return create_result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_producer_publish_detection_for_stream(
    mut stream_name: *const ::core::ffi::c_char,
    mut result: *const detection_result_t,
    mut occurred_at: time_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"stream name is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: stream_config_t = stream_config_t {
        camera_uuid: [0; 37],
        location_uuid: [0; 37],
        name: [0; 256],
        url: [0; 512],
        enabled: false,
        width: 0,
        height: 0,
        fps: 0,
        codec: [0; 16],
        priority: 0,
        record: false,
        segment_duration: 0,
        detection_based_recording: false,
        detection_model: [0; 512],
        detection_interval: 0,
        detection_threshold: 0.,
        pre_detection_buffer: 0,
        post_detection_buffer: 0,
        detection_api_url: [0; 512],
        detection_object_filter: [0; 16],
        detection_object_filter_list: [0; 256],
        streaming_enabled: false,
        protocol: stream_protocol_t::STREAM_PROTOCOL_TCP,
        record_audio: false,
        audio_voice_enhancement: false,
        onvif_username: [0; 64],
        onvif_password: [0; 64],
        onvif_profile: [0; 64],
        onvif_port: 0,
        onvif_discovery_enabled: false,
        is_onvif: false,
        backchannel_enabled: false,
        retention_days: 0,
        detection_retention_days: 0,
        max_storage_mb: 0,
        tier_critical_multiplier: 0.,
        tier_important_multiplier: 0.,
        tier_ephemeral_multiplier: 0.,
        storage_priority: 0,
        ptz_enabled: false,
        ptz_max_x: 0,
        ptz_max_y: 0,
        ptz_max_z: 0,
        ptz_has_home: false,
        record_on_schedule: false,
        recording_schedule: [0; 168],
        detection_record_on_schedule: false,
        detection_recording_schedule: [0; 168],
        tags: [0; 256],
        admin_url: [0; 512],
        privacy_mode: false,
        motion_trigger_source: [0; 256],
        go2rtc_source_override: [0; 8192],
        sub_stream_url: [0; 512],
        detection_url: [0; 512],
        publish_url: [0; 512],
        playback_transport: [0; 24],
        eptz_config: [0; 1024],
        observation_evidence_enabled: false,
        observation_labels: [0; 512],
        observation_max_age_days: 0,
        observation_max_bytes: 0,
    };
    if get_stream_config_by_name(stream_name, &raw mut stream) != 0 as ::core::ffi::c_int
        || !lightnvr_uuid_is_valid(&raw mut stream.camera_uuid as *mut ::core::ffi::c_char)
    {
        set_error(
            error,
            error_size,
            b"stream has no immutable camera identity\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return event_producer_publish_detection(
        &raw mut stream.camera_uuid as *mut ::core::ffi::c_char,
        stream_name,
        result,
        occurred_at,
        error,
        error_size,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_producer_publish_lpr_read(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut read_uuid: *const ::core::ffi::c_char,
    mut source: *const ::core::ffi::c_char,
    mut occurred_at: time_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if !lightnvr_uuid_is_valid(camera_uuid)
        || !lightnvr_uuid_is_valid(read_uuid)
        || !valid_text(stream_name, MAX_STREAM_NAME as size_t)
        || !valid_text(source, 64 as size_t)
    {
        set_error(
            error,
            error_size,
            b"protected LPR event input is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut subject: [::core::ffi::c_char; 96] = [0; 96];
    let mut subject_length: ::core::ffi::c_int = snprintf(
        &raw mut subject as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        b"camera/%s\0".as_ptr() as *const ::core::ffi::c_char,
        camera_uuid,
    );
    if subject_length < 0 as ::core::ffi::c_int
        || subject_length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 96]>()
    {
        set_error(
            error,
            error_size,
            b"camera event subject is too long\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut data: *mut cJSON = cJSON_CreateObject();
    if data.is_null()
        || cJSON_AddStringToObject(
            data,
            b"read_id\0".as_ptr() as *const ::core::ffi::c_char,
            read_uuid,
        )
        .is_null()
        || cJSON_AddStringToObject(
            data,
            b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        )
        .is_null()
        || cJSON_AddStringToObject(
            data,
            b"source\0".as_ptr() as *const ::core::ffi::c_char,
            source,
        )
        .is_null()
    {
        cJSON_Delete(data);
        set_error(
            error,
            error_size,
            b"protected LPR event allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = publish_event(
        LPR_READ_EVENT_TYPE.as_ptr(),
        &raw mut subject as *mut ::core::ffi::c_char,
        occurred_at,
        data,
        error,
        error_size,
    );
    cJSON_Delete(data);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_producer_publish_camera_offline_for_stream(
    mut stream_name: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
    mut consecutive_failures: ::core::ffi::c_int,
    mut occurred_at: time_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if !valid_text(stream_name, MAX_STREAM_NAME as size_t)
        || !valid_text(reason, 128 as size_t)
        || consecutive_failures < 1 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"camera offline event input is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut data: *mut cJSON = cJSON_CreateObject();
    if data.is_null()
        || cJSON_AddStringToObject(
            data,
            b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        )
        .is_null()
        || cJSON_AddStringToObject(
            data,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        )
        .is_null()
        || cJSON_AddNumberToObject(
            data,
            b"consecutive_failures\0".as_ptr() as *const ::core::ffi::c_char,
            consecutive_failures as ::core::ffi::c_double,
        )
        .is_null()
    {
        cJSON_Delete(data);
        set_error(
            error,
            error_size,
            b"camera offline event allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = publish_camera_event_for_stream(
        CAMERA_OFFLINE_EVENT_TYPE.as_ptr(),
        stream_name,
        occurred_at,
        data,
        error,
        error_size,
    );
    cJSON_Delete(data);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_producer_publish_camera_recovered_for_stream(
    mut stream_name: *const ::core::ffi::c_char,
    mut downtime_ms: int64_t,
    mut occurred_at: time_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if !valid_text(stream_name, MAX_STREAM_NAME as size_t) || downtime_ms < 0 as int64_t {
        set_error(
            error,
            error_size,
            b"camera recovered event input is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut data: *mut cJSON = cJSON_CreateObject();
    if data.is_null()
        || cJSON_AddStringToObject(
            data,
            b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        )
        .is_null()
        || cJSON_AddStringToObject(
            data,
            b"previous_state\0".as_ptr() as *const ::core::ffi::c_char,
            b"offline\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || cJSON_AddNumberToObject(
            data,
            b"downtime_ms\0".as_ptr() as *const ::core::ffi::c_char,
            downtime_ms as ::core::ffi::c_double,
        )
        .is_null()
    {
        cJSON_Delete(data);
        set_error(
            error,
            error_size,
            b"camera recovered event allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = publish_camera_event_for_stream(
        CAMERA_RECOVERED_EVENT_TYPE.as_ptr(),
        stream_name,
        occurred_at,
        data,
        error,
        error_size,
    );
    cJSON_Delete(data);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_producer_publish_stream_degraded_for_stream(
    mut stream_name: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
    mut observed_fps: ::core::ffi::c_double,
    mut expected_fps: ::core::ffi::c_double,
    mut occurred_at: time_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if !valid_text(stream_name, MAX_STREAM_NAME as size_t)
        || !valid_text(reason, 128 as size_t)
        || observed_fps < 0.0f64
        || expected_fps < 0.0f64
    {
        set_error(
            error,
            error_size,
            b"stream degraded event input is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut data: *mut cJSON = cJSON_CreateObject();
    if data.is_null()
        || cJSON_AddStringToObject(
            data,
            b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        )
        .is_null()
        || cJSON_AddStringToObject(
            data,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        )
        .is_null()
        || cJSON_AddNumberToObject(
            data,
            b"observed_fps\0".as_ptr() as *const ::core::ffi::c_char,
            observed_fps,
        )
        .is_null()
        || cJSON_AddNumberToObject(
            data,
            b"expected_fps\0".as_ptr() as *const ::core::ffi::c_char,
            expected_fps,
        )
        .is_null()
    {
        cJSON_Delete(data);
        set_error(
            error,
            error_size,
            b"stream degraded event allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = publish_camera_event_for_stream(
        STREAM_DEGRADED_EVENT_TYPE.as_ptr(),
        stream_name,
        occurred_at,
        data,
        error,
        error_size,
    );
    cJSON_Delete(data);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_producer_publish_stream_recovered_for_stream(
    mut stream_name: *const ::core::ffi::c_char,
    mut observed_fps: ::core::ffi::c_double,
    mut expected_fps: ::core::ffi::c_double,
    mut occurred_at: time_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if !valid_text(stream_name, MAX_STREAM_NAME as size_t)
        || observed_fps < 0.0f64
        || expected_fps < 0.0f64
    {
        set_error(
            error,
            error_size,
            b"stream recovered event input is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut data: *mut cJSON = cJSON_CreateObject();
    if data.is_null()
        || cJSON_AddStringToObject(
            data,
            b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        )
        .is_null()
        || cJSON_AddStringToObject(
            data,
            b"previous_state\0".as_ptr() as *const ::core::ffi::c_char,
            b"degraded\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || cJSON_AddNumberToObject(
            data,
            b"observed_fps\0".as_ptr() as *const ::core::ffi::c_char,
            observed_fps,
        )
        .is_null()
        || cJSON_AddNumberToObject(
            data,
            b"expected_fps\0".as_ptr() as *const ::core::ffi::c_char,
            expected_fps,
        )
        .is_null()
    {
        cJSON_Delete(data);
        set_error(
            error,
            error_size,
            b"stream recovered event allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = publish_camera_event_for_stream(
        STREAM_RECOVERED_EVENT_TYPE.as_ptr(),
        stream_name,
        occurred_at,
        data,
        error,
        error_size,
    );
    cJSON_Delete(data);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_producer_publish_recording_gap_for_stream(
    mut stream_name: *const ::core::ffi::c_char,
    mut started_at: time_t,
    mut duration_ms: int64_t,
    mut occurred_at: time_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    let mut started_at_text: [::core::ffi::c_char; 32] = [0; 32];
    if !valid_text(stream_name, MAX_STREAM_NAME as size_t)
        || duration_ms < 0 as int64_t
        || format_event_time(
            started_at,
            &raw mut started_at_text as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"recording gap event input is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut data: *mut cJSON = cJSON_CreateObject();
    if data.is_null()
        || cJSON_AddStringToObject(
            data,
            b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        )
        .is_null()
        || cJSON_AddStringToObject(
            data,
            b"started_at\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut started_at_text as *mut ::core::ffi::c_char,
        )
        .is_null()
        || cJSON_AddNumberToObject(
            data,
            b"duration_ms\0".as_ptr() as *const ::core::ffi::c_char,
            duration_ms as ::core::ffi::c_double,
        )
        .is_null()
    {
        cJSON_Delete(data);
        set_error(
            error,
            error_size,
            b"recording gap event allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = publish_camera_event_for_stream(
        RECORDING_GAP_EVENT_TYPE.as_ptr(),
        stream_name,
        occurred_at,
        data,
        error,
        error_size,
    );
    cJSON_Delete(data);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_producer_publish_storage_pressure(
    mut level: *const ::core::ffi::c_char,
    mut previous_level: *const ::core::ffi::c_char,
    mut used_percent: ::core::ffi::c_double,
    mut free_bytes: uint64_t,
    mut occurred_at: time_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if !valid_pressure_level(level)
        || !valid_text(previous_level, 32 as size_t)
        || used_percent < 0.0f64
        || used_percent > 100.0f64
    {
        set_error(
            error,
            error_size,
            b"storage pressure event input is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut data: *mut cJSON = cJSON_CreateObject();
    if data.is_null()
        || cJSON_AddStringToObject(
            data,
            b"level\0".as_ptr() as *const ::core::ffi::c_char,
            level,
        )
        .is_null()
        || cJSON_AddStringToObject(
            data,
            b"previous_level\0".as_ptr() as *const ::core::ffi::c_char,
            previous_level,
        )
        .is_null()
        || cJSON_AddNumberToObject(
            data,
            b"used_percent\0".as_ptr() as *const ::core::ffi::c_char,
            used_percent,
        )
        .is_null()
        || cJSON_AddNumberToObject(
            data,
            b"free_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            free_bytes as ::core::ffi::c_double,
        )
        .is_null()
    {
        cJSON_Delete(data);
        set_error(
            error,
            error_size,
            b"storage pressure event allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut severity: event_severity_t = event_severity_t(
        (if strcmp(level, b"warning\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            event_severity_t::EVENT_SEVERITY_WARNING.0 as ::core::ffi::c_int
        } else {
            event_severity_t::EVENT_SEVERITY_CRITICAL.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
    let mut result: ::core::ffi::c_int = publish_event_with_severity_and_id(
        STORAGE_PRESSURE_EVENT_TYPE.as_ptr(),
        b"system/storage\0".as_ptr() as *const ::core::ffi::c_char,
        occurred_at,
        data,
        severity,
        ::core::ptr::null::<::core::ffi::c_char>(),
        error,
        error_size,
    );
    cJSON_Delete(data);
    return result;
}
unsafe extern "C" fn system_health_event_severity(
    mut health: system_health_severity_t,
    mut event: *mut event_severity_t,
) -> bool {
    if event.is_null() {
        return r#false != 0;
    }
    match health {
        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_WARNING => {
            *event = event_severity_t::EVENT_SEVERITY_WARNING;
            return r#true != 0;
        }
        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_ERROR => {
            *event = event_severity_t::EVENT_SEVERITY_ERROR;
            return r#true != 0;
        }
        system_health_severity_t::SYSTEM_HEALTH_SEVERITY_CRITICAL => {
            *event = event_severity_t::EVENT_SEVERITY_CRITICAL;
            return r#true != 0;
        }
        _ => return r#false != 0,
    };
}
unsafe extern "C" fn system_health_event_subject(
    mut scope: system_health_scope_t,
) -> *const ::core::ffi::c_char {
    match scope {
        system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS => {
            return b"system/process\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_scope_t::SYSTEM_HEALTH_SCOPE_CONTAINER => {
            return b"system/container\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_scope_t::SYSTEM_HEALTH_SCOPE_HOST => {
            return b"system/host\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_scope_t::SYSTEM_HEALTH_SCOPE_FILESYSTEM
        | system_health_scope_t::SYSTEM_HEALTH_SCOPE_DEVICE => {
            return b"system/storage\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return ::core::ptr::null::<::core::ffi::c_char>(),
    };
}
unsafe extern "C" fn system_health_event_state(
    mut action: system_health_incident_action_t,
) -> *const ::core::ffi::c_char {
    match action {
        system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_OPEN
        | system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_ONE_SHOT => {
            return b"open\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_ESCALATE => {
            return b"escalated\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE => {
            return b"updated\0".as_ptr() as *const ::core::ffi::c_char;
        }
        system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER => {
            return b"recovered\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn system_health_observation_json(
    mut observation: *const system_health_observation_t,
) -> *mut cJSON {
    let mut result: *mut cJSON = cJSON_CreateObject();
    if result.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut valid: bool = r#false != 0;
    if (*observation).value_valid as ::core::ffi::c_int != 0
        && (*observation).value.is_finite() as ::core::ffi::c_int != 0
    {
        valid = !cJSON_AddNumberToObject(
            result,
            b"value\0".as_ptr() as *const ::core::ffi::c_char,
            (*observation).value,
        )
        .is_null()
            && !cJSON_AddStringToObject(
                result,
                b"unit\0".as_ptr() as *const ::core::ffi::c_char,
                system_health_unit_name((*observation).unit),
            )
            .is_null();
    } else {
        valid = !cJSON_AddStringToObject(
            result,
            b"capability\0".as_ptr() as *const ::core::ffi::c_char,
            system_health_capability_name((*observation).capability),
        )
        .is_null();
    }
    if !valid {
        cJSON_Delete(result);
        return ::core::ptr::null_mut::<cJSON>();
    }
    return result;
}
unsafe extern "C" fn add_owned_json(
    mut parent: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut item: *mut *mut cJSON,
) -> bool {
    if parent.is_null()
        || name.is_null()
        || item.is_null()
        || (*item).is_null()
        || cJSON_AddItemToObject(parent, name, *item) == 0
    {
        return r#false != 0;
    }
    *item = ::core::ptr::null_mut::<cJSON>();
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_producer_publish_system_health_transition(
    mut transition: *const system_health_transition_t,
    mut event_id: *const ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if transition.is_null()
        || !(*transition).persisted
        || !lightnvr_uuid_is_valid(event_id)
        || !lightnvr_uuid_is_valid(&raw const (*transition).event_id as *const ::core::ffi::c_char)
        || strcmp(
            &raw const (*transition).event_id as *const ::core::ffi::c_char,
            event_id,
        ) != 0 as ::core::ffi::c_int
        || !lightnvr_uuid_is_valid(
            &raw const (*transition).incident_id as *const ::core::ffi::c_char,
        )
        || (*transition).condition.0 < 0 as ::core::ffi::c_uint
        || (*transition).condition.0 >= system_health_condition_t::SYSTEM_HEALTH_CONDITION_COUNT.0
        || !valid_text(
            &raw const (*transition).subject as *const ::core::ffi::c_char,
            SYSTEM_HEALTH_ID_LENGTH as size_t,
        )
        || (*transition).observed_at_ms <= 0 as int64_t
    {
        set_error(
            error,
            error_size,
            b"persisted system health transition input is invalid\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut r#type: *const ::core::ffi::c_char = if (*transition).action.0
        == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER.0
    {
        SYSTEM_HEALTH_RECOVERED_EVENT_TYPE.as_ptr()
    } else {
        SYSTEM_HEALTH_ALERT_EVENT_TYPE.as_ptr()
    };
    let mut subject: *const ::core::ffi::c_char = system_health_event_subject((*transition).scope);
    let mut state: *const ::core::ffi::c_char = system_health_event_state((*transition).action);
    let mut code: *const ::core::ffi::c_char =
        system_health_condition_code((*transition).condition);
    let mut severity: event_severity_t = event_severity_t::EVENT_SEVERITY_INFO;
    let mut payload_severity: system_health_severity_t = system_health_severity_t(
        if (*transition).action.0
            == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER.0
        {
            (*transition).previous_severity.0
        } else {
            (*transition).severity.0
        },
    );
    if subject.is_null()
        || state.is_null()
        || code.is_null()
        || !system_health_event_severity(payload_severity, &raw mut severity)
    {
        set_error(
            error,
            error_size,
            b"system health transition classification is invalid\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut data: *mut cJSON = cJSON_CreateObject();
    let mut observation: *mut cJSON =
        system_health_observation_json(&raw const (*transition).observation);
    let mut valid: bool = !data.is_null()
        && !observation.is_null()
        && !cJSON_AddStringToObject(
            data,
            b"incident_id\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*transition).incident_id as *const ::core::ffi::c_char,
        )
        .is_null()
        && !cJSON_AddStringToObject(data, b"code\0".as_ptr() as *const ::core::ffi::c_char, code)
            .is_null()
        && !cJSON_AddStringToObject(
            data,
            b"scope\0".as_ptr() as *const ::core::ffi::c_char,
            system_health_scope_name((*transition).scope),
        )
        .is_null()
        && !cJSON_AddStringToObject(
            data,
            b"resource\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*transition).subject as *const ::core::ffi::c_char,
        )
        .is_null()
        && !cJSON_AddStringToObject(
            data,
            b"state\0".as_ptr() as *const ::core::ffi::c_char,
            state,
        )
        .is_null();
    if !valid {
        cJSON_Delete(observation);
        cJSON_Delete(data);
        set_error(
            error,
            error_size,
            b"system health event allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*transition).action.0 == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER.0 {
        valid = !cJSON_AddStringToObject(
            data,
            b"previous_severity\0".as_ptr() as *const ::core::ffi::c_char,
            system_health_severity_name(payload_severity),
        )
        .is_null()
            && !cJSON_AddNumberToObject(
                data,
                b"duration_ms\0".as_ptr() as *const ::core::ffi::c_char,
                (*transition).incident_duration_ms as ::core::ffi::c_double,
            )
            .is_null()
            && add_owned_json(
                data,
                b"safe_observation\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut observation,
            ) as ::core::ffi::c_int
                != 0;
    } else {
        let mut threshold: *mut cJSON = cJSON_CreateObject();
        let mut operator_name: *const ::core::ffi::c_char = if (*transition).threshold_direction.0
            == system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE.0
        {
            b"lt\0".as_ptr() as *const ::core::ffi::c_char
        } else if (*transition).threshold_direction.0
            == system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE.0
        {
            b"gt\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"event\0".as_ptr() as *const ::core::ffi::c_char
        };
        let mut first_observed_at: [::core::ffi::c_char; 32] = [0; 32];
        let mut first_observed: time_t =
            ((*transition).first_observed_at_ms / 1000 as int64_t) as time_t;
        let mut threshold_valid: bool = !threshold.is_null()
            && (*transition).threshold_value.is_finite() as ::core::ffi::c_int != 0
            && format_event_time(
                first_observed,
                &raw mut first_observed_at as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            && !cJSON_AddStringToObject(
                threshold,
                b"operator\0".as_ptr() as *const ::core::ffi::c_char,
                operator_name,
            )
            .is_null()
            && !cJSON_AddNumberToObject(
                threshold,
                b"value\0".as_ptr() as *const ::core::ffi::c_char,
                (*transition).threshold_value,
            )
            .is_null()
            && !cJSON_AddNumberToObject(
                threshold,
                b"for_ms\0".as_ptr() as *const ::core::ffi::c_char,
                (*transition).threshold_for_ms as ::core::ffi::c_double,
            )
            .is_null();
        if !threshold_valid {
            cJSON_Delete(threshold);
            valid = r#false != 0;
        } else {
            valid = !cJSON_AddStringToObject(
                data,
                b"severity\0".as_ptr() as *const ::core::ffi::c_char,
                system_health_severity_name(payload_severity),
            )
            .is_null()
                && add_owned_json(
                    data,
                    b"observed\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut observation,
                ) as ::core::ffi::c_int
                    != 0
                && add_owned_json(
                    data,
                    b"threshold\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut threshold,
                ) as ::core::ffi::c_int
                    != 0
                && !cJSON_AddStringToObject(
                    data,
                    b"first_observed_at\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut first_observed_at as *mut ::core::ffi::c_char,
                )
                .is_null();
            cJSON_Delete(threshold);
        }
    }
    if !valid {
        cJSON_Delete(observation);
        cJSON_Delete(data);
        set_error(
            error,
            error_size,
            b"system health event allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*transition).action.0 == system_health_incident_action_t::SYSTEM_HEALTH_INCIDENT_RECOVER.0 {
        severity = event_severity_t::EVENT_SEVERITY_INFO;
    }
    let mut occurred_at: time_t = ((*transition).observed_at_ms / 1000 as int64_t) as time_t;
    let mut source: [::core::ffi::c_char; 128] = [0; 128];
    let mut event: event_envelope_t = event_envelope_t {
        specversion: [0; 4],
        id: [0; 37],
        r#type: [0; 96],
        source: [0; 128],
        subject: [0; 96],
        time: [0; 32],
        datacontenttype: [0; 32],
        occurred_at: 0,
        expires_at: 0,
        severity: event_severity_t::EVENT_SEVERITY_INFO,
        data: ::core::ptr::null_mut::<cJSON>(),
    };
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if event_identity_get_source(
        &raw mut source as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    ) != 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"event identity is not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if event_envelope_create_with_severity_and_id(
        &raw mut event,
        r#type,
        &raw mut source as *mut ::core::ffi::c_char,
        subject,
        occurred_at,
        data,
        severity,
        event_id,
        error,
        error_size,
    ) == 0 as ::core::ffi::c_int
    {
        result = durably_enqueue_event(&raw mut event, error, error_size);
        if result == 0 as ::core::ffi::c_int {
            event_bus_publish(
                &raw mut event,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                0 as size_t,
            );
        }
        event_envelope_clear(&raw mut event);
    }
    cJSON_Delete(data);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_producer_publish_storage_recovered(
    mut previous_level: *const ::core::ffi::c_char,
    mut used_percent: ::core::ffi::c_double,
    mut free_bytes: uint64_t,
    mut occurred_at: time_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if !valid_pressure_level(previous_level) || used_percent < 0.0f64 || used_percent > 100.0f64 {
        set_error(
            error,
            error_size,
            b"storage recovered event input is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut data: *mut cJSON = cJSON_CreateObject();
    if data.is_null()
        || cJSON_AddStringToObject(
            data,
            b"previous_level\0".as_ptr() as *const ::core::ffi::c_char,
            previous_level,
        )
        .is_null()
        || cJSON_AddNumberToObject(
            data,
            b"used_percent\0".as_ptr() as *const ::core::ffi::c_char,
            used_percent,
        )
        .is_null()
        || cJSON_AddNumberToObject(
            data,
            b"free_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            free_bytes as ::core::ffi::c_double,
        )
        .is_null()
    {
        cJSON_Delete(data);
        set_error(
            error,
            error_size,
            b"storage recovered event allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = publish_event(
        STORAGE_RECOVERED_EVENT_TYPE.as_ptr(),
        b"system/storage\0".as_ptr() as *const ::core::ffi::c_char,
        occurred_at,
        data,
        error,
        error_size,
    );
    cJSON_Delete(data);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_producer_publish_storage_target_unavailable(
    mut target_uuid: *const ::core::ffi::c_char,
    mut previous_state: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
    mut is_default: bool,
    mut occurred_at: time_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if !lightnvr_uuid_is_valid(target_uuid)
        || !valid_storage_target_previous_state(previous_state)
        || !valid_storage_target_reason(reason)
    {
        set_error(
            error,
            error_size,
            b"storage target unavailable event input is invalid\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut data: *mut cJSON = cJSON_CreateObject();
    if data.is_null()
        || cJSON_AddStringToObject(
            data,
            b"target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            target_uuid,
        )
        .is_null()
        || cJSON_AddStringToObject(
            data,
            b"previous_state\0".as_ptr() as *const ::core::ffi::c_char,
            previous_state,
        )
        .is_null()
        || cJSON_AddStringToObject(
            data,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        )
        .is_null()
        || cJSON_AddBoolToObject(
            data,
            b"is_default\0".as_ptr() as *const ::core::ffi::c_char,
            is_default as cJSON_bool,
        )
        .is_null()
    {
        cJSON_Delete(data);
        set_error(
            error,
            error_size,
            b"storage target unavailable event allocation failed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = publish_event(
        STORAGE_TARGET_UNAVAILABLE_EVENT_TYPE.as_ptr(),
        b"system/storage\0".as_ptr() as *const ::core::ffi::c_char,
        occurred_at,
        data,
        error,
        error_size,
    );
    cJSON_Delete(data);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_producer_publish_storage_target_recovered(
    mut target_uuid: *const ::core::ffi::c_char,
    mut current_state: *const ::core::ffi::c_char,
    mut downtime_ms: int64_t,
    mut is_default: bool,
    mut occurred_at: time_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if !lightnvr_uuid_is_valid(target_uuid)
        || !valid_storage_target_current_state(current_state)
        || downtime_ms < 0 as int64_t
    {
        set_error(
            error,
            error_size,
            b"storage target recovered event input is invalid\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut data: *mut cJSON = cJSON_CreateObject();
    if data.is_null()
        || cJSON_AddStringToObject(
            data,
            b"target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            target_uuid,
        )
        .is_null()
        || cJSON_AddStringToObject(
            data,
            b"previous_state\0".as_ptr() as *const ::core::ffi::c_char,
            b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || cJSON_AddStringToObject(
            data,
            b"current_state\0".as_ptr() as *const ::core::ffi::c_char,
            current_state,
        )
        .is_null()
        || cJSON_AddNumberToObject(
            data,
            b"downtime_ms\0".as_ptr() as *const ::core::ffi::c_char,
            downtime_ms as ::core::ffi::c_double,
        )
        .is_null()
        || cJSON_AddBoolToObject(
            data,
            b"is_default\0".as_ptr() as *const ::core::ffi::c_char,
            is_default as cJSON_bool,
        )
        .is_null()
    {
        cJSON_Delete(data);
        set_error(
            error,
            error_size,
            b"storage target recovered event allocation failed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = publish_event(
        STORAGE_TARGET_RECOVERED_EVENT_TYPE.as_ptr(),
        b"system/storage\0".as_ptr() as *const ::core::ffi::c_char,
        occurred_at,
        data,
        error,
        error_size,
    );
    cJSON_Delete(data);
    return result;
}
