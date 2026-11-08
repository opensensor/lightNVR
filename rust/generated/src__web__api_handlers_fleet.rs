unsafe extern "C" {
    pub type fleet_selector;
    pub type authorization_context;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateString(string: *const ::core::ffi::c_char) -> *mut cJSON;
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
    unsafe fn cJSON_AddObjectToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddArrayToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut g_config: config_t;
    unsafe fn fleet_selector_parse(
        json: *const cJSON,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> *mut fleet_selector_t;
    unsafe fn fleet_selector_free(selector: *mut fleet_selector_t);
    unsafe fn fleet_selector_matches(
        selector: *const fleet_selector_t,
        camera: *const fleet_camera_t,
        explanation: *mut fleet_selector_explanation_t,
    ) -> bool;
    unsafe fn fleet_health_state_name(state: fleet_health_state_t) -> *const ::core::ffi::c_char;
    unsafe fn fleet_availability_state_name(
        state: fleet_availability_state_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn fleet_camera_recording_mode(
        camera: *const fleet_camera_t,
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
    unsafe fn authorization_filter_visible_cameras(
        user: *const user_t,
        cameras: *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn camera_collection_filter_load(
        collection_uuid: *const ::core::ffi::c_char,
        user: *const user_t,
        filter: *mut camera_collection_filter_t,
    ) -> camera_collection_filter_result_t;
    unsafe fn camera_collection_filter_free(filter: *mut camera_collection_filter_t);
    unsafe fn camera_collection_filter_matches(
        filter: *const camera_collection_filter_t,
        camera: *const fleet_camera_t,
    ) -> bool;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fleet_camera_enrich_runtime_health(
        cameras: *mut fleet_camera_t,
        count: ::core::ffi::c_int,
    );
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
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
    unsafe fn fleet_health_enrich_go2rtc_activity(
        cameras: *mut fleet_camera_t,
        camera_count: ::core::ffi::c_int,
    );
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
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
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = usize;
pub type int64_t = i64;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
pub type fleet_selector_t = fleet_selector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_selector_explanation_t {
    pub clauses: [[::core::ffi::c_char; 160]; 64],
    pub clause_count: ::core::ffi::c_int,
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
pub type authorization_context_t = authorization_context;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct camera_collection_filter_result_t(pub ::core::ffi::c_int);
impl camera_collection_filter_result_t {
    pub const CAMERA_COLLECTION_FILTER_OK: Self = Self(0);
    pub const CAMERA_COLLECTION_FILTER_NOT_FOUND: Self = Self(-1);
    pub const CAMERA_COLLECTION_FILTER_INVALID_SELECTOR: Self = Self(-2);
    pub const CAMERA_COLLECTION_FILTER_OUT_OF_MEMORY: Self = Self(-3);
    pub const CAMERA_COLLECTION_FILTER_DATABASE_ERROR: Self = Self(-4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camera_collection_filter_t {
    pub active: bool,
    pub smart_selector: *mut fleet_selector_t,
    pub member_uuids: *mut [::core::ffi::c_char; 37],
    pub member_count: ::core::ffi::c_int,
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
pub struct fleet_query_options_t {
    pub page: ::core::ffi::c_int,
    pub page_size: ::core::ffi::c_int,
    pub sort_by: [::core::ffi::c_char; 32],
    pub descending: bool,
    pub search: [::core::ffi::c_char; 256],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub collection_uuid: [::core::ffi::c_char; 37],
    pub availability: [::core::ffi::c_char; 24],
    pub include_facets: bool,
    pub explain: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct facet_count_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub label: [::core::ffi::c_char; 256],
    pub count: ::core::ffi::c_int,
}
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const FLEET_SELECTOR_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FLEET_QUERY_DEFAULT_PAGE_SIZE: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const FLEET_QUERY_MAX_PAGE_SIZE: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
pub const FLEET_PREVIEW_MAX_PAGE_SIZE: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
#[thread_local]
static mut comparator_sort_by: [::core::ffi::c_char; 32] = [0; 32];
#[thread_local]
static mut comparator_descending: bool = false;
unsafe extern "C" fn valid_uuid(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null()
        || strlen(value) != (CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int) as size_t
    {
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int {
        let mut c: ::core::ffi::c_char = *value.offset(i as isize);
        if i == 8 as ::core::ffi::c_int
            || i == 13 as ::core::ffi::c_int
            || i == 18 as ::core::ffi::c_int
            || i == 23 as ::core::ffi::c_int
        {
            if c as ::core::ffi::c_int != '-' as ::core::ffi::c_int {
                return r#false != 0;
            }
        } else if !(c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            || c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'f' as ::core::ffi::c_int
            || c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int)
        {
            return r#false != 0;
        }
        i += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn contains_case_insensitive(
    mut haystack: *const ::core::ffi::c_char,
    mut needle: *const ::core::ffi::c_char,
) -> bool {
    if needle.is_null()
        || *needle.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    if haystack.is_null() {
        return r#false != 0;
    }
    let mut needle_length: size_t = strlen(needle);
    let mut candidate: *const ::core::ffi::c_char = haystack;
    while *candidate != 0 {
        if strncasecmp(candidate, needle, needle_length) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        candidate = candidate.offset(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn camera_matches_search(
    mut camera: *const fleet_camera_t,
    mut search: *const ::core::ffi::c_char,
) -> bool {
    if search.is_null()
        || *search.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    if contains_case_insensitive(
        &raw const (*camera).name as *const ::core::ffi::c_char,
        search,
    ) as ::core::ffi::c_int
        != 0
        || contains_case_insensitive(
            &raw const (*camera).camera_uuid as *const ::core::ffi::c_char,
            search,
        ) as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(
            &raw const (*camera).address as *const ::core::ffi::c_char,
            search,
        ) as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(
            &raw const (*camera).location_path as *const ::core::ffi::c_char,
            search,
        ) as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(
            &raw const (*camera).manufacturer as *const ::core::ffi::c_char,
            search,
        ) as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(
            &raw const (*camera).model as *const ::core::ffi::c_char,
            search,
        ) as ::core::ffi::c_int
            != 0
    {
        return r#true != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*camera).tag_count {
        if contains_case_insensitive(
            &raw const (*(&raw const (*camera).tags as *const fleet_camera_tag_t)
                .offset(i as isize))
            .label as *const ::core::ffi::c_char,
            search,
        ) {
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn compare_bool(mut left: bool, mut right: bool) -> ::core::ffi::c_int {
    return if left as ::core::ffi::c_int == right as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else if left as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn compare_camera_pointers(
    mut left_pointer: *const ::core::ffi::c_void,
    mut right_pointer: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut left: *const fleet_camera_t = *(left_pointer as *const *mut fleet_camera_t);
    let mut right: *const fleet_camera_t = *(right_pointer as *const *mut fleet_camera_t);
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if strcmp(
        &raw mut comparator_sort_by as *mut ::core::ffi::c_char,
        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        result = strcmp(
            &raw const (*left).camera_uuid as *const ::core::ffi::c_char,
            &raw const (*right).camera_uuid as *const ::core::ffi::c_char,
        );
    } else if strcmp(
        &raw mut comparator_sort_by as *mut ::core::ffi::c_char,
        b"location\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        result = strcasecmp(
            &raw const (*left).location_path as *const ::core::ffi::c_char,
            &raw const (*right).location_path as *const ::core::ffi::c_char,
        );
    } else if strcmp(
        &raw mut comparator_sort_by as *mut ::core::ffi::c_char,
        b"health\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        result = (*left).health.0 as ::core::ffi::c_int - (*right).health.0 as ::core::ffi::c_int;
    } else if strcmp(
        &raw mut comparator_sort_by as *mut ::core::ffi::c_char,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        result = compare_bool((*left).enabled, (*right).enabled);
    } else if strcmp(
        &raw mut comparator_sort_by as *mut ::core::ffi::c_char,
        b"recording_mode\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        result = strcmp(
            fleet_camera_recording_mode(left),
            fleet_camera_recording_mode(right),
        );
    } else if strcmp(
        &raw mut comparator_sort_by as *mut ::core::ffi::c_char,
        b"address\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        result = strcasecmp(
            &raw const (*left).address as *const ::core::ffi::c_char,
            &raw const (*right).address as *const ::core::ffi::c_char,
        );
    } else {
        result = strcasecmp(
            &raw const (*left).name as *const ::core::ffi::c_char,
            &raw const (*right).name as *const ::core::ffi::c_char,
        );
    }
    if result == 0 as ::core::ffi::c_int {
        result = strcmp(
            &raw const (*left).camera_uuid as *const ::core::ffi::c_char,
            &raw const (*right).camera_uuid as *const ::core::ffi::c_char,
        );
    }
    return if comparator_descending as ::core::ffi::c_int != 0 {
        -result
    } else {
        result
    };
}
unsafe extern "C" fn valid_sort_field(mut field: *const ::core::ffi::c_char) -> bool {
    return strcmp(field, b"name\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcmp(
            field,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(field, b"location\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(field, b"health\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(field, b"enabled\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(
            field,
            b"recording_mode\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(field, b"address\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_positive_int(
    mut body: *const cJSON,
    mut field: *const ::core::ffi::c_char,
    mut default_value: ::core::ffi::c_int,
    mut maximum: ::core::ffi::c_int,
    mut output: *mut ::core::ffi::c_int,
    mut res: *mut http_response_t,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, field);
    if item.is_null() {
        *output = default_value;
        return r#true != 0;
    }
    if cJSON_IsNumber(item) == 0
        || (*item).valuedouble != (*item).valueint as ::core::ffi::c_double
        || (*item).valueint < 1 as ::core::ffi::c_int
        || (*item).valueint > maximum
    {
        let mut message: [::core::ffi::c_char; 128] = [0; 128];
        snprintf(
            &raw mut message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            b"%s must be an integer from 1 to %d\0".as_ptr() as *const ::core::ffi::c_char,
            field,
            maximum,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            &raw mut message as *mut ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *output = (*item).valueint;
    return r#true != 0;
}
unsafe extern "C" fn parse_options(
    mut body: *const cJSON,
    mut preview: bool,
    mut options: *mut fleet_query_options_t,
    mut res: *mut http_response_t,
) -> bool {
    memset(
        options as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<fleet_query_options_t>(),
    );
    (*options).include_facets = r#true != 0;
    safe_strcpy(
        &raw mut (*options).sort_by as *mut ::core::ffi::c_char,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*options).availability as *mut ::core::ffi::c_char,
        b"all\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        0 as size_t,
    );
    let mut maximum_page_size: ::core::ffi::c_int = if preview as ::core::ffi::c_int != 0 {
        FLEET_PREVIEW_MAX_PAGE_SIZE
    } else {
        FLEET_QUERY_MAX_PAGE_SIZE
    };
    if !parse_positive_int(
        body,
        b"page\0".as_ptr() as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
        1000000 as ::core::ffi::c_int,
        &raw mut (*options).page,
        res,
    ) || !parse_positive_int(
        body,
        b"page_size\0".as_ptr() as *const ::core::ffi::c_char,
        FLEET_QUERY_DEFAULT_PAGE_SIZE,
        maximum_page_size,
        &raw mut (*options).page_size,
        res,
    ) {
        return r#false != 0;
    }
    let mut sort_by: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"sort_by\0".as_ptr() as *const ::core::ffi::c_char);
    if !sort_by.is_null() {
        if cJSON_IsString(sort_by) == 0
            || (*sort_by).valuestring.is_null()
            || !valid_sort_field((*sort_by).valuestring)
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid sort_by field\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*options).sort_by as *mut ::core::ffi::c_char,
            (*sort_by).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            0 as size_t,
        );
    }
    let mut sort_order: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"sort_order\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !sort_order.is_null() {
        if cJSON_IsString(sort_order) == 0
            || (*sort_order).valuestring.is_null()
            || strcmp(
                (*sort_order).valuestring,
                b"asc\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
                && strcmp(
                    (*sort_order).valuestring,
                    b"desc\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"sort_order must be asc or desc\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*options).descending = strcmp(
            (*sort_order).valuestring,
            b"desc\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
    }
    let mut search: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"search\0".as_ptr() as *const ::core::ffi::c_char);
    if !search.is_null() {
        if cJSON_IsString(search) == 0
            || (*search).valuestring.is_null()
            || strlen((*search).valuestring) >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid search value\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*options).search as *mut ::core::ffi::c_char,
            (*search).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
    }
    let mut camera_uuid: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !camera_uuid.is_null() {
        if cJSON_IsString(camera_uuid) == 0 || !valid_uuid((*camera_uuid).valuestring) {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*options).camera_uuid as *mut ::core::ffi::c_char,
            (*camera_uuid).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
    }
    let mut collection_uuid: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !collection_uuid.is_null() {
        if cJSON_IsString(collection_uuid) == 0 || !valid_uuid((*collection_uuid).valuestring) {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*options).collection_uuid as *mut ::core::ffi::c_char,
            (*collection_uuid).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
    }
    let mut availability: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"availability\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !availability.is_null() {
        let mut value: *const ::core::ffi::c_char = if cJSON_IsString(availability) != 0 {
            (*availability).valuestring
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
        if value.is_null()
            || strcmp(value, b"all\0".as_ptr() as *const ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
                && strcmp(value, b"live\0".as_ptr() as *const ::core::ffi::c_char)
                    != 0 as ::core::ffi::c_int
                && strcmp(value, b"offline\0".as_ptr() as *const ::core::ffi::c_char)
                    != 0 as ::core::ffi::c_int
                && strcmp(
                    value,
                    b"never_connected\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                && strcmp(value, b"disabled\0".as_ptr() as *const ::core::ffi::c_char)
                    != 0 as ::core::ffi::c_int
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid availability value\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*options).availability as *mut ::core::ffi::c_char,
            value,
            ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
            0 as size_t,
        );
    }
    let mut facets: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"facets\0".as_ptr() as *const ::core::ffi::c_char);
    if !facets.is_null() {
        if cJSON_IsBool(facets) == 0 {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"facets must be boolean\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*options).include_facets = cJSON_IsTrue(facets) != 0;
    }
    let mut explain_item: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"explain\0".as_ptr() as *const ::core::ffi::c_char);
    if !explain_item.is_null() && cJSON_IsBool(explain_item) == 0 {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"explain must be boolean\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    (*options).explain = preview as ::core::ffi::c_int != 0
        || !explain_item.is_null() && cJSON_IsTrue(explain_item) != 0;
    return r#true != 0;
}
unsafe extern "C" fn facet_increment(
    mut facets: *mut *mut facet_count_t,
    mut count: *mut ::core::ffi::c_int,
    mut capacity: *mut ::core::ffi::c_int,
    mut uuid: *const ::core::ffi::c_char,
    mut label: *const ::core::ffi::c_char,
) -> bool {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < *count {
        if strcasecmp(
            &raw mut (*(*facets).offset(i as isize)).uuid as *mut ::core::ffi::c_char,
            uuid,
        ) == 0 as ::core::ffi::c_int
        {
            (*(*facets).offset(i as isize)).count += 1;
            if (*(*facets).offset(i as isize)).label[0usize] as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int
                && !label.is_null()
                && *label.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                safe_strcpy(
                    &raw mut (*(*facets).offset(i as isize)).label as *mut ::core::ffi::c_char,
                    label,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    0 as size_t,
                );
            }
            return r#true != 0;
        }
        i += 1;
    }
    if *count == *capacity {
        let mut new_capacity: ::core::ffi::c_int = if *capacity == 0 as ::core::ffi::c_int {
            16 as ::core::ffi::c_int
        } else {
            *capacity * 2 as ::core::ffi::c_int
        };
        let mut resized: *mut facet_count_t = realloc(
            *facets as *mut ::core::ffi::c_void,
            (new_capacity as size_t).wrapping_mul(::core::mem::size_of::<facet_count_t>()),
        ) as *mut facet_count_t;
        if resized.is_null() {
            return r#false != 0;
        }
        *facets = resized;
        *capacity = new_capacity;
    }
    let c2rust_fresh1 = *count;
    *count += 1;
    let mut facet: *mut facet_count_t = (*facets).offset(c2rust_fresh1 as isize);
    memset(
        facet as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<facet_count_t>(),
    );
    safe_strcpy(
        &raw mut (*facet).uuid as *mut ::core::ffi::c_char,
        uuid,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*facet).label as *mut ::core::ffi::c_char,
        if !label.is_null() {
            label
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    (*facet).count = 1 as ::core::ffi::c_int;
    return r#true != 0;
}
unsafe extern "C" fn compare_facets(
    mut left_pointer: *const ::core::ffi::c_void,
    mut right_pointer: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut left: *const facet_count_t = left_pointer as *const facet_count_t;
    let mut right: *const facet_count_t = right_pointer as *const facet_count_t;
    let mut result: ::core::ffi::c_int = strcasecmp(
        &raw const (*left).label as *const ::core::ffi::c_char,
        &raw const (*right).label as *const ::core::ffi::c_char,
    );
    return if result == 0 as ::core::ffi::c_int {
        strcmp(
            &raw const (*left).uuid as *const ::core::ffi::c_char,
            &raw const (*right).uuid as *const ::core::ffi::c_char,
        )
    } else {
        result
    };
}
unsafe extern "C" fn build_facets(
    mut matches: *mut *mut fleet_camera_t,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut root: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut health: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut enabled: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut recording: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut availability: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut tags: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut locations: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut recording_names: [*const ::core::ffi::c_char; 3] =
        [::core::ptr::null::<::core::ffi::c_char>(); 3];
    let mut health_counts: [::core::ffi::c_int; 5] = [0 as ::core::ffi::c_int; 5];
    let mut enabled_counts: [::core::ffi::c_int; 2] = [0 as ::core::ffi::c_int; 2];
    let mut recording_counts: [::core::ffi::c_int; 3] = [0 as ::core::ffi::c_int; 3];
    let mut availability_counts: [::core::ffi::c_int; 4] = [0 as ::core::ffi::c_int; 4];
    let mut tag_facets: *mut facet_count_t = ::core::ptr::null_mut::<facet_count_t>();
    let mut location_facets: *mut facet_count_t = ::core::ptr::null_mut::<facet_count_t>();
    let mut tag_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tag_capacity: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut location_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut location_capacity: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    '_fail: {
        while i < count {
            let mut camera: *mut fleet_camera_t = *matches.offset(i as isize);
            if (*camera).health.0 >= fleet_health_state_t::FLEET_HEALTH_UNKNOWN.0
                && (*camera).health.0 <= fleet_health_state_t::FLEET_HEALTH_DISABLED.0
            {
                health_counts[(*camera).health.0 as usize] += 1;
            }
            enabled_counts[(if (*camera).enabled as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as usize] += 1;
            if (*camera).availability.0 >= fleet_availability_state_t::FLEET_AVAILABILITY_LIVE.0
                && (*camera).availability.0 < fleet_availability_state_t::FLEET_AVAILABILITY_COUNT.0
            {
                availability_counts[(*camera).availability.0 as usize] += 1;
            }
            let mut mode: *const ::core::ffi::c_char = fleet_camera_recording_mode(camera);
            recording_counts[(if strcmp(mode, b"off\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                0 as ::core::ffi::c_int
            } else if strcmp(mode, b"continuous\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            }) as usize] += 1;
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < (*camera).tag_count {
                if !facet_increment(
                    &raw mut tag_facets,
                    &raw mut tag_count,
                    &raw mut tag_capacity,
                    &raw mut (*(&raw mut (*camera).tags as *mut fleet_camera_tag_t)
                        .offset(j as isize))
                    .uuid as *mut ::core::ffi::c_char,
                    &raw mut (*(&raw mut (*camera).tags as *mut fleet_camera_tag_t)
                        .offset(j as isize))
                    .label as *mut ::core::ffi::c_char,
                ) {
                    break '_fail;
                }
                j += 1;
            }
            let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j_0 < (*camera).location_depth {
                let mut label: *const ::core::ffi::c_char = if strcmp(
                    &raw mut *(&raw mut (*camera).location_ancestor_uuids
                        as *mut [::core::ffi::c_char; 37])
                        .offset(j_0 as isize) as *mut ::core::ffi::c_char,
                    &raw mut (*camera).location_uuid as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    &raw mut (*camera).location_name as *mut ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                };
                if !facet_increment(
                    &raw mut location_facets,
                    &raw mut location_count,
                    &raw mut location_capacity,
                    &raw mut *(&raw mut (*camera).location_ancestor_uuids
                        as *mut [::core::ffi::c_char; 37])
                        .offset(j_0 as isize) as *mut ::core::ffi::c_char,
                    label,
                ) {
                    break '_fail;
                }
                j_0 += 1;
            }
            i += 1;
        }
        if tag_count > 1 as ::core::ffi::c_int {
            qsort(
                tag_facets as *mut ::core::ffi::c_void,
                tag_count as size_t,
                ::core::mem::size_of::<facet_count_t>(),
                Some(
                    compare_facets
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_void,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
            );
        }
        if location_count > 1 as ::core::ffi::c_int {
            qsort(
                location_facets as *mut ::core::ffi::c_void,
                location_count as size_t,
                ::core::mem::size_of::<facet_count_t>(),
                Some(
                    compare_facets
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_void,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
            );
        }
        root = cJSON_CreateObject();
        health = cJSON_CreateArray();
        enabled = cJSON_CreateArray();
        recording = cJSON_CreateArray();
        availability = cJSON_CreateArray();
        tags = cJSON_CreateArray();
        locations = cJSON_CreateArray();
        if root.is_null()
            || health.is_null()
            || enabled.is_null()
            || recording.is_null()
            || availability.is_null()
            || tags.is_null()
            || locations.is_null()
        {
            cJSON_Delete(root);
            cJSON_Delete(health);
            cJSON_Delete(enabled);
            cJSON_Delete(recording);
            cJSON_Delete(availability);
            cJSON_Delete(tags);
            cJSON_Delete(locations);
        } else {
            let mut i_0: ::core::ffi::c_int =
                fleet_availability_state_t::FLEET_AVAILABILITY_LIVE.0 as ::core::ffi::c_int;
            while i_0 < fleet_availability_state_t::FLEET_AVAILABILITY_COUNT.0 as ::core::ffi::c_int
            {
                let mut item: *mut cJSON = cJSON_CreateObject();
                cJSON_AddStringToObject(
                    item,
                    b"value\0".as_ptr() as *const ::core::ffi::c_char,
                    fleet_availability_state_name(fleet_availability_state_t(
                        i_0 as ::core::ffi::c_uint,
                    )),
                );
                cJSON_AddNumberToObject(
                    item,
                    b"count\0".as_ptr() as *const ::core::ffi::c_char,
                    availability_counts[i_0 as usize] as ::core::ffi::c_double,
                );
                cJSON_AddItemToArray(availability, item);
                i_0 += 1;
            }
            let mut i_1: ::core::ffi::c_int =
                fleet_health_state_t::FLEET_HEALTH_UNKNOWN.0 as ::core::ffi::c_int;
            while i_1 <= fleet_health_state_t::FLEET_HEALTH_DISABLED.0 as ::core::ffi::c_int {
                let mut item_0: *mut cJSON = cJSON_CreateObject();
                cJSON_AddStringToObject(
                    item_0,
                    b"value\0".as_ptr() as *const ::core::ffi::c_char,
                    fleet_health_state_name(fleet_health_state_t(i_1 as ::core::ffi::c_uint)),
                );
                cJSON_AddNumberToObject(
                    item_0,
                    b"count\0".as_ptr() as *const ::core::ffi::c_char,
                    health_counts[i_1 as usize] as ::core::ffi::c_double,
                );
                cJSON_AddItemToArray(health, item_0);
                i_1 += 1;
            }
            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_2 < 2 as ::core::ffi::c_int {
                let mut item_1: *mut cJSON = cJSON_CreateObject();
                cJSON_AddBoolToObject(
                    item_1,
                    b"value\0".as_ptr() as *const ::core::ffi::c_char,
                    (i_2 != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                );
                cJSON_AddNumberToObject(
                    item_1,
                    b"count\0".as_ptr() as *const ::core::ffi::c_char,
                    enabled_counts[i_2 as usize] as ::core::ffi::c_double,
                );
                cJSON_AddItemToArray(enabled, item_1);
                i_2 += 1;
            }
            recording_names = [
                b"off\0".as_ptr() as *const ::core::ffi::c_char,
                b"continuous\0".as_ptr() as *const ::core::ffi::c_char,
                b"detection\0".as_ptr() as *const ::core::ffi::c_char,
            ];
            let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_3 < 3 as ::core::ffi::c_int {
                let mut item_2: *mut cJSON = cJSON_CreateObject();
                cJSON_AddStringToObject(
                    item_2,
                    b"value\0".as_ptr() as *const ::core::ffi::c_char,
                    recording_names[i_3 as usize],
                );
                cJSON_AddNumberToObject(
                    item_2,
                    b"count\0".as_ptr() as *const ::core::ffi::c_char,
                    recording_counts[i_3 as usize] as ::core::ffi::c_double,
                );
                cJSON_AddItemToArray(recording, item_2);
                i_3 += 1;
            }
            let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_4 < tag_count {
                let mut item_3: *mut cJSON = cJSON_CreateObject();
                cJSON_AddStringToObject(
                    item_3,
                    b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*tag_facets.offset(i_4 as isize)).uuid as *mut ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    item_3,
                    b"label\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*tag_facets.offset(i_4 as isize)).label as *mut ::core::ffi::c_char,
                );
                cJSON_AddNumberToObject(
                    item_3,
                    b"count\0".as_ptr() as *const ::core::ffi::c_char,
                    (*tag_facets.offset(i_4 as isize)).count as ::core::ffi::c_double,
                );
                cJSON_AddItemToArray(tags, item_3);
                i_4 += 1;
            }
            let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_5 < location_count {
                let mut item_4: *mut cJSON = cJSON_CreateObject();
                cJSON_AddStringToObject(
                    item_4,
                    b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*location_facets.offset(i_5 as isize)).uuid
                        as *mut ::core::ffi::c_char,
                );
                if (*location_facets.offset(i_5 as isize)).label[0usize] != 0 {
                    cJSON_AddStringToObject(
                        item_4,
                        b"label\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*location_facets.offset(i_5 as isize)).label
                            as *mut ::core::ffi::c_char,
                    );
                }
                cJSON_AddNumberToObject(
                    item_4,
                    b"count\0".as_ptr() as *const ::core::ffi::c_char,
                    (*location_facets.offset(i_5 as isize)).count as ::core::ffi::c_double,
                );
                cJSON_AddItemToArray(locations, item_4);
                i_5 += 1;
            }
            cJSON_AddItemToObject(
                root,
                b"health\0".as_ptr() as *const ::core::ffi::c_char,
                health,
            );
            cJSON_AddItemToObject(
                root,
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
                enabled,
            );
            cJSON_AddItemToObject(
                root,
                b"recording_mode\0".as_ptr() as *const ::core::ffi::c_char,
                recording,
            );
            cJSON_AddItemToObject(
                root,
                b"availability\0".as_ptr() as *const ::core::ffi::c_char,
                availability,
            );
            cJSON_AddItemToObject(root, b"tags\0".as_ptr() as *const ::core::ffi::c_char, tags);
            cJSON_AddItemToObject(
                root,
                b"locations\0".as_ptr() as *const ::core::ffi::c_char,
                locations,
            );
            free(tag_facets as *mut ::core::ffi::c_void);
            free(location_facets as *mut ::core::ffi::c_void);
            return root;
        }
    }
    free(tag_facets as *mut ::core::ffi::c_void);
    free(location_facets as *mut ::core::ffi::c_void);
    return ::core::ptr::null_mut::<cJSON>();
}
unsafe extern "C" fn camera_to_json(
    mut camera: *const fleet_camera_t,
    mut selector: *const fleet_selector_t,
    mut explain_match: bool,
    mut can_configure: bool,
) -> *mut cJSON {
    let mut capabilities: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut location: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut tags: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut object: *mut cJSON = cJSON_CreateObject();
    if object.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    '_fail: {
        if !(cJSON_AddStringToObject(
            object,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*camera).camera_uuid as *const ::core::ffi::c_char,
        )
        .is_null()
            || cJSON_AddStringToObject(
                object,
                b"name\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*camera).name as *const ::core::ffi::c_char,
            )
            .is_null()
            || cJSON_AddStringToObject(
                object,
                b"address\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*camera).address as *const ::core::ffi::c_char,
            )
            .is_null()
            || cJSON_AddBoolToObject(
                object,
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
                (*camera).enabled as cJSON_bool,
            )
            .is_null()
            || cJSON_AddStringToObject(
                object,
                b"recording_mode\0".as_ptr() as *const ::core::ffi::c_char,
                fleet_camera_recording_mode(camera),
            )
            .is_null()
            || cJSON_AddStringToObject(
                object,
                b"health\0".as_ptr() as *const ::core::ffi::c_char,
                fleet_health_state_name((*camera).health),
            )
            .is_null()
            || cJSON_AddStringToObject(
                object,
                b"availability\0".as_ptr() as *const ::core::ffi::c_char,
                fleet_availability_state_name((*camera).availability),
            )
            .is_null()
            || cJSON_AddNumberToObject(
                object,
                b"health_changed_at\0".as_ptr() as *const ::core::ffi::c_char,
                (*camera).health_changed_at as ::core::ffi::c_double,
            )
            .is_null()
            || cJSON_AddNumberToObject(
                object,
                b"last_frame_ts\0".as_ptr() as *const ::core::ffi::c_char,
                (*camera).last_frame_ts as ::core::ffi::c_double,
            )
            .is_null()
            || cJSON_AddNumberToObject(
                object,
                b"first_video_at\0".as_ptr() as *const ::core::ffi::c_char,
                (*camera).first_video_at as ::core::ffi::c_double,
            )
            .is_null()
            || cJSON_AddNumberToObject(
                object,
                b"last_recording_at\0".as_ptr() as *const ::core::ffi::c_char,
                (*camera).last_recording_at as ::core::ffi::c_double,
            )
            .is_null()
            || cJSON_AddNumberToObject(
                object,
                b"current_fps\0".as_ptr() as *const ::core::ffi::c_char,
                (*camera).current_fps,
            )
            .is_null()
            || cJSON_AddBoolToObject(
                object,
                b"recording_active\0".as_ptr() as *const ::core::ffi::c_char,
                (*camera).recording_active as cJSON_bool,
            )
            .is_null()
            || cJSON_AddBoolToObject(
                object,
                b"can_configure\0".as_ptr() as *const ::core::ffi::c_char,
                can_configure as cJSON_bool,
            )
            .is_null()
            || cJSON_AddStringToObject(
                object,
                b"manufacturer\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*camera).manufacturer as *const ::core::ffi::c_char,
            )
            .is_null()
            || cJSON_AddStringToObject(
                object,
                b"model\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*camera).model as *const ::core::ffi::c_char,
            )
            .is_null())
        {
            capabilities = cJSON_CreateArray();
            if capabilities.is_null()
                || cJSON_AddItemToObject(
                    object,
                    b"capabilities\0".as_ptr() as *const ::core::ffi::c_char,
                    capabilities,
                ) == 0
            {
                cJSON_Delete(capabilities);
            } else {
                if (*camera).is_onvif {
                    let mut capability: *mut cJSON =
                        cJSON_CreateString(b"onvif\0".as_ptr() as *const ::core::ffi::c_char);
                    if capability.is_null() || cJSON_AddItemToArray(capabilities, capability) == 0 {
                        cJSON_Delete(capability);
                        break '_fail;
                    }
                }
                if (*camera).ptz_enabled {
                    let mut capability_0: *mut cJSON =
                        cJSON_CreateString(b"ptz\0".as_ptr() as *const ::core::ffi::c_char);
                    if capability_0.is_null()
                        || cJSON_AddItemToArray(capabilities, capability_0) == 0
                    {
                        cJSON_Delete(capability_0);
                        break '_fail;
                    }
                }
                if (*camera).backchannel_enabled as ::core::ffi::c_int != 0
                    && !g_config.audio_disabled
                {
                    let mut capability_1: *mut cJSON =
                        cJSON_CreateString(b"backchannel\0".as_ptr() as *const ::core::ffi::c_char);
                    if capability_1.is_null()
                        || cJSON_AddItemToArray(capabilities, capability_1) == 0
                    {
                        cJSON_Delete(capability_1);
                        break '_fail;
                    }
                }
                location = cJSON_CreateObject();
                if location.is_null()
                    || cJSON_AddItemToObject(
                        object,
                        b"location\0".as_ptr() as *const ::core::ffi::c_char,
                        location,
                    ) == 0
                {
                    cJSON_Delete(location);
                } else if !(cJSON_AddStringToObject(
                    location,
                    b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*camera).location_uuid as *const ::core::ffi::c_char,
                )
                .is_null()
                    || cJSON_AddStringToObject(
                        location,
                        b"name\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw const (*camera).location_name as *const ::core::ffi::c_char,
                    )
                    .is_null()
                    || cJSON_AddStringToObject(
                        location,
                        b"path\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw const (*camera).location_path as *const ::core::ffi::c_char,
                    )
                    .is_null())
                {
                    tags = cJSON_CreateArray();
                    if tags.is_null()
                        || cJSON_AddItemToObject(
                            object,
                            b"tags\0".as_ptr() as *const ::core::ffi::c_char,
                            tags,
                        ) == 0
                    {
                        cJSON_Delete(tags);
                    } else {
                        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i < (*camera).tag_count {
                            let mut tag: *mut cJSON = cJSON_CreateObject();
                            if tag.is_null()
                                || cJSON_AddStringToObject(
                                    tag,
                                    b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
                                    &raw const (*(&raw const (*camera).tags
                                        as *const fleet_camera_tag_t)
                                        .offset(i as isize))
                                    .uuid
                                        as *const ::core::ffi::c_char,
                                )
                                .is_null()
                                || cJSON_AddStringToObject(
                                    tag,
                                    b"label\0".as_ptr() as *const ::core::ffi::c_char,
                                    &raw const (*(&raw const (*camera).tags
                                        as *const fleet_camera_tag_t)
                                        .offset(i as isize))
                                    .label
                                        as *const ::core::ffi::c_char,
                                )
                                .is_null()
                                || cJSON_AddItemToArray(tags, tag) == 0
                            {
                                cJSON_Delete(tag);
                                break '_fail;
                            } else {
                                i += 1;
                            }
                        }
                        's_192: {
                            if explain_match {
                                let mut explanation: fleet_selector_explanation_t =
                                    fleet_selector_explanation_t {
                                        clauses: [[0; 160]; 64],
                                        clause_count: 0,
                                    };
                                if fleet_selector_matches(selector, camera, &raw mut explanation) {
                                    let mut clauses: *mut cJSON = cJSON_CreateArray();
                                    if clauses.is_null()
                                        || cJSON_AddItemToObject(
                                            object,
                                            b"matched_clauses\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            clauses,
                                        ) == 0
                                    {
                                        cJSON_Delete(clauses);
                                        break '_fail;
                                    } else {
                                        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                        loop {
                                            if i_0 >= explanation.clause_count {
                                                break 's_192;
                                            }
                                            let mut clause: *mut cJSON = cJSON_CreateString(
                                                &raw mut *(&raw mut explanation.clauses
                                                    as *mut [::core::ffi::c_char; 160])
                                                    .offset(i_0 as isize)
                                                    as *mut ::core::ffi::c_char,
                                            );
                                            if clause.is_null()
                                                || cJSON_AddItemToArray(clauses, clause) == 0
                                            {
                                                cJSON_Delete(clause);
                                                break '_fail;
                                            } else {
                                                i_0 += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        return object;
                    }
                }
            }
        }
    }
    cJSON_Delete(object);
    return ::core::ptr::null_mut::<cJSON>();
}
unsafe extern "C" fn operational_queue_item(
    mut key: *const ::core::ffi::c_char,
    mut severity: *const ::core::ffi::c_char,
    mut health: *const ::core::ffi::c_char,
    mut remediation: *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
    mut oldest: int64_t,
) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_CreateObject();
    if item.is_null()
        || cJSON_AddStringToObject(item, b"key\0".as_ptr() as *const ::core::ffi::c_char, key)
            .is_null()
        || cJSON_AddStringToObject(
            item,
            b"severity\0".as_ptr() as *const ::core::ffi::c_char,
            severity,
        )
        .is_null()
        || cJSON_AddNumberToObject(
            item,
            b"count\0".as_ptr() as *const ::core::ffi::c_char,
            count as ::core::ffi::c_double,
        )
        .is_null()
        || cJSON_AddStringToObject(
            item,
            b"remediation\0".as_ptr() as *const ::core::ffi::c_char,
            remediation,
        )
        .is_null()
    {
        cJSON_Delete(item);
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut selector: *mut cJSON =
        cJSON_AddObjectToObject(item, b"selector\0".as_ptr() as *const ::core::ffi::c_char);
    let mut expression: *mut cJSON = if !selector.is_null() {
        cJSON_AddObjectToObject(
            selector,
            b"expression\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut values: *mut cJSON = if !expression.is_null() {
        cJSON_AddArrayToObject(
            expression,
            b"values\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if selector.is_null()
        || expression.is_null()
        || values.is_null()
        || cJSON_AddNumberToObject(
            selector,
            b"version\0".as_ptr() as *const ::core::ffi::c_char,
            FLEET_SELECTOR_VERSION as ::core::ffi::c_double,
        )
        .is_null()
        || cJSON_AddStringToObject(
            expression,
            b"op\0".as_ptr() as *const ::core::ffi::c_char,
            b"health\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        || cJSON_AddItemToArray(values, cJSON_CreateString(health)) == 0
    {
        cJSON_Delete(item);
        return ::core::ptr::null_mut::<cJSON>();
    }
    if oldest > 0 as int64_t {
        cJSON_AddNumberToObject(
            item,
            b"oldest_unresolved_at\0".as_ptr() as *const ::core::ffi::c_char,
            oldest as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(
            item,
            b"oldest_unresolved_at\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return item;
}
unsafe extern "C" fn build_operational_queues(
    mut cameras: *const fleet_camera_t,
    mut count: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut offline_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut degraded_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut offline_oldest: int64_t = 0 as int64_t;
    let mut degraded_oldest: int64_t = 0 as int64_t;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut camera: *const fleet_camera_t = cameras.offset(index as isize);
        let mut queue_count: *mut ::core::ffi::c_int =
            ::core::ptr::null_mut::<::core::ffi::c_int>();
        let mut oldest: *mut int64_t = ::core::ptr::null_mut::<int64_t>();
        if (*camera).health.0 == fleet_health_state_t::FLEET_HEALTH_DOWN.0 {
            queue_count = &raw mut offline_count;
            oldest = &raw mut offline_oldest;
        } else if (*camera).health.0 == fleet_health_state_t::FLEET_HEALTH_DEGRADED.0 {
            queue_count = &raw mut degraded_count;
            oldest = &raw mut degraded_oldest;
        }
        if !queue_count.is_null() {
            *queue_count += 1;
            if (*camera).health_changed_at > 0 as int64_t
                && (*oldest == 0 as int64_t || (*camera).health_changed_at < *oldest)
            {
                *oldest = (*camera).health_changed_at;
            }
        }
        index += 1;
    }
    let mut queues: *mut cJSON = cJSON_CreateArray();
    let mut offline: *mut cJSON = operational_queue_item(
        b"offline\0".as_ptr() as *const ::core::ffi::c_char,
        b"critical\0".as_ptr() as *const ::core::ffi::c_char,
        b"down\0".as_ptr() as *const ::core::ffi::c_char,
        b"inspect_connectivity\0".as_ptr() as *const ::core::ffi::c_char,
        offline_count,
        offline_oldest,
    );
    let mut degraded: *mut cJSON = operational_queue_item(
        b"degraded\0".as_ptr() as *const ::core::ffi::c_char,
        b"warning\0".as_ptr() as *const ::core::ffi::c_char,
        b"degraded\0".as_ptr() as *const ::core::ffi::c_char,
        b"inspect_stream_health\0".as_ptr() as *const ::core::ffi::c_char,
        degraded_count,
        degraded_oldest,
    );
    if queues.is_null() || offline.is_null() || degraded.is_null() {
        cJSON_Delete(queues);
        cJSON_Delete(offline);
        cJSON_Delete(degraded);
        return ::core::ptr::null_mut::<cJSON>();
    }
    if cJSON_AddItemToArray(queues, offline) == 0 {
        cJSON_Delete(queues);
        cJSON_Delete(offline);
        cJSON_Delete(degraded);
        return ::core::ptr::null_mut::<cJSON>();
    }
    if cJSON_AddItemToArray(queues, degraded) == 0 {
        cJSON_Delete(queues);
        cJSON_Delete(degraded);
        return ::core::ptr::null_mut::<cJSON>();
    }
    return queues;
}
unsafe extern "C" fn handle_fleet_query(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut preview: bool,
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
    memset(
        &raw mut user as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    if httpd_check_action_access(req, &raw mut user) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    if cJSON_IsObject(body) == 0 {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Request body must be a JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut options: fleet_query_options_t = fleet_query_options_t {
        page: 0,
        page_size: 0,
        sort_by: [0; 32],
        descending: false,
        search: [0; 256],
        camera_uuid: [0; 37],
        collection_uuid: [0; 37],
        availability: [0; 24],
        include_facets: false,
        explain: false,
    };
    if !parse_options(body, preview, &raw mut options, res) {
        cJSON_Delete(body);
        return;
    }
    let mut selector_json: *mut cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut default_selector_json: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    if selector_json.is_null() {
        default_selector_json = cJSON_Parse(
            b"{\"version\":1,\"expression\":{\"op\":\"all\"}}\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        selector_json = default_selector_json;
    }
    let mut selector_error: [::core::ffi::c_char; 256] = [
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
    let mut selector: *mut fleet_selector_t = fleet_selector_parse(
        selector_json,
        &raw mut selector_error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    cJSON_Delete(default_selector_json);
    if selector.is_null() {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            if selector_error[0usize] as ::core::ffi::c_int != 0 {
                &raw mut selector_error as *mut ::core::ffi::c_char
            } else {
                b"Invalid selector\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    let mut collection_filter: camera_collection_filter_t = camera_collection_filter_t {
        active: false,
        smart_selector: ::core::ptr::null_mut::<fleet_selector_t>(),
        member_uuids: ::core::ptr::null_mut::<[::core::ffi::c_char; 37]>(),
        member_count: 0,
    };
    let mut collection_result: camera_collection_filter_result_t = camera_collection_filter_load(
        &raw mut options.collection_uuid as *mut ::core::ffi::c_char,
        &raw mut user,
        &raw mut collection_filter,
    );
    if collection_result.0 != camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_OK.0 {
        fleet_selector_free(selector);
        cJSON_Delete(body);
        if collection_result.0
            == camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_NOT_FOUND.0
        {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Collection not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if collection_result.0
            == camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_OUT_OF_MEMORY.0
        {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if collection_result.0
            == camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_INVALID_SELECTOR.0
        {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Collection selector is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to load collection members\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if db_fleet_camera_load(&raw mut cameras, &raw mut camera_count) != 0 as ::core::ffi::c_int {
        camera_collection_filter_free(&raw mut collection_filter);
        fleet_selector_free(selector);
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load fleet cameras\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    fleet_camera_enrich_runtime_health(cameras, camera_count);
    fleet_health_enrich_go2rtc_activity(cameras, camera_count);
    if authorization_filter_visible_cameras(&raw mut user, cameras, &raw mut camera_count)
        != 0 as ::core::ffi::c_int
    {
        free(cameras as *mut ::core::ffi::c_void);
        camera_collection_filter_free(&raw mut collection_filter);
        fleet_selector_free(selector);
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut matches: *mut *mut fleet_camera_t = (if camera_count > 0 as ::core::ffi::c_int {
        calloc(
            camera_count as size_t,
            ::core::mem::size_of::<*mut fleet_camera_t>(),
        )
    } else {
        NULL
    }) as *mut *mut fleet_camera_t;
    if camera_count > 0 as ::core::ffi::c_int && matches.is_null() {
        free(cameras as *mut ::core::ffi::c_void);
        camera_collection_filter_free(&raw mut collection_filter);
        fleet_selector_free(selector);
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut match_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < camera_count {
        if !(options.camera_uuid[0usize] as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut options.camera_uuid as *mut ::core::ffi::c_char,
                &raw mut (*cameras.offset(i as isize)).camera_uuid as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int)
        {
            if camera_collection_filter_matches(
                &raw mut collection_filter,
                cameras.offset(i as isize),
            ) {
                if camera_matches_search(
                    cameras.offset(i as isize),
                    &raw mut options.search as *mut ::core::ffi::c_char,
                ) {
                    if !(strcmp(
                        &raw mut options.availability as *mut ::core::ffi::c_char,
                        b"all\0".as_ptr() as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                        && strcmp(
                            &raw mut options.availability as *mut ::core::ffi::c_char,
                            fleet_availability_state_name(
                                (*cameras.offset(i as isize)).availability,
                            ),
                        ) != 0 as ::core::ffi::c_int)
                    {
                        if fleet_selector_matches(
                            selector,
                            cameras.offset(i as isize),
                            ::core::ptr::null_mut::<fleet_selector_explanation_t>(),
                        ) {
                            let c2rust_fresh0 = match_count;
                            match_count += 1;
                            *matches.offset(c2rust_fresh0 as isize) = cameras.offset(i as isize);
                        }
                    }
                }
            }
        }
        i += 1;
    }
    safe_strcpy(
        &raw mut comparator_sort_by as *mut ::core::ffi::c_char,
        &raw mut options.sort_by as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        0 as size_t,
    );
    comparator_descending = options.descending;
    if match_count > 1 as ::core::ffi::c_int {
        qsort(
            matches as *mut ::core::ffi::c_void,
            match_count as size_t,
            ::core::mem::size_of::<*mut fleet_camera_t>(),
            Some(
                compare_camera_pointers
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || items.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(matches as *mut ::core::ffi::c_void);
        free(cameras as *mut ::core::ffi::c_void);
        camera_collection_filter_free(&raw mut collection_filter);
        fleet_selector_free(selector);
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddNumberToObject(
        root,
        b"selector_version\0".as_ptr() as *const ::core::ffi::c_char,
        FLEET_SELECTOR_VERSION as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"queue_version\0".as_ptr() as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        root,
        b"preview\0".as_ptr() as *const ::core::ffi::c_char,
        preview as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        root,
        b"page\0".as_ptr() as *const ::core::ffi::c_char,
        options.page as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"page_size\0".as_ptr() as *const ::core::ffi::c_char,
        options.page_size as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"total\0".as_ptr() as *const ::core::ffi::c_char,
        match_count as ::core::ffi::c_double,
    );
    let mut total_pages: ::core::ffi::c_int = if match_count == 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        (match_count + options.page_size - 1 as ::core::ffi::c_int) / options.page_size
    };
    cJSON_AddNumberToObject(
        root,
        b"total_pages\0".as_ptr() as *const ::core::ffi::c_char,
        total_pages as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        root,
        b"sort_by\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut options.sort_by as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"sort_order\0".as_ptr() as *const ::core::ffi::c_char,
        if options.descending as ::core::ffi::c_int != 0 {
            b"desc\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"asc\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if options.collection_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            root,
            b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut options.collection_uuid as *mut ::core::ffi::c_char,
        );
    }
    cJSON_AddItemToObject(
        root,
        b"cameras\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    let mut queues: *mut cJSON = build_operational_queues(cameras, camera_count);
    if queues.is_null() {
        cJSON_Delete(root);
        free(matches as *mut ::core::ffi::c_void);
        free(cameras as *mut ::core::ffi::c_void);
        camera_collection_filter_free(&raw mut collection_filter);
        fleet_selector_free(selector);
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build operational queues\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddItemToObject(
        root,
        b"queues\0".as_ptr() as *const ::core::ffi::c_char,
        queues,
    );
    if options.include_facets {
        let mut facets: *mut cJSON = build_facets(matches, match_count);
        if facets.is_null() {
            cJSON_Delete(root);
            free(matches as *mut ::core::ffi::c_void);
            free(cameras as *mut ::core::ffi::c_void);
            camera_collection_filter_free(&raw mut collection_filter);
            fleet_selector_free(selector);
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to build fleet facets\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddItemToObject(
            root,
            b"facets\0".as_ptr() as *const ::core::ffi::c_char,
            facets,
        );
    }
    let mut start64: int64_t =
        (options.page as int64_t - 1 as int64_t) * options.page_size as int64_t;
    let mut start: ::core::ffi::c_int = if start64 < match_count as int64_t {
        start64 as ::core::ffi::c_int
    } else {
        match_count
    };
    let mut end: ::core::ffi::c_int = start + options.page_size;
    if end > match_count {
        end = match_count;
    }
    let mut authz_context: *mut authorization_context_t = authorization_context_create();
    if authz_context.is_null() {
        cJSON_Delete(root);
        free(matches as *mut ::core::ffi::c_void);
        free(cameras as *mut ::core::ffi::c_void);
        camera_collection_filter_free(&raw mut collection_filter);
        fleet_selector_free(selector);
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut i_0: ::core::ffi::c_int = start;
    while i_0 < end {
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
        if authorization_evaluate_in_context(
            authz_context,
            &raw mut user,
            authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
            *matches.offset(i_0 as isize),
            &raw mut configure,
        ) != 0 as ::core::ffi::c_int
        {
            authorization_context_free(authz_context);
            cJSON_Delete(root);
            free(matches as *mut ::core::ffi::c_void);
            free(cameras as *mut ::core::ffi::c_void);
            camera_collection_filter_free(&raw mut collection_filter);
            fleet_selector_free(selector);
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut item: *mut cJSON = camera_to_json(
            *matches.offset(i_0 as isize),
            selector,
            options.explain,
            configure.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0,
        );
        if item.is_null() {
            authorization_context_free(authz_context);
            cJSON_Delete(root);
            free(matches as *mut ::core::ffi::c_void);
            free(cameras as *mut ::core::ffi::c_void);
            camera_collection_filter_free(&raw mut collection_filter);
            fleet_selector_free(selector);
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create camera response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddItemToArray(items, item);
        i_0 += 1;
    }
    authorization_context_free(authz_context);
    let mut json: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    free(matches as *mut ::core::ffi::c_void);
    free(cameras as *mut ::core::ffi::c_void);
    camera_collection_filter_free(&raw mut collection_filter);
    fleet_selector_free(selector);
    cJSON_Delete(body);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_fleet_camera_query(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    handle_fleet_query(req, res, r#false != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_fleet_selector_preview(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    handle_fleet_query(req, res, r#true != 0);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
