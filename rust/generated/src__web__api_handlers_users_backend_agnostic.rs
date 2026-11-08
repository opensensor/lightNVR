unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_int64(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int)
    -> sqlite3_int64;
    unsafe fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNull(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
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
    unsafe fn auth_clear_login_rate_limit_for_username(
        username: *const ::core::ffi::c_char,
    ) -> bool;
    unsafe fn db_auth_create_user(
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        email: *const ::core::ffi::c_char,
        role: user_role_t,
        is_active: bool,
        user_id: *mut int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_update_user(
        user_id: int64_t,
        username: *const ::core::ffi::c_char,
        email: *const ::core::ffi::c_char,
        role: ::core::ffi::c_int,
        is_active: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_update_user_account(
        user_id: int64_t,
        username: *const ::core::ffi::c_char,
        email: *const ::core::ffi::c_char,
        role: ::core::ffi::c_int,
        is_active: ::core::ffi::c_int,
        password: *const ::core::ffi::c_char,
        set_login_cidrs: bool,
        allowed_login_cidrs: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_change_password_verified(
        user_id: int64_t,
        new_password: *const ::core::ffi::c_char,
        old_password: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_delete_user(user_id: int64_t) -> ::core::ffi::c_int;
    unsafe fn db_auth_get_user_by_id(user_id: int64_t, user: *mut user_t) -> ::core::ffi::c_int;
    unsafe fn db_auth_generate_api_key(
        user_id: int64_t,
        api_key: *mut ::core::ffi::c_char,
        api_key_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_set_password_lock(user_id: int64_t, locked: bool) -> ::core::ffi::c_int;
    unsafe fn db_auth_get_role_name(role: user_role_t) -> *const ::core::ffi::c_char;
    unsafe fn db_auth_validate_allowed_login_cidrs(
        allowed_login_cidrs: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_set_allowed_login_cidrs(
        user_id: int64_t,
        allowed_login_cidrs: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn authorization_action_bit(action: authorization_action_t) -> uint64_t;
    unsafe fn authorization_evaluate(
        user: *const user_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_clear_session_cookie(res: *mut http_response_t);
    unsafe fn httpd_clear_trusted_device_cookie(res: *mut http_response_t);
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_authorize_global_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
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
    unsafe fn db_authorization_role_get(
        uuid: *const ::core::ffi::c_char,
        role: *mut authorization_role_t,
    ) -> db_authorization_result_t;
    unsafe fn db_authorization_replace_user_policy(
        user_id: int64_t,
        mode: *const ::core::ffi::c_char,
        grants: *const authorization_grant_input_t,
        grant_count: ::core::ffi::c_int,
        expected_version: int64_t,
        new_version: *mut int64_t,
    ) -> db_authorization_result_t;
    unsafe fn db_authorization_get_policy_version(version: *mut int64_t) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn cached_column_exists(
        table_name: *const ::core::ffi::c_char,
        column_name: *const ::core::ffi::c_char,
    ) -> bool;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type int64_t = i64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
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
pub struct authorization_grant_input_t {
    pub role_uuid: [::core::ffi::c_char; 37],
    pub scope_type: [::core::ffi::c_char; 16],
    pub selector_json: [::core::ffi::c_char; 8192],
    pub collection_uuid: [::core::ffi::c_char; 37],
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
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_MISUSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const LOG_COMPONENT: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"UsersAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
unsafe extern "C" fn extract_user_id(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut suffix: *const ::core::ffi::c_char,
    mut user_id: *mut int64_t,
) -> bool {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parsed: ::core::ffi::c_longlong = 0;
    let mut prefix: *const ::core::ffi::c_char =
        b"/api/auth/users/\0".as_ptr() as *const ::core::ffi::c_char;
    let mut id: *const ::core::ffi::c_char = &raw const (*req).path as *const ::core::ffi::c_char;
    if strncmp(id, prefix, strlen(prefix)) == 0 as ::core::ffi::c_int {
        id = id.offset(strlen(prefix) as isize);
        if !((*id as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || *id as ::core::ffi::c_int > '9' as ::core::ffi::c_int)
        {
            *__errno_location() = 0 as ::core::ffi::c_int;
            end = ::core::ptr::null_mut::<::core::ffi::c_char>();
            parsed = strtoll(id, &raw mut end, 10 as ::core::ffi::c_int);
            if !(*__errno_location() == ERANGE
                || parsed <= 0 as ::core::ffi::c_longlong
                || parsed > INT64_MAX as ::core::ffi::c_longlong
                || strncmp(end, suffix, strlen(suffix)) != 0 as ::core::ffi::c_int)
            {
                end = end.offset(strlen(suffix) as isize);
                while *end as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                    end = end.offset(1);
                }
                if !(*end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                    && *end as ::core::ffi::c_int != '?' as ::core::ffi::c_int)
                {
                    *user_id = parsed as int64_t;
                    return r#true != 0;
                }
            }
        }
    }
    http_response_set_json_error(
        res,
        400 as ::core::ffi::c_int,
        b"Invalid user ID\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return r#false != 0;
}
unsafe extern "C" fn user_to_json(mut user: *const user_t) -> *mut cJSON {
    let mut json: *mut cJSON = cJSON_CreateObject();
    cJSON_AddNumberToObject(
        json,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        (*user).id as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        json,
        b"username\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*user).username as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        json,
        b"email\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*user).email as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        json,
        b"role\0".as_ptr() as *const ::core::ffi::c_char,
        (*user).role.0 as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        json,
        b"role_name\0".as_ptr() as *const ::core::ffi::c_char,
        db_auth_get_role_name((*user).role),
    );
    cJSON_AddNumberToObject(
        json,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*user).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        json,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*user).updated_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        json,
        b"last_login\0".as_ptr() as *const ::core::ffi::c_char,
        (*user).last_login as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        json,
        b"is_active\0".as_ptr() as *const ::core::ffi::c_char,
        (*user).is_active as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        json,
        b"password_change_locked\0".as_ptr() as *const ::core::ffi::c_char,
        (*user).password_change_locked as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        json,
        b"must_change_password\0".as_ptr() as *const ::core::ffi::c_char,
        (*user).must_change_password as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        json,
        b"totp_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        (*user).totp_enabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        json,
        b"authorization_mode\0".as_ptr() as *const ::core::ffi::c_char,
        if (*user).authorization_mode[0usize] as ::core::ffi::c_int != 0 {
            &raw const (*user).authorization_mode as *const ::core::ffi::c_char
        } else {
            b"legacy\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if (*user).has_login_cidr_restriction {
        cJSON_AddStringToObject(
            json,
            b"allowed_login_cidrs\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*user).allowed_login_cidrs as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            json,
            b"allowed_login_cidrs\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return json;
}
unsafe extern "C" fn prepare_user_select_stmt(
    mut db: *mut sqlite3,
    mut suffix: *const ::core::ffi::c_char,
    mut stmt: *mut *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    if db.is_null() || suffix.is_null() || stmt.is_null() {
        return SQLITE_MISUSE;
    }
    let mut has_totp: bool = cached_column_exists(
        b"users\0".as_ptr() as *const ::core::ffi::c_char,
        b"totp_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_allowed_login_cidrs: bool = cached_column_exists(
        b"users\0".as_ptr() as *const ::core::ffi::c_char,
        b"allowed_login_cidrs\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_authorization_mode: bool = cached_column_exists(
        b"users\0".as_ptr() as *const ::core::ffi::c_char,
        b"authorization_mode\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_must_change_password: bool = cached_column_exists(
        b"users\0".as_ptr() as *const ::core::ffi::c_char,
        b"must_change_password\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut sql: [::core::ffi::c_char; 768] = [0; 768];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 768]>(),
        b"SELECT id, username, email, role, api_key, created_at, updated_at, last_login, is_active, password_change_locked, %s, %s, %s, %s FROM users %s;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        if has_must_change_password as ::core::ffi::c_int != 0 {
            b"must_change_password\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"0\0".as_ptr() as *const ::core::ffi::c_char
        },
        if has_totp as ::core::ffi::c_int != 0 {
            b"totp_enabled\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"0\0".as_ptr() as *const ::core::ffi::c_char
        },
        if has_allowed_login_cidrs as ::core::ffi::c_int != 0 {
            b"allowed_login_cidrs\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"NULL\0".as_ptr() as *const ::core::ffi::c_char
        },
        if has_authorization_mode as ::core::ffi::c_int != 0 {
            b"authorization_mode\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"'legacy'\0".as_ptr() as *const ::core::ffi::c_char
        },
        suffix,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 768]>()
    {
        return SQLITE_TOOBIG;
    }
    return sqlite3_prepare_v2(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn populate_user_from_stmt(mut stmt: *mut sqlite3_stmt, mut user: *mut user_t) {
    memset(
        user as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    (*user).id = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
    safe_strcpy(
        &raw mut (*user).username as *mut ::core::ffi::c_char,
        sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    let mut email: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    if !email.is_null() {
        safe_strcpy(
            &raw mut (*user).email as *mut ::core::ffi::c_char,
            email,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
    }
    (*user).role =
        user_role_t(sqlite3_column_int(stmt, 3 as ::core::ffi::c_int) as ::core::ffi::c_uint);
    let mut api_key: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 4 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    if !api_key.is_null() {
        safe_strcpy(
            &raw mut (*user).api_key as *mut ::core::ffi::c_char,
            api_key,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    (*user).created_at = sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as int64_t;
    (*user).updated_at = sqlite3_column_int64(stmt, 6 as ::core::ffi::c_int) as int64_t;
    (*user).last_login = sqlite3_column_int64(stmt, 7 as ::core::ffi::c_int) as int64_t;
    (*user).is_active =
        sqlite3_column_int(stmt, 8 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    (*user).password_change_locked =
        sqlite3_column_int(stmt, 9 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    (*user).must_change_password =
        sqlite3_column_int(stmt, 10 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    (*user).totp_enabled =
        sqlite3_column_int(stmt, 11 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    let mut allowed_login_cidrs: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 12 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    if !allowed_login_cidrs.is_null()
        && *allowed_login_cidrs.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut (*user).allowed_login_cidrs as *mut ::core::ffi::c_char,
            allowed_login_cidrs,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            0 as size_t,
        );
        (*user).has_login_cidr_restriction = r#true != 0;
    }
    let mut authorization_mode: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 13 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    safe_strcpy(
        &raw mut (*user).authorization_mode as *mut ::core::ffi::c_char,
        if !authorization_mode.is_null() {
            authorization_mode
        } else {
            b"legacy\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
}
unsafe extern "C" fn principal_holds_global_action(
    mut user: *const user_t,
    mut action: authorization_action_t,
) -> bool {
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
    return !user.is_null()
        && authorization_evaluate(
            user,
            action,
            ::core::ptr::null::<fleet_camera_t>(),
            &raw mut evaluation,
        ) == 0 as ::core::ffi::c_int
        && evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0;
}
unsafe extern "C" fn built_in_role_uuid_for_legacy_role(
    mut role: user_role_t,
) -> *const ::core::ffi::c_char {
    match role {
        user_role_t::USER_ROLE_ADMIN => {
            return b"00000000-0000-4000-8000-000000000001\0".as_ptr()
                as *const ::core::ffi::c_char;
        }
        user_role_t::USER_ROLE_USER => {
            return b"00000000-0000-4000-8000-000000000002\0".as_ptr()
                as *const ::core::ffi::c_char;
        }
        user_role_t::USER_ROLE_VIEWER => {
            return b"00000000-0000-4000-8000-000000000003\0".as_ptr()
                as *const ::core::ffi::c_char;
        }
        user_role_t::USER_ROLE_API => {
            return b"00000000-0000-4000-8000-000000000004\0".as_ptr()
                as *const ::core::ffi::c_char;
        }
        _ => return ::core::ptr::null::<::core::ffi::c_char>(),
    };
}
unsafe extern "C" fn authorize_initial_user_role(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut role: user_role_t,
) -> bool {
    let mut requester: user_t = user_t {
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
    if httpd_check_action_access(req, &raw mut requester) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if requester.authenticated_via_scoped_token {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Scoped API tokens cannot create users\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut initial_role: authorization_role_t = authorization_role_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 256],
        is_builtin: false,
        action_mask: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut uuid: *const ::core::ffi::c_char = built_in_role_uuid_for_legacy_role(role);
    if uuid.is_null()
        || db_authorization_role_get(uuid, &raw mut initial_role).0
            != db_authorization_result_t::DB_AUTHORIZATION_OK.0
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to resolve initial user role\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut action: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while action < authorization_action_t::AUTHZ_ACTION_COUNT.0 {
        if initial_role.action_mask & authorization_action_bit(authorization_action_t(action)) != 0
        {
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
            if authorization_evaluate(
                &raw mut requester,
                authorization_action_t(action),
                ::core::ptr::null::<fleet_camera_t>(),
                &raw mut evaluation,
            ) != 0 as ::core::ffi::c_int
            {
                http_response_set_json_error(
                    res,
                    500 as ::core::ffi::c_int,
                    b"Failed to evaluate initial user role\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return r#false != 0;
            }
            if evaluation.decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0 {
                http_response_set_json_error(
                    res,
                    403 as ::core::ffi::c_int,
                    b"Initial user role exceeds your authority or camera scope\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return r#false != 0;
            }
        }
        action += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn provision_new_user_policy(
    mut user_id: int64_t,
    mut role: user_role_t,
) -> bool {
    let mut role_uuid: *const ::core::ffi::c_char = built_in_role_uuid_for_legacy_role(role);
    if role_uuid.is_null() {
        return r#false != 0;
    }
    let mut grant: authorization_grant_input_t = authorization_grant_input_t {
        role_uuid: [0; 37],
        scope_type: [0; 16],
        selector_json: [0; 8192],
        collection_uuid: [0; 37],
    };
    memset(
        &raw mut grant as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<authorization_grant_input_t>(),
    );
    safe_strcpy(
        &raw mut grant.role_uuid as *mut ::core::ffi::c_char,
        role_uuid,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut grant.scope_type as *mut ::core::ffi::c_char,
        b"all\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    let mut attempt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while attempt < 3 as ::core::ffi::c_int {
        let mut version: int64_t = 0 as int64_t;
        let mut new_version: int64_t = 0 as int64_t;
        if db_authorization_get_policy_version(&raw mut version) != 0 as ::core::ffi::c_int {
            return r#false != 0;
        }
        let mut result: db_authorization_result_t = db_authorization_replace_user_policy(
            user_id,
            b"policy\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut grant,
            1 as ::core::ffi::c_int,
            version,
            &raw mut new_version,
        );
        if result.0 == db_authorization_result_t::DB_AUTHORIZATION_OK.0 {
            return r#true != 0;
        }
        if result.0 != db_authorization_result_t::DB_AUTHORIZATION_STALE.0 {
            return r#false != 0;
        }
        attempt += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn check_generate_api_key_permission(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut target_user_id: int64_t,
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
    if httpd_check_action_access(req, &raw mut user) != 0 {
        if user.authenticated_via_scoped_token {
            http_response_set_json_error(
                res,
                403 as ::core::ffi::c_int,
                b"Scoped tokens cannot generate legacy API keys\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
        if principal_holds_global_action(&raw mut user, authorization_action_t::AUTHZ_USERS_MANAGE)
        {
            return 1 as ::core::ffi::c_int;
        }
        if user.id == target_user_id {
            return 1 as ::core::ffi::c_int;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Access denied: User '%s' (role: %s) cannot generate API key for user ID %lld\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut user.username as *mut ::core::ffi::c_char,
            db_auth_get_role_name(user.role),
            target_user_id as ::core::ffi::c_longlong,
        );
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Forbidden: You can only generate API keys for yourself unless you are an admin\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Access denied: Unauthenticated request attempted to generate API key\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    http_response_set_json_error(
        res,
        401 as ::core::ffi::c_int,
        b"Unauthorized: Authentication required\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn check_delete_user_permission(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut target_user_id: int64_t,
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
    if httpd_check_action_access(req, &raw mut user) != 0 {
        if principal_holds_global_action(&raw mut user, authorization_action_t::AUTHZ_USERS_MANAGE)
        {
            if user.id != target_user_id {
                return 1 as ::core::ffi::c_int;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Access denied: Admin '%s' attempted to delete themselves\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut user.username as *mut ::core::ffi::c_char,
            );
            http_response_set_json_error(
                res,
                403 as ::core::ffi::c_int,
                b"Forbidden: You cannot delete yourself\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Access denied: User '%s' (role: %s) cannot delete users\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut user.username as *mut ::core::ffi::c_char,
            db_auth_get_role_name(user.role),
        );
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Forbidden: Only admins can delete users\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Access denied: Unauthenticated request attempted to delete user\0".as_ptr()
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
pub unsafe extern "C" fn handle_users_list(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/auth/users request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_USERS_MANAGE) == 0 {
        return;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = prepare_user_select_stmt(
        db,
        b"ORDER BY id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stmt,
    );
    if rc != SQLITE_OK {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to prepare statement\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    let mut users_array: *mut cJSON = cJSON_CreateArray();
    while sqlite3_step(stmt) == SQLITE_ROW {
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
        populate_user_from_stmt(stmt, &raw mut user);
        cJSON_AddItemToArray(users_array, user_to_json(&raw mut user));
    }
    sqlite3_finalize(stmt);
    cJSON_AddItemToObject(
        response,
        b"users\0".as_ptr() as *const ::core::ffi::c_char,
        users_array,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/auth/users request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_users_get(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/auth/users/:id request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_USERS_MANAGE) == 0 {
        return;
    }
    let mut user_id: int64_t = 0;
    if !extract_user_id(
        req,
        res,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut user_id,
    ) {
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
    let mut rc: ::core::ffi::c_int = db_auth_get_user_by_id(user_id, &raw mut user);
    if rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"User not found: %lld\0".as_ptr() as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_longlong,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"User not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut user_json: *mut cJSON = user_to_json(&raw mut user);
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(user_json);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(user_json);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/auth/users/:id request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_users_create(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/auth/users request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_USERS_MANAGE) == 0 {
        return;
    }
    let mut json_req: *mut cJSON = httpd_parse_json_body(req);
    if json_req.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to parse user JSON from request body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON in request body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut username_json: *mut cJSON = cJSON_GetObjectItem(
        json_req,
        b"username\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut password_json: *mut cJSON = cJSON_GetObjectItem(
        json_req,
        b"password\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut email_json: *mut cJSON =
        cJSON_GetObjectItem(json_req, b"email\0".as_ptr() as *const ::core::ffi::c_char);
    let mut role_json: *mut cJSON =
        cJSON_GetObjectItem(json_req, b"role\0".as_ptr() as *const ::core::ffi::c_char);
    let mut is_active_json: *mut cJSON = cJSON_GetObjectItem(
        json_req,
        b"is_active\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut allowed_tags_create_json: *mut cJSON = cJSON_GetObjectItem(
        json_req,
        b"allowed_tags\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut allowed_login_cidrs_create_json: *mut cJSON = cJSON_GetObjectItem(
        json_req,
        b"allowed_login_cidrs\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !allowed_tags_create_json.is_null() {
        cJSON_Delete(json_req);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"allowed_tags is retired; update selector-backed authorization grants instead\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if username_json.is_null()
        || cJSON_IsString(username_json) == 0
        || password_json.is_null()
        || cJSON_IsString(password_json) == 0
    {
        cJSON_Delete(json_req);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing required fields: username and password\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut username_copy: [::core::ffi::c_char; 64] = [0; 64];
    let mut username: *const ::core::ffi::c_char = (*username_json).valuestring;
    safe_strcpy(
        &raw mut username_copy as *mut ::core::ffi::c_char,
        username,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    let mut password: *const ::core::ffi::c_char = (*password_json).valuestring;
    let mut email: *const ::core::ffi::c_char =
        if !email_json.is_null() && cJSON_IsString(email_json) != 0 {
            (*email_json).valuestring
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
    if !role_json.is_null() {
        if cJSON_IsNumber(role_json) == 0
            || (*role_json).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            || (*role_json).valuedouble > 3 as ::core::ffi::c_int as ::core::ffi::c_double
            || (*role_json).valuedouble != (*role_json).valueint as ::core::ffi::c_double
        {
            cJSON_Delete(json_req);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid role\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    let mut role: ::core::ffi::c_int = if !role_json.is_null() {
        (*role_json).valueint
    } else {
        user_role_t::USER_ROLE_USER.0 as ::core::ffi::c_int
    };
    let mut is_active: ::core::ffi::c_int =
        if !is_active_json.is_null() && cJSON_IsBool(is_active_json) != 0 {
            cJSON_IsTrue(is_active_json)
        } else {
            1 as ::core::ffi::c_int
        };
    if strlen(username) < 3 as size_t || strlen(username) > 32 as size_t {
        cJSON_Delete(json_req);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Username must be between 3 and 32 characters\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if strlen(password) < 8 as size_t {
        cJSON_Delete(json_req);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Password must be at least 8 characters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if role < 0 as ::core::ffi::c_int || role > 3 as ::core::ffi::c_int {
        cJSON_Delete(json_req);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid role\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut allowed_login_cidrs_buf: [::core::ffi::c_char; 1024] = [
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
    ];
    let mut has_cidr_create: bool = r#false != 0;
    let mut cidr_create_is_null: bool = r#false != 0;
    if !allowed_login_cidrs_create_json.is_null() {
        if cJSON_IsNull(allowed_login_cidrs_create_json) != 0 {
            has_cidr_create = r#true != 0;
            cidr_create_is_null = r#true != 0;
        } else if cJSON_IsString(allowed_login_cidrs_create_json) != 0 {
            safe_strcpy(
                &raw mut allowed_login_cidrs_buf as *mut ::core::ffi::c_char,
                (*allowed_login_cidrs_create_json).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                0 as size_t,
            );
            has_cidr_create = r#true != 0;
        } else {
            cJSON_Delete(json_req);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"allowed_login_cidrs must be a string or null\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        if db_auth_validate_allowed_login_cidrs(if cidr_create_is_null as ::core::ffi::c_int != 0 {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        } else {
            &raw mut allowed_login_cidrs_buf as *mut ::core::ffi::c_char
        }) != 0 as ::core::ffi::c_int
        {
            cJSON_Delete(json_req);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"allowed_login_cidrs must contain valid IPv4/IPv6 CIDR entries or single IP addresses\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    if !authorize_initial_user_role(req, res, user_role_t(role as ::core::ffi::c_uint)) {
        cJSON_Delete(json_req);
        return;
    }
    let mut user_id: int64_t = 0;
    let mut rc: ::core::ffi::c_int = db_auth_create_user(
        username,
        password,
        email,
        user_role_t(role as ::core::ffi::c_uint),
        r#false != 0,
        &raw mut user_id,
    );
    cJSON_Delete(json_req);
    if rc != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create user\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if has_cidr_create as ::core::ffi::c_int != 0
        && db_auth_set_allowed_login_cidrs(
            user_id,
            if cidr_create_is_null as ::core::ffi::c_int != 0 {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            } else {
                &raw mut allowed_login_cidrs_buf as *mut ::core::ffi::c_char
            },
        ) != 0 as ::core::ffi::c_int
    {
        db_auth_delete_user(user_id);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to save allowed_login_cidrs\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !provision_new_user_policy(user_id, user_role_t(role as ::core::ffi::c_uint)) {
        db_auth_delete_user(user_id);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to provision user authorization policy\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if is_active != 0
        && db_auth_update_user(
            user_id,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            -1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int
    {
        db_auth_delete_user(user_id);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to activate user\0".as_ptr() as *const ::core::ffi::c_char,
        );
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
    rc = db_auth_get_user_by_id(user_id, &raw mut user);
    if rc != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"User created but failed to retrieve\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = user_to_json(&raw mut user);
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully created user: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut username_copy as *mut ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_users_update(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling PUT /api/auth/users/:id request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut current_user: user_t = user_t {
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
    if httpd_check_action_access(req, &raw mut current_user) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut user_id: int64_t = 0;
    if !extract_user_id(
        req,
        res,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut user_id,
    ) {
        return;
    }
    let mut can_manage_users: bool = principal_holds_global_action(
        &raw mut current_user,
        authorization_action_t::AUTHZ_USERS_MANAGE,
    );
    let mut is_self_update: bool =
        !current_user.authenticated_via_scoped_token && current_user.id == user_id;
    if !can_manage_users && !is_self_update {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"You can only update your own account\0".as_ptr() as *const ::core::ffi::c_char,
        );
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
    let mut rc: ::core::ffi::c_int = db_auth_get_user_by_id(user_id, &raw mut user);
    if rc != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"User not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut json_req: *mut cJSON = httpd_parse_json_body(req);
    if json_req.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut username_json: *mut cJSON = cJSON_GetObjectItem(
        json_req,
        b"username\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut password_json: *mut cJSON = cJSON_GetObjectItem(
        json_req,
        b"password\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut email_json: *mut cJSON =
        cJSON_GetObjectItem(json_req, b"email\0".as_ptr() as *const ::core::ffi::c_char);
    let mut role_json: *mut cJSON =
        cJSON_GetObjectItem(json_req, b"role\0".as_ptr() as *const ::core::ffi::c_char);
    let mut is_active_json: *mut cJSON = cJSON_GetObjectItem(
        json_req,
        b"is_active\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut allowed_tags_json: *mut cJSON = cJSON_GetObjectItem(
        json_req,
        b"allowed_tags\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut allowed_login_cidrs_json: *mut cJSON = cJSON_GetObjectItem(
        json_req,
        b"allowed_login_cidrs\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !allowed_tags_json.is_null() {
        cJSON_Delete(json_req);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"allowed_tags is retired; update selector-backed authorization grants instead\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !role_json.is_null() {
        cJSON_Delete(json_req);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"role updates are not supported via this endpoint; update authorization grants instead\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !can_manage_users
        && (!password_json.is_null()
            || !role_json.is_null()
            || !is_active_json.is_null()
            || !allowed_login_cidrs_json.is_null())
    {
        cJSON_Delete(json_req);
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"You can only update your own username and email\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut username: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !username_json.is_null() {
        if cJSON_IsString(username_json) == 0 {
            cJSON_Delete(json_req);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"username must be a string\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        username = (*username_json).valuestring;
        if strlen(username) < 3 as size_t || strlen(username) > 32 as size_t {
            cJSON_Delete(json_req);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Username must be between 3 and 32 characters\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    let mut allowed_login_cidrs: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    let mut set_allowed_login_cidrs: bool = r#false != 0;
    if !allowed_login_cidrs_json.is_null() {
        if cJSON_IsNull(allowed_login_cidrs_json) != 0 {
            set_allowed_login_cidrs = r#true != 0;
        } else if cJSON_IsString(allowed_login_cidrs_json) != 0 {
            allowed_login_cidrs = (*allowed_login_cidrs_json).valuestring;
            set_allowed_login_cidrs = r#true != 0;
        } else {
            cJSON_Delete(json_req);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"allowed_login_cidrs must be a string or null\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        if db_auth_validate_allowed_login_cidrs(allowed_login_cidrs) != 0 as ::core::ffi::c_int {
            cJSON_Delete(json_req);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"allowed_login_cidrs must contain valid IPv4/IPv6 CIDR entries or single IP addresses\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    let mut password: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut has_password_update: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !password_json.is_null() {
        if cJSON_IsString(password_json) == 0 {
            cJSON_Delete(json_req);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"password must be a string\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        password = (*password_json).valuestring;
        if strlen(password) > 0 as size_t {
            if strlen(password) < 8 as size_t {
                cJSON_Delete(json_req);
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"Password must be at least 8 characters\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return;
            }
            has_password_update = 1 as ::core::ffi::c_int;
        }
    }
    let mut email: *const ::core::ffi::c_char =
        if !email_json.is_null() && cJSON_IsString(email_json) != 0 {
            (*email_json).valuestring
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
    let mut is_active: ::core::ffi::c_int =
        if !is_active_json.is_null() && cJSON_IsBool(is_active_json) != 0 {
            cJSON_IsTrue(is_active_json)
        } else {
            -1 as ::core::ffi::c_int
        };
    rc = db_auth_update_user_account(
        user_id,
        username,
        email,
        -1 as ::core::ffi::c_int,
        is_active,
        if has_password_update != 0 {
            password
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        },
        set_allowed_login_cidrs,
        allowed_login_cidrs,
    );
    cJSON_Delete(json_req);
    if rc != 0 as ::core::ffi::c_int {
        if rc == -2 as ::core::ffi::c_int {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Username already exists\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if rc == -3 as ::core::ffi::c_int {
            http_response_set_json_error(
                res,
                403 as ::core::ffi::c_int,
                b"Password changes are locked for this user\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to update user\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    rc = db_auth_get_user_by_id(user_id, &raw mut user);
    if rc != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"User updated but failed to retrieve\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = user_to_json(&raw mut user);
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully updated user: %s (ID: %lld)\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut user.username as *mut ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_users_delete(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling DELETE /api/auth/users/:id request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut user_id: int64_t = 0;
    if !extract_user_id(
        req,
        res,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut user_id,
    ) {
        return;
    }
    if check_delete_user_permission(req, res, user_id) == 0 {
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
    let mut rc: ::core::ffi::c_int = db_auth_get_user_by_id(user_id, &raw mut user);
    if rc != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"User not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if user.role.0 == user_role_t::USER_ROLE_ADMIN.0 {
        let mut db: *mut sqlite3 = get_db_handle();
        if db.is_null() {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = sqlite3_prepare_v2(
            db,
            b"SELECT COUNT(*) FROM users WHERE role = 0;\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc != SQLITE_OK {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to prepare statement\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        if sqlite3_step(stmt) == SQLITE_ROW {
            let mut admin_count: ::core::ffi::c_int =
                sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
            sqlite3_finalize(stmt);
            if admin_count <= 1 as ::core::ffi::c_int {
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"Cannot delete the last admin user\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
        } else {
            sqlite3_finalize(stmt);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to count admin users\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    rc = db_auth_delete_user(user_id);
    if rc != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to delete user\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        1 as cJSON_bool,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"User deleted successfully\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully deleted user ID: %lld\0".as_ptr() as *const ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_users_generate_api_key(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/auth/users/:id/api-key request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut user_id: int64_t = 0;
    if !extract_user_id(
        req,
        res,
        b"/api-key\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut user_id,
    ) {
        return;
    }
    if check_generate_api_key_permission(req, res, user_id) == 0 {
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
    let mut rc: ::core::ffi::c_int = db_auth_get_user_by_id(user_id, &raw mut user);
    if rc != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"User not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut api_key: [::core::ffi::c_char; 64] = [
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
    rc = db_auth_generate_api_key(
        user_id,
        &raw mut api_key as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    if rc != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to generate API key\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        1 as cJSON_bool,
    );
    cJSON_AddStringToObject(
        response,
        b"api_key\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut api_key as *mut ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully generated API key for user ID: %lld\0".as_ptr()
            as *const ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_users_change_password(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling PUT /api/auth/users/:id/password request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut current_user: user_t = user_t {
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
    if httpd_check_action_access(req, &raw mut current_user) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut target_user_id: int64_t = 0;
    if !extract_user_id(
        req,
        res,
        b"/password\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut target_user_id,
    ) {
        return;
    }
    let mut target_user: user_t = user_t {
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
    let mut rc: ::core::ffi::c_int = db_auth_get_user_by_id(target_user_id, &raw mut target_user);
    if rc != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"User not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut is_own_password: bool =
        !current_user.authenticated_via_scoped_token && current_user.id == target_user_id;
    let mut can_manage_users: bool = principal_holds_global_action(
        &raw mut current_user,
        authorization_action_t::AUTHZ_USERS_MANAGE,
    );
    if !can_manage_users && !is_own_password {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"You can only change your own password\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut json_req: *mut cJSON = httpd_parse_json_body(req);
    if json_req.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut old_password_json: *mut cJSON = cJSON_GetObjectItem(
        json_req,
        b"old_password\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut new_password_json: *mut cJSON = cJSON_GetObjectItem(
        json_req,
        b"new_password\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if new_password_json.is_null() || cJSON_IsString(new_password_json) == 0 {
        cJSON_Delete(json_req);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"New password is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut new_password: *const ::core::ffi::c_char = (*new_password_json).valuestring;
    if strlen(new_password) < 8 as size_t {
        cJSON_Delete(json_req);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Password must be at least 8 characters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut forced_own_change: bool = current_user.must_change_password as ::core::ffi::c_int != 0
        && is_own_password as ::core::ffi::c_int != 0
        && !g_config.demo_mode;
    let mut old_password: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !can_manage_users || forced_own_change as ::core::ffi::c_int != 0 {
        if cJSON_IsString(old_password_json) == 0 {
            cJSON_Delete(json_req);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Current password is required\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        old_password = (*old_password_json).valuestring;
    }
    rc = db_auth_change_password_verified(target_user_id, new_password, old_password);
    if rc == -4 as ::core::ffi::c_int {
        cJSON_Delete(json_req);
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Current password is incorrect\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if rc == -2 as ::core::ffi::c_int {
        cJSON_Delete(json_req);
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Password changes are locked for this user\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    } else if rc == -3 as ::core::ffi::c_int {
        cJSON_Delete(json_req);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"New password cannot be the default admin password\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    } else if rc != 0 as ::core::ffi::c_int {
        cJSON_Delete(json_req);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to change password\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_Delete(json_req);
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddBoolToObject(
        response,
        b"must_change_password\0".as_ptr() as *const ::core::ffi::c_char,
        r#false,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    if is_own_password {
        httpd_clear_session_cookie(res);
    }
    if is_own_password {
        httpd_clear_trusted_device_cookie(res);
    }
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully changed password for user ID: %lld\0".as_ptr() as *const ::core::ffi::c_char,
        target_user_id as ::core::ffi::c_longlong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_users_password_lock(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling PUT /api/auth/users/:id/password-lock request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_USERS_MANAGE) == 0 {
        return;
    }
    let mut user_id: int64_t = 0;
    if !extract_user_id(
        req,
        res,
        b"/password-lock\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut user_id,
    ) {
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
    let mut rc: ::core::ffi::c_int = db_auth_get_user_by_id(user_id, &raw mut user);
    if rc != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"User not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut json_req: *mut cJSON = httpd_parse_json_body(req);
    if json_req.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut locked_json: *mut cJSON =
        cJSON_GetObjectItem(json_req, b"locked\0".as_ptr() as *const ::core::ffi::c_char);
    if locked_json.is_null() || cJSON_IsBool(locked_json) == 0 {
        cJSON_Delete(json_req);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Locked field is required and must be a boolean\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut locked: bool = cJSON_IsTrue(locked_json) != 0;
    rc = db_auth_set_password_lock(user_id, locked);
    cJSON_Delete(json_req);
    if rc != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to update password lock status\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    rc = db_auth_get_user_by_id(user_id, &raw mut user);
    if rc != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Password lock updated but failed to retrieve user\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = user_to_json(&raw mut user);
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully updated password lock status for user: %s (ID: %lld, locked: %d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut user.username as *mut ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
        locked as ::core::ffi::c_int,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_users_clear_login_lockout(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/auth/users/:id/login-lockout/clear request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_USERS_MANAGE) == 0 {
        return;
    }
    let mut user_id: int64_t = 0;
    if !extract_user_id(
        req,
        res,
        b"/login-lockout/clear\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut user_id,
    ) {
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
    let mut rc: ::core::ffi::c_int = db_auth_get_user_by_id(user_id, &raw mut user);
    if rc != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"User not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut cleared: bool = auth_clear_login_rate_limit_for_username(
        &raw mut user.username as *mut ::core::ffi::c_char,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddBoolToObject(
        response,
        b"cleared\0".as_ptr() as *const ::core::ffi::c_char,
        cleared as cJSON_bool,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        if cleared as ::core::ffi::c_int != 0 {
            b"Login lockout cleared successfully\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"No active login lockout found\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Cleared login lockout for user: %s (ID: %lld, existed: %d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut user.username as *mut ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
        if cleared as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
