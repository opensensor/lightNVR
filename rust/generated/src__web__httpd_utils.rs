unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn inet_pton(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn inet_ntop(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_void,
        __buf: *mut ::core::ffi::c_char,
        __len: socklen_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn mbedtls_sha256(
        input: *const ::core::ffi::c_uchar,
        ilen: size_t,
        output: *mut ::core::ffi::c_uchar,
        is224: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddStringToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    static mut g_config: config_t;
    unsafe fn http_request_get_header(
        req: *const http_request_t,
        name: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    unsafe fn http_response_add_header(
        res: *mut http_response_t,
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_set_json_error(
        res: *mut http_response_t,
        status_code: ::core::ffi::c_int,
        error_message: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_get_user_by_id(user_id: int64_t, user: *mut user_t) -> ::core::ffi::c_int;
    unsafe fn db_auth_get_user_by_api_key(
        api_key: *const ::core::ffi::c_char,
        user: *mut user_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_authenticate(
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        user_id: *mut int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_validate_session_with_context(
        token: *const ::core::ffi::c_char,
        user_id: *mut int64_t,
        ip_address: *const ::core::ffi::c_char,
        user_agent: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_validate_session(
        token: *const ::core::ffi::c_char,
        user_id: *mut int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_get_role_name(role: user_role_t) -> *const ::core::ffi::c_char;
    unsafe fn db_auth_ip_allowed_for_user(
        user: *const user_t,
        client_ip: *const ::core::ffi::c_char,
    ) -> bool;
    unsafe fn authorization_action_metadata(
        action: authorization_action_t,
    ) -> *const authorization_action_metadata_t;
    unsafe fn authorization_evaluate(
        user: *const user_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn trim_ascii_whitespace(value: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn copy_trimmed_value(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
        input: *const ::core::ffi::c_char,
        input_size: size_t,
    ) -> size_t;
    unsafe fn db_api_token_authenticate(
        secret: *const ::core::ffi::c_char,
        user_id: *mut int64_t,
        token_uuid: *mut ::core::ffi::c_char,
        usage_audit_due: *mut bool,
    ) -> db_api_token_result_t;
    unsafe fn db_fleet_camera_find_by_name_readonly(
        stream_name: *const ::core::ffi::c_char,
        camera: *mut fleet_camera_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_find_by_uuid_readonly(
        camera_uuid: *const ::core::ffi::c_char,
        camera: *mut fleet_camera_t,
    ) -> ::core::ffi::c_int;
    unsafe fn audit_log_append(
        req: *const http_request_t,
        user: *const user_t,
        action: *const ::core::ffi::c_char,
        target_type: *const ::core::ffi::c_char,
        target_uuid: *const ::core::ffi::c_char,
        outcome: *const ::core::ffi::c_char,
        details: *const cJSON,
    );
    unsafe fn audit_log_authorization(
        req: *const http_request_t,
        user: *const user_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        evaluation: *const authorization_evaluation_t,
        outcome: *const ::core::ffi::c_char,
    );
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    unsafe fn strtok_r(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
        __save_ptr: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type time_t = __time_t;
pub type int64_t = i64;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const _ISupper: Self = Self(256);
    pub const _ISlower: Self = Self(512);
    pub const _ISalpha: Self = Self(1024);
    pub const _ISdigit: Self = Self(2048);
    pub const _ISxdigit: Self = Self(4096);
    pub const _ISspace: Self = Self(8192);
    pub const _ISprint: Self = Self(16384);
    pub const _ISgraph: Self = Self(32768);
    pub const _ISblank: Self = Self(1);
    pub const _IScntrl: Self = Self(2);
    pub const _ISpunct: Self = Self(4);
    pub const _ISalnum: Self = Self(8);
}
pub type socklen_t = __socklen_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
    pub const AUTHZ_ACTION_COUNT: Self = Self(19);
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
pub struct media_auth_cache_entry_t {
    pub occupied: bool,
    pub fingerprint: [::core::ffi::c_char; 65],
    pub stream_name: [::core::ffi::c_char; 256],
    pub action: authorization_action_t,
    pub authorized_until: time_t,
    pub last_used: time_t,
}
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const LOG_COMPONENT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"HTTP\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MEDIA_AUTH_CACHE_CAPACITY: ::core::ffi::c_int =
    if (MAX_STREAMS * 2 as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
        64 as ::core::ffi::c_int
    } else if MAX_STREAMS * 2 as ::core::ffi::c_int > 2048 as ::core::ffi::c_int {
        2048 as ::core::ffi::c_int
    } else {
        MAX_STREAMS * 2 as ::core::ffi::c_int
    };
pub const MEDIA_AUTH_CACHE_TTL_SECONDS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
static mut g_media_auth_cache: [media_auth_cache_entry_t; 2048] = [media_auth_cache_entry_t {
    occupied: false,
    fingerprint: [0; 65],
    stream_name: [0; 256],
    action: authorization_action_t::AUTHZ_LIVE_VIEW,
    authorized_until: 0,
    last_used: 0,
}; 2048];
static mut g_media_auth_cache_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_0::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_parse_json_body(mut req: *const http_request_t) -> *mut cJSON {
    if req.is_null() || (*req).body.is_null() || (*req).body_len == 0 as size_t {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut body: *mut ::core::ffi::c_char =
        malloc((*req).body_len.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    if body.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to allocate memory for request body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<cJSON>();
    }
    memcpy(
        body as *mut ::core::ffi::c_void,
        (*req).body,
        (*req).body_len,
    );
    *body.offset((*req).body_len as isize) = '\0' as ::core::ffi::c_char;
    let mut json: *mut cJSON = cJSON_Parse(body);
    free(body as *mut ::core::ffi::c_void);
    if json.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to parse JSON from request body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<cJSON>();
    }
    return json;
}
unsafe extern "C" fn base64_decode(
    mut src: *const ::core::ffi::c_char,
    mut src_len: size_t,
    mut dst: *mut ::core::ffi::c_char,
    mut dst_size: size_t,
) -> ::core::ffi::c_int {
    static mut table: [::core::ffi::c_uchar; 256] = [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        62 as ::core::ffi::c_uchar,
        0,
        0,
        0,
        63 as ::core::ffi::c_uchar,
        52 as ::core::ffi::c_uchar,
        53 as ::core::ffi::c_uchar,
        54 as ::core::ffi::c_uchar,
        55 as ::core::ffi::c_uchar,
        56 as ::core::ffi::c_uchar,
        57 as ::core::ffi::c_uchar,
        58 as ::core::ffi::c_uchar,
        59 as ::core::ffi::c_uchar,
        60 as ::core::ffi::c_uchar,
        61 as ::core::ffi::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0 as ::core::ffi::c_uchar,
        1 as ::core::ffi::c_uchar,
        2 as ::core::ffi::c_uchar,
        3 as ::core::ffi::c_uchar,
        4 as ::core::ffi::c_uchar,
        5 as ::core::ffi::c_uchar,
        6 as ::core::ffi::c_uchar,
        7 as ::core::ffi::c_uchar,
        8 as ::core::ffi::c_uchar,
        9 as ::core::ffi::c_uchar,
        10 as ::core::ffi::c_uchar,
        11 as ::core::ffi::c_uchar,
        12 as ::core::ffi::c_uchar,
        13 as ::core::ffi::c_uchar,
        14 as ::core::ffi::c_uchar,
        15 as ::core::ffi::c_uchar,
        16 as ::core::ffi::c_uchar,
        17 as ::core::ffi::c_uchar,
        18 as ::core::ffi::c_uchar,
        19 as ::core::ffi::c_uchar,
        20 as ::core::ffi::c_uchar,
        21 as ::core::ffi::c_uchar,
        22 as ::core::ffi::c_uchar,
        23 as ::core::ffi::c_uchar,
        24 as ::core::ffi::c_uchar,
        25 as ::core::ffi::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        26 as ::core::ffi::c_uchar,
        27 as ::core::ffi::c_uchar,
        28 as ::core::ffi::c_uchar,
        29 as ::core::ffi::c_uchar,
        30 as ::core::ffi::c_uchar,
        31 as ::core::ffi::c_uchar,
        32 as ::core::ffi::c_uchar,
        33 as ::core::ffi::c_uchar,
        34 as ::core::ffi::c_uchar,
        35 as ::core::ffi::c_uchar,
        36 as ::core::ffi::c_uchar,
        37 as ::core::ffi::c_uchar,
        38 as ::core::ffi::c_uchar,
        39 as ::core::ffi::c_uchar,
        40 as ::core::ffi::c_uchar,
        41 as ::core::ffi::c_uchar,
        42 as ::core::ffi::c_uchar,
        43 as ::core::ffi::c_uchar,
        44 as ::core::ffi::c_uchar,
        45 as ::core::ffi::c_uchar,
        46 as ::core::ffi::c_uchar,
        47 as ::core::ffi::c_uchar,
        48 as ::core::ffi::c_uchar,
        49 as ::core::ffi::c_uchar,
        50 as ::core::ffi::c_uchar,
        51 as ::core::ffi::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut j: size_t = 0 as size_t;
    let mut accum: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut bits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    while i < src_len && *src.offset(i as isize) as ::core::ffi::c_int != '=' as ::core::ffi::c_int
    {
        let mut c: ::core::ffi::c_uchar = *src.offset(i as isize) as ::core::ffi::c_uchar;
        if !(c as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int)
        {
            accum = accum << 6 as ::core::ffi::c_int | table[c as usize] as ::core::ffi::c_uint;
            bits += 6 as ::core::ffi::c_int;
            if bits >= 8 as ::core::ffi::c_int {
                bits -= 8 as ::core::ffi::c_int;
                if j < dst_size.wrapping_sub(1 as size_t) {
                    let c2rust_fresh0 = j;
                    j = j.wrapping_add(1);
                    *dst.offset(c2rust_fresh0 as isize) =
                        (accum >> bits & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_char;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    *dst.offset(j as isize) = '\0' as ::core::ffi::c_char;
    return j as ::core::ffi::c_int;
}
unsafe extern "C" fn normalize_ip_literal(
    mut input: *const ::core::ffi::c_char,
    mut normalized: *mut ::core::ffi::c_char,
    mut normalized_size: size_t,
) -> bool {
    if input.is_null() || normalized.is_null() || normalized_size == 0 as size_t {
        return r#false != 0;
    }
    *normalized.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut candidate_buffer: [::core::ffi::c_char; 54] = [0; 54];
    if strlen(input) >= ::core::mem::size_of::<[::core::ffi::c_char; 54]>() {
        return r#false != 0;
    }
    safe_strcpy(
        &raw mut candidate_buffer as *mut ::core::ffi::c_char,
        input,
        ::core::mem::size_of::<[::core::ffi::c_char; 54]>(),
        0 as size_t,
    );
    let mut candidate: *mut ::core::ffi::c_char =
        trim_ascii_whitespace(&raw mut candidate_buffer as *mut ::core::ffi::c_char);
    if candidate.is_null()
        || *candidate.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    if *candidate.offset(0isize) as ::core::ffi::c_int == '[' as ::core::ffi::c_int {
        let mut closing: *mut ::core::ffi::c_char = strchr(candidate, ']' as ::core::ffi::c_int);
        if closing.is_null()
            || *closing.offset(1isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        *closing = '\0' as ::core::ffi::c_char;
        candidate = candidate.offset(1);
    }
    let mut addr: [::core::ffi::c_uchar; 16] = [
        0 as ::core::ffi::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
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
    if inet_pton(
        AF_INET,
        candidate,
        &raw mut addr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
    ) == 1 as ::core::ffi::c_int
    {
        return !inet_ntop(
            AF_INET,
            &raw mut addr as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            normalized,
            normalized_size as socklen_t,
        )
        .is_null();
    }
    if inet_pton(
        AF_INET6,
        candidate,
        &raw mut addr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
    ) == 1 as ::core::ffi::c_int
    {
        return !inet_ntop(
            AF_INET6,
            &raw mut addr as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            normalized,
            normalized_size as socklen_t,
        )
        .is_null();
    }
    return r#false != 0;
}
unsafe extern "C" fn ip_matches_cidr_list(
    mut cidr_list: *const ::core::ffi::c_char,
    mut ip: *const ::core::ffi::c_char,
) -> bool {
    if cidr_list.is_null()
        || *cidr_list.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || ip.is_null()
        || *ip.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut rules: user_t = user_t {
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
    memset(
        &raw mut rules as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    safe_strcpy(
        &raw mut rules.allowed_login_cidrs as *mut ::core::ffi::c_char,
        cidr_list,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        0 as size_t,
    );
    rules.has_login_cidr_restriction = r#true != 0;
    return db_auth_ip_allowed_for_user(&raw mut rules, ip);
}
unsafe extern "C" fn resolve_client_ip_from_forwarded_headers(
    mut req: *const http_request_t,
    mut client_ip: *mut ::core::ffi::c_char,
    mut client_ip_size: size_t,
) -> ::core::ffi::c_int {
    let mut xff: *const ::core::ffi::c_char = http_request_get_header(
        req,
        b"X-Forwarded-For\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !xff.is_null() && *xff.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut xff_copy: [::core::ffi::c_char; 1024] = [0; 1024];
        if strlen(xff) >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() {
            return -1 as ::core::ffi::c_int;
        }
        safe_strcpy(
            &raw mut xff_copy as *mut ::core::ffi::c_char,
            xff,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            0 as size_t,
        );
        let mut normalized_tokens: [[::core::ffi::c_char; 46]; 16] = [
            [
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
            ],
            [0; 46],
            [0; 46],
            [0; 46],
            [0; 46],
            [0; 46],
            [0; 46],
            [0; 46],
            [0; 46],
            [0; 46],
            [0; 46],
            [0; 46],
            [0; 46],
            [0; 46],
            [0; 46],
            [0; 46],
        ];
        let mut token_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut token: *mut ::core::ffi::c_char = strtok_r(
            &raw mut xff_copy as *mut ::core::ffi::c_char,
            b",\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
        while !token.is_null() && token_count < 16 as ::core::ffi::c_int {
            let mut normalized: [::core::ffi::c_char; 46] = [
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
            ];
            if !normalize_ip_literal(
                token,
                &raw mut normalized as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 46]>(),
            ) {
                return -1 as ::core::ffi::c_int;
            }
            safe_strcpy(
                &raw mut *(&raw mut normalized_tokens as *mut [::core::ffi::c_char; 46])
                    .offset(token_count as isize) as *mut ::core::ffi::c_char,
                &raw mut normalized as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 46]>(),
                0 as size_t,
            );
            token_count += 1;
            token = strtok_r(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                b",\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut saveptr,
            );
        }
        if token_count > 0 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = token_count - 1 as ::core::ffi::c_int;
            while i >= 0 as ::core::ffi::c_int {
                if !ip_matches_cidr_list(
                    &raw mut g_config.trusted_proxy_cidrs as *mut ::core::ffi::c_char,
                    &raw mut *(&raw mut normalized_tokens as *mut [::core::ffi::c_char; 46])
                        .offset(i as isize) as *mut ::core::ffi::c_char,
                ) {
                    safe_strcpy(
                        client_ip,
                        &raw mut *(&raw mut normalized_tokens as *mut [::core::ffi::c_char; 46])
                            .offset(i as isize) as *mut ::core::ffi::c_char,
                        client_ip_size,
                        0 as size_t,
                    );
                    return 0 as ::core::ffi::c_int;
                }
                i -= 1;
            }
        }
    }
    let mut x_real_ip: *const ::core::ffi::c_char =
        http_request_get_header(req, b"X-Real-IP\0".as_ptr() as *const ::core::ffi::c_char);
    if !x_real_ip.is_null()
        && *x_real_ip.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        let mut normalized_0: [::core::ffi::c_char; 46] = [
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
        ];
        if !normalize_ip_literal(
            x_real_ip,
            &raw mut normalized_0 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 46]>(),
        ) {
            return -1 as ::core::ffi::c_int;
        }
        safe_strcpy(
            client_ip,
            &raw mut normalized_0 as *mut ::core::ffi::c_char,
            client_ip_size,
            0 as size_t,
        );
        return 0 as ::core::ffi::c_int;
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_get_basic_auth_credentials(
    mut req: *const http_request_t,
    mut username: *mut ::core::ffi::c_char,
    mut username_size: size_t,
    mut password: *mut ::core::ffi::c_char,
    mut password_size: size_t,
) -> ::core::ffi::c_int {
    if req.is_null() || username.is_null() || password.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *username.offset(0isize) = '\0' as ::core::ffi::c_char;
    *password.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut auth: *const ::core::ffi::c_char = http_request_get_header(
        req,
        b"Authorization\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if auth.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if strncasecmp(
        auth,
        b"Basic \0".as_ptr() as *const ::core::ffi::c_char,
        6 as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut encoded: *const ::core::ffi::c_char = auth.offset(6 as ::core::ffi::c_int as isize);
    while *encoded as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
        encoded = encoded.offset(1);
    }
    let mut decoded: [::core::ffi::c_char; 256] = [
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
    base64_decode(
        encoded,
        strlen(encoded),
        &raw mut decoded as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    let mut colon: *mut ::core::ffi::c_char = strchr(
        &raw mut decoded as *mut ::core::ffi::c_char,
        ':' as ::core::ffi::c_int,
    );
    if colon.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *colon = '\0' as ::core::ffi::c_char;
    safe_strcpy(
        username,
        &raw mut decoded as *mut ::core::ffi::c_char,
        username_size,
        0 as size_t,
    );
    safe_strcpy(
        password,
        colon.offset(1 as ::core::ffi::c_int as isize),
        password_size,
        0 as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_get_api_key(
    mut req: *const http_request_t,
    mut api_key: *mut ::core::ffi::c_char,
    mut api_key_size: size_t,
) -> ::core::ffi::c_int {
    if req.is_null() || api_key.is_null() || api_key_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    *api_key.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut header_value: *const ::core::ffi::c_char =
        http_request_get_header(req, b"X-API-Key\0".as_ptr() as *const ::core::ffi::c_char);
    if !header_value.is_null()
        && copy_trimmed_value(api_key, api_key_size, header_value, 0 as size_t) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut auth: *const ::core::ffi::c_char = http_request_get_header(
        req,
        b"Authorization\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if auth.is_null()
        || strncasecmp(
            auth,
            b"Bearer \0".as_ptr() as *const ::core::ffi::c_char,
            7 as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    return if copy_trimmed_value(
        api_key,
        api_key_size,
        auth.offset(7 as ::core::ffi::c_int as isize),
        0 as size_t,
    ) != 0
    {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_peer_is_trusted_proxy(mut req: *const http_request_t) -> bool {
    if req.is_null() || (*req).client_ip[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    if g_config.trusted_proxy_cidrs[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut peer_ip: [::core::ffi::c_char; 46] = [
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
    ];
    if !normalize_ip_literal(
        &raw const (*req).client_ip as *const ::core::ffi::c_char,
        &raw mut peer_ip as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 46]>(),
    ) {
        return r#false != 0;
    }
    return ip_matches_cidr_list(
        &raw mut g_config.trusted_proxy_cidrs as *mut ::core::ffi::c_char,
        &raw mut peer_ip as *mut ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_get_effective_client_ip(
    mut req: *const http_request_t,
    mut client_ip: *mut ::core::ffi::c_char,
    mut client_ip_size: size_t,
) -> ::core::ffi::c_int {
    if req.is_null() || client_ip.is_null() || client_ip_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    *client_ip.offset(0isize) = '\0' as ::core::ffi::c_char;
    if (*req).client_ip[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut peer_ip: [::core::ffi::c_char; 46] = [
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
    ];
    if !normalize_ip_literal(
        &raw const (*req).client_ip as *const ::core::ffi::c_char,
        &raw mut peer_ip as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 46]>(),
    ) {
        return if copy_trimmed_value(
            client_ip,
            client_ip_size,
            &raw const (*req).client_ip as *const ::core::ffi::c_char,
            0 as size_t,
        ) != 0
        {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    if g_config.trusted_proxy_cidrs[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && ip_matches_cidr_list(
            &raw mut g_config.trusted_proxy_cidrs as *mut ::core::ffi::c_char,
            &raw mut peer_ip as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        && resolve_client_ip_from_forwarded_headers(req, client_ip, client_ip_size)
            == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    safe_strcpy(
        client_ip,
        &raw mut peer_ip as *mut ::core::ffi::c_char,
        client_ip_size,
        0 as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_get_cookie_value(
    mut req: *const http_request_t,
    mut cookie_name: *const ::core::ffi::c_char,
    mut value: *mut ::core::ffi::c_char,
    mut value_size: size_t,
) -> ::core::ffi::c_int {
    if req.is_null() || cookie_name.is_null() || value.is_null() || value_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    *value.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut cookie: *const ::core::ffi::c_char =
        http_request_get_header(req, b"Cookie\0".as_ptr() as *const ::core::ffi::c_char);
    if cookie.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut name_len: size_t = strlen(cookie_name);
    let mut cursor: *const ::core::ffi::c_char = cookie;
    while !cursor.is_null() && *cursor as ::core::ffi::c_int != 0 {
        while *cursor as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *cursor as ::core::ffi::c_int == ';' as ::core::ffi::c_int
        {
            cursor = cursor.offset(1);
        }
        if strncmp(cursor, cookie_name, name_len) == 0 as ::core::ffi::c_int
            && *cursor.offset(name_len as isize) as ::core::ffi::c_int == '=' as ::core::ffi::c_int
        {
            let mut value_start: *const ::core::ffi::c_char = cursor
                .offset(name_len as isize)
                .offset(1 as ::core::ffi::c_int as isize);
            let mut value_end: *const ::core::ffi::c_char =
                strchr(value_start, ';' as ::core::ffi::c_int);
            let mut len: size_t = if !value_end.is_null() {
                value_end.offset_from(value_start) as size_t
            } else {
                strlen(value_start)
            };
            if len == 0 as size_t || len >= value_size {
                return -1 as ::core::ffi::c_int;
            }
            memcpy(
                value as *mut ::core::ffi::c_void,
                value_start as *const ::core::ffi::c_void,
                len,
            );
            *value.offset(len as isize) = '\0' as ::core::ffi::c_char;
            return 0 as ::core::ffi::c_int;
        }
        cursor = strchr(cursor, ';' as ::core::ffi::c_int);
        if !cursor.is_null() {
            cursor = cursor.offset(1);
        }
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_get_session_token(
    mut req: *const http_request_t,
    mut token: *mut ::core::ffi::c_char,
    mut token_size: size_t,
) -> ::core::ffi::c_int {
    return httpd_get_cookie_value(
        req,
        b"session\0".as_ptr() as *const ::core::ffi::c_char,
        token,
        token_size,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_auth_absolute_timeout_seconds() -> ::core::ffi::c_int {
    let mut seconds: int64_t = g_config.auth_absolute_timeout_hours as int64_t * 3600 as int64_t;
    if seconds <= 0 as int64_t || seconds > INT32_MAX as int64_t {
        return 604800 as ::core::ffi::c_int;
    }
    return seconds as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_trusted_device_lifetime_seconds() -> ::core::ffi::c_int {
    let mut seconds: int64_t = g_config.trusted_device_days as int64_t * 86400 as int64_t;
    if seconds <= 0 as int64_t || seconds > INT32_MAX as int64_t {
        return 0 as ::core::ffi::c_int;
    }
    return seconds as ::core::ffi::c_int;
}
static mut g_secure_cookies: bool = r#false != 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_set_secure_cookies(mut secure: bool) {
    ::core::ptr::write_volatile(&raw mut g_secure_cookies, secure);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_secure_cookies() -> bool {
    return ::core::ptr::read_volatile::<bool>(&raw const g_secure_cookies);
}
unsafe extern "C" fn cookie_secure_attribute() -> *const ::core::ffi::c_char {
    return if ::core::ptr::read_volatile::<bool>(&raw const g_secure_cookies) as ::core::ffi::c_int
        != 0
    {
        b"; Secure\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_add_session_cookie(
    mut res: *mut http_response_t,
    mut token: *const ::core::ffi::c_char,
) {
    if res.is_null()
        || token.is_null()
        || *token.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return;
    }
    let mut cookie_header: [::core::ffi::c_char; 256] = [0; 256];
    snprintf(
        &raw mut cookie_header as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"session=%s; Path=/; Max-Age=%d; HttpOnly; SameSite=Lax%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        token,
        httpd_auth_absolute_timeout_seconds(),
        cookie_secure_attribute(),
    );
    http_response_add_header(
        res,
        b"Set-Cookie\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut cookie_header as *mut ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_clear_session_cookie(mut res: *mut http_response_t) {
    if res.is_null() {
        return;
    }
    let mut cookie_header: [::core::ffi::c_char; 128] = [0; 128];
    snprintf(
        &raw mut cookie_header as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        b"session=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        cookie_secure_attribute(),
    );
    http_response_add_header(
        res,
        b"Set-Cookie\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut cookie_header as *mut ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_add_trusted_device_cookie(
    mut res: *mut http_response_t,
    mut token: *const ::core::ffi::c_char,
) {
    let mut lifetime: ::core::ffi::c_int = httpd_trusted_device_lifetime_seconds();
    if res.is_null()
        || token.is_null()
        || *token.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || lifetime <= 0 as ::core::ffi::c_int
    {
        return;
    }
    let mut cookie_header: [::core::ffi::c_char; 256] = [0; 256];
    snprintf(
        &raw mut cookie_header as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"trusted_device=%s; Path=/; Max-Age=%d; HttpOnly; SameSite=Lax%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        token,
        lifetime,
        cookie_secure_attribute(),
    );
    http_response_add_header(
        res,
        b"Set-Cookie\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut cookie_header as *mut ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_clear_trusted_device_cookie(mut res: *mut http_response_t) {
    if res.is_null() {
        return;
    }
    let mut cookie_header: [::core::ffi::c_char; 128] = [0; 128];
    snprintf(
        &raw mut cookie_header as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        b"trusted_device=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        cookie_secure_attribute(),
    );
    http_response_add_header(
        res,
        b"Set-Cookie\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut cookie_header as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn request_allowed_during_required_password_change(
    mut req: *const http_request_t,
    mut user: *const user_t,
) -> bool {
    if req.is_null()
        || user.is_null()
        || !(*user).must_change_password
        || g_config.demo_mode as ::core::ffi::c_int != 0
    {
        return r#true != 0;
    }
    if (*req).method.0 == http_method_t::HTTP_METHOD_GET.0
        && strcmp(
            &raw const (*req).path as *const ::core::ffi::c_char,
            b"/api/auth/verify\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    let mut password_path: [::core::ffi::c_char; 128] = [0; 128];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut password_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        b"/api/auth/users/%lld/password\0".as_ptr() as *const ::core::ffi::c_char,
        (*user).id as ::core::ffi::c_longlong,
    );
    return written > 0 as ::core::ffi::c_int
        && (written as size_t) < ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
        && (*req).method.0 == http_method_t::HTTP_METHOD_PUT.0
        && strcmp(
            &raw const (*req).path as *const ::core::ffi::c_char,
            &raw mut password_path as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn get_authenticated_user(
    mut req: *const http_request_t,
    mut user: *mut user_t,
    mut allow_scoped_token: bool,
) -> ::core::ffi::c_int {
    if req.is_null() || user.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut effective_client_ip: [::core::ffi::c_char; 64] = [
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
    ];
    if httpd_get_effective_client_ip(
        req,
        &raw mut effective_client_ip as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut effective_client_ip as *mut ::core::ffi::c_char,
            &raw const (*req).client_ip as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    if !g_config.web_auth_enabled {
        memset(
            user as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<user_t>(),
        );
        safe_strcpy(
            &raw mut (*user).username as *mut ::core::ffi::c_char,
            b"admin\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*user).authentication_method as *mut ::core::ffi::c_char,
            b"auth_disabled\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
            0 as size_t,
        );
        (*user).role = user_role_t::USER_ROLE_ADMIN;
        (*user).is_active = r#true != 0;
        return 1 as ::core::ffi::c_int;
    }
    let mut session_token: [::core::ffi::c_char; 64] = [
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
    ];
    if httpd_get_session_token(
        req,
        &raw mut session_token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) == 0 as ::core::ffi::c_int
    {
        let mut user_id: int64_t = 0;
        let mut rc: ::core::ffi::c_int = db_auth_validate_session(
            &raw mut session_token as *mut ::core::ffi::c_char,
            &raw mut user_id,
        );
        if rc == 0 as ::core::ffi::c_int {
            rc = db_auth_get_user_by_id(user_id, user);
            if rc == 0 as ::core::ffi::c_int
                && db_auth_ip_allowed_for_user(
                    user,
                    &raw mut effective_client_ip as *mut ::core::ffi::c_char,
                ) as ::core::ffi::c_int
                    != 0
            {
                rc = db_auth_validate_session_with_context(
                    &raw mut session_token as *mut ::core::ffi::c_char,
                    &raw mut user_id,
                    &raw mut effective_client_ip as *mut ::core::ffi::c_char,
                    &raw const (*req).user_agent as *const ::core::ffi::c_char,
                );
                if rc == 0 as ::core::ffi::c_int {
                    safe_strcpy(
                        &raw mut (*user).authentication_method as *mut ::core::ffi::c_char,
                        b"session\0".as_ptr() as *const ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
                        0 as size_t,
                    );
                    if request_allowed_during_required_password_change(req, user) {
                        return 1 as ::core::ffi::c_int;
                    }
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Session access restricted pending password change for user '%s'\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*user).username as *mut ::core::ffi::c_char,
                    );
                }
            } else if rc == 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Session auth blocked by allowed_login_cidrs for user '%s' from IP %s\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*user).username as *mut ::core::ffi::c_char,
                    if effective_client_ip[0usize] as ::core::ffi::c_int
                        != '\0' as ::core::ffi::c_int
                    {
                        &raw mut effective_client_ip as *mut ::core::ffi::c_char
                    } else {
                        b"(unknown)\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
            }
        }
    }
    let mut username: [::core::ffi::c_char; 64] = [
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
    ];
    let mut password: [::core::ffi::c_char; 64] = [
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
    ];
    if httpd_get_basic_auth_credentials(
        req,
        &raw mut username as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        &raw mut password as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) == 0 as ::core::ffi::c_int
    {
        if username[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && password[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            let mut user_id_0: int64_t = 0;
            let mut rc_0: ::core::ffi::c_int = db_auth_authenticate(
                &raw mut username as *mut ::core::ffi::c_char,
                &raw mut password as *mut ::core::ffi::c_char,
                &raw mut user_id_0,
            );
            if rc_0 == 0 as ::core::ffi::c_int {
                rc_0 = db_auth_get_user_by_id(user_id_0, user);
                if rc_0 == 0 as ::core::ffi::c_int
                    && db_auth_ip_allowed_for_user(
                        user,
                        &raw mut effective_client_ip as *mut ::core::ffi::c_char,
                    ) as ::core::ffi::c_int
                        != 0
                {
                    safe_strcpy(
                        &raw mut (*user).authentication_method as *mut ::core::ffi::c_char,
                        b"basic\0".as_ptr() as *const ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
                        0 as size_t,
                    );
                    if request_allowed_during_required_password_change(req, user) {
                        return 1 as ::core::ffi::c_int;
                    }
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Basic auth access restricted pending password change for user '%s'\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*user).username as *mut ::core::ffi::c_char,
                    );
                } else if rc_0 == 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Basic auth blocked by allowed_login_cidrs for user '%s' from IP %s\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*user).username as *mut ::core::ffi::c_char,
                        if effective_client_ip[0usize] as ::core::ffi::c_int
                            != '\0' as ::core::ffi::c_int
                        {
                            &raw mut effective_client_ip as *mut ::core::ffi::c_char
                        } else {
                            b"(unknown)\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                }
            }
        }
    }
    let mut api_key: [::core::ffi::c_char; 128] = [
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
    ];
    if httpd_get_api_key(
        req,
        &raw mut api_key as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    ) == 0 as ::core::ffi::c_int
    {
        let mut rc_1: ::core::ffi::c_int =
            db_auth_get_user_by_api_key(&raw mut api_key as *mut ::core::ffi::c_char, user);
        if rc_1 == 0 as ::core::ffi::c_int
            && (*user).is_active as ::core::ffi::c_int != 0
            && db_auth_ip_allowed_for_user(
                user,
                &raw mut effective_client_ip as *mut ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
        {
            safe_strcpy(
                &raw mut (*user).authentication_method as *mut ::core::ffi::c_char,
                b"legacy_api_key\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
                0 as size_t,
            );
            return 1 as ::core::ffi::c_int;
        }
        if rc_1 == 0 as ::core::ffi::c_int && (*user).is_active as ::core::ffi::c_int != 0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"API key auth blocked by allowed_login_cidrs for user '%s' from IP %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*user).username as *mut ::core::ffi::c_char,
                if effective_client_ip[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                    &raw mut effective_client_ip as *mut ::core::ffi::c_char
                } else {
                    b"(unknown)\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        if rc_1 != 0 as ::core::ffi::c_int && allow_scoped_token as ::core::ffi::c_int != 0 {
            let mut user_id_1: int64_t = 0 as int64_t;
            let mut token_uuid: [::core::ffi::c_char; 37] = [
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
            ];
            let mut usage_audit_due: bool = r#false != 0;
            let mut token_result: db_api_token_result_t = db_api_token_authenticate(
                &raw mut api_key as *mut ::core::ffi::c_char,
                &raw mut user_id_1,
                &raw mut token_uuid as *mut ::core::ffi::c_char,
                &raw mut usage_audit_due,
            );
            if token_result.0 == db_api_token_result_t::DB_API_TOKEN_OK.0
                && db_auth_get_user_by_id(user_id_1, user) == 0 as ::core::ffi::c_int
                && (*user).is_active as ::core::ffi::c_int != 0
                && db_auth_ip_allowed_for_user(
                    user,
                    &raw mut effective_client_ip as *mut ::core::ffi::c_char,
                ) as ::core::ffi::c_int
                    != 0
            {
                (*user).authenticated_via_scoped_token = r#true != 0;
                safe_strcpy(
                    &raw mut (*user).authentication_method as *mut ::core::ffi::c_char,
                    b"scoped_token\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
                    0 as size_t,
                );
                safe_strcpy(
                    &raw mut (*user).api_token_uuid as *mut ::core::ffi::c_char,
                    &raw mut token_uuid as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                    0 as size_t,
                );
                if usage_audit_due {
                    let mut details: *mut cJSON = cJSON_CreateObject();
                    if !details.is_null() {
                        cJSON_AddStringToObject(
                            details,
                            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
                            b"api_token.use\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        cJSON_AddStringToObject(
                            details,
                            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
                            b"active\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    audit_log_append(
                        req,
                        user,
                        b"api_token.use\0".as_ptr() as *const ::core::ffi::c_char,
                        b"api_token\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut token_uuid as *mut ::core::ffi::c_char,
                        b"success\0".as_ptr() as *const ::core::ffi::c_char,
                        details,
                    );
                    cJSON_Delete(details);
                }
                return 1 as ::core::ffi::c_int;
            }
            if strncmp(
                &raw mut api_key as *mut ::core::ffi::c_char,
                b"lnvr_\0".as_ptr() as *const ::core::ffi::c_char,
                5 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                let mut denied_user: user_t = user_t {
                    id: 0 as int64_t,
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
                let mut principal: *mut user_t = ::core::ptr::null_mut::<user_t>();
                if user_id_1 > 0 as int64_t
                    && db_auth_get_user_by_id(user_id_1, &raw mut denied_user)
                        == 0 as ::core::ffi::c_int
                {
                    safe_strcpy(
                        &raw mut denied_user.authentication_method as *mut ::core::ffi::c_char,
                        b"scoped_token\0".as_ptr() as *const ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
                        0 as size_t,
                    );
                    denied_user.authenticated_via_scoped_token = r#true != 0;
                    safe_strcpy(
                        &raw mut denied_user.api_token_uuid as *mut ::core::ffi::c_char,
                        &raw mut token_uuid as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                        0 as size_t,
                    );
                    principal = &raw mut denied_user;
                }
                let mut reason: *const ::core::ffi::c_char = if token_result.0
                    == db_api_token_result_t::DB_API_TOKEN_EXPIRED.0
                {
                    b"expired\0".as_ptr() as *const ::core::ffi::c_char
                } else if token_result.0 == db_api_token_result_t::DB_API_TOKEN_REVOKED.0 {
                    b"revoked\0".as_ptr() as *const ::core::ffi::c_char
                } else if token_result.0 == db_api_token_result_t::DB_API_TOKEN_INACTIVE_OWNER.0 {
                    b"inactive_owner\0".as_ptr() as *const ::core::ffi::c_char
                } else if token_result.0 == db_api_token_result_t::DB_API_TOKEN_ERROR.0 {
                    b"error\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"unknown\0".as_ptr() as *const ::core::ffi::c_char
                };
                let mut details_0: *mut cJSON = cJSON_CreateObject();
                if !details_0.is_null() {
                    cJSON_AddStringToObject(
                        details_0,
                        b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
                        b"api_token.use\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    cJSON_AddStringToObject(
                        details_0,
                        b"reason\0".as_ptr() as *const ::core::ffi::c_char,
                        reason,
                    );
                }
                audit_log_append(
                    req,
                    principal,
                    b"api_token.use\0".as_ptr() as *const ::core::ffi::c_char,
                    b"api_token\0".as_ptr() as *const ::core::ffi::c_char,
                    if token_uuid[0usize] as ::core::ffi::c_int != 0 {
                        &raw mut token_uuid as *mut ::core::ffi::c_char
                    } else {
                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                    },
                    b"denied\0".as_ptr() as *const ::core::ffi::c_char,
                    details_0,
                );
                cJSON_Delete(details_0);
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_get_authenticated_user(
    mut req: *const http_request_t,
    mut user: *mut user_t,
) -> ::core::ffi::c_int {
    return get_authenticated_user(req, user, r#false != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_check_admin_privileges(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) -> ::core::ffi::c_int {
    if !g_config.web_auth_enabled {
        return 1 as ::core::ffi::c_int;
    }
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
    if httpd_get_authenticated_user(req, &raw mut user) != 0 {
        if user.role.0 == user_role_t::USER_ROLE_ADMIN.0 {
            return 1 as ::core::ffi::c_int;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Access denied: User '%s' (role: %s) attempted admin action\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut user.username as *mut ::core::ffi::c_char,
            db_auth_get_role_name(user.role),
        );
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Forbidden: Admin privileges required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Access denied: Unauthenticated request attempted admin action\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    http_response_set_json_error(
        res,
        401 as ::core::ffi::c_int,
        b"Unauthorized: Authentication required\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_is_demo_mode() -> ::core::ffi::c_int {
    return if g_config.demo_mode as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn check_viewer_access(
    mut req: *const http_request_t,
    mut user: *mut user_t,
    mut allow_scoped_token: bool,
) -> ::core::ffi::c_int {
    if user.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if get_authenticated_user(req, user, allow_scoped_token) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if g_config.demo_mode {
        memset(
            user as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<user_t>(),
        );
        safe_strcpy(
            &raw mut (*user).username as *mut ::core::ffi::c_char,
            b"demo\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*user).authentication_method as *mut ::core::ffi::c_char,
            b"demo\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
            0 as size_t,
        );
        (*user).role = user_role_t::USER_ROLE_VIEWER;
        (*user).is_active = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Demo mode: granting viewer access to unauthenticated user\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_check_viewer_access(
    mut req: *const http_request_t,
    mut user: *mut user_t,
) -> ::core::ffi::c_int {
    return check_viewer_access(req, user, r#false != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_check_action_access(
    mut req: *const http_request_t,
    mut user: *mut user_t,
) -> ::core::ffi::c_int {
    return check_viewer_access(req, user, r#true != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_authorize_action(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut action: authorization_action_t,
    mut camera: *const fleet_camera_t,
    mut user: *mut user_t,
    mut evaluation: *mut authorization_evaluation_t,
) -> ::core::ffi::c_int {
    if req.is_null() || res.is_null() || user.is_null() || evaluation.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    memset(
        user as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    memset(
        evaluation as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<authorization_evaluation_t>(),
    );
    if httpd_check_action_access(req, user) == 0 {
        audit_log_authorization(
            req,
            ::core::ptr::null::<user_t>(),
            action,
            camera,
            ::core::ptr::null::<authorization_evaluation_t>(),
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if authorization_evaluate(user, action, camera, evaluation) != 0 as ::core::ffi::c_int {
        audit_log_authorization(
            req,
            user,
            action,
            camera,
            ::core::ptr::null::<authorization_evaluation_t>(),
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Authorization evaluation failed for user '%s' and action %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*user).username as *mut ::core::ffi::c_char,
            action.0,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if (*evaluation).decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0 {
        audit_log_authorization(
            req,
            user,
            action,
            camera,
            evaluation,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Access denied: User '%s' action %d: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*user).username as *mut ::core::ffi::c_char,
            action.0,
            &raw mut (*evaluation).explanation as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    audit_log_authorization(
        req,
        user,
        action,
        camera,
        evaluation,
        b"allowed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_authorize_global_action(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut action: authorization_action_t,
) -> ::core::ffi::c_int {
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
    return httpd_authorize_action(
        req,
        res,
        action,
        ::core::ptr::null::<fleet_camera_t>(),
        &raw mut user,
        &raw mut evaluation,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_evaluate_stream_action(
    mut user: *const user_t,
    mut action: authorization_action_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut evaluation: *mut authorization_evaluation_t,
) -> ::core::ffi::c_int {
    if user.is_null() || stream_name.is_null() || evaluation.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut camera: fleet_camera_t = fleet_camera_t {
        camera_uuid: [0; 37],
        name: [0; 256],
        address: [0; 512],
        legacy_tags: [0; 256],
        location_uuid: [0; 37],
        location_name: [0; 128],
        location_path: [0; 1024],
        location_ancestor_uuids: [[0; 37]; 32],
        location_depth: 0,
        tags: [fleet_camera_tag_t {
            uuid: [0; 37],
            label: [0; 256],
        }; 64],
        tag_count: 0,
        manufacturer: [0; 128],
        model: [0; 128],
        enabled: false,
        streaming_enabled: false,
        record: false,
        detection_based_recording: false,
        is_onvif: false,
        ptz_enabled: false,
        backchannel_enabled: false,
        health: fleet_health_state_t::FLEET_HEALTH_UNKNOWN,
        availability: fleet_availability_state_t::FLEET_AVAILABILITY_LIVE,
        health_changed_at: 0,
        first_video_at: 0,
        last_frame_ts: 0,
        last_recording_at: 0,
        current_fps: 0.,
        recording_active: false,
    };
    memset(
        &raw mut camera as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<fleet_camera_t>(),
    );
    let mut result: ::core::ffi::c_int =
        db_fleet_camera_find_by_name_readonly(stream_name, &raw mut camera);
    if result != 0 as ::core::ffi::c_int {
        return result;
    }
    return authorization_evaluate(user, action, &raw mut camera, evaluation);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_authorize_stream_action(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut action: authorization_action_t,
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
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
    let mut camera: fleet_camera_t = fleet_camera_t {
        camera_uuid: [0; 37],
        name: [0; 256],
        address: [0; 512],
        legacy_tags: [0; 256],
        location_uuid: [0; 37],
        location_name: [0; 128],
        location_path: [0; 1024],
        location_ancestor_uuids: [[0; 37]; 32],
        location_depth: 0,
        tags: [fleet_camera_tag_t {
            uuid: [0; 37],
            label: [0; 256],
        }; 64],
        tag_count: 0,
        manufacturer: [0; 128],
        model: [0; 128],
        enabled: false,
        streaming_enabled: false,
        record: false,
        detection_based_recording: false,
        is_onvif: false,
        ptz_enabled: false,
        backchannel_enabled: false,
        health: fleet_health_state_t::FLEET_HEALTH_UNKNOWN,
        availability: fleet_availability_state_t::FLEET_AVAILABILITY_LIVE,
        health_changed_at: 0,
        first_video_at: 0,
        last_frame_ts: 0,
        last_recording_at: 0,
        current_fps: 0.,
        recording_active: false,
    };
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
    return httpd_authorize_stream_action_with_context(
        req,
        res,
        action,
        stream_name,
        &raw mut user,
        &raw mut camera,
        &raw mut evaluation,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_request_auth_fingerprint(
    mut req: *const http_request_t,
    mut fingerprint: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if req.is_null() || fingerprint.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut material: [::core::ffi::c_char; 1152] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut session_token: [::core::ffi::c_char; 64] = [
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
    ];
    let mut authorization: *const ::core::ffi::c_char = http_request_get_header(
        req,
        b"Authorization\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut api_key: *const ::core::ffi::c_char =
        http_request_get_header(req, b"X-API-Key\0".as_ptr() as *const ::core::ffi::c_char);
    if httpd_get_session_token(
        req,
        &raw mut session_token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) == 0 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut material as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1152]>(),
            b"session:%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut session_token as *mut ::core::ffi::c_char,
        );
    } else if !authorization.is_null()
        && strncasecmp(
            authorization,
            b"Basic \0".as_ptr() as *const ::core::ffi::c_char,
            6 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut material as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1152]>(),
            b"basic:%s\0".as_ptr() as *const ::core::ffi::c_char,
            authorization.offset(6 as ::core::ffi::c_int as isize),
        );
    } else if !api_key.is_null()
        && *api_key.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        snprintf(
            &raw mut material as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1152]>(),
            b"api:%s\0".as_ptr() as *const ::core::ffi::c_char,
            api_key,
        );
    } else if !authorization.is_null()
        && strncasecmp(
            authorization,
            b"Bearer \0".as_ptr() as *const ::core::ffi::c_char,
            7 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut material as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1152]>(),
            b"bearer:%s\0".as_ptr() as *const ::core::ffi::c_char,
            authorization.offset(7 as ::core::ffi::c_int as isize),
        );
    } else if !g_config.web_auth_enabled {
        snprintf(
            &raw mut material as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1152]>(),
            b"auth-disabled:%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*req).client_ip as *const ::core::ffi::c_char,
        );
    } else if g_config.demo_mode {
        snprintf(
            &raw mut material as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1152]>(),
            b"demo:%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*req).client_ip as *const ::core::ffi::c_char,
        );
    } else {
        return -1 as ::core::ffi::c_int;
    }
    let mut digest: [::core::ffi::c_uchar; 32] = [0; 32];
    if mbedtls_sha256(
        &raw mut material as *mut ::core::ffi::c_char as *const ::core::ffi::c_uchar,
        strlen(&raw mut material as *mut ::core::ffi::c_char),
        &raw mut digest as *mut ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int,
    ) != 0 as ::core::ffi::c_int
    {
        memset(
            &raw mut material as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_char; 1152]>(),
        );
        return -1 as ::core::ffi::c_int;
    }
    static mut hex: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"0123456789abcdef\0")
    };
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[::core::ffi::c_uchar; 32]>() {
        *fingerprint.offset(i.wrapping_mul(2 as size_t) as isize) =
            hex[(digest[i] as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as usize];
        *fingerprint.offset(i.wrapping_mul(2 as size_t).wrapping_add(1 as size_t) as isize) =
            hex[(digest[i] as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize];
        i = i.wrapping_add(1);
    }
    *fingerprint.offset(64isize) = '\0' as ::core::ffi::c_char;
    memset(
        &raw mut digest as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 32]>(),
    );
    memset(
        &raw mut material as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 1152]>(),
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_authorize_media_stream_action(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut action: authorization_action_t,
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if req.is_null()
        || res.is_null()
        || stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut fingerprint: [::core::ffi::c_char; 65] = [0; 65];
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if httpd_request_auth_fingerprint(req, &raw mut fingerprint as *mut ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        pthread_mutex_lock(&raw mut g_media_auth_cache_mutex);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < MEDIA_AUTH_CACHE_CAPACITY {
            let mut entry: *mut media_auth_cache_entry_t =
                (&raw mut g_media_auth_cache as *mut media_auth_cache_entry_t).offset(i as isize);
            if (*entry).occupied as ::core::ffi::c_int != 0
                && (*entry).authorized_until >= now
                && (*entry).action.0 == action.0
                && strcmp(
                    &raw mut (*entry).fingerprint as *mut ::core::ffi::c_char,
                    &raw mut fingerprint as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                && strcmp(
                    &raw mut (*entry).stream_name as *mut ::core::ffi::c_char,
                    stream_name,
                ) == 0 as ::core::ffi::c_int
            {
                (*entry).last_used = now;
                pthread_mutex_unlock(&raw mut g_media_auth_cache_mutex);
                return 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        pthread_mutex_unlock(&raw mut g_media_auth_cache_mutex);
    }
    if httpd_authorize_stream_action(req, res, action, stream_name) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if httpd_request_auth_fingerprint(req, &raw mut fingerprint as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut g_media_auth_cache_mutex);
    let mut selected: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut oldest: time_t = g_media_auth_cache[0usize].last_used;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < MEDIA_AUTH_CACHE_CAPACITY {
        let mut entry_0: *mut media_auth_cache_entry_t =
            (&raw mut g_media_auth_cache as *mut media_auth_cache_entry_t).offset(i_0 as isize);
        if !(*entry_0).occupied || (*entry_0).authorized_until < now {
            selected = i_0;
            break;
        } else {
            if i_0 == 0 as ::core::ffi::c_int || (*entry_0).last_used < oldest {
                selected = i_0;
                oldest = (*entry_0).last_used;
            }
            i_0 += 1;
        }
    }
    let mut entry_1: *mut media_auth_cache_entry_t =
        (&raw mut g_media_auth_cache as *mut media_auth_cache_entry_t).offset(selected as isize);
    memset(
        entry_1 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<media_auth_cache_entry_t>(),
    );
    (*entry_1).occupied = r#true != 0;
    safe_strcpy(
        &raw mut (*entry_1).fingerprint as *mut ::core::ffi::c_char,
        &raw mut fingerprint as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*entry_1).stream_name as *mut ::core::ffi::c_char,
        stream_name,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    (*entry_1).action = action;
    (*entry_1).authorized_until = now + MEDIA_AUTH_CACHE_TTL_SECONDS as time_t;
    (*entry_1).last_used = now;
    pthread_mutex_unlock(&raw mut g_media_auth_cache_mutex);
    return 1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_authorize_stream_action_with_context(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut action: authorization_action_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut user: *mut user_t,
    mut camera: *mut fleet_camera_t,
    mut evaluation: *mut authorization_evaluation_t,
) -> ::core::ffi::c_int {
    if req.is_null()
        || res.is_null()
        || stream_name.is_null()
        || user.is_null()
        || camera.is_null()
        || evaluation.is_null()
    {
        return 0 as ::core::ffi::c_int;
    }
    memset(
        user as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    memset(
        camera as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<fleet_camera_t>(),
    );
    memset(
        evaluation as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<authorization_evaluation_t>(),
    );
    if httpd_check_action_access(req, user) == 0 {
        audit_log_authorization(
            req,
            ::core::ptr::null::<user_t>(),
            action,
            ::core::ptr::null::<fleet_camera_t>(),
            ::core::ptr::null::<authorization_evaluation_t>(),
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = db_fleet_camera_find_by_name_readonly(stream_name, camera);
    if result > 0 as ::core::ffi::c_int {
        let mut metadata: *const authorization_action_metadata_t =
            authorization_action_metadata(action);
        let mut details: *mut cJSON = cJSON_CreateObject();
        if !details.is_null() {
            cJSON_AddStringToObject(
                details,
                b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
                b"authorization.resource_not_found\0".as_ptr() as *const ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                details,
                b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
                stream_name,
            );
        }
        audit_log_append(
            req,
            user,
            if !metadata.is_null() {
                (*metadata).key
            } else {
                b"unknown\0".as_ptr() as *const ::core::ffi::c_char
            },
            b"camera\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            details,
        );
        cJSON_Delete(details);
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Camera not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if result < 0 as ::core::ffi::c_int
        || authorization_evaluate(user, action, camera, evaluation) != 0 as ::core::ffi::c_int
    {
        audit_log_authorization(
            req,
            user,
            action,
            camera,
            if result < 0 as ::core::ffi::c_int {
                ::core::ptr::null_mut::<authorization_evaluation_t>()
            } else {
                evaluation
            },
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to evaluate authorization for stream '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if (*evaluation).decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0 {
        audit_log_authorization(
            req,
            user,
            action,
            camera,
            evaluation,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Access denied: User '%s' action %d on stream '%s': %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*user).username as *mut ::core::ffi::c_char,
            action.0,
            stream_name,
            &raw mut (*evaluation).explanation as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    audit_log_authorization(
        req,
        user,
        action,
        camera,
        evaluation,
        b"allowed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_evaluate_camera_identity_action(
    mut user: *const user_t,
    mut action: authorization_action_t,
    mut camera_uuid: *const ::core::ffi::c_char,
    mut legacy_stream_name: *const ::core::ffi::c_char,
    mut evaluation: *mut authorization_evaluation_t,
) -> ::core::ffi::c_int {
    if user.is_null() || evaluation.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        evaluation as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<authorization_evaluation_t>(),
    );
    let mut camera: fleet_camera_t = fleet_camera_t {
        camera_uuid: [
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
        name: [0; 256],
        address: [0; 512],
        legacy_tags: [0; 256],
        location_uuid: [0; 37],
        location_name: [0; 128],
        location_path: [0; 1024],
        location_ancestor_uuids: [[0; 37]; 32],
        location_depth: 0,
        tags: [fleet_camera_tag_t {
            uuid: [0; 37],
            label: [0; 256],
        }; 64],
        tag_count: 0,
        manufacturer: [0; 128],
        model: [0; 128],
        enabled: false,
        streaming_enabled: false,
        record: false,
        detection_based_recording: false,
        is_onvif: false,
        ptz_enabled: false,
        backchannel_enabled: false,
        health: fleet_health_state_t::FLEET_HEALTH_UNKNOWN,
        availability: fleet_availability_state_t::FLEET_AVAILABILITY_LIVE,
        health_changed_at: 0,
        first_video_at: 0,
        last_frame_ts: 0,
        last_recording_at: 0,
        current_fps: 0.,
        recording_active: false,
    };
    let mut result: ::core::ffi::c_int =
        if !camera_uuid.is_null() && *camera_uuid.offset(0isize) as ::core::ffi::c_int != 0 {
            db_fleet_camera_find_by_uuid_readonly(camera_uuid, &raw mut camera)
        } else {
            db_fleet_camera_find_by_name_readonly(legacy_stream_name, &raw mut camera)
        };
    if result != 0 as ::core::ffi::c_int {
        return result;
    }
    return authorization_evaluate(user, action, &raw mut camera, evaluation);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_authorize_camera_identity_action_with_context(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut action: authorization_action_t,
    mut camera_uuid: *const ::core::ffi::c_char,
    mut legacy_stream_name: *const ::core::ffi::c_char,
    mut user: *mut user_t,
    mut camera: *mut fleet_camera_t,
    mut evaluation: *mut authorization_evaluation_t,
) -> ::core::ffi::c_int {
    if camera_uuid.is_null() || *camera_uuid.offset(0isize) == 0 {
        return httpd_authorize_stream_action_with_context(
            req,
            res,
            action,
            legacy_stream_name,
            user,
            camera,
            evaluation,
        );
    }
    if req.is_null() || res.is_null() || user.is_null() || camera.is_null() || evaluation.is_null()
    {
        return 0 as ::core::ffi::c_int;
    }
    memset(
        user as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    memset(
        camera as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<fleet_camera_t>(),
    );
    memset(
        evaluation as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<authorization_evaluation_t>(),
    );
    let mut result: ::core::ffi::c_int = db_fleet_camera_find_by_uuid_readonly(camera_uuid, camera);
    if result > 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Camera not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if httpd_check_action_access(req, user) == 0 {
        audit_log_authorization(
            req,
            ::core::ptr::null::<user_t>(),
            action,
            ::core::ptr::null::<fleet_camera_t>(),
            ::core::ptr::null::<authorization_evaluation_t>(),
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if result < 0 as ::core::ffi::c_int
        || authorization_evaluate(user, action, camera, evaluation) != 0 as ::core::ffi::c_int
    {
        audit_log_authorization(
            req,
            user,
            action,
            camera,
            if result < 0 as ::core::ffi::c_int {
                ::core::ptr::null_mut::<authorization_evaluation_t>()
            } else {
                evaluation
            },
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if (*evaluation).decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0 {
        audit_log_authorization(
            req,
            user,
            action,
            camera,
            evaluation,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    audit_log_authorization(
        req,
        user,
        action,
        camera,
        evaluation,
        b"allowed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn httpd_sanitize_attachment_filename(
    mut input: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) {
    if output.is_null() || output_size == 0 as size_t {
        return;
    }
    *output.offset(0isize) = '\0' as ::core::ffi::c_char;
    if input.is_null() {
        return;
    }
    let mut basename: *const ::core::ffi::c_char = input;
    let mut cursor: *const ::core::ffi::c_char = input;
    while *cursor != 0 {
        if *cursor as ::core::ffi::c_int == '/' as ::core::ffi::c_int
            || *cursor as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
        {
            basename = cursor.offset(1 as ::core::ffi::c_int as isize);
        }
        cursor = cursor.offset(1);
    }
    let mut written: size_t = 0 as size_t;
    let mut cursor_0: *const ::core::ffi::c_uchar = basename as *const ::core::ffi::c_uchar;
    while *cursor_0 as ::core::ffi::c_int != 0 && written.wrapping_add(1 as size_t) < output_size {
        let mut character: ::core::ffi::c_uchar = *cursor_0;
        let c2rust_fresh1 = written;
        written = written.wrapping_add(1);
        *output.offset(c2rust_fresh1 as isize) = (if *(*__ctype_b_loc())
            .offset(character as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
            || character as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            || character as ::core::ffi::c_int == '-' as ::core::ffi::c_int
            || character as ::core::ffi::c_int == '_' as ::core::ffi::c_int
        {
            character as ::core::ffi::c_char as ::core::ffi::c_int
        } else {
            '_' as ::core::ffi::c_int
        }) as ::core::ffi::c_char;
        cursor_0 = cursor_0.offset(1);
    }
    *output.offset(written as isize) = '\0' as ::core::ffi::c_char;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
