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
    unsafe fn cJSON_CreateNull() -> *mut cJSON;
    unsafe fn cJSON_CreateString(string: *const ::core::ffi::c_char) -> *mut cJSON;
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
    unsafe fn cJSON_free(object: *mut ::core::ffi::c_void);
    static mut g_config: config_t;
    unsafe fn observation_asset_state_name(
        state: observation_asset_state_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn observation_cursor_encode(
        seq: int64_t,
        epoch_uuid: *const ::core::ffi::c_char,
        out: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_observation_get(
        observation_uuid: *const ::core::ffi::c_char,
        out: *mut observation_record_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_observation_asset_get(
        asset_uuid: *const ::core::ffi::c_char,
        out: *mut observation_asset_record_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_observations_list(
        query: *const observation_list_query_t,
        cursor: *const ::core::ffi::c_char,
        page: *mut observation_page_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_observation_page_free(page: *mut observation_page_t);
    unsafe fn db_observation_meta_get(out: *mut observation_meta_t) -> ::core::ffi::c_int;
    unsafe fn observation_classification_kind_name(
        kind: observation_classification_kind_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn observation_classification_kind_parse(
        name: *const ::core::ffi::c_char,
        kind: *mut observation_classification_kind_t,
    ) -> ::core::ffi::c_int;
    unsafe fn observation_classification_segment_is_valid(
        segment: *const ::core::ffi::c_char,
    ) -> bool;
    unsafe fn db_observation_classification_put(
        observation_uuid: *const ::core::ffi::c_char,
        input: *const observation_classification_input_t,
        result: *mut observation_classification_result_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_observation_classifications_list(
        observation_uuid: *const ::core::ffi::c_char,
        out: *mut observation_classification_set_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_observation_classifications_list_many(
        observation_uuids: *const *const ::core::ffi::c_char,
        count: size_t,
        visit: observation_classification_visit_fn,
        context: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn http_request_get_header(
        req: *const http_request_t,
        name: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
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
    unsafe fn http_serve_file(
        req: *const http_request_t,
        res: *const http_response_t,
        file_path: *const ::core::ffi::c_char,
        content_type: *const ::core::ffi::c_char,
        extra_headers: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn timegm(__tp: *mut tm) -> time_t;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn authorization_action_metadata(
        action: authorization_action_t,
    ) -> *const authorization_action_metadata_t;
    unsafe fn authorization_decision_source_name(
        source: authorization_decision_source_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn authorization_filter_cameras(
        user: *const user_t,
        action: authorization_action_t,
        cameras: *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn event_identity_get_source(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn observation_evidence_get_stats(out: *mut observation_evidence_stats_t);
    unsafe fn observation_snapshot_retry_after_seconds(
        asset_age_ms: int64_t,
        items_ahead: uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_load_readonly(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_get_system_setting(
        key: *const ::core::ffi::c_char,
        out: *mut ::core::ffi::c_char,
        out_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
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
    unsafe fn audit_log_operation(
        req: *const http_request_t,
        user: *const user_t,
        action: *const ::core::ffi::c_char,
        target_type: *const ::core::ffi::c_char,
        target_uuid: *const ::core::ffi::c_char,
        operation: *const ::core::ffi::c_char,
        outcome: *const ::core::ffi::c_char,
        context: *const cJSON,
    );
    unsafe fn audit_log_authorization(
        req: *const http_request_t,
        user: *const user_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        evaluation: *const authorization_evaluation_t,
        outcome: *const ::core::ffi::c_char,
    );
    unsafe fn audit_log_get_decision_mode(action: authorization_action_t) -> audit_decision_mode_t;
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_evaluate_camera_identity_action(
        user: *const user_t,
        action: authorization_action_t,
        camera_uuid: *const ::core::ffi::c_char,
        legacy_stream_name: *const ::core::ffi::c_char,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn get_all_stream_configs_readonly(
        streams: *mut stream_config_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_int);
impl C2Rust_Unnamed {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_asset_record_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub state: observation_asset_state_t,
    pub reason: [::core::ffi::c_char; 33],
    pub origin: [::core::ffi::c_char; 24],
    pub media_type: [::core::ffi::c_char; 24],
    pub path: [::core::ffi::c_char; 512],
    pub has_bytes: bool,
    pub bytes: int64_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub captured_at_ms: int64_t,
    pub transform_json: [::core::ffi::c_char; 1024],
    pub is_protected: bool,
    pub expires_at_ms: int64_t,
    pub created_at_ms: int64_t,
    pub updated_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_record_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub stream_name_at_capture: [::core::ffi::c_char; 64],
    pub batch_id: [::core::ffi::c_char; 37],
    pub object_index: ::core::ffi::c_int,
    pub observed_at_ms: int64_t,
    pub created_at_ms: int64_t,
    pub engine_key: [::core::ffi::c_char; 65],
    pub engine_type: [::core::ffi::c_char; 16],
    pub model: [::core::ffi::c_char; 129],
    pub model_version: [::core::ffi::c_char; 65],
    pub label: [::core::ffi::c_char; 32],
    pub confidence: ::core::ffi::c_double,
    pub has_box: bool,
    pub box_x: ::core::ffi::c_double,
    pub box_y: ::core::ffi::c_double,
    pub box_width: ::core::ffi::c_double,
    pub box_height: ::core::ffi::c_double,
    pub has_track: bool,
    pub track_id: [::core::ffi::c_char; 65],
    pub track_namespace: [::core::ffi::c_char; 65],
    pub track_session: [::core::ffi::c_char; 37],
    pub recording_id: int64_t,
    pub revision: ::core::ffi::c_int,
    pub asset: observation_asset_record_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_list_query_t {
    pub camera_uuids: *const [::core::ffi::c_char; 37],
    pub camera_count: size_t,
    pub all_cameras: bool,
    pub labels: [[::core::ffi::c_char; 32]; 8],
    pub label_count: size_t,
    pub has_since: bool,
    pub since_ms: int64_t,
    pub has_until: bool,
    pub until_ms: int64_t,
    pub limit: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_page_t {
    pub items: *mut observation_record_t,
    pub count: size_t,
    pub next_cursor: [::core::ffi::c_char; 257],
    pub has_more: bool,
    pub oldest_cursor: [::core::ffi::c_char; 257],
    pub has_oldest_observed_at: bool,
    pub oldest_observed_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_meta_t {
    pub epoch_uuid: [::core::ffi::c_char; 37],
    pub oldest_seq: int64_t,
    pub head_seq: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_classification_kind_t(pub ::core::ffi::c_uint);
impl observation_classification_kind_t {
    pub const OBS_CLS_KIND_AUTOMATED: Self = Self(0);
    pub const OBS_CLS_KIND_OWNER_CONFIRMED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_0 {
    pub const DB_OBS_CLS_CREATED: Self = Self(0);
    pub const DB_OBS_CLS_UPDATED: Self = Self(1);
    pub const DB_OBS_CLS_NOOP: Self = Self(2);
    pub const DB_OBS_CLS_REVISION_CONFLICT: Self = Self(3);
    pub const DB_OBS_CLS_OWNER_LOCKED: Self = Self(4);
    pub const DB_OBS_CLS_EVIDENCE_MISMATCH: Self = Self(5);
    pub const DB_OBS_CLS_WRITER_MISMATCH: Self = Self(6);
    pub const DB_OBS_CLS_LIMIT: Self = Self(7);
    pub const DB_OBS_CLS_ERROR: Self = Self(-1);
    pub const DB_OBS_CLS_INVALID: Self = Self(-2);
    pub const DB_OBS_CLS_NOT_FOUND: Self = Self(-3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_classification_t {
    pub key: [::core::ffi::c_char; 66],
    pub writer_namespace: [::core::ffi::c_char; 33],
    pub writer_user_id: int64_t,
    pub classifier: [::core::ffi::c_char; 65],
    pub model_version: [::core::ffi::c_char; 65],
    pub label: [::core::ffi::c_char; 129],
    pub taxonomy_id: [::core::ffi::c_char; 65],
    pub confidence: ::core::ffi::c_double,
    pub classified_at_ms: int64_t,
    pub input_asset_uuid: [::core::ffi::c_char; 37],
    pub input_revision: ::core::ffi::c_int,
    pub kind: observation_classification_kind_t,
    pub annotation_revision: ::core::ffi::c_int,
    pub created_at_ms: int64_t,
    pub updated_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_classification_input_t {
    pub writer_namespace: *const ::core::ffi::c_char,
    pub writer_user_id: int64_t,
    pub name: *const ::core::ffi::c_char,
    pub classifier: *const ::core::ffi::c_char,
    pub model_version: *const ::core::ffi::c_char,
    pub label: *const ::core::ffi::c_char,
    pub taxonomy_id: *const ::core::ffi::c_char,
    pub confidence: ::core::ffi::c_double,
    pub classified_at_ms: int64_t,
    pub input_asset_uuid: *const ::core::ffi::c_char,
    pub input_revision: ::core::ffi::c_int,
    pub kind: observation_classification_kind_t,
    pub expected_revision: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_classification_result_t {
    pub has_annotation: bool,
    pub annotation: observation_classification_t,
    pub observation_revision: ::core::ffi::c_int,
    pub change_seq: int64_t,
    pub observation_asset_uuid: [::core::ffi::c_char; 37],
    pub evidence_state: observation_asset_state_t,
    pub limit: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_classification_set_t {
    pub automated_count: ::core::ffi::c_int,
    pub automated: [observation_classification_t; 8],
    pub owner_confirmed_count: ::core::ffi::c_int,
    pub owner_confirmed: [observation_classification_t; 8],
}
pub type observation_classification_visit_fn = Option<
    unsafe extern "C" fn(
        size_t,
        *const observation_classification_set_t,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_detection_link_t {
    pub linked: bool,
    pub observation_uuid: [::core::ffi::c_char; 37],
    pub has_classification: bool,
    pub owner_confirmed: bool,
    pub classification: observation_classification_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct page_item_context_t {
    pub page: *const observation_page_t,
    pub source: *const ::core::ffi::c_char,
    pub items: *mut cJSON,
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
pub struct authorization_decision_t(pub ::core::ffi::c_uint);
impl authorization_decision_t {
    pub const AUTHZ_DECISION_DENY: Self = Self(0);
    pub const AUTHZ_DECISION_ALLOW: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_evidence_stats_t {
    pub initialized: bool,
    pub worker_running: bool,
    pub allocated: bool,
    pub accepted: uint64_t,
    pub dropped_queue_full: uint64_t,
    pub dropped_budget: uint64_t,
    pub dropped_disabled: uint64_t,
    pub dropped_untracked: uint64_t,
    pub unavailable_submitted: uint64_t,
    pub ready: uint64_t,
    pub failed_encode: uint64_t,
    pub failed_write: uint64_t,
    pub failed_db: uint64_t,
    pub disabled_at_encode: uint64_t,
    pub interrupted: uint64_t,
    pub bytes_written: uint64_t,
    pub backlog_items: uint32_t,
    pub backlog_bytes: uint64_t,
    pub drop_ring_depth: uint32_t,
    pub high_water_items: uint32_t,
    pub high_water_bytes: uint64_t,
    pub budget_exhausted_cameras: uint32_t,
    pub limit_items: uint32_t,
    pub limit_camera_items: uint32_t,
    pub limit_bytes: uint64_t,
    pub limit_camera_bytes: uint64_t,
    pub camera_high_water_items: uint32_t,
    pub submit_latency_count: uint64_t,
    pub submit_latency_sum_ns: uint64_t,
    pub submit_latency_max_ns: uint64_t,
    pub submit_latency_bucket: [uint64_t; 10],
    pub discard_deferred: uint64_t,
    pub discard_inline: uint64_t,
    pub discard_pending_bytes: uint64_t,
    pub limit_discard_bytes: uint64_t,
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
pub struct audit_decision_mode_t(pub ::core::ffi::c_uint);
impl audit_decision_mode_t {
    pub const AUDIT_DECISION_MODE_RECORD: Self = Self(0);
    pub const AUDIT_DECISION_MODE_SUMMARIZE: Self = Self(1);
    pub const AUDIT_DECISION_MODE_OFF: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct put_target_t {
    pub id: [::core::ffi::c_char; 37],
    pub ns: [::core::ffi::c_char; 33],
    pub name: [::core::ffi::c_char; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct annotate_audit_t {
    pub req: *const http_request_t,
    pub user: *const user_t,
    pub observation_uuid: *const ::core::ffi::c_char,
    pub camera_uuid: *const ::core::ffi::c_char,
    pub key: [::core::ffi::c_char; 66],
    pub kind: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct put_body_t {
    pub classifier: [::core::ffi::c_char; 65],
    pub model_version: [::core::ffi::c_char; 65],
    pub label: [::core::ffi::c_char; 129],
    pub taxonomy_id: [::core::ffi::c_char; 65],
    pub confidence: ::core::ffi::c_double,
    pub classified_at_ms: int64_t,
    pub input_asset_uuid: [::core::ffi::c_char; 37],
    pub input_revision: ::core::ffi::c_int,
    pub kind: observation_classification_kind_t,
    pub has_kind: bool,
    pub expected_revision: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
#[inline]
unsafe extern "C" fn configured_stream_slots() -> ::core::ffi::c_int {
    let mut slots: ::core::ffi::c_int = g_config.max_streams;
    if slots <= 0 as ::core::ffi::c_int || slots > MAX_STREAMS {
        return MAX_STREAMS;
    }
    return slots;
}
pub const OBS_UUID_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const OBS_MAX_BATCH: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const OBS_LABEL_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const OBS_LIST_DEFAULT_LIMIT: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const OBS_LIST_MAX_LIMIT: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
pub const OBS_QUERY_MAX_LABELS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const OBS_CLS_SEGMENT_MAX: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const OBS_CLS_CLASSIFIER_SIZE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const OBS_CLS_MODEL_VERSION_SIZE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const OBS_CLS_LABEL_SIZE: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const OBS_CLS_TAXONOMY_SIZE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const OBS_WRITER_NAMESPACE_SIZE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const OBS_PUT_BODY_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OBS_SNAPSHOT_RETRY_AFTER_MAX: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const LIGHTNVR_VERSION_STRING: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"0.48.2\0") };
pub const LOG_COMPONENT: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"ObservationsAPI\0") };
pub const OBS_SOURCE_TEXT_SIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const OBS_FILTER_CAMERAS_MAX: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const OBS_CAPABILITY_CAMERAS_MAX: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const OBS_SNAPSHOT_RETRY_MAX: ::core::ffi::c_int = OBS_SNAPSHOT_RETRY_AFTER_MAX;
pub const OBS_SNAPSHOT_CACHE_CONTROL: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"private, max-age=300\0")
};
pub const OBS_HISTORY_RETENTION_DEFAULT: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const OBS_EVENT_TYPE: [::core::ffi::c_char; 45] = unsafe {
    ::core::mem::transmute::<[u8; 45], [::core::ffi::c_char; 45]>(
        *b"io.lightnvr.detection.observation_changed.v1\0",
    )
};
pub const OBS_ANNOTATIONS_MAX: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const URN_PREFIX: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"urn:lightnvr:\0") };
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_format_time(
    mut ms: int64_t,
    mut out: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> ::core::ffi::c_int {
    if out.is_null() || out_size < 25 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut seconds: int64_t = ms / 1000 as int64_t;
    let mut millis: ::core::ffi::c_int = (ms % 1000 as int64_t) as ::core::ffi::c_int;
    if millis < 0 as ::core::ffi::c_int {
        millis += 1000 as ::core::ffi::c_int;
        seconds -= 1;
    }
    let mut t: time_t = seconds as time_t;
    let mut tm: tm = tm {
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
    if gmtime_r(&raw mut t, &raw mut tm).is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut n: ::core::ffi::c_int = snprintf(
        out,
        out_size,
        b"%04d-%02d-%02dT%02d:%02d:%02d.%03dZ\0".as_ptr() as *const ::core::ffi::c_char,
        tm.tm_year + 1900 as ::core::ffi::c_int,
        tm.tm_mon + 1 as ::core::ffi::c_int,
        tm.tm_mday,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec,
        millis,
    );
    return if n > 0 as ::core::ffi::c_int && (n as size_t) < out_size {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn digits(
    mut text: *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
    mut value: *mut ::core::ffi::c_int,
) -> bool {
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if *(*__ctype_b_loc())
            .offset(*text.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed_1::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        {
            return r#false != 0;
        }
        result = result * 10 as ::core::ffi::c_int
            + (*text.offset(i as isize) as ::core::ffi::c_int - '0' as ::core::ffi::c_int);
        i += 1;
    }
    *value = result;
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_parse_time(
    mut text: *const ::core::ffi::c_char,
    mut ms: *mut int64_t,
) -> ::core::ffi::c_int {
    if text.is_null() || ms.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut year: ::core::ffi::c_int = 0;
    let mut month: ::core::ffi::c_int = 0;
    let mut day: ::core::ffi::c_int = 0;
    let mut hour: ::core::ffi::c_int = 0;
    let mut minute: ::core::ffi::c_int = 0;
    let mut second: ::core::ffi::c_int = 0;
    if strnlen(text, 20 as size_t) < 20 as size_t
        || !digits(text, 4 as ::core::ffi::c_int, &raw mut year)
        || *text.offset(4isize) as ::core::ffi::c_int != '-' as ::core::ffi::c_int
        || !digits(
            text.offset(5 as ::core::ffi::c_int as isize),
            2 as ::core::ffi::c_int,
            &raw mut month,
        )
        || *text.offset(7isize) as ::core::ffi::c_int != '-' as ::core::ffi::c_int
        || !digits(
            text.offset(8 as ::core::ffi::c_int as isize),
            2 as ::core::ffi::c_int,
            &raw mut day,
        )
        || *text.offset(10isize) as ::core::ffi::c_int != 'T' as ::core::ffi::c_int
            && *text.offset(10isize) as ::core::ffi::c_int != 't' as ::core::ffi::c_int
        || !digits(
            text.offset(11 as ::core::ffi::c_int as isize),
            2 as ::core::ffi::c_int,
            &raw mut hour,
        )
        || *text.offset(13isize) as ::core::ffi::c_int != ':' as ::core::ffi::c_int
        || !digits(
            text.offset(14 as ::core::ffi::c_int as isize),
            2 as ::core::ffi::c_int,
            &raw mut minute,
        )
        || *text.offset(16isize) as ::core::ffi::c_int != ':' as ::core::ffi::c_int
        || !digits(
            text.offset(17 as ::core::ffi::c_int as isize),
            2 as ::core::ffi::c_int,
            &raw mut second,
        )
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut p: *const ::core::ffi::c_char = text.offset(19 as ::core::ffi::c_int as isize);
    let mut millis: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *p as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
        p = p.offset(1);
        let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed_1::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            if count < 3 as ::core::ffi::c_int {
                millis = millis * 10 as ::core::ffi::c_int
                    + (*p as ::core::ffi::c_int - '0' as ::core::ffi::c_int);
            }
            count += 1;
            p = p.offset(1);
        }
        if count == 0 as ::core::ffi::c_int || count > 9 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        let mut i: ::core::ffi::c_int = count;
        while i < 3 as ::core::ffi::c_int {
            millis *= 10 as ::core::ffi::c_int;
            i += 1;
        }
    }
    let mut offset_minutes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *p as ::core::ffi::c_int == 'Z' as ::core::ffi::c_int
        || *p as ::core::ffi::c_int == 'z' as ::core::ffi::c_int
    {
        p = p.offset(1);
    } else if *p as ::core::ffi::c_int == '+' as ::core::ffi::c_int
        || *p as ::core::ffi::c_int == '-' as ::core::ffi::c_int
    {
        let mut oh: ::core::ffi::c_int = 0;
        let mut om: ::core::ffi::c_int = 0;
        if !digits(
            p.offset(1 as ::core::ffi::c_int as isize),
            2 as ::core::ffi::c_int,
            &raw mut oh,
        ) || *p.offset(3isize) as ::core::ffi::c_int != ':' as ::core::ffi::c_int
            || !digits(
                p.offset(4 as ::core::ffi::c_int as isize),
                2 as ::core::ffi::c_int,
                &raw mut om,
            )
            || oh > 23 as ::core::ffi::c_int
            || om > 59 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        offset_minutes = (oh * 60 as ::core::ffi::c_int + om)
            * if *p as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
                -1 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
        p = p.offset(6 as ::core::ffi::c_int as isize);
    } else {
        return -1 as ::core::ffi::c_int;
    }
    if *p as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if year < 1970 as ::core::ffi::c_int
        || month < 1 as ::core::ffi::c_int
        || month > 12 as ::core::ffi::c_int
        || day < 1 as ::core::ffi::c_int
        || day > 31 as ::core::ffi::c_int
        || hour > 23 as ::core::ffi::c_int
        || minute > 59 as ::core::ffi::c_int
        || second > 59 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut tm: tm = tm {
        tm_sec: 0 as ::core::ffi::c_int,
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
    tm.tm_year = year - 1900 as ::core::ffi::c_int;
    tm.tm_mon = month - 1 as ::core::ffi::c_int;
    tm.tm_mday = day;
    tm.tm_hour = hour;
    tm.tm_min = minute;
    tm.tm_sec = second;
    let mut t: time_t = timegm(&raw mut tm);
    let mut check: tm = tm {
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
    if t == -1 as ::core::ffi::c_int as time_t
        || gmtime_r(&raw mut t, &raw mut check).is_null()
        || check.tm_mday != day
        || check.tm_mon != month - 1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    *ms = (t as int64_t - offset_minutes as int64_t * 60 as int64_t) * 1000 as int64_t
        + millis as int64_t;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_writer_namespace(
    mut username: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut reserved: [*const ::core::ffi::c_char; 4] = [
        b"lightnvr\0".as_ptr() as *const ::core::ffi::c_char,
        b"system\0".as_ptr() as *const ::core::ffi::c_char,
        b"owner\0".as_ptr() as *const ::core::ffi::c_char,
        b"local\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if out.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *out.offset(0isize) = '\0' as ::core::ffi::c_char;
    if username.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut length: size_t = strlen(username);
    if length < 1 as size_t
        || length > (OBS_WRITER_NAMESPACE_SIZE - 1 as ::core::ffi::c_int) as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut normalized: [::core::ffi::c_char; 33] = [0; 33];
    let mut i: size_t = 0 as size_t;
    while i < length {
        let mut c: ::core::ffi::c_uchar = *username.offset(i as isize) as ::core::ffi::c_uchar;
        if c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
            && c as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
        {
            c = (c as ::core::ffi::c_int - 'A' as ::core::ffi::c_int + 'a' as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
        }
        let mut allowed: bool = c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
            && c as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
            || c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '_' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '-' as ::core::ffi::c_int;
        normalized[i] = (if allowed as ::core::ffi::c_int != 0 {
            c as ::core::ffi::c_char as ::core::ffi::c_int
        } else {
            '-' as ::core::ffi::c_int
        }) as ::core::ffi::c_char;
        i = i.wrapping_add(1);
    }
    normalized[length] = '\0' as ::core::ffi::c_char;
    let mut first: ::core::ffi::c_char = normalized[0usize];
    if !(first as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
        && first as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
        || first as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && first as ::core::ffi::c_int <= '9' as ::core::ffi::c_int)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut i_0: size_t = 0 as size_t;
    while i_0
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        if strcmp(
            &raw mut normalized as *mut ::core::ffi::c_char,
            reserved[i_0],
        ) == 0 as ::core::ffi::c_int
        {
            return -2 as ::core::ffi::c_int;
        }
        i_0 = i_0.wrapping_add(1);
    }
    memcpy(
        out as *mut ::core::ffi::c_void,
        &raw mut normalized as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        length.wrapping_add(1 as size_t),
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn time_or_null(mut ms: int64_t) -> *mut cJSON {
    let mut text: [::core::ffi::c_char; 32] = [0; 32];
    if ms <= 0 as int64_t
        || observation_format_time(
            ms,
            &raw mut text as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        ) != 0 as ::core::ffi::c_int
    {
        return cJSON_CreateNull();
    }
    return cJSON_CreateString(&raw mut text as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn text_or_null(mut value: *const ::core::ffi::c_char) -> *mut cJSON {
    return if !value.is_null() && *value.offset(0isize) as ::core::ffi::c_int != 0 {
        cJSON_CreateString(value)
    } else {
        cJSON_CreateNull()
    };
}
unsafe extern "C" fn time_text(mut ms: int64_t) -> *mut cJSON {
    let mut text: [::core::ffi::c_char; 32] = [0; 32];
    if ms < 0 as int64_t
        || observation_format_time(
            ms,
            &raw mut text as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        ) != 0 as ::core::ffi::c_int
    {
        return cJSON_CreateNull();
    }
    return cJSON_CreateString(&raw mut text as *mut ::core::ffi::c_char);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_classification_to_json(
    mut annotation: *const observation_classification_t,
) -> *mut cJSON {
    if annotation.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut json: *mut cJSON = cJSON_CreateObject();
    if json.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        json,
        b"key\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*annotation).key as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        json,
        b"writer_namespace\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*annotation).writer_namespace as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        json,
        b"classifier\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*annotation).classifier as *const ::core::ffi::c_char,
    );
    cJSON_AddItemToObject(
        json,
        b"model_version\0".as_ptr() as *const ::core::ffi::c_char,
        text_or_null(&raw const (*annotation).model_version as *const ::core::ffi::c_char),
    );
    cJSON_AddStringToObject(
        json,
        b"label\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*annotation).label as *const ::core::ffi::c_char,
    );
    cJSON_AddItemToObject(
        json,
        b"taxonomy_id\0".as_ptr() as *const ::core::ffi::c_char,
        text_or_null(&raw const (*annotation).taxonomy_id as *const ::core::ffi::c_char),
    );
    cJSON_AddNumberToObject(
        json,
        b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
        (*annotation).confidence,
    );
    cJSON_AddItemToObject(
        json,
        b"classified_at\0".as_ptr() as *const ::core::ffi::c_char,
        time_text((*annotation).classified_at_ms),
    );
    cJSON_AddStringToObject(
        json,
        b"input_asset_id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*annotation).input_asset_uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        json,
        b"input_revision\0".as_ptr() as *const ::core::ffi::c_char,
        (*annotation).input_revision as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        json,
        b"kind\0".as_ptr() as *const ::core::ffi::c_char,
        observation_classification_kind_name((*annotation).kind),
    );
    cJSON_AddNumberToObject(
        json,
        b"annotation_revision\0".as_ptr() as *const ::core::ffi::c_char,
        (*annotation).annotation_revision as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        json,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        time_text((*annotation).created_at_ms),
    );
    cJSON_AddItemToObject(
        json,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        time_text((*annotation).updated_at_ms),
    );
    return json;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_detection_link_to_json(
    mut link: *const observation_detection_link_t,
) -> *mut cJSON {
    if link.is_null() || !(*link).linked {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut json: *mut cJSON = cJSON_CreateObject();
    if json.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        json,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*link).observation_uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        json,
        b"owner_confirmed\0".as_ptr() as *const ::core::ffi::c_char,
        (*link).owner_confirmed as cJSON_bool,
    );
    let mut classification: *mut cJSON = if (*link).has_classification as ::core::ffi::c_int != 0 {
        observation_classification_to_json(&raw const (*link).classification)
    } else {
        cJSON_CreateNull()
    };
    if classification.is_null()
        || cJSON_AddItemToObject(
            json,
            b"classification\0".as_ptr() as *const ::core::ffi::c_char,
            classification,
        ) == 0
    {
        cJSON_Delete(classification);
        cJSON_Delete(json);
        return ::core::ptr::null_mut::<cJSON>();
    }
    return json;
}
unsafe extern "C" fn add_annotations(
    mut list: *mut cJSON,
    mut items: *const observation_classification_t,
    mut count: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut item: *mut cJSON = observation_classification_to_json(items.offset(i as isize));
        if item.is_null() {
            return r#false != 0;
        }
        cJSON_AddItemToArray(list, item);
        i += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn evidence_to_json(mut record: *const observation_record_t) -> *mut cJSON {
    let mut asset: *const observation_asset_record_t = &raw const (*record).asset;
    let mut evidence: *mut cJSON = cJSON_CreateObject();
    if evidence.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut has_media: bool =
        (*asset).origin[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
    let mut sized: bool = has_media as ::core::ffi::c_int != 0
        && (*asset).state.0 != observation_asset_state_t::OBS_ASSET_PENDING.0;
    let mut reasoned: bool = (*asset).state.0 == observation_asset_state_t::OBS_ASSET_UNAVAILABLE.0
        || (*asset).state.0 == observation_asset_state_t::OBS_ASSET_EXPIRED.0;
    cJSON_AddStringToObject(
        evidence,
        b"asset_id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*asset).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        evidence,
        b"state\0".as_ptr() as *const ::core::ffi::c_char,
        observation_asset_state_name((*asset).state),
    );
    cJSON_AddItemToObject(
        evidence,
        b"reason\0".as_ptr() as *const ::core::ffi::c_char,
        text_or_null(if reasoned as ::core::ffi::c_int != 0 {
            &raw const (*asset).reason as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        }),
    );
    cJSON_AddItemToObject(
        evidence,
        b"origin\0".as_ptr() as *const ::core::ffi::c_char,
        text_or_null(if has_media as ::core::ffi::c_int != 0 {
            &raw const (*asset).origin as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        }),
    );
    cJSON_AddItemToObject(
        evidence,
        b"media_type\0".as_ptr() as *const ::core::ffi::c_char,
        text_or_null(if has_media as ::core::ffi::c_int != 0 {
            &raw const (*asset).media_type as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        }),
    );
    if sized as ::core::ffi::c_int != 0 && (*asset).width > 0 as ::core::ffi::c_int {
        cJSON_AddNumberToObject(
            evidence,
            b"width\0".as_ptr() as *const ::core::ffi::c_char,
            (*asset).width as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(evidence, b"width\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if sized as ::core::ffi::c_int != 0 && (*asset).height > 0 as ::core::ffi::c_int {
        cJSON_AddNumberToObject(
            evidence,
            b"height\0".as_ptr() as *const ::core::ffi::c_char,
            (*asset).height as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(evidence, b"height\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if sized as ::core::ffi::c_int != 0 && (*asset).has_bytes as ::core::ffi::c_int != 0 {
        cJSON_AddNumberToObject(
            evidence,
            b"bytes\0".as_ptr() as *const ::core::ffi::c_char,
            (*asset).bytes as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(evidence, b"bytes\0".as_ptr() as *const ::core::ffi::c_char);
    }
    cJSON_AddItemToObject(
        evidence,
        b"captured_at\0".as_ptr() as *const ::core::ffi::c_char,
        if has_media as ::core::ffi::c_int != 0 {
            time_or_null((*asset).captured_at_ms)
        } else {
            cJSON_CreateNull()
        },
    );
    cJSON_AddItemToObject(
        evidence,
        b"expires_at\0".as_ptr() as *const ::core::ffi::c_char,
        if sized as ::core::ffi::c_int != 0 {
            time_or_null((*asset).expires_at_ms)
        } else {
            cJSON_CreateNull()
        },
    );
    let mut transform: *mut cJSON = if has_media as ::core::ffi::c_int != 0
        && (*asset).transform_json[0usize] as ::core::ffi::c_int != 0
    {
        cJSON_Parse(&raw const (*asset).transform_json as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if !transform.is_null() && cJSON_IsObject(transform) == 0 {
        cJSON_Delete(transform);
        transform = ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddItemToObject(
        evidence,
        b"transform\0".as_ptr() as *const ::core::ffi::c_char,
        if !transform.is_null() {
            transform
        } else {
            cJSON_CreateNull()
        },
    );
    if (*asset).state.0 == observation_asset_state_t::OBS_ASSET_READY.0 {
        let mut url: [::core::ffi::c_char; 96] = [0; 96];
        snprintf(
            &raw mut url as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
            b"/api/observations/%s/snapshot\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*record).uuid as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            evidence,
            b"snapshot_url\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut url as *mut ::core::ffi::c_char,
        );
    }
    return evidence;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_record_to_json(
    mut record: *const observation_record_t,
    mut source: *const ::core::ffi::c_char,
    mut classifications: *const observation_classification_set_t,
) -> *mut cJSON {
    if record.is_null() || source.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        root,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*record).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"source\0".as_ptr() as *const ::core::ffi::c_char,
        source,
    );
    cJSON_AddStringToObject(
        root,
        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*record).camera_uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"stream_name_at_capture\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*record).stream_name_at_capture as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"batch_id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*record).batch_id as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        root,
        b"object_index\0".as_ptr() as *const ::core::ffi::c_char,
        (*record).object_index as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"observed_at\0".as_ptr() as *const ::core::ffi::c_char,
        time_or_null((*record).observed_at_ms),
    );
    cJSON_AddItemToObject(
        root,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        time_or_null((*record).created_at_ms),
    );
    let mut detector: *mut cJSON =
        cJSON_AddObjectToObject(root, b"detector\0".as_ptr() as *const ::core::ffi::c_char);
    if !detector.is_null() {
        cJSON_AddStringToObject(
            detector,
            b"engine_key\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*record).engine_key as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            detector,
            b"engine_type\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*record).engine_type as *const ::core::ffi::c_char,
        );
        cJSON_AddItemToObject(
            detector,
            b"model\0".as_ptr() as *const ::core::ffi::c_char,
            text_or_null(&raw const (*record).model as *const ::core::ffi::c_char),
        );
        cJSON_AddItemToObject(
            detector,
            b"model_version\0".as_ptr() as *const ::core::ffi::c_char,
            text_or_null(&raw const (*record).model_version as *const ::core::ffi::c_char),
        );
    }
    cJSON_AddStringToObject(
        root,
        b"label\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*record).label as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        root,
        b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
        (*record).confidence,
    );
    if (*record).has_box {
        let mut r#box: *mut cJSON =
            cJSON_AddObjectToObject(root, b"box\0".as_ptr() as *const ::core::ffi::c_char);
        if !r#box.is_null() {
            cJSON_AddStringToObject(
                r#box,
                b"format\0".as_ptr() as *const ::core::ffi::c_char,
                b"xywh_normalized\0".as_ptr() as *const ::core::ffi::c_char,
            );
            cJSON_AddNumberToObject(
                r#box,
                b"x\0".as_ptr() as *const ::core::ffi::c_char,
                (*record).box_x,
            );
            cJSON_AddNumberToObject(
                r#box,
                b"y\0".as_ptr() as *const ::core::ffi::c_char,
                (*record).box_y,
            );
            cJSON_AddNumberToObject(
                r#box,
                b"width\0".as_ptr() as *const ::core::ffi::c_char,
                (*record).box_width,
            );
            cJSON_AddNumberToObject(
                r#box,
                b"height\0".as_ptr() as *const ::core::ffi::c_char,
                (*record).box_height,
            );
        }
    } else {
        cJSON_AddNullToObject(root, b"box\0".as_ptr() as *const ::core::ffi::c_char);
    }
    let mut evidence: *mut cJSON = evidence_to_json(record);
    if !evidence.is_null() {
        cJSON_AddItemToObject(
            root,
            b"evidence\0".as_ptr() as *const ::core::ffi::c_char,
            evidence,
        );
    }
    if (*record).has_track as ::core::ffi::c_int != 0
        && (*record).track_id[0usize] as ::core::ffi::c_int != 0
    {
        let mut track: *mut cJSON =
            cJSON_AddObjectToObject(root, b"track\0".as_ptr() as *const ::core::ffi::c_char);
        if !track.is_null() {
            cJSON_AddStringToObject(
                track,
                b"id\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*record).track_id as *const ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                track,
                b"namespace\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*record).track_namespace as *const ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                track,
                b"session\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*record).track_session as *const ::core::ffi::c_char,
            );
        }
    } else {
        cJSON_AddNullToObject(root, b"track\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if (*record).recording_id > 0 as int64_t {
        let mut recording: *mut cJSON = cJSON_AddObjectToObject(
            root,
            b"recording_reference\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !recording.is_null() {
            cJSON_AddNumberToObject(
                recording,
                b"recording_id\0".as_ptr() as *const ::core::ffi::c_char,
                (*record).recording_id as ::core::ffi::c_double,
            );
        }
    } else {
        cJSON_AddNullToObject(
            root,
            b"recording_reference\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddNumberToObject(
        root,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        (*record).revision as ::core::ffi::c_double,
    );
    let mut lists: *mut cJSON = cJSON_AddObjectToObject(
        root,
        b"classifications\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut listed: bool = r#false != 0;
    if !lists.is_null() {
        let mut automated: *mut cJSON =
            cJSON_AddArrayToObject(lists, b"automated\0".as_ptr() as *const ::core::ffi::c_char);
        let mut owner: *mut cJSON = cJSON_AddArrayToObject(
            lists,
            b"owner_confirmed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        listed = !automated.is_null() && !owner.is_null();
        if listed as ::core::ffi::c_int != 0 && !classifications.is_null() {
            listed = add_annotations(
                automated,
                &raw const (*classifications).automated as *const observation_classification_t,
                (*classifications).automated_count,
            ) as ::core::ffi::c_int
                != 0
                && add_annotations(
                    owner,
                    &raw const (*classifications).owner_confirmed
                        as *const observation_classification_t,
                    (*classifications).owner_confirmed_count,
                ) as ::core::ffi::c_int
                    != 0;
        }
    }
    if detector.is_null() || evidence.is_null() || !listed {
        cJSON_Delete(root);
        return ::core::ptr::null_mut::<cJSON>();
    }
    return root;
}
unsafe extern "C" fn send_json(
    mut res: *mut http_response_t,
    mut status: ::core::ffi::c_int,
    mut body: *mut cJSON,
) {
    let mut json: *mut ::core::ffi::c_char = if !body.is_null() {
        cJSON_PrintUnformatted(body)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    cJSON_Delete(body);
    if json.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Internal server error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, status, json);
    cJSON_free(json as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn error_body(
    mut message: *const ::core::ffi::c_char,
    mut code: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut body: *mut cJSON = cJSON_CreateObject();
    if !body.is_null() {
        cJSON_AddStringToObject(
            body,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            message,
        );
        cJSON_AddStringToObject(body, b"code\0".as_ptr() as *const ::core::ffi::c_char, code);
    }
    return body;
}
unsafe extern "C" fn send_error(
    mut res: *mut http_response_t,
    mut status: ::core::ffi::c_int,
    mut code: *const ::core::ffi::c_char,
    mut message: *const ::core::ffi::c_char,
) {
    send_json(res, status, error_body(message, code));
}
unsafe extern "C" fn send_invalid(
    mut res: *mut http_response_t,
    mut field: *const ::core::ffi::c_char,
    mut message: *const ::core::ffi::c_char,
) {
    let mut body: *mut cJSON = error_body(
        message,
        b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !body.is_null() && !field.is_null() {
        cJSON_AddStringToObject(
            body,
            b"field\0".as_ptr() as *const ::core::ffi::c_char,
            field,
        );
    }
    send_json(res, 400 as ::core::ffi::c_int, body);
}
unsafe extern "C" fn send_internal(mut res: *mut http_response_t) {
    send_error(
        res,
        500 as ::core::ffi::c_int,
        b"internal_error\0".as_ptr() as *const ::core::ffi::c_char,
        b"Internal server error\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn send_not_found(mut res: *mut http_response_t) {
    send_error(
        res,
        404 as ::core::ffi::c_int,
        b"observation_not_found\0".as_ptr() as *const ::core::ffi::c_char,
        b"Observation not found\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn send_unavailable(mut res: *mut http_response_t) {
    http_response_add_header(
        res,
        b"Retry-After\0".as_ptr() as *const ::core::ffi::c_char,
        b"5\0".as_ptr() as *const ::core::ffi::c_char,
    );
    send_error(
        res,
        503 as ::core::ffi::c_int,
        b"service_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        b"Service temporarily unavailable\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn authenticate(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut user: *mut user_t,
) -> bool {
    memset(
        user as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    if httpd_check_action_access(req, user) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn installation_source(mut source: *mut ::core::ffi::c_char) -> bool {
    return event_identity_get_source(source, OBS_SOURCE_TEXT_SIZE as size_t)
        == 0 as ::core::ffi::c_int
        && strncmp(source, URN_PREFIX.as_ptr(), strlen(URN_PREFIX.as_ptr()))
            == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn lowercase(mut text: *mut ::core::ffi::c_char) {
    while *text != 0 {
        if *text as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
            && *text as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
        {
            *text = (*text as ::core::ffi::c_int - 'A' as ::core::ffi::c_int
                + 'a' as ::core::ffi::c_int) as ::core::ffi::c_char;
        }
        text = text.offset(1);
    }
}
unsafe extern "C" fn path_observation_id(
    mut req: *const http_request_t,
    mut suffix: *const ::core::ffi::c_char,
    mut id: *mut ::core::ffi::c_char,
) -> bool {
    let mut param: [::core::ffi::c_char; 256] = [0; 256];
    if http_request_extract_path_param(
        req,
        b"/api/observations/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut param as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    if !suffix.is_null() {
        let mut length: size_t = strlen(&raw mut param as *mut ::core::ffi::c_char);
        let mut suffix_length: size_t = strlen(suffix);
        if length <= suffix_length
            || strcmp(
                (&raw mut param as *mut ::core::ffi::c_char)
                    .offset(length as isize)
                    .offset(-(suffix_length as isize)),
                suffix,
            ) != 0 as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        param[length.wrapping_sub(suffix_length)] = '\0' as ::core::ffi::c_char;
    }
    lowercase(&raw mut param as *mut ::core::ffi::c_char);
    if !lightnvr_uuid_is_valid(&raw mut param as *mut ::core::ffi::c_char)
        || strlen(&raw mut param as *mut ::core::ffi::c_char)
            != (OBS_UUID_SIZE - 1 as ::core::ffi::c_int) as size_t
    {
        return r#false != 0;
    }
    memcpy(
        id as *mut ::core::ffi::c_void,
        &raw mut param as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        OBS_UUID_SIZE as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn audit_camera_decision(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut action: authorization_action_t,
    mut camera_uuid: *const ::core::ffi::c_char,
    mut evaluation: *const authorization_evaluation_t,
    mut outcome: *const ::core::ffi::c_char,
) {
    let mut camera: *mut fleet_camera_t =
        calloc(1 as size_t, ::core::mem::size_of::<fleet_camera_t>()) as *mut fleet_camera_t;
    if camera.is_null() {
        return;
    }
    safe_strcpy(
        &raw mut (*camera).camera_uuid as *mut ::core::ffi::c_char,
        camera_uuid,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    audit_log_authorization(req, user, action, camera, evaluation, outcome);
    free(camera as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn audit_snapshot(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut record: *const observation_record_t,
    mut evaluation: *const authorization_evaluation_t,
    mut outcome: *const ::core::ffi::c_char,
) {
    if strcmp(outcome, b"allowed\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        let mut mode: audit_decision_mode_t =
            audit_log_get_decision_mode(authorization_action_t::AUTHZ_SNAPSHOT_CREATE);
        if mode.0 == audit_decision_mode_t::AUDIT_DECISION_MODE_OFF.0 {
            return;
        }
        if mode.0 == audit_decision_mode_t::AUDIT_DECISION_MODE_SUMMARIZE.0 {
            audit_camera_decision(
                req,
                user,
                authorization_action_t::AUTHZ_SNAPSHOT_CREATE,
                &raw const (*record).camera_uuid as *const ::core::ffi::c_char,
                evaluation,
                outcome,
            );
            return;
        }
    }
    let mut metadata: *const authorization_action_metadata_t =
        authorization_action_metadata(authorization_action_t::AUTHZ_SNAPSHOT_CREATE);
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
            b"authorization.decision\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"method\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*req).method_str as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"path\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*req).path as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*record).camera_uuid as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"asset_id\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*record).asset.uuid as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"evidence_state\0".as_ptr() as *const ::core::ffi::c_char,
            observation_asset_state_name((*record).asset.state),
        );
        if !evaluation.is_null() {
            cJSON_AddStringToObject(
                details,
                b"decision_source\0".as_ptr() as *const ::core::ffi::c_char,
                authorization_decision_source_name((*evaluation).source),
            );
            cJSON_AddStringToObject(
                details,
                b"explanation\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*evaluation).explanation as *const ::core::ffi::c_char,
            );
        }
    }
    audit_log_append(
        req,
        user,
        if !metadata.is_null() {
            (*metadata).key
        } else {
            b"snapshot.create\0".as_ptr() as *const ::core::ffi::c_char
        },
        b"observation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*record).uuid as *const ::core::ffi::c_char,
        outcome,
        details,
    );
    cJSON_Delete(details);
}
unsafe extern "C" fn authorized_cameras(
    mut user: *const user_t,
    mut action: authorization_action_t,
    mut cameras: *mut *mut fleet_camera_t,
    mut count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *cameras = ::core::ptr::null_mut::<fleet_camera_t>();
    *count = 0 as ::core::ffi::c_int;
    if db_fleet_camera_load_readonly(cameras, count) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if *count > 0 as ::core::ffi::c_int
        && authorization_filter_cameras(user, action, *cameras, count) != 0 as ::core::ffi::c_int
    {
        free(*cameras as *mut ::core::ffi::c_void);
        *cameras = ::core::ptr::null_mut::<fleet_camera_t>();
        *count = 0 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn camera_in(
    mut cameras: *const fleet_camera_t,
    mut count: ::core::ffi::c_int,
    mut uuid: *const ::core::ffi::c_char,
) -> bool {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if strcmp(
            &raw const (*cameras.offset(i as isize)).camera_uuid as *const ::core::ffi::c_char,
            uuid,
        ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn add_operation(
    mut operations: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut supported: bool,
    mut authorized: bool,
    mut reason: *const ::core::ffi::c_char,
) {
    let mut op: *mut cJSON = cJSON_AddObjectToObject(operations, name);
    if op.is_null() {
        return;
    }
    cJSON_AddBoolToObject(
        op,
        b"supported\0".as_ptr() as *const ::core::ffi::c_char,
        supported as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        op,
        b"authorized\0".as_ptr() as *const ::core::ffi::c_char,
        authorized as cJSON_bool,
    );
    if !reason.is_null() && (!supported || !authorized) {
        cJSON_AddStringToObject(
            op,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        );
    }
}
unsafe extern "C" fn history_retention_days() -> ::core::ffi::c_int {
    let mut value: [::core::ffi::c_char; 32] = [
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
    if db_get_system_setting(
        b"observation_history_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as ::core::ffi::c_int,
    ) != 0 as ::core::ffi::c_int
    {
        return OBS_HISTORY_RETENTION_DEFAULT;
    }
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut days: ::core::ffi::c_long = strtol(
        &raw mut value as *mut ::core::ffi::c_char,
        &raw mut end,
        10 as ::core::ffi::c_int,
    );
    if *__errno_location() != 0 as ::core::ffi::c_int
        || end == &raw mut value as *mut ::core::ffi::c_char
        || *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || days < 1 as ::core::ffi::c_long
        || days > 365 as ::core::ffi::c_long
    {
        return OBS_HISTORY_RETENTION_DEFAULT;
    }
    return days as ::core::ffi::c_int;
}
unsafe extern "C" fn history_bounds(mut history: *mut cJSON) -> ::core::ffi::c_int {
    let mut meta: observation_meta_t = observation_meta_t {
        epoch_uuid: [0; 37],
        oldest_seq: 0,
        head_seq: 0,
    };
    if db_observation_meta_get(&raw mut meta) != C2Rust_Unnamed::DB_OBS_OK.0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut oldest: [::core::ffi::c_char; 257] = [0; 257];
    if observation_cursor_encode(
        meta.oldest_seq - 1 as int64_t,
        &raw mut meta.epoch_uuid as *mut ::core::ffi::c_char,
        &raw mut oldest as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 257]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut other_epoch: [::core::ffi::c_char; 37] = [0; 37];
    loop {
        if lightnvr_uuid_generate_v4(&raw mut other_epoch as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if strcmp(
            &raw mut other_epoch as *mut ::core::ffi::c_char,
            &raw mut meta.epoch_uuid as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            break;
        }
    }
    let mut probe: [::core::ffi::c_char; 257] = [0; 257];
    if observation_cursor_encode(
        0 as int64_t,
        &raw mut other_epoch as *mut ::core::ffi::c_char,
        &raw mut probe as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 257]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut query: observation_list_query_t = observation_list_query_t {
        camera_uuids: ::core::ptr::null::<[::core::ffi::c_char; 37]>(),
        camera_count: 0,
        all_cameras: false,
        labels: [[0; 32]; 8],
        label_count: 0,
        has_since: false,
        since_ms: 0,
        has_until: false,
        until_ms: 0,
        limit: 0,
    };
    memset(
        &raw mut query as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_list_query_t>(),
    );
    let mut page: observation_page_t = observation_page_t {
        items: ::core::ptr::null_mut::<observation_record_t>(),
        count: 0,
        next_cursor: [0; 257],
        has_more: false,
        oldest_cursor: [0; 257],
        has_oldest_observed_at: false,
        oldest_observed_at_ms: 0,
    };
    if db_observations_list(
        &raw mut query,
        &raw mut probe as *mut ::core::ffi::c_char,
        &raw mut page,
    ) != C2Rust_Unnamed::DB_OBS_CURSOR_EXPIRED.0
    {
        db_observation_page_free(&raw mut page);
        return -1 as ::core::ffi::c_int;
    }
    cJSON_AddStringToObject(
        history,
        b"oldest_cursor\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut oldest as *mut ::core::ffi::c_char,
    );
    cJSON_AddItemToObject(
        history,
        b"oldest_observed_at\0".as_ptr() as *const ::core::ffi::c_char,
        if page.has_oldest_observed_at as ::core::ffi::c_int != 0 {
            time_or_null(page.oldest_observed_at_ms)
        } else {
            cJSON_CreateNull()
        },
    );
    db_observation_page_free(&raw mut page);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn camera_evidence_enabled(
    mut streams: *const stream_config_t,
    mut count: ::core::ffi::c_int,
    mut uuid: *const ::core::ffi::c_char,
) -> bool {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !streams.is_null() && i < count {
        if strcmp(
            &raw const (*streams.offset(i as isize)).camera_uuid as *const ::core::ffi::c_char,
            uuid,
        ) == 0 as ::core::ffi::c_int
        {
            return (*streams.offset(i as isize)).observation_evidence_enabled;
        }
        i += 1;
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_integration_capabilities(
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
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    let mut source: [::core::ffi::c_char; 128] = [0; 128];
    if !installation_source(&raw mut source as *mut ::core::ffi::c_char) {
        send_unavailable(res);
        return;
    }
    let mut viewable: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut snapshot: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut annotatable: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut viewable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut snapshot_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut annotatable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if authorized_cameras(
        &raw mut user,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut viewable,
        &raw mut viewable_count,
    ) != 0 as ::core::ffi::c_int
        || authorized_cameras(
            &raw mut user,
            authorization_action_t::AUTHZ_SNAPSHOT_CREATE,
            &raw mut snapshot,
            &raw mut snapshot_count,
        ) != 0 as ::core::ffi::c_int
        || authorized_cameras(
            &raw mut user,
            authorization_action_t::AUTHZ_OBSERVATIONS_ANNOTATE,
            &raw mut annotatable,
            &raw mut annotatable_count,
        ) != 0 as ::core::ffi::c_int
    {
        free(viewable as *mut ::core::ffi::c_void);
        free(snapshot as *mut ::core::ffi::c_void);
        free(annotatable as *mut ::core::ffi::c_void);
        send_error(
            res,
            500 as ::core::ffi::c_int,
            b"internal_error\0".as_ptr() as *const ::core::ffi::c_char,
            b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut slots: ::core::ffi::c_int = configured_stream_slots();
    let mut streams: *mut stream_config_t =
        calloc(slots as size_t, ::core::mem::size_of::<stream_config_t>()) as *mut stream_config_t;
    let mut stream_count: ::core::ffi::c_int = if !streams.is_null() {
        get_all_stream_configs_readonly(streams, slots)
    } else {
        -1 as ::core::ffi::c_int
    };
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut history: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut ok: bool = !root.is_null() && stream_count >= 0 as ::core::ffi::c_int;
    if ok {
        let mut versions: *mut cJSON = cJSON_AddObjectToObject(
            root,
            b"contract_versions\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            versions,
            b"observations\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            versions,
            b"classifications\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int as ::core::ffi::c_double,
        );
        cJSON_AddStringToObject(
            versions,
            b"observation_event\0".as_ptr() as *const ::core::ffi::c_char,
            OBS_EVENT_TYPE.as_ptr(),
        );
        let mut src: *mut cJSON =
            cJSON_AddObjectToObject(root, b"source\0".as_ptr() as *const ::core::ffi::c_char);
        cJSON_AddStringToObject(
            src,
            b"id\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut source as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            src,
            b"kind\0".as_ptr() as *const ::core::ffi::c_char,
            b"lightnvr\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            src,
            b"installation_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            (&raw mut source as *mut ::core::ffi::c_char)
                .offset(strlen(URN_PREFIX.as_ptr()) as isize),
        );
        cJSON_AddStringToObject(
            src,
            b"product_version\0".as_ptr() as *const ::core::ffi::c_char,
            LIGHTNVR_VERSION_STRING.as_ptr(),
        );
        let mut writer: [::core::ffi::c_char; 33] = [0; 33];
        let mut namespace_rc: ::core::ffi::c_int = observation_writer_namespace(
            &raw mut user.username as *mut ::core::ffi::c_char,
            &raw mut writer as *mut ::core::ffi::c_char,
        );
        let mut principal: *mut cJSON =
            cJSON_AddObjectToObject(root, b"principal\0".as_ptr() as *const ::core::ffi::c_char);
        cJSON_AddItemToObject(
            principal,
            b"writer_namespace\0".as_ptr() as *const ::core::ffi::c_char,
            if namespace_rc == 0 as ::core::ffi::c_int {
                cJSON_CreateString(&raw mut writer as *mut ::core::ffi::c_char)
            } else {
                cJSON_CreateNull()
            },
        );
        let mut operations: *mut cJSON =
            cJSON_AddObjectToObject(root, b"operations\0".as_ptr() as *const ::core::ffi::c_char);
        add_operation(
            operations,
            b"observations.history\0".as_ptr() as *const ::core::ffi::c_char,
            r#true != 0,
            viewable_count > 0 as ::core::ffi::c_int,
            b"not_authorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        add_operation(
            operations,
            b"observations.snapshot\0".as_ptr() as *const ::core::ffi::c_char,
            r#true != 0,
            snapshot_count > 0 as ::core::ffi::c_int,
            b"not_authorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        add_operation(
            operations,
            b"observations.annotate\0".as_ptr() as *const ::core::ffi::c_char,
            r#true != 0,
            namespace_rc == 0 as ::core::ffi::c_int && annotatable_count > 0 as ::core::ffi::c_int,
            if namespace_rc == -2 as ::core::ffi::c_int {
                b"reserved_writer_namespace\0".as_ptr() as *const ::core::ffi::c_char
            } else if namespace_rc != 0 as ::core::ffi::c_int {
                b"invalid_writer_namespace\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"not_authorized\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        add_operation(
            operations,
            b"track_lifecycle\0".as_ptr() as *const ::core::ffi::c_char,
            r#false != 0,
            r#false != 0,
            b"not_implemented\0".as_ptr() as *const ::core::ffi::c_char,
        );
        add_operation(
            operations,
            b"recorded_frame\0".as_ptr() as *const ::core::ffi::c_char,
            r#false != 0,
            r#false != 0,
            b"not_implemented\0".as_ptr() as *const ::core::ffi::c_char,
        );
        add_operation(
            operations,
            b"event_clip\0".as_ptr() as *const ::core::ffi::c_char,
            r#false != 0,
            r#false != 0,
            b"not_implemented\0".as_ptr() as *const ::core::ffi::c_char,
        );
        add_operation(
            operations,
            b"range_clip\0".as_ptr() as *const ::core::ffi::c_char,
            r#false != 0,
            r#false != 0,
            b"not_implemented\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut limits: *mut cJSON =
            cJSON_AddObjectToObject(root, b"limits\0".as_ptr() as *const ::core::ffi::c_char);
        cJSON_AddNumberToObject(
            limits,
            b"history_page_default\0".as_ptr() as *const ::core::ffi::c_char,
            OBS_LIST_DEFAULT_LIMIT as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            limits,
            b"history_page_max\0".as_ptr() as *const ::core::ffi::c_char,
            OBS_LIST_MAX_LIMIT as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            limits,
            b"history_filter_cameras_max\0".as_ptr() as *const ::core::ffi::c_char,
            OBS_FILTER_CAMERAS_MAX as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            limits,
            b"history_filter_labels_max\0".as_ptr() as *const ::core::ffi::c_char,
            OBS_QUERY_MAX_LABELS as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            limits,
            b"max_objects_per_batch\0".as_ptr() as *const ::core::ffi::c_char,
            OBS_MAX_BATCH as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            limits,
            b"label_max_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            (OBS_LABEL_SIZE - 1 as ::core::ffi::c_int) as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            limits,
            b"snapshot_retry_after_max_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            OBS_SNAPSHOT_RETRY_MAX as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            limits,
            b"annotations_per_observation_max\0".as_ptr() as *const ::core::ffi::c_char,
            OBS_ANNOTATIONS_MAX as ::core::ffi::c_double,
        );
        history =
            cJSON_AddObjectToObject(root, b"history\0".as_ptr() as *const ::core::ffi::c_char);
        cJSON_AddNumberToObject(
            history,
            b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
            history_retention_days() as ::core::ffi::c_double,
        );
        ok = !history.is_null() && history_bounds(history) == 0 as ::core::ffi::c_int;
        let mut delivery: *mut cJSON =
            cJSON_AddObjectToObject(root, b"delivery\0".as_ptr() as *const ::core::ffi::c_char);
        cJSON_AddBoolToObject(
            delivery,
            b"mqtt_enabled\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.mqtt_enabled as cJSON_bool,
        );
        if g_config.mqtt_topic_prefix[0usize] != 0 {
            let mut topic: [::core::ffi::c_char; 256] = [0; 256];
            snprintf(
                &raw mut topic as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                b"%s/v1/events/%s/{camera_uuid}\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut g_config.mqtt_topic_prefix as *mut ::core::ffi::c_char,
                OBS_EVENT_TYPE.as_ptr(),
            );
            cJSON_AddStringToObject(
                delivery,
                b"topic\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut topic as *mut ::core::ffi::c_char,
            );
        } else {
            cJSON_AddNullToObject(delivery, b"topic\0".as_ptr() as *const ::core::ffi::c_char);
        }
        let mut cameras: *mut cJSON =
            cJSON_AddArrayToObject(root, b"cameras\0".as_ptr() as *const ::core::ffi::c_char);
        let mut all: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
        let mut all_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if db_fleet_camera_load_readonly(&raw mut all, &raw mut all_count)
            != 0 as ::core::ffi::c_int
        {
            ok = r#false != 0;
        }
        let mut listed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while ok as ::core::ffi::c_int != 0 && i < all_count && listed < OBS_CAPABILITY_CAMERAS_MAX
        {
            let mut uuid: *const ::core::ffi::c_char =
                &raw mut (*all.offset(i as isize)).camera_uuid as *mut ::core::ffi::c_char;
            let mut live: bool = camera_in(viewable, viewable_count, uuid);
            let mut snap: bool = camera_in(snapshot, snapshot_count, uuid);
            let mut annotate: bool = camera_in(annotatable, annotatable_count, uuid);
            if !(!live && !snap && !annotate) {
                let mut camera: *mut cJSON = cJSON_CreateObject();
                cJSON_AddStringToObject(
                    camera,
                    b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                    uuid,
                );
                cJSON_AddBoolToObject(
                    camera,
                    b"evidence_enabled\0".as_ptr() as *const ::core::ffi::c_char,
                    camera_evidence_enabled(streams, stream_count, uuid) as cJSON_bool,
                );
                let mut actions: *mut cJSON = cJSON_AddArrayToObject(
                    camera,
                    b"actions\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if live {
                    cJSON_AddItemToArray(
                        actions,
                        cJSON_CreateString(b"live.view\0".as_ptr() as *const ::core::ffi::c_char),
                    );
                }
                if snap {
                    cJSON_AddItemToArray(
                        actions,
                        cJSON_CreateString(
                            b"snapshot.create\0".as_ptr() as *const ::core::ffi::c_char
                        ),
                    );
                }
                if annotate {
                    cJSON_AddItemToArray(
                        actions,
                        cJSON_CreateString(
                            b"observations.annotate\0".as_ptr() as *const ::core::ffi::c_char
                        ),
                    );
                }
                cJSON_AddItemToArray(cameras, camera);
                listed += 1;
            }
            i += 1;
        }
        free(all as *mut ::core::ffi::c_void);
    }
    free(streams as *mut ::core::ffi::c_void);
    free(viewable as *mut ::core::ffi::c_void);
    free(snapshot as *mut ::core::ffi::c_void);
    free(annotatable as *mut ::core::ffi::c_void);
    if !ok {
        cJSON_Delete(root);
        send_internal(res);
        return;
    }
    send_json(res, 200 as ::core::ffi::c_int, root);
}
unsafe extern "C" fn query_has(
    mut query: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
) -> bool {
    let mut length: size_t = strlen(name);
    let mut p: *const ::core::ffi::c_char = query;
    while !p.is_null() && *p as ::core::ffi::c_int != 0 {
        if strncmp(p, name, length) == 0 as ::core::ffi::c_int
            && *p.offset(length as isize) as ::core::ffi::c_int == '=' as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        p = strchr(p, '&' as ::core::ffi::c_int);
        if !p.is_null() {
            p = p.offset(1);
        }
    }
    return r#false != 0;
}
unsafe extern "C" fn query_value(
    mut req: *const http_request_t,
    mut name: *const ::core::ffi::c_char,
    mut value: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_int {
    *value.offset(0isize) = '\0' as ::core::ffi::c_char;
    if !query_has(
        &raw const (*req).query_string as *const ::core::ffi::c_char,
        name,
    ) {
        return -1 as ::core::ffi::c_int;
    }
    let mut n: ::core::ffi::c_int = http_request_get_query_param(req, name, value, size);
    return if n < 0 as ::core::ffi::c_int {
        -2 as ::core::ffi::c_int
    } else {
        n
    };
}
unsafe extern "C" fn parse_limit(
    mut text: *const ::core::ffi::c_char,
    mut limit: *mut ::core::ffi::c_int,
) -> bool {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut value: ::core::ffi::c_long = strtol(text, &raw mut end, 10 as ::core::ffi::c_int);
    if *__errno_location() != 0 as ::core::ffi::c_int
        || end == text as *mut ::core::ffi::c_char
        || *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || *(*__ctype_b_loc())
            .offset(*text.offset(0isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed_1::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
    {
        return r#false != 0;
    }
    if value < 1 as ::core::ffi::c_long || value > OBS_LIST_MAX_LIMIT as ::core::ffi::c_long {
        return r#false != 0;
    }
    *limit = value as ::core::ffi::c_int;
    return r#true != 0;
}
unsafe extern "C" fn parse_labels(
    mut text: *mut ::core::ffi::c_char,
    mut query: *mut observation_list_query_t,
) -> bool {
    let mut count: size_t = 0 as size_t;
    let mut start: *mut ::core::ffi::c_char = text;
    loop {
        let mut comma: *mut ::core::ffi::c_char = strchr(start, ',' as ::core::ffi::c_int);
        let mut length: size_t = if !comma.is_null() {
            comma.offset_from(start) as size_t
        } else {
            strlen(start)
        };
        if length < 1 as size_t
            || length > (OBS_LABEL_SIZE - 1 as ::core::ffi::c_int) as size_t
            || count >= OBS_QUERY_MAX_LABELS as size_t
        {
            return r#false != 0;
        }
        let mut i: size_t = 0 as size_t;
        while i < length {
            if (*start.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int)
                < 0x20 as ::core::ffi::c_int
                || *start.offset(i as isize) as ::core::ffi::c_int == 0x7f as ::core::ffi::c_int
            {
                return r#false != 0;
            }
            i = i.wrapping_add(1);
        }
        memcpy(
            &raw mut *(&raw mut (*query).labels as *mut [::core::ffi::c_char; 32])
                .offset(count as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            start as *const ::core::ffi::c_void,
            length,
        );
        (*query).labels[count][length] = '\0' as ::core::ffi::c_char;
        count = count.wrapping_add(1);
        if comma.is_null() {
            break;
        }
        start = comma.offset(1 as ::core::ffi::c_int as isize);
    }
    (*query).label_count = count;
    return r#true != 0;
}
unsafe extern "C" fn parse_cameras(
    mut text: *mut ::core::ffi::c_char,
    mut cameras: *mut [::core::ffi::c_char; 37],
    mut count: *mut size_t,
) -> bool {
    *count = 0 as size_t;
    let mut start: *mut ::core::ffi::c_char = text;
    loop {
        let mut comma: *mut ::core::ffi::c_char = strchr(start, ',' as ::core::ffi::c_int);
        let mut length: size_t = if !comma.is_null() {
            comma.offset_from(start) as size_t
        } else {
            strlen(start)
        };
        if length != (OBS_UUID_SIZE - 1 as ::core::ffi::c_int) as size_t
            || *count >= OBS_FILTER_CAMERAS_MAX as size_t
        {
            return r#false != 0;
        }
        memcpy(
            &raw mut *cameras.offset(*count as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            start as *const ::core::ffi::c_void,
            length,
        );
        (*cameras.offset(*count as isize))[length] = '\0' as ::core::ffi::c_char;
        lowercase(&raw mut *cameras.offset(*count as isize) as *mut ::core::ffi::c_char);
        if !lightnvr_uuid_is_valid(
            &raw mut *cameras.offset(*count as isize) as *mut ::core::ffi::c_char
        ) {
            return r#false != 0;
        }
        *count = (*count).wrapping_add(1);
        if comma.is_null() {
            break;
        }
        start = comma.offset(1 as ::core::ffi::c_int as isize);
    }
    return r#true != 0;
}
unsafe extern "C" fn send_cursor_expired(
    mut res: *mut http_response_t,
    mut page: *const observation_page_t,
) {
    let mut body: *mut cJSON = error_body(
        b"Cursor is older than retained observation history\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"cursor_expired\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !body.is_null() {
        cJSON_AddStringToObject(
            body,
            b"oldest_cursor\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*page).oldest_cursor as *const ::core::ffi::c_char,
        );
        cJSON_AddItemToObject(
            body,
            b"oldest_observed_at\0".as_ptr() as *const ::core::ffi::c_char,
            if (*page).has_oldest_observed_at as ::core::ffi::c_int != 0 {
                time_or_null((*page).oldest_observed_at_ms)
            } else {
                cJSON_CreateNull()
            },
        );
    }
    send_json(res, 410 as ::core::ffi::c_int, body);
}
unsafe extern "C" fn add_page_item(
    mut index: size_t,
    mut set: *const observation_classification_set_t,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut page: *mut page_item_context_t = context as *mut page_item_context_t;
    let mut item: *mut cJSON = observation_record_to_json(
        (*(*page).page).items.offset(index as isize),
        (*page).source,
        set,
    );
    if item.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    cJSON_AddItemToArray((*page).items, item);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_observations(
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
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    let mut source: [::core::ffi::c_char; 128] = [0; 128];
    if !installation_source(&raw mut source as *mut ::core::ffi::c_char) {
        send_unavailable(res);
        return;
    }
    let mut query: observation_list_query_t = observation_list_query_t {
        camera_uuids: ::core::ptr::null::<[::core::ffi::c_char; 37]>(),
        camera_count: 0,
        all_cameras: false,
        labels: [[0; 32]; 8],
        label_count: 0,
        has_since: false,
        since_ms: 0,
        has_until: false,
        until_ms: 0,
        limit: 0,
    };
    memset(
        &raw mut query as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_list_query_t>(),
    );
    let mut value: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut n: ::core::ffi::c_int = 0;
    n = query_value(
        req,
        b"limit\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    );
    if n != -1 as ::core::ffi::c_int
        && (n <= 0 as ::core::ffi::c_int
            || !parse_limit(
                &raw mut value as *mut ::core::ffi::c_char,
                &raw mut query.limit,
            ))
    {
        send_invalid(
            res,
            b"limit\0".as_ptr() as *const ::core::ffi::c_char,
            b"limit must be an integer from 1 to 500\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    n = query_value(
        req,
        b"label\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    );
    if n != -1 as ::core::ffi::c_int
        && (n <= 0 as ::core::ffi::c_int
            || !parse_labels(&raw mut value as *mut ::core::ffi::c_char, &raw mut query))
    {
        send_invalid(
            res,
            b"label\0".as_ptr() as *const ::core::ffi::c_char,
            b"label must list 1 to 8 labels of 1 to 31 bytes\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    n = query_value(
        req,
        b"since\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    );
    if n != -1 as ::core::ffi::c_int {
        if n <= 0 as ::core::ffi::c_int
            || observation_parse_time(
                &raw mut value as *mut ::core::ffi::c_char,
                &raw mut query.since_ms,
            ) != 0 as ::core::ffi::c_int
        {
            send_invalid(
                res,
                b"since\0".as_ptr() as *const ::core::ffi::c_char,
                b"since must be an RFC 3339 time\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        query.has_since = r#true != 0;
    }
    n = query_value(
        req,
        b"until\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    );
    if n != -1 as ::core::ffi::c_int {
        if n <= 0 as ::core::ffi::c_int
            || observation_parse_time(
                &raw mut value as *mut ::core::ffi::c_char,
                &raw mut query.until_ms,
            ) != 0 as ::core::ffi::c_int
        {
            send_invalid(
                res,
                b"until\0".as_ptr() as *const ::core::ffi::c_char,
                b"until must be an RFC 3339 time\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        query.has_until = r#true != 0;
    }
    if query.has_since as ::core::ffi::c_int != 0
        && query.has_until as ::core::ffi::c_int != 0
        && query.since_ms >= query.until_ms
    {
        send_invalid(
            res,
            b"until\0".as_ptr() as *const ::core::ffi::c_char,
            b"since must be before until\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut cursor: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut has_cursor: bool = r#false != 0;
    n = query_value(
        req,
        b"cursor\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut cursor as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    );
    if n != -1 as ::core::ffi::c_int {
        if n <= 0 as ::core::ffi::c_int {
            send_invalid(
                res,
                b"cursor\0".as_ptr() as *const ::core::ffi::c_char,
                b"cursor must not be empty\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        has_cursor = r#true != 0;
    }
    let mut requested: [[::core::ffi::c_char; 37]; 10] = [[0; 37]; 10];
    let mut requested_count: size_t = 0 as size_t;
    n = query_value(
        req,
        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    );
    if n != -1 as ::core::ffi::c_int
        && (n <= 0 as ::core::ffi::c_int
            || !parse_cameras(
                &raw mut value as *mut ::core::ffi::c_char,
                &raw mut requested as *mut [::core::ffi::c_char; 37],
                &raw mut requested_count,
            ))
    {
        send_invalid(
            res,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            b"camera_uuid must list 1 to 10 camera UUIDs\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if authorized_cameras(
        &raw mut user,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut cameras,
        &raw mut camera_count,
    ) != 0 as ::core::ffi::c_int
    {
        send_error(
            res,
            500 as ::core::ffi::c_int,
            b"internal_error\0".as_ptr() as *const ::core::ffi::c_char,
            b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut scope: *mut [::core::ffi::c_char; 37] =
        ::core::ptr::null_mut::<[::core::ffi::c_char; 37]>();
    if requested_count > 0 as size_t {
        let mut i: size_t = 0 as size_t;
        while i < requested_count {
            if !camera_in(
                cameras,
                camera_count,
                &raw mut *(&raw mut requested as *mut [::core::ffi::c_char; 37]).offset(i as isize)
                    as *mut ::core::ffi::c_char,
            ) {
                audit_camera_decision(
                    req,
                    &raw mut user,
                    authorization_action_t::AUTHZ_LIVE_VIEW,
                    &raw mut *(&raw mut requested as *mut [::core::ffi::c_char; 37])
                        .offset(i as isize) as *mut ::core::ffi::c_char,
                    ::core::ptr::null::<authorization_evaluation_t>(),
                    b"denied\0".as_ptr() as *const ::core::ffi::c_char,
                );
                free(cameras as *mut ::core::ffi::c_void);
                send_error(
                    res,
                    403 as ::core::ffi::c_int,
                    b"forbidden\0".as_ptr() as *const ::core::ffi::c_char,
                    b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            i = i.wrapping_add(1);
        }
        query.camera_uuids = &raw mut requested as *mut [::core::ffi::c_char; 37]
            as *const [::core::ffi::c_char; 37];
        query.camera_count = requested_count;
    } else if camera_count > 0 as ::core::ffi::c_int {
        scope = calloc(
            camera_count as size_t,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        ) as *mut [::core::ffi::c_char; 37];
        if scope.is_null() {
            free(cameras as *mut ::core::ffi::c_void);
            send_internal(res);
            return;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < camera_count {
            safe_strcpy(
                &raw mut *scope.offset(i_0 as isize) as *mut ::core::ffi::c_char,
                &raw mut (*cameras.offset(i_0 as isize)).camera_uuid as *mut ::core::ffi::c_char,
                OBS_UUID_SIZE as size_t,
                0 as size_t,
            );
            i_0 += 1;
        }
        query.camera_uuids = scope as *const [::core::ffi::c_char; 37];
        query.camera_count = camera_count as size_t;
    }
    free(cameras as *mut ::core::ffi::c_void);
    let mut page: observation_page_t = observation_page_t {
        items: ::core::ptr::null_mut::<observation_record_t>(),
        count: 0,
        next_cursor: [0; 257],
        has_more: false,
        oldest_cursor: [0; 257],
        has_oldest_observed_at: false,
        oldest_observed_at_ms: 0,
    };
    let mut rc: ::core::ffi::c_int = db_observations_list(
        &raw mut query,
        if has_cursor as ::core::ffi::c_int != 0 {
            &raw mut cursor as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        &raw mut page,
    );
    free(scope as *mut ::core::ffi::c_void);
    match rc {
        0 => {}
        -4 => {
            send_error(
                res,
                400 as ::core::ffi::c_int,
                b"invalid_cursor\0".as_ptr() as *const ::core::ffi::c_char,
                b"Malformed cursor\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        -5 => {
            send_cursor_expired(res, &raw mut page);
            db_observation_page_free(&raw mut page);
            return;
        }
        -2 => {
            send_invalid(
                res,
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Invalid observation query\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        _ => {
            send_internal(res);
            return;
        }
    }
    let mut body: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = if !body.is_null() {
        cJSON_AddArrayToObject(body, b"items\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut ok: bool = !items.is_null();
    if ok as ::core::ffi::c_int != 0 && page.count > 0 as size_t {
        let mut ids: *mut *const ::core::ffi::c_char = calloc(
            page.count,
            ::core::mem::size_of::<*const ::core::ffi::c_char>(),
        ) as *mut *const ::core::ffi::c_char;
        ok = !ids.is_null();
        let mut i_1: size_t = 0 as size_t;
        while ok as ::core::ffi::c_int != 0 && i_1 < page.count {
            *ids.offset(i_1 as isize) =
                &raw mut (*page.items.offset(i_1 as isize)).uuid as *mut ::core::ffi::c_char;
            i_1 = i_1.wrapping_add(1);
        }
        let mut context: page_item_context_t = page_item_context_t {
            page: &raw mut page,
            source: &raw mut source as *mut ::core::ffi::c_char,
            items: items,
        };
        ok = ok as ::core::ffi::c_int != 0
            && db_observation_classifications_list_many(
                ids,
                page.count,
                Some(
                    add_page_item
                        as unsafe extern "C" fn(
                            size_t,
                            *const observation_classification_set_t,
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                &raw mut context as *mut ::core::ffi::c_void,
            ) == C2Rust_Unnamed::DB_OBS_OK.0;
        free(ids as *mut ::core::ffi::c_void);
    }
    if ok {
        cJSON_AddStringToObject(
            body,
            b"next_cursor\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut page.next_cursor as *mut ::core::ffi::c_char,
        );
        cJSON_AddBoolToObject(
            body,
            b"has_more\0".as_ptr() as *const ::core::ffi::c_char,
            page.has_more as cJSON_bool,
        );
    }
    db_observation_page_free(&raw mut page);
    if !ok {
        cJSON_Delete(body);
        send_internal(res);
        return;
    }
    send_json(res, 200 as ::core::ffi::c_int, body);
}
unsafe extern "C" fn load_observation(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut suffix: *const ::core::ffi::c_char,
    mut record: *mut observation_record_t,
) -> ::core::ffi::c_int {
    let mut id: [::core::ffi::c_char; 37] = [0; 37];
    if !path_observation_id(req, suffix, &raw mut id as *mut ::core::ffi::c_char) {
        send_invalid(
            res,
            b"id\0".as_ptr() as *const ::core::ffi::c_char,
            b"Observation ID must be a UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut rc: ::core::ffi::c_int =
        db_observation_get(&raw mut id as *mut ::core::ffi::c_char, record);
    if rc == C2Rust_Unnamed::DB_OBS_NOT_FOUND.0 {
        return 0 as ::core::ffi::c_int;
    }
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        send_internal(res);
        return -1 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_observation(
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
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    let mut source: [::core::ffi::c_char; 128] = [0; 128];
    if !installation_source(&raw mut source as *mut ::core::ffi::c_char) {
        send_unavailable(res);
        return;
    }
    let mut record: *mut observation_record_t =
        calloc(1 as size_t, ::core::mem::size_of::<observation_record_t>())
            as *mut observation_record_t;
    if record.is_null() {
        send_internal(res);
        return;
    }
    let mut found: ::core::ffi::c_int =
        load_observation(req, res, ::core::ptr::null::<::core::ffi::c_char>(), record);
    if found <= 0 as ::core::ffi::c_int {
        if found == 0 as ::core::ffi::c_int {
            send_not_found(res);
        }
        free(record as *mut ::core::ffi::c_void);
        return;
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
    let mut rc: ::core::ffi::c_int = httpd_evaluate_camera_identity_action(
        &raw mut user,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut (*record).camera_uuid as *mut ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut evaluation,
    );
    if rc < 0 as ::core::ffi::c_int {
        audit_camera_decision(
            req,
            &raw mut user,
            authorization_action_t::AUTHZ_LIVE_VIEW,
            &raw mut (*record).camera_uuid as *mut ::core::ffi::c_char,
            ::core::ptr::null::<authorization_evaluation_t>(),
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        send_internal(res);
        free(record as *mut ::core::ffi::c_void);
        return;
    }
    if rc > 0 as ::core::ffi::c_int
        || evaluation.decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0
    {
        if rc == 0 as ::core::ffi::c_int {
            audit_camera_decision(
                req,
                &raw mut user,
                authorization_action_t::AUTHZ_LIVE_VIEW,
                &raw mut (*record).camera_uuid as *mut ::core::ffi::c_char,
                &raw mut evaluation,
                b"denied\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        send_not_found(res);
        free(record as *mut ::core::ffi::c_void);
        return;
    }
    audit_camera_decision(
        req,
        &raw mut user,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut (*record).camera_uuid as *mut ::core::ffi::c_char,
        &raw mut evaluation,
        b"allowed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut set: *mut observation_classification_set_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<observation_classification_set_t>(),
    )
        as *mut observation_classification_set_t;
    let mut body: *mut cJSON = if !set.is_null()
        && db_observation_classifications_list(
            &raw mut (*record).uuid as *mut ::core::ffi::c_char,
            set,
        ) == C2Rust_Unnamed::DB_OBS_OK.0
    {
        observation_record_to_json(record, &raw mut source as *mut ::core::ffi::c_char, set)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    free(set as *mut ::core::ffi::c_void);
    free(record as *mut ::core::ffi::c_void);
    if body.is_null() {
        send_internal(res);
        return;
    }
    send_json(res, 200 as ::core::ffi::c_int, body);
}
unsafe extern "C" fn resolve_asset_file(
    mut stored: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_int {
    let mut root: [::core::ffi::c_char; 512] = [0; 512];
    safe_strcpy(
        &raw mut root as *mut ::core::ffi::c_char,
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    let mut root_length: size_t = strlen(&raw mut root as *mut ::core::ffi::c_char);
    while root_length > 1 as size_t
        && root[root_length.wrapping_sub(1 as size_t)] as ::core::ffi::c_int
            == '/' as ::core::ffi::c_int
    {
        root_length = root_length.wrapping_sub(1);
        root[root_length] = '\0' as ::core::ffi::c_char;
    }
    if stored.is_null() || *stored.offset(0isize) == 0 || root_length == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut relative: *const ::core::ffi::c_char = stored;
    if *stored.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
        if strncmp(
            stored,
            &raw mut root as *mut ::core::ffi::c_char,
            root_length,
        ) != 0 as ::core::ffi::c_int
            || *stored.offset(root_length as isize) as ::core::ffi::c_int
                != '/' as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        relative = stored
            .offset(root_length as isize)
            .offset(1 as ::core::ffi::c_int as isize);
    }
    if strncmp(
        relative,
        b"observations/\0".as_ptr() as *const ::core::ffi::c_char,
        strlen(b"observations/\0".as_ptr() as *const ::core::ffi::c_char),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if !strchr(relative, '\\' as ::core::ffi::c_int).is_null()
        || !strstr(relative, b"//\0".as_ptr() as *const ::core::ffi::c_char).is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut segment: *const ::core::ffi::c_char = relative;
    while !segment.is_null() {
        if strncmp(
            segment,
            b"..\0".as_ptr() as *const ::core::ffi::c_char,
            2 as size_t,
        ) == 0 as ::core::ffi::c_int
            && (*segment.offset(2isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
                || *segment.offset(2isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int)
        {
            return -1 as ::core::ffi::c_int;
        }
        if *segment.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            && (*segment.offset(1isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
                || *segment.offset(1isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int)
        {
            return -1 as ::core::ffi::c_int;
        }
        segment = strchr(segment, '/' as ::core::ffi::c_int);
        if !segment.is_null() {
            segment = segment.offset(1);
        }
    }
    let mut length: size_t = strlen(relative);
    if length < 5 as size_t
        || strcmp(
            relative
                .offset(length as isize)
                .offset(-(4 as ::core::ffi::c_int as isize)),
            b".jpg\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut n: ::core::ffi::c_int = snprintf(
        out,
        size,
        b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut root as *mut ::core::ffi::c_char,
        relative,
    );
    return if n > 0 as ::core::ffi::c_int && (n as size_t) < size {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn now_ms() -> int64_t {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_REALTIME, &raw mut ts);
    return ts.tv_sec as int64_t * 1000 as int64_t + ts.tv_nsec as int64_t / 1000000 as int64_t;
}
unsafe extern "C" fn retry_after_seconds(
    mut asset: *const observation_asset_record_t,
) -> ::core::ffi::c_int {
    let mut stats: observation_evidence_stats_t = observation_evidence_stats_t {
        initialized: false,
        worker_running: false,
        allocated: false,
        accepted: 0,
        dropped_queue_full: 0,
        dropped_budget: 0,
        dropped_disabled: 0,
        dropped_untracked: 0,
        unavailable_submitted: 0,
        ready: 0,
        failed_encode: 0,
        failed_write: 0,
        failed_db: 0,
        disabled_at_encode: 0,
        interrupted: 0,
        bytes_written: 0,
        backlog_items: 0,
        backlog_bytes: 0,
        drop_ring_depth: 0,
        high_water_items: 0,
        high_water_bytes: 0,
        budget_exhausted_cameras: 0,
        limit_items: 0,
        limit_camera_items: 0,
        limit_bytes: 0,
        limit_camera_bytes: 0,
        camera_high_water_items: 0,
        submit_latency_count: 0,
        submit_latency_sum_ns: 0,
        submit_latency_max_ns: 0,
        submit_latency_bucket: [0; 10],
        discard_deferred: 0,
        discard_inline: 0,
        discard_pending_bytes: 0,
        limit_discard_bytes: 0,
    };
    observation_evidence_get_stats(&raw mut stats);
    return observation_snapshot_retry_after_seconds(
        now_ms() - (*asset).created_at_ms,
        stats.backlog_items as uint64_t,
    );
}
unsafe extern "C" fn send_evidence_unavailable(
    mut res: *mut http_response_t,
    mut reason: *const ::core::ffi::c_char,
) {
    let mut body: *mut cJSON = error_body(
        b"Evidence is unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        b"evidence_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !body.is_null() {
        cJSON_AddStringToObject(
            body,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            if !reason.is_null() && *reason.offset(0isize) as ::core::ffi::c_int != 0 {
                reason
            } else {
                b"write_failed\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    send_json(res, 404 as ::core::ffi::c_int, body);
}
unsafe extern "C" fn send_evidence_expired(
    mut res: *mut http_response_t,
    mut asset: *const observation_asset_record_t,
) {
    let mut body: *mut cJSON = error_body(
        b"Evidence media has expired\0".as_ptr() as *const ::core::ffi::c_char,
        b"evidence_expired\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !body.is_null() {
        cJSON_AddStringToObject(
            body,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            if (*asset).reason[0usize] as ::core::ffi::c_int != 0 {
                &raw const (*asset).reason as *const ::core::ffi::c_char
            } else {
                b"retention_expired\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        cJSON_AddItemToObject(
            body,
            b"expired_at\0".as_ptr() as *const ::core::ffi::c_char,
            time_or_null((*asset).updated_at_ms),
        );
    }
    send_json(res, 410 as ::core::ffi::c_int, body);
}
unsafe extern "C" fn etag_matches(
    mut req: *const http_request_t,
    mut etag: *const ::core::ffi::c_char,
) -> bool {
    let mut header: *const ::core::ffi::c_char = http_request_get_header(
        req,
        b"If-None-Match\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if header.is_null() {
        return r#false != 0;
    }
    while *header as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
        header = header.offset(1);
    }
    if strcmp(header, b"*\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return r#true != 0;
    }
    return !strstr(header, etag).is_null();
}
unsafe extern "C" fn serve_ready_snapshot(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut record: *const observation_record_t,
) {
    let mut file: [::core::ffi::c_char; 512] = [0; 512];
    if resolve_asset_file(
        &raw const (*record).asset.path as *const ::core::ffi::c_char,
        &raw mut file as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Observation asset %s has a path outside observation storage\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*record).asset.uuid as *const ::core::ffi::c_char,
        );
        send_internal(res);
        return;
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
    if stat(&raw mut file as *mut ::core::ffi::c_char, &raw mut st) != 0 as ::core::ffi::c_int
        || !(st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
    {
        let mut current: observation_asset_record_t = observation_asset_record_t {
            uuid: [0; 37],
            camera_uuid: [0; 37],
            state: observation_asset_state_t::OBS_ASSET_PENDING,
            reason: [0; 33],
            origin: [0; 24],
            media_type: [0; 24],
            path: [0; 512],
            has_bytes: false,
            bytes: 0,
            width: 0,
            height: 0,
            captured_at_ms: 0,
            transform_json: [0; 1024],
            is_protected: false,
            expires_at_ms: 0,
            created_at_ms: 0,
            updated_at_ms: 0,
        };
        if db_observation_asset_get(
            &raw const (*record).asset.uuid as *const ::core::ffi::c_char,
            &raw mut current,
        ) == C2Rust_Unnamed::DB_OBS_OK.0
            && current.state.0 == observation_asset_state_t::OBS_ASSET_EXPIRED.0
        {
            send_evidence_expired(res, &raw mut current);
            return;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Ready observation asset %s has no media file\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*record).asset.uuid as *const ::core::ffi::c_char,
        );
        send_evidence_unavailable(
            res,
            b"write_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut etag: [::core::ffi::c_char; 39] = [0; 39];
    snprintf(
        &raw mut etag as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 39]>(),
        b"\"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*record).asset.uuid as *const ::core::ffi::c_char,
    );
    if etag_matches(req, &raw mut etag as *mut ::core::ffi::c_char) {
        (*res).status_code = 304 as ::core::ffi::c_int;
        http_response_add_header(
            res,
            b"ETag\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut etag as *mut ::core::ffi::c_char,
        );
        http_response_add_header(
            res,
            b"Cache-Control\0".as_ptr() as *const ::core::ffi::c_char,
            OBS_SNAPSHOT_CACHE_CONTROL.as_ptr(),
        );
        return;
    }
    let mut headers: [::core::ffi::c_char; 160] = [0; 160];
    snprintf(
        &raw mut headers as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
        b"Cache-Control: %s\r\nETag: %s\r\n\0".as_ptr() as *const ::core::ffi::c_char,
        OBS_SNAPSHOT_CACHE_CONTROL.as_ptr(),
        &raw mut etag as *mut ::core::ffi::c_char,
    );
    if http_serve_file(
        req,
        res,
        &raw mut file as *mut ::core::ffi::c_char,
        b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut headers as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        send_internal(res);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_observation_snapshot(
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
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    let mut record: *mut observation_record_t =
        calloc(1 as size_t, ::core::mem::size_of::<observation_record_t>())
            as *mut observation_record_t;
    if record.is_null() {
        send_internal(res);
        return;
    }
    let mut found: ::core::ffi::c_int = load_observation(
        req,
        res,
        b"/snapshot\0".as_ptr() as *const ::core::ffi::c_char,
        record,
    );
    if found <= 0 as ::core::ffi::c_int {
        if found == 0 as ::core::ffi::c_int {
            send_not_found(res);
        }
        free(record as *mut ::core::ffi::c_void);
        return;
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
    let mut rc: ::core::ffi::c_int = httpd_evaluate_camera_identity_action(
        &raw mut user,
        authorization_action_t::AUTHZ_SNAPSHOT_CREATE,
        &raw mut (*record).camera_uuid as *mut ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut evaluation,
    );
    if rc < 0 as ::core::ffi::c_int {
        audit_snapshot(
            req,
            &raw mut user,
            record,
            ::core::ptr::null::<authorization_evaluation_t>(),
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        send_internal(res);
        free(record as *mut ::core::ffi::c_void);
        return;
    }
    if rc > 0 as ::core::ffi::c_int {
        send_not_found(res);
        free(record as *mut ::core::ffi::c_void);
        return;
    }
    if evaluation.decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0 {
        audit_snapshot(
            req,
            &raw mut user,
            record,
            &raw mut evaluation,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut view: authorization_evaluation_t = authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        };
        let mut view_rc: ::core::ffi::c_int = httpd_evaluate_camera_identity_action(
            &raw mut user,
            authorization_action_t::AUTHZ_LIVE_VIEW,
            &raw mut (*record).camera_uuid as *mut ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw mut view,
        );
        if view_rc < 0 as ::core::ffi::c_int {
            send_internal(res);
        } else if view_rc == 0 as ::core::ffi::c_int
            && view.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0
        {
            send_error(
                res,
                403 as ::core::ffi::c_int,
                b"forbidden\0".as_ptr() as *const ::core::ffi::c_char,
                b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            send_not_found(res);
        }
        free(record as *mut ::core::ffi::c_void);
        return;
    }
    audit_snapshot(
        req,
        &raw mut user,
        record,
        &raw mut evaluation,
        b"allowed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    match (*record).asset.state {
        observation_asset_state_t::OBS_ASSET_READY => {
            serve_ready_snapshot(req, res, record);
        }
        observation_asset_state_t::OBS_ASSET_PENDING => {
            let mut seconds: ::core::ffi::c_int = retry_after_seconds(&raw mut (*record).asset);
            let mut header: [::core::ffi::c_char; 16] = [0; 16];
            snprintf(
                &raw mut header as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                seconds,
            );
            http_response_add_header(
                res,
                b"Retry-After\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut header as *mut ::core::ffi::c_char,
            );
            let mut body: *mut cJSON = error_body(
                b"Evidence is not ready yet\0".as_ptr() as *const ::core::ffi::c_char,
                b"evidence_pending\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !body.is_null() {
                cJSON_AddNumberToObject(
                    body,
                    b"retry_after_seconds\0".as_ptr() as *const ::core::ffi::c_char,
                    seconds as ::core::ffi::c_double,
                );
            }
            send_json(res, 202 as ::core::ffi::c_int, body);
        }
        observation_asset_state_t::OBS_ASSET_EXPIRED => {
            send_evidence_expired(res, &raw mut (*record).asset);
        }
        observation_asset_state_t::OBS_ASSET_UNAVAILABLE | _ => {
            send_evidence_unavailable(
                res,
                &raw mut (*record).asset.reason as *mut ::core::ffi::c_char,
            );
        }
    }
    free(record as *mut ::core::ffi::c_void);
}
pub const OBS_PUT_PREFIX: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"/api/observations/\0")
};
pub const OBS_PUT_INFIX: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"/classifications/\0")
};
pub const OBS_ANNOTATE_ACTION: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"observations.annotate\0")
};
unsafe extern "C" fn copy_segment(
    mut start: *const ::core::ffi::c_char,
    mut length: size_t,
    mut out: *mut ::core::ffi::c_char,
) {
    *out.offset(0isize) = '\0' as ::core::ffi::c_char;
    if length == 0 as size_t || length > OBS_CLS_SEGMENT_MAX as size_t {
        return;
    }
    memcpy(
        out as *mut ::core::ffi::c_void,
        start as *const ::core::ffi::c_void,
        length,
    );
    *out.offset(length as isize) = '\0' as ::core::ffi::c_char;
    if !observation_classification_segment_is_valid(out) {
        *out.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn parse_put_path(
    mut path: *const ::core::ffi::c_char,
    mut target: *mut put_target_t,
) -> bool {
    memset(
        target as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<put_target_t>(),
    );
    let mut prefix: size_t = strlen(OBS_PUT_PREFIX.as_ptr());
    let mut infix: size_t = strlen(OBS_PUT_INFIX.as_ptr());
    if path.is_null() || strncmp(path, OBS_PUT_PREFIX.as_ptr(), prefix) != 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut id: *const ::core::ffi::c_char = path.offset(prefix as isize);
    let mut slash: *const ::core::ffi::c_char = strchr(id, '/' as ::core::ffi::c_int);
    if slash.is_null()
        || slash.offset_from(id) != (OBS_UUID_SIZE - 1 as ::core::ffi::c_int) as isize
        || strncmp(slash, OBS_PUT_INFIX.as_ptr(), infix) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    memcpy(
        &raw mut (*target).id as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        id as *const ::core::ffi::c_void,
        (OBS_UUID_SIZE - 1 as ::core::ffi::c_int) as size_t,
    );
    (*target).id[(OBS_UUID_SIZE - 1 as ::core::ffi::c_int) as usize] = '\0' as ::core::ffi::c_char;
    lowercase(&raw mut (*target).id as *mut ::core::ffi::c_char);
    if !lightnvr_uuid_is_valid(&raw mut (*target).id as *mut ::core::ffi::c_char) {
        return r#false != 0;
    }
    let mut ns: *const ::core::ffi::c_char = slash.offset(infix as isize);
    let mut separator: *const ::core::ffi::c_char = strchr(ns, '/' as ::core::ffi::c_int);
    if separator.is_null() {
        return r#false != 0;
    }
    copy_segment(
        ns,
        separator.offset_from(ns) as size_t,
        &raw mut (*target).ns as *mut ::core::ffi::c_char,
    );
    copy_segment(
        separator.offset(1 as ::core::ffi::c_int as isize),
        strlen(separator.offset(1 as ::core::ffi::c_int as isize)),
        &raw mut (*target).name as *mut ::core::ffi::c_char,
    );
    return r#true != 0;
}
unsafe extern "C" fn clean_utf8(mut text: *const ::core::ffi::c_char) -> bool {
    let mut p: *const ::core::ffi::c_uchar = text as *const ::core::ffi::c_uchar;
    while *p != 0 {
        let mut c: ::core::ffi::c_uchar = *p;
        let mut extra: ::core::ffi::c_int = 0;
        let mut code: uint32_t = 0;
        if (c as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
            if (c as ::core::ffi::c_int) < 0x20 as ::core::ffi::c_int
                || c as ::core::ffi::c_int == 0x7f as ::core::ffi::c_int
            {
                return r#false != 0;
            }
            p = p.offset(1);
        } else {
            if c as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int == 0xc0 as ::core::ffi::c_int {
                extra = 1 as ::core::ffi::c_int;
                code = (c as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as uint32_t;
            } else if c as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                == 0xe0 as ::core::ffi::c_int
            {
                extra = 2 as ::core::ffi::c_int;
                code = (c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as uint32_t;
            } else if c as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
                == 0xf0 as ::core::ffi::c_int
            {
                extra = 3 as ::core::ffi::c_int;
                code = (c as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as uint32_t;
            } else {
                return r#false != 0;
            }
            let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while i <= extra {
                if *p.offset(i as isize) as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
                {
                    return r#false != 0;
                }
                code = code << 6 as ::core::ffi::c_int
                    | (*p.offset(i as isize) as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                        as uint32_t;
                i += 1;
            }
            static mut minimum: [uint32_t; 4] = [
                0 as uint32_t,
                0x80 as uint32_t,
                0x800 as uint32_t,
                0x10000 as uint32_t,
            ];
            if code < minimum[extra as usize]
                || code > 0x10ffff as uint32_t
                || code >= 0xd800 as uint32_t && code <= 0xdfff as uint32_t
            {
                return r#false != 0;
            }
            p = p.offset((extra + 1 as ::core::ffi::c_int) as isize);
        }
    }
    return r#true != 0;
}
unsafe extern "C" fn body_text(
    mut body: *const cJSON,
    mut name: *const ::core::ffi::c_char,
    mut required: bool,
    mut max: size_t,
    mut out: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> bool {
    *out.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, name);
    if item.is_null() {
        return !required;
    }
    if !required && cJSON_IsNull(item) != 0 {
        return r#true != 0;
    }
    if cJSON_IsString(item) == 0 || (*item).valuestring.is_null() {
        return r#false != 0;
    }
    let mut length: size_t = strlen((*item).valuestring);
    if length > max
        || required as ::core::ffi::c_int != 0 && length == 0 as size_t
        || length >= size
    {
        return r#false != 0;
    }
    if !clean_utf8((*item).valuestring) {
        return r#false != 0;
    }
    memcpy(
        out as *mut ::core::ffi::c_void,
        (*item).valuestring as *const ::core::ffi::c_void,
        length.wrapping_add(1 as size_t),
    );
    return r#true != 0;
}
unsafe extern "C" fn body_integer(
    mut body: *const cJSON,
    mut name: *const ::core::ffi::c_char,
    mut minimum: ::core::ffi::c_int,
    mut out: *mut ::core::ffi::c_int,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, name);
    if cJSON_IsNumber(item) == 0 {
        return r#false != 0;
    }
    let mut value: ::core::ffi::c_double = (*item).valuedouble;
    if value.is_finite() as ::core::ffi::c_int == 0
        || value != floor(value)
        || value < minimum as ::core::ffi::c_double
        || value > INT_MAX as ::core::ffi::c_double
    {
        return r#false != 0;
    }
    *out = value as ::core::ffi::c_int;
    return r#true != 0;
}
unsafe extern "C" fn parse_put_body(
    mut body: *const cJSON,
    mut out: *mut put_body_t,
) -> *const ::core::ffi::c_char {
    static mut allowed: [*const ::core::ffi::c_char; 12] = [
        b"classifier\0".as_ptr() as *const ::core::ffi::c_char,
        b"model_version\0".as_ptr() as *const ::core::ffi::c_char,
        b"label\0".as_ptr() as *const ::core::ffi::c_char,
        b"taxonomy_id\0".as_ptr() as *const ::core::ffi::c_char,
        b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
        b"classified_at\0".as_ptr() as *const ::core::ffi::c_char,
        b"input_asset_id\0".as_ptr() as *const ::core::ffi::c_char,
        b"input_revision\0".as_ptr() as *const ::core::ffi::c_char,
        b"kind\0".as_ptr() as *const ::core::ffi::c_char,
        b"expected_revision\0".as_ptr() as *const ::core::ffi::c_char,
        b"writer_namespace\0".as_ptr() as *const ::core::ffi::c_char,
        b"writer\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    memset(
        out as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<put_body_t>(),
    );
    let mut item: *const cJSON = ::core::ptr::null::<cJSON>();
    item = if !body.is_null() {
        (*body).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !item.is_null() {
        let mut known: bool = r#false != 0;
        let mut i: size_t = 0 as size_t;
        while !known
            && i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 12]>()
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
        {
            known = !(*item).string.is_null()
                && strcmp((*item).string, allowed[i]) == 0 as ::core::ffi::c_int;
            i = i.wrapping_add(1);
        }
        if !known {
            return if !(*item).string.is_null() {
                (*item).string
            } else {
                b"body\0".as_ptr() as *const ::core::ffi::c_char
            };
        }
        item = (*item).next;
    }
    if !body_text(
        body,
        b"classifier\0".as_ptr() as *const ::core::ffi::c_char,
        r#true != 0,
        (OBS_CLS_CLASSIFIER_SIZE - 1 as ::core::ffi::c_int) as size_t,
        &raw mut (*out).classifier as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    ) {
        return b"classifier\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if !body_text(
        body,
        b"model_version\0".as_ptr() as *const ::core::ffi::c_char,
        r#false != 0,
        (OBS_CLS_MODEL_VERSION_SIZE - 1 as ::core::ffi::c_int) as size_t,
        &raw mut (*out).model_version as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    ) {
        return b"model_version\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if !body_text(
        body,
        b"label\0".as_ptr() as *const ::core::ffi::c_char,
        r#true != 0,
        (OBS_CLS_LABEL_SIZE - 1 as ::core::ffi::c_int) as size_t,
        &raw mut (*out).label as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 129]>(),
    ) {
        return b"label\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if !body_text(
        body,
        b"taxonomy_id\0".as_ptr() as *const ::core::ffi::c_char,
        r#false != 0,
        (OBS_CLS_TAXONOMY_SIZE - 1 as ::core::ffi::c_int) as size_t,
        &raw mut (*out).taxonomy_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    ) {
        return b"taxonomy_id\0".as_ptr() as *const ::core::ffi::c_char;
    }
    item = cJSON_GetObjectItemCaseSensitive(
        body,
        b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsNumber(item) == 0
        || (*item).valuedouble.is_finite() as ::core::ffi::c_int == 0
        || (*item).valuedouble < 0.0f64
        || (*item).valuedouble > 1.0f64
    {
        return b"confidence\0".as_ptr() as *const ::core::ffi::c_char;
    }
    (*out).confidence = (*item).valuedouble;
    item = cJSON_GetObjectItemCaseSensitive(
        body,
        b"classified_at\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsString(item) == 0
        || observation_parse_time((*item).valuestring, &raw mut (*out).classified_at_ms)
            != 0 as ::core::ffi::c_int
    {
        return b"classified_at\0".as_ptr() as *const ::core::ffi::c_char;
    }
    item = cJSON_GetObjectItemCaseSensitive(
        body,
        b"input_asset_id\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsString(item) == 0
        || strlen((*item).valuestring) != (OBS_UUID_SIZE - 1 as ::core::ffi::c_int) as size_t
    {
        return b"input_asset_id\0".as_ptr() as *const ::core::ffi::c_char;
    }
    memcpy(
        &raw mut (*out).input_asset_uuid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (*item).valuestring as *const ::core::ffi::c_void,
        OBS_UUID_SIZE as size_t,
    );
    lowercase(&raw mut (*out).input_asset_uuid as *mut ::core::ffi::c_char);
    if !lightnvr_uuid_is_valid(&raw mut (*out).input_asset_uuid as *mut ::core::ffi::c_char) {
        return b"input_asset_id\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if !body_integer(
        body,
        b"input_revision\0".as_ptr() as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
        &raw mut (*out).input_revision,
    ) {
        return b"input_revision\0".as_ptr() as *const ::core::ffi::c_char;
    }
    item = cJSON_GetObjectItemCaseSensitive(body, b"kind\0".as_ptr() as *const ::core::ffi::c_char);
    if cJSON_IsString(item) == 0
        || observation_classification_kind_parse((*item).valuestring, &raw mut (*out).kind)
            != 0 as ::core::ffi::c_int
    {
        return b"kind\0".as_ptr() as *const ::core::ffi::c_char;
    }
    (*out).has_kind = r#true != 0;
    if !cJSON_GetObjectItemCaseSensitive(
        body,
        b"expected_revision\0".as_ptr() as *const ::core::ffi::c_char,
    )
    .is_null()
        && !body_integer(
            body,
            b"expected_revision\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut (*out).expected_revision,
        )
    {
        return b"expected_revision\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn audit_annotation(
    mut audit: *const annotate_audit_t,
    mut outcome: *const ::core::ffi::c_char,
    mut result: *const ::core::ffi::c_char,
    mut before: ::core::ffi::c_int,
    mut after: ::core::ffi::c_int,
) {
    let mut context: *mut cJSON = cJSON_CreateObject();
    if !context.is_null() {
        cJSON_AddItemToObject(
            context,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            text_or_null((*audit).camera_uuid),
        );
        cJSON_AddItemToObject(
            context,
            b"key\0".as_ptr() as *const ::core::ffi::c_char,
            text_or_null(&raw const (*audit).key as *const ::core::ffi::c_char),
        );
        cJSON_AddItemToObject(
            context,
            b"kind\0".as_ptr() as *const ::core::ffi::c_char,
            text_or_null((*audit).kind),
        );
        cJSON_AddStringToObject(
            context,
            b"result\0".as_ptr() as *const ::core::ffi::c_char,
            result,
        );
        if before >= 0 as ::core::ffi::c_int {
            cJSON_AddNumberToObject(
                context,
                b"annotation_revision_before\0".as_ptr() as *const ::core::ffi::c_char,
                before as ::core::ffi::c_double,
            );
        } else {
            cJSON_AddNullToObject(
                context,
                b"annotation_revision_before\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if after >= 0 as ::core::ffi::c_int {
            cJSON_AddNumberToObject(
                context,
                b"annotation_revision_after\0".as_ptr() as *const ::core::ffi::c_char,
                after as ::core::ffi::c_double,
            );
        } else {
            cJSON_AddNullToObject(
                context,
                b"annotation_revision_after\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    audit_log_operation(
        (*audit).req,
        (*audit).user,
        OBS_ANNOTATE_ACTION.as_ptr(),
        b"observation\0".as_ptr() as *const ::core::ffi::c_char,
        (*audit).observation_uuid,
        b"annotate\0".as_ptr() as *const ::core::ffi::c_char,
        outcome,
        context,
    );
    cJSON_Delete(context);
}
unsafe extern "C" fn send_annotation_error(
    mut res: *mut http_response_t,
    mut status: ::core::ffi::c_int,
    mut code: *const ::core::ffi::c_char,
    mut message: *const ::core::ffi::c_char,
    mut result: *const observation_classification_result_t,
) {
    let mut body: *mut cJSON = error_body(message, code);
    if !body.is_null() && !result.is_null() && (*result).has_annotation as ::core::ffi::c_int != 0 {
        let mut current: *mut cJSON =
            observation_classification_to_json(&raw const (*result).annotation);
        if !current.is_null() {
            cJSON_AddItemToObject(
                body,
                b"current\0".as_ptr() as *const ::core::ffi::c_char,
                current,
            );
        }
    }
    send_json(res, status, body);
}
unsafe extern "C" fn respond_to_put(
    mut audit: *const annotate_audit_t,
    mut res: *mut http_response_t,
    mut rc: ::core::ffi::c_int,
    mut result: *const observation_classification_result_t,
) {
    let mut stored: ::core::ffi::c_int = if (*result).has_annotation as ::core::ffi::c_int != 0 {
        (*result).annotation.annotation_revision
    } else {
        -1 as ::core::ffi::c_int
    };
    match rc {
        0 | 1 | 2 => {
            let mut changed: bool = rc != C2Rust_Unnamed_0::DB_OBS_CLS_NOOP.0;
            let mut body: *mut cJSON = cJSON_CreateObject();
            let mut annotation: *mut cJSON =
                observation_classification_to_json(&raw const (*result).annotation);
            if body.is_null() || annotation.is_null() {
                cJSON_Delete(body);
                cJSON_Delete(annotation);
                audit_annotation(
                    audit,
                    b"error\0".as_ptr() as *const ::core::ffi::c_char,
                    b"internal_error\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                    -1 as ::core::ffi::c_int,
                );
                send_internal(res);
                return;
            }
            cJSON_AddBoolToObject(
                body,
                b"changed\0".as_ptr() as *const ::core::ffi::c_char,
                changed as cJSON_bool,
            );
            cJSON_AddNumberToObject(
                body,
                b"observation_revision\0".as_ptr() as *const ::core::ffi::c_char,
                (*result).observation_revision as ::core::ffi::c_double,
            );
            cJSON_AddItemToObject(
                body,
                b"annotation\0".as_ptr() as *const ::core::ffi::c_char,
                annotation,
            );
            let mut name: *const ::core::ffi::c_char =
                if rc == C2Rust_Unnamed_0::DB_OBS_CLS_CREATED.0 {
                    b"created\0".as_ptr() as *const ::core::ffi::c_char
                } else if rc == C2Rust_Unnamed_0::DB_OBS_CLS_UPDATED.0 {
                    b"updated\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"noop\0".as_ptr() as *const ::core::ffi::c_char
                };
            let mut before: ::core::ffi::c_int = if rc == C2Rust_Unnamed_0::DB_OBS_CLS_CREATED.0 {
                -1 as ::core::ffi::c_int
            } else if rc == C2Rust_Unnamed_0::DB_OBS_CLS_UPDATED.0 {
                stored - 1 as ::core::ffi::c_int
            } else {
                stored
            };
            audit_annotation(
                audit,
                b"success\0".as_ptr() as *const ::core::ffi::c_char,
                name,
                before,
                stored,
            );
            send_json(
                res,
                if rc == C2Rust_Unnamed_0::DB_OBS_CLS_CREATED.0 {
                    201 as ::core::ffi::c_int
                } else {
                    200 as ::core::ffi::c_int
                },
                body,
            );
            return;
        }
        3 => {
            audit_annotation(
                audit,
                b"failure\0".as_ptr() as *const ::core::ffi::c_char,
                b"revision_conflict\0".as_ptr() as *const ::core::ffi::c_char,
                stored,
                stored,
            );
            send_annotation_error(
                res,
                409 as ::core::ffi::c_int,
                b"revision_conflict\0".as_ptr() as *const ::core::ffi::c_char,
                b"Annotation revision does not match\0".as_ptr() as *const ::core::ffi::c_char,
                result,
            );
            return;
        }
        4 => {
            audit_annotation(
                audit,
                b"failure\0".as_ptr() as *const ::core::ffi::c_char,
                b"owner_decision_locked\0".as_ptr() as *const ::core::ffi::c_char,
                stored,
                stored,
            );
            send_annotation_error(
                res,
                409 as ::core::ffi::c_int,
                b"owner_decision_locked\0".as_ptr() as *const ::core::ffi::c_char,
                b"An owner-confirmed annotation cannot be replaced by an automated one\0".as_ptr()
                    as *const ::core::ffi::c_char,
                result,
            );
            return;
        }
        5 => {
            audit_annotation(
                audit,
                b"failure\0".as_ptr() as *const ::core::ffi::c_char,
                b"evidence_mismatch\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                -1 as ::core::ffi::c_int,
            );
            let mut body_0: *mut cJSON = error_body(
                b"Annotation input does not match the observation evidence\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"evidence_mismatch\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !body_0.is_null() {
                cJSON_AddStringToObject(
                    body_0,
                    b"observation_asset_id\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*result).observation_asset_uuid as *const ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    body_0,
                    b"evidence_state\0".as_ptr() as *const ::core::ffi::c_char,
                    observation_asset_state_name((*result).evidence_state),
                );
            }
            send_json(res, 422 as ::core::ffi::c_int, body_0);
            return;
        }
        6 => {
            audit_annotation(
                audit,
                b"denied\0".as_ptr() as *const ::core::ffi::c_char,
                b"namespace_forbidden\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                -1 as ::core::ffi::c_int,
            );
            send_error(
                res,
                403 as ::core::ffi::c_int,
                b"namespace_forbidden\0".as_ptr() as *const ::core::ffi::c_char,
                b"Writer namespace not permitted\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        7 => {
            audit_annotation(
                audit,
                b"failure\0".as_ptr() as *const ::core::ffi::c_char,
                b"annotation_limit\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                -1 as ::core::ffi::c_int,
            );
            let mut body_1: *mut cJSON = error_body(
                b"Annotation limit reached for this observation\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"annotation_limit\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !body_1.is_null() {
                cJSON_AddNumberToObject(
                    body_1,
                    b"limit\0".as_ptr() as *const ::core::ffi::c_char,
                    (*result).limit as ::core::ffi::c_double,
                );
            }
            send_json(res, 409 as ::core::ffi::c_int, body_1);
            return;
        }
        -2 => {
            audit_annotation(
                audit,
                b"failure\0".as_ptr() as *const ::core::ffi::c_char,
                b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                -1 as ::core::ffi::c_int,
            );
            send_invalid(
                res,
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Invalid annotation\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        -3 => {
            audit_annotation(
                audit,
                b"failure\0".as_ptr() as *const ::core::ffi::c_char,
                b"observation_not_found\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                -1 as ::core::ffi::c_int,
            );
            send_not_found(res);
            return;
        }
        _ => {
            audit_annotation(
                audit,
                b"error\0".as_ptr() as *const ::core::ffi::c_char,
                b"internal_error\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                -1 as ::core::ffi::c_int,
            );
            send_internal(res);
            return;
        }
    };
}
unsafe extern "C" fn authorize_annotation(
    mut audit: *const annotate_audit_t,
    mut res: *mut http_response_t,
    mut user: *const user_t,
    mut camera_uuid: *const ::core::ffi::c_char,
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
    let mut rc: ::core::ffi::c_int = httpd_evaluate_camera_identity_action(
        user,
        authorization_action_t::AUTHZ_OBSERVATIONS_ANNOTATE,
        camera_uuid,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut evaluation,
    );
    if rc == 0 as ::core::ffi::c_int
        && evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0
    {
        return r#true != 0;
    }
    if rc < 0 as ::core::ffi::c_int {
        audit_annotation(
            audit,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"internal_error\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
        );
        send_internal(res);
        return r#false != 0;
    }
    if rc > 0 as ::core::ffi::c_int {
        audit_annotation(
            audit,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
            b"observation_not_found\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
        );
        send_not_found(res);
        return r#false != 0;
    }
    let mut view: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t::AUTHZ_DECISION_DENY,
        source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
        policy_version: 0,
        grant_uuid: [0; 37],
        token_uuid: [0; 37],
        role_uuid: [0; 37],
        role_name: [0; 128],
        explanation: [0; 256],
    };
    let mut view_rc: ::core::ffi::c_int = httpd_evaluate_camera_identity_action(
        user,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        camera_uuid,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut view,
    );
    if view_rc < 0 as ::core::ffi::c_int {
        audit_annotation(
            audit,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"internal_error\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
        );
        send_internal(res);
    } else if view_rc == 0 as ::core::ffi::c_int
        && view.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0
    {
        audit_annotation(
            audit,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
            b"forbidden\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
        );
        send_error(
            res,
            403 as ::core::ffi::c_int,
            b"forbidden\0".as_ptr() as *const ::core::ffi::c_char,
            b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        audit_annotation(
            audit,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
            b"observation_not_found\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
        );
        send_not_found(res);
    }
    return r#false != 0;
}
unsafe extern "C" fn body_names_other_writer(
    mut body: *const cJSON,
    mut field: *const ::core::ffi::c_char,
    mut writer: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, field);
    if item.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if cJSON_IsString(item) == 0 || (*item).valuestring.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    return if strcmp((*item).valuestring, writer) == 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_observation_classification(
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
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    let mut source: [::core::ffi::c_char; 128] = [0; 128];
    if !installation_source(&raw mut source as *mut ::core::ffi::c_char) {
        send_unavailable(res);
        return;
    }
    let mut target: put_target_t = put_target_t {
        id: [0; 37],
        ns: [0; 33],
        name: [0; 33],
    };
    if !parse_put_path(
        &raw const (*req).path as *const ::core::ffi::c_char,
        &raw mut target,
    ) {
        send_invalid(
            res,
            b"id\0".as_ptr() as *const ::core::ffi::c_char,
            b"Observation ID must be a UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut audit: annotate_audit_t = annotate_audit_t {
        req: req,
        user: &raw mut user,
        observation_uuid: &raw mut target.id as *mut ::core::ffi::c_char,
        camera_uuid: ::core::ptr::null::<::core::ffi::c_char>(),
        key: [0; 66],
        kind: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    if target.ns[0usize] as ::core::ffi::c_int != 0
        && target.name[0usize] as ::core::ffi::c_int != 0
    {
        snprintf(
            &raw mut audit.key as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 66]>(),
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut target.ns as *mut ::core::ffi::c_char,
            &raw mut target.name as *mut ::core::ffi::c_char,
        );
    }
    let mut record: *mut observation_record_t =
        calloc(1 as size_t, ::core::mem::size_of::<observation_record_t>())
            as *mut observation_record_t;
    if record.is_null() {
        send_internal(res);
        return;
    }
    let mut rc: ::core::ffi::c_int =
        db_observation_get(&raw mut target.id as *mut ::core::ffi::c_char, record);
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        if rc == C2Rust_Unnamed::DB_OBS_NOT_FOUND.0 {
            audit_annotation(
                &raw mut audit,
                b"failure\0".as_ptr() as *const ::core::ffi::c_char,
                b"observation_not_found\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                -1 as ::core::ffi::c_int,
            );
            send_not_found(res);
        } else {
            audit_annotation(
                &raw mut audit,
                b"error\0".as_ptr() as *const ::core::ffi::c_char,
                b"internal_error\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                -1 as ::core::ffi::c_int,
            );
            send_internal(res);
        }
        free(record as *mut ::core::ffi::c_void);
        return;
    }
    let mut camera_uuid: [::core::ffi::c_char; 37] = [0; 37];
    safe_strcpy(
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        &raw mut (*record).camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    free(record as *mut ::core::ffi::c_void);
    audit.camera_uuid = &raw mut camera_uuid as *mut ::core::ffi::c_char;
    if !authorize_annotation(
        &raw mut audit,
        res,
        &raw mut user,
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
    ) {
        return;
    }
    let mut writer: [::core::ffi::c_char; 33] = [0; 33];
    if observation_writer_namespace(
        &raw mut user.username as *mut ::core::ffi::c_char,
        &raw mut writer as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
        || target.ns[0usize] == 0
        || strcmp(
            &raw mut target.ns as *mut ::core::ffi::c_char,
            &raw mut writer as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        audit_annotation(
            &raw mut audit,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
            b"namespace_forbidden\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
        );
        send_error(
            res,
            403 as ::core::ffi::c_int,
            b"namespace_forbidden\0".as_ptr() as *const ::core::ffi::c_char,
            b"Writer namespace not permitted\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*req).body.is_null()
        || (*req).body_len == 0 as size_t
        || (*req).body_len > OBS_PUT_BODY_MAX as size_t
    {
        audit_annotation(
            &raw mut audit,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
        );
        send_invalid(
            res,
            b"body\0".as_ptr() as *const ::core::ffi::c_char,
            if (*req).body_len > OBS_PUT_BODY_MAX as size_t {
                b"Request body is too large\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"Request body is required\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    if cJSON_IsObject(body) == 0 {
        cJSON_Delete(body);
        audit_annotation(
            &raw mut audit,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
        );
        send_invalid(
            res,
            b"body\0".as_ptr() as *const ::core::ffi::c_char,
            b"Request body must be a JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut writer_fields: [*const ::core::ffi::c_char; 2] = [
        b"writer_namespace\0".as_ptr() as *const ::core::ffi::c_char,
        b"writer\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        let mut other: ::core::ffi::c_int = body_names_other_writer(
            body,
            writer_fields[i as usize],
            &raw mut writer as *mut ::core::ffi::c_char,
        );
        if other == 1 as ::core::ffi::c_int {
            cJSON_Delete(body);
            audit_annotation(
                &raw mut audit,
                b"denied\0".as_ptr() as *const ::core::ffi::c_char,
                b"namespace_forbidden\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                -1 as ::core::ffi::c_int,
            );
            send_error(
                res,
                403 as ::core::ffi::c_int,
                b"namespace_forbidden\0".as_ptr() as *const ::core::ffi::c_char,
                b"Writer namespace not permitted\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        if other < 0 as ::core::ffi::c_int {
            cJSON_Delete(body);
            audit_annotation(
                &raw mut audit,
                b"failure\0".as_ptr() as *const ::core::ffi::c_char,
                b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                -1 as ::core::ffi::c_int,
            );
            send_invalid(
                res,
                writer_fields[i as usize],
                b"Writer fields must be strings\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        i += 1;
    }
    if target.name[0usize] == 0 {
        cJSON_Delete(body);
        audit_annotation(
            &raw mut audit,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
        );
        send_invalid(
            res,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            b"Annotation name must match ^[a-z0-9][a-z0-9_.-]{0,31}$\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut fields: put_body_t = put_body_t {
        classifier: [0; 65],
        model_version: [0; 65],
        label: [0; 129],
        taxonomy_id: [0; 65],
        confidence: 0.,
        classified_at_ms: 0,
        input_asset_uuid: [0; 37],
        input_revision: 0,
        kind: observation_classification_kind_t::OBS_CLS_KIND_AUTOMATED,
        has_kind: false,
        expected_revision: 0,
    };
    let mut invalid: *const ::core::ffi::c_char = parse_put_body(body, &raw mut fields);
    if fields.has_kind {
        audit.kind = observation_classification_kind_name(fields.kind);
    }
    if !invalid.is_null() {
        let mut field: [::core::ffi::c_char; 64] = [0; 64];
        let mut message: [::core::ffi::c_char; 128] = [0; 128];
        safe_strcpy(
            &raw mut field as *mut ::core::ffi::c_char,
            invalid,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        cJSON_Delete(body);
        snprintf(
            &raw mut message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            b"Invalid or unknown field: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut field as *mut ::core::ffi::c_char,
        );
        audit_annotation(
            &raw mut audit,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
        );
        send_invalid(
            res,
            &raw mut field as *mut ::core::ffi::c_char,
            &raw mut message as *mut ::core::ffi::c_char,
        );
        return;
    }
    cJSON_Delete(body);
    let mut input: observation_classification_input_t = observation_classification_input_t {
        writer_namespace: &raw mut writer as *mut ::core::ffi::c_char,
        writer_user_id: user.id,
        name: &raw mut target.name as *mut ::core::ffi::c_char,
        classifier: &raw mut fields.classifier as *mut ::core::ffi::c_char,
        model_version: &raw mut fields.model_version as *mut ::core::ffi::c_char,
        label: &raw mut fields.label as *mut ::core::ffi::c_char,
        taxonomy_id: &raw mut fields.taxonomy_id as *mut ::core::ffi::c_char,
        confidence: fields.confidence,
        classified_at_ms: fields.classified_at_ms,
        input_asset_uuid: &raw mut fields.input_asset_uuid as *mut ::core::ffi::c_char,
        input_revision: fields.input_revision,
        kind: fields.kind,
        expected_revision: fields.expected_revision,
    };
    let mut result: *mut observation_classification_result_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<observation_classification_result_t>(),
    )
        as *mut observation_classification_result_t;
    if result.is_null() {
        audit_annotation(
            &raw mut audit,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"internal_error\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
        );
        send_internal(res);
        return;
    }
    rc = db_observation_classification_put(
        &raw mut target.id as *mut ::core::ffi::c_char,
        &raw mut input,
        result,
    );
    respond_to_put(&raw mut audit, res, rc, result);
    free(result as *mut ::core::ffi::c_void);
}
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
