unsafe extern "C" {
    unsafe fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    unsafe fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddNullToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
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
    static mut g_config: config_t;
    unsafe fn observation_asset_state_name(
        state: observation_asset_state_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn observation_change_cause_name(
        cause: observation_change_cause_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn db_observation_meta_get(out: *mut observation_meta_t) -> ::core::ffi::c_int;
    unsafe fn db_observation_pending_publications(
        out: *mut observation_publication_t,
        max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_observation_publication_mark(
        change_seq: int64_t,
        state: observation_publication_state_t,
        last_error: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_observations_set_publication_hook(hook: db_observation_publication_hook_t);
    unsafe fn observation_cursor_head_write(
        storage_path: *const ::core::ffi::c_char,
        epoch_uuid: *const ::core::ffi::c_char,
        head_seq: int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn observation_cursor_head_reconcile(
        storage_path: *const ::core::ffi::c_char,
        regenerated: *mut bool,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_condattr_init(__attr: *mut pthread_condattr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_condattr_destroy(__attr: *mut pthread_condattr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_condattr_setclock(
        __attr: *mut pthread_condattr_t,
        __clock_id: __clockid_t,
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn event_bus_publish(
        event: *const event_envelope_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> event_bus_result_t;
    unsafe fn event_identity_get_source(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn event_producer_enqueue_durable(
        event: *const event_envelope_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> event_producer_enqueue_result_t;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn db_event_outbox_count_undelivered_type(
        event_type: *const ::core::ffi::c_char,
        count: *mut int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: ::core::ffi::c_ulonglong,
    pub __value32: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __glibc_reserved: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_size: [::core::ffi::c_uint; 2],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2],
    pub __unused_initialized_1: ::core::ffi::c_uint,
    pub __unused_initialized_2: ::core::ffi::c_uint,
}
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::core::ffi::c_char; 48],
    pub __align: ::core::ffi::c_longlong,
}
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
    pub go2rtc_external_ip: [::core::ffi::c_char; 256],
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
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_0 {
    pub const DB_OBS_OK: Self = Self(0);
    pub const DB_OBS_DUPLICATE: Self = Self(1);
    pub const DB_OBS_CAS_LOST: Self = Self(2);
    pub const DB_OBS_ERROR: Self = Self(-1);
    pub const DB_OBS_INVALID: Self = Self(-2);
    pub const DB_OBS_NOT_FOUND: Self = Self(-3);
    pub const DB_OBS_INVALID_CURSOR: Self = Self(-4);
    pub const DB_OBS_CURSOR_EXPIRED: Self = Self(-5);
    pub const DB_OBS_CONFLICT: Self = Self(-6);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_asset_state_t(pub ::core::ffi::c_uint);
impl observation_asset_state_t {
    pub const OBS_ASSET_PENDING: Self = Self(0);
    pub const OBS_ASSET_READY: Self = Self(1);
    pub const OBS_ASSET_UNAVAILABLE: Self = Self(2);
    pub const OBS_ASSET_EXPIRED: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_change_cause_t(pub ::core::ffi::c_uint);
impl observation_change_cause_t {
    pub const OBS_CAUSE_CREATED: Self = Self(0);
    pub const OBS_CAUSE_EVIDENCE: Self = Self(1);
    pub const OBS_CAUSE_CLASSIFICATION: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_publication_state_t(pub ::core::ffi::c_uint);
impl observation_publication_state_t {
    pub const OBS_PUBLICATION_PENDING: Self = Self(0);
    pub const OBS_PUBLICATION_RELAYED: Self = Self(1);
    pub const OBS_PUBLICATION_RELAYED_NO_ROUTE: Self = Self(2);
    pub const OBS_PUBLICATION_EXPIRED: Self = Self(3);
    pub const OBS_PUBLICATION_FAILED: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_meta_t {
    pub epoch_uuid: [::core::ffi::c_char; 37],
    pub oldest_seq: int64_t,
    pub head_seq: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_publication_t {
    pub change_seq: int64_t,
    pub message_id: [::core::ffi::c_char; 37],
    pub attempts: ::core::ffi::c_int,
    pub observation_uuid: [::core::ffi::c_char; 37],
    pub revision: ::core::ffi::c_int,
    pub cause: observation_change_cause_t,
    pub evidence_state: observation_asset_state_t,
    pub evidence_reason: [::core::ffi::c_char; 33],
    pub change_created_at_ms: int64_t,
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub label: [::core::ffi::c_char; 32],
    pub observed_at_ms: int64_t,
    pub asset_uuid: [::core::ffi::c_char; 37],
}
pub type db_observation_publication_hook_t = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_relay_options_t {
    pub batch_size: ::core::ffi::c_int,
    pub poll_interval_ms: ::core::ffi::c_int,
    pub outbox_cap: ::core::ffi::c_int,
    pub startup_drain_ms: ::core::ffi::c_int,
    pub storage_path: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_relay_stats_t {
    pub running: bool,
    pub passes: uint64_t,
    pub relayed: uint64_t,
    pub duplicates: uint64_t,
    pub relayed_no_route: uint64_t,
    pub expired: uint64_t,
    pub failed: uint64_t,
    pub enqueue_full: uint64_t,
    pub enqueue_errors: uint64_t,
    pub mark_errors: uint64_t,
    pub read_errors: uint64_t,
    pub cap_pauses: uint64_t,
    pub backoffs: uint64_t,
    pub sidecar_writes: uint64_t,
    pub sidecar_errors: uint64_t,
    pub epoch_regenerations: uint64_t,
    pub last_relayed_seq: int64_t,
    pub oldest_pending_created_at_ms: int64_t,
    pub last_pass_pending: ::core::ffi::c_int,
    pub outbox_type_pending: int64_t,
    pub consecutive_failures: ::core::ffi::c_int,
    pub backoff_until_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_relay_pass_t {
    pub read: ::core::ffi::c_int,
    pub processed: ::core::ffi::c_int,
    pub backoff: bool,
    pub cap_reached: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct relay_settings_t {
    pub batch_size: ::core::ffi::c_int,
    pub poll_interval_ms: ::core::ffi::c_int,
    pub outbox_cap: ::core::ffi::c_int,
    pub startup_drain_ms: ::core::ffi::c_int,
    pub storage_path: [::core::ffi::c_char; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub mutex: pthread_mutex_t,
    pub wake: pthread_cond_t,
    pub cond_ready: bool,
    pub run_mutex: pthread_mutex_t,
    pub thread: pthread_t,
    pub running: bool,
    pub stop_requested: bool,
    pub notified: bool,
    pub settings: relay_settings_t,
    pub stats: observation_relay_stats_t,
    pub sidecar_dirty: bool,
    pub sidecar_last_write_ms: int64_t,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_relay_stage_t(pub ::core::ffi::c_uint);
impl observation_relay_stage_t {
    pub const OBS_RELAY_STAGE_BEFORE_ENQUEUE: Self = Self(0);
    pub const OBS_RELAY_STAGE_AFTER_ENQUEUE: Self = Self(1);
    pub const OBS_RELAY_STAGE_AFTER_MARK: Self = Self(2);
}
pub type observation_relay_stage_fn = Option<
    unsafe extern "C" fn(
        observation_relay_stage_t,
        *const observation_publication_t,
        *mut ::core::ffi::c_void,
    ) -> (),
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const PTHREAD_MUTEX_TIMED_NP: Self = Self(0);
    pub const PTHREAD_MUTEX_RECURSIVE_NP: Self = Self(1);
    pub const PTHREAD_MUTEX_ERRORCHECK_NP: Self = Self(2);
    pub const PTHREAD_MUTEX_ADAPTIVE_NP: Self = Self(3);
    pub const PTHREAD_MUTEX_NORMAL: Self = Self(0);
    pub const PTHREAD_MUTEX_RECURSIVE: Self = Self(1);
    pub const PTHREAD_MUTEX_ERRORCHECK: Self = Self(2);
    pub const PTHREAD_MUTEX_DEFAULT: Self = Self(0);
    pub const PTHREAD_MUTEX_FAST_NP: Self = Self(0);
}
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OBSERVATION_CHANGED_EVENT_TYPE: [::core::ffi::c_char; 45] = unsafe {
    ::core::mem::transmute::<[u8; 45], [::core::ffi::c_char; 45]>(
        *b"io.lightnvr.detection.observation_changed.v1\0",
    )
};
pub const OBSERVATION_RELAY_OUTBOX_CAP: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
pub const OBSERVATION_RELAY_DEFAULT_BATCH: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const OBSERVATION_RELAY_MAX_BATCH: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const OBSERVATION_RELAY_DEFAULT_POLL_MS: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const OBSERVATION_RELAY_MAX_BACKOFF_MS: ::core::ffi::c_int = 30000 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const RELAY_DEFAULT_STARTUP_DRAIN_MS: ::core::ffi::c_int = 5000 as ::core::ffi::c_int;
static mut RELAY: C2Rust_Unnamed_1 = C2Rust_Unnamed_1 {
    mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0 as ::core::ffi::c_int,
            __count: 0 as ::core::ffi::c_uint,
            __owner: 0 as ::core::ffi::c_int,
            __nusers: 0 as ::core::ffi::c_uint,
            __kind: C2Rust_Unnamed_2::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
            __spins: 0 as ::core::ffi::c_short,
            __glibc_reserved: 0 as ::core::ffi::c_short,
            __list: __pthread_internal_list {
                __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
            },
        },
    },
    wake: pthread_cond_t {
        __data: __pthread_cond_s {
            __wseq: __atomic_wide_counter { __value64: 0 },
            __g1_start: __atomic_wide_counter { __value64: 0 },
            __g_size: [0; 2],
            __g1_orig_size: 0,
            __wrefs: 0,
            __g_signals: [0; 2],
            __unused_initialized_1: 0,
            __unused_initialized_2: 0,
        },
    },
    cond_ready: false,
    run_mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0 as ::core::ffi::c_int,
            __count: 0 as ::core::ffi::c_uint,
            __owner: 0 as ::core::ffi::c_int,
            __nusers: 0 as ::core::ffi::c_uint,
            __kind: C2Rust_Unnamed_2::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
            __spins: 0 as ::core::ffi::c_short,
            __glibc_reserved: 0 as ::core::ffi::c_short,
            __list: __pthread_internal_list {
                __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
            },
        },
    },
    thread: 0,
    running: false,
    stop_requested: false,
    notified: false,
    settings: relay_settings_t {
        batch_size: 0,
        poll_interval_ms: 0,
        outbox_cap: 0,
        startup_drain_ms: 0,
        storage_path: [0; 512],
    },
    stats: observation_relay_stats_t {
        running: false,
        passes: 0,
        relayed: 0,
        duplicates: 0,
        relayed_no_route: 0,
        expired: 0,
        failed: 0,
        enqueue_full: 0,
        enqueue_errors: 0,
        mark_errors: 0,
        read_errors: 0,
        cap_pauses: 0,
        backoffs: 0,
        sidecar_writes: 0,
        sidecar_errors: 0,
        epoch_regenerations: 0,
        last_relayed_seq: 0,
        oldest_pending_created_at_ms: 0,
        last_pass_pending: 0,
        outbox_type_pending: 0,
        consecutive_failures: 0,
        backoff_until_ms: 0,
    },
    sidecar_dirty: r#true != 0,
    sidecar_last_write_ms: 0,
};
unsafe extern "C" fn realtime_ms() -> int64_t {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_REALTIME, &raw mut ts);
    return ts.tv_sec as int64_t * 1000 as int64_t + ts.tv_nsec as int64_t / 1000000 as int64_t;
}
unsafe extern "C" fn monotonic_ms() -> int64_t {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut ts);
    return ts.tv_sec as int64_t * 1000 as int64_t + ts.tv_nsec as int64_t / 1000000 as int64_t;
}
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
            b"observation relay error\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
}
unsafe extern "C" fn ensure_cond_locked() {
    if RELAY.cond_ready {
        return;
    }
    let mut attr: pthread_condattr_t = pthread_condattr_t { __size: [0; 4] };
    pthread_condattr_init(&raw mut attr);
    pthread_condattr_setclock(&raw mut attr, CLOCK_MONOTONIC);
    pthread_cond_init(&raw mut RELAY.wake, &raw mut attr);
    pthread_condattr_destroy(&raw mut attr);
    RELAY.cond_ready = r#true != 0;
}
unsafe extern "C" fn resolve_settings(
    mut options: *const observation_relay_options_t,
) -> relay_settings_t {
    let mut settings: relay_settings_t = relay_settings_t {
        batch_size: 0,
        poll_interval_ms: 0,
        outbox_cap: 0,
        startup_drain_ms: 0,
        storage_path: [0; 512],
    };
    memset(
        &raw mut settings as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<relay_settings_t>(),
    );
    settings.batch_size = OBSERVATION_RELAY_DEFAULT_BATCH;
    settings.poll_interval_ms = OBSERVATION_RELAY_DEFAULT_POLL_MS;
    settings.outbox_cap = OBSERVATION_RELAY_OUTBOX_CAP;
    settings.startup_drain_ms = RELAY_DEFAULT_STARTUP_DRAIN_MS;
    let mut storage: *const ::core::ffi::c_char =
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char;
    if !options.is_null() {
        if (*options).batch_size > 0 as ::core::ffi::c_int {
            settings.batch_size = (*options).batch_size;
        }
        if (*options).poll_interval_ms > 0 as ::core::ffi::c_int {
            settings.poll_interval_ms = (*options).poll_interval_ms;
        }
        if (*options).outbox_cap > 0 as ::core::ffi::c_int {
            settings.outbox_cap = (*options).outbox_cap;
        }
        if (*options).startup_drain_ms > 0 as ::core::ffi::c_int {
            settings.startup_drain_ms = (*options).startup_drain_ms;
        }
        if !(*options).storage_path.is_null() {
            storage = (*options).storage_path;
        }
    }
    if settings.batch_size > OBSERVATION_RELAY_MAX_BATCH {
        settings.batch_size = OBSERVATION_RELAY_MAX_BATCH;
    }
    safe_strcpy(
        &raw mut settings.storage_path as *mut ::core::ffi::c_char,
        if !storage.is_null() {
            storage
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    return settings;
}
unsafe extern "C" fn format_ms_utc(
    mut ms: int64_t,
    mut out: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if ms <= 0 as int64_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut seconds: time_t = (ms / 1000 as int64_t) as time_t;
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
    if gmtime_r(&raw mut seconds, &raw mut utc).is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut base: [::core::ffi::c_char; 24] = [0; 24];
    if strftime(
        &raw mut base as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        b"%Y-%m-%dT%H:%M:%S\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut utc,
    ) == 0 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut written: ::core::ffi::c_int = snprintf(
        out,
        32 as size_t,
        b"%s.%03dZ\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut base as *mut ::core::ffi::c_char,
        (ms % 1000 as int64_t) as ::core::ffi::c_int,
    );
    return if written > 0 as ::core::ffi::c_int && written < 32 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_relay_build_envelope(
    mut publication: *const observation_publication_t,
    mut source: *const ::core::ffi::c_char,
    mut event: *mut event_envelope_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if publication.is_null() || source.is_null() || event.is_null() {
        set_error(
            error,
            error_size,
            b"publication, source and event are required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_envelope_t>(),
    );
    let mut state: *const ::core::ffi::c_char =
        observation_asset_state_name((*publication).evidence_state);
    let mut cause: *const ::core::ffi::c_char = observation_change_cause_name((*publication).cause);
    let mut observed_at: [::core::ffi::c_char; 32] = [0; 32];
    if state.is_null()
        || cause.is_null()
        || format_ms_utc(
            (*publication).observed_at_ms,
            &raw mut observed_at as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        || (*publication).change_created_at_ms < 1000 as int64_t
    {
        set_error(
            error,
            error_size,
            b"publication state, cause or timestamps are invalid\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut subject: [::core::ffi::c_char; 96] = [0; 96];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut subject as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        b"camera/%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*publication).camera_uuid as *const ::core::ffi::c_char,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 96]>()
    {
        set_error(
            error,
            error_size,
            b"publication camera identity is invalid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut data: *mut cJSON = cJSON_CreateObject();
    let mut ok: bool = !data.is_null()
        && !cJSON_AddStringToObject(
            data,
            b"observation_id\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*publication).observation_uuid as *const ::core::ffi::c_char,
        )
        .is_null()
        && !cJSON_AddNumberToObject(
            data,
            b"revision\0".as_ptr() as *const ::core::ffi::c_char,
            (*publication).revision as ::core::ffi::c_double,
        )
        .is_null()
        && !cJSON_AddStringToObject(
            data,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*publication).camera_uuid as *const ::core::ffi::c_char,
        )
        .is_null()
        && !cJSON_AddStringToObject(
            data,
            b"label\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*publication).label as *const ::core::ffi::c_char,
        )
        .is_null()
        && !cJSON_AddStringToObject(
            data,
            b"observed_at\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut observed_at as *mut ::core::ffi::c_char,
        )
        .is_null()
        && !cJSON_AddStringToObject(
            data,
            b"change_cause\0".as_ptr() as *const ::core::ffi::c_char,
            cause,
        )
        .is_null()
        && !cJSON_AddStringToObject(
            data,
            b"evidence_state\0".as_ptr() as *const ::core::ffi::c_char,
            state,
        )
        .is_null()
        && if (*publication).evidence_reason[0usize] as ::core::ffi::c_int != 0 {
            !cJSON_AddStringToObject(
                data,
                b"evidence_reason\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*publication).evidence_reason as *const ::core::ffi::c_char,
            )
            .is_null() as ::core::ffi::c_int
        } else {
            !cJSON_AddNullToObject(
                data,
                b"evidence_reason\0".as_ptr() as *const ::core::ffi::c_char,
            )
            .is_null() as ::core::ffi::c_int
        } != 0
        && !cJSON_AddStringToObject(
            data,
            b"asset_id\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*publication).asset_uuid as *const ::core::ffi::c_char,
        )
        .is_null();
    if ok as ::core::ffi::c_int != 0
        && (*publication).evidence_state.0 == observation_asset_state_t::OBS_ASSET_READY.0
    {
        let mut url: [::core::ffi::c_char; 96] = [0; 96];
        snprintf(
            &raw mut url as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
            b"/api/observations/%s/snapshot\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*publication).observation_uuid as *const ::core::ffi::c_char,
        );
        ok = !cJSON_AddStringToObject(
            data,
            b"snapshot_url\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut url as *mut ::core::ffi::c_char,
        )
        .is_null();
    }
    if !ok {
        cJSON_Delete(data);
        set_error(
            error,
            error_size,
            b"observation event allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = event_envelope_create_with_severity_and_id(
        event,
        OBSERVATION_CHANGED_EVENT_TYPE.as_ptr(),
        source,
        &raw mut subject as *mut ::core::ffi::c_char,
        ((*publication).change_created_at_ms / 1000 as int64_t) as time_t,
        data,
        event_severity_t::EVENT_SEVERITY_INFO,
        &raw const (*publication).message_id as *const ::core::ffi::c_char,
        error,
        error_size,
    );
    cJSON_Delete(data);
    return result;
}
unsafe extern "C" fn write_sidecar_now(
    mut storage_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut meta: observation_meta_t = observation_meta_t {
        epoch_uuid: [0; 37],
        oldest_seq: 0,
        head_seq: 0,
    };
    if db_observation_meta_get(&raw mut meta) != C2Rust_Unnamed_0::DB_OBS_OK.0
        || observation_cursor_head_write(
            storage_path,
            &raw mut meta.epoch_uuid as *mut ::core::ffi::c_char,
            meta.head_seq,
        ) != 0 as ::core::ffi::c_int
    {
        pthread_mutex_lock(&raw mut RELAY.mutex);
        RELAY.stats.sidecar_errors = RELAY.stats.sidecar_errors.wrapping_add(1 as uint64_t);
        pthread_mutex_unlock(&raw mut RELAY.mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut RELAY.mutex);
    RELAY.stats.sidecar_writes = RELAY.stats.sidecar_writes.wrapping_add(1 as uint64_t);
    pthread_mutex_unlock(&raw mut RELAY.mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_relay_write_cursor_head(
    mut storage_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut path: *const ::core::ffi::c_char = if !storage_path.is_null() {
        storage_path
    } else {
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char
    };
    if path.is_null() || *path.offset(0isize) == 0 {
        return -1 as ::core::ffi::c_int;
    }
    return write_sidecar_now(path);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_relay_check_cursor_head(
    mut storage_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut path: *const ::core::ffi::c_char = if !storage_path.is_null() {
        storage_path
    } else {
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char
    };
    if path.is_null() || *path.offset(0isize) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut regenerated: bool = r#false != 0;
    if observation_cursor_head_reconcile(path, &raw mut regenerated) != 0 as ::core::ffi::c_int {
        pthread_mutex_lock(&raw mut RELAY.mutex);
        RELAY.stats.sidecar_errors = RELAY.stats.sidecar_errors.wrapping_add(1 as uint64_t);
        pthread_mutex_unlock(&raw mut RELAY.mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut RELAY.mutex);
    RELAY.stats.sidecar_writes = RELAY.stats.sidecar_writes.wrapping_add(1 as uint64_t);
    pthread_mutex_unlock(&raw mut RELAY.mutex);
    if !regenerated {
        return 0 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut RELAY.mutex);
    RELAY.stats.epoch_regenerations = RELAY.stats.epoch_regenerations.wrapping_add(1 as uint64_t);
    pthread_mutex_unlock(&raw mut RELAY.mutex);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn maybe_write_sidecar_locked(mut settings: *const relay_settings_t) {
    if !RELAY.sidecar_dirty || (*settings).storage_path[0usize] == 0 {
        return;
    }
    let mut now: int64_t = monotonic_ms();
    if RELAY.sidecar_last_write_ms != 0 as int64_t
        && now - RELAY.sidecar_last_write_ms < 1000 as int64_t
    {
        return;
    }
    if write_sidecar_now(&raw const (*settings).storage_path as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        RELAY.sidecar_dirty = r#false != 0;
        RELAY.sidecar_last_write_ms = now;
    }
}
static mut g_stage_lock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_2::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut g_stage_hook: observation_relay_stage_fn = None;
static mut g_stage_ctx: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_relay_set_stage_hook(
    mut r#fn: observation_relay_stage_fn,
    mut ctx: *mut ::core::ffi::c_void,
) {
    pthread_mutex_lock(&raw mut g_stage_lock);
    g_stage_hook = r#fn;
    g_stage_ctx = ctx;
    pthread_mutex_unlock(&raw mut g_stage_lock);
}
unsafe extern "C" fn stage_hook(
    mut stage: observation_relay_stage_t,
    mut row: *const observation_publication_t,
) {
    pthread_mutex_lock(&raw mut g_stage_lock);
    let mut r#fn: observation_relay_stage_fn = g_stage_hook;
    let mut ctx: *mut ::core::ffi::c_void = g_stage_ctx;
    pthread_mutex_unlock(&raw mut g_stage_lock);
    if r#fn.is_some() {
        r#fn.expect("non-null function pointer")(stage, row, ctx);
    }
}
unsafe extern "C" fn mark(
    mut seq: int64_t,
    mut state: observation_publication_state_t,
    mut error: *const ::core::ffi::c_char,
    mut cas_lost: *mut bool,
) -> bool {
    let mut rc: ::core::ffi::c_int = db_observation_publication_mark(seq, state, error);
    *cas_lost =
        rc == C2Rust_Unnamed_0::DB_OBS_CAS_LOST.0 || rc == C2Rust_Unnamed_0::DB_OBS_NOT_FOUND.0;
    if rc == C2Rust_Unnamed_0::DB_OBS_OK.0 || *cas_lost as ::core::ffi::c_int != 0 {
        return r#true != 0;
    }
    pthread_mutex_lock(&raw mut RELAY.mutex);
    RELAY.stats.mark_errors = RELAY.stats.mark_errors.wrapping_add(1 as uint64_t);
    pthread_mutex_unlock(&raw mut RELAY.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT,
        _log_stream_name,
        b"Observation relay could not mark publication %lld (%d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        seq as ::core::ffi::c_longlong,
        rc,
    );
    return r#false != 0;
}
unsafe extern "C" fn run_pass_locked(
    mut settings: *const relay_settings_t,
    mut pass: *mut observation_relay_pass_t,
) -> ::core::ffi::c_int {
    let mut rows: *mut observation_publication_t = calloc(
        (*settings).batch_size as size_t,
        ::core::mem::size_of::<observation_publication_t>(),
    ) as *mut observation_publication_t;
    if rows.is_null() {
        (*pass).backoff = r#true != 0;
        pthread_mutex_lock(&raw mut RELAY.mutex);
        RELAY.stats.read_errors = RELAY.stats.read_errors.wrapping_add(1 as uint64_t);
        pthread_mutex_unlock(&raw mut RELAY.mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int =
        db_observation_pending_publications(rows, (*settings).batch_size);
    if count < 0 as ::core::ffi::c_int {
        free(rows as *mut ::core::ffi::c_void);
        (*pass).backoff = r#true != 0;
        pthread_mutex_lock(&raw mut RELAY.mutex);
        RELAY.stats.read_errors = RELAY.stats.read_errors.wrapping_add(1 as uint64_t);
        pthread_mutex_unlock(&raw mut RELAY.mutex);
        return -1 as ::core::ffi::c_int;
    }
    (*pass).read = count;
    pthread_mutex_lock(&raw mut RELAY.mutex);
    RELAY.stats.passes = RELAY.stats.passes.wrapping_add(1);
    RELAY.stats.last_pass_pending = count;
    RELAY.stats.oldest_pending_created_at_ms = if count > 0 as ::core::ffi::c_int {
        (*rows.offset(0isize)).change_created_at_ms
    } else {
        0 as int64_t
    };
    pthread_mutex_unlock(&raw mut RELAY.mutex);
    if count == 0 as ::core::ffi::c_int {
        free(rows as *mut ::core::ffi::c_void);
        maybe_write_sidecar_locked(settings);
        return 0 as ::core::ffi::c_int;
    }
    RELAY.sidecar_dirty = r#true != 0;
    let mut source: [::core::ffi::c_char; 128] = [0; 128];
    let mut outbox_pending: int64_t = 0 as int64_t;
    if event_identity_get_source(
        &raw mut source as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    ) != 0 as ::core::ffi::c_int
        || db_event_outbox_count_undelivered_type(
            OBSERVATION_CHANGED_EVENT_TYPE.as_ptr(),
            &raw mut outbox_pending,
        ) != 0 as ::core::ffi::c_int
    {
        free(rows as *mut ::core::ffi::c_void);
        (*pass).backoff = r#true != 0;
        pthread_mutex_lock(&raw mut RELAY.mutex);
        RELAY.stats.read_errors = RELAY.stats.read_errors.wrapping_add(1 as uint64_t);
        pthread_mutex_unlock(&raw mut RELAY.mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut RELAY.mutex);
    RELAY.stats.outbox_type_pending = outbox_pending;
    pthread_mutex_unlock(&raw mut RELAY.mutex);
    let mut room: int64_t = (*settings).outbox_cap as int64_t - outbox_pending;
    let mut definition: *const event_type_definition_t =
        event_registry_find(OBSERVATION_CHANGED_EVENT_TYPE.as_ptr());
    let mut expiry_ms: int64_t = (if !definition.is_null() {
        (*definition).default_expiry_seconds
    } else {
        3600 as ::core::ffi::c_int
    }) as int64_t
        * 1000 as int64_t;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut row: *const observation_publication_t = rows.offset(index as isize);
        let mut cas_lost: bool = r#false != 0;
        if realtime_ms() >= (*row).change_created_at_ms + expiry_ms {
            if !mark(
                (*row).change_seq,
                observation_publication_state_t::OBS_PUBLICATION_EXPIRED,
                b"delivery expiry elapsed\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut cas_lost,
            ) {
                (*pass).backoff = r#true != 0;
                break;
            } else if !cas_lost {
                (*pass).processed += 1;
                pthread_mutex_lock(&raw mut RELAY.mutex);
                RELAY.stats.expired = RELAY.stats.expired.wrapping_add(1 as uint64_t);
                pthread_mutex_unlock(&raw mut RELAY.mutex);
            }
        } else {
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
            let mut error: [::core::ffi::c_char; 256] = ::core::mem::transmute::<
                [u8; 256],
                [::core::ffi::c_char; 256],
            >(
                *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            );
            if observation_relay_build_envelope(
                row,
                &raw mut source as *mut ::core::ffi::c_char,
                &raw mut event,
                &raw mut error as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            ) != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Observation publication %lld is not a valid event: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*row).change_seq as ::core::ffi::c_longlong,
                    &raw mut error as *mut ::core::ffi::c_char,
                );
                if !mark(
                    (*row).change_seq,
                    observation_publication_state_t::OBS_PUBLICATION_FAILED,
                    &raw mut error as *mut ::core::ffi::c_char,
                    &raw mut cas_lost,
                ) {
                    (*pass).backoff = r#true != 0;
                    break;
                } else if !cas_lost {
                    (*pass).processed += 1;
                    pthread_mutex_lock(&raw mut RELAY.mutex);
                    RELAY.stats.failed = RELAY.stats.failed.wrapping_add(1 as uint64_t);
                    pthread_mutex_unlock(&raw mut RELAY.mutex);
                }
            } else if room <= 0 as int64_t {
                event_envelope_clear(&raw mut event);
                (*pass).cap_reached = r#true != 0;
                (*pass).backoff = r#true != 0;
                pthread_mutex_lock(&raw mut RELAY.mutex);
                RELAY.stats.cap_pauses = RELAY.stats.cap_pauses.wrapping_add(1 as uint64_t);
                pthread_mutex_unlock(&raw mut RELAY.mutex);
                break;
            } else {
                stage_hook(
                    observation_relay_stage_t::OBS_RELAY_STAGE_BEFORE_ENQUEUE,
                    row,
                );
                let mut result: event_producer_enqueue_result_t = event_producer_enqueue_durable(
                    &raw mut event,
                    &raw mut error as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                );
                if result.0 == event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_ENQUEUED.0
                    || result.0
                        == event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_DUPLICATE.0
                {
                    stage_hook(
                        observation_relay_stage_t::OBS_RELAY_STAGE_AFTER_ENQUEUE,
                        row,
                    );
                }
                let mut state: observation_publication_state_t =
                    observation_publication_state_t::OBS_PUBLICATION_PENDING;
                let mut retry: bool = r#false != 0;
                match result {
                    event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_ENQUEUED => {
                        room -= 1;
                        state = observation_publication_state_t::OBS_PUBLICATION_RELAYED;
                    }
                    event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_DUPLICATE => {
                        state = observation_publication_state_t::OBS_PUBLICATION_RELAYED;
                    }
                    event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_NO_ROUTE
                    | event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_MQTT_DISABLED => {
                        state = observation_publication_state_t::OBS_PUBLICATION_RELAYED_NO_ROUTE;
                    }
                    event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_FULL => {
                        pthread_mutex_lock(&raw mut RELAY.mutex);
                        RELAY.stats.enqueue_full =
                            RELAY.stats.enqueue_full.wrapping_add(1 as uint64_t);
                        pthread_mutex_unlock(&raw mut RELAY.mutex);
                        state = observation_publication_state_t::OBS_PUBLICATION_PENDING;
                        retry = r#true != 0;
                        set_error(
                            &raw mut error as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                            b"outbox full\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_FAILED | _ => {
                        pthread_mutex_lock(&raw mut RELAY.mutex);
                        RELAY.stats.enqueue_errors =
                            RELAY.stats.enqueue_errors.wrapping_add(1 as uint64_t);
                        pthread_mutex_unlock(&raw mut RELAY.mutex);
                        state = observation_publication_state_t::OBS_PUBLICATION_PENDING;
                        retry = r#true != 0;
                        if error[0usize] == 0 {
                            set_error(
                                &raw mut error as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                                b"durable enqueue failed\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    }
                }
                let mut marked: bool = mark(
                    (*row).change_seq,
                    state,
                    if retry as ::core::ffi::c_int != 0 {
                        &raw mut error as *mut ::core::ffi::c_char
                    } else {
                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                    },
                    &raw mut cas_lost,
                );
                if retry as ::core::ffi::c_int != 0 || !marked {
                    event_envelope_clear(&raw mut event);
                    (*pass).backoff = r#true != 0;
                    break;
                } else {
                    if !cas_lost {
                        stage_hook(observation_relay_stage_t::OBS_RELAY_STAGE_AFTER_MARK, row);
                        (*pass).processed += 1;
                        pthread_mutex_lock(&raw mut RELAY.mutex);
                        if state.0 == observation_publication_state_t::OBS_PUBLICATION_RELAYED.0 {
                            RELAY.stats.relayed = RELAY.stats.relayed.wrapping_add(1);
                            if result.0
                                == event_producer_enqueue_result_t::EVENT_PRODUCER_ENQUEUE_DUPLICATE
                                    .0
                            {
                                RELAY.stats.duplicates = RELAY.stats.duplicates.wrapping_add(1);
                            }
                        } else {
                            RELAY.stats.relayed_no_route =
                                RELAY.stats.relayed_no_route.wrapping_add(1);
                        }
                        RELAY.stats.last_relayed_seq = (*row).change_seq;
                        pthread_mutex_unlock(&raw mut RELAY.mutex);
                        event_bus_publish(
                            &raw mut event,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            0 as size_t,
                        );
                    }
                    event_envelope_clear(&raw mut event);
                }
            }
        }
        index += 1;
    }
    free(rows as *mut ::core::ffi::c_void);
    maybe_write_sidecar_locked(settings);
    return (*pass).processed;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_relay_run_once(
    mut options: *const observation_relay_options_t,
    mut pass: *mut observation_relay_pass_t,
) -> ::core::ffi::c_int {
    let mut local: observation_relay_pass_t = observation_relay_pass_t {
        read: 0,
        processed: 0,
        backoff: false,
        cap_reached: false,
    };
    if pass.is_null() {
        pass = &raw mut local;
    }
    memset(
        pass as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_relay_pass_t>(),
    );
    let mut settings: relay_settings_t = relay_settings_t {
        batch_size: 0,
        poll_interval_ms: 0,
        outbox_cap: 0,
        startup_drain_ms: 0,
        storage_path: [0; 512],
    };
    if !options.is_null() {
        settings = resolve_settings(options);
    } else {
        pthread_mutex_lock(&raw mut RELAY.mutex);
        let mut running: bool = RELAY.running;
        settings = RELAY.settings;
        pthread_mutex_unlock(&raw mut RELAY.mutex);
        if !running {
            settings = resolve_settings(::core::ptr::null::<observation_relay_options_t>());
        }
    }
    pthread_mutex_lock(&raw mut RELAY.run_mutex);
    let mut result: ::core::ffi::c_int = run_pass_locked(&raw mut settings, pass);
    pthread_mutex_unlock(&raw mut RELAY.run_mutex);
    return result;
}
unsafe extern "C" fn relay_main(
    mut argument: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut backoff_ms: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        pthread_mutex_lock(&raw mut RELAY.mutex);
        let mut stop: bool = RELAY.stop_requested;
        let mut settings: relay_settings_t = RELAY.settings;
        RELAY.notified = r#false != 0;
        pthread_mutex_unlock(&raw mut RELAY.mutex);
        if stop {
            break;
        }
        let mut pass: observation_relay_pass_t = observation_relay_pass_t {
            read: 0,
            processed: 0,
            backoff: false,
            cap_reached: false,
        };
        memset(
            &raw mut pass as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<observation_relay_pass_t>(),
        );
        pthread_mutex_lock(&raw mut RELAY.run_mutex);
        run_pass_locked(&raw mut settings, &raw mut pass);
        pthread_mutex_unlock(&raw mut RELAY.run_mutex);
        let mut wait_ms: ::core::ffi::c_int = 0;
        let mut in_backoff: bool = pass.backoff;
        if pass.backoff {
            backoff_ms = if backoff_ms <= 0 as ::core::ffi::c_int {
                1000 as ::core::ffi::c_int
            } else {
                backoff_ms * 2 as ::core::ffi::c_int
            };
            if backoff_ms > OBSERVATION_RELAY_MAX_BACKOFF_MS {
                backoff_ms = OBSERVATION_RELAY_MAX_BACKOFF_MS;
            }
            wait_ms = backoff_ms;
        } else {
            backoff_ms = 0 as ::core::ffi::c_int;
            wait_ms = if pass.read >= settings.batch_size {
                0 as ::core::ffi::c_int
            } else {
                settings.poll_interval_ms
            };
        }
        pthread_mutex_lock(&raw mut RELAY.mutex);
        if in_backoff {
            RELAY.stats.backoffs = RELAY.stats.backoffs.wrapping_add(1);
            RELAY.stats.consecutive_failures += 1;
            RELAY.stats.backoff_until_ms = realtime_ms() + wait_ms as int64_t;
        } else {
            RELAY.stats.consecutive_failures = 0 as ::core::ffi::c_int;
            RELAY.stats.backoff_until_ms = 0 as int64_t;
        }
        let mut deadline: int64_t = monotonic_ms() + wait_ms as int64_t;
        while !RELAY.stop_requested && (in_backoff as ::core::ffi::c_int != 0 || !RELAY.notified) {
            let mut remaining: int64_t = deadline - monotonic_ms();
            if remaining <= 0 as int64_t {
                break;
            }
            let mut until: timespec = timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            clock_gettime(CLOCK_MONOTONIC, &raw mut until);
            until.tv_sec += (remaining / 1000 as int64_t) as ::core::ffi::c_long;
            until.tv_nsec +=
                (remaining % 1000 as int64_t * 1000000 as int64_t) as ::core::ffi::c_long;
            if until.tv_nsec >= 1000000000 as ::core::ffi::c_long {
                until.tv_sec += 1;
                until.tv_nsec -= 1000000000 as ::core::ffi::c_long;
            }
            pthread_cond_timedwait(&raw mut RELAY.wake, &raw mut RELAY.mutex, &raw mut until);
        }
        pthread_mutex_unlock(&raw mut RELAY.mutex);
    }
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_relay_start(
    mut options: *const observation_relay_options_t,
) -> ::core::ffi::c_int {
    let mut settings: relay_settings_t = resolve_settings(options);
    pthread_mutex_lock(&raw mut RELAY.mutex);
    ensure_cond_locked();
    if RELAY.running {
        pthread_mutex_unlock(&raw mut RELAY.mutex);
        return 0 as ::core::ffi::c_int;
    }
    RELAY.settings = settings;
    RELAY.stop_requested = r#false != 0;
    RELAY.notified = r#false != 0;
    pthread_mutex_unlock(&raw mut RELAY.mutex);
    db_observations_set_publication_hook(Some(
        observation_relay_notify as unsafe extern "C" fn() -> (),
    ));
    if settings.storage_path[0usize] as ::core::ffi::c_int != 0
        && observation_relay_check_cursor_head(
            &raw mut settings.storage_path as *mut ::core::ffi::c_char,
        ) < 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Observation relay could not verify the cursor-head sidecar\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut drain_deadline: int64_t = monotonic_ms() + settings.startup_drain_ms as int64_t;
    let mut drained: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let mut pass: observation_relay_pass_t = observation_relay_pass_t {
            read: 0,
            processed: 0,
            backoff: false,
            cap_reached: false,
        };
        memset(
            &raw mut pass as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<observation_relay_pass_t>(),
        );
        pthread_mutex_lock(&raw mut RELAY.run_mutex);
        run_pass_locked(&raw mut settings, &raw mut pass);
        pthread_mutex_unlock(&raw mut RELAY.run_mutex);
        drained += pass.processed;
        if pass.backoff as ::core::ffi::c_int != 0
            || pass.read < settings.batch_size
            || monotonic_ms() >= drain_deadline
        {
            break;
        }
    }
    if drained > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Observation relay drained %d staged publication(s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            drained,
        );
    }
    pthread_mutex_lock(&raw mut RELAY.mutex);
    let mut rc: ::core::ffi::c_int = pthread_create(
        &raw mut RELAY.thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            relay_main
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    );
    if rc == 0 as ::core::ffi::c_int {
        RELAY.running = r#true != 0;
        RELAY.stats.running = r#true != 0;
    }
    pthread_mutex_unlock(&raw mut RELAY.mutex);
    if rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to start observation relay thread: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(rc),
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Observation publication relay started\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_relay_stop() {
    pthread_mutex_lock(&raw mut RELAY.mutex);
    if !RELAY.running {
        pthread_mutex_unlock(&raw mut RELAY.mutex);
        return;
    }
    RELAY.stop_requested = r#true != 0;
    pthread_cond_broadcast(&raw mut RELAY.wake);
    let mut thread: pthread_t = RELAY.thread;
    pthread_mutex_unlock(&raw mut RELAY.mutex);
    pthread_join(thread, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
    pthread_mutex_lock(&raw mut RELAY.mutex);
    RELAY.running = r#false != 0;
    RELAY.stats.running = r#false != 0;
    RELAY.stop_requested = r#false != 0;
    pthread_mutex_unlock(&raw mut RELAY.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Observation publication relay stopped\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_relay_notify() {
    pthread_mutex_lock(&raw mut RELAY.mutex);
    RELAY.notified = r#true != 0;
    if RELAY.cond_ready {
        pthread_cond_signal(&raw mut RELAY.wake);
    }
    pthread_mutex_unlock(&raw mut RELAY.mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_relay_get_stats(mut stats: *mut observation_relay_stats_t) {
    if stats.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut RELAY.mutex);
    *stats = RELAY.stats;
    pthread_mutex_unlock(&raw mut RELAY.mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_relay_reset_stats() {
    pthread_mutex_lock(&raw mut RELAY.run_mutex);
    RELAY.sidecar_dirty = r#true != 0;
    RELAY.sidecar_last_write_ms = 0 as int64_t;
    pthread_mutex_unlock(&raw mut RELAY.run_mutex);
    pthread_mutex_lock(&raw mut RELAY.mutex);
    let mut running: bool = RELAY.stats.running;
    memset(
        &raw mut RELAY.stats as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_relay_stats_t>(),
    );
    RELAY.stats.running = running;
    pthread_mutex_unlock(&raw mut RELAY.mutex);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
