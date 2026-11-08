unsafe extern "C" {
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
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
    unsafe fn url_decode(
        src: *const ::core::ffi::c_char,
        dst: *mut ::core::ffi::c_char,
        dst_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_init() -> ::core::ffi::c_int;
    unsafe fn db_auth_get_user_by_id(user_id: int64_t, user: *mut user_t) -> ::core::ffi::c_int;
    unsafe fn db_auth_authenticate(
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        user_id: *mut int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_complete_totp_login(
        user_id: int64_t,
        code: *const ::core::ffi::c_char,
        challenge: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        ip: *const ::core::ffi::c_char,
        agent: *const ::core::ffi::c_char,
        trusted_seconds: ::core::ffi::c_int,
        session: *mut ::core::ffi::c_char,
        trusted: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_create_password_session(
        user_id: int64_t,
        password: *const ::core::ffi::c_char,
        trusted_token: *const ::core::ffi::c_char,
        ip: *const ::core::ffi::c_char,
        agent: *const ::core::ffi::c_char,
        session: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_create_mfa_challenge_verified(
        user_id: int64_t,
        password: *const ::core::ffi::c_char,
        ip_address: *const ::core::ffi::c_char,
        user_agent: *const ::core::ffi::c_char,
        token: *mut ::core::ffi::c_char,
        token_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_delete_session(token: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn db_auth_list_user_sessions(
        user_id: int64_t,
        sessions: *mut session_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_revoke_session(
        user_id: int64_t,
        session_id: int64_t,
        current_token: *const ::core::ffi::c_char,
        deleted_current: *mut bool,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_revoke_trusted_device(
        user_id: int64_t,
        device_id: int64_t,
        current_token: *const ::core::ffi::c_char,
        deleted_current: *mut bool,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_validate_trusted_device(
        user_id: int64_t,
        token: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_get_trusted_device_id(
        user_id: int64_t,
        token: *const ::core::ffi::c_char,
        trusted_device_id: *mut int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_list_trusted_devices(
        user_id: int64_t,
        devices: *mut trusted_device_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_get_role_name(role: user_role_t) -> *const ::core::ffi::c_char;
    unsafe fn db_auth_get_totp_info(
        user_id: int64_t,
        secret: *mut ::core::ffi::c_char,
        secret_size: size_t,
        enabled: *mut bool,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_ip_allowed_for_user(
        user: *const user_t,
        client_ip: *const ::core::ffi::c_char,
    ) -> bool;
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_get_effective_client_ip(
        req: *const http_request_t,
        client_ip: *mut ::core::ffi::c_char,
        client_ip_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_get_authenticated_user(
        req: *const http_request_t,
        user: *mut user_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_get_session_token(
        req: *const http_request_t,
        token: *mut ::core::ffi::c_char,
        token_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_get_cookie_value(
        req: *const http_request_t,
        cookie_name: *const ::core::ffi::c_char,
        value: *mut ::core::ffi::c_char,
        value_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_trusted_device_lifetime_seconds() -> ::core::ffi::c_int;
    unsafe fn httpd_add_session_cookie(
        res: *mut http_response_t,
        token: *const ::core::ffi::c_char,
    );
    unsafe fn httpd_clear_session_cookie(res: *mut http_response_t);
    unsafe fn httpd_add_trusted_device_cookie(
        res: *mut http_response_t,
        token: *const ::core::ffi::c_char,
    );
    unsafe fn httpd_clear_trusted_device_cookie(res: *mut http_response_t);
    unsafe fn audit_log_login(
        req: *const http_request_t,
        user: *const user_t,
        attempted_username: *const ::core::ffi::c_char,
        authentication_method: *const ::core::ffi::c_char,
        outcome: *const ::core::ffi::c_char,
        reason: *const ::core::ffi::c_char,
    );
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strndup(
        __string: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int64_t = i64;
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
pub struct rate_limit_entry_t {
    pub username: [::core::ffi::c_char; 64],
    pub attempt_count: ::core::ffi::c_int,
    pub window_start: time_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct session_t {
    pub id: int64_t,
    pub user_id: int64_t,
    pub token: [::core::ffi::c_char; 128],
    pub created_at: int64_t,
    pub last_activity_at: int64_t,
    pub idle_expires_at: int64_t,
    pub expires_at: int64_t,
    pub ip_address: [::core::ffi::c_char; 46],
    pub user_agent: [::core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trusted_device_t {
    pub id: int64_t,
    pub user_id: int64_t,
    pub token: [::core::ffi::c_char; 128],
    pub created_at: int64_t,
    pub last_used_at: int64_t,
    pub expires_at: int64_t,
    pub ip_address: [::core::ffi::c_char; 46],
    pub user_agent: [::core::ffi::c_char; 256],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_int);
impl C2Rust_Unnamed {
    pub const DB_AUTH_IDENTITY_ERROR: Self = Self(-1);
    pub const DB_AUTH_IDENTITY_INVALID: Self = Self(-2);
    pub const DB_AUTH_IDENTITY_LIMITED: Self = Self(-3);
    pub const DB_AUTH_IDENTITY_CONFLICT: Self = Self(-4);
}
pub const LOG_COMPONENT: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"AuthAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_RATE_LIMIT_ENTRIES: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
static mut rate_limit_table: [rate_limit_entry_t; 256] = [rate_limit_entry_t {
    username: [0; 64],
    attempt_count: 0,
    window_start: 0,
}; 256];
static mut rate_limit_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn check_rate_limit(mut username: *const ::core::ffi::c_char) -> bool {
    if !g_config.login_rate_limit_enabled || username.is_null() {
        return r#false != 0;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut max_attempts: ::core::ffi::c_int = g_config.login_rate_limit_max_attempts;
    let mut window: ::core::ffi::c_int = g_config.login_rate_limit_window_seconds;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < rate_limit_count {
        if strcmp(
            &raw mut (*(&raw mut rate_limit_table as *mut rate_limit_entry_t).offset(i as isize))
                .username as *mut ::core::ffi::c_char,
            username,
        ) == 0 as ::core::ffi::c_int
        {
            if now - rate_limit_table[i as usize].window_start > window as time_t {
                rate_limit_table[i as usize].attempt_count = 0 as ::core::ffi::c_int;
                rate_limit_table[i as usize].window_start = now;
                return r#false != 0;
            }
            return rate_limit_table[i as usize].attempt_count >= max_attempts;
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn record_failed_attempt(mut username: *const ::core::ffi::c_char) {
    if !g_config.login_rate_limit_enabled || username.is_null() {
        return;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut window: ::core::ffi::c_int = g_config.login_rate_limit_window_seconds;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < rate_limit_count {
        if strcmp(
            &raw mut (*(&raw mut rate_limit_table as *mut rate_limit_entry_t).offset(i as isize))
                .username as *mut ::core::ffi::c_char,
            username,
        ) == 0 as ::core::ffi::c_int
        {
            if now - rate_limit_table[i as usize].window_start > window as time_t {
                rate_limit_table[i as usize].attempt_count = 1 as ::core::ffi::c_int;
                rate_limit_table[i as usize].window_start = now;
            } else {
                rate_limit_table[i as usize].attempt_count += 1;
            }
            return;
        }
        i += 1;
    }
    if rate_limit_count < MAX_RATE_LIMIT_ENTRIES {
        safe_strcpy(
            &raw mut (*(&raw mut rate_limit_table as *mut rate_limit_entry_t)
                .offset(rate_limit_count as isize))
            .username as *mut ::core::ffi::c_char,
            username,
            64 as size_t,
            0 as size_t,
        );
        rate_limit_table[rate_limit_count as usize].attempt_count = 1 as ::core::ffi::c_int;
        rate_limit_table[rate_limit_count as usize].window_start = now;
        rate_limit_count += 1;
    } else {
        let mut oldest_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut oldest_time: time_t = rate_limit_table[0usize].window_start;
        let mut i_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i_0 < MAX_RATE_LIMIT_ENTRIES {
            if rate_limit_table[i_0 as usize].window_start < oldest_time {
                oldest_time = rate_limit_table[i_0 as usize].window_start;
                oldest_idx = i_0;
            }
            i_0 += 1;
        }
        safe_strcpy(
            &raw mut (*(&raw mut rate_limit_table as *mut rate_limit_entry_t)
                .offset(oldest_idx as isize))
            .username as *mut ::core::ffi::c_char,
            username,
            64 as size_t,
            0 as size_t,
        );
        rate_limit_table[oldest_idx as usize].attempt_count = 1 as ::core::ffi::c_int;
        rate_limit_table[oldest_idx as usize].window_start = now;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn auth_clear_login_rate_limit_for_username(
    mut username: *const ::core::ffi::c_char,
) -> bool {
    if username.is_null()
        || *username.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < rate_limit_count {
        if strcmp(
            &raw mut (*(&raw mut rate_limit_table as *mut rate_limit_entry_t).offset(i as isize))
                .username as *mut ::core::ffi::c_char,
            username,
        ) == 0 as ::core::ffi::c_int
        {
            rate_limit_table[i as usize].attempt_count = 0 as ::core::ffi::c_int;
            rate_limit_table[i as usize].window_start = 0 as time_t;
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn request_has_valid_trusted_device(
    mut req: *const http_request_t,
    mut user_id: int64_t,
) -> bool {
    let mut trusted_token: [::core::ffi::c_char; 128] = [
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
    if httpd_trusted_device_lifetime_seconds() <= 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    if httpd_get_cookie_value(
        req,
        b"trusted_device\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut trusted_token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    return db_auth_validate_trusted_device(
        user_id,
        &raw mut trusted_token as *mut ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_auth_system() -> ::core::ffi::c_int {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Initializing authentication system\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut rc: ::core::ffi::c_int = db_auth_init();
    if rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to initialize database authentication system\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Authentication system initialized successfully\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_form_credentials(
    mut body: *const ::core::ffi::c_char,
    mut body_len: size_t,
    mut username: *mut ::core::ffi::c_char,
    mut username_size: size_t,
    mut password: *mut ::core::ffi::c_char,
    mut password_size: size_t,
) -> ::core::ffi::c_int {
    if body.is_null() || body_len == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut body_copy: *mut ::core::ffi::c_char = strndup(body, body_len);
    if body_copy.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut username_start: *mut ::core::ffi::c_char = strstr(
        body_copy,
        b"username=\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !username_start.is_null() {
        username_start = username_start.offset(9 as ::core::ffi::c_int as isize);
        let mut username_end: *mut ::core::ffi::c_char =
            strchr(username_start, '&' as ::core::ffi::c_int);
        if !username_end.is_null() {
            *username_end = '\0' as ::core::ffi::c_char;
        }
        url_decode(username_start, username, username_size);
        let mut password_start: *mut ::core::ffi::c_char = strstr(
            if !username_end.is_null() {
                username_end.offset(1 as ::core::ffi::c_int as isize)
            } else {
                body_copy
            },
            b"password=\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !password_start.is_null() {
            password_start = password_start.offset(9 as ::core::ffi::c_int as isize);
            let mut password_end: *mut ::core::ffi::c_char =
                strchr(password_start, '&' as ::core::ffi::c_int);
            if !password_end.is_null() {
                *password_end = '\0' as ::core::ffi::c_char;
            }
            url_decode(password_start, password, password_size);
            result = 0 as ::core::ffi::c_int;
        }
    }
    free(body_copy as *mut ::core::ffi::c_void);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_auth_login(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/auth/login request\0".as_ptr() as *const ::core::ffi::c_char,
    );
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
    let mut totp_code: [::core::ffi::c_char; 8] = [0 as ::core::ffi::c_char, 0, 0, 0, 0, 0, 0, 0];
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
    let mut is_form: bool = r#false != 0;
    let mut remember_device: bool = r#false != 0;
    let mut trusted_device_used: bool = r#false != 0;
    let mut totp_verified: bool = r#false != 0;
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
    let mut content_type: *const ::core::ffi::c_char = http_request_get_header(
        req,
        b"Content-Type\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !content_type.is_null()
        && !strstr(
            content_type,
            b"application/x-www-form-urlencoded\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        if parse_form_credentials(
            (*req).body as *const ::core::ffi::c_char,
            (*req).body_len,
            &raw mut username as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            &raw mut password as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        ) == 0 as ::core::ffi::c_int
        {
            is_form = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Extracted form data: username=%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut username as *mut ::core::ffi::c_char,
            );
        }
    }
    if !is_form {
        let mut login: *mut cJSON = httpd_parse_json_body(req);
        if login.is_null() {
            if !(*req).body.is_null() && (*req).body_len > 0 as size_t {
                if parse_form_credentials(
                    (*req).body as *const ::core::ffi::c_char,
                    (*req).body_len,
                    &raw mut username as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                    &raw mut password as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                ) == 0 as ::core::ffi::c_int
                {
                    is_form = r#true != 0;
                }
            }
            if !is_form {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to parse login data from request body\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                audit_log_login(
                    req,
                    ::core::ptr::null::<user_t>(),
                    b"\0".as_ptr() as *const ::core::ffi::c_char,
                    b"password\0".as_ptr() as *const ::core::ffi::c_char,
                    b"failure\0".as_ptr() as *const ::core::ffi::c_char,
                    b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
                );
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"Invalid login data\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
        } else {
            let mut username_json: *mut cJSON =
                cJSON_GetObjectItem(login, b"username\0".as_ptr() as *const ::core::ffi::c_char);
            let mut password_json: *mut cJSON =
                cJSON_GetObjectItem(login, b"password\0".as_ptr() as *const ::core::ffi::c_char);
            if username_json.is_null()
                || cJSON_IsString(username_json) == 0
                || password_json.is_null()
                || cJSON_IsString(password_json) == 0
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Missing or invalid username/password in login request\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                cJSON_Delete(login);
                audit_log_login(
                    req,
                    ::core::ptr::null::<user_t>(),
                    b"\0".as_ptr() as *const ::core::ffi::c_char,
                    b"password\0".as_ptr() as *const ::core::ffi::c_char,
                    b"failure\0".as_ptr() as *const ::core::ffi::c_char,
                    b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
                );
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"Missing or invalid username/password\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return;
            }
            safe_strcpy(
                &raw mut username as *mut ::core::ffi::c_char,
                (*username_json).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut password as *mut ::core::ffi::c_char,
                (*password_json).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
            let mut totp_code_json: *mut cJSON =
                cJSON_GetObjectItem(login, b"totp_code\0".as_ptr() as *const ::core::ffi::c_char);
            if !totp_code_json.is_null() && cJSON_IsString(totp_code_json) != 0 {
                safe_strcpy(
                    &raw mut totp_code as *mut ::core::ffi::c_char,
                    (*totp_code_json).valuestring,
                    ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
                    0 as size_t,
                );
            }
            let mut remember_device_json: *mut cJSON = cJSON_GetObjectItem(
                login,
                b"remember_device\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !remember_device_json.is_null() && cJSON_IsBool(remember_device_json) != 0 {
                remember_device = cJSON_IsTrue(remember_device_json) != 0;
            }
            cJSON_Delete(login);
        }
    }
    if check_rate_limit(&raw mut username as *mut ::core::ffi::c_char) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Login rate-limited for user: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut username as *mut ::core::ffi::c_char,
        );
        audit_log_login(
            req,
            ::core::ptr::null::<user_t>(),
            &raw mut username as *mut ::core::ffi::c_char,
            b"password\0".as_ptr() as *const ::core::ffi::c_char,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
            b"rate_limited\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if is_form {
            http_response_add_header(
                res,
                b"Location\0".as_ptr() as *const ::core::ffi::c_char,
                b"/login.html?error=rate_limited\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*res).status_code = 302 as ::core::ffi::c_int;
            (*res).body = NULL;
            (*res).body_length = 0 as size_t;
        } else {
            http_response_set_json_error(
                res,
                429 as ::core::ffi::c_int,
                b"Too many login attempts. Please try again later.\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    let mut user_id: int64_t = 0;
    let mut rc: ::core::ffi::c_int = db_auth_authenticate(
        &raw mut username as *mut ::core::ffi::c_char,
        &raw mut password as *mut ::core::ffi::c_char,
        &raw mut user_id,
    );
    if rc != 0 as ::core::ffi::c_int {
        record_failed_attempt(&raw mut username as *mut ::core::ffi::c_char);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Login failed for user: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut username as *mut ::core::ffi::c_char,
        );
        audit_log_login(
            req,
            ::core::ptr::null::<user_t>(),
            &raw mut username as *mut ::core::ffi::c_char,
            b"password\0".as_ptr() as *const ::core::ffi::c_char,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_credentials\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if is_form {
            http_response_add_header(
                res,
                b"Location\0".as_ptr() as *const ::core::ffi::c_char,
                b"/login.html?error=1\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*res).status_code = 302 as ::core::ffi::c_int;
            (*res).body = NULL;
            (*res).body_length = 0 as size_t;
        } else {
            http_response_set_json_error(
                res,
                401 as ::core::ffi::c_int,
                b"Invalid credentials\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Password verified for user: %s (ID: %lld)\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut username as *mut ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
    );
    let mut authenticated_user: user_t = user_t {
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
    if db_auth_get_user_by_id(user_id, &raw mut authenticated_user) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to load authenticated user record for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut username as *mut ::core::ffi::c_char,
        );
        let mut unavailable_user: user_t = user_t {
            id: user_id,
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
        audit_log_login(
            req,
            &raw mut unavailable_user,
            &raw mut username as *mut ::core::ffi::c_char,
            b"password\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"user_load_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load user\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !db_auth_ip_allowed_for_user(
        &raw mut authenticated_user,
        &raw mut effective_client_ip as *mut ::core::ffi::c_char,
    ) {
        record_failed_attempt(&raw mut username as *mut ::core::ffi::c_char);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Login blocked by allowed_login_cidrs for user '%s' from IP %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut username as *mut ::core::ffi::c_char,
            if effective_client_ip[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                &raw mut effective_client_ip as *mut ::core::ffi::c_char
            } else {
                b"(unknown)\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        audit_log_login(
            req,
            &raw mut authenticated_user,
            &raw mut username as *mut ::core::ffi::c_char,
            b"password\0".as_ptr() as *const ::core::ffi::c_char,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
            b"source_address_not_allowed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if is_form {
            http_response_add_header(
                res,
                b"Location\0".as_ptr() as *const ::core::ffi::c_char,
                b"/login.html?error=1\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*res).status_code = 302 as ::core::ffi::c_int;
            (*res).body = NULL;
            (*res).body_length = 0 as size_t;
        } else {
            http_response_set_json_error(
                res,
                401 as ::core::ffi::c_int,
                b"Invalid credentials\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    let mut must_change_password: bool =
        authenticated_user.must_change_password as ::core::ffi::c_int != 0 && !g_config.demo_mode;
    if is_form as ::core::ffi::c_int != 0
        && authenticated_user.totp_enabled as ::core::ffi::c_int != 0
    {
        audit_log_login(
            req,
            &raw mut authenticated_user,
            &raw mut username as *mut ::core::ffi::c_char,
            b"password\0".as_ptr() as *const ::core::ffi::c_char,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
            b"mfa_required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_add_header(
            res,
            b"Location\0".as_ptr() as *const ::core::ffi::c_char,
            b"/login.html\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*res).status_code = 302 as ::core::ffi::c_int;
        (*res).body = NULL;
        (*res).body_length = 0 as size_t;
        return;
    }
    if !is_form {
        let mut totp_secret: [::core::ffi::c_char; 64] = [
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
        let mut totp_enabled: bool = r#false != 0;
        if db_auth_get_totp_info(
            user_id,
            &raw mut totp_secret as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            &raw mut totp_enabled,
        ) == 0 as ::core::ffi::c_int
            && totp_enabled as ::core::ffi::c_int != 0
        {
            let mut trusted_device_valid: bool = request_has_valid_trusted_device(req, user_id);
            if g_config.force_mfa_on_login {
                if totp_code[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                    if trusted_device_valid {
                        trusted_device_used = r#true != 0;
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Trusted device accepted for user: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut username as *mut ::core::ffi::c_char,
                        );
                    } else {
                        record_failed_attempt(&raw mut username as *mut ::core::ffi::c_char);
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Force MFA: no TOTP code provided for user: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut username as *mut ::core::ffi::c_char,
                        );
                        audit_log_login(
                            req,
                            &raw mut authenticated_user,
                            &raw mut username as *mut ::core::ffi::c_char,
                            b"password_totp\0".as_ptr() as *const ::core::ffi::c_char,
                            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
                            b"mfa_required\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        http_response_set_json_error(
                            res,
                            401 as ::core::ffi::c_int,
                            b"Invalid credentials\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        return;
                    }
                }
                if !trusted_device_used {
                    totp_verified = r#true != 0;
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Force MFA: inline code supplied for user: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut username as *mut ::core::ffi::c_char,
                    );
                }
            } else if totp_code[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                totp_verified = r#true != 0;
            } else if trusted_device_valid {
                trusted_device_used = r#true != 0;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Trusted device accepted for user: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut username as *mut ::core::ffi::c_char,
                );
            } else {
                let mut totp_token: [::core::ffi::c_char; 33] = [0; 33];
                rc = db_auth_create_mfa_challenge_verified(
                    user_id,
                    &raw mut password as *mut ::core::ffi::c_char,
                    &raw mut effective_client_ip as *mut ::core::ffi::c_char,
                    &raw const (*req).user_agent as *const ::core::ffi::c_char,
                    &raw mut totp_token as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 33]>(),
                );
                if rc != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Failed to create pending MFA session for user: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut username as *mut ::core::ffi::c_char,
                    );
                    audit_log_login(
                        req,
                        &raw mut authenticated_user,
                        &raw mut username as *mut ::core::ffi::c_char,
                        b"password\0".as_ptr() as *const ::core::ffi::c_char,
                        b"error\0".as_ptr() as *const ::core::ffi::c_char,
                        b"mfa_challenge_create_failed\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    http_response_set_json_error(
                        res,
                        500 as ::core::ffi::c_int,
                        b"Failed to create MFA session\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return;
                }
                let mut response: *mut cJSON = cJSON_CreateObject();
                cJSON_AddBoolToObject(
                    response,
                    b"totp_required\0".as_ptr() as *const ::core::ffi::c_char,
                    r#true,
                );
                cJSON_AddStringToObject(
                    response,
                    b"totp_token\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut totp_token as *mut ::core::ffi::c_char,
                );
                let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
                http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
                free(json_str as *mut ::core::ffi::c_void);
                cJSON_Delete(response);
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"TOTP verification required for user: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut username as *mut ::core::ffi::c_char,
                );
                audit_log_login(
                    req,
                    &raw mut authenticated_user,
                    &raw mut username as *mut ::core::ffi::c_char,
                    b"password\0".as_ptr() as *const ::core::ffi::c_char,
                    b"allowed\0".as_ptr() as *const ::core::ffi::c_char,
                    b"mfa_challenge_issued\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
        } else if g_config.force_mfa_on_login as ::core::ffi::c_int != 0
            && totp_code[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Force MFA: user %s has no TOTP configured, allowing login\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut username as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut token: [::core::ffi::c_char; 33] = [0; 33];
    let mut trusted_token: [::core::ffi::c_char; 33] = [
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
    ];
    if totp_verified {
        let mut trusted_seconds: ::core::ffi::c_int = if remember_device as ::core::ffi::c_int != 0
        {
            httpd_trusted_device_lifetime_seconds()
        } else {
            0 as ::core::ffi::c_int
        };
        rc = db_auth_complete_totp_login(
            user_id,
            &raw mut totp_code as *mut ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw mut password as *mut ::core::ffi::c_char,
            &raw mut effective_client_ip as *mut ::core::ffi::c_char,
            &raw const (*req).user_agent as *const ::core::ffi::c_char,
            trusted_seconds,
            &raw mut token as *mut ::core::ffi::c_char,
            &raw mut trusted_token as *mut ::core::ffi::c_char,
        );
    } else {
        let mut existing_trusted: [::core::ffi::c_char; 128] = [
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
        if trusted_device_used as ::core::ffi::c_int != 0
            && httpd_trusted_device_lifetime_seconds() > 0 as ::core::ffi::c_int
        {
            httpd_get_cookie_value(
                req,
                b"trusted_device\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut existing_trusted as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            );
        }
        rc = db_auth_create_password_session(
            user_id,
            &raw mut password as *mut ::core::ffi::c_char,
            &raw mut existing_trusted as *mut ::core::ffi::c_char,
            &raw mut effective_client_ip as *mut ::core::ffi::c_char,
            &raw const (*req).user_agent as *const ::core::ffi::c_char,
            &raw mut token as *mut ::core::ffi::c_char,
        );
    }
    if rc != 0 as ::core::ffi::c_int {
        if rc == C2Rust_Unnamed::DB_AUTH_IDENTITY_INVALID.0 {
            record_failed_attempt(&raw mut username as *mut ::core::ffi::c_char);
        }
        audit_log_login(
            req,
            &raw mut authenticated_user,
            &raw mut username as *mut ::core::ffi::c_char,
            b"password_totp\0".as_ptr() as *const ::core::ffi::c_char,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
            if rc == C2Rust_Unnamed::DB_AUTH_IDENTITY_LIMITED.0 {
                b"rate_limited\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"credential_issue_failed\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        http_response_set_json_error(
            res,
            if rc == C2Rust_Unnamed::DB_AUTH_IDENTITY_LIMITED.0 {
                429 as ::core::ffi::c_int
            } else if rc == C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0 {
                500 as ::core::ffi::c_int
            } else {
                401 as ::core::ffi::c_int
            },
            if rc == C2Rust_Unnamed::DB_AUTH_IDENTITY_LIMITED.0 {
                b"Too many TOTP attempts. Please try again later.\0".as_ptr()
                    as *const ::core::ffi::c_char
            } else if rc == C2Rust_Unnamed::DB_AUTH_IDENTITY_ERROR.0 {
                b"Failed to create session\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"Invalid credentials\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    auth_clear_login_rate_limit_for_username(&raw mut username as *mut ::core::ffi::c_char);
    httpd_add_session_cookie(res, &raw mut token as *mut ::core::ffi::c_char);
    if trusted_token[0usize] != 0 {
        httpd_add_trusted_device_cookie(res, &raw mut trusted_token as *mut ::core::ffi::c_char);
    }
    if is_form {
        http_response_add_header(
            res,
            b"Location\0".as_ptr() as *const ::core::ffi::c_char,
            b"/index.html\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*res).status_code = 302 as ::core::ffi::c_int;
        (*res).body = NULL;
        (*res).body_length = 0 as size_t;
    } else {
        let mut response_0: *mut cJSON = cJSON_CreateObject();
        cJSON_AddBoolToObject(
            response_0,
            b"success\0".as_ptr() as *const ::core::ffi::c_char,
            r#true,
        );
        cJSON_AddStringToObject(
            response_0,
            b"redirect\0".as_ptr() as *const ::core::ffi::c_char,
            b"/index.html\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddBoolToObject(
            response_0,
            b"must_change_password\0".as_ptr() as *const ::core::ffi::c_char,
            must_change_password as cJSON_bool,
        );
        let mut json_str_0: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response_0);
        http_response_set_json(res, 200 as ::core::ffi::c_int, json_str_0);
        free(json_str_0 as *mut ::core::ffi::c_void);
        cJSON_Delete(response_0);
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Session created successfully for user: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut username as *mut ::core::ffi::c_char,
    );
    let mut authentication_method: *const ::core::ffi::c_char =
        if trusted_device_used as ::core::ffi::c_int != 0 {
            b"password_trusted_device\0".as_ptr() as *const ::core::ffi::c_char
        } else if totp_verified as ::core::ffi::c_int != 0 {
            b"password_totp\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"password\0".as_ptr() as *const ::core::ffi::c_char
        };
    audit_log_login(
        req,
        &raw mut authenticated_user,
        &raw mut username as *mut ::core::ffi::c_char,
        authentication_method,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"session_created\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_auth_logout(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling logout request\0".as_ptr() as *const ::core::ffi::c_char,
    );
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
        db_auth_delete_session(&raw mut session_token as *mut ::core::ffi::c_char);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Session deleted for logout request\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    httpd_clear_session_cookie(res);
    httpd_clear_trusted_device_cookie(res);
    let mut accept: *const ::core::ffi::c_char =
        http_request_get_header(req, b"Accept\0".as_ptr() as *const ::core::ffi::c_char);
    let mut requested_with: *const ::core::ffi::c_char = http_request_get_header(
        req,
        b"X-Requested-With\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut is_api_request: bool = !accept.is_null()
        && !strstr(
            accept,
            b"application/json\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || !requested_with.is_null();
    if is_api_request {
        let mut response: *mut cJSON = cJSON_CreateObject();
        cJSON_AddBoolToObject(
            response,
            b"success\0".as_ptr() as *const ::core::ffi::c_char,
            r#true,
        );
        cJSON_AddStringToObject(
            response,
            b"redirect\0".as_ptr() as *const ::core::ffi::c_char,
            b"/login.html?logout=true\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
        http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
        free(json_str as *mut ::core::ffi::c_void);
        cJSON_Delete(response);
    } else {
        http_response_add_header(
            res,
            b"Location\0".as_ptr() as *const ::core::ffi::c_char,
            b"/login.html?logout=true\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*res).status_code = 302 as ::core::ffi::c_int;
        (*res).body = NULL;
        (*res).body_length = 0 as size_t;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Logout successful\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_auth_verify(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/auth/verify request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !g_config.web_auth_enabled {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Authentication is disabled, returning success for verify request\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        let mut response: *mut cJSON = cJSON_CreateObject();
        cJSON_AddBoolToObject(
            response,
            b"authenticated\0".as_ptr() as *const ::core::ffi::c_char,
            r#true,
        );
        cJSON_AddStringToObject(
            response,
            b"username\0".as_ptr() as *const ::core::ffi::c_char,
            b"admin\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            response,
            b"role\0".as_ptr() as *const ::core::ffi::c_char,
            b"admin\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddBoolToObject(
            response,
            b"must_change_password\0".as_ptr() as *const ::core::ffi::c_char,
            r#false,
        );
        cJSON_AddBoolToObject(
            response,
            b"auth_enabled\0".as_ptr() as *const ::core::ffi::c_char,
            r#false,
        );
        cJSON_AddNumberToObject(
            response,
            b"auth_timeout_hours\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.auth_timeout_hours as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            response,
            b"auth_absolute_timeout_hours\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.auth_absolute_timeout_hours as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            response,
            b"trusted_device_days\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.trusted_device_days as ::core::ffi::c_double,
        );
        let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
        http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
        free(json_str as *mut ::core::ffi::c_void);
        cJSON_Delete(response);
        return;
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
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Authentication successful for user: %s (role: %s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut user.username as *mut ::core::ffi::c_char,
            db_auth_get_role_name(user.role),
        );
        let mut response_0: *mut cJSON = cJSON_CreateObject();
        cJSON_AddBoolToObject(
            response_0,
            b"authenticated\0".as_ptr() as *const ::core::ffi::c_char,
            r#true,
        );
        cJSON_AddNumberToObject(
            response_0,
            b"id\0".as_ptr() as *const ::core::ffi::c_char,
            user.id as ::core::ffi::c_double,
        );
        cJSON_AddStringToObject(
            response_0,
            b"username\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut user.username as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            response_0,
            b"email\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut user.email as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            response_0,
            b"role\0".as_ptr() as *const ::core::ffi::c_char,
            db_auth_get_role_name(user.role),
        );
        cJSON_AddNumberToObject(
            response_0,
            b"role_id\0".as_ptr() as *const ::core::ffi::c_char,
            user.role.0 as ::core::ffi::c_double,
        );
        cJSON_AddBoolToObject(
            response_0,
            b"is_active\0".as_ptr() as *const ::core::ffi::c_char,
            user.is_active as cJSON_bool,
        );
        cJSON_AddBoolToObject(
            response_0,
            b"password_change_locked\0".as_ptr() as *const ::core::ffi::c_char,
            user.password_change_locked as cJSON_bool,
        );
        cJSON_AddBoolToObject(
            response_0,
            b"must_change_password\0".as_ptr() as *const ::core::ffi::c_char,
            (user.must_change_password as ::core::ffi::c_int != 0 && !g_config.demo_mode)
                as ::core::ffi::c_int,
        );
        cJSON_AddBoolToObject(
            response_0,
            b"auth_enabled\0".as_ptr() as *const ::core::ffi::c_char,
            r#true,
        );
        cJSON_AddNumberToObject(
            response_0,
            b"auth_timeout_hours\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.auth_timeout_hours as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            response_0,
            b"auth_absolute_timeout_hours\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.auth_absolute_timeout_hours as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            response_0,
            b"trusted_device_days\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.trusted_device_days as ::core::ffi::c_double,
        );
        let mut json_str_0: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response_0);
        http_response_set_json(res, 200 as ::core::ffi::c_int, json_str_0);
        free(json_str_0 as *mut ::core::ffi::c_void);
        cJSON_Delete(response_0);
        return;
    }
    if g_config.demo_mode {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Demo mode: returning viewer access for unauthenticated user\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        let mut response_1: *mut cJSON = cJSON_CreateObject();
        cJSON_AddBoolToObject(
            response_1,
            b"authenticated\0".as_ptr() as *const ::core::ffi::c_char,
            r#false,
        );
        cJSON_AddBoolToObject(
            response_1,
            b"demo_mode\0".as_ptr() as *const ::core::ffi::c_char,
            r#true,
        );
        cJSON_AddStringToObject(
            response_1,
            b"username\0".as_ptr() as *const ::core::ffi::c_char,
            b"demo\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            response_1,
            b"role\0".as_ptr() as *const ::core::ffi::c_char,
            b"viewer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddBoolToObject(
            response_1,
            b"must_change_password\0".as_ptr() as *const ::core::ffi::c_char,
            r#false,
        );
        let mut json_str_1: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response_1);
        http_response_set_json(res, 200 as ::core::ffi::c_int, json_str_1);
        free(json_str_1 as *mut ::core::ffi::c_void);
        cJSON_Delete(response_1);
        return;
    }
    httpd_clear_session_cookie(res);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Authentication verification failed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    http_response_set_json_error(
        res,
        401 as ::core::ffi::c_int,
        b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_auth_login_config(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"force_mfa_on_login\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.force_mfa_on_login as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        response,
        b"remember_device_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        (g_config.trusted_device_days > 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    cJSON_AddNumberToObject(
        response,
        b"auth_timeout_hours\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.auth_timeout_hours as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        response,
        b"auth_absolute_timeout_hours\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.auth_absolute_timeout_hours as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        response,
        b"trusted_device_days\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.trusted_device_days as ::core::ffi::c_double,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
}
unsafe extern "C" fn credential_owner(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut user: *mut user_t,
) -> bool {
    http_response_add_header(
        res,
        b"Cache-Control\0".as_ptr() as *const ::core::ffi::c_char,
        b"private, no-store\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if httpd_get_authenticated_user(req, user) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if (*user).id <= 0 as int64_t {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"A personal account is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn credential_id(
    mut req: *const http_request_t,
    mut prefix: *const ::core::ffi::c_char,
    mut id: *mut int64_t,
) -> bool {
    let mut length: size_t = strlen(prefix);
    if strncmp(
        &raw const (*req).path as *const ::core::ffi::c_char,
        prefix,
        length,
    ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut p: *const ::core::ffi::c_char =
        (&raw const (*req).path as *const ::core::ffi::c_char).offset(length as isize);
    if (*p as ::core::ffi::c_int) < '1' as ::core::ffi::c_int
        || *p as ::core::ffi::c_int > '9' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut value: int64_t = 0 as int64_t;
    while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != '?' as ::core::ffi::c_int {
        if (*p as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int > '9' as ::core::ffi::c_int
            || value
                > (9007199254740991 as int64_t
                    - (*p as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as int64_t)
                    / 10 as int64_t
        {
            return r#false != 0;
        }
        value = value * 10 as int64_t
            + (*p as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as int64_t;
        p = p.offset(1);
    }
    *id = value;
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_auth_sessions_list(
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
    if !credential_owner(req, res, &raw mut user) {
        return;
    }
    let mut sessions: [session_t; 32] = [session_t {
        id: 0,
        user_id: 0,
        token: [0; 128],
        created_at: 0,
        last_activity_at: 0,
        idle_expires_at: 0,
        expires_at: 0,
        ip_address: [0; 46],
        user_agent: [0; 256],
    }; 32];
    let mut count: ::core::ffi::c_int = db_auth_list_user_sessions(
        user.id,
        &raw mut sessions as *mut session_t,
        32 as ::core::ffi::c_int,
    );
    if count < 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to list sessions\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut current_token: [::core::ffi::c_char; 128] = [
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
    httpd_get_session_token(
        req,
        &raw mut current_token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_AddArrayToObject(
        response,
        b"sessions\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut item: *mut cJSON = cJSON_CreateObject();
        cJSON_AddNumberToObject(
            item,
            b"id\0".as_ptr() as *const ::core::ffi::c_char,
            sessions[i as usize].id as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
            sessions[i as usize].created_at as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"last_activity_at\0".as_ptr() as *const ::core::ffi::c_char,
            sessions[i as usize].last_activity_at as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"idle_expires_at\0".as_ptr() as *const ::core::ffi::c_char,
            sessions[i as usize].idle_expires_at as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"expires_at\0".as_ptr() as *const ::core::ffi::c_char,
            sessions[i as usize].expires_at as ::core::ffi::c_double,
        );
        cJSON_AddStringToObject(
            item,
            b"ip_address\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut sessions as *mut session_t).offset(i as isize)).ip_address
                as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            item,
            b"user_agent\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut sessions as *mut session_t).offset(i as isize)).user_agent
                as *mut ::core::ffi::c_char,
        );
        cJSON_AddBoolToObject(
            item,
            b"current\0".as_ptr() as *const ::core::ffi::c_char,
            (current_token[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                && strcmp(
                    &raw mut current_token as *mut ::core::ffi::c_char,
                    &raw mut (*(&raw mut sessions as *mut session_t).offset(i as isize)).token
                        as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
        cJSON_AddItemToArray(items, item);
        i += 1;
    }
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_auth_sessions_delete(
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
    if !credential_owner(req, res, &raw mut user) {
        return;
    }
    let mut id: int64_t = 0;
    if !credential_id(
        req,
        b"/api/auth/sessions/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut id,
    ) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid session ID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut current_token: [::core::ffi::c_char; 128] = [
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
    httpd_get_session_token(
        req,
        &raw mut current_token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    );
    let mut deleted_current: bool = r#false != 0;
    let mut result: ::core::ffi::c_int = db_auth_revoke_session(
        user.id,
        id,
        &raw mut current_token as *mut ::core::ffi::c_char,
        &raw mut deleted_current,
    );
    if result != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            if result > 0 as ::core::ffi::c_int {
                404 as ::core::ffi::c_int
            } else {
                500 as ::core::ffi::c_int
            },
            if result > 0 as ::core::ffi::c_int {
                b"Credential not found\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"Failed to revoke credential\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    if deleted_current {
        httpd_clear_session_cookie(res);
    }
    http_response_set_json(
        res,
        200 as ::core::ffi::c_int,
        b"{\"success\":true}\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_auth_trusted_devices_list(
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
    if !credential_owner(req, res, &raw mut user) {
        return;
    }
    let mut devices: [trusted_device_t; 32] = [trusted_device_t {
        id: 0,
        user_id: 0,
        token: [0; 128],
        created_at: 0,
        last_used_at: 0,
        expires_at: 0,
        ip_address: [0; 46],
        user_agent: [0; 256],
    }; 32];
    let mut count: ::core::ffi::c_int = db_auth_list_trusted_devices(
        user.id,
        &raw mut devices as *mut trusted_device_t,
        32 as ::core::ffi::c_int,
    );
    if count < 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to list trusted devices\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut current_token: [::core::ffi::c_char; 128] = [
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
    let mut current_device_id: int64_t = 0 as int64_t;
    if httpd_get_cookie_value(
        req,
        b"trusted_device\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut current_token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    ) == 0 as ::core::ffi::c_int
    {
        db_auth_get_trusted_device_id(
            user.id,
            &raw mut current_token as *mut ::core::ffi::c_char,
            &raw mut current_device_id,
        );
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_AddArrayToObject(
        response,
        b"trusted_devices\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut item: *mut cJSON = cJSON_CreateObject();
        cJSON_AddNumberToObject(
            item,
            b"id\0".as_ptr() as *const ::core::ffi::c_char,
            devices[i as usize].id as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
            devices[i as usize].created_at as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"last_used_at\0".as_ptr() as *const ::core::ffi::c_char,
            devices[i as usize].last_used_at as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            item,
            b"expires_at\0".as_ptr() as *const ::core::ffi::c_char,
            devices[i as usize].expires_at as ::core::ffi::c_double,
        );
        cJSON_AddStringToObject(
            item,
            b"ip_address\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut trusted_device_t).offset(i as isize)).ip_address
                as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            item,
            b"user_agent\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut trusted_device_t).offset(i as isize)).user_agent
                as *mut ::core::ffi::c_char,
        );
        cJSON_AddBoolToObject(
            item,
            b"current\0".as_ptr() as *const ::core::ffi::c_char,
            (current_device_id > 0 as int64_t && current_device_id == devices[i as usize].id)
                as ::core::ffi::c_int,
        );
        cJSON_AddItemToArray(items, item);
        i += 1;
    }
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_auth_trusted_devices_delete(
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
    if !credential_owner(req, res, &raw mut user) {
        return;
    }
    let mut id: int64_t = 0;
    if !credential_id(
        req,
        b"/api/auth/trusted-devices/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut id,
    ) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid trusted device ID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut current_token: [::core::ffi::c_char; 128] = [
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
    httpd_get_cookie_value(
        req,
        b"trusted_device\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut current_token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    );
    let mut deleted_current: bool = r#false != 0;
    let mut result: ::core::ffi::c_int = db_auth_revoke_trusted_device(
        user.id,
        id,
        &raw mut current_token as *mut ::core::ffi::c_char,
        &raw mut deleted_current,
    );
    if result != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            if result > 0 as ::core::ffi::c_int {
                404 as ::core::ffi::c_int
            } else {
                500 as ::core::ffi::c_int
            },
            if result > 0 as ::core::ffi::c_int {
                b"Credential not found\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"Failed to revoke credential\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    if deleted_current {
        httpd_clear_trusted_device_cookie(res);
    }
    http_response_set_json(
        res,
        200 as ::core::ffi::c_int,
        b"{\"success\":true}\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
