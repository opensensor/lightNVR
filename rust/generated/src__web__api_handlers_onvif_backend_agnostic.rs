unsafe extern "C" {
    unsafe fn cJSON_ParseWithLengthOpts(
        value: *const ::core::ffi::c_char,
        buffer_length: size_t,
        return_parse_end: *mut *const ::core::ffi::c_char,
        require_null_terminated: cJSON_bool,
    ) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateString(string: *const ::core::ffi::c_char) -> *mut cJSON;
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
    unsafe fn url_strip_credentials(
        url: *const ::core::ffi::c_char,
        out_url: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_discovered_onvif_devices(
        devices: *mut onvif_device_info_t,
        max_devices: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn get_onvif_device_profiles(
        device_url: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        profiles: *mut onvif_profile_t,
        max_profiles: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn add_onvif_device_as_stream(
        device_info: *const onvif_device_info_t,
        profile: *const onvif_profile_t,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn discover_onvif_devices(
        network: *const ::core::ffi::c_char,
        devices: *mut onvif_device_info_t,
        max_devices: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn test_onvif_connection(
        url: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_onvif_inventory_list_addresses(
        inventory_uuid: *const ::core::ffi::c_char,
        addresses: *mut [::core::ffi::c_char; 512],
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_onvif_inventory_claim_stream(
        inventory_uuid: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
    ) -> db_onvif_inventory_result_t;
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
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
pub struct onvif_device_info_t {
    pub inventory_uuid: [::core::ffi::c_char; 37],
    pub endpoint: [::core::ffi::c_char; 512],
    pub device_service: [::core::ffi::c_char; 512],
    pub media_service: [::core::ffi::c_char; 512],
    pub ptz_service: [::core::ffi::c_char; 512],
    pub imaging_service: [::core::ffi::c_char; 512],
    pub manufacturer: [::core::ffi::c_char; 64],
    pub model: [::core::ffi::c_char; 64],
    pub firmware_version: [::core::ffi::c_char; 32],
    pub serial_number: [::core::ffi::c_char; 64],
    pub hardware_id: [::core::ffi::c_char; 64],
    pub ip_address: [::core::ffi::c_char; 64],
    pub mac_address: [::core::ffi::c_char; 32],
    pub discovery_time: time_t,
    pub first_seen_at: time_t,
    pub last_seen_at: time_t,
    pub last_scan_network: [::core::ffi::c_char; 64],
    pub claim_state: [::core::ffi::c_char; 16],
    pub claimed_camera_uuid: [::core::ffi::c_char; 37],
    pub duplicate_suspected: bool,
    pub online: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_profile_t {
    pub token: [::core::ffi::c_char; 64],
    pub name: [::core::ffi::c_char; 64],
    pub video_source_token: [::core::ffi::c_char; 64],
    pub ptz_configuration_token: [::core::ffi::c_char; 64],
    pub snapshot_uri: [::core::ffi::c_char; 512],
    pub stream_uri: [::core::ffi::c_char; 512],
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub encoding: [::core::ffi::c_char; 16],
    pub fps: ::core::ffi::c_int,
    pub bitrate: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_onvif_inventory_result_t(pub ::core::ffi::c_int);
impl db_onvif_inventory_result_t {
    pub const DB_ONVIF_INVENTORY_OK: Self = Self(0);
    pub const DB_ONVIF_INVENTORY_NOT_FOUND: Self = Self(-1);
    pub const DB_ONVIF_INVENTORY_INVALID: Self = Self(-2);
    pub const DB_ONVIF_INVENTORY_CONFLICT: Self = Self(-3);
    pub const DB_ONVIF_INVENTORY_ERROR: Self = Self(-4);
}
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MAX_URL_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn device_json_body(mut req: *const http_request_t) -> *mut cJSON {
    if (*req).body.is_null() || (*req).body_len == 0 {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut limit: *const ::core::ffi::c_char =
        (*req).body.offset((*req).body_len as isize) as *const ::core::ffi::c_char;
    let mut body: *mut cJSON = cJSON_ParseWithLengthOpts(
        (*req).body as *const ::core::ffi::c_char,
        (*req).body_len,
        &raw mut end,
        0 as cJSON_bool,
    );
    while !end.is_null()
        && end < limit
        && *end as ::core::ffi::c_int != 0
        && !strchr(
            b" \t\r\n\0".as_ptr() as *const ::core::ffi::c_char,
            *end as ::core::ffi::c_int,
        )
        .is_null()
    {
        end = end.offset(1);
    }
    if body.is_null() || cJSON_IsObject(body) == 0 || end != limit {
        cJSON_Delete(body);
        return ::core::ptr::null_mut::<cJSON>();
    }
    return body;
}
#[inline]
unsafe extern "C" fn device_keys(
    mut body: *const cJSON,
    mut keys: *const *const ::core::ffi::c_char,
    mut count: size_t,
) -> bool {
    if cJSON_IsObject(body) == 0 {
        return r#false != 0;
    }
    let mut field: *const cJSON = ::core::ptr::null::<cJSON>();
    field = if !body.is_null() {
        (*body).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !field.is_null() {
        let mut known: bool = r#false != 0;
        let mut i: size_t = 0 as size_t;
        while i < count {
            if strcmp((*field).string, *keys.offset(i as isize)) == 0 {
                known = r#true != 0;
            }
            i = i.wrapping_add(1);
        }
        if !known {
            return r#false != 0;
        }
        let mut prior: *const cJSON = (*body).child;
        while prior != field {
            if strcmp((*prior).string, (*field).string) == 0 {
                return r#false != 0;
            }
            prior = (*prior).next;
        }
        field = (*field).next;
    }
    return r#true != 0;
}
#[inline]
unsafe extern "C" fn device_string(
    mut field: *const cJSON,
    mut maximum: size_t,
    mut empty: bool,
) -> bool {
    return cJSON_IsString(field) != 0
        && !(*field).valuestring.is_null()
        && (empty as ::core::ffi::c_int != 0 || *(*field).valuestring as ::core::ffi::c_int != 0)
        && strlen((*field).valuestring) <= maximum;
}
pub const LOG_COMPONENT: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"ONVIFAPI\0") };
unsafe extern "C" fn add_inventory_metadata(
    mut json: *mut cJSON,
    mut device: *const onvif_device_info_t,
) {
    cJSON_AddStringToObject(
        json,
        b"inventory_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*device).inventory_uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        json,
        b"first_seen_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*device).first_seen_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        json,
        b"last_seen_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*device).last_seen_at as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        json,
        b"last_scan_network\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*device).last_scan_network as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        json,
        b"claim_state\0".as_ptr() as *const ::core::ffi::c_char,
        if (*device).claim_state[0usize] as ::core::ffi::c_int != 0 {
            &raw const (*device).claim_state as *const ::core::ffi::c_char
        } else {
            b"unclaimed\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddStringToObject(
        json,
        b"claimed_camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*device).claimed_camera_uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        json,
        b"duplicate_suspected\0".as_ptr() as *const ::core::ffi::c_char,
        (*device).duplicate_suspected as cJSON_bool,
    );
    let mut addresses_json: *mut cJSON =
        cJSON_AddArrayToObject(json, b"addresses\0".as_ptr() as *const ::core::ffi::c_char);
    if addresses_json.is_null() || (*device).inventory_uuid[0usize] == 0 {
        return;
    }
    let mut addresses: [[::core::ffi::c_char; 512]; 16] = [[0; 512]; 16];
    let mut count: ::core::ffi::c_int = db_onvif_inventory_list_addresses(
        &raw const (*device).inventory_uuid as *const ::core::ffi::c_char,
        &raw mut addresses as *mut [::core::ffi::c_char; 512],
        ONVIF_DISCOVERY_ADDRESS_MAX,
    );
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        cJSON_AddItemToArray(
            addresses_json,
            cJSON_CreateString(
                &raw mut *(&raw mut addresses as *mut [::core::ffi::c_char; 512])
                    .offset(index as isize) as *mut ::core::ffi::c_char,
            ),
        );
        index += 1;
    }
}
unsafe extern "C" fn valid_onvif_body(
    mut body: *const cJSON,
    mut operation: *const ::core::ffi::c_char,
) -> bool {
    static mut discovery: [*const ::core::ffi::c_char; 1] =
        [b"network\0".as_ptr() as *const ::core::ffi::c_char];
    static mut test: [*const ::core::ffi::c_char; 4] = [
        b"url\0".as_ptr() as *const ::core::ffi::c_char,
        b"username\0".as_ptr() as *const ::core::ffi::c_char,
        b"password\0".as_ptr() as *const ::core::ffi::c_char,
        b"insecure\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut add: [*const ::core::ffi::c_char; 5] = [
        b"device_url\0".as_ptr() as *const ::core::ffi::c_char,
        b"profile_token\0".as_ptr() as *const ::core::ffi::c_char,
        b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
        b"username\0".as_ptr() as *const ::core::ffi::c_char,
        b"password\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut claim: [*const ::core::ffi::c_char; 2] = [
        b"inventory_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut keys: *const *const ::core::ffi::c_char =
        &raw mut discovery as *mut *const ::core::ffi::c_char;
    let mut count: size_t = 1 as size_t;
    if strcmp(operation, b"test\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
        keys = &raw mut test as *mut *const ::core::ffi::c_char;
        count = 4 as size_t;
    }
    if strcmp(operation, b"add\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
        keys = &raw mut add as *mut *const ::core::ffi::c_char;
        count = 5 as size_t;
    }
    if strcmp(operation, b"claim\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
        keys = &raw mut claim as *mut *const ::core::ffi::c_char;
        count = 2 as size_t;
    }
    if !device_keys(body, keys, count) {
        return r#false != 0;
    }
    let mut field: *const cJSON = ::core::ptr::null::<cJSON>();
    field = if !body.is_null() {
        (*body).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !field.is_null() {
        if strcmp(
            (*field).string,
            b"insecure\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            if cJSON_IsBool(field) == 0 {
                return r#false != 0;
            }
        } else {
            let mut maximum: size_t = 63 as size_t;
            if strcmp(
                (*field).string,
                b"url\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
                || strcmp(
                    (*field).string,
                    b"device_url\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0
            {
                maximum = (MAX_URL_LENGTH - 1 as ::core::ffi::c_int) as size_t;
            }
            if strcmp(
                (*field).string,
                b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            {
                maximum = (MAX_STREAM_NAME - 1 as ::core::ffi::c_int) as size_t;
            }
            if strcmp(
                (*field).string,
                b"network\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            {
                maximum = 255 as size_t;
            }
            let mut empty: bool = strcmp(
                (*field).string,
                b"network\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
                || strcmp(
                    (*field).string,
                    b"username\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0
                || strcmp(
                    (*field).string,
                    b"password\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0;
            if !device_string(field, maximum, empty) {
                return r#false != 0;
            }
        }
        field = (*field).next;
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_onvif_discovery_status(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/onvif/discovery/status request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        root,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.onvif_discovery_enabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        root,
        b"network\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.onvif_discovery_network as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        root,
        b"interval\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.onvif_discovery_interval as ::core::ffi::c_double,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to generate JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to generate JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/onvif/discovery/status request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_discovered_onvif_devices(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/onvif/devices request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut devices: [onvif_device_info_t; 32] = [onvif_device_info_t {
        inventory_uuid: [0; 37],
        endpoint: [0; 512],
        device_service: [0; 512],
        media_service: [0; 512],
        ptz_service: [0; 512],
        imaging_service: [0; 512],
        manufacturer: [0; 64],
        model: [0; 64],
        firmware_version: [0; 32],
        serial_number: [0; 64],
        hardware_id: [0; 64],
        ip_address: [0; 64],
        mac_address: [0; 32],
        discovery_time: 0,
        first_seen_at: 0,
        last_seen_at: 0,
        last_scan_network: [0; 64],
        claim_state: [0; 16],
        claimed_camera_uuid: [0; 37],
        duplicate_suspected: false,
        online: false,
    }; 32];
    let mut count: ::core::ffi::c_int = get_discovered_onvif_devices(
        &raw mut devices as *mut onvif_device_info_t,
        32 as ::core::ffi::c_int,
    );
    if count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to get discovered ONVIF devices\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get discovered ONVIF devices\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut devices_array: *mut cJSON =
        cJSON_AddArrayToObject(root, b"devices\0".as_ptr() as *const ::core::ffi::c_char);
    if devices_array.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(root);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut device: *mut cJSON = cJSON_CreateObject();
        if device.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            cJSON_Delete(root);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddStringToObject(
            device,
            b"endpoint\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize)).endpoint
                as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"device_service\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .device_service as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"media_service\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .media_service as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"ptz_service\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .ptz_service as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"imaging_service\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .imaging_service as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"manufacturer\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .manufacturer as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"model\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize)).model
                as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"firmware_version\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .firmware_version as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"serial_number\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .serial_number as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"hardware_id\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .hardware_id as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"ip_address\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize)).ip_address
                as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"mac_address\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .mac_address as *mut ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            device,
            b"discovery_time\0".as_ptr() as *const ::core::ffi::c_char,
            devices[i as usize].discovery_time as ::core::ffi::c_double,
        );
        cJSON_AddBoolToObject(
            device,
            b"online\0".as_ptr() as *const ::core::ffi::c_char,
            devices[i as usize].online as cJSON_bool,
        );
        add_inventory_metadata(
            device,
            (&raw mut devices as *mut onvif_device_info_t).offset(i as isize),
        );
        cJSON_AddItemToArray(devices_array, device);
        i += 1;
    }
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to generate JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to generate JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/onvif/devices request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_discover_onvif_devices(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/onvif/discovery/discover request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut root: *mut cJSON = device_json_body(req);
    if !valid_onvif_body(root, b"discovery\0".as_ptr() as *const ::core::ffi::c_char) {
        cJSON_Delete(root);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid JSON request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut network_json: *mut cJSON =
        cJSON_GetObjectItem(root, b"network\0".as_ptr() as *const ::core::ffi::c_char);
    let mut network: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !network_json.is_null() && cJSON_IsString(network_json) != 0 {
        network = (*network_json).valuestring;
        if strcmp(network, b"auto\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strlen(network) == 0 as size_t
        {
            network = ::core::ptr::null::<::core::ffi::c_char>();
        }
    }
    if network.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Network parameter not provided or set to 'auto', will use auto-detection\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut devices: [onvif_device_info_t; 32] = [onvif_device_info_t {
        inventory_uuid: [0; 37],
        endpoint: [0; 512],
        device_service: [0; 512],
        media_service: [0; 512],
        ptz_service: [0; 512],
        imaging_service: [0; 512],
        manufacturer: [0; 64],
        model: [0; 64],
        firmware_version: [0; 32],
        serial_number: [0; 64],
        hardware_id: [0; 64],
        ip_address: [0; 64],
        mac_address: [0; 32],
        discovery_time: 0,
        first_seen_at: 0,
        last_seen_at: 0,
        last_scan_network: [0; 64],
        claim_state: [0; 16],
        claimed_camera_uuid: [0; 37],
        duplicate_suspected: false,
        online: false,
    }; 32];
    let mut count: ::core::ffi::c_int = discover_onvif_devices(
        network,
        &raw mut devices as *mut onvif_device_info_t,
        32 as ::core::ffi::c_int,
    );
    cJSON_Delete(root);
    if count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to discover ONVIF devices\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to discover ONVIF devices\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response_json: *mut cJSON = cJSON_CreateObject();
    if response_json.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut devices_array: *mut cJSON = cJSON_AddArrayToObject(
        response_json,
        b"devices\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if devices_array.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(response_json);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut device: *mut cJSON = cJSON_CreateObject();
        if device.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            cJSON_Delete(response_json);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddStringToObject(
            device,
            b"endpoint\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize)).endpoint
                as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"device_service\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .device_service as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"media_service\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .media_service as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"ptz_service\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .ptz_service as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"imaging_service\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .imaging_service as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"manufacturer\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .manufacturer as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"model\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize)).model
                as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"firmware_version\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .firmware_version as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"serial_number\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .serial_number as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"hardware_id\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .hardware_id as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"ip_address\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize)).ip_address
                as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            device,
            b"mac_address\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut devices as *mut onvif_device_info_t).offset(i as isize))
                .mac_address as *mut ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            device,
            b"discovery_time\0".as_ptr() as *const ::core::ffi::c_char,
            devices[i as usize].discovery_time as ::core::ffi::c_double,
        );
        cJSON_AddBoolToObject(
            device,
            b"online\0".as_ptr() as *const ::core::ffi::c_char,
            devices[i as usize].online as cJSON_bool,
        );
        add_inventory_metadata(
            device,
            (&raw mut devices as *mut onvif_device_info_t).offset(i as isize),
        );
        cJSON_AddItemToArray(devices_array, device);
        i += 1;
    }
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response_json);
    cJSON_Delete(response_json);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to generate JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to generate JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled POST /api/onvif/discovery/discover request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_onvif_device_profiles(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/onvif/device/profiles request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut device_url_param: *const ::core::ffi::c_char = http_request_get_header(
        req,
        b"X-Device-URL\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut username_param: *const ::core::ffi::c_char =
        http_request_get_header(req, b"X-Username\0".as_ptr() as *const ::core::ffi::c_char);
    let mut password_param: *const ::core::ffi::c_char =
        http_request_get_header(req, b"X-Password\0".as_ptr() as *const ::core::ffi::c_char);
    if device_url_param.is_null()
        || *device_url_param == 0
        || strlen(device_url_param) >= MAX_URL_LENGTH as size_t
        || !username_param.is_null() && strlen(username_param) >= 64 as size_t
        || !password_param.is_null() && strlen(password_param) >= 64 as size_t
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Missing device_url parameter\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing device_url parameter\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut device_url: [::core::ffi::c_char; 512] = [0; 512];
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
    safe_strcpy(
        &raw mut device_url as *mut ::core::ffi::c_char,
        device_url_param,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    if !username_param.is_null() {
        safe_strcpy(
            &raw mut username as *mut ::core::ffi::c_char,
            username_param,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    if !password_param.is_null() {
        safe_strcpy(
            &raw mut password as *mut ::core::ffi::c_char,
            password_param,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    let mut profiles: [onvif_profile_t; 16] = [onvif_profile_t {
        token: [0; 64],
        name: [0; 64],
        video_source_token: [0; 64],
        ptz_configuration_token: [0; 64],
        snapshot_uri: [0; 512],
        stream_uri: [0; 512],
        width: 0,
        height: 0,
        encoding: [0; 16],
        fps: 0,
        bitrate: 0,
    }; 16];
    let mut count: ::core::ffi::c_int = get_onvif_device_profiles(
        &raw mut device_url as *mut ::core::ffi::c_char,
        if username[0usize] as ::core::ffi::c_int != 0 {
            &raw mut username as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        if password[0usize] as ::core::ffi::c_int != 0 {
            &raw mut password as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        &raw mut profiles as *mut onvif_profile_t,
        16 as ::core::ffi::c_int,
    );
    if count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to get ONVIF device profiles\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get ONVIF device profiles\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut profiles_array: *mut cJSON =
        cJSON_AddArrayToObject(root, b"profiles\0".as_ptr() as *const ::core::ffi::c_char);
    if profiles_array.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(root);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut profile: *mut cJSON = cJSON_CreateObject();
        if profile.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            cJSON_Delete(root);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddStringToObject(
            profile,
            b"token\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut profiles as *mut onvif_profile_t).offset(i as isize)).token
                as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            profile,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut profiles as *mut onvif_profile_t).offset(i as isize)).name
                as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            profile,
            b"video_source_token\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut profiles as *mut onvif_profile_t).offset(i as isize))
                .video_source_token as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            profile,
            b"ptz_configuration_token\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut profiles as *mut onvif_profile_t).offset(i as isize))
                .ptz_configuration_token as *mut ::core::ffi::c_char,
        );
        cJSON_AddBoolToObject(
            profile,
            b"ptz_supported\0".as_ptr() as *const ::core::ffi::c_char,
            (profiles[i as usize].ptz_configuration_token[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
        let mut safe_snapshot_uri: [::core::ffi::c_char; 512] = [0; 512];
        let mut safe_stream_uri: [::core::ffi::c_char; 512] = [0; 512];
        if url_strip_credentials(
            &raw mut (*(&raw mut profiles as *mut onvif_profile_t).offset(i as isize)).snapshot_uri
                as *mut ::core::ffi::c_char,
            &raw mut safe_snapshot_uri as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut safe_snapshot_uri as *mut ::core::ffi::c_char,
                &raw mut (*(&raw mut profiles as *mut onvif_profile_t).offset(i as isize))
                    .snapshot_uri as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        }
        if url_strip_credentials(
            &raw mut (*(&raw mut profiles as *mut onvif_profile_t).offset(i as isize)).stream_uri
                as *mut ::core::ffi::c_char,
            &raw mut safe_stream_uri as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut safe_stream_uri as *mut ::core::ffi::c_char,
                &raw mut (*(&raw mut profiles as *mut onvif_profile_t).offset(i as isize))
                    .stream_uri as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        }
        cJSON_AddStringToObject(
            profile,
            b"snapshot_uri\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut safe_snapshot_uri as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            profile,
            b"stream_uri\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut safe_stream_uri as *mut ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            profile,
            b"width\0".as_ptr() as *const ::core::ffi::c_char,
            profiles[i as usize].width as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            profile,
            b"height\0".as_ptr() as *const ::core::ffi::c_char,
            profiles[i as usize].height as ::core::ffi::c_double,
        );
        cJSON_AddStringToObject(
            profile,
            b"encoding\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut profiles as *mut onvif_profile_t).offset(i as isize)).encoding
                as *mut ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            profile,
            b"fps\0".as_ptr() as *const ::core::ffi::c_char,
            profiles[i as usize].fps as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            profile,
            b"bitrate\0".as_ptr() as *const ::core::ffi::c_char,
            profiles[i as usize].bitrate as ::core::ffi::c_double,
        );
        cJSON_AddItemToArray(profiles_array, profile);
        i += 1;
    }
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to generate JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to generate JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/onvif/device/profiles request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_add_onvif_device_as_stream(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/onvif/device/add request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut root: *mut cJSON = device_json_body(req);
    if !valid_onvif_body(root, b"add\0".as_ptr() as *const ::core::ffi::c_char) {
        cJSON_Delete(root);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid JSON request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut device_url_json: *mut cJSON =
        cJSON_GetObjectItem(root, b"device_url\0".as_ptr() as *const ::core::ffi::c_char);
    let mut profile_token_json: *mut cJSON = cJSON_GetObjectItem(
        root,
        b"profile_token\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut stream_name_json: *mut cJSON = cJSON_GetObjectItem(
        root,
        b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut username_json: *mut cJSON =
        cJSON_GetObjectItem(root, b"username\0".as_ptr() as *const ::core::ffi::c_char);
    let mut password_json: *mut cJSON =
        cJSON_GetObjectItem(root, b"password\0".as_ptr() as *const ::core::ffi::c_char);
    if device_url_json.is_null()
        || cJSON_IsString(device_url_json) == 0
        || profile_token_json.is_null()
        || cJSON_IsString(profile_token_json) == 0
        || stream_name_json.is_null()
        || cJSON_IsString(stream_name_json) == 0
    {
        cJSON_Delete(root);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Missing or invalid parameters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing or invalid parameters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut device_url: *const ::core::ffi::c_char = (*device_url_json).valuestring;
    let mut profile_token: *const ::core::ffi::c_char = (*profile_token_json).valuestring;
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        (*stream_name_json).valuestring,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    let mut username: *const ::core::ffi::c_char =
        if !username_json.is_null() && cJSON_IsString(username_json) != 0 {
            (*username_json).valuestring
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
    let mut password: *const ::core::ffi::c_char =
        if !password_json.is_null() && cJSON_IsString(password_json) != 0 {
            (*password_json).valuestring
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
    if strlen(device_url) == 0 as size_t
        || strlen(profile_token) == 0 as size_t
        || strlen(&raw mut stream_name as *mut ::core::ffi::c_char) == 0 as size_t
    {
        cJSON_Delete(root);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid parameters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid parameters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut profiles: [onvif_profile_t; 16] = [onvif_profile_t {
        token: [0; 64],
        name: [0; 64],
        video_source_token: [0; 64],
        ptz_configuration_token: [0; 64],
        snapshot_uri: [0; 512],
        stream_uri: [0; 512],
        width: 0,
        height: 0,
        encoding: [0; 16],
        fps: 0,
        bitrate: 0,
    }; 16];
    let mut count: ::core::ffi::c_int = get_onvif_device_profiles(
        device_url,
        username,
        password,
        &raw mut profiles as *mut onvif_profile_t,
        16 as ::core::ffi::c_int,
    );
    if count < 0 as ::core::ffi::c_int {
        cJSON_Delete(root);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to get ONVIF device profiles\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get ONVIF device profiles\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut profile: *const onvif_profile_t = ::core::ptr::null::<onvif_profile_t>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if strcmp(
            &raw mut (*(&raw mut profiles as *mut onvif_profile_t).offset(i as isize)).token
                as *mut ::core::ffi::c_char,
            profile_token,
        ) == 0 as ::core::ffi::c_int
        {
            profile = (&raw mut profiles as *mut onvif_profile_t).offset(i as isize);
            break;
        } else {
            i += 1;
        }
    }
    if profile.is_null() {
        cJSON_Delete(root);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Profile not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Profile not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut device_info: onvif_device_info_t = onvif_device_info_t {
        inventory_uuid: [0; 37],
        endpoint: [0; 512],
        device_service: [0; 512],
        media_service: [0; 512],
        ptz_service: [0; 512],
        imaging_service: [0; 512],
        manufacturer: [0; 64],
        model: [0; 64],
        firmware_version: [0; 32],
        serial_number: [0; 64],
        hardware_id: [0; 64],
        ip_address: [0; 64],
        mac_address: [0; 32],
        discovery_time: 0,
        first_seen_at: 0,
        last_seen_at: 0,
        last_scan_network: [0; 64],
        claim_state: [0; 16],
        claimed_camera_uuid: [0; 37],
        duplicate_suspected: false,
        online: false,
    };
    memset(
        &raw mut device_info as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<onvif_device_info_t>(),
    );
    safe_strcpy(
        &raw mut device_info.device_service as *mut ::core::ffi::c_char,
        device_url,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    let mut result: ::core::ffi::c_int = add_onvif_device_as_stream(
        &raw mut device_info,
        profile,
        username,
        password,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    cJSON_Delete(root);
    if result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to add ONVIF device as stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to add ONVIF device as stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response_json: *mut cJSON = cJSON_CreateObject();
    if response_json.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        response_json,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response_json,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"ONVIF device added as stream successfully\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        response_json,
        b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response_json);
    cJSON_Delete(response_json);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to generate JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to generate JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled POST /api/onvif/device/add request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_claim_onvif_device(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    let mut root: *mut cJSON = device_json_body(req);
    if !valid_onvif_body(root, b"claim\0".as_ptr() as *const ::core::ffi::c_char) {
        cJSON_Delete(root);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut inventory_uuid: *const cJSON = cJSON_GetObjectItem(
        root,
        b"inventory_uuid\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut stream_name: *const cJSON = cJSON_GetObjectItem(
        root,
        b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsString(inventory_uuid) == 0
        || (*inventory_uuid).valuestring.is_null()
        || cJSON_IsString(stream_name) == 0
        || (*stream_name).valuestring.is_null()
    {
        cJSON_Delete(root);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"inventory_uuid and stream_name are required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut result: db_onvif_inventory_result_t =
        db_onvif_inventory_claim_stream((*inventory_uuid).valuestring, (*stream_name).valuestring);
    cJSON_Delete(root);
    if result.0 == db_onvif_inventory_result_t::DB_ONVIF_INVENTORY_INVALID.0 {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid claim request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if result.0 == db_onvif_inventory_result_t::DB_ONVIF_INVENTORY_NOT_FOUND.0 {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Inventory device or stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if result.0 != db_onvif_inventory_result_t::DB_ONVIF_INVENTORY_OK.0 {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to claim ONVIF device\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn handle_post_test_onvif_connection(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/onvif/device/test request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut root: *mut cJSON = device_json_body(req);
    if !valid_onvif_body(root, b"test\0".as_ptr() as *const ::core::ffi::c_char) {
        cJSON_Delete(root);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid JSON request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut url_json: *mut cJSON =
        cJSON_GetObjectItem(root, b"url\0".as_ptr() as *const ::core::ffi::c_char);
    let mut username_json: *mut cJSON =
        cJSON_GetObjectItem(root, b"username\0".as_ptr() as *const ::core::ffi::c_char);
    let mut password_json: *mut cJSON =
        cJSON_GetObjectItem(root, b"password\0".as_ptr() as *const ::core::ffi::c_char);
    if url_json.is_null() || cJSON_IsString(url_json) == 0 {
        cJSON_Delete(root);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Missing or invalid parameters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing or invalid parameters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut url: *const ::core::ffi::c_char = (*url_json).valuestring;
    let mut username: *const ::core::ffi::c_char =
        if !username_json.is_null() && cJSON_IsString(username_json) != 0 {
            (*username_json).valuestring
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
    let mut password: *const ::core::ffi::c_char =
        if !password_json.is_null() && cJSON_IsString(password_json) != 0 {
            (*password_json).valuestring
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
    if strlen(url) == 0 as size_t {
        cJSON_Delete(root);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid parameters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid parameters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut result: ::core::ffi::c_int = test_onvif_connection(url, username, password);
    cJSON_Delete(root);
    if result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to connect to ONVIF device\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to connect to ONVIF device\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response_json: *mut cJSON = cJSON_CreateObject();
    if response_json.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        response_json,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response_json,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"Successfully connected to ONVIF device\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response_json);
    cJSON_Delete(response_json);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to generate JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to generate JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled POST /api/onvif/device/test request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
pub const _log_stream_name: *const ::core::ffi::c_char = NULL_0 as *const ::core::ffi::c_char;
pub const ONVIF_DISCOVERY_ADDRESS_MAX: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
