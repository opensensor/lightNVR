unsafe extern "C" {
    pub type __dirstream;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    static mut g_config: config_t;
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
    unsafe fn cJSON_CreateArray() -> *mut cJSON;
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
    unsafe fn authorization_filter_cameras(
        user: *const user_t,
        action: authorization_action_t,
        cameras: *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn authorization_evaluate(
        user: *const user_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn is_model_supported(model_path: *const ::core::ffi::c_char) -> bool;
    unsafe fn get_model_type(model_path: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    unsafe fn is_sod_available() -> bool;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
pub type __clockid_t = ::core::ffi::c_int;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type DIR = __dirstream;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct api_phase_timer_t {
    pub started: timespec,
    pub request: *const http_request_t,
    pub phase: *const ::core::ffi::c_char,
    pub valid: bool,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_camera_tag_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub label: [::core::ffi::c_char; 256],
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
pub struct user_role_t(pub ::core::ffi::c_uint);
impl user_role_t {
    pub const USER_ROLE_ADMIN: Self = Self(0);
    pub const USER_ROLE_USER: Self = Self(1);
    pub const USER_ROLE_VIEWER: Self = Self(2);
    pub const USER_ROLE_API: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct authorization_decision_t(pub ::core::ffi::c_uint);
impl authorization_decision_t {
    pub const AUTHZ_DECISION_DENY: Self = Self(0);
    pub const AUTHZ_DECISION_ALLOW: Self = Self(1);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct authorization_decision_source_t(pub ::core::ffi::c_uint);
impl authorization_decision_source_t {
    pub const AUTHZ_SOURCE_NONE: Self = Self(0);
    pub const AUTHZ_SOURCE_LEGACY_ROLE: Self = Self(1);
    pub const AUTHZ_SOURCE_POLICY_GRANT: Self = Self(2);
}
pub const LOG_COMPONENT: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"DetectionAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn api_phase_begin(
    mut request: *const http_request_t,
    mut phase: *const ::core::ffi::c_char,
) -> api_phase_timer_t {
    let mut timer: api_phase_timer_t = api_phase_timer_t {
        started: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        request: request,
        phase: phase,
        valid: false,
    };
    timer.valid = clock_gettime(CLOCK_MONOTONIC, &raw mut timer.started) == 0 as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"API phase starting: path=%s phase=%s request_id=%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw const (*request).path as *const ::core::ffi::c_char,
        phase,
        if (*request).request_id[0usize] as ::core::ffi::c_int != 0 {
            &raw const (*request).request_id as *const ::core::ffi::c_char
        } else {
            b"-\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return timer;
}
#[inline]
unsafe extern "C" fn api_phase_end(mut timer: *const api_phase_timer_t) {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if !(*timer).valid || clock_gettime(CLOCK_MONOTONIC, &raw mut now) != 0 as ::core::ffi::c_int {
        return;
    }
    let mut elapsed_ms: ::core::ffi::c_double =
        (now.tv_sec - (*timer).started.tv_sec) as ::core::ffi::c_double * 1000.0f64
            + (now.tv_nsec - (*timer).started.tv_nsec) as ::core::ffi::c_double / 1000000.0f64;
    if elapsed_ms >= 250.0f64 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Slow API phase: path=%s phase=%s duration_ms=%.0f request_id=%s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*(*timer).request).path as *const ::core::ffi::c_char,
            (*timer).phase,
            elapsed_ms,
            if (*(*timer).request).request_id[0usize] as ::core::ffi::c_int != 0 {
                &raw const (*(*timer).request).request_id as *const ::core::ffi::c_char
            } else {
                b"-\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
}
pub const MODEL_TYPE_SOD: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"sod\0") };
pub const MODEL_TYPE_TFLITE: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"tflite\0") };
pub const MODEL_TYPE_ONVIF: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"onvif\0") };
pub const MODEL_TYPE_API: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"api\0") };
pub const DEFAULT_MODELS_DIR: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"/var/lib/lightnvr/models\0")
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_detection_models(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut user: user_t = user_t {
        id: 0,
        username: [0; 64],
        email: [0; 128],
        role: user_role_t::USER_ROLE_ADMIN,
        api_key: [0; 64],
        created_at: 0,
        updated_at: 0,
        last_login: 0,
        is_active: false,
        password_change_locked: false,
        must_change_password: false,
        totp_enabled: false,
        allowed_login_cidrs: [0; 1024],
        has_login_cidr_restriction: false,
        authorization_mode: [0; 16],
        authenticated_via_scoped_token: false,
        api_token_uuid: [0; 37],
        authentication_method: [0; 24],
    };
    let mut phase: api_phase_timer_t = api_phase_begin(
        req,
        b"model list authentication\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut authenticated: bool = httpd_check_action_access(req, &raw mut user) != 0;
    api_phase_end(&raw mut phase);
    if !authenticated {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut global_evaluation: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t::AUTHZ_DECISION_DENY,
        source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
        policy_version: 0,
        grant_uuid: [0; 37],
        token_uuid: [0; 37],
        role_uuid: [0; 37],
        role_name: [0; 128],
        explanation: [0; 256],
    };
    let mut globally_allowed: bool = authorization_evaluate(
        &raw mut user,
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        ::core::ptr::null::<fleet_camera_t>(),
        &raw mut global_evaluation,
    ) == 0 as ::core::ffi::c_int
        && global_evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0;
    if !globally_allowed {
        let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
        let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if db_fleet_camera_load(&raw mut cameras, &raw mut camera_count) != 0 as ::core::ffi::c_int
            || authorization_filter_cameras(
                &raw mut user,
                authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
                cameras,
                &raw mut camera_count,
            ) != 0 as ::core::ffi::c_int
        {
            free(cameras as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        free(cameras as *mut ::core::ffi::c_void);
        if camera_count == 0 as ::core::ffi::c_int {
            http_response_set_json_error(
                res,
                403 as ::core::ffi::c_int,
                b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/detection/models request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut config: *const config_t = &raw mut g_config;
    let mut models_dir: *const ::core::ffi::c_char =
        &raw const (*config).models_path as *const ::core::ffi::c_char;
    if models_dir.is_null() || strlen(models_dir) == 0 as size_t {
        models_dir = DEFAULT_MODELS_DIR.as_ptr();
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Scanning models directory: %s\0".as_ptr() as *const ::core::ffi::c_char,
        models_dir,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    if response.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create response JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut models_array: *mut cJSON = cJSON_CreateArray();
    if models_array.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create models JSON array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(response);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create models JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddItemToObject(
        response,
        b"models\0".as_ptr() as *const ::core::ffi::c_char,
        models_array,
    );
    cJSON_AddBoolToObject(
        response,
        b"litert_available\0".as_ptr() as *const ::core::ffi::c_char,
        r#false,
    );
    cJSON_AddBoolToObject(
        response,
        b"sod_available\0".as_ptr() as *const ::core::ffi::c_char,
        is_sod_available() as cJSON_bool,
    );
    let mut model_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut motion_model: *mut cJSON = cJSON_CreateObject();
    if !motion_model.is_null() {
        cJSON_AddStringToObject(
            motion_model,
            b"id\0".as_ptr() as *const ::core::ffi::c_char,
            b"motion\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            motion_model,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            b"Built-in Motion Detection\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            motion_model,
            b"path\0".as_ptr() as *const ::core::ffi::c_char,
            b"motion\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            motion_model,
            b"type\0".as_ptr() as *const ::core::ffi::c_char,
            b"builtin\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddBoolToObject(
            motion_model,
            b"supported\0".as_ptr() as *const ::core::ffi::c_char,
            r#true,
        );
        cJSON_AddStringToObject(
            motion_model,
            b"description\0".as_ptr() as *const ::core::ffi::c_char,
            b"Built-in motion detection\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddItemToArray(models_array, motion_model);
        model_count += 1;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Added built-in motion detection model\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create motion model JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut api_model: *mut cJSON = cJSON_CreateObject();
    if !api_model.is_null() {
        cJSON_AddStringToObject(
            api_model,
            b"id\0".as_ptr() as *const ::core::ffi::c_char,
            b"api-detection\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            api_model,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            b"API Detection\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            api_model,
            b"path\0".as_ptr() as *const ::core::ffi::c_char,
            b"api-detection\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            api_model,
            b"type\0".as_ptr() as *const ::core::ffi::c_char,
            MODEL_TYPE_API.as_ptr(),
        );
        cJSON_AddBoolToObject(
            api_model,
            b"supported\0".as_ptr() as *const ::core::ffi::c_char,
            r#true,
        );
        cJSON_AddStringToObject(
            api_model,
            b"description\0".as_ptr() as *const ::core::ffi::c_char,
            b"External API-based object detection\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddItemToArray(models_array, api_model);
        model_count += 1;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Added API detection model\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create API model JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut onvif_model: *mut cJSON = cJSON_CreateObject();
    if !onvif_model.is_null() {
        cJSON_AddStringToObject(
            onvif_model,
            b"id\0".as_ptr() as *const ::core::ffi::c_char,
            b"onvif\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            onvif_model,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            b"ONVIF Motion Events\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            onvif_model,
            b"path\0".as_ptr() as *const ::core::ffi::c_char,
            b"onvif\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            onvif_model,
            b"type\0".as_ptr() as *const ::core::ffi::c_char,
            MODEL_TYPE_ONVIF.as_ptr(),
        );
        cJSON_AddBoolToObject(
            onvif_model,
            b"supported\0".as_ptr() as *const ::core::ffi::c_char,
            r#true,
        );
        cJSON_AddStringToObject(
            onvif_model,
            b"description\0".as_ptr() as *const ::core::ffi::c_char,
            b"ONVIF camera motion events detection\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddItemToArray(models_array, onvif_model);
        model_count += 1;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Added ONVIF detection model\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create ONVIF model JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    phase = api_phase_begin(
        req,
        b"model directory scan\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut dir: *mut DIR = opendir(models_dir);
    if dir.is_null() {
        api_phase_end(&raw mut phase);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Models directory does not exist: %s\0".as_ptr() as *const ::core::ffi::c_char,
            models_dir,
        );
        cJSON_AddStringToObject(
            response,
            b"message\0".as_ptr() as *const ::core::ffi::c_char,
            b"Models directory does not exist\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            response,
            b"count\0".as_ptr() as *const ::core::ffi::c_char,
            model_count as ::core::ffi::c_double,
        );
        let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
        if json_str.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to convert response JSON to string\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            cJSON_Delete(response);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to convert response JSON to string\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
        free(json_str as *mut ::core::ffi::c_void);
        cJSON_Delete(response);
        return;
    }
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        if strcmp(
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
            b".\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
                b"..\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            continue;
        }
        let mut full_path: [::core::ffi::c_char; 512] = [0; 512];
        snprintf(
            &raw mut full_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            models_dir,
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
        );
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
        if stat(&raw mut full_path as *mut ::core::ffi::c_char, &raw mut st)
            == 0 as ::core::ffi::c_int
            && st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
        {
            continue;
        }
        let mut supported: bool =
            is_model_supported(&raw mut full_path as *mut ::core::ffi::c_char);
        let mut ext: *const ::core::ffi::c_char = strrchr(
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
            '.' as ::core::ffi::c_int,
        );
        let mut is_tflite: bool = !ext.is_null()
            && strcasecmp(ext, b".tflite\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int;
        let mut is_sod: bool = !ext.is_null()
            && strcasecmp(ext, b".sod\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int;
        if !supported && !is_tflite && !is_sod {
            continue;
        }
        let mut model_type: *const ::core::ffi::c_char = if supported as ::core::ffi::c_int != 0 {
            get_model_type(&raw mut full_path as *mut ::core::ffi::c_char)
        } else if is_tflite as ::core::ffi::c_int != 0 {
            MODEL_TYPE_TFLITE.as_ptr()
        } else {
            MODEL_TYPE_SOD.as_ptr()
        };
        let mut model_obj: *mut cJSON = cJSON_CreateObject();
        if model_obj.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to create model JSON object\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            cJSON_AddStringToObject(
                model_obj,
                b"id\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut full_path as *mut ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                model_obj,
                b"name\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                model_obj,
                b"path\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut full_path as *mut ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                model_obj,
                b"type\0".as_ptr() as *const ::core::ffi::c_char,
                model_type,
            );
            cJSON_AddBoolToObject(
                model_obj,
                b"supported\0".as_ptr() as *const ::core::ffi::c_char,
                supported as cJSON_bool,
            );
            if !supported {
                cJSON_AddStringToObject(
                    model_obj,
                    b"unsupported_reason\0".as_ptr() as *const ::core::ffi::c_char,
                    if is_tflite as ::core::ffi::c_int != 0 {
                        b"This build has no in-process LiteRT engine (rebuild with -DENABLE_LITERT=ON and the third_party/litert submodule initialized)\0"
                            .as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"This build has no SOD support (rebuild with -DENABLE_SOD=ON)\0".as_ptr()
                            as *const ::core::ffi::c_char
                    },
                );
            }
            if stat(&raw mut full_path as *mut ::core::ffi::c_char, &raw mut st)
                == 0 as ::core::ffi::c_int
            {
                cJSON_AddNumberToObject(
                    model_obj,
                    b"size\0".as_ptr() as *const ::core::ffi::c_char,
                    st.st_size as ::core::ffi::c_double,
                );
            }
            cJSON_AddItemToArray(models_array, model_obj);
            model_count += 1;
        }
    }
    closedir(dir);
    api_phase_end(&raw mut phase);
    cJSON_AddNumberToObject(
        response,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        model_count as ::core::ffi::c_double,
    );
    let mut json_str_0: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    if json_str_0.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert response JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(response);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert response JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str_0);
    free(json_str_0 as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/detection/models request, found %d models\0".as_ptr()
            as *const ::core::ffi::c_char,
        model_count,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
