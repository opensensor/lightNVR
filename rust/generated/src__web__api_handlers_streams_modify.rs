unsafe extern "C" {
    pub type stream_handle_s;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
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
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNull(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
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
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_authorize_global_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
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
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn log_set_thread_context(
        component: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
    );
    unsafe fn is_valid_stream_name(name: *const ::core::ffi::c_char) -> bool;
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
    unsafe fn url_build_onvif_device_service_url(
        url: *const ::core::ffi::c_char,
        onvif_port: ::core::ffi::c_int,
        out_url: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn url_redact_for_logging(
        url: *const ::core::ffi::c_char,
        out_url: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn publish_url_list_is_valid(list: *const ::core::ffi::c_char) -> bool;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn string_has_non_whitespace(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn add_stream(config: *const stream_config_t) -> stream_handle_t;
    unsafe fn remove_stream(handle: stream_handle_t) -> ::core::ffi::c_int;
    unsafe fn start_stream(handle: stream_handle_t) -> ::core::ffi::c_int;
    unsafe fn stop_stream(handle: stream_handle_t) -> ::core::ffi::c_int;
    unsafe fn get_stream_status(handle: stream_handle_t) -> stream_status_t;
    unsafe fn get_stream_config(
        handle: stream_handle_t,
        config: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_by_name(name: *const ::core::ffi::c_char) -> stream_handle_t;
    unsafe fn get_total_stream_count() -> ::core::ffi::c_int;
    unsafe fn get_stream_capacity() -> ::core::ffi::c_int;
    unsafe fn set_stream_recording(handle: stream_handle_t, enable: bool) -> ::core::ffi::c_int;
    unsafe fn set_stream_detection_recording(
        handle: stream_handle_t,
        enable: bool,
        model_path: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn set_stream_detection_params(
        handle: stream_handle_t,
        interval: ::core::ffi::c_int,
        threshold: ::core::ffi::c_float,
        pre_buffer: ::core::ffi::c_int,
        post_buffer: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn set_stream_streaming_enabled(
        handle: stream_handle_t,
        enabled: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn is_detection_recording_scheduled(config: *const stream_config_t) -> bool;
    unsafe fn clear_stream_hls_segments(
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn signal_mp4_recording_reconnect(stream_name: *const ::core::ffi::c_char);
    unsafe fn stream_restart_hls(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn start_unified_detection_thread(
        stream_name: *const ::core::ffi::c_char,
        model_path: *const ::core::ffi::c_char,
        threshold: ::core::ffi::c_float,
        pre_buffer_seconds: ::core::ffi::c_int,
        post_buffer_seconds: ::core::ffi::c_int,
        annotation_only: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn stop_unified_detection_thread(
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn is_unified_detection_running(stream_name: *const ::core::ffi::c_char) -> bool;
    unsafe fn add_stream_config(stream: *const stream_config_t) -> uint64_t;
    unsafe fn update_stream_config(
        name: *const ::core::ffi::c_char,
        stream: *const stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn set_stream_privacy_mode(
        name: *const ::core::ffi::c_char,
        privacy_mode: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn delete_stream_config(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn delete_stream_config_internal(
        name: *const ::core::ffi::c_char,
        permanent: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_config_by_name(
        name: *const ::core::ffi::c_char,
        stream: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_find_by_name_readonly(
        stream_name: *const ::core::ffi::c_char,
        camera: *mut fleet_camera_t,
    ) -> ::core::ffi::c_int;
    unsafe fn test_onvif_connection(
        url: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn go2rtc_stream_register_substream(
        stream_id: *const ::core::ffi::c_char,
        stream_url: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        protocol: stream_protocol_t,
    ) -> bool;
    unsafe fn go2rtc_stream_is_ready() -> bool;
    unsafe fn go2rtc_integration_full_start() -> bool;
    unsafe fn go2rtc_integration_restart_process() -> bool;
    unsafe fn go2rtc_integration_is_initialized() -> bool;
    unsafe fn go2rtc_integration_reload_stream_config(
        stream_name: *const ::core::ffi::c_char,
        new_url: *const ::core::ffi::c_char,
        new_username: *const ::core::ffi::c_char,
        new_password: *const ::core::ffi::c_char,
        new_backchannel_enabled: ::core::ffi::c_int,
        new_protocol: ::core::ffi::c_int,
        new_record_audio: ::core::ffi::c_int,
    ) -> bool;
    unsafe fn go2rtc_integration_reload_stream(stream_name: *const ::core::ffi::c_char) -> bool;
    unsafe fn go2rtc_integration_unregister_stream(stream_name: *const ::core::ffi::c_char)
    -> bool;
    unsafe fn go2rtc_api_remove_stream(stream_id: *const ::core::ffi::c_char) -> bool;
    unsafe fn go2rtc_lifecycle_begin(
        operation: go2rtc_lifecycle_operation_t,
        coalesce: bool,
        intentional: bool,
        guard: *mut go2rtc_lifecycle_guard_t,
    ) -> bool;
    unsafe fn go2rtc_lifecycle_end(guard: *mut go2rtc_lifecycle_guard_t, result: bool);
    unsafe fn go2rtc_lifecycle_restart_generation() -> uint64_t;
    unsafe fn yaml_validate_str(
        src: *const ::core::ffi::c_char,
        len: size_t,
        err_buf: *mut ::core::ffi::c_char,
        err_size: size_t,
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
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __clockid_t = ::core::ffi::c_int;
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
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const PTHREAD_CREATE_JOINABLE: Self = Self(0);
    pub const PTHREAD_CREATE_DETACHED: Self = Self(1);
}
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
pub type stream_handle_t = *mut stream_handle_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct put_stream_task_t {
    pub config: stream_config_t,
    pub stream_id: [::core::ffi::c_char; 256],
    pub original_url: [::core::ffi::c_char; 512],
    pub original_protocol: stream_protocol_t,
    pub original_record_audio: bool,
    pub original_backchannel_enabled: bool,
    pub original_enabled: bool,
    pub config_changed: bool,
    pub requires_restart: bool,
    pub is_running: bool,
    pub onvif_test_performed: bool,
    pub onvif_test_success: bool,
    pub original_detection_based_recording: bool,
    pub has_detection_based_recording: bool,
    pub detection_based_recording_value: bool,
    pub has_detection_model: bool,
    pub has_detection_threshold: bool,
    pub has_detection_interval: bool,
    pub has_detection_url: bool,
    pub has_record: bool,
    pub has_streaming_enabled: bool,
    pub non_dynamic_config_changed: bool,
    pub credentials_changed: bool,
    pub go2rtc_override_changed: bool,
    pub sub_stream_changed: bool,
    pub publish_changed: bool,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delete_stream_task_t {
    pub stream_id: [::core::ffi::c_char; 256],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub permanent_delete: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refresh_stream_guard_t {
    pub used: bool,
    pub in_progress: bool,
    pub stream_name: [::core::ffi::c_char; 256],
    pub last_started: timespec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refresh_stream_task_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub restart_generation: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct go2rtc_lifecycle_guard_t {
    pub acquired: bool,
    pub outermost: bool,
    pub coalesced: bool,
    pub result: bool,
    pub generation: uint64_t,
    pub restart_generation: uint64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct go2rtc_lifecycle_operation_t(pub ::core::ffi::c_uint);
impl go2rtc_lifecycle_operation_t {
    pub const GO2RTC_LIFECYCLE_CHECK: Self = Self(0);
    pub const GO2RTC_LIFECYCLE_PROCESS_START: Self = Self(1);
    pub const GO2RTC_LIFECYCLE_PROCESS_STOP: Self = Self(2);
    pub const GO2RTC_LIFECYCLE_FULL_START: Self = Self(3);
    pub const GO2RTC_LIFECYCLE_RESTART: Self = Self(4);
    pub const GO2RTC_LIFECYCLE_RECONFIGURE: Self = Self(5);
    pub const GO2RTC_LIFECYCLE_CLEANUP: Self = Self(6);
    pub const GO2RTC_LIFECYCLE_OPERATION_COUNT: Self = Self(7);
}
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAX_URL_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const LOG_COMPONENT: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"StreamsAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut g_put_stream_update_mutex: pthread_mutex_t = pthread_mutex_t {
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
unsafe extern "C" fn check_stream_write_access(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut config: *mut stream_config_t,
) -> bool {
    if config.is_null() {
        return httpd_authorize_global_action(
            req,
            res,
            authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        ) != 0 as ::core::ffi::c_int;
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
    if httpd_authorize_stream_action_with_context(
        req,
        res,
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        &raw mut (*config).name as *mut ::core::ffi::c_char,
        &raw mut user,
        &raw mut camera,
        &raw mut evaluation,
    ) == 0
    {
        return r#false != 0;
    }
    safe_strcpy(
        &raw mut (*config).camera_uuid as *mut ::core::ffi::c_char,
        &raw mut camera.camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn stream_task_identity_matches(
    mut name: *const ::core::ffi::c_char,
    mut camera_uuid: *const ::core::ffi::c_char,
) -> bool {
    let mut current: fleet_camera_t = fleet_camera_t {
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
    return *camera_uuid.offset(0isize) as ::core::ffi::c_int != 0
        && db_fleet_camera_find_by_name_readonly(name, &raw mut current)
            == 0 as ::core::ffi::c_int
        && strcmp(
            camera_uuid,
            &raw mut current.camera_uuid as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn format_stream_capacity_error(
    mut buf: *mut ::core::ffi::c_char,
    mut buf_size: size_t,
    mut runtime_capacity: ::core::ffi::c_int,
    mut configured_capacity: ::core::ffi::c_int,
    mut current_total: ::core::ffi::c_int,
) {
    if buf.is_null() || buf_size == 0 as size_t {
        return;
    }
    if configured_capacity > runtime_capacity {
        snprintf(
            buf,
            buf_size,
            b"Max streams limit reached (%d/%d in the current runtime). Settings are already saved for %d streams, but you must restart LightNVR before adding more cameras.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            current_total,
            runtime_capacity,
            configured_capacity,
        );
        return;
    }
    snprintf(
        buf,
        buf_size,
        b"Max streams limit reached (%d/%d). Increase Settings -> Max Streams and restart LightNVR before adding more cameras.\0"
            .as_ptr() as *const ::core::ffi::c_char,
        current_total,
        runtime_capacity,
    );
}
unsafe extern "C" fn put_stream_task_free(mut task: *mut put_stream_task_t) {
    if !task.is_null() {
        free(task as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn normalize_stream_url_credentials(mut config: *mut stream_config_t) {
    let mut extracted_username: [::core::ffi::c_char; 64] = [
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
    let mut extracted_password: [::core::ffi::c_char; 64] = [
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
    let mut stripped_url: [::core::ffi::c_char; 512] = [0; 512];
    let mut use_separate_credentials: bool = false;
    if config.is_null() {
        return;
    }
    use_separate_credentials = (*config).is_onvif as ::core::ffi::c_int != 0
        || (*config).onvif_username[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || (*config).onvif_password[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
    if !use_separate_credentials {
        return;
    }
    if url_extract_credentials(
        &raw mut (*config).url as *mut ::core::ffi::c_char,
        &raw mut extracted_username as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        &raw mut extracted_password as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) == 0 as ::core::ffi::c_int
    {
        if (*config).onvif_username[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            && extracted_username[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).onvif_username as *mut ::core::ffi::c_char,
                &raw mut extracted_username as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
        if (*config).onvif_password[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            && extracted_password[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*config).onvif_password as *mut ::core::ffi::c_char,
                &raw mut extracted_password as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
    }
    if url_strip_credentials(
        &raw mut (*config).url as *mut ::core::ffi::c_char,
        &raw mut stripped_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) == 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut (*config).url as *mut ::core::ffi::c_char,
            &raw mut stripped_url as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
}
unsafe extern "C" fn build_onvif_test_url(
    mut config: *const stream_config_t,
    mut out_url: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> ::core::ffi::c_int {
    if config.is_null() || out_url.is_null() || out_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    return url_build_onvif_device_service_url(
        &raw const (*config).url as *const ::core::ffi::c_char,
        (*config).onvif_port,
        out_url,
        out_size,
    );
}
unsafe extern "C" fn redact_url_for_log(
    mut url: *const ::core::ffi::c_char,
    mut out_url: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) {
    if url_redact_for_logging(url, out_url, out_size) != 0 as ::core::ffi::c_int {
        safe_strcpy(
            out_url,
            b"[invalid-url]\0".as_ptr() as *const ::core::ffi::c_char,
            out_size,
            0 as size_t,
        );
    }
}
unsafe extern "C" fn put_stream_worker_locked(mut task: *mut put_stream_task_t) {
    log_set_thread_context(
        b"StreamAPI\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Processing PUT /api/streams/%s in worker thread\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
    );
    let mut stream: stream_handle_t =
        get_stream_by_name(&raw mut (*task).stream_id as *mut ::core::ffi::c_char);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Stream %s no longer exists, dropping queued PUT update\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
        );
        put_stream_task_free(task);
        return;
    }
    if !stream_task_identity_matches(
        &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
        &raw mut (*task).config.camera_uuid as *mut ::core::ffi::c_char,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Camera identity changed; dropping queued PUT for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
        );
        put_stream_task_free(task);
        return;
    }
    if update_stream_config(
        &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
        &raw mut (*task).config,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to update stream configuration in database for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
        );
        put_stream_task_free(task);
        return;
    }
    let mut updated_config: stream_config_t = stream_config_t {
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
    };
    if get_stream_config(stream, &raw mut updated_config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to refresh stream configuration from database for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).config.name as *mut ::core::ffi::c_char,
        );
        put_stream_task_free(task);
        return;
    }
    if set_stream_detection_params(
        stream,
        (*task).config.detection_interval,
        (*task).config.detection_threshold,
        (*task).config.pre_detection_buffer,
        (*task).config.post_detection_buffer,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to update detection parameters for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).config.name as *mut ::core::ffi::c_char,
        );
    }
    if set_stream_detection_recording(
        stream,
        (*task).config.detection_based_recording,
        &raw mut (*task).config.detection_model as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to update detection recording for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).config.name as *mut ::core::ffi::c_char,
        );
    }
    if (*task).config_changed as ::core::ffi::c_int != 0
        && ((*task).has_detection_based_recording as ::core::ffi::c_int != 0
            || (*task).has_detection_model as ::core::ffi::c_int != 0
            || (*task).has_detection_threshold as ::core::ffi::c_int != 0
            || (*task).has_detection_interval as ::core::ffi::c_int != 0
            || (*task).has_detection_url as ::core::ffi::c_int != 0)
        && (*task).is_running as ::core::ffi::c_int != 0
        && !(*task).requires_restart
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Detection settings changed for stream %s, marking for restart to apply changes\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*task).config.name as *mut ::core::ffi::c_char,
        );
        (*task).requires_restart = r#true != 0;
    }
    if (*task).has_record {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"PUT stream worker: About to call set_stream_recording for stream '%s' with value %d (%s)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            (*task).config.record as ::core::ffi::c_int,
            if (*task).config.record as ::core::ffi::c_int != 0 {
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"disabled\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        if set_stream_recording(stream, (*task).config.record) != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to update recording setting for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    if (*task).has_streaming_enabled {
        if set_stream_streaming_enabled(stream, (*task).config.streaming_enabled)
            != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to update streaming setting for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut detection_was_enabled: bool = r#false != 0;
    let mut detection_now_enabled: bool = r#false != 0;
    if (*task).has_detection_based_recording {
        detection_was_enabled = (*task).original_detection_based_recording;
        detection_now_enabled = (*task).detection_based_recording_value;
    } else {
        detection_now_enabled = (*task).config.detection_based_recording;
        detection_was_enabled =
            is_unified_detection_running(&raw mut (*task).config.name as *mut ::core::ffi::c_char);
        if detection_now_enabled as ::core::ffi::c_int != 0 && !detection_was_enabled {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Detection is enabled in config for stream %s but no thread is running\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            );
            detection_was_enabled = r#false != 0;
        }
    }
    if detection_was_enabled as ::core::ffi::c_int != 0 && !detection_now_enabled {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Detection disabled for stream %s, stopping unified detection thread\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).config.name as *mut ::core::ffi::c_char,
        );
        if stop_unified_detection_thread(&raw mut (*task).config.name as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to stop unified detection thread for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Successfully stopped unified detection thread for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            );
        }
    } else if !detection_was_enabled && detection_now_enabled as ::core::ffi::c_int != 0 {
        if (*task).config.enabled as ::core::ffi::c_int != 0
            && is_detection_recording_scheduled(&raw mut (*task).config) as ::core::ffi::c_int != 0
        {
            let mut annotation_only: bool = (*task).config.record;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Detection enabled for stream %s, starting unified detection thread with model %s (annotation_only=%s)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                &raw mut (*task).config.detection_model as *mut ::core::ffi::c_char,
                if annotation_only as ::core::ffi::c_int != 0 {
                    b"true\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"false\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            if go2rtc_integration_reload_stream(
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            ) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Successfully ensured stream %s is registered with go2rtc\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to ensure stream %s is registered with go2rtc\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                );
            }
            if start_unified_detection_thread(
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                &raw mut (*task).config.detection_model as *mut ::core::ffi::c_char,
                (*task).config.detection_threshold,
                (*task).config.pre_detection_buffer,
                (*task).config.post_detection_buffer,
                annotation_only,
            ) != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to start unified detection thread for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Successfully started unified detection thread for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                );
            }
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Detection enabled for stream %s but inactive due to stream state or schedule\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            );
        }
    } else if detection_now_enabled as ::core::ffi::c_int != 0
        && ((*task).has_detection_model as ::core::ffi::c_int != 0
            || (*task).has_detection_threshold as ::core::ffi::c_int != 0
            || (*task).has_detection_interval as ::core::ffi::c_int != 0
            || (*task).has_detection_url as ::core::ffi::c_int != 0)
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Detection settings changed for stream %s, restarting unified detection thread\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*task).config.name as *mut ::core::ffi::c_char,
        );
        if stop_unified_detection_thread(&raw mut (*task).config.name as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to stop existing unified detection thread for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            );
        }
        if (*task).config.enabled as ::core::ffi::c_int != 0
            && is_detection_recording_scheduled(&raw mut (*task).config) as ::core::ffi::c_int != 0
        {
            let mut annotation_only_0: bool = (*task).config.record;
            if start_unified_detection_thread(
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                &raw mut (*task).config.detection_model as *mut ::core::ffi::c_char,
                (*task).config.detection_threshold,
                (*task).config.pre_detection_buffer,
                (*task).config.post_detection_buffer,
                annotation_only_0,
            ) != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to restart unified detection thread for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Successfully restarted unified detection thread for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                );
            }
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Updated stream configuration in memory for stream %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*task).config.name as *mut ::core::ffi::c_char,
    );
    if get_stream_config(stream, &raw mut updated_config) == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Detection settings after update - Model: %s, Threshold: %.2f, Interval: %d, Pre-buffer: %d, Post-buffer: %d\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut updated_config.detection_model as *mut ::core::ffi::c_char,
            updated_config.detection_threshold as ::core::ffi::c_double,
            updated_config.detection_interval,
            updated_config.pre_detection_buffer,
            updated_config.post_detection_buffer,
        );
    }
    let mut enabled_changed_to_true: bool =
        !(*task).original_enabled && (*task).config.enabled as ::core::ffi::c_int != 0;
    if (*task).config_changed as ::core::ffi::c_int != 0
        && (enabled_changed_to_true as ::core::ffi::c_int != 0
            || (*task).requires_restart as ::core::ffi::c_int != 0
            || (*task).is_running as ::core::ffi::c_int != 0
                && (*task).non_dynamic_config_changed as ::core::ffi::c_int != 0)
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Restarting stream %s (enabled_now=%s, requires_restart=%s, is_running=%s, non_dynamic_changed=%s)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            if enabled_changed_to_true as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
            if (*task).requires_restart as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
            if (*task).is_running as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
            if (*task).non_dynamic_config_changed as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        let mut url_changed: bool = strcmp(
            &raw mut (*task).original_url as *mut ::core::ffi::c_char,
            &raw mut (*task).config.url as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int;
        let mut protocol_changed: bool = (*task).original_protocol.0 != (*task).config.protocol.0;
        let mut record_audio_changed: bool = (*task).original_record_audio as ::core::ffi::c_int
            != (*task).config.record_audio as ::core::ffi::c_int;
        let mut backchannel_changed: bool = (*task).original_backchannel_enabled
            as ::core::ffi::c_int
            != (*task).config.backchannel_enabled as ::core::ffi::c_int;
        if url_changed {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"URL changed for stream %s, clearing HLS segments\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            );
            if clear_stream_hls_segments(&raw mut (*task).config.name as *mut ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to clear HLS segments for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                );
            }
        }
        if (*task).is_running {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Stopping stream %s for restart\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            );
            if stop_stream(stream) != 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to stop stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
                );
            }
            let mut timeout: ::core::ffi::c_int =
                if (*task).requires_restart as ::core::ffi::c_int != 0 {
                    50 as ::core::ffi::c_int
                } else {
                    30 as ::core::ffi::c_int
                };
            while get_stream_status(stream).0 != stream_status_t::STREAM_STATUS_STOPPED.0
                && timeout > 0 as ::core::ffi::c_int
            {
                usleep(100000 as ::core::ffi::c_int as __useconds_t);
                timeout -= 1;
            }
            if timeout == 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Timeout waiting for stream %s to stop, continuing anyway\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                );
            }
        }
        let mut go2rtc_reloaded: bool = r#false != 0;
        if !(*task).go2rtc_override_changed
            && (*task).config.go2rtc_source_override[0usize] as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int
            && (url_changed as ::core::ffi::c_int != 0
                || protocol_changed as ::core::ffi::c_int != 0
                || record_audio_changed as ::core::ffi::c_int != 0
                || backchannel_changed as ::core::ffi::c_int != 0
                || (*task).credentials_changed as ::core::ffi::c_int != 0)
        {
            go2rtc_reloaded = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"URL, protocol, record_audio, backchannel, or credentials changed for stream %s, updating go2rtc registration\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            );
            if go2rtc_integration_reload_stream_config(
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                &raw mut (*task).config.url as *mut ::core::ffi::c_char,
                if (*task).config.onvif_username[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                {
                    &raw mut (*task).config.onvif_username as *mut ::core::ffi::c_char
                } else {
                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                },
                if (*task).config.onvif_password[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                {
                    &raw mut (*task).config.onvif_password as *mut ::core::ffi::c_char
                } else {
                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                },
                if (*task).config.backchannel_enabled as ::core::ffi::c_int != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                },
                (*task).config.protocol.0 as ::core::ffi::c_int,
                if (*task).config.record_audio as ::core::ffi::c_int != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                },
            ) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Successfully reloaded stream %s in go2rtc with updated config\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to reload stream %s in go2rtc\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                );
            }
            usleep(500000 as ::core::ffi::c_int as __useconds_t);
        }
        if (*task).go2rtc_override_changed {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"go2rtc source override changed for stream %s, restarting go2rtc to load generated YAML\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            );
            if go2rtc_integration_restart_process() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Successfully restarted go2rtc after source override change for stream %s\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to restart go2rtc after source override change for stream %s\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                );
            }
            usleep(500000 as ::core::ffi::c_int as __useconds_t);
        }
        if (*task).sub_stream_changed as ::core::ffi::c_int != 0 && !(*task).go2rtc_override_changed
        {
            let mut sub_name: [::core::ffi::c_char; 264] = [0; 264];
            snprintf(
                &raw mut sub_name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 264]>(),
                b"%s_sub\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            );
            go2rtc_api_remove_stream(&raw mut sub_name as *mut ::core::ffi::c_char);
            if (*task).config.sub_stream_url[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Registering updated sub-stream %s with go2rtc\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut sub_name as *mut ::core::ffi::c_char,
                );
                go2rtc_stream_register_substream(
                    &raw mut sub_name as *mut ::core::ffi::c_char,
                    &raw mut (*task).config.sub_stream_url as *mut ::core::ffi::c_char,
                    if (*task).config.onvif_username[0usize] as ::core::ffi::c_int
                        != '\0' as ::core::ffi::c_int
                    {
                        &raw mut (*task).config.onvif_username as *mut ::core::ffi::c_char
                    } else {
                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                    },
                    if (*task).config.onvif_password[0usize] as ::core::ffi::c_int
                        != '\0' as ::core::ffi::c_int
                    {
                        &raw mut (*task).config.onvif_password as *mut ::core::ffi::c_char
                    } else {
                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                    },
                    (*task).config.protocol,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Sub-stream %s removed from go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut sub_name as *mut ::core::ffi::c_char,
                );
            }
            usleep(500000 as ::core::ffi::c_int as __useconds_t);
        }
        if enabled_changed_to_true as ::core::ffi::c_int != 0
            && !go2rtc_reloaded
            && !(*task).go2rtc_override_changed
        {
            if go2rtc_integration_reload_stream(
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            ) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Registered newly enabled stream %s with go2rtc\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to register newly enabled stream %s with go2rtc (go2rtc may not be ready)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                );
            }
        }
        if (*task).config.enabled {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Starting stream %s after configuration update\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            );
            if start_stream(stream) != 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to restart stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
                );
            }
            if (url_changed as ::core::ffi::c_int != 0
                || protocol_changed as ::core::ffi::c_int != 0)
                && (*task).config.streaming_enabled as ::core::ffi::c_int != 0
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Force restarting HLS stream thread for %s after go2rtc update\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                );
                if stream_restart_hls(&raw mut (*task).config.name as *mut ::core::ffi::c_char)
                    != 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Failed to force restart HLS stream for %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                    );
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Successfully force restarted HLS stream for %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*task).config.name as *mut ::core::ffi::c_char,
                    );
                }
            }
        }
    } else if (*task).config_changed {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Configuration changed for stream %s but restart not required\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).config.name as *mut ::core::ffi::c_char,
        );
    }
    if (*task).config_changed as ::core::ffi::c_int != 0
        && (*task).publish_changed as ::core::ffi::c_int != 0
        && !(*task).go2rtc_override_changed
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Publish URL changed for stream %s, restarting go2rtc to apply restream config\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*task).config.name as *mut ::core::ffi::c_char,
        );
        if go2rtc_integration_restart_process() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Successfully restarted go2rtc after publish URL change for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to restart go2rtc after publish URL change for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*task).config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully completed stream update for: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
    );
    put_stream_task_free(task);
}
unsafe extern "C" fn put_stream_worker(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut task: *mut put_stream_task_t = arg as *mut put_stream_task_t;
    if task.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid PUT stream task\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    pthread_mutex_lock(&raw mut g_put_stream_update_mutex);
    put_stream_worker_locked(task);
    pthread_mutex_unlock(&raw mut g_put_stream_update_mutex);
    return NULL;
}
unsafe extern "C" fn validate_go2rtc_source_override(
    mut src: *const ::core::ffi::c_char,
    mut max_size: size_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut src_len: size_t = strlen(src);
    if src_len >= max_size {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc_source_override too long (%zu bytes, max %zu)\0".as_ptr()
                as *const ::core::ffi::c_char,
            src_len,
            max_size.wrapping_sub(1 as size_t),
        );
        http_response_set_json_error(
            res,
            413 as ::core::ffi::c_int,
            b"go2rtc_source_override exceeds size limit\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if strchr(src, '\n' as ::core::ffi::c_int).is_null() {
        return r#true != 0;
    }
    let mut err: [::core::ffi::c_char; 256] = [
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
    if yaml_validate_str(
        src,
        src_len,
        &raw mut err as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) >= 0 as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Rejecting go2rtc_source_override: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut err as *mut ::core::ffi::c_char,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    if !response.is_null() {
        cJSON_AddBoolToObject(
            response,
            b"valid\0".as_ptr() as *const ::core::ffi::c_char,
            r#false,
        );
        let mut errobj: *mut cJSON = cJSON_CreateObject();
        if !errobj.is_null() {
            cJSON_AddStringToObject(
                errobj,
                b"message\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut err as *mut ::core::ffi::c_char,
            );
            cJSON_AddItemToObject(
                response,
                b"error\0".as_ptr() as *const ::core::ffi::c_char,
                errobj,
            );
        }
        let mut body: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
        cJSON_Delete(response);
        if !body.is_null() {
            http_response_set_json(res, 400 as ::core::ffi::c_int, body);
            free(body as *mut ::core::ffi::c_void);
            return r#false != 0;
        }
    }
    http_response_set_json_error(
        res,
        400 as ::core::ffi::c_int,
        b"go2rtc_source_override is not valid YAML\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return r#false != 0;
}
unsafe extern "C" fn is_allowed_detection_url(mut u: *const ::core::ffi::c_char) -> bool {
    if u.is_null() || *u.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return r#true != 0;
    }
    static mut prefixes: [*const ::core::ffi::c_char; 5] = [
        b"http://\0".as_ptr() as *const ::core::ffi::c_char,
        b"https://\0".as_ptr() as *const ::core::ffi::c_char,
        b"rtsp://\0".as_ptr() as *const ::core::ffi::c_char,
        b"rtsps://\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut i: size_t = 0 as size_t;
    while !prefixes[i].is_null() {
        if strncasecmp(u, prefixes[i], strlen(prefixes[i])) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn is_allowed_publish_url(mut u: *const ::core::ffi::c_char) -> bool {
    return publish_url_list_is_valid(u);
}
unsafe extern "C" fn eptz_number_in_range(
    mut root: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut minimum: ::core::ffi::c_double,
    mut maximum: ::core::ffi::c_double,
) -> bool {
    let mut value: *mut cJSON = cJSON_GetObjectItemCaseSensitive(root, name);
    return cJSON_IsNumber(value) != 0
        && (*value).valuedouble >= minimum
        && (*value).valuedouble <= maximum;
}
unsafe extern "C" fn copy_valid_eptz_config(
    mut value: *mut cJSON,
    mut destination: *mut ::core::ffi::c_char,
    mut destination_size: size_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> bool {
    if value.is_null() || destination.is_null() || destination_size == 0 as size_t {
        return r#false != 0;
    }
    if cJSON_IsString(value) == 0 || (*value).valuestring.is_null() {
        snprintf(
            error,
            error_size,
            b"eptz_config must be a JSON string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut text: *const ::core::ffi::c_char = (*value).valuestring;
    if *text.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        *destination.offset(0isize) = '\0' as ::core::ffi::c_char;
        return r#true != 0;
    }
    if strlen(text) >= destination_size {
        snprintf(
            error,
            error_size,
            b"eptz_config exceeds size limit\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut root: *mut cJSON = cJSON_Parse(text);
    let mut version: *mut cJSON = if !root.is_null() {
        cJSON_GetObjectItemCaseSensitive(root, b"version\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut projection: *mut cJSON = if !root.is_null() {
        cJSON_GetObjectItemCaseSensitive(
            root,
            b"projection\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut mount: *mut cJSON = if !root.is_null() {
        cJSON_GetObjectItemCaseSensitive(root, b"mount\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut valid: bool = cJSON_IsObject(root) != 0
        && cJSON_IsNumber(version) != 0
        && (*version).valuedouble == 1.0f64
        && cJSON_IsString(projection) != 0
        && strcmp(
            (*projection).valuestring,
            b"equidistant\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && cJSON_IsString(mount) != 0
        && strcmp(
            (*mount).valuestring,
            b"ceiling\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && eptz_number_in_range(
            root,
            b"centerX\0".as_ptr() as *const ::core::ffi::c_char,
            0.0f64,
            1.0f64,
        ) as ::core::ffi::c_int
            != 0
        && eptz_number_in_range(
            root,
            b"centerY\0".as_ptr() as *const ::core::ffi::c_char,
            0.0f64,
            1.0f64,
        ) as ::core::ffi::c_int
            != 0
        && eptz_number_in_range(
            root,
            b"radius\0".as_ptr() as *const ::core::ffi::c_char,
            0.05f64,
            1.0f64,
        ) as ::core::ffi::c_int
            != 0
        && eptz_number_in_range(
            root,
            b"fov\0".as_ptr() as *const ::core::ffi::c_char,
            160.0f64,
            360.0f64,
        ) as ::core::ffi::c_int
            != 0
        && eptz_number_in_range(
            root,
            b"rotation\0".as_ptr() as *const ::core::ffi::c_char,
            -360.0f64,
            360.0f64,
        ) as ::core::ffi::c_int
            != 0
        && eptz_number_in_range(
            root,
            b"defaultYaw\0".as_ptr() as *const ::core::ffi::c_char,
            -360.0f64,
            360.0f64,
        ) as ::core::ffi::c_int
            != 0
        && eptz_number_in_range(
            root,
            b"defaultTilt\0".as_ptr() as *const ::core::ffi::c_char,
            -90.0f64,
            30.0f64,
        ) as ::core::ffi::c_int
            != 0
        && eptz_number_in_range(
            root,
            b"defaultViewFov\0".as_ptr() as *const ::core::ffi::c_char,
            20.0f64,
            120.0f64,
        ) as ::core::ffi::c_int
            != 0;
    cJSON_Delete(root);
    if !valid {
        snprintf(
            error,
            error_size,
            b"eptz_config is not a supported version-1 equidistant ceiling configuration\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(destination, text, destination_size, 0 as size_t);
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_stream(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/streams request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !check_stream_write_access(req, res, ::core::ptr::null_mut::<stream_config_t>()) {
        return;
    }
    let mut stream_json: *mut cJSON = httpd_parse_json_body(req);
    if stream_json.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to parse stream JSON from request body\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON in request body\0".as_ptr() as *const ::core::ffi::c_char,
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
    };
    memset(
        &raw mut config as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<stream_config_t>(),
    );
    let mut name: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut url: *mut cJSON =
        cJSON_GetObjectItem(stream_json, b"url\0".as_ptr() as *const ::core::ffi::c_char);
    if name.is_null() || cJSON_IsString(name) == 0 || url.is_null() || cJSON_IsString(url) == 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Missing required fields in stream configuration\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        cJSON_Delete(stream_json);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing required fields (name, url)\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut name_str: *const ::core::ffi::c_char = (*name).valuestring;
    while *name_str as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || *name_str as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
    {
        name_str = name_str.offset(1);
    }
    if *name_str as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Stream name is empty or whitespace-only\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(stream_json);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Stream name cannot be empty\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_strcpy(
        &raw mut config.name as *mut ::core::ffi::c_char,
        name_str,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    let mut name_len: size_t = strlen(&raw mut config.name as *mut ::core::ffi::c_char);
    while name_len > 0 as size_t
        && (config.name[name_len.wrapping_sub(1 as size_t)] as ::core::ffi::c_int
            == ' ' as ::core::ffi::c_int
            || config.name[name_len.wrapping_sub(1 as size_t)] as ::core::ffi::c_int
                == '\t' as ::core::ffi::c_int)
    {
        name_len = name_len.wrapping_sub(1);
        config.name[name_len] = '\0' as ::core::ffi::c_char;
    }
    if !is_valid_stream_name(&raw mut config.name as *mut ::core::ffi::c_char) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid stream name: '%s' (allowed: letters, digits, '.', '-', '_')\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config.name as *mut ::core::ffi::c_char,
        );
        cJSON_Delete(stream_json);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Stream name can only contain letters, digits, '.', '-', and '_'\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_strcpy(
        &raw mut config.url as *mut ::core::ffi::c_char,
        (*url).valuestring,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    config.enabled = r#true != 0;
    config.streaming_enabled = r#true != 0;
    config.width = 0 as ::core::ffi::c_int;
    config.height = 0 as ::core::ffi::c_int;
    config.fps = 0 as ::core::ffi::c_int;
    config.codec[0usize] = '\0' as ::core::ffi::c_char;
    config.priority = 5 as ::core::ffi::c_int;
    config.record = r#true != 0;
    config.segment_duration = 30 as ::core::ffi::c_int;
    config.detection_based_recording = r#false != 0;
    config.detection_interval = 10 as ::core::ffi::c_int;
    config.detection_threshold = 0.5f32;
    config.pre_detection_buffer = 5 as ::core::ffi::c_int;
    config.post_detection_buffer = 5 as ::core::ffi::c_int;
    config.protocol = stream_protocol_t::STREAM_PROTOCOL_TCP;
    config.record_audio = r#true != 0;
    config.retention_days = -1 as ::core::ffi::c_int;
    config.detection_retention_days = -1 as ::core::ffi::c_int;
    safe_strcpy(
        &raw mut config.playback_transport as *mut ::core::ffi::c_char,
        if playback_transport_is_valid(
            &raw mut g_config.default_playback_transport as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        {
            &raw mut g_config.default_playback_transport as *mut ::core::ffi::c_char
        } else {
            b"auto\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        0 as size_t,
    );
    memset(
        &raw mut config.recording_schedule as *mut uint8_t as *mut ::core::ffi::c_void,
        1 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 168]>(),
    );
    memset(
        &raw mut config.detection_recording_schedule as *mut uint8_t as *mut ::core::ffi::c_void,
        1 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 168]>(),
    );
    let mut enabled: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !enabled.is_null() && cJSON_IsBool(enabled) != 0 {
        config.enabled = cJSON_IsTrue(enabled) != 0;
    }
    let mut streaming_enabled: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"streaming_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !streaming_enabled.is_null() && cJSON_IsBool(streaming_enabled) != 0 {
        config.streaming_enabled = cJSON_IsTrue(streaming_enabled) != 0;
    }
    let mut playback_transport: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"playback_transport\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !playback_transport.is_null() {
        if cJSON_IsString(playback_transport) == 0
            || !playback_transport_is_valid((*playback_transport).valuestring)
        {
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"playback_transport is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        safe_strcpy(
            &raw mut config.playback_transport as *mut ::core::ffi::c_char,
            (*playback_transport).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
            0 as size_t,
        );
    }
    let mut eptz_config: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"eptz_config\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !eptz_config.is_null() {
        let mut eptz_error: [::core::ffi::c_char; 160] = [0; 160];
        if !copy_valid_eptz_config(
            eptz_config,
            &raw mut config.eptz_config as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            &raw mut eptz_error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
        ) {
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                &raw mut eptz_error as *mut ::core::ffi::c_char,
            );
            return;
        }
    }
    let mut priority: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"priority\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !priority.is_null() && cJSON_IsNumber(priority) != 0 {
        config.priority = (*priority).valueint;
    }
    let mut record: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"record\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !record.is_null() && cJSON_IsBool(record) != 0 {
        config.record = cJSON_IsTrue(record) != 0;
    }
    let mut segment_duration: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"segment_duration\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !segment_duration.is_null() && cJSON_IsNumber(segment_duration) != 0 {
        config.segment_duration = (*segment_duration).valueint;
    }
    let mut detection_based_recording: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_based_recording\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_based_recording.is_null() && cJSON_IsBool(detection_based_recording) != 0 {
        config.detection_based_recording = cJSON_IsTrue(detection_based_recording) != 0;
    }
    let mut detection_model: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_model\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_model.is_null() && cJSON_IsString(detection_model) != 0 {
        safe_strcpy(
            &raw mut config.detection_model as *mut ::core::ffi::c_char,
            (*detection_model).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    let mut detection_threshold: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_threshold\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_threshold.is_null() && cJSON_IsNumber(detection_threshold) != 0 {
        config.detection_threshold =
            ((*detection_threshold).valuedouble / 100.0f64) as ::core::ffi::c_float;
    }
    let mut detection_interval: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_interval\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_interval.is_null() && cJSON_IsNumber(detection_interval) != 0 {
        config.detection_interval = (*detection_interval).valueint;
    }
    let mut pre_detection_buffer: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"pre_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !pre_detection_buffer.is_null() && cJSON_IsNumber(pre_detection_buffer) != 0 {
        config.pre_detection_buffer = (*pre_detection_buffer).valueint;
    }
    let mut post_detection_buffer: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"post_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !post_detection_buffer.is_null() && cJSON_IsNumber(post_detection_buffer) != 0 {
        config.post_detection_buffer = (*post_detection_buffer).valueint;
    }
    let mut detection_object_filter: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_object_filter\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_object_filter.is_null() && cJSON_IsString(detection_object_filter) != 0 {
        safe_strcpy(
            &raw mut config.detection_object_filter as *mut ::core::ffi::c_char,
            (*detection_object_filter).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
    }
    let mut detection_object_filter_list: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_object_filter_list\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_object_filter_list.is_null() && cJSON_IsString(detection_object_filter_list) != 0
    {
        safe_strcpy(
            &raw mut config.detection_object_filter_list as *mut ::core::ffi::c_char,
            (*detection_object_filter_list).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
    }
    let mut protocol: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"protocol\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !protocol.is_null() && cJSON_IsNumber(protocol) != 0 {
        config.protocol = stream_protocol_t((*protocol).valueint as ::core::ffi::c_uint);
    }
    let mut codec_hint: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"codec\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !codec_hint.is_null() && cJSON_IsString(codec_hint) != 0 {
        safe_strcpy(
            &raw mut config.codec as *mut ::core::ffi::c_char,
            (*codec_hint).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
    }
    let mut record_audio: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"record_audio\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !record_audio.is_null() && cJSON_IsBool(record_audio) != 0 {
        config.record_audio = cJSON_IsTrue(record_audio) != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Audio recording %s for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            if config.record_audio as ::core::ffi::c_int != 0 {
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"disabled\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw mut config.name as *mut ::core::ffi::c_char,
        );
    }
    let mut audio_voice_enhancement: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"audio_voice_enhancement\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !audio_voice_enhancement.is_null() && cJSON_IsBool(audio_voice_enhancement) != 0 {
        config.audio_voice_enhancement = cJSON_IsTrue(audio_voice_enhancement) != 0;
    }
    let mut backchannel_enabled: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"backchannel_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !backchannel_enabled.is_null() && cJSON_IsBool(backchannel_enabled) != 0 {
        config.backchannel_enabled = cJSON_IsTrue(backchannel_enabled) != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Backchannel audio %s for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            if config.backchannel_enabled as ::core::ffi::c_int != 0 {
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"disabled\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw mut config.name as *mut ::core::ffi::c_char,
        );
    }
    let mut retention_days: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !retention_days.is_null() && cJSON_IsNumber(retention_days) != 0 {
        if (*retention_days).valueint < -1 as ::core::ffi::c_int
            || (*retention_days).valueint > 365 as ::core::ffi::c_int
        {
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"retention_days must be between -1 and 365\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        config.retention_days = (*retention_days).valueint;
    }
    let mut detection_retention_days: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_retention_days.is_null() && cJSON_IsNumber(detection_retention_days) != 0 {
        if (*detection_retention_days).valueint < -1 as ::core::ffi::c_int
            || (*detection_retention_days).valueint > 365 as ::core::ffi::c_int
        {
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"detection_retention_days must be between -1 and 365\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        config.detection_retention_days = (*detection_retention_days).valueint;
    }
    let mut max_storage_mb: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"max_storage_mb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !max_storage_mb.is_null() && cJSON_IsNumber(max_storage_mb) != 0 {
        config.max_storage_mb = (*max_storage_mb).valueint;
    }
    let mut tier_critical_multiplier: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"tier_critical_multiplier\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !tier_critical_multiplier.is_null() && cJSON_IsNumber(tier_critical_multiplier) != 0 {
        config.tier_critical_multiplier = (*tier_critical_multiplier).valuedouble;
    }
    let mut tier_important_multiplier: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"tier_important_multiplier\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !tier_important_multiplier.is_null() && cJSON_IsNumber(tier_important_multiplier) != 0 {
        config.tier_important_multiplier = (*tier_important_multiplier).valuedouble;
    }
    let mut tier_ephemeral_multiplier: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"tier_ephemeral_multiplier\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !tier_ephemeral_multiplier.is_null() && cJSON_IsNumber(tier_ephemeral_multiplier) != 0 {
        config.tier_ephemeral_multiplier = (*tier_ephemeral_multiplier).valuedouble;
    }
    let mut storage_priority: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"storage_priority\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !storage_priority.is_null() && cJSON_IsNumber(storage_priority) != 0 {
        config.storage_priority = (*storage_priority).valueint;
    }
    let mut ptz_enabled: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"ptz_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !ptz_enabled.is_null() && cJSON_IsBool(ptz_enabled) != 0 {
        config.ptz_enabled = cJSON_IsTrue(ptz_enabled) != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"PTZ %s for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            if config.ptz_enabled as ::core::ffi::c_int != 0 {
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"disabled\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw mut config.name as *mut ::core::ffi::c_char,
        );
    }
    let mut ptz_max_x: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"ptz_max_x\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !ptz_max_x.is_null() && cJSON_IsNumber(ptz_max_x) != 0 {
        config.ptz_max_x = (*ptz_max_x).valueint;
    }
    let mut ptz_max_y: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"ptz_max_y\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !ptz_max_y.is_null() && cJSON_IsNumber(ptz_max_y) != 0 {
        config.ptz_max_y = (*ptz_max_y).valueint;
    }
    let mut ptz_max_z: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"ptz_max_z\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !ptz_max_z.is_null() && cJSON_IsNumber(ptz_max_z) != 0 {
        config.ptz_max_z = (*ptz_max_z).valueint;
    }
    let mut ptz_has_home: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"ptz_has_home\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !ptz_has_home.is_null() && cJSON_IsBool(ptz_has_home) != 0 {
        config.ptz_has_home = cJSON_IsTrue(ptz_has_home) != 0;
    }
    let mut record_on_schedule: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"record_on_schedule\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !record_on_schedule.is_null() && cJSON_IsBool(record_on_schedule) != 0 {
        config.record_on_schedule = cJSON_IsTrue(record_on_schedule) != 0;
    }
    let mut recording_schedule: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"recording_schedule\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !recording_schedule.is_null() && cJSON_IsArray(recording_schedule) != 0 {
        let mut arr_size: ::core::ffi::c_int = cJSON_GetArraySize(recording_schedule);
        if arr_size == 168 as ::core::ffi::c_int {
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < 168 as ::core::ffi::c_int {
                let mut item: *mut cJSON = cJSON_GetArrayItem(recording_schedule, j);
                config.recording_schedule[j as usize] =
                    (if !item.is_null() && cJSON_IsTrue(item) != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t;
                j += 1;
            }
        }
    }
    let mut detection_record_on_schedule: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_record_on_schedule\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_record_on_schedule.is_null() && cJSON_IsBool(detection_record_on_schedule) != 0 {
        config.detection_record_on_schedule = cJSON_IsTrue(detection_record_on_schedule) != 0;
    }
    let mut detection_recording_schedule: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_recording_schedule\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_recording_schedule.is_null()
        && cJSON_IsArray(detection_recording_schedule) != 0
        && cJSON_GetArraySize(detection_recording_schedule) == 168 as ::core::ffi::c_int
    {
        let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j_0 < 168 as ::core::ffi::c_int {
            let mut item_0: *mut cJSON = cJSON_GetArrayItem(detection_recording_schedule, j_0);
            config.detection_recording_schedule[j_0 as usize] =
                (if !item_0.is_null() && cJSON_IsTrue(item_0) != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t;
            j_0 += 1;
        }
    }
    let mut tags_post: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"tags\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !tags_post.is_null() && cJSON_IsString(tags_post) != 0 {
        safe_strcpy(
            &raw mut config.tags as *mut ::core::ffi::c_char,
            (*tags_post).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
    } else {
        config.tags[0usize] = '\0' as ::core::ffi::c_char;
    }
    let mut admin_url_post: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"admin_url\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !admin_url_post.is_null() && cJSON_IsString(admin_url_post) != 0 {
        safe_strcpy(
            &raw mut config.admin_url as *mut ::core::ffi::c_char,
            (*admin_url_post).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    } else {
        config.admin_url[0usize] = '\0' as ::core::ffi::c_char;
    }
    let mut motion_trigger_source_post: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"motion_trigger_source\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !motion_trigger_source_post.is_null() && cJSON_IsString(motion_trigger_source_post) != 0 {
        safe_strcpy(
            &raw mut config.motion_trigger_source as *mut ::core::ffi::c_char,
            (*motion_trigger_source_post).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
    } else {
        config.motion_trigger_source[0usize] = '\0' as ::core::ffi::c_char;
    }
    let mut go2rtc_source_override_post: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"go2rtc_source_override\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_source_override_post.is_null() && cJSON_IsString(go2rtc_source_override_post) != 0 {
        let mut src: *const ::core::ffi::c_char = (*go2rtc_source_override_post).valuestring;
        if !string_has_non_whitespace(src) {
            config.go2rtc_source_override[0usize] = '\0' as ::core::ffi::c_char;
        } else {
            if !validate_go2rtc_source_override(
                src,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
                res,
            ) {
                cJSON_Delete(stream_json);
                return;
            }
            safe_strcpy(
                &raw mut config.go2rtc_source_override as *mut ::core::ffi::c_char,
                src,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
                0 as size_t,
            );
        }
    } else {
        config.go2rtc_source_override[0usize] = '\0' as ::core::ffi::c_char;
    }
    let mut sub_stream_url_post: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"sub_stream_url\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !sub_stream_url_post.is_null() && cJSON_IsString(sub_stream_url_post) != 0 {
        safe_strcpy(
            &raw mut config.sub_stream_url as *mut ::core::ffi::c_char,
            (*sub_stream_url_post).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    } else {
        config.sub_stream_url[0usize] = '\0' as ::core::ffi::c_char;
    }
    let mut detection_url_post: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_url\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_url_post.is_null() && cJSON_IsString(detection_url_post) != 0 {
        if !is_allowed_detection_url((*detection_url_post).valuestring) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Rejected detection_url with disallowed scheme for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"detection_url must use http, https, rtsp, or rtsps\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        if strlen((*detection_url_post).valuestring)
            >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"detection_url too long (%zu bytes, max %zu) for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strlen((*detection_url_post).valuestring),
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>().wrapping_sub(1usize),
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                413 as ::core::ffi::c_int,
                b"detection_url exceeds size limit\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        safe_strcpy(
            &raw mut config.detection_url as *mut ::core::ffi::c_char,
            (*detection_url_post).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    } else {
        config.detection_url[0usize] = '\0' as ::core::ffi::c_char;
    }
    let mut publish_url_post: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"publish_url\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !publish_url_post.is_null() && cJSON_IsString(publish_url_post) != 0 {
        let mut r#pub: *const ::core::ffi::c_char = (*publish_url_post).valuestring;
        if *r#pub.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && !is_allowed_publish_url(r#pub)
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Rejected invalid publish_url for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"publish_url destinations must be rtmp:// or rtmps:// URLs\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        if strlen(r#pub) >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"publish_url too long (%zu bytes, max %zu) for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strlen(r#pub),
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>().wrapping_sub(1usize),
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                413 as ::core::ffi::c_int,
                b"publish_url exceeds size limit\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        safe_strcpy(
            &raw mut config.publish_url as *mut ::core::ffi::c_char,
            r#pub,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    } else {
        config.publish_url[0usize] = '\0' as ::core::ffi::c_char;
    }
    let mut is_onvif: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"isOnvif\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !is_onvif.is_null() && cJSON_IsBool(is_onvif) != 0 {
        config.is_onvif = cJSON_IsTrue(is_onvif) != 0;
    } else {
        config.is_onvif = !strstr(
            &raw mut config.url as *mut ::core::ffi::c_char,
            b"onvif\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null();
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"ONVIF flag for stream %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.name as *mut ::core::ffi::c_char,
        if config.is_onvif as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut onvif_port_json: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"onvif_port\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !onvif_port_json.is_null() && cJSON_IsNumber(onvif_port_json) != 0 {
        config.onvif_port = (*onvif_port_json).valueint;
    }
    let mut onvif_test_success: bool = r#true != 0;
    let mut onvif_test_performed: bool = r#false != 0;
    if config.is_onvif {
        onvif_test_performed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Testing ONVIF capabilities for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut config.name as *mut ::core::ffi::c_char,
        );
        let mut onvif_username: *mut cJSON = cJSON_GetObjectItem(
            stream_json,
            b"onvif_username\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut onvif_password: *mut cJSON = cJSON_GetObjectItem(
            stream_json,
            b"onvif_password\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut onvif_profile: *mut cJSON = cJSON_GetObjectItem(
            stream_json,
            b"onvif_profile\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !onvif_username.is_null() && cJSON_IsString(onvif_username) != 0 {
            safe_strcpy(
                &raw mut config.onvif_username as *mut ::core::ffi::c_char,
                (*onvif_username).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
        if !onvif_password.is_null() && cJSON_IsString(onvif_password) != 0 {
            safe_strcpy(
                &raw mut config.onvif_password as *mut ::core::ffi::c_char,
                (*onvif_password).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
        if !onvif_profile.is_null() && cJSON_IsString(onvif_profile) != 0 {
            safe_strcpy(
                &raw mut config.onvif_profile as *mut ::core::ffi::c_char,
                (*onvif_profile).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
        normalize_stream_url_credentials(&raw mut config);
        let mut onvif_device_url: [::core::ffi::c_char; 512] = [0; 512];
        if build_onvif_test_url(
            &raw mut config,
            &raw mut onvif_device_url as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut onvif_device_url as *mut ::core::ffi::c_char,
                &raw mut config.url as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        }
        let mut result: ::core::ffi::c_int = test_onvif_connection(
            &raw mut onvif_device_url as *mut ::core::ffi::c_char,
            if config.onvif_username[0usize] as ::core::ffi::c_int != 0 {
                &raw mut config.onvif_username as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            if config.onvif_password[0usize] as ::core::ffi::c_int != 0 {
                &raw mut config.onvif_password as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
        );
        onvif_test_success = result == 0 as ::core::ffi::c_int;
        if !onvif_test_success {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"ONVIF test failed for stream %s; keeping user-selected ONVIF flag\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    cJSON_Delete(stream_json);
    let mut existing_stream: stream_handle_t =
        get_stream_by_name(&raw mut config.name as *mut ::core::ffi::c_char);
    if !existing_stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Stream already exists: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut config.name as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            409 as ::core::ffi::c_int,
            b"Stream already exists\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut runtime_capacity: ::core::ffi::c_int = get_stream_capacity();
    let mut current_total: ::core::ffi::c_int = get_total_stream_count();
    if runtime_capacity > 0 as ::core::ffi::c_int && current_total >= runtime_capacity {
        let mut error_message: [::core::ffi::c_char; 256] = [0; 256];
        format_stream_capacity_error(
            &raw mut error_message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            runtime_capacity,
            g_config.max_streams,
            current_total,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Rejecting stream '%s': %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut config.name as *mut ::core::ffi::c_char,
            &raw mut error_message as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            409 as ::core::ffi::c_int,
            &raw mut error_message as *mut ::core::ffi::c_char,
        );
        return;
    }
    let mut stream_id: uint64_t = add_stream_config(&raw mut config);
    if stream_id == 0 as uint64_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to add stream configuration to database\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to add stream configuration\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut persisted_config: *mut stream_config_t =
        calloc(1 as size_t, ::core::mem::size_of::<stream_config_t>()) as *mut stream_config_t;
    if persisted_config.is_null()
        || get_stream_config_by_name(
            &raw mut config.name as *mut ::core::ffi::c_char,
            persisted_config,
        ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to load generated camera UUID for stream: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config.name as *mut ::core::ffi::c_char,
        );
        free(persisted_config as *mut ::core::ffi::c_void);
        delete_stream_config_internal(
            &raw mut config.name as *mut ::core::ffi::c_char,
            r#true != 0,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load created stream identity\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_strcpy(
        &raw mut config.camera_uuid as *mut ::core::ffi::c_char,
        &raw mut (*persisted_config).camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut config.location_uuid as *mut ::core::ffi::c_char,
        &raw mut (*persisted_config).location_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    free(persisted_config as *mut ::core::ffi::c_void);
    let mut stream: stream_handle_t = add_stream(&raw mut config);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut config.name as *mut ::core::ffi::c_char,
        );
        delete_stream_config(&raw mut config.name as *mut ::core::ffi::c_char);
        runtime_capacity = get_stream_capacity();
        current_total = get_total_stream_count();
        if runtime_capacity > 0 as ::core::ffi::c_int && current_total >= runtime_capacity {
            let mut error_message_0: [::core::ffi::c_char; 256] = [0; 256];
            format_stream_capacity_error(
                &raw mut error_message_0 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                runtime_capacity,
                g_config.max_streams,
                current_total,
            );
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                &raw mut error_message_0 as *mut ::core::ffi::c_char,
            );
            return;
        }
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if config.enabled {
        if start_stream(stream) != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to start stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
        if config.detection_based_recording as ::core::ffi::c_int != 0
            && is_detection_recording_scheduled(&raw mut config) as ::core::ffi::c_int != 0
        {
            let mut annotation_only: bool = config.record;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Detection enabled for new stream %s, starting unified detection thread with model %s (annotation_only=%s)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
                &raw mut config.detection_model as *mut ::core::ffi::c_char,
                if annotation_only as ::core::ffi::c_int != 0 {
                    b"true\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"false\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            if start_unified_detection_thread(
                &raw mut config.name as *mut ::core::ffi::c_char,
                &raw mut config.detection_model as *mut ::core::ffi::c_char,
                config.detection_threshold,
                config.pre_detection_buffer,
                config.post_detection_buffer,
                annotation_only,
            ) != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to start unified detection thread for new stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut config.name as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Successfully started unified detection thread for new stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut config.name as *mut ::core::ffi::c_char,
                );
            }
        }
    }
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
        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.camera_uuid as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        success,
        b"location_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.location_uuid as *mut ::core::ffi::c_char,
    );
    if onvif_test_performed {
        if onvif_test_success {
            cJSON_AddStringToObject(
                success,
                b"onvif_status\0".as_ptr() as *const ::core::ffi::c_char,
                b"success\0".as_ptr() as *const ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                success,
                b"onvif_message\0".as_ptr() as *const ::core::ffi::c_char,
                b"ONVIF capabilities detected successfully\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            cJSON_AddStringToObject(
                success,
                b"onvif_status\0".as_ptr() as *const ::core::ffi::c_char,
                b"error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                success,
                b"onvif_message\0".as_ptr() as *const ::core::ffi::c_char,
                b"Failed to detect ONVIF capabilities\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(success);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert success JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(success);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert success JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 201 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(success);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully created stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.name as *mut ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_stream(
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
        b"Handling PUT /api/streams/%s request\0".as_ptr() as *const ::core::ffi::c_char,
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
    if !check_stream_write_access(req, res, &raw mut config) {
        return;
    }
    let mut stream_json: *mut cJSON = httpd_parse_json_body(req);
    if stream_json.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to parse stream JSON from request body\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON in request body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut privacy_request: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"set_privacy_mode\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !privacy_request.is_null() && cJSON_IsBool(privacy_request) != 0 {
        let mut field: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
        field = (if !stream_json.is_null() {
            (*stream_json).child
        } else {
            ::core::ptr::null_mut::<cJSON>()
        }) as *mut cJSON;
        while !field.is_null() {
            if (*field).string.is_null()
                || strcmp(
                    (*field).string,
                    b"set_privacy_mode\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                    && strcmp(
                        (*field).string,
                        b"name\0".as_ptr() as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
            {
                cJSON_Delete(stream_json);
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"set_privacy_mode cannot be combined with other update fields\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return;
            }
            field = (*field).next as *mut cJSON;
        }
        let mut enable_privacy: bool = cJSON_IsTrue(privacy_request) != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Privacy mode %s requested for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            if enable_privacy as ::core::ffi::c_int != 0 {
                b"enable\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"disable\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw mut stream_id as *mut ::core::ffi::c_char,
        );
        if set_stream_privacy_mode(
            &raw mut stream_id as *mut ::core::ffi::c_char,
            enable_privacy,
        ) != 0 as ::core::ffi::c_int
        {
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to update privacy mode\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Successfully set privacy_mode=%d for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            if enable_privacy as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
            &raw mut stream_id as *mut ::core::ffi::c_char,
        );
        let mut sub_name: [::core::ffi::c_char; 264] = [0; 264];
        snprintf(
            &raw mut sub_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 264]>(),
            b"%s_sub\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_id as *mut ::core::ffi::c_char,
        );
        let mut has_sub_stream: bool =
            config.sub_stream_url[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
        let mut yaml_backed: bool = !g_config.audio_disabled
            && config.go2rtc_source_override[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            || config.publish_url[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
        let mut go2rtc_applied: bool = r#true != 0;
        if enable_privacy {
            if go2rtc_stream_is_ready() {
                if !go2rtc_integration_unregister_stream(
                    &raw mut stream_id as *mut ::core::ffi::c_char,
                ) {
                    go2rtc_applied = r#false != 0;
                }
                if has_sub_stream as ::core::ffi::c_int != 0
                    && !go2rtc_api_remove_stream(&raw mut sub_name as *mut ::core::ffi::c_char)
                {
                    go2rtc_applied = r#false != 0;
                }
                if yaml_backed as ::core::ffi::c_int != 0 && !go2rtc_integration_restart_process() {
                    go2rtc_applied = r#false != 0;
                }
            }
            if go2rtc_applied {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Stream %s is no longer served by go2rtc (privacy mode)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_id as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Privacy mode saved for stream %s but go2rtc still serves it\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_id as *mut ::core::ffi::c_char,
                );
            }
        } else if !config.enabled {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Privacy mode cleared for disabled stream %s; not registering with go2rtc\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut stream_id as *mut ::core::ffi::c_char,
            );
        } else if yaml_backed {
            if go2rtc_integration_restart_process() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Restarted go2rtc to restore stream %s after privacy mode cleared\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_id as *mut ::core::ffi::c_char,
                );
            } else {
                go2rtc_applied = r#false != 0;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to restart go2rtc to restore stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_id as *mut ::core::ffi::c_char,
                );
            }
        } else {
            if go2rtc_integration_reload_stream(&raw mut stream_id as *mut ::core::ffi::c_char) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Re-registered stream %s with go2rtc after privacy mode cleared\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_id as *mut ::core::ffi::c_char,
                );
            } else {
                go2rtc_applied = r#false != 0;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to re-register stream %s with go2rtc\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_id as *mut ::core::ffi::c_char,
                );
            }
            if has_sub_stream as ::core::ffi::c_int != 0
                && !go2rtc_stream_register_substream(
                    &raw mut sub_name as *mut ::core::ffi::c_char,
                    &raw mut config.sub_stream_url as *mut ::core::ffi::c_char,
                    if config.onvif_username[0usize] as ::core::ffi::c_int
                        != '\0' as ::core::ffi::c_int
                    {
                        &raw mut config.onvif_username as *mut ::core::ffi::c_char
                    } else {
                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                    },
                    if config.onvif_password[0usize] as ::core::ffi::c_int
                        != '\0' as ::core::ffi::c_int
                    {
                        &raw mut config.onvif_password as *mut ::core::ffi::c_char
                    } else {
                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                    },
                    config.protocol,
                )
            {
                go2rtc_applied = r#false != 0;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to re-register sub-stream %s with go2rtc\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut sub_name as *mut ::core::ffi::c_char,
                );
            }
        }
        cJSON_Delete(stream_json);
        if enable_privacy as ::core::ffi::c_int != 0 && !go2rtc_applied {
            http_response_set_json_error(
                res,
                502 as ::core::ffi::c_int,
                b"Privacy mode was saved but the stream could not be removed from go2rtc; retry\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut response: *mut cJSON = cJSON_CreateObject();
        if !response.is_null() {
            cJSON_AddBoolToObject(
                response,
                b"success\0".as_ptr() as *const ::core::ffi::c_char,
                r#true,
            );
            cJSON_AddBoolToObject(
                response,
                b"privacy_mode\0".as_ptr() as *const ::core::ffi::c_char,
                enable_privacy as cJSON_bool,
            );
            cJSON_AddBoolToObject(
                response,
                b"stream_registered\0".as_ptr() as *const ::core::ffi::c_char,
                (!enable_privacy
                    && config.enabled as ::core::ffi::c_int != 0
                    && go2rtc_applied as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int,
            );
            let mut response_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
            cJSON_Delete(response);
            if !response_str.is_null() {
                http_response_set_json(res, 200 as ::core::ffi::c_int, response_str);
                free(response_str as *mut ::core::ffi::c_void);
                return;
            }
        }
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut config_changed: bool = r#false != 0;
    let mut requires_restart: bool = r#false != 0;
    let mut go2rtc_override_changed: bool = r#false != 0;
    let mut sub_stream_changed: bool = r#false != 0;
    let mut has_record: bool = r#false != 0;
    let mut has_streaming_enabled: bool = r#false != 0;
    let mut non_dynamic_config_changed: bool = r#false != 0;
    let mut credentials_changed: bool = r#false != 0;
    let mut original_url: [::core::ffi::c_char; 512] = [0; 512];
    safe_strcpy(
        &raw mut original_url as *mut ::core::ffi::c_char,
        &raw mut config.url as *mut ::core::ffi::c_char,
        MAX_URL_LENGTH as size_t,
        0 as size_t,
    );
    let mut original_protocol: stream_protocol_t = config.protocol;
    let mut original_record_audio: bool = config.record_audio;
    let mut original_backchannel_enabled: bool = config.backchannel_enabled;
    let mut url: *mut cJSON =
        cJSON_GetObjectItem(stream_json, b"url\0".as_ptr() as *const ::core::ffi::c_char);
    if !url.is_null() && cJSON_IsString(url) != 0 {
        if strcmp(
            &raw mut config.url as *mut ::core::ffi::c_char,
            (*url).valuestring,
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut config.url as *mut ::core::ffi::c_char,
                (*url).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
            config_changed = r#true != 0;
            requires_restart = r#true != 0;
            let mut safe_original_url: [::core::ffi::c_char; 512] = [0; 512];
            let mut safe_new_url: [::core::ffi::c_char; 512] = [0; 512];
            redact_url_for_log(
                &raw mut original_url as *mut ::core::ffi::c_char,
                &raw mut safe_original_url as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            );
            redact_url_for_log(
                &raw mut config.url as *mut ::core::ffi::c_char,
                &raw mut safe_new_url as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"URL changed from '%s' to '%s' - restart required\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut safe_original_url as *mut ::core::ffi::c_char,
                &raw mut safe_new_url as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut original_enabled: bool = config.enabled;
    let mut enabled: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !enabled.is_null() && cJSON_IsBool(enabled) != 0 {
        let mut new_enabled: bool = cJSON_IsTrue(enabled) != 0;
        if config.enabled as ::core::ffi::c_int != new_enabled as ::core::ffi::c_int {
            config.enabled = new_enabled;
            config_changed = r#true != 0;
            non_dynamic_config_changed = r#true != 0;
        }
    }
    let mut streaming_enabled: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"streaming_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !streaming_enabled.is_null() && cJSON_IsBool(streaming_enabled) != 0 {
        config.streaming_enabled = cJSON_IsTrue(streaming_enabled) != 0;
        config_changed = r#true != 0;
        has_streaming_enabled = r#true != 0;
    }
    let mut playback_transport: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"playback_transport\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !playback_transport.is_null() {
        if cJSON_IsString(playback_transport) == 0
            || !playback_transport_is_valid((*playback_transport).valuestring)
        {
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"playback_transport is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        if strcmp(
            &raw mut config.playback_transport as *mut ::core::ffi::c_char,
            (*playback_transport).valuestring,
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut config.playback_transport as *mut ::core::ffi::c_char,
                (*playback_transport).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
                0 as size_t,
            );
            config_changed = r#true != 0;
        }
    }
    let mut eptz_config: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"eptz_config\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !eptz_config.is_null() {
        let mut parsed_eptz_config: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut eptz_error: [::core::ffi::c_char; 160] = [0; 160];
        if !copy_valid_eptz_config(
            eptz_config,
            &raw mut parsed_eptz_config as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            &raw mut eptz_error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
        ) {
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                &raw mut eptz_error as *mut ::core::ffi::c_char,
            );
            return;
        }
        if strcmp(
            &raw mut config.eptz_config as *mut ::core::ffi::c_char,
            &raw mut parsed_eptz_config as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut config.eptz_config as *mut ::core::ffi::c_char,
                &raw mut parsed_eptz_config as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                0 as size_t,
            );
            config_changed = r#true != 0;
        }
    }
    let mut priority: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"priority\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !priority.is_null() && cJSON_IsNumber(priority) != 0 {
        if config.priority != (*priority).valueint {
            config.priority = (*priority).valueint;
            config_changed = r#true != 0;
            non_dynamic_config_changed = r#true != 0;
        }
    }
    let mut record: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"record\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !record.is_null() && cJSON_IsBool(record) != 0 {
        config.record = cJSON_IsTrue(record) != 0;
        config_changed = r#true != 0;
        has_record = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"PUT stream JSON parsing: record field parsed as %d (%s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            config.record as ::core::ffi::c_int,
            if config.record as ::core::ffi::c_int != 0 {
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"disabled\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut segment_duration: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"segment_duration\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !segment_duration.is_null() && cJSON_IsNumber(segment_duration) != 0 {
        config.segment_duration = (*segment_duration).valueint;
        config_changed = r#true != 0;
        non_dynamic_config_changed = r#true != 0;
    }
    let mut detection_based_recording_json: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_based_recording\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut detection_based_recording_value: bool = r#false != 0;
    let mut has_detection_based_recording: bool = r#false != 0;
    let mut original_detection_based_recording: bool = config.detection_based_recording;
    if !detection_based_recording_json.is_null()
        && cJSON_IsBool(detection_based_recording_json) != 0
    {
        detection_based_recording_value = cJSON_IsTrue(detection_based_recording_json) != 0;
        has_detection_based_recording = r#true != 0;
        config.detection_based_recording = detection_based_recording_value;
        config_changed = r#true != 0;
        non_dynamic_config_changed = r#true != 0;
    }
    let mut detection_model_json: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_model\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_detection_model: bool = r#false != 0;
    if !detection_model_json.is_null() && cJSON_IsString(detection_model_json) != 0 {
        let mut detection_model_value: [::core::ffi::c_char; 256] = [0; 256];
        safe_strcpy(
            &raw mut detection_model_value as *mut ::core::ffi::c_char,
            (*detection_model_json).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        has_detection_model = r#true != 0;
        safe_strcpy(
            &raw mut config.detection_model as *mut ::core::ffi::c_char,
            &raw mut detection_model_value as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        config_changed = r#true != 0;
        non_dynamic_config_changed = r#true != 0;
    }
    let mut detection_threshold_json: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_threshold\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_detection_threshold: bool = r#false != 0;
    if !detection_threshold_json.is_null() && cJSON_IsNumber(detection_threshold_json) != 0 {
        has_detection_threshold = r#true != 0;
        config.detection_threshold =
            ((*detection_threshold_json).valuedouble / 100.0f64) as ::core::ffi::c_float;
        config_changed = r#true != 0;
        non_dynamic_config_changed = r#true != 0;
    }
    let mut detection_interval_json: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_interval\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_detection_interval: bool = r#false != 0;
    if !detection_interval_json.is_null() && cJSON_IsNumber(detection_interval_json) != 0 {
        has_detection_interval = r#true != 0;
        config.detection_interval = (*detection_interval_json).valueint;
        config_changed = r#true != 0;
        non_dynamic_config_changed = r#true != 0;
    }
    let mut pre_detection_buffer: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"pre_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !pre_detection_buffer.is_null() && cJSON_IsNumber(pre_detection_buffer) != 0 {
        config.pre_detection_buffer = (*pre_detection_buffer).valueint;
        config_changed = r#true != 0;
        non_dynamic_config_changed = r#true != 0;
    }
    let mut post_detection_buffer: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"post_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !post_detection_buffer.is_null() && cJSON_IsNumber(post_detection_buffer) != 0 {
        config.post_detection_buffer = (*post_detection_buffer).valueint;
        config_changed = r#true != 0;
        non_dynamic_config_changed = r#true != 0;
    }
    let mut detection_object_filter: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_object_filter\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_object_filter.is_null() && cJSON_IsString(detection_object_filter) != 0 {
        safe_strcpy(
            &raw mut config.detection_object_filter as *mut ::core::ffi::c_char,
            (*detection_object_filter).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        config_changed = r#true != 0;
    }
    let mut detection_object_filter_list: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_object_filter_list\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_object_filter_list.is_null() && cJSON_IsString(detection_object_filter_list) != 0
    {
        safe_strcpy(
            &raw mut config.detection_object_filter_list as *mut ::core::ffi::c_char,
            (*detection_object_filter_list).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        config_changed = r#true != 0;
    }
    let mut record_audio: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"record_audio\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !record_audio.is_null() && cJSON_IsBool(record_audio) != 0 {
        let mut prev_record_audio: bool = config.record_audio;
        config.record_audio = cJSON_IsTrue(record_audio) != 0;
        if prev_record_audio as ::core::ffi::c_int != config.record_audio as ::core::ffi::c_int {
            config_changed = r#true != 0;
            non_dynamic_config_changed = r#true != 0;
            requires_restart = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Audio recording changed from %s to %s - restart required\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if prev_record_audio as ::core::ffi::c_int != 0 {
                    b"enabled\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"disabled\0".as_ptr() as *const ::core::ffi::c_char
                },
                if config.record_audio as ::core::ffi::c_int != 0 {
                    b"enabled\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"disabled\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
    }
    let mut audio_voice_enhancement: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"audio_voice_enhancement\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !audio_voice_enhancement.is_null() && cJSON_IsBool(audio_voice_enhancement) != 0 {
        let mut prev_avoe: bool = config.audio_voice_enhancement;
        config.audio_voice_enhancement = cJSON_IsTrue(audio_voice_enhancement) != 0;
        if prev_avoe as ::core::ffi::c_int != config.audio_voice_enhancement as ::core::ffi::c_int {
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Audio voice enhancement %s for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if config.audio_voice_enhancement as ::core::ffi::c_int != 0 {
                    b"enabled\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"disabled\0".as_ptr() as *const ::core::ffi::c_char
                },
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut backchannel_enabled: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"backchannel_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !backchannel_enabled.is_null() && cJSON_IsBool(backchannel_enabled) != 0 {
        let mut original_backchannel: bool = config.backchannel_enabled;
        config.backchannel_enabled = cJSON_IsTrue(backchannel_enabled) != 0;
        if original_backchannel as ::core::ffi::c_int
            != config.backchannel_enabled as ::core::ffi::c_int
        {
            config_changed = r#true != 0;
            non_dynamic_config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Backchannel audio changed from %s to %s\0".as_ptr() as *const ::core::ffi::c_char,
                if original_backchannel as ::core::ffi::c_int != 0 {
                    b"enabled\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"disabled\0".as_ptr() as *const ::core::ffi::c_char
                },
                if config.backchannel_enabled as ::core::ffi::c_int != 0 {
                    b"enabled\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"disabled\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
    }
    let mut retention_days: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !retention_days.is_null() && cJSON_IsNumber(retention_days) != 0 {
        let mut new_retention: ::core::ffi::c_int = (*retention_days).valueint;
        if new_retention < -1 as ::core::ffi::c_int || new_retention > 365 as ::core::ffi::c_int {
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"retention_days must be between -1 and 365\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        if config.retention_days != new_retention {
            config.retention_days = new_retention;
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Retention days changed to %d for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                new_retention,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut detection_retention_days: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_retention_days.is_null() && cJSON_IsNumber(detection_retention_days) != 0 {
        let mut new_detection_retention: ::core::ffi::c_int = (*detection_retention_days).valueint;
        if new_detection_retention < -1 as ::core::ffi::c_int
            || new_detection_retention > 365 as ::core::ffi::c_int
        {
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"detection_retention_days must be between -1 and 365\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        if config.detection_retention_days != new_detection_retention {
            config.detection_retention_days = new_detection_retention;
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Detection retention days changed to %d for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                new_detection_retention,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut max_storage_mb: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"max_storage_mb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !max_storage_mb.is_null() && cJSON_IsNumber(max_storage_mb) != 0 {
        let mut new_max_storage: ::core::ffi::c_int = (*max_storage_mb).valueint;
        if config.max_storage_mb != new_max_storage {
            config.max_storage_mb = new_max_storage;
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Max storage MB changed to %d for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                new_max_storage,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut tier_critical_multiplier: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"tier_critical_multiplier\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !tier_critical_multiplier.is_null() && cJSON_IsNumber(tier_critical_multiplier) != 0 {
        let mut new_val: ::core::ffi::c_double = (*tier_critical_multiplier).valuedouble;
        if config.tier_critical_multiplier != new_val {
            config.tier_critical_multiplier = new_val;
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Tier critical multiplier changed to %.2f for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                new_val,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut tier_important_multiplier: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"tier_important_multiplier\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !tier_important_multiplier.is_null() && cJSON_IsNumber(tier_important_multiplier) != 0 {
        let mut new_val_0: ::core::ffi::c_double = (*tier_important_multiplier).valuedouble;
        if config.tier_important_multiplier != new_val_0 {
            config.tier_important_multiplier = new_val_0;
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Tier important multiplier changed to %.2f for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                new_val_0,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut tier_ephemeral_multiplier: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"tier_ephemeral_multiplier\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !tier_ephemeral_multiplier.is_null() && cJSON_IsNumber(tier_ephemeral_multiplier) != 0 {
        let mut new_val_1: ::core::ffi::c_double = (*tier_ephemeral_multiplier).valuedouble;
        if config.tier_ephemeral_multiplier != new_val_1 {
            config.tier_ephemeral_multiplier = new_val_1;
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Tier ephemeral multiplier changed to %.2f for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                new_val_1,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut storage_priority: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"storage_priority\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !storage_priority.is_null() && cJSON_IsNumber(storage_priority) != 0 {
        let mut new_val_2: ::core::ffi::c_int = (*storage_priority).valueint;
        if config.storage_priority != new_val_2 {
            config.storage_priority = new_val_2;
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Storage priority changed to %d for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                new_val_2,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut ptz_enabled: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"ptz_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !ptz_enabled.is_null() && cJSON_IsBool(ptz_enabled) != 0 {
        let mut new_ptz_enabled: bool = cJSON_IsTrue(ptz_enabled) != 0;
        if config.ptz_enabled as ::core::ffi::c_int != new_ptz_enabled as ::core::ffi::c_int {
            config.ptz_enabled = new_ptz_enabled;
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"PTZ %s for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                if config.ptz_enabled as ::core::ffi::c_int != 0 {
                    b"enabled\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"disabled\0".as_ptr() as *const ::core::ffi::c_char
                },
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut ptz_max_x: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"ptz_max_x\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !ptz_max_x.is_null() && cJSON_IsNumber(ptz_max_x) != 0 {
        let mut new_ptz_max_x: ::core::ffi::c_int = (*ptz_max_x).valueint;
        if config.ptz_max_x != new_ptz_max_x {
            config.ptz_max_x = new_ptz_max_x;
            config_changed = r#true != 0;
        }
    }
    let mut ptz_max_y: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"ptz_max_y\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !ptz_max_y.is_null() && cJSON_IsNumber(ptz_max_y) != 0 {
        let mut new_ptz_max_y: ::core::ffi::c_int = (*ptz_max_y).valueint;
        if config.ptz_max_y != new_ptz_max_y {
            config.ptz_max_y = new_ptz_max_y;
            config_changed = r#true != 0;
        }
    }
    let mut ptz_max_z: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"ptz_max_z\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !ptz_max_z.is_null() && cJSON_IsNumber(ptz_max_z) != 0 {
        let mut new_ptz_max_z: ::core::ffi::c_int = (*ptz_max_z).valueint;
        if config.ptz_max_z != new_ptz_max_z {
            config.ptz_max_z = new_ptz_max_z;
            config_changed = r#true != 0;
        }
    }
    let mut ptz_has_home: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"ptz_has_home\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !ptz_has_home.is_null() && cJSON_IsBool(ptz_has_home) != 0 {
        let mut new_ptz_has_home: bool = cJSON_IsTrue(ptz_has_home) != 0;
        if config.ptz_has_home as ::core::ffi::c_int != new_ptz_has_home as ::core::ffi::c_int {
            config.ptz_has_home = new_ptz_has_home;
            config_changed = r#true != 0;
        }
    }
    let mut record_on_schedule: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"record_on_schedule\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !record_on_schedule.is_null() && cJSON_IsBool(record_on_schedule) != 0 {
        let mut new_ros: bool = cJSON_IsTrue(record_on_schedule) != 0;
        if config.record_on_schedule as ::core::ffi::c_int != new_ros as ::core::ffi::c_int {
            config.record_on_schedule = new_ros;
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Recording schedule mode changed to %s for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if new_ros as ::core::ffi::c_int != 0 {
                    b"enabled\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"disabled\0".as_ptr() as *const ::core::ffi::c_char
                },
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut recording_schedule: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"recording_schedule\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !recording_schedule.is_null() && cJSON_IsArray(recording_schedule) != 0 {
        let mut arr_size: ::core::ffi::c_int = cJSON_GetArraySize(recording_schedule);
        if arr_size == 168 as ::core::ffi::c_int {
            let mut schedule_changed: bool = r#false != 0;
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < 168 as ::core::ffi::c_int {
                let mut item: *mut cJSON = cJSON_GetArrayItem(recording_schedule, j);
                let mut new_val_3: uint8_t = (if !item.is_null() && cJSON_IsTrue(item) != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t;
                if config.recording_schedule[j as usize] as ::core::ffi::c_int
                    != new_val_3 as ::core::ffi::c_int
                {
                    config.recording_schedule[j as usize] = new_val_3;
                    schedule_changed = r#true != 0;
                }
                j += 1;
            }
            if schedule_changed {
                config_changed = r#true != 0;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Recording schedule updated for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut config.name as *mut ::core::ffi::c_char,
                );
            }
        }
    }
    let mut detection_record_on_schedule: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_record_on_schedule\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_record_on_schedule.is_null() && cJSON_IsBool(detection_record_on_schedule) != 0 {
        let mut new_value: bool = cJSON_IsTrue(detection_record_on_schedule) != 0;
        if config.detection_record_on_schedule as ::core::ffi::c_int
            != new_value as ::core::ffi::c_int
        {
            config.detection_record_on_schedule = new_value;
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Detection recording schedule mode changed to %s for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if new_value as ::core::ffi::c_int != 0 {
                    b"enabled\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"disabled\0".as_ptr() as *const ::core::ffi::c_char
                },
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut detection_recording_schedule: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_recording_schedule\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_recording_schedule.is_null()
        && cJSON_IsArray(detection_recording_schedule) != 0
        && cJSON_GetArraySize(detection_recording_schedule) == 168 as ::core::ffi::c_int
    {
        let mut changed: bool = r#false != 0;
        let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j_0 < 168 as ::core::ffi::c_int {
            let mut item_0: *mut cJSON = cJSON_GetArrayItem(detection_recording_schedule, j_0);
            let mut new_value_0: uint8_t = (if !item_0.is_null() && cJSON_IsTrue(item_0) != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            if config.detection_recording_schedule[j_0 as usize] as ::core::ffi::c_int
                != new_value_0 as ::core::ffi::c_int
            {
                config.detection_recording_schedule[j_0 as usize] = new_value_0;
                changed = r#true != 0;
            }
            j_0 += 1;
        }
        if changed {
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Detection recording schedule updated for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut protocol: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"protocol\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !protocol.is_null() && cJSON_IsNumber(protocol) != 0 {
        let mut new_protocol: stream_protocol_t =
            stream_protocol_t((*protocol).valueint as ::core::ffi::c_uint);
        if config.protocol.0 != new_protocol.0 {
            config.protocol = new_protocol;
            config_changed = r#true != 0;
            non_dynamic_config_changed = r#true != 0;
            requires_restart = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Protocol changed from %d to %d - restart required\0".as_ptr()
                    as *const ::core::ffi::c_char,
                original_protocol.0,
                config.protocol.0,
            );
        }
    }
    let mut codec_put: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"codec\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !codec_put.is_null() && cJSON_IsString(codec_put) != 0 {
        let mut new_codec: [::core::ffi::c_char; 16] = [0; 16];
        safe_strcpy(
            &raw mut new_codec as *mut ::core::ffi::c_char,
            (*codec_put).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        if strcasecmp(
            &raw mut config.codec as *mut ::core::ffi::c_char,
            &raw mut new_codec as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut config.codec as *mut ::core::ffi::c_char,
                &raw mut new_codec as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
            config_changed = r#true != 0;
            non_dynamic_config_changed = r#true != 0;
            requires_restart = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Codec hint changed to '%s' for stream %s - restart required\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut config.codec as *mut ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut tags_put: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"tags\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !tags_put.is_null() && cJSON_IsString(tags_put) != 0 {
        if strncmp(
            &raw mut config.tags as *mut ::core::ffi::c_char,
            (*tags_put).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(1 as size_t),
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut config.tags as *mut ::core::ffi::c_char,
                (*tags_put).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Tags changed to '%s' for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut config.tags as *mut ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    } else if !tags_put.is_null() && cJSON_IsNull(tags_put) != 0 {
        if config.tags[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            config.tags[0usize] = '\0' as ::core::ffi::c_char;
            config_changed = r#true != 0;
        }
    }
    let mut admin_url_put: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"admin_url\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !admin_url_put.is_null() && cJSON_IsString(admin_url_put) != 0 {
        if strncmp(
            &raw mut config.admin_url as *mut ::core::ffi::c_char,
            (*admin_url_put).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>().wrapping_sub(1 as size_t),
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut config.admin_url as *mut ::core::ffi::c_char,
                (*admin_url_put).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Admin URL changed for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    } else if !admin_url_put.is_null() && cJSON_IsNull(admin_url_put) != 0 {
        if config.admin_url[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            config.admin_url[0usize] = '\0' as ::core::ffi::c_char;
            config_changed = r#true != 0;
        }
    }
    let mut motion_trigger_source_put: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"motion_trigger_source\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !motion_trigger_source_put.is_null() && cJSON_IsString(motion_trigger_source_put) != 0 {
        if strncmp(
            &raw mut config.motion_trigger_source as *mut ::core::ffi::c_char,
            (*motion_trigger_source_put).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>().wrapping_sub(1 as size_t),
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut config.motion_trigger_source as *mut ::core::ffi::c_char,
                (*motion_trigger_source_put).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Motion trigger source changed to '%s' for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut config.motion_trigger_source as *mut ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    } else if !motion_trigger_source_put.is_null() && cJSON_IsNull(motion_trigger_source_put) != 0 {
        if config.motion_trigger_source[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            config.motion_trigger_source[0usize] = '\0' as ::core::ffi::c_char;
            config_changed = r#true != 0;
        }
    }
    let mut go2rtc_source_override_put: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"go2rtc_source_override\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_source_override_put.is_null() && cJSON_IsString(go2rtc_source_override_put) != 0 {
        let mut src: *const ::core::ffi::c_char = (*go2rtc_source_override_put).valuestring;
        if !string_has_non_whitespace(src) {
            src = b"\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if *src.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && !validate_go2rtc_source_override(
                src,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
                res,
            )
        {
            cJSON_Delete(stream_json);
            return;
        }
        if strncmp(
            &raw mut config.go2rtc_source_override as *mut ::core::ffi::c_char,
            src,
            ::core::mem::size_of::<[::core::ffi::c_char; 8192]>().wrapping_sub(1 as size_t),
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut config.go2rtc_source_override as *mut ::core::ffi::c_char,
                src,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
                0 as size_t,
            );
            config_changed = r#true != 0;
            requires_restart = r#true != 0;
            go2rtc_override_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"go2rtc source override changed for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    } else if !go2rtc_source_override_put.is_null() && cJSON_IsNull(go2rtc_source_override_put) != 0
    {
        if config.go2rtc_source_override[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            config.go2rtc_source_override[0usize] = '\0' as ::core::ffi::c_char;
            config_changed = r#true != 0;
            requires_restart = r#true != 0;
            go2rtc_override_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"go2rtc source override cleared for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut sub_stream_url_put: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"sub_stream_url\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !sub_stream_url_put.is_null() && cJSON_IsString(sub_stream_url_put) != 0 {
        if strncmp(
            &raw mut config.sub_stream_url as *mut ::core::ffi::c_char,
            (*sub_stream_url_put).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>().wrapping_sub(1 as size_t),
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut config.sub_stream_url as *mut ::core::ffi::c_char,
                (*sub_stream_url_put).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
            config_changed = r#true != 0;
            requires_restart = r#true != 0;
            sub_stream_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Sub-stream URL changed for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    } else if !sub_stream_url_put.is_null() && cJSON_IsNull(sub_stream_url_put) != 0 {
        if config.sub_stream_url[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            config.sub_stream_url[0usize] = '\0' as ::core::ffi::c_char;
            config_changed = r#true != 0;
            requires_restart = r#true != 0;
            sub_stream_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Sub-stream URL cleared for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut has_detection_url: bool = r#false != 0;
    let mut detection_url_put: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"detection_url\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_url_put.is_null() && cJSON_IsString(detection_url_put) != 0 {
        if !is_allowed_detection_url((*detection_url_put).valuestring) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Rejected detection_url with disallowed scheme for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"detection_url must use http, https, rtsp, or rtsps\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        if strlen((*detection_url_put).valuestring)
            >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"detection_url too long (%zu bytes, max %zu) for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strlen((*detection_url_put).valuestring),
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>().wrapping_sub(1usize),
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                413 as ::core::ffi::c_int,
                b"detection_url exceeds size limit\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        if strncmp(
            &raw mut config.detection_url as *mut ::core::ffi::c_char,
            (*detection_url_put).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>().wrapping_sub(1 as size_t),
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut config.detection_url as *mut ::core::ffi::c_char,
                (*detection_url_put).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
            has_detection_url = r#true != 0;
            config_changed = r#true != 0;
            non_dynamic_config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Detection URL changed for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    } else if !detection_url_put.is_null() && cJSON_IsNull(detection_url_put) != 0 {
        if config.detection_url[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            config.detection_url[0usize] = '\0' as ::core::ffi::c_char;
            has_detection_url = r#true != 0;
            config_changed = r#true != 0;
            non_dynamic_config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Detection URL cleared for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut publish_changed: bool = r#false != 0;
    let mut publish_url_put: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"publish_url\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !publish_url_put.is_null() && cJSON_IsString(publish_url_put) != 0 {
        let mut r#pub: *const ::core::ffi::c_char = (*publish_url_put).valuestring;
        if *r#pub.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && !is_allowed_publish_url(r#pub)
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Rejected invalid publish_url for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"publish_url destinations must be rtmp:// or rtmps:// URLs\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        if strlen(r#pub) >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"publish_url too long (%zu bytes, max %zu) for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strlen(r#pub),
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>().wrapping_sub(1usize),
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                413 as ::core::ffi::c_int,
                b"publish_url exceeds size limit\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        if strncmp(
            &raw mut config.publish_url as *mut ::core::ffi::c_char,
            r#pub,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>().wrapping_sub(1 as size_t),
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut config.publish_url as *mut ::core::ffi::c_char,
                r#pub,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
            publish_changed = r#true != 0;
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Publish (restream) URL changed for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    } else if !publish_url_put.is_null() && cJSON_IsNull(publish_url_put) != 0 {
        if config.publish_url[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            config.publish_url[0usize] = '\0' as ::core::ffi::c_char;
            publish_changed = r#true != 0;
            config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Publish (restream) URL cleared for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut original_is_onvif: bool = config.is_onvif;
    let mut is_onvif: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"isOnvif\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !is_onvif.is_null() && cJSON_IsBool(is_onvif) != 0 {
        config.is_onvif = cJSON_IsTrue(is_onvif) != 0;
    }
    if original_is_onvif as ::core::ffi::c_int != config.is_onvif as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"ONVIF flag changed from %s to %s\0".as_ptr() as *const ::core::ffi::c_char,
            if original_is_onvif as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
            if config.is_onvif as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        config_changed = r#true != 0;
        non_dynamic_config_changed = r#true != 0;
    }
    let mut onvif_port_json: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"onvif_port\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !onvif_port_json.is_null() && cJSON_IsNumber(onvif_port_json) != 0 {
        let mut new_port: ::core::ffi::c_int = (*onvif_port_json).valueint;
        if new_port != config.onvif_port {
            config.onvif_port = new_port;
            config_changed = r#true != 0;
            non_dynamic_config_changed = r#true != 0;
        }
    }
    let mut onvif_test_success: bool = r#true != 0;
    let mut onvif_test_performed: bool = r#false != 0;
    let mut original_onvif_username: [::core::ffi::c_char; 64] = [0; 64];
    let mut original_onvif_password: [::core::ffi::c_char; 64] = [0; 64];
    let mut original_onvif_profile: [::core::ffi::c_char; 64] = [0; 64];
    safe_strcpy(
        &raw mut original_onvif_username as *mut ::core::ffi::c_char,
        &raw mut config.onvif_username as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut original_onvif_password as *mut ::core::ffi::c_char,
        &raw mut config.onvif_password as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut original_onvif_profile as *mut ::core::ffi::c_char,
        &raw mut config.onvif_profile as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    if config.is_onvif {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Testing ONVIF capabilities for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut config.name as *mut ::core::ffi::c_char,
        );
        onvif_test_performed = r#true != 0;
        let mut onvif_username: *mut cJSON = cJSON_GetObjectItem(
            stream_json,
            b"onvif_username\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut onvif_password: *mut cJSON = cJSON_GetObjectItem(
            stream_json,
            b"onvif_password\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut onvif_profile: *mut cJSON = cJSON_GetObjectItem(
            stream_json,
            b"onvif_profile\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !onvif_username.is_null() && cJSON_IsString(onvif_username) != 0 {
            safe_strcpy(
                &raw mut config.onvif_username as *mut ::core::ffi::c_char,
                (*onvif_username).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
        if !onvif_password.is_null() && cJSON_IsString(onvif_password) != 0 {
            safe_strcpy(
                &raw mut config.onvif_password as *mut ::core::ffi::c_char,
                (*onvif_password).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
        if !onvif_profile.is_null() && cJSON_IsString(onvif_profile) != 0 {
            safe_strcpy(
                &raw mut config.onvif_profile as *mut ::core::ffi::c_char,
                (*onvif_profile).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        }
        normalize_stream_url_credentials(&raw mut config);
        if strcmp(
            &raw mut original_onvif_username as *mut ::core::ffi::c_char,
            &raw mut config.onvif_username as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut original_onvif_password as *mut ::core::ffi::c_char,
                &raw mut config.onvif_password as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut original_onvif_profile as *mut ::core::ffi::c_char,
                &raw mut config.onvif_profile as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            credentials_changed = r#true != 0;
            config_changed = r#true != 0;
            requires_restart = r#true != 0;
            non_dynamic_config_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"ONVIF credentials/profile changed for stream %s - restart and go2rtc reload required\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
        let mut onvif_device_url: [::core::ffi::c_char; 512] = [0; 512];
        if build_onvif_test_url(
            &raw mut config,
            &raw mut onvif_device_url as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut onvif_device_url as *mut ::core::ffi::c_char,
                &raw mut config.url as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        }
        let mut result: ::core::ffi::c_int = test_onvif_connection(
            &raw mut onvif_device_url as *mut ::core::ffi::c_char,
            if config.onvif_username[0usize] as ::core::ffi::c_int != 0 {
                &raw mut config.onvif_username as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            if config.onvif_password[0usize] as ::core::ffi::c_int != 0 {
                &raw mut config.onvif_password as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
        );
        onvif_test_success = result == 0 as ::core::ffi::c_int;
        if !onvif_test_success {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"ONVIF test failed for stream %s; keeping user-selected ONVIF flag\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut config.name as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut enable_request: *mut cJSON = cJSON_GetObjectItem(
        stream_json,
        b"enable_disabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !enable_request.is_null()
        && cJSON_IsBool(enable_request) != 0
        && cJSON_IsTrue(enable_request) != 0
    {
        if !enabled.is_null() && cJSON_IsBool(enabled) != 0 && cJSON_IsTrue(enabled) == 0 {
            cJSON_Delete(stream_json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"enable_disabled cannot be combined with enabled=false\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        if !original_enabled {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Enable requested for disabled stream %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut stream_id as *mut ::core::ffi::c_char,
            );
            config.enabled = r#true != 0;
            config_changed = r#true != 0;
            non_dynamic_config_changed = r#true != 0;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Enable requested for stream %s, which is already enabled\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_id as *mut ::core::ffi::c_char,
            );
        }
    }
    cJSON_Delete(stream_json);
    let mut status: stream_status_t = get_stream_status(stream);
    let mut is_running: bool = status.0 == stream_status_t::STREAM_STATUS_RUNNING.0
        || status.0 == stream_status_t::STREAM_STATUS_STARTING.0;
    let mut task: *mut put_stream_task_t =
        calloc(1 as size_t, ::core::mem::size_of::<put_stream_task_t>()) as *mut put_stream_task_t;
    if task.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to allocate memory for PUT stream task\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to allocate memory for task\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    memcpy(
        &raw mut (*task).config as *mut ::core::ffi::c_void,
        &raw mut config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    safe_strcpy(
        &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
        &raw mut stream_id as *mut ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*task).original_url as *mut ::core::ffi::c_char,
        &raw mut original_url as *mut ::core::ffi::c_char,
        MAX_URL_LENGTH as size_t,
        0 as size_t,
    );
    (*task).original_protocol = original_protocol;
    (*task).original_record_audio = original_record_audio;
    (*task).original_backchannel_enabled = original_backchannel_enabled;
    (*task).original_enabled = original_enabled;
    (*task).config_changed = config_changed;
    (*task).requires_restart = requires_restart;
    (*task).is_running = is_running;
    (*task).onvif_test_performed = onvif_test_performed;
    (*task).onvif_test_success = onvif_test_success;
    (*task).original_detection_based_recording = original_detection_based_recording;
    (*task).has_detection_based_recording = has_detection_based_recording;
    (*task).detection_based_recording_value = detection_based_recording_value;
    (*task).has_detection_model = has_detection_model;
    (*task).has_detection_threshold = has_detection_threshold;
    (*task).has_detection_interval = has_detection_interval;
    (*task).has_detection_url = has_detection_url;
    (*task).has_record = has_record;
    (*task).has_streaming_enabled = has_streaming_enabled;
    (*task).non_dynamic_config_changed = non_dynamic_config_changed;
    (*task).credentials_changed = credentials_changed;
    (*task).go2rtc_override_changed = go2rtc_override_changed;
    (*task).sub_stream_changed = sub_stream_changed;
    (*task).publish_changed = publish_changed;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Detection settings before update - Model: %s, Threshold: %.2f, Interval: %d, Pre-buffer: %d, Post-buffer: %d\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw mut config.detection_model as *mut ::core::ffi::c_char,
        config.detection_threshold as ::core::ffi::c_double,
        config.detection_interval,
        config.pre_detection_buffer,
        config.post_detection_buffer,
    );
    let mut response_0: *mut cJSON = cJSON_CreateObject();
    if response_0.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create response JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        put_stream_task_free(task);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        response_0,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response_0,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"Stream update request accepted and processing\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if onvif_test_performed {
        if onvif_test_success {
            cJSON_AddStringToObject(
                response_0,
                b"onvif_status\0".as_ptr() as *const ::core::ffi::c_char,
                b"success\0".as_ptr() as *const ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                response_0,
                b"onvif_message\0".as_ptr() as *const ::core::ffi::c_char,
                b"ONVIF capabilities detected successfully\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            cJSON_AddStringToObject(
                response_0,
                b"onvif_status\0".as_ptr() as *const ::core::ffi::c_char,
                b"error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                response_0,
                b"onvif_message\0".as_ptr() as *const ::core::ffi::c_char,
                b"Failed to detect ONVIF capabilities\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response_0);
    cJSON_Delete(response_0);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert response JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        put_stream_task_free(task);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert response JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut thread_id: pthread_t = 0;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut attr_rc: ::core::ffi::c_int = pthread_attr_init(&raw mut attr);
    if attr_rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to initialize thread attributes for PUT stream (rc=%d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            attr_rc,
        );
        free(json_str as *mut ::core::ffi::c_void);
        put_stream_task_free(task);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to start update task\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    attr_rc = pthread_attr_setdetachstate(
        &raw mut attr,
        C2Rust_Unnamed::PTHREAD_CREATE_DETACHED.0 as ::core::ffi::c_int,
    );
    if attr_rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to set detached thread attribute for PUT stream (rc=%d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            attr_rc,
        );
        pthread_attr_destroy(&raw mut attr);
        free(json_str as *mut ::core::ffi::c_void);
        put_stream_task_free(task);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to start update task\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if pthread_create(
        &raw mut thread_id,
        &raw mut attr,
        Some(
            put_stream_worker
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        task as *mut ::core::ffi::c_void,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create worker thread for PUT stream\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(json_str as *mut ::core::ffi::c_void);
        put_stream_task_free(task);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to start update task\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        http_response_set_json(res, 202 as ::core::ffi::c_int, json_str);
        free(json_str as *mut ::core::ffi::c_void);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"PUT stream task started in worker thread for: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_id as *mut ::core::ffi::c_char,
        );
    }
    pthread_attr_destroy(&raw mut attr);
}
unsafe extern "C" fn delete_stream_worker(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut task: *mut delete_stream_task_t = arg as *mut delete_stream_task_t;
    if task.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid DELETE stream task\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    log_set_thread_context(
        b"StreamAPI\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Processing DELETE /api/streams/%s in worker thread\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
    );
    let mut stream: stream_handle_t =
        get_stream_by_name(&raw mut (*task).stream_id as *mut ::core::ffi::c_char);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Stream %s no longer exists, nothing to delete\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
        );
        free(task as *mut ::core::ffi::c_void);
        return NULL;
    }
    if !stream_task_identity_matches(
        &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
        &raw mut (*task).camera_uuid as *mut ::core::ffi::c_char,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Camera identity changed; dropping queued DELETE for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
        );
        free(task as *mut ::core::ffi::c_void);
        return NULL;
    }
    let mut status: stream_status_t = get_stream_status(stream);
    if status.0 == stream_status_t::STREAM_STATUS_RUNNING.0
        || status.0 == stream_status_t::STREAM_STATUS_STARTING.0
    {
        if stop_stream(stream) != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to stop stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
            );
        }
        let mut timeout: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
        while get_stream_status(stream).0 != stream_status_t::STREAM_STATUS_STOPPED.0
            && timeout > 0 as ::core::ffi::c_int
        {
            usleep(100000 as ::core::ffi::c_int as __useconds_t);
            timeout -= 1;
        }
    }
    if is_unified_detection_running(&raw mut (*task).stream_id as *mut ::core::ffi::c_char) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Stopping unified detection thread for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
        );
        if stop_unified_detection_thread(&raw mut (*task).stream_id as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to stop unified detection thread for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Successfully stopped unified detection thread for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
            );
        }
    }
    if remove_stream(stream) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to delete stream from memory: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
        );
        free(task as *mut ::core::ffi::c_void);
        return NULL;
    }
    if delete_stream_config_internal(
        &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
        (*task).permanent_delete,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to %s stream configuration in database for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            if (*task).permanent_delete as ::core::ffi::c_int != 0 {
                b"permanently delete\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"disable\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
        );
        free(task as *mut ::core::ffi::c_void);
        return NULL;
    }
    if !go2rtc_integration_unregister_stream(&raw mut (*task).stream_id as *mut ::core::ffi::c_char)
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to unregister stream %s from go2rtc\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully %s stream in worker thread: %s\0".as_ptr() as *const ::core::ffi::c_char,
        if (*task).permanent_delete as ::core::ffi::c_int != 0 {
            b"permanently deleted\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char
        },
        &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
    );
    free(task as *mut ::core::ffi::c_void);
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_stream(
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
        b"Handling DELETE /api/streams/%s request\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_id as *mut ::core::ffi::c_char,
    );
    let mut permanent_delete: bool = r#false != 0;
    let mut permanent_param: [::core::ffi::c_char; 16] = [
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
    ];
    if http_request_get_query_param(
        req,
        b"permanent\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut permanent_param as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
    ) > 0 as ::core::ffi::c_int
    {
        if strcmp(
            &raw mut permanent_param as *mut ::core::ffi::c_char,
            b"true\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            permanent_delete = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Permanent delete requested for stream: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_id as *mut ::core::ffi::c_char,
            );
        }
    }
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
    };
    if get_stream_config(stream, &raw mut config) != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get stream configuration\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !check_stream_write_access(req, res, &raw mut config) {
        return;
    }
    let mut task: *mut delete_stream_task_t =
        calloc(1 as size_t, ::core::mem::size_of::<delete_stream_task_t>())
            as *mut delete_stream_task_t;
    if task.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to allocate delete stream task\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to allocate delete task\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_strcpy(
        &raw mut (*task).stream_id as *mut ::core::ffi::c_char,
        &raw mut stream_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*task).camera_uuid as *mut ::core::ffi::c_char,
        &raw mut config.camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    (*task).permanent_delete = permanent_delete;
    let mut response_json: *mut cJSON = cJSON_CreateObject();
    if response_json.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create response JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(task as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        response_json,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddBoolToObject(
        response_json,
        b"permanent\0".as_ptr() as *const ::core::ffi::c_char,
        permanent_delete as cJSON_bool,
    );
    cJSON_AddStringToObject(
        response_json,
        b"status\0".as_ptr() as *const ::core::ffi::c_char,
        b"accepted\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response_json);
    cJSON_Delete(response_json);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert response JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(task as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert response JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut thread_id: pthread_t = 0;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut attr_rc: ::core::ffi::c_int = pthread_attr_init(&raw mut attr);
    if attr_rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to initialize pthread attributes for DELETE stream worker (rc=%d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            attr_rc,
        );
        free(task as *mut ::core::ffi::c_void);
        free(json_str as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to start delete task\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    attr_rc = pthread_attr_setdetachstate(
        &raw mut attr,
        C2Rust_Unnamed::PTHREAD_CREATE_DETACHED.0 as ::core::ffi::c_int,
    );
    if attr_rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to set detached state for DELETE stream worker (rc=%d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            attr_rc,
        );
        pthread_attr_destroy(&raw mut attr);
        free(task as *mut ::core::ffi::c_void);
        free(json_str as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to start delete task\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if pthread_create(
        &raw mut thread_id,
        &raw mut attr,
        Some(
            delete_stream_worker
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        task as *mut ::core::ffi::c_void,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create worker thread for DELETE stream\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(task as *mut ::core::ffi::c_void);
        free(json_str as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to start delete task\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        http_response_set_json(res, 202 as ::core::ffi::c_int, json_str);
        free(json_str as *mut ::core::ffi::c_void);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"DELETE stream task started in worker thread for: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_id as *mut ::core::ffi::c_char,
        );
    }
    pthread_attr_destroy(&raw mut attr);
}
pub const STREAM_REFRESH_COOLDOWN_SECONDS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
static mut refresh_stream_guards: [refresh_stream_guard_t; 1024] = [refresh_stream_guard_t {
    used: false,
    in_progress: false,
    stream_name: [0; 256],
    last_started: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
}; 1024];
static mut refresh_stream_guards_mutex: pthread_mutex_t = pthread_mutex_t {
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
unsafe extern "C" fn reserve_stream_refresh(
    mut stream_name: *const ::core::ffi::c_char,
    mut retry_after_seconds: *mut ::core::ffi::c_int,
) -> bool {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_MONOTONIC, &raw mut now) != 0 as ::core::ffi::c_int {
        now.tv_sec = time(::core::ptr::null_mut::<time_t>()) as __time_t;
        now.tv_nsec = 0 as __syscall_slong_t;
    }
    if !retry_after_seconds.is_null() {
        *retry_after_seconds = 0 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut refresh_stream_guards_mutex);
    let mut free_slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut reusable_slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut oldest_start: time_t = now.tv_sec;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_STREAMS {
        let mut guard: *mut refresh_stream_guard_t =
            (&raw mut refresh_stream_guards as *mut refresh_stream_guard_t).offset(i as isize);
        if !(*guard).used {
            if free_slot < 0 as ::core::ffi::c_int {
                free_slot = i;
            }
        } else {
            if strcmp(
                &raw mut (*guard).stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
            {
                let mut elapsed: time_t = now.tv_sec - (*guard).last_started.tv_sec;
                if elapsed < 0 as time_t {
                    (*guard).last_started = now;
                    elapsed = 0 as time_t;
                }
                if (*guard).in_progress as ::core::ffi::c_int != 0
                    || elapsed < STREAM_REFRESH_COOLDOWN_SECONDS as time_t
                {
                    if !retry_after_seconds.is_null() {
                        *retry_after_seconds = if (*guard).in_progress as ::core::ffi::c_int != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            STREAM_REFRESH_COOLDOWN_SECONDS - elapsed as ::core::ffi::c_int
                        };
                    }
                    pthread_mutex_unlock(&raw mut refresh_stream_guards_mutex);
                    return r#false != 0;
                }
                (*guard).in_progress = r#true != 0;
                (*guard).last_started = now;
                pthread_mutex_unlock(&raw mut refresh_stream_guards_mutex);
                return r#true != 0;
            }
            if !(*guard).in_progress && (*guard).last_started.tv_sec <= oldest_start {
                oldest_start = (*guard).last_started.tv_sec as time_t;
                reusable_slot = i;
            }
        }
        i += 1;
    }
    let mut slot: ::core::ffi::c_int = if free_slot >= 0 as ::core::ffi::c_int {
        free_slot
    } else {
        reusable_slot
    };
    if slot < 0 as ::core::ffi::c_int {
        if !retry_after_seconds.is_null() {
            *retry_after_seconds = 1 as ::core::ffi::c_int;
        }
        pthread_mutex_unlock(&raw mut refresh_stream_guards_mutex);
        return r#false != 0;
    }
    let mut guard_0: *mut refresh_stream_guard_t =
        (&raw mut refresh_stream_guards as *mut refresh_stream_guard_t).offset(slot as isize);
    memset(
        guard_0 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<refresh_stream_guard_t>(),
    );
    (*guard_0).used = r#true != 0;
    (*guard_0).in_progress = r#true != 0;
    (*guard_0).last_started = now;
    safe_strcpy(
        &raw mut (*guard_0).stream_name as *mut ::core::ffi::c_char,
        stream_name,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    pthread_mutex_unlock(&raw mut refresh_stream_guards_mutex);
    return r#true != 0;
}
unsafe extern "C" fn finish_stream_refresh(
    mut stream_name: *const ::core::ffi::c_char,
    mut discard: bool,
) {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_MONOTONIC, &raw mut now) != 0 as ::core::ffi::c_int {
        now.tv_sec = time(::core::ptr::null_mut::<time_t>()) as __time_t;
        now.tv_nsec = 0 as __syscall_slong_t;
    }
    pthread_mutex_lock(&raw mut refresh_stream_guards_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_STREAMS {
        let mut guard: *mut refresh_stream_guard_t =
            (&raw mut refresh_stream_guards as *mut refresh_stream_guard_t).offset(i as isize);
        if (*guard).used as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*guard).stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            if discard {
                memset(
                    guard as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<refresh_stream_guard_t>(),
                );
            } else {
                (*guard).in_progress = r#false != 0;
                (*guard).last_started = now;
            }
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut refresh_stream_guards_mutex);
}
unsafe extern "C" fn refresh_stream_worker(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut task: *mut refresh_stream_task_t = arg as *mut refresh_stream_task_t;
    if task.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid refresh stream task\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    log_set_thread_context(
        b"StreamAPI\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Processing stream refresh for %s in worker thread\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
    );
    if !stream_task_identity_matches(
        &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
        &raw mut (*task).camera_uuid as *mut ::core::ffi::c_char,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Camera identity changed; dropping queued refresh for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
        );
        finish_stream_refresh(
            &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
            r#true != 0,
        );
        free(task as *mut ::core::ffi::c_void);
        return NULL;
    }
    if !go2rtc_integration_is_initialized() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc integration not initialized, attempting full start\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        if !go2rtc_integration_full_start() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to start go2rtc integration for stream refresh: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
            );
            finish_stream_refresh(
                &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
                r#false != 0,
            );
            free(task as *mut ::core::ffi::c_void);
            return NULL;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc integration started successfully\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut lifecycle_guard: go2rtc_lifecycle_guard_t = go2rtc_lifecycle_guard_t {
        acquired: false,
        outermost: false,
        coalesced: false,
        result: false,
        generation: 0,
        restart_generation: 0,
    };
    let mut lifecycle_entered: bool = go2rtc_lifecycle_begin(
        go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_RECONFIGURE,
        r#false != 0,
        r#true != 0,
        &raw mut lifecycle_guard,
    );
    let mut success: bool = r#false != 0;
    if !lifecycle_entered {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to enter go2rtc lifecycle for stream refresh: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
        );
    } else if !stream_task_identity_matches(
        &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
        &raw mut (*task).camera_uuid as *mut ::core::ffi::c_char,
    ) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Camera identity changed while refresh waited for lifecycle: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
        );
    } else if lifecycle_guard.restart_generation != (*task).restart_generation {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Coalesced refresh for %s with completed global go2rtc restart\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
        );
        success = r#true != 0;
    } else {
        success = go2rtc_integration_reload_stream(
            &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
        );
    }
    if lifecycle_entered {
        go2rtc_lifecycle_end(&raw mut lifecycle_guard, success);
    }
    if success {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Successfully refreshed go2rtc registration for stream: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
        );
        signal_mp4_recording_reconnect(&raw mut (*task).stream_name as *mut ::core::ffi::c_char);
        let mut stream: stream_handle_t =
            get_stream_by_name(&raw mut (*task).stream_name as *mut ::core::ffi::c_char);
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
        };
        if !stream.is_null()
            && get_stream_config(stream, &raw mut config) == 0 as ::core::ffi::c_int
        {
            if config.detection_based_recording as ::core::ffi::c_int != 0
                && is_detection_recording_scheduled(&raw mut config) as ::core::ffi::c_int != 0
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Restarting unified detection thread for stream: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
                );
                if is_unified_detection_running(
                    &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
                ) {
                    stop_unified_detection_thread(
                        &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
                    );
                    usleep(500000 as ::core::ffi::c_int as __useconds_t);
                }
                let mut annotation_only: bool = config.record;
                if start_unified_detection_thread(
                    &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
                    &raw mut config.detection_model as *mut ::core::ffi::c_char,
                    config.detection_threshold,
                    config.pre_detection_buffer,
                    config.post_detection_buffer,
                    annotation_only,
                ) != 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Failed to restart unified detection thread for stream: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
                    );
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Successfully restarted unified detection thread for stream: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
                    );
                }
            }
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to refresh go2rtc registration for stream: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
        );
    }
    finish_stream_refresh(
        &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
        r#false != 0,
    );
    free(task as *mut ::core::ffi::c_void);
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_stream_refresh(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/streams/:name/refresh request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut stream_name: [::core::ffi::c_char; 256] = [
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
    if http_request_extract_path_param(
        req,
        b"/api/streams/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to extract stream name from URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name in URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut suffix: *mut ::core::ffi::c_char = strstr(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        b"/refresh\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !suffix.is_null() {
        *suffix = '\0' as ::core::ffi::c_char;
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
    let mut authorized_camera: fleet_camera_t = fleet_camera_t {
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
    if httpd_authorize_stream_action_with_context(
        req,
        res,
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut user,
        &raw mut authorized_camera,
        &raw mut evaluation,
    ) == 0
    {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Refreshing go2rtc registration for stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut stream: stream_handle_t =
        get_stream_by_name(&raw mut stream_name as *mut ::core::ffi::c_char);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Stream not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut retry_after_seconds: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !reserve_stream_refresh(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut retry_after_seconds,
    ) {
        let mut response: *mut cJSON = cJSON_CreateObject();
        if response.is_null() {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create response JSON\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddBoolToObject(
            response,
            b"success\0".as_ptr() as *const ::core::ffi::c_char,
            r#true,
        );
        cJSON_AddBoolToObject(
            response,
            b"coalesced\0".as_ptr() as *const ::core::ffi::c_char,
            r#true,
        );
        cJSON_AddStringToObject(
            response,
            b"message\0".as_ptr() as *const ::core::ffi::c_char,
            b"A recent stream refresh is already active or cooling down\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            response,
            b"stream\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            response,
            b"retry_after_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            retry_after_seconds as ::core::ffi::c_double,
        );
        let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
        cJSON_Delete(response);
        if json_str.is_null() {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to convert response JSON\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        http_response_set_json(res, 202 as ::core::ffi::c_int, json_str);
        free(json_str as *mut ::core::ffi::c_void);
        return;
    }
    let mut task: *mut refresh_stream_task_t =
        calloc(1 as size_t, ::core::mem::size_of::<refresh_stream_task_t>())
            as *mut refresh_stream_task_t;
    if task.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to allocate refresh stream task\0".as_ptr() as *const ::core::ffi::c_char,
        );
        finish_stream_refresh(
            &raw mut stream_name as *mut ::core::ffi::c_char,
            r#true != 0,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Internal server error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_strcpy(
        &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*task).camera_uuid as *mut ::core::ffi::c_char,
        &raw mut authorized_camera.camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    (*task).restart_generation = go2rtc_lifecycle_restart_generation();
    let mut response_0: *mut cJSON = cJSON_CreateObject();
    if response_0.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create response JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(task as *mut ::core::ffi::c_void);
        finish_stream_refresh(
            &raw mut stream_name as *mut ::core::ffi::c_char,
            r#true != 0,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        response_0,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response_0,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"Stream refresh request accepted and processing\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        response_0,
        b"stream\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut json_str_0: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response_0);
    cJSON_Delete(response_0);
    if json_str_0.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert response JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(task as *mut ::core::ffi::c_void);
        finish_stream_refresh(
            &raw mut stream_name as *mut ::core::ffi::c_char,
            r#true != 0,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert response JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut thread_id: pthread_t = 0;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut attr_rc: ::core::ffi::c_int = pthread_attr_init(&raw mut attr);
    if attr_rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to initialize pthread attributes for stream refresh\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(task as *mut ::core::ffi::c_void);
        finish_stream_refresh(
            &raw mut stream_name as *mut ::core::ffi::c_char,
            r#true != 0,
        );
        free(json_str_0 as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to start stream refresh\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    attr_rc = pthread_attr_setdetachstate(
        &raw mut attr,
        C2Rust_Unnamed::PTHREAD_CREATE_DETACHED.0 as ::core::ffi::c_int,
    );
    if attr_rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to configure detached pthread attributes for stream refresh\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(task as *mut ::core::ffi::c_void);
        finish_stream_refresh(
            &raw mut stream_name as *mut ::core::ffi::c_char,
            r#true != 0,
        );
        free(json_str_0 as *mut ::core::ffi::c_void);
        pthread_attr_destroy(&raw mut attr);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to start stream refresh\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if pthread_create(
        &raw mut thread_id,
        &raw mut attr,
        Some(
            refresh_stream_worker
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        task as *mut ::core::ffi::c_void,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create worker thread for stream refresh\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(task as *mut ::core::ffi::c_void);
        finish_stream_refresh(
            &raw mut stream_name as *mut ::core::ffi::c_char,
            r#true != 0,
        );
        free(json_str_0 as *mut ::core::ffi::c_void);
        pthread_attr_destroy(&raw mut attr);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to start stream refresh\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Stream refresh task started in worker thread for: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    pthread_attr_destroy(&raw mut attr);
    http_response_set_json(res, 202 as ::core::ffi::c_int, json_str_0);
    free(json_str_0 as *mut ::core::ffi::c_void);
}
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
