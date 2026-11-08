unsafe extern "C" {
    pub type authorization_context;
    pub type stream_handle_s;
    unsafe fn playback_transport_is_valid(value: *const ::core::ffi::c_char) -> bool;
    static mut g_config: config_t;
    unsafe fn http_request_get_query_param(
        req: *const http_request_t,
        name: *const ::core::ffi::c_char,
        value: *mut ::core::ffi::c_char,
        value_len: size_t,
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
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_CreateBool(boolean: cJSON_bool) -> *mut cJSON;
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
    unsafe fn cJSON_AddArrayToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn fleet_availability_state_name(
        state: fleet_availability_state_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn authorization_context_create() -> *mut authorization_context_t;
    unsafe fn authorization_context_free(context: *mut authorization_context_t);
    unsafe fn authorization_evaluate_in_context(
        context: *mut authorization_context_t,
        user: *const user_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        evaluation: *mut authorization_evaluation_t,
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
    unsafe fn httpd_authorize_stream_action_with_context(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        stream_name: *const ::core::ffi::c_char,
        user: *mut user_t,
        camera: *mut fleet_camera_t,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn stream_observation_settings_to_json(
        object: *mut cJSON,
        config: *const stream_config_t,
    );
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn fleet_health_enrich_go2rtc_activity(
        cameras: *mut fleet_camera_t,
        camera_count: ::core::ffi::c_int,
    );
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
    unsafe fn url_extract_credentials(
        url: *const ::core::ffi::c_char,
        username: *mut ::core::ffi::c_char,
        username_size: size_t,
        password: *mut ::core::ffi::c_char,
        password_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_status(handle: stream_handle_t) -> stream_status_t;
    unsafe fn get_stream_config(
        handle: stream_handle_t,
        config: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_by_name(name: *const ::core::ffi::c_char) -> stream_handle_t;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn get_stream_state_by_name(
        name: *const ::core::ffi::c_char,
    ) -> *mut stream_state_manager_t;
    unsafe fn get_all_stream_configs_readonly(
        streams: *mut stream_config_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_load_readonly(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fleet_camera_enrich_runtime_health(
        cameras: *mut fleet_camera_t,
        count: ::core::ffi::c_int,
    );
    unsafe fn go2rtc_integration_is_using_go2rtc_for_hls(
        stream_name: *const ::core::ffi::c_char,
    ) -> bool;
    unsafe fn go2rtc_integration_is_initialized() -> bool;
    unsafe fn get_unified_detection_effective_status(
        stream_name: *const ::core::ffi::c_char,
    ) -> stream_status_t;
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
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
pub type authorization_context_t = authorization_context;
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
pub struct api_phase_timer_t {
    pub started: timespec,
    pub request: *const http_request_t,
    pub phase: *const ::core::ffi::c_char,
    pub valid: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_state_manager_t {
    pub name: [::core::ffi::c_char; 256],
    pub state: stream_state_t,
    pub features: stream_features_t,
    pub protocol_state: stream_protocol_state_t,
    pub timestamp_state: stream_timestamp_state_t,
    pub stats: stream_stats_t,
    pub config: stream_config_t,
    pub mutex: pthread_mutex_t,
    pub state_mutex: pthread_mutex_t,
    pub ref_count: ::core::ffi::c_int,
    pub component_refs: [::core::ffi::c_int; 6],
    pub reader_ctx: *mut ::core::ffi::c_void,
    pub hls_ctx: *mut ::core::ffi::c_void,
    pub mp4_ctx: *mut ::core::ffi::c_void,
    pub detection_ctx: *mut ::core::ffi::c_void,
    pub callbacks_enabled: bool,
    pub last_error_message: [::core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_stats_t {
    pub bytes_received: uint64_t,
    pub frames_received: uint64_t,
    pub frames_dropped: uint64_t,
    pub errors: uint64_t,
    pub reconnects: uint64_t,
    pub bitrate: ::core::ffi::c_double,
    pub fps: ::core::ffi::c_double,
    pub uptime: uint64_t,
    pub last_frame_time: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_timestamp_state_t {
    pub last_pts: int64_t,
    pub last_dts: int64_t,
    pub expected_next_pts: int64_t,
    pub pts_discontinuity_count: int64_t,
    pub timestamps_initialized: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_protocol_state_t {
    pub protocol: stream_protocol_t,
    pub is_multicast: bool,
    pub reconnect_attempts: ::core::ffi::c_int,
    pub last_reconnect_time: int64_t,
    pub buffer_size: ::core::ffi::c_int,
    pub timeout_ms: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_features_t {
    pub streaming_enabled: bool,
    pub recording_enabled: bool,
    pub detection_enabled: bool,
    pub motion_detection_enabled: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_state_t(pub ::core::ffi::c_uint);
impl stream_state_t {
    pub const STREAM_STATE_INACTIVE: Self = Self(0);
    pub const STREAM_STATE_STARTING: Self = Self(1);
    pub const STREAM_STATE_ACTIVE: Self = Self(2);
    pub const STREAM_STATE_STOPPING: Self = Self(3);
    pub const STREAM_STATE_ERROR: Self = Self(4);
    pub const STREAM_STATE_RECONNECTING: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_status_t(pub ::core::ffi::c_uint);
impl stream_status_t {
    pub const STREAM_STATUS_UNKNOWN: Self = Self(0);
    pub const STREAM_STATUS_STOPPED: Self = Self(1);
    pub const STREAM_STATUS_STARTING: Self = Self(2);
    pub const STREAM_STATUS_RUNNING: Self = Self(3);
    pub const STREAM_STATUS_STOPPING: Self = Self(4);
    pub const STREAM_STATUS_ERROR: Self = Self(5);
    pub const STREAM_STATUS_RECONNECTING: Self = Self(6);
}
pub type stream_handle_t = *mut stream_handle_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_summary_row_t {
    pub config: *const stream_config_t,
    pub camera: *const fleet_camera_t,
    pub can_configure: bool,
}
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOG_COMPONENT: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"StreamsAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
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
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn resolve_effective_stream_status(
    mut raw_status: stream_status_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut stream_enabled: bool,
) -> stream_status_t {
    if raw_status.0 == stream_status_t::STREAM_STATUS_STOPPED.0
        && stream_enabled as ::core::ffi::c_int != 0
        && go2rtc_integration_is_initialized() as ::core::ffi::c_int != 0
    {
        let mut udt_status: stream_status_t = get_unified_detection_effective_status(stream_name);
        if udt_status.0 != stream_status_t::STREAM_STATUS_STOPPED.0 {
            return udt_status;
        }
        return stream_status_t::STREAM_STATUS_STOPPED;
    }
    return raw_status;
}
unsafe extern "C" fn get_stream_api_credentials(
    mut config: *const stream_config_t,
    mut safe_url: *mut ::core::ffi::c_char,
    mut safe_url_size: size_t,
    mut onvif_username: *mut ::core::ffi::c_char,
    mut onvif_username_size: size_t,
    mut onvif_password: *mut ::core::ffi::c_char,
    mut onvif_password_size: size_t,
    mut expose_sensitive_config: bool,
) {
    let mut extracted_username: [::core::ffi::c_char; 128] = [
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
    let mut extracted_password: [::core::ffi::c_char; 128] = [
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
    let mut use_separate_credentials: bool = false;
    if config.is_null() {
        return;
    }
    safe_strcpy(
        safe_url,
        &raw const (*config).url as *const ::core::ffi::c_char,
        safe_url_size,
        0 as size_t,
    );
    safe_strcpy(
        onvif_username,
        &raw const (*config).onvif_username as *const ::core::ffi::c_char,
        onvif_username_size,
        0 as size_t,
    );
    safe_strcpy(
        onvif_password,
        &raw const (*config).onvif_password as *const ::core::ffi::c_char,
        onvif_password_size,
        0 as size_t,
    );
    use_separate_credentials = (*config).is_onvif as ::core::ffi::c_int != 0
        || (*config).onvif_username[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || (*config).onvif_password[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
    if use_separate_credentials as ::core::ffi::c_int != 0
        && (*onvif_username.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            || *onvif_password.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int)
        && url_extract_credentials(
            &raw const (*config).url as *const ::core::ffi::c_char,
            &raw mut extracted_username as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            &raw mut extracted_password as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        ) == 0 as ::core::ffi::c_int
    {
        if *onvif_username.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            && extracted_username[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            safe_strcpy(
                onvif_username,
                &raw mut extracted_username as *mut ::core::ffi::c_char,
                onvif_username_size,
                0 as size_t,
            );
        }
        if *onvif_password.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            && extracted_password[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            safe_strcpy(
                onvif_password,
                &raw mut extracted_password as *mut ::core::ffi::c_char,
                onvif_password_size,
                0 as size_t,
            );
        }
    }
    if use_separate_credentials as ::core::ffi::c_int != 0 || !expose_sensitive_config {
        if url_strip_credentials(
            &raw const (*config).url as *const ::core::ffi::c_char,
            safe_url,
            safe_url_size,
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                safe_url,
                if expose_sensitive_config as ::core::ffi::c_int != 0 {
                    &raw const (*config).url as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                safe_url_size,
                0 as size_t,
            );
        }
    }
    if !expose_sensitive_config {
        *onvif_username.offset(0isize) = '\0' as ::core::ffi::c_char;
        *onvif_password.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn stream_user_can_modify(
    mut user: *const user_t,
    mut camera: *const fleet_camera_t,
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
        && !camera.is_null()
        && authorization_evaluate(
            user,
            authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
            camera,
            &raw mut evaluation,
        ) == 0 as ::core::ffi::c_int
        && evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0;
}
unsafe extern "C" fn find_loaded_camera(
    mut cameras: *const fleet_camera_t,
    mut camera_count: ::core::ffi::c_int,
    mut config: *const stream_config_t,
) -> *const fleet_camera_t {
    if cameras.is_null() || config.is_null() {
        return ::core::ptr::null::<fleet_camera_t>();
    }
    if (*config).camera_uuid[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut low: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut high: ::core::ffi::c_int = camera_count - 1 as ::core::ffi::c_int;
        while low <= high {
            let mut middle: ::core::ffi::c_int = low + (high - low) / 2 as ::core::ffi::c_int;
            let mut comparison: ::core::ffi::c_int = strcmp(
                &raw const (*cameras.offset(middle as isize)).camera_uuid
                    as *const ::core::ffi::c_char,
                &raw const (*config).camera_uuid as *const ::core::ffi::c_char,
            );
            if comparison == 0 as ::core::ffi::c_int {
                return cameras.offset(middle as isize);
            }
            if comparison < 0 as ::core::ffi::c_int {
                low = middle + 1 as ::core::ffi::c_int;
            } else {
                high = middle - 1 as ::core::ffi::c_int;
            }
        }
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (*config).camera_uuid[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        && i < camera_count
    {
        if strcmp(
            &raw const (*cameras.offset(i as isize)).name as *const ::core::ffi::c_char,
            &raw const (*config).name as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return cameras.offset(i as isize);
        }
        i += 1;
    }
    return ::core::ptr::null::<fleet_camera_t>();
}
#[thread_local]
static mut stream_summary_sort: [::core::ffi::c_char; 24] = [0; 24];
#[thread_local]
static mut stream_summary_descending: bool = false;
unsafe extern "C" fn summary_status(
    mut camera: *const fleet_camera_t,
) -> *const ::core::ffi::c_char {
    if camera.is_null() || !(*camera).enabled {
        return b"Stopped\0".as_ptr() as *const ::core::ffi::c_char;
    }
    match (*camera).health {
        fleet_health_state_t::FLEET_HEALTH_UP => {
            return b"Running\0".as_ptr() as *const ::core::ffi::c_char;
        }
        fleet_health_state_t::FLEET_HEALTH_DEGRADED => {
            return b"Degraded\0".as_ptr() as *const ::core::ffi::c_char;
        }
        fleet_health_state_t::FLEET_HEALTH_DOWN => {
            return b"Error\0".as_ptr() as *const ::core::ffi::c_char;
        }
        fleet_health_state_t::FLEET_HEALTH_DISABLED => {
            return b"Stopped\0".as_ptr() as *const ::core::ffi::c_char;
        }
        fleet_health_state_t::FLEET_HEALTH_UNKNOWN | _ => {
            return b"Unknown\0".as_ptr() as *const ::core::ffi::c_char;
        }
    };
}
unsafe extern "C" fn compare_stream_summaries(
    mut left_ptr: *const ::core::ffi::c_void,
    mut right_ptr: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut left: *const stream_summary_row_t = left_ptr as *const stream_summary_row_t;
    let mut right: *const stream_summary_row_t = right_ptr as *const stream_summary_row_t;
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if strcmp(
        &raw mut stream_summary_sort as *mut ::core::ffi::c_char,
        b"status\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        result = strcasecmp(
            summary_status((*left).camera),
            summary_status((*right).camera),
        );
    } else if strcmp(
        &raw mut stream_summary_sort as *mut ::core::ffi::c_char,
        b"resolution\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut left_pixels: int64_t =
            (*(*left).config).width as int64_t * (*(*left).config).height as int64_t;
        let mut right_pixels: int64_t =
            (*(*right).config).width as int64_t * (*(*right).config).height as int64_t;
        result = if left_pixels < right_pixels {
            -1 as ::core::ffi::c_int
        } else {
            (left_pixels > right_pixels) as ::core::ffi::c_int
        };
    } else if strcmp(
        &raw mut stream_summary_sort as *mut ::core::ffi::c_char,
        b"fps\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        result = (*(*left).config).fps - (*(*right).config).fps;
    } else if strcmp(
        &raw mut stream_summary_sort as *mut ::core::ffi::c_char,
        b"recording\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        result = (*(*left).config).record as ::core::ffi::c_int
            - (*(*right).config).record as ::core::ffi::c_int;
    } else {
        result = strcasecmp(
            &raw const (*(*left).config).name as *const ::core::ffi::c_char,
            &raw const (*(*right).config).name as *const ::core::ffi::c_char,
        );
    }
    if result == 0 as ::core::ffi::c_int {
        result = strcasecmp(
            &raw const (*(*left).config).name as *const ::core::ffi::c_char,
            &raw const (*(*right).config).name as *const ::core::ffi::c_char,
        );
    }
    return if stream_summary_descending as ::core::ffi::c_int != 0 {
        -result
    } else {
        result
    };
}
unsafe extern "C" fn contains_case_insensitive(
    mut text: *const ::core::ffi::c_char,
    mut needle: *const ::core::ffi::c_char,
) -> bool {
    if needle.is_null()
        || *needle.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    if text.is_null() {
        return r#false != 0;
    }
    let mut length: size_t = strlen(needle);
    let mut cursor: *const ::core::ffi::c_char = text;
    while *cursor != 0 {
        if strncasecmp(cursor, needle, length) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn stream_summary_matches(
    mut config: *const stream_config_t,
    mut camera: *const fleet_camera_t,
    mut search: *const ::core::ffi::c_char,
) -> bool {
    return search.is_null()
        || *search.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || contains_case_insensitive(
            &raw const (*config).name as *const ::core::ffi::c_char,
            search,
        ) as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(
            &raw const (*config).camera_uuid as *const ::core::ffi::c_char,
            search,
        ) as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(
            &raw const (*config).tags as *const ::core::ffi::c_char,
            search,
        ) as ::core::ffi::c_int
            != 0
        || !camera.is_null()
            && (contains_case_insensitive(
                &raw const (*camera).location_path as *const ::core::ffi::c_char,
                search,
            ) as ::core::ffi::c_int
                != 0
                || contains_case_insensitive(
                    &raw const (*camera).address as *const ::core::ffi::c_char,
                    search,
                ) as ::core::ffi::c_int
                    != 0);
}
unsafe extern "C" fn parse_summary_positive_int(
    mut req: *const http_request_t,
    mut name: *const ::core::ffi::c_char,
    mut fallback: ::core::ffi::c_int,
    mut maximum: ::core::ffi::c_int,
    mut value: *mut ::core::ffi::c_int,
) -> bool {
    let mut text: [::core::ffi::c_char; 32] = [
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
    if http_request_get_query_param(
        req,
        name,
        &raw mut text as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    ) < 0 as ::core::ffi::c_int
    {
        *value = fallback;
        return r#true != 0;
    }
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parsed: ::core::ffi::c_long = strtol(
        &raw mut text as *mut ::core::ffi::c_char,
        &raw mut end,
        10 as ::core::ffi::c_int,
    );
    if text[0usize] == 0
        || end.is_null()
        || *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || parsed < 1 as ::core::ffi::c_long
        || parsed > maximum as ::core::ffi::c_long
    {
        return r#false != 0;
    }
    *value = parsed as ::core::ffi::c_int;
    return r#true != 0;
}
unsafe extern "C" fn handle_get_stream_summaries(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut user: *const user_t,
    mut authz_context: *mut authorization_context_t,
    mut db_streams: *mut stream_config_t,
    mut stream_count: ::core::ffi::c_int,
    mut fleet_cameras: *mut fleet_camera_t,
    mut fleet_camera_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut page: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut page_size: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    if !parse_summary_positive_int(
        req,
        b"page\0".as_ptr() as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
        1000000 as ::core::ffi::c_int,
        &raw mut page,
    ) || !parse_summary_positive_int(
        req,
        b"page_size\0".as_ptr() as *const ::core::ffi::c_char,
        50 as ::core::ffi::c_int,
        100 as ::core::ffi::c_int,
        &raw mut page_size,
    ) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"page and page_size must be positive; page_size max is 100\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut search: [::core::ffi::c_char; 256] = [
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
    let mut sort_by: [::core::ffi::c_char; 24] =
        ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(
            *b"name\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
    let mut sort_order: [::core::ffi::c_char; 8] =
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"asc\0\0\0\0\0");
    let mut surface: [::core::ffi::c_char; 16] = ::core::mem::transmute::<
        [u8; 16],
        [::core::ffi::c_char; 16],
    >(*b"admin\0\0\0\0\0\0\0\0\0\0\0");
    let mut availability: [::core::ffi::c_char; 24] =
        ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(
            *b"all\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
    let mut include_admin_url_param: [::core::ffi::c_char; 8] =
        [0 as ::core::ffi::c_char, 0, 0, 0, 0, 0, 0, 0];
    http_request_get_query_param(
        req,
        b"search\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut search as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    http_request_get_query_param(
        req,
        b"sort_by\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sort_by as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
    );
    http_request_get_query_param(
        req,
        b"sort_order\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sort_order as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
    );
    http_request_get_query_param(
        req,
        b"surface\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut surface as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
    );
    http_request_get_query_param(
        req,
        b"availability\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut availability as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
    );
    let mut include_admin_url: bool = http_request_get_query_param(
        req,
        b"include_admin_url\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut include_admin_url_param as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
    ) > 0 as ::core::ffi::c_int
        && (strcmp(
            &raw mut include_admin_url_param as *mut ::core::ffi::c_char,
            b"true\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut include_admin_url_param as *mut ::core::ffi::c_char,
                b"1\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int);
    if strcmp(
        &raw mut sort_by as *mut ::core::ffi::c_char,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut sort_by as *mut ::core::ffi::c_char,
            b"status\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut sort_by as *mut ::core::ffi::c_char,
            b"resolution\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut sort_by as *mut ::core::ffi::c_char,
            b"fps\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut sort_by as *mut ::core::ffi::c_char,
            b"recording\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream summary sort field\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw mut sort_order as *mut ::core::ffi::c_char,
        b"asc\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut sort_order as *mut ::core::ffi::c_char,
            b"desc\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"sort_order must be asc or desc\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut live_surface: bool = strcmp(
        &raw mut surface as *mut ::core::ffi::c_char,
        b"live\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    let mut lpr_surface: bool = strcmp(
        &raw mut surface as *mut ::core::ffi::c_char,
        b"lpr\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    if !live_surface
        && !lpr_surface
        && strcmp(
            &raw mut surface as *mut ::core::ffi::c_char,
            b"admin\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"surface must be admin, live or lpr\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw mut availability as *mut ::core::ffi::c_char,
        b"all\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut availability as *mut ::core::ffi::c_char,
            b"live\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut availability as *mut ::core::ffi::c_char,
            b"offline\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut availability as *mut ::core::ffi::c_char,
            b"never_connected\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut availability as *mut ::core::ffi::c_char,
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid availability value\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut rows: *mut stream_summary_row_t = calloc(
        stream_count as size_t,
        ::core::mem::size_of::<stream_summary_row_t>(),
    ) as *mut stream_summary_row_t;
    if rows.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut row_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < stream_count {
        let mut camera: *const fleet_camera_t = find_loaded_camera(
            fleet_cameras,
            fleet_camera_count,
            db_streams.offset(i as isize),
        );
        let mut visible: authorization_evaluation_t = authorization_evaluation_t {
            decision: authorization_decision_t(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        };
        let mut lpr_read: authorization_evaluation_t = authorization_evaluation_t {
            decision: authorization_decision_t(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        };
        let mut configure: authorization_evaluation_t = authorization_evaluation_t {
            decision: authorization_decision_t(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        };
        if camera.is_null()
            || authorization_evaluate_in_context(
                authz_context,
                user,
                authorization_action_t(if lpr_surface as ::core::ffi::c_int != 0 {
                    authorization_action_t::AUTHZ_LPR_SEARCH.0
                } else {
                    authorization_action_t::AUTHZ_LIVE_VIEW.0
                }),
                camera,
                &raw mut visible,
            ) != 0 as ::core::ffi::c_int
            || lpr_surface as ::core::ffi::c_int != 0
                && authorization_evaluate_in_context(
                    authz_context,
                    user,
                    authorization_action_t::AUTHZ_LPR_READ,
                    camera,
                    &raw mut lpr_read,
                ) != 0 as ::core::ffi::c_int
            || !lpr_surface
                && authorization_evaluate_in_context(
                    authz_context,
                    user,
                    authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
                    camera,
                    &raw mut configure,
                ) != 0 as ::core::ffi::c_int
        {
            free(rows as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        if !(visible.decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0
            || lpr_surface as ::core::ffi::c_int != 0
                && lpr_read.decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0
            || !stream_summary_matches(
                db_streams.offset(i as isize),
                camera,
                &raw mut search as *mut ::core::ffi::c_char,
            )
            || strcmp(
                &raw mut availability as *mut ::core::ffi::c_char,
                b"all\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
                && strcmp(
                    &raw mut availability as *mut ::core::ffi::c_char,
                    fleet_availability_state_name((*camera).availability),
                ) != 0 as ::core::ffi::c_int)
        {
            let c2rust_fresh0 = row_count;
            row_count += 1;
            *rows.offset(c2rust_fresh0 as isize) = stream_summary_row_t {
                config: db_streams.offset(i as isize),
                camera: camera,
                can_configure: configure.decision.0
                    == authorization_decision_t::AUTHZ_DECISION_ALLOW.0,
            };
        }
        i += 1;
    }
    safe_strcpy(
        &raw mut stream_summary_sort as *mut ::core::ffi::c_char,
        &raw mut sort_by as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        0 as size_t,
    );
    stream_summary_descending = strcmp(
        &raw mut sort_order as *mut ::core::ffi::c_char,
        b"desc\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    if row_count > 1 as ::core::ffi::c_int {
        qsort(
            rows as *mut ::core::ffi::c_void,
            row_count as size_t,
            ::core::mem::size_of::<stream_summary_row_t>(),
            Some(
                compare_stream_summaries
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = if !root.is_null() {
        cJSON_AddArrayToObject(root, b"streams\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if root.is_null() || items.is_null() {
        cJSON_Delete(root);
        free(rows as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create stream summaries\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut offset64: int64_t = (page - 1 as ::core::ffi::c_int) as int64_t * page_size as int64_t;
    let mut offset: ::core::ffi::c_int = if offset64 > row_count as int64_t {
        row_count
    } else {
        offset64 as ::core::ffi::c_int
    };
    let mut end: ::core::ffi::c_int = offset + page_size;
    if end > row_count {
        end = row_count;
    }
    let mut i_0: ::core::ffi::c_int = offset;
    while i_0 < end {
        let mut config: *const stream_config_t = (*rows.offset(i_0 as isize)).config;
        let mut camera_0: *const fleet_camera_t = (*rows.offset(i_0 as isize)).camera;
        let mut item: *mut cJSON = cJSON_CreateObject();
        if !item.is_null() {
            cJSON_AddStringToObject(
                item,
                b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*config).camera_uuid as *const ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                item,
                b"name\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*config).name as *const ::core::ffi::c_char,
            );
            if lpr_surface {
                cJSON_AddItemToArray(items, item);
            } else {
                cJSON_AddStringToObject(
                    item,
                    b"location_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*config).location_uuid as *const ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    item,
                    b"location_name\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*camera_0).location_name as *const ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    item,
                    b"location_path\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*camera_0).location_path as *const ::core::ffi::c_char,
                );
                cJSON_AddBoolToObject(
                    item,
                    b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
                    (*config).enabled as cJSON_bool,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"width\0".as_ptr() as *const ::core::ffi::c_char,
                    (*config).width as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"height\0".as_ptr() as *const ::core::ffi::c_char,
                    (*config).height as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"fps\0".as_ptr() as *const ::core::ffi::c_char,
                    (*config).fps as ::core::ffi::c_double,
                );
                cJSON_AddStringToObject(
                    item,
                    b"codec\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*config).codec as *const ::core::ffi::c_char,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"protocol\0".as_ptr() as *const ::core::ffi::c_char,
                    (*config).protocol.0 as ::core::ffi::c_int as ::core::ffi::c_double,
                );
                cJSON_AddBoolToObject(
                    item,
                    b"record\0".as_ptr() as *const ::core::ffi::c_char,
                    (*config).record as cJSON_bool,
                );
                cJSON_AddBoolToObject(
                    item,
                    b"record_on_schedule\0".as_ptr() as *const ::core::ffi::c_char,
                    (*config).record_on_schedule as cJSON_bool,
                );
                cJSON_AddBoolToObject(
                    item,
                    b"detection_based_recording\0".as_ptr() as *const ::core::ffi::c_char,
                    (*config).detection_based_recording as cJSON_bool,
                );
                cJSON_AddBoolToObject(
                    item,
                    b"isOnvif\0".as_ptr() as *const ::core::ffi::c_char,
                    (*config).is_onvif as cJSON_bool,
                );
                cJSON_AddStringToObject(
                    item,
                    b"tags\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*config).tags as *const ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    item,
                    b"status\0".as_ptr() as *const ::core::ffi::c_char,
                    summary_status(camera_0),
                );
                cJSON_AddStringToObject(
                    item,
                    b"availability\0".as_ptr() as *const ::core::ffi::c_char,
                    fleet_availability_state_name((*camera_0).availability),
                );
                cJSON_AddNumberToObject(
                    item,
                    b"first_video_at\0".as_ptr() as *const ::core::ffi::c_char,
                    (*camera_0).first_video_at as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"last_frame_ts\0".as_ptr() as *const ::core::ffi::c_char,
                    (*camera_0).last_frame_ts as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"current_fps\0".as_ptr() as *const ::core::ffi::c_char,
                    (*camera_0).current_fps,
                );
                cJSON_AddBoolToObject(
                    item,
                    b"recording_active\0".as_ptr() as *const ::core::ffi::c_char,
                    (*camera_0).recording_active as cJSON_bool,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"last_recording_at\0".as_ptr() as *const ::core::ffi::c_char,
                    (*camera_0).last_recording_at as ::core::ffi::c_double,
                );
                cJSON_AddBoolToObject(
                    item,
                    b"can_configure\0".as_ptr() as *const ::core::ffi::c_char,
                    (*rows.offset(i_0 as isize)).can_configure as cJSON_bool,
                );
                if !live_surface && include_admin_url as ::core::ffi::c_int != 0 {
                    cJSON_AddStringToObject(
                        item,
                        b"admin_url\0".as_ptr() as *const ::core::ffi::c_char,
                        if (*rows.offset(i_0 as isize)).can_configure as ::core::ffi::c_int != 0 {
                            &raw const (*config).admin_url as *const ::core::ffi::c_char
                        } else {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                }
                if live_surface {
                    cJSON_AddBoolToObject(
                        item,
                        b"streaming_enabled\0".as_ptr() as *const ::core::ffi::c_char,
                        (*config).streaming_enabled as cJSON_bool,
                    );
                    cJSON_AddStringToObject(
                        item,
                        b"playback_transport\0".as_ptr() as *const ::core::ffi::c_char,
                        if playback_transport_is_valid(
                            &raw const (*config).playback_transport as *const ::core::ffi::c_char,
                        ) as ::core::ffi::c_int
                            != 0
                        {
                            &raw const (*config).playback_transport as *const ::core::ffi::c_char
                        } else {
                            b"auto\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                    cJSON_AddStringToObject(
                        item,
                        b"eptz_config\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw const (*config).eptz_config as *const ::core::ffi::c_char,
                    );
                    cJSON_AddStringToObject(
                        item,
                        b"detection_model\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw const (*config).detection_model as *const ::core::ffi::c_char,
                    );
                    cJSON_AddBoolToObject(
                        item,
                        b"privacy_mode\0".as_ptr() as *const ::core::ffi::c_char,
                        (*config).privacy_mode as cJSON_bool,
                    );
                    cJSON_AddBoolToObject(
                        item,
                        b"can_control_privacy\0".as_ptr() as *const ::core::ffi::c_char,
                        (*rows.offset(i_0 as isize)).can_configure as cJSON_bool,
                    );
                    cJSON_AddBoolToObject(
                        item,
                        b"has_sub_stream\0".as_ptr() as *const ::core::ffi::c_char,
                        ((*config).sub_stream_url[0usize] as ::core::ffi::c_int
                            != '\0' as ::core::ffi::c_int)
                            as ::core::ffi::c_int,
                    );
                    cJSON_AddBoolToObject(
                        item,
                        b"ptz_enabled\0".as_ptr() as *const ::core::ffi::c_char,
                        (*config).ptz_enabled as cJSON_bool,
                    );
                    cJSON_AddBoolToObject(
                        item,
                        b"backchannel_enabled\0".as_ptr() as *const ::core::ffi::c_char,
                        (*config).backchannel_enabled as cJSON_bool,
                    );
                    cJSON_AddBoolToObject(
                        item,
                        b"go2rtc_hls_available\0".as_ptr() as *const ::core::ffi::c_char,
                        go2rtc_integration_is_using_go2rtc_for_hls(
                            &raw const (*config).name as *const ::core::ffi::c_char,
                        ) as cJSON_bool,
                    );
                }
                cJSON_AddItemToArray(items, item);
            }
        }
        i_0 += 1;
    }
    cJSON_AddNumberToObject(
        root,
        b"total\0".as_ptr() as *const ::core::ffi::c_char,
        row_count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"page\0".as_ptr() as *const ::core::ffi::c_char,
        page as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"page_size\0".as_ptr() as *const ::core::ffi::c_char,
        page_size as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"total_pages\0".as_ptr() as *const ::core::ffi::c_char,
        (if row_count == 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            (row_count + page_size - 1 as ::core::ffi::c_int) / page_size
        }) as ::core::ffi::c_double,
    );
    let mut encoded: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    if encoded.is_null() {
        cJSON_Delete(root);
        free(rows as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to encode stream summaries\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, encoded);
    free(encoded as *mut ::core::ffi::c_void);
    cJSON_Delete(root);
    free(rows as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_streams(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/streams request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut auth_user: user_t = user_t {
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
        &raw mut auth_user as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    let mut phase: api_phase_timer_t = api_phase_begin(
        req,
        b"authentication\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut authenticated: bool = httpd_check_action_access(req, &raw mut auth_user) != 0;
    api_phase_end(&raw mut phase);
    if !authenticated {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut authz_context: *mut authorization_context_t = authorization_context_create();
    if authz_context.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut db_streams: *mut stream_config_t = calloc(
        g_config.max_streams as size_t,
        ::core::mem::size_of::<stream_config_t>(),
    ) as *mut stream_config_t;
    if db_streams.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"handle_get_streams: out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Internal error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        authorization_context_free(authz_context);
        return;
    }
    phase = api_phase_begin(
        req,
        b"stream configuration database read\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut count: ::core::ffi::c_int =
        get_all_stream_configs_readonly(db_streams, g_config.max_streams);
    api_phase_end(&raw mut phase);
    if count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to get stream configurations from database\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(db_streams as *mut ::core::ffi::c_void);
        authorization_context_free(authz_context);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get stream configurations\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut fleet_cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut fleet_camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    phase = api_phase_begin(
        req,
        b"camera inventory database read\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut inventory_result: ::core::ffi::c_int =
        db_fleet_camera_load_readonly(&raw mut fleet_cameras, &raw mut fleet_camera_count);
    api_phase_end(&raw mut phase);
    if inventory_result != 0 as ::core::ffi::c_int {
        free(db_streams as *mut ::core::ffi::c_void);
        authorization_context_free(authz_context);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load camera inventory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    phase = api_phase_begin(
        req,
        b"runtime metrics snapshot\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fleet_camera_enrich_runtime_health(fleet_cameras, fleet_camera_count);
    api_phase_end(&raw mut phase);
    phase = api_phase_begin(
        req,
        b"go2rtc activity lookup\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fleet_health_enrich_go2rtc_activity(fleet_cameras, fleet_camera_count);
    api_phase_end(&raw mut phase);
    let mut summary_param: [::core::ffi::c_char; 8] =
        [0 as ::core::ffi::c_char, 0, 0, 0, 0, 0, 0, 0];
    let mut summary_request: bool = http_request_get_query_param(
        req,
        b"summary\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut summary_param as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
    ) > 0 as ::core::ffi::c_int
        && (strcmp(
            &raw mut summary_param as *mut ::core::ffi::c_char,
            b"true\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut summary_param as *mut ::core::ffi::c_char,
                b"1\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int);
    if summary_request {
        phase = api_phase_begin(
            req,
            b"summary authorization and serialization\0".as_ptr() as *const ::core::ffi::c_char,
        );
        handle_get_stream_summaries(
            req,
            res,
            &raw mut auth_user,
            authz_context,
            db_streams,
            count,
            fleet_cameras,
            fleet_camera_count,
        );
        api_phase_end(&raw mut phase);
        free(fleet_cameras as *mut ::core::ffi::c_void);
        free(db_streams as *mut ::core::ffi::c_void);
        authorization_context_free(authz_context);
        return;
    }
    let mut streams_array: *mut cJSON = cJSON_CreateArray();
    if streams_array.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create streams JSON array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(fleet_cameras as *mut ::core::ffi::c_void);
        free(db_streams as *mut ::core::ffi::c_void);
        authorization_context_free(authz_context);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create streams JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    phase = api_phase_begin(
        req,
        b"legacy stream authorization and details\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut camera: *const fleet_camera_t = find_loaded_camera(
            fleet_cameras,
            fleet_camera_count,
            db_streams.offset(i as isize),
        );
        let mut live_evaluation: authorization_evaluation_t = authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        };
        let mut configure_evaluation: authorization_evaluation_t = authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        };
        if camera.is_null()
            || authorization_evaluate_in_context(
                authz_context,
                &raw mut auth_user,
                authorization_action_t::AUTHZ_LIVE_VIEW,
                camera,
                &raw mut live_evaluation,
            ) != 0 as ::core::ffi::c_int
            || authorization_evaluate_in_context(
                authz_context,
                &raw mut auth_user,
                authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
                camera,
                &raw mut configure_evaluation,
            ) != 0 as ::core::ffi::c_int
        {
            cJSON_Delete(streams_array);
            free(fleet_cameras as *mut ::core::ffi::c_void);
            free(db_streams as *mut ::core::ffi::c_void);
            authorization_context_free(authz_context);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        if live_evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0 {
            let expose_sensitive_config: bool =
                configure_evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0;
            let mut stream_obj: *mut cJSON = cJSON_CreateObject();
            if stream_obj.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to create stream JSON object\0".as_ptr() as *const ::core::ffi::c_char,
                );
                cJSON_Delete(streams_array);
                free(fleet_cameras as *mut ::core::ffi::c_void);
                free(db_streams as *mut ::core::ffi::c_void);
                authorization_context_free(authz_context);
                http_response_set_json_error(
                    res,
                    500 as ::core::ffi::c_int,
                    b"Failed to create stream JSON\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            let mut safe_url: [::core::ffi::c_char; 512] = [0; 512];
            let mut api_onvif_username: [::core::ffi::c_char; 64] = [0; 64];
            let mut api_onvif_password: [::core::ffi::c_char; 64] = [0; 64];
            get_stream_api_credentials(
                db_streams.offset(i as isize),
                &raw mut safe_url as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                &raw mut api_onvif_username as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                &raw mut api_onvif_password as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                expose_sensitive_config,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*db_streams.offset(i as isize)).camera_uuid as *mut ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"location_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*db_streams.offset(i as isize)).location_uuid as *mut ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"name\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*db_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"url\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut safe_url as *mut ::core::ffi::c_char,
            );
            cJSON_AddBoolToObject(
                stream_obj,
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).enabled as cJSON_bool,
            );
            cJSON_AddBoolToObject(
                stream_obj,
                b"streaming_enabled\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).streaming_enabled as cJSON_bool,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"playback_transport\0".as_ptr() as *const ::core::ffi::c_char,
                if playback_transport_is_valid(
                    &raw mut (*db_streams.offset(i as isize)).playback_transport
                        as *mut ::core::ffi::c_char,
                ) as ::core::ffi::c_int
                    != 0
                {
                    &raw mut (*db_streams.offset(i as isize)).playback_transport
                        as *mut ::core::ffi::c_char
                } else {
                    b"auto\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"eptz_config\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*db_streams.offset(i as isize)).eptz_config as *mut ::core::ffi::c_char,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"width\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).width as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"height\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).height as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"fps\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).fps as ::core::ffi::c_double,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"codec\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*db_streams.offset(i as isize)).codec as *mut ::core::ffi::c_char,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"priority\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).priority as ::core::ffi::c_double,
            );
            cJSON_AddBoolToObject(
                stream_obj,
                b"record\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).record as cJSON_bool,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"segment_duration\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).segment_duration as ::core::ffi::c_double,
            );
            cJSON_AddBoolToObject(
                stream_obj,
                b"detection_based_recording\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).detection_based_recording as cJSON_bool,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"detection_model\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*db_streams.offset(i as isize)).detection_model
                    as *mut ::core::ffi::c_char,
            );
            let mut threshold_percent: ::core::ffi::c_int =
                ((*db_streams.offset(i as isize)).detection_threshold * 100.0f32)
                    as ::core::ffi::c_int;
            cJSON_AddNumberToObject(
                stream_obj,
                b"detection_threshold\0".as_ptr() as *const ::core::ffi::c_char,
                threshold_percent as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"detection_interval\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).detection_interval as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"pre_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).pre_detection_buffer as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"post_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).post_detection_buffer as ::core::ffi::c_double,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"detection_object_filter\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*db_streams.offset(i as isize)).detection_object_filter
                    as *mut ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"detection_object_filter_list\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*db_streams.offset(i as isize)).detection_object_filter_list
                    as *mut ::core::ffi::c_char,
            );
            stream_observation_settings_to_json(stream_obj, db_streams.offset(i as isize));
            cJSON_AddNumberToObject(
                stream_obj,
                b"protocol\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).protocol.0 as ::core::ffi::c_int
                    as ::core::ffi::c_double,
            );
            cJSON_AddBoolToObject(
                stream_obj,
                b"record_audio\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).record_audio as cJSON_bool,
            );
            cJSON_AddBoolToObject(
                stream_obj,
                b"audio_voice_enhancement\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).audio_voice_enhancement as cJSON_bool,
            );
            cJSON_AddBoolToObject(
                stream_obj,
                b"isOnvif\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).is_onvif as cJSON_bool,
            );
            cJSON_AddBoolToObject(
                stream_obj,
                b"backchannel_enabled\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).backchannel_enabled as cJSON_bool,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).retention_days as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"detection_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).detection_retention_days as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"max_storage_mb\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).max_storage_mb as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"tier_critical_multiplier\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).tier_critical_multiplier,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"tier_important_multiplier\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).tier_important_multiplier,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"tier_ephemeral_multiplier\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).tier_ephemeral_multiplier,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"storage_priority\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).storage_priority as ::core::ffi::c_double,
            );
            cJSON_AddBoolToObject(
                stream_obj,
                b"ptz_enabled\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).ptz_enabled as cJSON_bool,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"ptz_max_x\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).ptz_max_x as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"ptz_max_y\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).ptz_max_y as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"ptz_max_z\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).ptz_max_z as ::core::ffi::c_double,
            );
            cJSON_AddBoolToObject(
                stream_obj,
                b"ptz_has_home\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).ptz_has_home as cJSON_bool,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"onvif_username\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut api_onvif_username as *mut ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"onvif_password\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut api_onvif_password as *mut ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"onvif_profile\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*db_streams.offset(i as isize)).onvif_profile as *mut ::core::ffi::c_char,
            );
            cJSON_AddNumberToObject(
                stream_obj,
                b"onvif_port\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).onvif_port as ::core::ffi::c_double,
            );
            cJSON_AddBoolToObject(
                stream_obj,
                b"record_on_schedule\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).record_on_schedule as cJSON_bool,
            );
            let mut schedule_arr_i: *mut cJSON = cJSON_CreateArray();
            if !schedule_arr_i.is_null() {
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j < 168 as ::core::ffi::c_int {
                    cJSON_AddItemToArray(
                        schedule_arr_i,
                        cJSON_CreateBool(
                            ((*db_streams.offset(i as isize)).recording_schedule[j as usize]
                                as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int,
                        ),
                    );
                    j += 1;
                }
                cJSON_AddItemToObject(
                    stream_obj,
                    b"recording_schedule\0".as_ptr() as *const ::core::ffi::c_char,
                    schedule_arr_i,
                );
            }
            cJSON_AddBoolToObject(
                stream_obj,
                b"detection_record_on_schedule\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).detection_record_on_schedule as cJSON_bool,
            );
            let mut detection_schedule_arr_i: *mut cJSON = cJSON_CreateArray();
            if !detection_schedule_arr_i.is_null() {
                let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j_0 < 168 as ::core::ffi::c_int {
                    cJSON_AddItemToArray(
                        detection_schedule_arr_i,
                        cJSON_CreateBool(
                            ((*db_streams.offset(i as isize)).detection_recording_schedule
                                [j_0 as usize] as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int,
                        ),
                    );
                    j_0 += 1;
                }
                cJSON_AddItemToObject(
                    stream_obj,
                    b"detection_recording_schedule\0".as_ptr() as *const ::core::ffi::c_char,
                    detection_schedule_arr_i,
                );
            }
            cJSON_AddStringToObject(
                stream_obj,
                b"tags\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*db_streams.offset(i as isize)).tags as *mut ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"admin_url\0".as_ptr() as *const ::core::ffi::c_char,
                if expose_sensitive_config as ::core::ffi::c_int != 0 {
                    &raw mut (*db_streams.offset(i as isize)).admin_url as *mut ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            cJSON_AddBoolToObject(
                stream_obj,
                b"privacy_mode\0".as_ptr() as *const ::core::ffi::c_char,
                (*db_streams.offset(i as isize)).privacy_mode as cJSON_bool,
            );
            cJSON_AddBoolToObject(
                stream_obj,
                b"can_control_privacy\0".as_ptr() as *const ::core::ffi::c_char,
                expose_sensitive_config as cJSON_bool,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"motion_trigger_source\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*db_streams.offset(i as isize)).motion_trigger_source
                    as *mut ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"go2rtc_source_override\0".as_ptr() as *const ::core::ffi::c_char,
                if expose_sensitive_config as ::core::ffi::c_int != 0 {
                    &raw mut (*db_streams.offset(i as isize)).go2rtc_source_override
                        as *mut ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"sub_stream_url\0".as_ptr() as *const ::core::ffi::c_char,
                if expose_sensitive_config as ::core::ffi::c_int != 0 {
                    &raw mut (*db_streams.offset(i as isize)).sub_stream_url
                        as *mut ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            cJSON_AddBoolToObject(
                stream_obj,
                b"has_sub_stream\0".as_ptr() as *const ::core::ffi::c_char,
                ((*db_streams.offset(i as isize)).sub_stream_url[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int) as ::core::ffi::c_int,
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"detection_url\0".as_ptr() as *const ::core::ffi::c_char,
                if expose_sensitive_config as ::core::ffi::c_int != 0 {
                    &raw mut (*db_streams.offset(i as isize)).detection_url
                        as *mut ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            cJSON_AddStringToObject(
                stream_obj,
                b"publish_url\0".as_ptr() as *const ::core::ffi::c_char,
                if expose_sensitive_config as ::core::ffi::c_int != 0 {
                    &raw mut (*db_streams.offset(i as isize)).publish_url
                        as *mut ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            let mut status: *const ::core::ffi::c_char = summary_status(camera);
            if (*camera).health.0 == fleet_health_state_t::FLEET_HEALTH_UNKNOWN.0
                && (*db_streams.offset(i as isize)).enabled as ::core::ffi::c_int != 0
            {
                let mut stream: stream_handle_t = get_stream_by_name(
                    &raw mut (*db_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                );
                status = b"Unknown\0".as_ptr() as *const ::core::ffi::c_char;
                if !stream.is_null() {
                    let mut stream_status: stream_status_t = get_stream_status(stream);
                    stream_status = resolve_effective_stream_status(
                        stream_status,
                        &raw mut (*db_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                        (*db_streams.offset(i as isize)).enabled,
                    );
                    match stream_status {
                        stream_status_t::STREAM_STATUS_STOPPED => {
                            status = b"Stopped\0".as_ptr() as *const ::core::ffi::c_char;
                        }
                        stream_status_t::STREAM_STATUS_STARTING => {
                            status = b"Starting\0".as_ptr() as *const ::core::ffi::c_char;
                        }
                        stream_status_t::STREAM_STATUS_RUNNING => {
                            status = b"Running\0".as_ptr() as *const ::core::ffi::c_char;
                        }
                        stream_status_t::STREAM_STATUS_STOPPING => {
                            status = b"Stopping\0".as_ptr() as *const ::core::ffi::c_char;
                        }
                        stream_status_t::STREAM_STATUS_ERROR => {
                            status = b"Error\0".as_ptr() as *const ::core::ffi::c_char;
                        }
                        stream_status_t::STREAM_STATUS_RECONNECTING => {
                            status = b"Reconnecting\0".as_ptr() as *const ::core::ffi::c_char;
                        }
                        _ => {
                            status = b"Unknown\0".as_ptr() as *const ::core::ffi::c_char;
                        }
                    }
                }
            }
            cJSON_AddStringToObject(
                stream_obj,
                b"status\0".as_ptr() as *const ::core::ffi::c_char,
                status,
            );
            let mut sm_err: *mut stream_state_manager_t = get_stream_state_by_name(
                &raw mut (*db_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
            );
            if !sm_err.is_null()
                && strcmp(status, b"Error\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
            {
                let mut err_msg: [::core::ffi::c_char; 256] = [0; 256];
                pthread_mutex_lock(&raw mut (*sm_err).mutex);
                safe_strcpy(
                    &raw mut err_msg as *mut ::core::ffi::c_char,
                    &raw mut (*sm_err).last_error_message as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    0 as size_t,
                );
                pthread_mutex_unlock(&raw mut (*sm_err).mutex);
                if err_msg[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                    cJSON_AddStringToObject(
                        stream_obj,
                        b"error_message\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut err_msg as *mut ::core::ffi::c_char,
                    );
                }
            }
            cJSON_AddBoolToObject(
                stream_obj,
                b"go2rtc_hls_available\0".as_ptr() as *const ::core::ffi::c_char,
                go2rtc_integration_is_using_go2rtc_for_hls(
                    &raw mut (*db_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                ) as cJSON_bool,
            );
            cJSON_AddItemToArray(streams_array, stream_obj);
        }
        i += 1;
    }
    api_phase_end(&raw mut phase);
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(streams_array);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert streams JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(streams_array);
        free(fleet_cameras as *mut ::core::ffi::c_void);
        free(db_streams as *mut ::core::ffi::c_void);
        authorization_context_free(authz_context);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert streams JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    free(fleet_cameras as *mut ::core::ffi::c_void);
    free(db_streams as *mut ::core::ffi::c_void);
    authorization_context_free(authz_context);
    cJSON_Delete(streams_array);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/streams request\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_stream(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_id: [::core::ffi::c_char; 256] = [0; 256];
    if http_request_extract_path_param(
        req,
        b"/api/streams/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to extract stream ID from URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid request path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/streams/%s request\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_id as *mut ::core::ffi::c_char,
    );
    let mut stream: stream_handle_t =
        get_stream_by_name(&raw mut stream_id as *mut ::core::ffi::c_char);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Stream not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_id as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut config: stream_config_t = stream_config_t {
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
    if get_stream_config(stream, &raw mut config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to get stream configuration for: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_id as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get stream configuration\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut auth_user: user_t = user_t {
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
    let mut live_evaluation: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t::AUTHZ_DECISION_DENY,
        source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
        policy_version: 0,
        grant_uuid: [0; 37],
        token_uuid: [0; 37],
        role_uuid: [0; 37],
        role_name: [0; 128],
        explanation: [0; 256],
    };
    if httpd_authorize_stream_action_with_context(
        req,
        res,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut stream_id as *mut ::core::ffi::c_char,
        &raw mut auth_user,
        &raw mut camera,
        &raw mut live_evaluation,
    ) == 0
    {
        return;
    }
    fleet_camera_enrich_runtime_health(&raw mut camera, 1 as ::core::ffi::c_int);
    let expose_sensitive_config: bool = stream_user_can_modify(&raw mut auth_user, &raw mut camera);
    let mut stream_obj: *mut cJSON = cJSON_CreateObject();
    if stream_obj.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create stream JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create stream JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut safe_url: [::core::ffi::c_char; 512] = [0; 512];
    let mut api_onvif_username: [::core::ffi::c_char; 64] = [0; 64];
    let mut api_onvif_password: [::core::ffi::c_char; 64] = [0; 64];
    get_stream_api_credentials(
        &raw mut config,
        &raw mut safe_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        &raw mut api_onvif_username as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        &raw mut api_onvif_password as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        expose_sensitive_config,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.camera_uuid as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"location_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.location_uuid as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.name as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"url\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut safe_url as *mut ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        config.enabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"streaming_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        config.streaming_enabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"playback_transport\0".as_ptr() as *const ::core::ffi::c_char,
        if playback_transport_is_valid(
            &raw mut config.playback_transport as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        {
            &raw mut config.playback_transport as *mut ::core::ffi::c_char
        } else {
            b"auto\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"eptz_config\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.eptz_config as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"width\0".as_ptr() as *const ::core::ffi::c_char,
        config.width as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"height\0".as_ptr() as *const ::core::ffi::c_char,
        config.height as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"fps\0".as_ptr() as *const ::core::ffi::c_char,
        config.fps as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"codec\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.codec as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"priority\0".as_ptr() as *const ::core::ffi::c_char,
        config.priority as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"record\0".as_ptr() as *const ::core::ffi::c_char,
        config.record as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"segment_duration\0".as_ptr() as *const ::core::ffi::c_char,
        config.segment_duration as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"detection_based_recording\0".as_ptr() as *const ::core::ffi::c_char,
        config.detection_based_recording as cJSON_bool,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"detection_model\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.detection_model as *mut ::core::ffi::c_char,
    );
    let mut threshold_percent: ::core::ffi::c_int =
        (config.detection_threshold * 100.0f32) as ::core::ffi::c_int;
    cJSON_AddNumberToObject(
        stream_obj,
        b"detection_threshold\0".as_ptr() as *const ::core::ffi::c_char,
        threshold_percent as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"detection_interval\0".as_ptr() as *const ::core::ffi::c_char,
        config.detection_interval as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"pre_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
        config.pre_detection_buffer as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"post_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
        config.post_detection_buffer as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"detection_object_filter\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.detection_object_filter as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"detection_object_filter_list\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.detection_object_filter_list as *mut ::core::ffi::c_char,
    );
    stream_observation_settings_to_json(stream_obj, &raw mut config);
    cJSON_AddNumberToObject(
        stream_obj,
        b"protocol\0".as_ptr() as *const ::core::ffi::c_char,
        config.protocol.0 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"record_audio\0".as_ptr() as *const ::core::ffi::c_char,
        config.record_audio as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"audio_voice_enhancement\0".as_ptr() as *const ::core::ffi::c_char,
        config.audio_voice_enhancement as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"isOnvif\0".as_ptr() as *const ::core::ffi::c_char,
        config.is_onvif as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"backchannel_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        config.backchannel_enabled as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        config.retention_days as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"detection_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        config.detection_retention_days as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"max_storage_mb\0".as_ptr() as *const ::core::ffi::c_char,
        config.max_storage_mb as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"tier_critical_multiplier\0".as_ptr() as *const ::core::ffi::c_char,
        config.tier_critical_multiplier,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"tier_important_multiplier\0".as_ptr() as *const ::core::ffi::c_char,
        config.tier_important_multiplier,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"tier_ephemeral_multiplier\0".as_ptr() as *const ::core::ffi::c_char,
        config.tier_ephemeral_multiplier,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"storage_priority\0".as_ptr() as *const ::core::ffi::c_char,
        config.storage_priority as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"ptz_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        config.ptz_enabled as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"ptz_max_x\0".as_ptr() as *const ::core::ffi::c_char,
        config.ptz_max_x as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"ptz_max_y\0".as_ptr() as *const ::core::ffi::c_char,
        config.ptz_max_y as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"ptz_max_z\0".as_ptr() as *const ::core::ffi::c_char,
        config.ptz_max_z as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"ptz_has_home\0".as_ptr() as *const ::core::ffi::c_char,
        config.ptz_has_home as cJSON_bool,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"onvif_username\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut api_onvif_username as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"onvif_password\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut api_onvif_password as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"onvif_profile\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.onvif_profile as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"onvif_port\0".as_ptr() as *const ::core::ffi::c_char,
        config.onvif_port as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"record_on_schedule\0".as_ptr() as *const ::core::ffi::c_char,
        config.record_on_schedule as cJSON_bool,
    );
    let mut schedule_arr_get: *mut cJSON = cJSON_CreateArray();
    if !schedule_arr_get.is_null() {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < 168 as ::core::ffi::c_int {
            cJSON_AddItemToArray(
                schedule_arr_get,
                cJSON_CreateBool(
                    (config.recording_schedule[j as usize] as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                ),
            );
            j += 1;
        }
        cJSON_AddItemToObject(
            stream_obj,
            b"recording_schedule\0".as_ptr() as *const ::core::ffi::c_char,
            schedule_arr_get,
        );
    }
    cJSON_AddBoolToObject(
        stream_obj,
        b"detection_record_on_schedule\0".as_ptr() as *const ::core::ffi::c_char,
        config.detection_record_on_schedule as cJSON_bool,
    );
    let mut detection_schedule_arr_get: *mut cJSON = cJSON_CreateArray();
    if !detection_schedule_arr_get.is_null() {
        let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j_0 < 168 as ::core::ffi::c_int {
            cJSON_AddItemToArray(
                detection_schedule_arr_get,
                cJSON_CreateBool(
                    (config.detection_recording_schedule[j_0 as usize] as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                ),
            );
            j_0 += 1;
        }
        cJSON_AddItemToObject(
            stream_obj,
            b"detection_recording_schedule\0".as_ptr() as *const ::core::ffi::c_char,
            detection_schedule_arr_get,
        );
    }
    cJSON_AddStringToObject(
        stream_obj,
        b"tags\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.tags as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"admin_url\0".as_ptr() as *const ::core::ffi::c_char,
        if expose_sensitive_config as ::core::ffi::c_int != 0 {
            &raw mut config.admin_url as *mut ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"privacy_mode\0".as_ptr() as *const ::core::ffi::c_char,
        config.privacy_mode as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"can_control_privacy\0".as_ptr() as *const ::core::ffi::c_char,
        expose_sensitive_config as cJSON_bool,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"motion_trigger_source\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.motion_trigger_source as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"go2rtc_source_override\0".as_ptr() as *const ::core::ffi::c_char,
        if expose_sensitive_config as ::core::ffi::c_int != 0 {
            &raw mut config.go2rtc_source_override as *mut ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"sub_stream_url\0".as_ptr() as *const ::core::ffi::c_char,
        if expose_sensitive_config as ::core::ffi::c_int != 0 {
            &raw mut config.sub_stream_url as *mut ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"has_sub_stream\0".as_ptr() as *const ::core::ffi::c_char,
        (config.sub_stream_url[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int)
            as ::core::ffi::c_int,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"detection_url\0".as_ptr() as *const ::core::ffi::c_char,
        if expose_sensitive_config as ::core::ffi::c_int != 0 {
            &raw mut config.detection_url as *mut ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"publish_url\0".as_ptr() as *const ::core::ffi::c_char,
        if expose_sensitive_config as ::core::ffi::c_int != 0 {
            &raw mut config.publish_url as *mut ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut status: *const ::core::ffi::c_char = summary_status(&raw mut camera);
    if camera.health.0 == fleet_health_state_t::FLEET_HEALTH_UNKNOWN.0
        && config.enabled as ::core::ffi::c_int != 0
    {
        let mut stream_status: stream_status_t = get_stream_status(stream);
        stream_status = resolve_effective_stream_status(
            stream_status,
            &raw mut config.name as *mut ::core::ffi::c_char,
            config.enabled,
        );
        match stream_status {
            stream_status_t::STREAM_STATUS_STOPPED => {
                status = b"Stopped\0".as_ptr() as *const ::core::ffi::c_char;
            }
            stream_status_t::STREAM_STATUS_STARTING => {
                status = b"Starting\0".as_ptr() as *const ::core::ffi::c_char;
            }
            stream_status_t::STREAM_STATUS_RUNNING => {
                status = b"Running\0".as_ptr() as *const ::core::ffi::c_char;
            }
            stream_status_t::STREAM_STATUS_STOPPING => {
                status = b"Stopping\0".as_ptr() as *const ::core::ffi::c_char;
            }
            stream_status_t::STREAM_STATUS_ERROR => {
                status = b"Error\0".as_ptr() as *const ::core::ffi::c_char;
            }
            stream_status_t::STREAM_STATUS_RECONNECTING => {
                status = b"Reconnecting\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {
                status = b"Unknown\0".as_ptr() as *const ::core::ffi::c_char;
            }
        }
    }
    cJSON_AddStringToObject(
        stream_obj,
        b"status\0".as_ptr() as *const ::core::ffi::c_char,
        status,
    );
    let mut sm_err: *mut stream_state_manager_t =
        get_stream_state_by_name(&raw mut config.name as *mut ::core::ffi::c_char);
    if !sm_err.is_null()
        && strcmp(status, b"Error\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        let mut err_msg: [::core::ffi::c_char; 256] = [0; 256];
        pthread_mutex_lock(&raw mut (*sm_err).mutex);
        safe_strcpy(
            &raw mut err_msg as *mut ::core::ffi::c_char,
            &raw mut (*sm_err).last_error_message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        pthread_mutex_unlock(&raw mut (*sm_err).mutex);
        if err_msg[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            cJSON_AddStringToObject(
                stream_obj,
                b"error_message\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut err_msg as *mut ::core::ffi::c_char,
            );
        }
    }
    cJSON_AddBoolToObject(
        stream_obj,
        b"go2rtc_hls_available\0".as_ptr() as *const ::core::ffi::c_char,
        go2rtc_integration_is_using_go2rtc_for_hls(&raw mut config.name as *mut ::core::ffi::c_char)
            as cJSON_bool,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(stream_obj);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert stream JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(stream_obj);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert stream JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(stream_obj);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/streams/%s request\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut stream_id as *mut ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_stream_full(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_id: [::core::ffi::c_char; 256] = [0; 256];
    if http_request_extract_path_param(
        req,
        b"/api/streams/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to extract stream ID from URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid request path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut slash: *mut ::core::ffi::c_char = strchr(
        &raw mut stream_id as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if !slash.is_null() {
        *slash = '\0' as ::core::ffi::c_char;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/streams/%s/full request\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_id as *mut ::core::ffi::c_char,
    );
    let mut stream: stream_handle_t =
        get_stream_by_name(&raw mut stream_id as *mut ::core::ffi::c_char);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Stream not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_id as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut config: stream_config_t = stream_config_t {
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
    if get_stream_config(stream, &raw mut config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to get stream configuration for: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_id as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get stream configuration\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut auth_user: user_t = user_t {
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
    let mut live_evaluation: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t::AUTHZ_DECISION_DENY,
        source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
        policy_version: 0,
        grant_uuid: [0; 37],
        token_uuid: [0; 37],
        role_uuid: [0; 37],
        role_name: [0; 128],
        explanation: [0; 256],
    };
    if httpd_authorize_stream_action_with_context(
        req,
        res,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut stream_id as *mut ::core::ffi::c_char,
        &raw mut auth_user,
        &raw mut camera,
        &raw mut live_evaluation,
    ) == 0
    {
        return;
    }
    fleet_camera_enrich_runtime_health(&raw mut camera, 1 as ::core::ffi::c_int);
    let expose_sensitive_config: bool = stream_user_can_modify(&raw mut auth_user, &raw mut camera);
    let mut stream_obj: *mut cJSON = cJSON_CreateObject();
    if stream_obj.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create stream JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create stream JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut safe_url_full: [::core::ffi::c_char; 512] = [0; 512];
    let mut api_onvif_username_full: [::core::ffi::c_char; 64] = [0; 64];
    let mut api_onvif_password_full: [::core::ffi::c_char; 64] = [0; 64];
    get_stream_api_credentials(
        &raw mut config,
        &raw mut safe_url_full as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        &raw mut api_onvif_username_full as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        &raw mut api_onvif_password_full as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        expose_sensitive_config,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.camera_uuid as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"location_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.location_uuid as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.name as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"url\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut safe_url_full as *mut ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        config.enabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"streaming_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        config.streaming_enabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"playback_transport\0".as_ptr() as *const ::core::ffi::c_char,
        if playback_transport_is_valid(
            &raw mut config.playback_transport as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        {
            &raw mut config.playback_transport as *mut ::core::ffi::c_char
        } else {
            b"auto\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"eptz_config\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.eptz_config as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"width\0".as_ptr() as *const ::core::ffi::c_char,
        config.width as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"height\0".as_ptr() as *const ::core::ffi::c_char,
        config.height as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"fps\0".as_ptr() as *const ::core::ffi::c_char,
        config.fps as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"codec\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.codec as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"priority\0".as_ptr() as *const ::core::ffi::c_char,
        config.priority as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"record\0".as_ptr() as *const ::core::ffi::c_char,
        config.record as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"segment_duration\0".as_ptr() as *const ::core::ffi::c_char,
        config.segment_duration as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"detection_based_recording\0".as_ptr() as *const ::core::ffi::c_char,
        config.detection_based_recording as cJSON_bool,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"detection_model\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.detection_model as *mut ::core::ffi::c_char,
    );
    let mut threshold_percent: ::core::ffi::c_int =
        (config.detection_threshold * 100.0f32) as ::core::ffi::c_int;
    cJSON_AddNumberToObject(
        stream_obj,
        b"detection_threshold\0".as_ptr() as *const ::core::ffi::c_char,
        threshold_percent as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"detection_interval\0".as_ptr() as *const ::core::ffi::c_char,
        config.detection_interval as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"pre_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
        config.pre_detection_buffer as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"post_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
        config.post_detection_buffer as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"detection_object_filter\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.detection_object_filter as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"detection_object_filter_list\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.detection_object_filter_list as *mut ::core::ffi::c_char,
    );
    stream_observation_settings_to_json(stream_obj, &raw mut config);
    cJSON_AddNumberToObject(
        stream_obj,
        b"protocol\0".as_ptr() as *const ::core::ffi::c_char,
        config.protocol.0 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"record_audio\0".as_ptr() as *const ::core::ffi::c_char,
        config.record_audio as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"audio_voice_enhancement\0".as_ptr() as *const ::core::ffi::c_char,
        config.audio_voice_enhancement as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"isOnvif\0".as_ptr() as *const ::core::ffi::c_char,
        config.is_onvif as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"backchannel_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        config.backchannel_enabled as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        config.retention_days as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"detection_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        config.detection_retention_days as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"max_storage_mb\0".as_ptr() as *const ::core::ffi::c_char,
        config.max_storage_mb as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"tier_critical_multiplier\0".as_ptr() as *const ::core::ffi::c_char,
        config.tier_critical_multiplier,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"tier_important_multiplier\0".as_ptr() as *const ::core::ffi::c_char,
        config.tier_important_multiplier,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"tier_ephemeral_multiplier\0".as_ptr() as *const ::core::ffi::c_char,
        config.tier_ephemeral_multiplier,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"storage_priority\0".as_ptr() as *const ::core::ffi::c_char,
        config.storage_priority as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"ptz_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        config.ptz_enabled as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"ptz_max_x\0".as_ptr() as *const ::core::ffi::c_char,
        config.ptz_max_x as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"ptz_max_y\0".as_ptr() as *const ::core::ffi::c_char,
        config.ptz_max_y as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"ptz_max_z\0".as_ptr() as *const ::core::ffi::c_char,
        config.ptz_max_z as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"ptz_has_home\0".as_ptr() as *const ::core::ffi::c_char,
        config.ptz_has_home as cJSON_bool,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"onvif_username\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut api_onvif_username_full as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"onvif_password\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut api_onvif_password_full as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"onvif_profile\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.onvif_profile as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        stream_obj,
        b"onvif_port\0".as_ptr() as *const ::core::ffi::c_char,
        config.onvif_port as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"record_on_schedule\0".as_ptr() as *const ::core::ffi::c_char,
        config.record_on_schedule as cJSON_bool,
    );
    let mut schedule_arr_full: *mut cJSON = cJSON_CreateArray();
    if !schedule_arr_full.is_null() {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < 168 as ::core::ffi::c_int {
            cJSON_AddItemToArray(
                schedule_arr_full,
                cJSON_CreateBool(
                    (config.recording_schedule[j as usize] as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                ),
            );
            j += 1;
        }
        cJSON_AddItemToObject(
            stream_obj,
            b"recording_schedule\0".as_ptr() as *const ::core::ffi::c_char,
            schedule_arr_full,
        );
    }
    cJSON_AddBoolToObject(
        stream_obj,
        b"detection_record_on_schedule\0".as_ptr() as *const ::core::ffi::c_char,
        config.detection_record_on_schedule as cJSON_bool,
    );
    let mut detection_schedule_arr_full: *mut cJSON = cJSON_CreateArray();
    if !detection_schedule_arr_full.is_null() {
        let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j_0 < 168 as ::core::ffi::c_int {
            cJSON_AddItemToArray(
                detection_schedule_arr_full,
                cJSON_CreateBool(
                    (config.detection_recording_schedule[j_0 as usize] as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                ),
            );
            j_0 += 1;
        }
        cJSON_AddItemToObject(
            stream_obj,
            b"detection_recording_schedule\0".as_ptr() as *const ::core::ffi::c_char,
            detection_schedule_arr_full,
        );
    }
    cJSON_AddStringToObject(
        stream_obj,
        b"tags\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.tags as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"admin_url\0".as_ptr() as *const ::core::ffi::c_char,
        if expose_sensitive_config as ::core::ffi::c_int != 0 {
            &raw mut config.admin_url as *mut ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"privacy_mode\0".as_ptr() as *const ::core::ffi::c_char,
        config.privacy_mode as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"can_control_privacy\0".as_ptr() as *const ::core::ffi::c_char,
        expose_sensitive_config as cJSON_bool,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"motion_trigger_source\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.motion_trigger_source as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"go2rtc_source_override\0".as_ptr() as *const ::core::ffi::c_char,
        if expose_sensitive_config as ::core::ffi::c_int != 0 {
            &raw mut config.go2rtc_source_override as *mut ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"sub_stream_url\0".as_ptr() as *const ::core::ffi::c_char,
        if expose_sensitive_config as ::core::ffi::c_int != 0 {
            &raw mut config.sub_stream_url as *mut ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddBoolToObject(
        stream_obj,
        b"has_sub_stream\0".as_ptr() as *const ::core::ffi::c_char,
        (config.sub_stream_url[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int)
            as ::core::ffi::c_int,
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"detection_url\0".as_ptr() as *const ::core::ffi::c_char,
        if expose_sensitive_config as ::core::ffi::c_int != 0 {
            &raw mut config.detection_url as *mut ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddStringToObject(
        stream_obj,
        b"publish_url\0".as_ptr() as *const ::core::ffi::c_char,
        if expose_sensitive_config as ::core::ffi::c_int != 0 {
            &raw mut config.publish_url as *mut ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut status: *const ::core::ffi::c_char = summary_status(&raw mut camera);
    if camera.health.0 == fleet_health_state_t::FLEET_HEALTH_UNKNOWN.0
        && config.enabled as ::core::ffi::c_int != 0
    {
        let mut stream_status: stream_status_t = get_stream_status(stream);
        stream_status = resolve_effective_stream_status(
            stream_status,
            &raw mut config.name as *mut ::core::ffi::c_char,
            config.enabled,
        );
        match stream_status {
            stream_status_t::STREAM_STATUS_STOPPED => {
                status = b"Stopped\0".as_ptr() as *const ::core::ffi::c_char;
            }
            stream_status_t::STREAM_STATUS_STARTING => {
                status = b"Starting\0".as_ptr() as *const ::core::ffi::c_char;
            }
            stream_status_t::STREAM_STATUS_RUNNING => {
                status = b"Running\0".as_ptr() as *const ::core::ffi::c_char;
            }
            stream_status_t::STREAM_STATUS_STOPPING => {
                status = b"Stopping\0".as_ptr() as *const ::core::ffi::c_char;
            }
            stream_status_t::STREAM_STATUS_ERROR => {
                status = b"Error\0".as_ptr() as *const ::core::ffi::c_char;
            }
            stream_status_t::STREAM_STATUS_RECONNECTING => {
                status = b"Reconnecting\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {
                status = b"Unknown\0".as_ptr() as *const ::core::ffi::c_char;
            }
        }
    }
    cJSON_AddStringToObject(
        stream_obj,
        b"status\0".as_ptr() as *const ::core::ffi::c_char,
        status,
    );
    let mut sm_err2: *mut stream_state_manager_t =
        get_stream_state_by_name(&raw mut config.name as *mut ::core::ffi::c_char);
    if !sm_err2.is_null()
        && strcmp(status, b"Error\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        let mut err_msg: [::core::ffi::c_char; 256] = [0; 256];
        pthread_mutex_lock(&raw mut (*sm_err2).mutex);
        safe_strcpy(
            &raw mut err_msg as *mut ::core::ffi::c_char,
            &raw mut (*sm_err2).last_error_message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        pthread_mutex_unlock(&raw mut (*sm_err2).mutex);
        if err_msg[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            cJSON_AddStringToObject(
                stream_obj,
                b"error_message\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut err_msg as *mut ::core::ffi::c_char,
            );
        }
    }
    cJSON_AddBoolToObject(
        stream_obj,
        b"go2rtc_hls_available\0".as_ptr() as *const ::core::ffi::c_char,
        go2rtc_integration_is_using_go2rtc_for_hls(&raw mut config.name as *mut ::core::ffi::c_char)
            as cJSON_bool,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    if response.is_null() {
        cJSON_Delete(stream_obj);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddItemToObject(
        response,
        b"stream\0".as_ptr() as *const ::core::ffi::c_char,
        stream_obj,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    if json_str.is_null() {
        cJSON_Delete(response);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
}
