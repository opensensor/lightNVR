unsafe extern "C" {
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
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
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
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
    unsafe fn cJSON_AddStringToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn get_json_logs_tail(
        max_verbosity: ::core::ffi::c_int,
        last_timestamp: *const ::core::ffi::c_char,
        max_lines: ::core::ffi::c_int,
    ) -> *mut cJSON;
    unsafe fn httpd_authorize_global_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_log_level_string(level: log_level_t) -> *const ::core::ffi::c_char;
    unsafe fn parse_log_level_string(log_level: *const ::core::ffi::c_char) -> log_level_t;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
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
pub type cJSON_bool = ::core::ffi::c_int;
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
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const O_APPEND: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const LOG_COMPONENT: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SystemAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL_0 as *const ::core::ffi::c_char;
static mut DEFAULT_LOG_FILE: *const ::core::ffi::c_char =
    b"/var/log/lightnvr.log\0".as_ptr() as *const ::core::ffi::c_char;
static mut FALLBACK_LOG_FILE: *const ::core::ffi::c_char =
    b"./lightnvr.log\0".as_ptr() as *const ::core::ffi::c_char;
static mut DEFAULT_MAX_LOG_ENTRIES: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
unsafe extern "C" fn can_open_for_write(mut path: *const ::core::ffi::c_char) -> bool {
    let mut fd: ::core::ffi::c_int = 0;
    if !is_safe_log_path(path) {
        return r#false != 0;
    }
    fd = open(path, O_WRONLY | O_APPEND | O_CLOEXEC);
    if fd < 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    close(fd);
    return r#true != 0;
}
unsafe extern "C" fn is_safe_log_path(mut path: *const ::core::ffi::c_char) -> bool {
    if path.is_null() || *path.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return r#false != 0;
    }
    if !strstr(path, b"/../\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
        return r#false != 0;
    }
    let mut len: size_t = strlen(path);
    if len >= 3 as size_t
        && strcmp(
            path.offset(len as isize)
                .offset(-(3 as ::core::ffi::c_int as isize)),
            b"/..\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    if strncmp(
        path,
        b"../\0".as_ptr() as *const ::core::ffi::c_char,
        3 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    if strcmp(path, b"..\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_system_logs(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/system/logs request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_SYSTEM_ADMIN) == 0 {
        return;
    }
    let mut level: log_level_t = log_level_t::LOG_LEVEL_DEBUG;
    let mut max_lines: ::core::ffi::c_int = DEFAULT_MAX_LOG_ENTRIES;
    let mut param_buf: [::core::ffi::c_char; 32] = [
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
    ];
    let mut last_ts: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if http_request_get_query_param(
        req,
        b"level\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut param_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    ) > 0 as ::core::ffi::c_int
        && param_buf[0usize] as ::core::ffi::c_int != 0
    {
        level = parse_log_level_string(&raw mut param_buf as *mut ::core::ffi::c_char);
    }
    if http_request_get_query_param(
        req,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut param_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    ) > 0 as ::core::ffi::c_int
        && param_buf[0usize] as ::core::ffi::c_int != 0
    {
        max_lines = atoi(&raw mut param_buf as *mut ::core::ffi::c_char);
    }
    if http_request_get_query_param(
        req,
        b"last_ts\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut param_buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    ) > 0 as ::core::ffi::c_int
        && param_buf[0usize] as ::core::ffi::c_int != 0
    {
        last_ts = &raw mut param_buf as *mut ::core::ffi::c_char;
    }
    let mut logs_obj: *mut cJSON = cJSON_CreateObject();
    if logs_obj.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create logs JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create logs JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut logs_array: *mut cJSON =
        get_json_logs_tail(level.0 as ::core::ffi::c_int, last_ts, max_lines);
    if logs_array.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to get JSON logs\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get system logs\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(logs_obj);
        return;
    }
    cJSON_AddItemToObject(
        logs_obj,
        b"logs\0".as_ptr() as *const ::core::ffi::c_char,
        logs_array,
    );
    cJSON_AddStringToObject(
        logs_obj,
        b"file\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.log_file as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        logs_obj,
        b"level\0".as_ptr() as *const ::core::ffi::c_char,
        get_log_level_string(level),
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(logs_obj);
    cJSON_Delete(logs_obj);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert logs JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert logs JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_system_logs_clear(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/system/logs/clear request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_SYSTEM_ADMIN) == 0 {
        return;
    }
    let mut log_file: *const ::core::ffi::c_char = DEFAULT_LOG_FILE;
    let mut fallback_log_file: *const ::core::ffi::c_char = FALLBACK_LOG_FILE;
    if g_config.log_file[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        log_file = &raw mut g_config.log_file as *mut ::core::ffi::c_char;
    }
    if !is_safe_log_path(log_file) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Configured log file path is unsafe: %s\0".as_ptr() as *const ::core::ffi::c_char,
            log_file,
        );
        log_file = fallback_log_file;
        if !is_safe_log_path(log_file) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Fallback log file path is also unsafe: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                log_file,
            );
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Invalid log file configuration\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut file_exists: bool = stat(log_file, &raw mut st) == 0 as ::core::ffi::c_int;
    let mut is_writable: bool = can_open_for_write(log_file);
    if !file_exists || !is_writable {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Primary log file not accessible, trying fallback: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            fallback_log_file,
        );
        if !is_safe_log_path(fallback_log_file) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Fallback log file path is unsafe: %s\0".as_ptr() as *const ::core::ffi::c_char,
                fallback_log_file,
            );
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Invalid fallback log file configuration\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        file_exists = stat(fallback_log_file, &raw mut st) == 0 as ::core::ffi::c_int;
        is_writable = can_open_for_write(fallback_log_file);
        if file_exists as ::core::ffi::c_int != 0 && is_writable as ::core::ffi::c_int != 0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Using fallback log file for clearing\0".as_ptr() as *const ::core::ffi::c_char,
            );
            log_file = fallback_log_file;
        }
    }
    let mut fd: ::core::ffi::c_int = open(
        log_file,
        O_WRONLY | O_TRUNC | O_CREAT | O_CLOEXEC,
        0o600 as ::core::ffi::c_int,
    );
    if fd >= 0 as ::core::ffi::c_int {
        close(fd);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Log file cleared via API: %s\0".as_ptr() as *const ::core::ffi::c_char,
            log_file,
        );
        let mut success: *mut cJSON = cJSON_CreateObject();
        if success.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to create success JSON object\0".as_ptr() as *const ::core::ffi::c_char,
            );
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create success JSON\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddBoolToObject(
            success,
            b"success\0".as_ptr() as *const ::core::ffi::c_char,
            r#true,
        );
        cJSON_AddStringToObject(
            success,
            b"message\0".as_ptr() as *const ::core::ffi::c_char,
            b"Logs cleared successfully\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(success);
        if json_str.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to convert success JSON to string\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            cJSON_Delete(success);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to convert success JSON to string\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
        free(json_str as *mut ::core::ffi::c_void);
        cJSON_Delete(success);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to clear log file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            log_file,
            strerror(*__errno_location()),
        );
        let mut error: *mut cJSON = cJSON_CreateObject();
        if error.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to create error JSON object\0".as_ptr() as *const ::core::ffi::c_char,
            );
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create error JSON\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddBoolToObject(
            error,
            b"success\0".as_ptr() as *const ::core::ffi::c_char,
            r#false,
        );
        cJSON_AddStringToObject(
            error,
            b"message\0".as_ptr() as *const ::core::ffi::c_char,
            b"Failed to clear logs\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut json_str_0: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(error);
        if json_str_0.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to convert error JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
            );
            cJSON_Delete(error);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to convert error JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        http_response_set_json(res, 500 as ::core::ffi::c_int, json_str_0);
        free(json_str_0 as *mut ::core::ffi::c_void);
        cJSON_Delete(error);
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled POST /api/system/logs/clear request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
