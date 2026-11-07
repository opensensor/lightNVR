unsafe extern "C" {
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsNull(item: *const cJSON) -> cJSON_bool;
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
    static mut g_config: config_t;
    unsafe fn db_location_get(
        uuid: *const ::core::ffi::c_char,
        location: *mut camera_location_t,
    ) -> db_location_result_t;
    unsafe fn db_location_count() -> ::core::ffi::c_int;
    unsafe fn db_location_list(
        locations: *mut camera_location_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_location_create(location: *mut camera_location_t) -> db_location_result_t;
    unsafe fn db_location_update(location: *mut camera_location_t) -> db_location_result_t;
    unsafe fn db_location_delete(uuid: *const ::core::ffi::c_char) -> db_location_result_t;
    unsafe fn db_location_assign_camera(
        camera_uuid: *const ::core::ffi::c_char,
        location_uuid: *const ::core::ffi::c_char,
    ) -> db_location_result_t;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn http_request_extract_path_param(
        req: *const http_request_t,
        prefix: *const ::core::ffi::c_char,
        param_buf: *mut ::core::ffi::c_char,
        buf_size: size_t,
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
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_authorize_global_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_authorize_camera_identity_action_with_context(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        camera_uuid: *const ::core::ffi::c_char,
        legacy_stream_name: *const ::core::ffi::c_char,
        user: *mut user_t,
        camera: *mut fleet_camera_t,
        evaluation: *mut authorization_evaluation_t,
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camera_location_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub parent_uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub r#type: [::core::ffi::c_char; 64],
    pub metadata_json: [::core::ffi::c_char; 512],
    pub sort_order: ::core::ffi::c_int,
    pub is_system: ::core::ffi::c_int,
    pub direct_child_count: ::core::ffi::c_int,
    pub direct_camera_count: ::core::ffi::c_int,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_location_result_t(pub ::core::ffi::c_int);
impl db_location_result_t {
    pub const DB_LOCATION_OK: Self = Self(0);
    pub const DB_LOCATION_NOT_FOUND: Self = Self(-1);
    pub const DB_LOCATION_CONFLICT: Self = Self(-2);
    pub const DB_LOCATION_INVALID: Self = Self(-3);
    pub const DB_LOCATION_ERROR: Self = Self(-4);
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
    pub const AUTHZ_ACTION_COUNT: Self = Self(19);
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
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
unsafe extern "C" fn valid_uuid_string(mut uuid: *const ::core::ffi::c_char) -> bool {
    return !uuid.is_null()
        && strlen(uuid) == (CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int) as size_t;
}
unsafe extern "C" fn location_to_json(mut location: *const camera_location_t) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_CreateObject();
    if object.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*location).uuid as *const ::core::ffi::c_char,
    );
    if (*location).parent_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            object,
            b"parent_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*location).parent_uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            object,
            b"parent_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddStringToObject(
        object,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*location).name as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*location).r#type as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        object,
        b"sort_order\0".as_ptr() as *const ::core::ffi::c_char,
        (*location).sort_order as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        object,
        b"is_system\0".as_ptr() as *const ::core::ffi::c_char,
        ((*location).is_system != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    let mut metadata: *mut cJSON =
        cJSON_Parse(&raw const (*location).metadata_json as *const ::core::ffi::c_char);
    if metadata.is_null() || cJSON_IsObject(metadata) == 0 {
        cJSON_Delete(metadata);
        metadata = cJSON_CreateObject();
    }
    cJSON_AddItemToObject(
        object,
        b"metadata\0".as_ptr() as *const ::core::ffi::c_char,
        metadata,
    );
    cJSON_AddNumberToObject(
        object,
        b"child_count\0".as_ptr() as *const ::core::ffi::c_char,
        (*location).direct_child_count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"camera_count\0".as_ptr() as *const ::core::ffi::c_char,
        (*location).direct_camera_count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*location).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*location).updated_at as ::core::ffi::c_double,
    );
    return object;
}
unsafe extern "C" fn set_location_json(
    mut res: *mut http_response_t,
    mut status: ::core::ffi::c_int,
    mut location: *const camera_location_t,
) {
    let mut object: *mut cJSON = location_to_json(location);
    let mut json: *mut ::core::ffi::c_char = if !object.is_null() {
        cJSON_PrintUnformatted(object)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    cJSON_Delete(object);
    if json.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize location\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, status, json);
    free(json as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn set_db_error(
    mut res: *mut http_response_t,
    mut result: db_location_result_t,
    mut conflict_message: *const ::core::ffi::c_char,
) {
    match result {
        db_location_result_t::DB_LOCATION_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Location or camera not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_location_result_t::DB_LOCATION_CONFLICT => {
            http_response_set_json_error(res, 409 as ::core::ffi::c_int, conflict_message);
        }
        db_location_result_t::DB_LOCATION_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid or immutable location\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Location database operation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn copy_json_string(
    mut root: *mut cJSON,
    mut field: *const ::core::ffi::c_char,
    mut destination: *mut ::core::ffi::c_char,
    mut destination_size: size_t,
    mut required: bool,
    mut res: *mut http_response_t,
) -> bool {
    let mut item: *mut cJSON = cJSON_GetObjectItemCaseSensitive(root, field);
    if item.is_null() {
        if required {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Missing required field\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        return r#true != 0;
    }
    if cJSON_IsString(item) == 0
        || (*item).valuestring.is_null()
        || *(*item).valuestring.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strlen((*item).valuestring) >= destination_size
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid string field\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        destination,
        (*item).valuestring,
        destination_size,
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn apply_parent_field(
    mut root: *mut cJSON,
    mut location: *mut camera_location_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut parent: *mut cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"parent_uuid\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if parent.is_null() {
        return r#true != 0;
    }
    if cJSON_IsNull(parent) != 0 {
        (*location).parent_uuid[0usize] = '\0' as ::core::ffi::c_char;
        return r#true != 0;
    }
    if cJSON_IsString(parent) == 0 || !valid_uuid_string((*parent).valuestring) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid parent_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        &raw mut (*location).parent_uuid as *mut ::core::ffi::c_char,
        (*parent).valuestring,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn apply_metadata_field(
    mut root: *mut cJSON,
    mut location: *mut camera_location_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut metadata: *mut cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"metadata\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if metadata.is_null() {
        return r#true != 0;
    }
    if cJSON_IsObject(metadata) == 0 {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"metadata must be an object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut serialized: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(metadata);
    if serialized.is_null()
        || strlen(serialized) >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
    {
        free(serialized as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"metadata is too large\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        &raw mut (*location).metadata_json as *mut ::core::ffi::c_char,
        serialized,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    free(serialized as *mut ::core::ffi::c_void);
    return r#true != 0;
}
unsafe extern "C" fn apply_location_fields(
    mut root: *mut cJSON,
    mut location: *mut camera_location_t,
    mut creating: bool,
    mut res: *mut http_response_t,
) -> bool {
    if cJSON_IsObject(root) == 0 {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Request body must be an object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !copy_json_string(
        root,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*location).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        creating,
        res,
    ) || !copy_json_string(
        root,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*location).r#type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        r#false != 0,
        res,
    ) || !apply_parent_field(root, location, res)
        || !apply_metadata_field(root, location, res)
    {
        return r#false != 0;
    }
    let mut sort_order: *mut cJSON = cJSON_GetObjectItemCaseSensitive(
        root,
        b"sort_order\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !sort_order.is_null() {
        if cJSON_IsNumber(sort_order) == 0 {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"sort_order must be a number\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*location).sort_order = (*sort_order).valueint;
    }
    return r#true != 0;
}
unsafe extern "C" fn extract_location_uuid(
    mut req: *const http_request_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut uuid_size: size_t,
    mut res: *mut http_response_t,
) -> bool {
    if http_request_extract_path_param(
        req,
        b"/api/locations/\0".as_ptr() as *const ::core::ffi::c_char,
        uuid,
        uuid_size,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid location path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut slash: *mut ::core::ffi::c_char = strchr(uuid, '/' as ::core::ffi::c_int);
    if !slash.is_null() {
        *slash = '\0' as ::core::ffi::c_char;
    }
    if !valid_uuid_string(uuid) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid location UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn load_visible_cameras(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut user: *mut user_t,
    mut cameras: *mut *mut fleet_camera_t,
    mut count: *mut ::core::ffi::c_int,
    mut all_fleet: *mut bool,
) -> bool {
    memset(
        user as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    *cameras = ::core::ptr::null_mut::<fleet_camera_t>();
    *count = 0 as ::core::ffi::c_int;
    *all_fleet = r#false != 0;
    if httpd_check_action_access(req, user) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
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
    *all_fleet = authorization_evaluate(
        user,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        ::core::ptr::null::<fleet_camera_t>(),
        &raw mut evaluation,
    ) == 0 as ::core::ffi::c_int
        && evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0
        || authorization_evaluate(
            user,
            authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
            ::core::ptr::null::<fleet_camera_t>(),
            &raw mut evaluation,
        ) == 0 as ::core::ffi::c_int
            && evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0;
    if db_fleet_camera_load(cameras, count) != 0 as ::core::ffi::c_int
        || authorization_filter_cameras(
            user,
            authorization_action_t::AUTHZ_LIVE_VIEW,
            *cameras,
            count,
        ) != 0 as ::core::ffi::c_int
    {
        free(*cameras as *mut ::core::ffi::c_void);
        *cameras = ::core::ptr::null_mut::<fleet_camera_t>();
        *count = 0 as ::core::ffi::c_int;
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn camera_reaches_location(
    mut camera: *const fleet_camera_t,
    mut location_uuid: *const ::core::ffi::c_char,
) -> bool {
    if strcmp(
        &raw const (*camera).location_uuid as *const ::core::ffi::c_char,
        location_uuid,
    ) == 0 as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*camera).location_depth {
        if strcmp(
            &raw const *(&raw const (*camera).location_ancestor_uuids
                as *const [::core::ffi::c_char; 37])
                .offset(i as isize) as *const ::core::ffi::c_char,
            location_uuid,
        ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn visible_location_camera_count(
    mut cameras: *const fleet_camera_t,
    mut camera_count: ::core::ffi::c_int,
    mut location_uuid: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < camera_count {
        if strcmp(
            &raw const (*cameras.offset(i as isize)).location_uuid as *const ::core::ffi::c_char,
            location_uuid,
        ) == 0 as ::core::ffi::c_int
        {
            count += 1;
        }
        i += 1;
    }
    return count;
}
unsafe extern "C" fn visible_direct_child_count(
    mut cameras: *const fleet_camera_t,
    mut camera_count: ::core::ffi::c_int,
    mut location_uuid: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut total: ::core::ffi::c_int = db_location_count();
    if total < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut locations: *mut camera_location_t = (if total > 0 as ::core::ffi::c_int {
        calloc(total as size_t, ::core::mem::size_of::<camera_location_t>())
    } else {
        NULL
    }) as *mut camera_location_t;
    if total > 0 as ::core::ffi::c_int && locations.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = if total > 0 as ::core::ffi::c_int {
        db_location_list(locations, total)
    } else {
        0 as ::core::ffi::c_int
    };
    if count < 0 as ::core::ffi::c_int {
        free(locations as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut visible_children: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut child: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while child < count {
        if strcmp(
            &raw mut (*locations.offset(child as isize)).parent_uuid as *mut ::core::ffi::c_char,
            location_uuid,
        ) == 0 as ::core::ffi::c_int
        {
            let mut camera_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while camera_index < camera_count {
                if camera_reaches_location(
                    cameras.offset(camera_index as isize),
                    &raw mut (*locations.offset(child as isize)).uuid as *mut ::core::ffi::c_char,
                ) {
                    visible_children += 1;
                    break;
                } else {
                    camera_index += 1;
                }
            }
        }
        child += 1;
    }
    free(locations as *mut ::core::ffi::c_void);
    return visible_children;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_locations(
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
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut all_fleet: bool = r#false != 0;
    if !load_visible_cameras(
        req,
        res,
        &raw mut user,
        &raw mut cameras,
        &raw mut camera_count,
        &raw mut all_fleet,
    ) {
        return;
    }
    let mut total: ::core::ffi::c_int = db_location_count();
    if total < 0 as ::core::ffi::c_int {
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to count locations\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut locations: *mut camera_location_t = (if total > 0 as ::core::ffi::c_int {
        calloc(total as size_t, ::core::mem::size_of::<camera_location_t>())
    } else {
        NULL
    }) as *mut camera_location_t;
    if total > 0 as ::core::ffi::c_int && locations.is_null() {
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int = if total > 0 as ::core::ffi::c_int {
        db_location_list(locations, total)
    } else {
        0 as ::core::ffi::c_int
    };
    if count < 0 as ::core::ffi::c_int {
        free(cameras as *mut ::core::ffi::c_void);
        free(locations as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to list locations\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || items.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(locations as *mut ::core::ffi::c_void);
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddItemToObject(
        root,
        b"locations\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    let mut visible_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut visible: bool = all_fleet;
        let mut camera_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while camera_index < camera_count {
            if camera_reaches_location(
                cameras.offset(camera_index as isize),
                &raw mut (*locations.offset(i as isize)).uuid as *mut ::core::ffi::c_char,
            ) {
                visible = r#true != 0;
                break;
            } else {
                camera_index += 1;
            }
        }
        if visible {
            let mut filtered: camera_location_t = *locations.offset(i as isize);
            if !all_fleet {
                filtered.direct_camera_count = visible_location_camera_count(
                    cameras,
                    camera_count,
                    &raw mut filtered.uuid as *mut ::core::ffi::c_char,
                );
                filtered.direct_child_count = 0 as ::core::ffi::c_int;
                let mut child: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while child < count {
                    if strcmp(
                        &raw mut (*locations.offset(child as isize)).parent_uuid
                            as *mut ::core::ffi::c_char,
                        &raw mut filtered.uuid as *mut ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        let mut camera_index_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while camera_index_0 < camera_count {
                            if camera_reaches_location(
                                cameras.offset(camera_index_0 as isize),
                                &raw mut (*locations.offset(child as isize)).uuid
                                    as *mut ::core::ffi::c_char,
                            ) {
                                filtered.direct_child_count += 1;
                                break;
                            } else {
                                camera_index_0 += 1;
                            }
                        }
                    }
                    child += 1;
                }
            }
            let mut item: *mut cJSON = location_to_json(&raw mut filtered);
            if item.is_null() {
                cJSON_Delete(root);
                free(locations as *mut ::core::ffi::c_void);
                free(cameras as *mut ::core::ffi::c_void);
                http_response_set_json_error(
                    res,
                    500 as ::core::ffi::c_int,
                    b"Failed to create location response\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            cJSON_AddItemToArray(items, item);
            visible_count += 1;
            if (*locations.offset(i as isize)).is_system != 0 {
                cJSON_AddStringToObject(
                    root,
                    b"unassigned_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*locations.offset(i as isize)).uuid as *mut ::core::ffi::c_char,
                );
            }
        }
        i += 1;
    }
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        visible_count as ::core::ffi::c_double,
    );
    free(locations as *mut ::core::ffi::c_void);
    free(cameras as *mut ::core::ffi::c_void);
    let mut json: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if json.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize locations\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json);
    free(json as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_location(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    if body.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON request body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut location: camera_location_t = camera_location_t {
        uuid: [0; 37],
        parent_uuid: [0; 37],
        name: [0; 128],
        r#type: [0; 64],
        metadata_json: [0; 512],
        sort_order: 0,
        is_system: 0,
        direct_child_count: 0,
        direct_camera_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    memset(
        &raw mut location as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<camera_location_t>(),
    );
    safe_strcpy(
        &raw mut location.r#type as *mut ::core::ffi::c_char,
        b"area\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut location.metadata_json as *mut ::core::ffi::c_char,
        b"{}\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    if !apply_location_fields(body, &raw mut location, r#true != 0, res) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    let mut result: db_location_result_t = db_location_create(&raw mut location);
    if result.0 != db_location_result_t::DB_LOCATION_OK.0 {
        set_db_error(
            res,
            result,
            b"A location with that name already exists under this parent\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    set_location_json(res, 201 as ::core::ffi::c_int, &raw mut location);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_location(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_location_uuid(
        req,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
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
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut all_fleet: bool = r#false != 0;
    if !load_visible_cameras(
        req,
        res,
        &raw mut user,
        &raw mut cameras,
        &raw mut camera_count,
        &raw mut all_fleet,
    ) {
        return;
    }
    let mut visible: bool = all_fleet;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < camera_count {
        if camera_reaches_location(
            cameras.offset(i as isize),
            &raw mut uuid as *mut ::core::ffi::c_char,
        ) {
            visible = r#true != 0;
            break;
        } else {
            i += 1;
        }
    }
    if !visible {
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut location: camera_location_t = camera_location_t {
        uuid: [0; 37],
        parent_uuid: [0; 37],
        name: [0; 128],
        r#type: [0; 64],
        metadata_json: [0; 512],
        sort_order: 0,
        is_system: 0,
        direct_child_count: 0,
        direct_camera_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_location_result_t =
        db_location_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut location);
    if result.0 != db_location_result_t::DB_LOCATION_OK.0 {
        free(cameras as *mut ::core::ffi::c_void);
        set_db_error(
            res,
            result,
            b"Location conflict\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !all_fleet {
        location.direct_camera_count = visible_location_camera_count(
            cameras,
            camera_count,
            &raw mut location.uuid as *mut ::core::ffi::c_char,
        );
        location.direct_child_count = visible_direct_child_count(
            cameras,
            camera_count,
            &raw mut location.uuid as *mut ::core::ffi::c_char,
        );
        if location.direct_child_count < 0 as ::core::ffi::c_int {
            free(cameras as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to evaluate visible child locations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    free(cameras as *mut ::core::ffi::c_void);
    set_location_json(res, 200 as ::core::ffi::c_int, &raw mut location);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_location(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_location_uuid(
        req,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) {
        return;
    }
    let mut location: camera_location_t = camera_location_t {
        uuid: [0; 37],
        parent_uuid: [0; 37],
        name: [0; 128],
        r#type: [0; 64],
        metadata_json: [0; 512],
        sort_order: 0,
        is_system: 0,
        direct_child_count: 0,
        direct_camera_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_location_result_t =
        db_location_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut location);
    if result.0 != db_location_result_t::DB_LOCATION_OK.0 {
        set_db_error(
            res,
            result,
            b"Location conflict\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    if body.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON request body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !apply_location_fields(body, &raw mut location, r#false != 0, res) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    result = db_location_update(&raw mut location);
    if result.0 != db_location_result_t::DB_LOCATION_OK.0 {
        set_db_error(
            res,
            result,
            b"Location move creates a cycle or sibling name conflicts\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    set_location_json(res, 200 as ::core::ffi::c_int, &raw mut location);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_location(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_location_uuid(
        req,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) {
        return;
    }
    let mut result: db_location_result_t =
        db_location_delete(&raw mut uuid as *mut ::core::ffi::c_char);
    if result.0 != db_location_result_t::DB_LOCATION_OK.0 {
        set_db_error(
            res,
            result,
            b"Location still contains cameras or child locations\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(
        res,
        200 as ::core::ffi::c_int,
        b"{\"success\":true}\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_camera_location(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut camera_path: [::core::ffi::c_char; 512] = [0; 512];
    if http_request_extract_path_param(
        req,
        b"/api/cameras/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut camera_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid camera path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut slash: *mut ::core::ffi::c_char = strchr(
        &raw mut camera_path as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if !slash.is_null() {
        *slash = '\0' as ::core::ffi::c_char;
    }
    if !valid_uuid_string(&raw mut camera_path as *mut ::core::ffi::c_char) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid camera UUID\0".as_ptr() as *const ::core::ffi::c_char,
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
    if httpd_authorize_camera_identity_action_with_context(
        req,
        res,
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        &raw mut camera_path as *mut ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut user,
        &raw mut camera,
        &raw mut evaluation,
    ) == 0
    {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut location_item: *mut cJSON = if !body.is_null() {
        cJSON_GetObjectItemCaseSensitive(
            body,
            b"location_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if body.is_null()
        || cJSON_IsObject(body) == 0
        || cJSON_IsString(location_item) == 0
        || !valid_uuid_string((*location_item).valuestring)
    {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"A valid location_uuid is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut location_uuid: [::core::ffi::c_char; 37] = [0; 37];
    safe_strcpy(
        &raw mut location_uuid as *mut ::core::ffi::c_char,
        (*location_item).valuestring,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    cJSON_Delete(body);
    let mut result: db_location_result_t = db_location_assign_camera(
        &raw mut camera_path as *mut ::core::ffi::c_char,
        &raw mut location_uuid as *mut ::core::ffi::c_char,
    );
    if result.0 != db_location_result_t::DB_LOCATION_OK.0 {
        set_db_error(
            res,
            result,
            b"Camera location assignment conflict\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !g_config.streams.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < g_config.max_streams {
            if strcmp(
                &raw mut (*g_config.streams.offset(i as isize)).camera_uuid
                    as *mut ::core::ffi::c_char,
                &raw mut camera_path as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                safe_strcpy(
                    &raw mut (*g_config.streams.offset(i as isize)).location_uuid
                        as *mut ::core::ffi::c_char,
                    &raw mut location_uuid as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                    0 as size_t,
                );
                break;
            } else {
                i += 1;
            }
        }
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    if response.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut camera_path as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        response,
        b"location_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut location_uuid as *mut ::core::ffi::c_char,
    );
    let mut json: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    if json.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json);
    free(json as *mut ::core::ffi::c_void);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
