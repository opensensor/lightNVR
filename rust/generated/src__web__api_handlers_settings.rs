unsafe extern "C" {
    pub type stream_handle_s;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
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
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
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
    unsafe fn system_health_policy_build(
        settings: *const system_health_policy_settings_t,
        override_json: *const ::core::ffi::c_char,
        policy: *mut system_health_policy_t,
        canonical_json: *mut ::core::ffi::c_char,
        canonical_json_size: size_t,
        error: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn system_health_policy_replace(
        candidate: *const system_health_policy_t,
        error: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn system_health_policy_serialize(
        policy: *const system_health_policy_t,
        json: *mut ::core::ffi::c_char,
        json_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn playback_transport_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn save_config(
        config: *const config_t,
        path: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn config_get_mp4_faststart(config: *const config_t) -> bool;
    unsafe fn config_set_mp4_faststart(config: *mut config_t, enabled: bool);
    unsafe fn config_set_detection_grace_period(config: *mut config_t, seconds: ::core::ffi::c_int);
    unsafe fn config_set_detection_engine_threads(
        config: *mut config_t,
        threads: ::core::ffi::c_int,
    );
    unsafe fn config_set_detection_engine_delegate(
        config: *mut config_t,
        delegate: *const ::core::ffi::c_char,
    ) -> bool;
    unsafe fn get_loaded_config_path() -> *const ::core::ffi::c_char;
    static mut g_config: config_t;
    unsafe fn api_detection_format_parse(
        name: *const ::core::ffi::c_char,
        out: *mut api_detection_format_t,
    ) -> bool;
    unsafe fn config_set_api_detection_format(
        config: *mut config_t,
        name: *const ::core::ffi::c_char,
    ) -> bool;
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
    unsafe fn db_auth_validate_allowed_login_cidrs(
        allowed_login_cidrs: *const ::core::ffi::c_char,
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
    unsafe fn set_log_level(level: log_level_t);
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
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn init_database(db_path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn shutdown_database();
    unsafe fn get_all_stream_configs(
        streams: *mut stream_config_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_get_system_setting(
        key: *const ::core::ffi::c_char,
        out: *mut ::core::ffi::c_char,
        out_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_get_system_setting_alloc(
        key: *const ::core::ffi::c_char,
        out: *mut *mut ::core::ffi::c_char,
        out_len: *mut size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_set_system_setting(
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn init_stream_manager(max_streams: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn shutdown_stream_manager();
    unsafe fn add_stream(config: *const stream_config_t) -> stream_handle_t;
    unsafe fn start_stream(handle: stream_handle_t) -> ::core::ffi::c_int;
    unsafe fn stop_stream(handle: stream_handle_t) -> ::core::ffi::c_int;
    unsafe fn get_stream_config(
        handle: stream_handle_t,
        config: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_by_name(name: *const ::core::ffi::c_char) -> stream_handle_t;
    unsafe fn start_hls_stream(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn stop_hls_stream(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn start_mp4_recording(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn stop_mp4_recording(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn stream_start_hls(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn stream_stop_hls(stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn mp4_directory_format_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn go2rtc_parse_external_candidates(
        list: *const ::core::ffi::c_char,
        port: ::core::ffi::c_int,
        out: *mut [::core::ffi::c_char; 128],
        max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn go2rtc_process_stop() -> bool;
    unsafe fn go2rtc_process_is_running() -> bool;
    unsafe fn go2rtc_process_clear_override_quarantine();
    unsafe fn go2rtc_process_try_get_pid(pid: *mut ::core::ffi::c_int) -> bool;
    unsafe fn go2rtc_api_get_base_path() -> *const ::core::ffi::c_char;
    unsafe fn go2rtc_stream_cleanup();
    unsafe fn go2rtc_integration_start_recording(
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn go2rtc_integration_restart_process() -> bool;
    unsafe fn go2rtc_integration_cleanup();
    unsafe fn go2rtc_lifecycle_begin(
        operation: go2rtc_lifecycle_operation_t,
        coalesce: bool,
        intentional: bool,
        guard: *mut go2rtc_lifecycle_guard_t,
    ) -> bool;
    unsafe fn go2rtc_lifecycle_end(guard: *mut go2rtc_lifecycle_guard_t, result: bool);
    unsafe fn mqtt_reinit(config: *const config_t) -> ::core::ffi::c_int;
    unsafe fn mqtt_delivery_worker_start() -> ::core::ffi::c_int;
    unsafe fn mqtt_delivery_worker_shutdown();
    unsafe fn set_retention_days(days: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn audit_log_decision_modes_init() -> ::core::ffi::c_int;
    unsafe fn audit_log_flush_summaries(closed_windows_only: bool) -> size_t;
    unsafe fn yaml_validate_go2rtc_override(
        src: *const ::core::ffi::c_char,
        len: size_t,
        out: *mut yaml_validation_result_t,
    );
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __useconds_t = ::core::ffi::c_uint;
pub type int64_t = i64;
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const PTHREAD_CREATE_JOINABLE: Self = Self(0);
    pub const PTHREAD_CREATE_DETACHED: Self = Self(1);
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
pub struct system_health_unit_t(pub ::core::ffi::c_uint);
impl system_health_unit_t {
    pub const SYSTEM_HEALTH_UNIT_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_UNIT_RATIO: Self = Self(1);
    pub const SYSTEM_HEALTH_UNIT_BYTES: Self = Self(2);
    pub const SYSTEM_HEALTH_UNIT_COUNT: Self = Self(3);
    pub const SYSTEM_HEALTH_UNIT_SECONDS: Self = Self(4);
    pub const SYSTEM_HEALTH_UNIT_CELSIUS: Self = Self(5);
    pub const SYSTEM_HEALTH_UNIT_HERTZ: Self = Self(6);
    pub const SYSTEM_HEALTH_UNIT_BOOLEAN: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_condition_t(pub ::core::ffi::c_uint);
impl system_health_condition_t {
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW: Self = Self(0);
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_OOM_KILL: Self = Self(1);
    pub const SYSTEM_HEALTH_CONDITION_MEMORY_SWAP_THRASH: Self = Self(2);
    pub const SYSTEM_HEALTH_CONDITION_CPU_SATURATION: Self = Self(3);
    pub const SYSTEM_HEALTH_CONDITION_CPU_THROTTLED: Self = Self(4);
    pub const SYSTEM_HEALTH_CONDITION_IO_PRESSURE: Self = Self(5);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_BYTES_LOW: Self = Self(6);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_INODES_LOW: Self = Self(7);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY: Self = Self(8);
    pub const SYSTEM_HEALTH_CONDITION_FILESYSTEM_WRITE_FAILED: Self = Self(9);
    pub const SYSTEM_HEALTH_CONDITION_THERMAL_HIGH: Self = Self(10);
    pub const SYSTEM_HEALTH_CONDITION_NETWORK_LINK_DOWN: Self = Self(11);
    pub const SYSTEM_HEALTH_CONDITION_NETWORK_ERROR_RATE: Self = Self(12);
    pub const SYSTEM_HEALTH_CONDITION_CLOCK_UNSYNCHRONIZED: Self = Self(13);
    pub const SYSTEM_HEALTH_CONDITION_CLOCK_JUMP: Self = Self(14);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_FD_EXHAUSTION: Self = Self(15);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_PID_EXHAUSTION: Self = Self(16);
    pub const SYSTEM_HEALTH_CONDITION_PROCESS_ALLOCATION_FAILED: Self = Self(17);
    pub const SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_PREFAIL: Self = Self(18);
    pub const SYSTEM_HEALTH_CONDITION_STORAGE_DEVICE_CRITICAL: Self = Self(19);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_CORRECTED: Self = Self(20);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_ECC_UNCORRECTABLE: Self = Self(21);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_FAN_FAILED: Self = Self(22);
    pub const SYSTEM_HEALTH_CONDITION_HARDWARE_POWER_UNSTABLE: Self = Self(23);
    pub const SYSTEM_HEALTH_CONDITION_COLLECTOR_STALE: Self = Self(24);
    pub const SYSTEM_HEALTH_CONDITION_UNEXPECTED_RESTART: Self = Self(25);
    pub const SYSTEM_HEALTH_CONDITION_EVENT_DELIVERY_DEGRADED: Self = Self(26);
    pub const SYSTEM_HEALTH_CONDITION_COUNT: Self = Self(27);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_profile_t(pub ::core::ffi::c_uint);
impl system_health_profile_t {
    pub const SYSTEM_HEALTH_PROFILE_BALANCED: Self = Self(0);
    pub const SYSTEM_HEALTH_PROFILE_CONSERVATIVE: Self = Self(1);
    pub const SYSTEM_HEALTH_PROFILE_DISABLED: Self = Self(2);
    pub const SYSTEM_HEALTH_PROFILE_CUSTOM: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_threshold_direction_t(pub ::core::ffi::c_uint);
impl system_health_threshold_direction_t {
    pub const SYSTEM_HEALTH_THRESHOLD_NONE: Self = Self(0);
    pub const SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE: Self = Self(1);
    pub const SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE: Self = Self(2);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_condition_policy_t {
    pub condition: system_health_condition_t,
    pub profile: system_health_profile_t,
    pub enabled: bool,
    pub overridden: bool,
    pub direction: system_health_threshold_direction_t,
    pub unit: system_health_unit_t,
    pub warning_threshold: ::core::ffi::c_double,
    pub critical_threshold: ::core::ffi::c_double,
    pub recovery_threshold: ::core::ffi::c_double,
    pub warning_for_seconds: uint32_t,
    pub critical_for_seconds: uint32_t,
    pub recovery_for_seconds: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_policy_t {
    pub generation: uint64_t,
    pub settings: system_health_policy_settings_t,
    pub conditions: [system_health_condition_policy_t; 27],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct api_detection_format_t(pub ::core::ffi::c_uint);
impl api_detection_format_t {
    pub const API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT: Self = Self(0);
    pub const API_DETECTION_FORMAT_DOODS2: Self = Self(1);
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
pub struct mqtt_settings_task_t {
    pub mqtt_now_enabled: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct go2rtc_settings_task_t {
    pub becoming_enabled: bool,
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
pub type stream_handle_t = *mut stream_handle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_validation_result_t {
    pub valid: ::core::ffi::c_int,
    pub err_message: [::core::ffi::c_char; 256],
    pub err_line: ::core::ffi::c_int,
    pub err_column: ::core::ffi::c_int,
    pub duplicate_keys: ::core::ffi::c_int,
    pub non_mapping_root: ::core::ffi::c_int,
    pub parse_error: ::core::ffi::c_int,
    pub warnings: [[::core::ffi::c_char; 192]; 16],
    pub warning_count: ::core::ffi::c_int,
}
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const LOG_USER: ::core::ffi::c_int = 8;
pub const LOG_DAEMON: ::core::ffi::c_int = 24;
pub const LOG_LOCAL0: ::core::ffi::c_int = 128;
pub const LOG_LOCAL1: ::core::ffi::c_int = 136;
pub const LOG_LOCAL2: ::core::ffi::c_int = 144;
pub const LOG_LOCAL3: ::core::ffi::c_int = 152;
pub const LOG_LOCAL4: ::core::ffi::c_int = 160;
pub const LOG_LOCAL5: ::core::ffi::c_int = 168;
pub const LOG_LOCAL6: ::core::ffi::c_int = 176;
pub const LOG_LOCAL7: ::core::ffi::c_int = 184;
pub const SYSTEM_HEALTH_POLICY_ERROR_LENGTH: ::core::ffi::c_uint = 256 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_OVERRIDE_JSON_MAX: ::core::ffi::c_uint = 16384 as ::core::ffi::c_uint;
pub const SYSTEM_HEALTH_OVERRIDES_SETTING_KEY: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(*b"health_condition_overrides\0")
};
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const DB_BACKUP_VERIFY_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DB_BACKUP_VERIFY_QUICK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DB_BACKUP_VERIFY_FULL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LOG_COMPONENT: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"SettingsAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const GO2RTC_MAX_EXTERNAL_CANDIDATES: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn is_safe_storage_path(mut path: *const ::core::ffi::c_char) -> bool {
    if path.is_null() || *path.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int {
        return r#false != 0;
    }
    static mut dangerous: [::core::ffi::c_char; 23] = unsafe {
        ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(
            *b";|&`$><\n\r\\\"'!{}()[]~*?\0",
        )
    };
    let mut c: *const ::core::ffi::c_char = path;
    while *c as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        if !strchr(
            &raw const dangerous as *const ::core::ffi::c_char,
            *c as ::core::ffi::c_int,
        )
        .is_null()
        {
            return r#false != 0;
        }
        c = c.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn health_setting_key_allowed(mut name: *const ::core::ffi::c_char) -> bool {
    static mut keys: [*const ::core::ffi::c_char; 11] = [
        b"health_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        b"health_profile\0".as_ptr() as *const ::core::ffi::c_char,
        b"health_fast_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        b"health_normal_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        b"health_slow_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        b"health_device_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        b"health_write_probe_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        b"health_hardware_provider\0".as_ptr() as *const ::core::ffi::c_char,
        b"health_presence_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        b"health_incident_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        b"health_condition_overrides\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut index: size_t = 0 as size_t;
    while index
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 11]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        if strcmp(name, keys[index]) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        index = index.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn health_json_u32(
    mut settings: *const cJSON,
    mut name: *const ::core::ffi::c_char,
    mut destination: *mut uint32_t,
    mut touched: *mut bool,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(settings, name);
    if item.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    *touched = r#true != 0;
    if cJSON_IsNumber(item) == 0
        || (*item).valuedouble < 0.0f64
        || (*item).valuedouble > UINT32_MAX as ::core::ffi::c_double
    {
        snprintf(
            error,
            SYSTEM_HEALTH_POLICY_ERROR_LENGTH as size_t,
            b"%s must be a non-negative whole number\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut parsed: uint32_t = (*item).valuedouble as uint32_t;
    if parsed as ::core::ffi::c_double != (*item).valuedouble {
        snprintf(
            error,
            SYSTEM_HEALTH_POLICY_ERROR_LENGTH as size_t,
            b"%s must be a non-negative whole number\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
        return -1 as ::core::ffi::c_int;
    }
    *destination = parsed;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn health_json_bool(
    mut settings: *const cJSON,
    mut name: *const ::core::ffi::c_char,
    mut destination: *mut bool,
    mut touched: *mut bool,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(settings, name);
    if item.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    *touched = r#true != 0;
    if cJSON_IsBool(item) == 0 {
        snprintf(
            error,
            SYSTEM_HEALTH_POLICY_ERROR_LENGTH as size_t,
            b"%s must be boolean\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
        return -1 as ::core::ffi::c_int;
    }
    *destination = cJSON_IsTrue(item) != 0;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn health_json_string(
    mut settings: *const cJSON,
    mut name: *const ::core::ffi::c_char,
    mut destination: *mut ::core::ffi::c_char,
    mut destination_size: size_t,
    mut touched: *mut bool,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(settings, name);
    if item.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    *touched = r#true != 0;
    if cJSON_IsString(item) == 0
        || (*item).valuestring.is_null()
        || strlen((*item).valuestring) >= destination_size
    {
        snprintf(
            error,
            SYSTEM_HEALTH_POLICY_ERROR_LENGTH as size_t,
            b"%s must be a bounded string\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
        return -1 as ::core::ffi::c_int;
    }
    safe_strcpy(
        destination,
        (*item).valuestring,
        destination_size,
        0 as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn prepare_health_policy_update(
    mut settings: *const cJSON,
    mut candidate: *mut system_health_policy_t,
    mut canonical_out: *mut *mut ::core::ffi::c_char,
    mut persist_overrides: *mut bool,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut touched: bool = r#false != 0;
    *canonical_out = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *persist_overrides = r#false != 0;
    *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut item: *const cJSON = (*settings).child;
    while !item.is_null() {
        if !(*item).string.is_null()
            && strncmp(
                (*item).string,
                b"health_\0".as_ptr() as *const ::core::ffi::c_char,
                7 as size_t,
            ) == 0 as ::core::ffi::c_int
            && !health_setting_key_allowed((*item).string)
        {
            snprintf(
                error,
                SYSTEM_HEALTH_POLICY_ERROR_LENGTH as size_t,
                b"unknown health setting: %.96s\0".as_ptr() as *const ::core::ffi::c_char,
                (*item).string,
            );
            return -1 as ::core::ffi::c_int;
        }
        item = (*item).next;
    }
    let mut scalar: system_health_policy_settings_t = g_config.health;
    if health_json_bool(
        settings,
        b"health_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut scalar.enabled,
        &raw mut touched,
        error,
    ) != 0 as ::core::ffi::c_int
        || health_json_string(
            settings,
            b"health_profile\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut scalar.profile as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            &raw mut touched,
            error,
        ) != 0 as ::core::ffi::c_int
        || health_json_u32(
            settings,
            b"health_fast_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut scalar.fast_interval_seconds,
            &raw mut touched,
            error,
        ) != 0 as ::core::ffi::c_int
        || health_json_u32(
            settings,
            b"health_normal_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut scalar.normal_interval_seconds,
            &raw mut touched,
            error,
        ) != 0 as ::core::ffi::c_int
        || health_json_u32(
            settings,
            b"health_slow_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut scalar.slow_interval_seconds,
            &raw mut touched,
            error,
        ) != 0 as ::core::ffi::c_int
        || health_json_u32(
            settings,
            b"health_device_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut scalar.device_interval_seconds,
            &raw mut touched,
            error,
        ) != 0 as ::core::ffi::c_int
        || health_json_bool(
            settings,
            b"health_write_probe_enabled\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut scalar.write_probe_enabled,
            &raw mut touched,
            error,
        ) != 0 as ::core::ffi::c_int
        || health_json_string(
            settings,
            b"health_hardware_provider\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut scalar.hardware_provider as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            &raw mut touched,
            error,
        ) != 0 as ::core::ffi::c_int
        || health_json_u32(
            settings,
            b"health_presence_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut scalar.presence_interval_seconds,
            &raw mut touched,
            error,
        ) != 0 as ::core::ffi::c_int
        || health_json_u32(
            settings,
            b"health_incident_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut scalar.incident_retention_days,
            &raw mut touched,
            error,
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut override_object: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        settings,
        b"health_condition_overrides\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut override_json: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !override_object.is_null() {
        touched = r#true != 0;
        *persist_overrides = r#true != 0;
        if cJSON_IsObject(override_object) == 0 {
            snprintf(
                error,
                SYSTEM_HEALTH_POLICY_ERROR_LENGTH as size_t,
                b"health_condition_overrides must be an object\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        override_json = cJSON_PrintUnformatted(override_object);
        if override_json.is_null() {
            snprintf(
                error,
                SYSTEM_HEALTH_POLICY_ERROR_LENGTH as size_t,
                b"could not allocate health overrides\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -2 as ::core::ffi::c_int;
        }
    }
    if !touched {
        return 0 as ::core::ffi::c_int;
    }
    if override_json.is_null() {
        let mut stored_length: size_t = 0 as size_t;
        let mut result: ::core::ffi::c_int = db_get_system_setting_alloc(
            SYSTEM_HEALTH_OVERRIDES_SETTING_KEY.as_ptr(),
            &raw mut override_json,
            &raw mut stored_length,
        );
        if result < 0 as ::core::ffi::c_int {
            snprintf(
                error,
                SYSTEM_HEALTH_POLICY_ERROR_LENGTH as size_t,
                b"could not read existing health overrides\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -2 as ::core::ffi::c_int;
        }
    }
    let mut canonical: *mut ::core::ffi::c_char = calloc(
        SYSTEM_HEALTH_OVERRIDE_JSON_MAX.wrapping_add(1 as ::core::ffi::c_uint) as size_t,
        1 as size_t,
    ) as *mut ::core::ffi::c_char;
    if canonical.is_null() {
        free(override_json as *mut ::core::ffi::c_void);
        snprintf(
            error,
            SYSTEM_HEALTH_POLICY_ERROR_LENGTH as size_t,
            b"could not allocate health policy\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -2 as ::core::ffi::c_int;
    }
    let mut result_0: ::core::ffi::c_int = system_health_policy_build(
        &raw mut scalar,
        override_json,
        candidate,
        canonical,
        SYSTEM_HEALTH_OVERRIDE_JSON_MAX.wrapping_add(1 as ::core::ffi::c_uint) as size_t,
        error,
    );
    free(override_json as *mut ::core::ffi::c_void);
    if result_0 != 0 as ::core::ffi::c_int {
        free(canonical as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    *canonical_out = canonical;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn add_health_settings_json(mut settings: *mut cJSON) {
    cJSON_AddBoolToObject(
        settings,
        b"health_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.health.enabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        settings,
        b"health_profile\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.health.profile as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        settings,
        b"health_fast_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.health.fast_interval_seconds as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"health_normal_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.health.normal_interval_seconds as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"health_slow_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.health.slow_interval_seconds as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"health_device_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.health.device_interval_seconds as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        settings,
        b"health_write_probe_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.health.write_probe_enabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        settings,
        b"health_hardware_provider\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.health.hardware_provider as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        settings,
        b"health_presence_interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.health.presence_interval_seconds as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"health_incident_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.health.incident_retention_days as ::core::ffi::c_double,
    );
    let mut stored: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut stored_length: size_t = 0 as size_t;
    let mut db_result: ::core::ffi::c_int = db_get_system_setting_alloc(
        SYSTEM_HEALTH_OVERRIDES_SETTING_KEY.as_ptr(),
        &raw mut stored,
        &raw mut stored_length,
    );
    let mut candidate: system_health_policy_t = system_health_policy_t {
        generation: 0,
        settings: system_health_policy_settings_t {
            enabled: false,
            profile: [0; 16],
            fast_interval_seconds: 0,
            normal_interval_seconds: 0,
            slow_interval_seconds: 0,
            device_interval_seconds: 0,
            write_probe_enabled: false,
            hardware_provider: [0; 16],
            presence_interval_seconds: 0,
            incident_retention_days: 0,
        },
        conditions: [system_health_condition_policy_t {
            condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
            profile: system_health_profile_t::SYSTEM_HEALTH_PROFILE_BALANCED,
            enabled: false,
            overridden: false,
            direction: system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE,
            unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
            warning_threshold: 0.,
            critical_threshold: 0.,
            recovery_threshold: 0.,
            warning_for_seconds: 0,
            critical_for_seconds: 0,
            recovery_for_seconds: 0,
        }; 27],
    };
    let mut canonical: *mut ::core::ffi::c_char = calloc(
        SYSTEM_HEALTH_OVERRIDE_JSON_MAX.wrapping_add(1 as ::core::ffi::c_uint) as size_t,
        1 as size_t,
    ) as *mut ::core::ffi::c_char;
    let mut error: [::core::ffi::c_char; 256] = [0; 256];
    error[0usize] = '\0' as ::core::ffi::c_char;
    let mut build_result: ::core::ffi::c_int = if !canonical.is_null() {
        system_health_policy_build(
            &raw mut g_config.health,
            if db_result == 0 as ::core::ffi::c_int {
                stored
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            &raw mut candidate,
            canonical,
            SYSTEM_HEALTH_OVERRIDE_JSON_MAX.wrapping_add(1 as ::core::ffi::c_uint) as size_t,
            &raw mut error as *mut ::core::ffi::c_char,
        )
    } else {
        -1 as ::core::ffi::c_int
    };
    free(stored as *mut ::core::ffi::c_void);
    if db_result >= 0 as ::core::ffi::c_int && build_result == 0 as ::core::ffi::c_int {
        let mut overrides: *mut cJSON = cJSON_Parse(canonical);
        if !overrides.is_null() {
            cJSON_AddItemToObject(
                settings,
                b"health_condition_overrides\0".as_ptr() as *const ::core::ffi::c_char,
                overrides,
            );
        }
        let mut effective_json: *mut ::core::ffi::c_char =
            calloc(65536 as ::core::ffi::c_uint as size_t, 1 as size_t) as *mut ::core::ffi::c_char;
        if !effective_json.is_null()
            && system_health_policy_serialize(
                &raw mut candidate,
                effective_json,
                65536 as ::core::ffi::c_uint as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            let mut effective: *mut cJSON = cJSON_Parse(effective_json);
            if !effective.is_null() {
                cJSON_AddItemToObject(
                    settings,
                    b"health_effective_policy\0".as_ptr() as *const ::core::ffi::c_char,
                    effective,
                );
            }
        }
        free(effective_json as *mut ::core::ffi::c_void);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Rejected persisted health policy; active policy unchanged: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            if build_result == 0 as ::core::ffi::c_int {
                b"atomic replacement failed\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                &raw mut error as *mut ::core::ffi::c_char
            },
        );
        cJSON_AddNullToObject(
            settings,
            b"health_condition_overrides\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    free(canonical as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn mqtt_settings_worker(
    mut argument: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut task: *mut mqtt_settings_task_t = argument as *mut mqtt_settings_task_t;
    if task.is_null() {
        return NULL;
    }
    log_set_thread_context(
        b"Settings\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"MQTT settings worker: reinitializing MQTT client...\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    mqtt_delivery_worker_shutdown();
    let mut rc: ::core::ffi::c_int = mqtt_reinit(&raw mut g_config);
    if rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"MQTT settings worker: reinit failed (rc=%d)\0".as_ptr() as *const ::core::ffi::c_char,
            rc,
        );
    } else if mqtt_delivery_worker_start() != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"MQTT settings worker: delivery worker restart failed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"MQTT settings worker: reinit complete (mqtt_enabled=%s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            if g_config.mqtt_enabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    free(task as *mut ::core::ffi::c_void);
    return NULL;
}
unsafe extern "C" fn go2rtc_settings_worker(mut task: *mut go2rtc_settings_task_t) {
    if task.is_null() {
        return;
    }
    log_set_thread_context(
        b"Settings\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let mut all_streams: *mut stream_config_t = calloc(
        g_config.max_streams as size_t,
        ::core::mem::size_of::<stream_config_t>(),
    ) as *mut stream_config_t;
    if all_streams.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc_settings_worker: out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(task as *mut ::core::ffi::c_void);
        return;
    }
    let mut stream_count: ::core::ffi::c_int =
        get_all_stream_configs(all_streams, g_config.max_streams);
    if !(*task).becoming_enabled {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc settings worker: disabling go2rtc...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut lifecycle_guard: go2rtc_lifecycle_guard_t = go2rtc_lifecycle_guard_t {
            acquired: false,
            outermost: false,
            coalesced: false,
            result: false,
            generation: 0,
            restart_generation: 0,
        };
        let mut lifecycle_entered: bool = go2rtc_lifecycle_begin(
            go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_CLEANUP,
            r#false != 0,
            r#true != 0,
            &raw mut lifecycle_guard,
        );
        if !lifecycle_entered {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"go2rtc settings worker: failed to enter lifecycle for disable\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            free(all_streams as *mut ::core::ffi::c_void);
            free(task as *mut ::core::ffi::c_void);
            return;
        }
        go2rtc_integration_cleanup();
        if go2rtc_process_is_running() {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < stream_count {
                if (*all_streams.offset(i as isize)).name[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                    && (*all_streams.offset(i as isize)).enabled as ::core::ffi::c_int != 0
                    && (*all_streams.offset(i as isize)).streaming_enabled as ::core::ffi::c_int
                        != 0
                {
                    stop_hls_stream(
                        &raw mut (*all_streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                    );
                }
                i += 1;
            }
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < stream_count {
                if (*all_streams.offset(i_0 as isize)).name[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                    && (*all_streams.offset(i_0 as isize)).enabled as ::core::ffi::c_int != 0
                    && (*all_streams.offset(i_0 as isize)).record as ::core::ffi::c_int != 0
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Stopping MP4 recording for %s before go2rtc shutdown\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*all_streams.offset(i_0 as isize)).name
                            as *mut ::core::ffi::c_char,
                    );
                    stop_mp4_recording(
                        &raw mut (*all_streams.offset(i_0 as isize)).name
                            as *mut ::core::ffi::c_char,
                    );
                }
                i_0 += 1;
            }
            if !go2rtc_process_stop() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to stop go2rtc process cleanly\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            sleep(2 as ::core::ffi::c_uint);
        }
        go2rtc_stream_cleanup();
        if lifecycle_entered {
            go2rtc_lifecycle_end(&raw mut lifecycle_guard, r#true != 0);
        }
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < stream_count {
            if (*all_streams.offset(i_1 as isize)).name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
                && (*all_streams.offset(i_1 as isize)).enabled as ::core::ffi::c_int != 0
                && (*all_streams.offset(i_1 as isize)).record as ::core::ffi::c_int != 0
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Restarting MP4 recording for %s with direct camera URL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*all_streams.offset(i_1 as isize)).name as *mut ::core::ffi::c_char,
                );
                if start_mp4_recording(
                    &raw mut (*all_streams.offset(i_1 as isize)).name as *mut ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Failed to restart MP4 recording for stream %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*all_streams.offset(i_1 as isize)).name
                            as *mut ::core::ffi::c_char,
                    );
                }
            }
            i_1 += 1;
        }
        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_2 < stream_count {
            if (*all_streams.offset(i_2 as isize)).name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
                && (*all_streams.offset(i_2 as isize)).enabled as ::core::ffi::c_int != 0
                && (*all_streams.offset(i_2 as isize)).streaming_enabled as ::core::ffi::c_int != 0
            {
                if start_hls_stream(
                    &raw mut (*all_streams.offset(i_2 as isize)).name as *mut ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Failed to start native HLS for stream %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*all_streams.offset(i_2 as isize)).name
                            as *mut ::core::ffi::c_char,
                    );
                }
            }
            i_2 += 1;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc settings worker: go2rtc disabled, native HLS and recordings restarted\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"go2rtc settings worker: enabling go2rtc...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_3 < stream_count {
            if (*all_streams.offset(i_3 as isize)).name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
                && (*all_streams.offset(i_3 as isize)).enabled as ::core::ffi::c_int != 0
                && (*all_streams.offset(i_3 as isize)).streaming_enabled as ::core::ffi::c_int != 0
            {
                stop_hls_stream(
                    &raw mut (*all_streams.offset(i_3 as isize)).name as *mut ::core::ffi::c_char,
                );
            }
            i_3 += 1;
        }
        let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_4 < stream_count {
            if (*all_streams.offset(i_4 as isize)).name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
                && (*all_streams.offset(i_4 as isize)).enabled as ::core::ffi::c_int != 0
                && (*all_streams.offset(i_4 as isize)).record as ::core::ffi::c_int != 0
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Stopping MP4 recording for %s before go2rtc startup\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*all_streams.offset(i_4 as isize)).name as *mut ::core::ffi::c_char,
                );
                stop_mp4_recording(
                    &raw mut (*all_streams.offset(i_4 as isize)).name as *mut ::core::ffi::c_char,
                );
            }
            i_4 += 1;
        }
        if !go2rtc_integration_restart_process() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to start go2rtc integration\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_5 < stream_count {
                if (*all_streams.offset(i_5 as isize)).name[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                    && (*all_streams.offset(i_5 as isize)).enabled as ::core::ffi::c_int != 0
                    && (*all_streams.offset(i_5 as isize)).record as ::core::ffi::c_int != 0
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Restarting MP4 recording for %s with direct URL (go2rtc failed)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*all_streams.offset(i_5 as isize)).name
                            as *mut ::core::ffi::c_char,
                    );
                    if start_mp4_recording(
                        &raw mut (*all_streams.offset(i_5 as isize)).name
                            as *mut ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Failed to restart MP4 recording for stream %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*all_streams.offset(i_5 as isize)).name
                                as *mut ::core::ffi::c_char,
                        );
                    }
                }
                i_5 += 1;
            }
        } else {
            let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_6 < stream_count {
                if (*all_streams.offset(i_6 as isize)).name[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                    && (*all_streams.offset(i_6 as isize)).enabled as ::core::ffi::c_int != 0
                    && (*all_streams.offset(i_6 as isize)).record as ::core::ffi::c_int != 0
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Restarting MP4 recording for %s via go2rtc\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*all_streams.offset(i_6 as isize)).name
                            as *mut ::core::ffi::c_char,
                    );
                    if go2rtc_integration_start_recording(
                        &raw mut (*all_streams.offset(i_6 as isize)).name
                            as *mut ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Failed to restart MP4 recording for stream %s via go2rtc\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*all_streams.offset(i_6 as isize)).name
                                as *mut ::core::ffi::c_char,
                        );
                    }
                }
                i_6 += 1;
            }
            let mut i_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_7 < stream_count {
                if (*all_streams.offset(i_7 as isize)).name[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                    && (*all_streams.offset(i_7 as isize)).enabled as ::core::ffi::c_int != 0
                    && (*all_streams.offset(i_7 as isize)).streaming_enabled as ::core::ffi::c_int
                        != 0
                {
                    if stream_start_hls(
                        &raw mut (*all_streams.offset(i_7 as isize)).name
                            as *mut ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Failed to start HLS for stream %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*all_streams.offset(i_7 as isize)).name
                                as *mut ::core::ffi::c_char,
                        );
                    }
                }
                i_7 += 1;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"go2rtc settings worker: go2rtc enabled, streams and recordings restarted\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    free(all_streams as *mut ::core::ffi::c_void);
    free(task as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn go2rtc_validation_result_to_json(
    mut r: *const yaml_validation_result_t,
) -> *mut cJSON {
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddBoolToObject(
        root,
        b"valid\0".as_ptr() as *const ::core::ffi::c_char,
        ((*r).valid != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    cJSON_AddBoolToObject(
        root,
        b"libyaml_available\0".as_ptr() as *const ::core::ffi::c_char,
        ((*r).valid != -1 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    cJSON_AddBoolToObject(
        root,
        b"skipped\0".as_ptr() as *const ::core::ffi::c_char,
        ((*r).valid == -1 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    if (*r).valid == 0 as ::core::ffi::c_int {
        let mut err: *mut cJSON = cJSON_CreateObject();
        if !err.is_null() {
            cJSON_AddNumberToObject(
                err,
                b"line\0".as_ptr() as *const ::core::ffi::c_char,
                (*r).err_line as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                err,
                b"column\0".as_ptr() as *const ::core::ffi::c_char,
                (*r).err_column as ::core::ffi::c_double,
            );
            cJSON_AddStringToObject(
                err,
                b"message\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*r).err_message as *const ::core::ffi::c_char,
            );
            cJSON_AddBoolToObject(
                err,
                b"duplicate_keys\0".as_ptr() as *const ::core::ffi::c_char,
                ((*r).duplicate_keys != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            );
            cJSON_AddBoolToObject(
                err,
                b"non_mapping_root\0".as_ptr() as *const ::core::ffi::c_char,
                ((*r).non_mapping_root != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            );
            cJSON_AddBoolToObject(
                err,
                b"parse_error\0".as_ptr() as *const ::core::ffi::c_char,
                ((*r).parse_error != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            );
            cJSON_AddItemToObject(root, b"error\0".as_ptr() as *const ::core::ffi::c_char, err);
        }
    } else if (*r).valid == -1 as ::core::ffi::c_int {
        let mut skip_obj: *mut cJSON = cJSON_CreateObject();
        if !skip_obj.is_null() {
            cJSON_AddStringToObject(
                skip_obj,
                b"message\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*r).err_message as *const ::core::ffi::c_char,
            );
            cJSON_AddItemToObject(
                root,
                b"skip_reason\0".as_ptr() as *const ::core::ffi::c_char,
                skip_obj,
            );
        }
    }
    let mut warnings: *mut cJSON = cJSON_CreateArray();
    if warnings.is_null() {
        cJSON_Delete(root);
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*r).warning_count {
        cJSON_AddItemToArray(
            warnings,
            cJSON_CreateString(
                &raw const *(&raw const (*r).warnings as *const [::core::ffi::c_char; 192])
                    .offset(i as isize) as *const ::core::ffi::c_char,
            ),
        );
        i += 1;
    }
    cJSON_AddItemToObject(
        root,
        b"warnings\0".as_ptr() as *const ::core::ffi::c_char,
        warnings,
    );
    return root;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_settings_go2rtc_validate(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/settings/go2rtc/validate request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_SYSTEM_ADMIN) == 0 {
        return;
    }
    let mut body: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut src: *const ::core::ffi::c_char = b"\0".as_ptr() as *const ::core::ffi::c_char;
    let mut src_len: size_t = 0 as size_t;
    if (*req).body_len > 0 as size_t {
        body = httpd_parse_json_body(req);
        if body.is_null() {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid JSON in request body\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut r#override: *mut cJSON =
            cJSON_GetObjectItem(body, b"override\0".as_ptr() as *const ::core::ffi::c_char);
        if !r#override.is_null()
            && cJSON_IsString(r#override) != 0
            && !(*r#override).valuestring.is_null()
        {
            src = (*r#override).valuestring;
            src_len = strlen(src);
        }
    }
    if src_len >= 65536 as ::core::ffi::c_int as size_t {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            413 as ::core::ffi::c_int,
            b"override exceeds 65535 byte limit\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut r: yaml_validation_result_t = yaml_validation_result_t {
        valid: 0,
        err_message: [0; 256],
        err_line: 0,
        err_column: 0,
        duplicate_keys: 0,
        non_mapping_root: 0,
        parse_error: 0,
        warnings: [[0; 192]; 16],
        warning_count: 0,
    };
    yaml_validate_go2rtc_override(src, src_len, &raw mut r);
    let mut result: *mut cJSON = go2rtc_validation_result_to_json(&raw mut r);
    cJSON_Delete(body);
    if result.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut result_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(result);
    cJSON_Delete(result);
    if result_str.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, result_str);
    free(result_str as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_client_config(
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
    if httpd_check_action_access(req, &raw mut user) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut config: *mut cJSON = cJSON_CreateObject();
    if config.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create client config\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        config,
        b"go2rtc_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.go2rtc_enabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        config,
        b"web_auth_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.web_auth_enabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        config,
        b"demo_mode\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.demo_mode as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        config,
        b"audio_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.audio_disabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        config,
        b"auto_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.auto_disabled as cJSON_bool,
    );
    let mut go2rtc_available: bool = r#false != 0;
    if g_config.go2rtc_enabled {
        let mut go2rtc_pid: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        if go2rtc_process_try_get_pid(&raw mut go2rtc_pid) {
            go2rtc_available = go2rtc_pid > 0 as ::core::ffi::c_int;
        } else {
            go2rtc_available = r#true != 0;
        }
    }
    cJSON_AddBoolToObject(
        config,
        b"go2rtc_available\0".as_ptr() as *const ::core::ffi::c_char,
        go2rtc_available as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        config,
        b"go2rtc_api_port\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.go2rtc_api_port as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        config,
        b"go2rtc_base_path\0".as_ptr() as *const ::core::ffi::c_char,
        go2rtc_api_get_base_path(),
    );
    cJSON_AddBoolToObject(
        config,
        b"webrtc_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.webrtc_disabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        config,
        b"hls_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.hls_disabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        config,
        b"mse_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.mse_disabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        config,
        b"default_playback_transport\0".as_ptr() as *const ::core::ffi::c_char,
        if playback_transport_is_valid(
            &raw mut g_config.default_playback_transport as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        {
            &raw mut g_config.default_playback_transport as *mut ::core::ffi::c_char
        } else {
            b"auto\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddBoolToObject(
        config,
        b"go2rtc_force_native_hls\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.go2rtc_force_native_hls as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        config,
        b"webrtc_connection_timeout_ms\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.webrtc_connection_timeout_ms as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        config,
        b"webrtc_ice_recovery_timeout_ms\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.webrtc_ice_recovery_timeout_ms as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        config,
        b"generate_thumbnails\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.generate_thumbnails as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        config,
        b"thumbnails_per_recording\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.thumbnails_per_recording as ::core::ffi::c_double,
    );
    let mut json: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(config);
    cJSON_Delete(config);
    if json.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize client config\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json);
    free(json as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_settings(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/settings request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_SYSTEM_ADMIN) == 0 {
        return;
    }
    let mut settings: *mut cJSON = cJSON_CreateObject();
    if settings.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create settings JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create settings JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddNumberToObject(
        settings,
        b"web_thread_pool_size\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.web_thread_pool_size as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"web_port\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.web_port as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        settings,
        b"web_bind_ip\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.web_bind_ip as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        settings,
        b"web_root\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.web_root as *mut ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        settings,
        b"web_auth_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.web_auth_enabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        settings,
        b"demo_mode\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.demo_mode as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        settings,
        b"audio_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.audio_disabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        settings,
        b"auto_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.auto_disabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        settings,
        b"webrtc_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.webrtc_disabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        settings,
        b"hls_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.hls_disabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        settings,
        b"mse_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.mse_disabled as cJSON_bool,
    );
    add_health_settings_json(settings);
    cJSON_AddStringToObject(
        settings,
        b"storage_path\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        settings,
        b"storage_path_hls\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.storage_path_hls as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        settings,
        b"max_storage_size\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.max_storage_size as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.retention_days as ::core::ffi::c_double,
    );
    let mut lpr_retention: [::core::ffi::c_char; 32] = [
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
    let mut days: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    if db_get_system_setting(
        b"lpr_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut lpr_retention as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as ::core::ffi::c_int,
    ) == 0 as ::core::ffi::c_int
    {
        let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut parsed: ::core::ffi::c_long = strtol(
            &raw mut lpr_retention as *mut ::core::ffi::c_char,
            &raw mut end,
            10 as ::core::ffi::c_int,
        );
        if end != &raw mut lpr_retention as *mut ::core::ffi::c_char
            && *end as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            && parsed >= 1 as ::core::ffi::c_long
            && parsed <= 36500 as ::core::ffi::c_long
        {
            days = parsed as ::core::ffi::c_int;
        }
    }
    cJSON_AddNumberToObject(
        settings,
        b"lpr_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        days as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        settings,
        b"auto_delete_oldest\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.auto_delete_oldest as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        settings,
        b"mp4_faststart\0".as_ptr() as *const ::core::ffi::c_char,
        config_get_mp4_faststart(&raw mut g_config) as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        settings,
        b"storage_min_free_pct\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.storage_min_free_pct as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"storage_pressure_warning_pct\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.storage_pressure_warning_pct,
    );
    cJSON_AddNumberToObject(
        settings,
        b"storage_pressure_critical_pct\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.storage_pressure_critical_pct,
    );
    cJSON_AddNumberToObject(
        settings,
        b"storage_pressure_emergency_pct\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.storage_pressure_emergency_pct,
    );
    cJSON_AddBoolToObject(
        settings,
        b"generate_thumbnails\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.generate_thumbnails as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        settings,
        b"thumbnails_per_recording\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.thumbnails_per_recording as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        settings,
        b"mp4_directory_format\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.mp4_directory_format as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        settings,
        b"max_streams\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.max_streams as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"max_streams_ceiling\0".as_ptr() as *const ::core::ffi::c_char,
        MAX_STREAMS as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        settings,
        b"log_file\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.log_file as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        settings,
        b"log_level\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.log_level as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        settings,
        b"pid_file\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.pid_file as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        settings,
        b"db_path\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.db_path as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        settings,
        b"db_backup_interval_minutes\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.db_backup_interval_minutes as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"db_backup_retention_count\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.db_backup_retention_count as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        settings,
        b"db_post_backup_script\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.db_post_backup_script as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        settings,
        b"db_backup_verify\0".as_ptr() as *const ::core::ffi::c_char,
        if g_config.db_backup_verify == DB_BACKUP_VERIFY_OFF {
            b"off\0".as_ptr() as *const ::core::ffi::c_char
        } else if g_config.db_backup_verify == DB_BACKUP_VERIFY_QUICK {
            b"quick\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"full\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddStringToObject(
        settings,
        b"models_path\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.models_path as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        settings,
        b"buffer_size\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.buffer_size as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        settings,
        b"use_swap\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.use_swap as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        settings,
        b"swap_size\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.swap_size as ::core::ffi::c_double
            / (1024 as ::core::ffi::c_ulonglong).wrapping_mul(1024 as ::core::ffi::c_ulonglong)
                as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        settings,
        b"syslog_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.syslog_enabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        settings,
        b"syslog_ident\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.syslog_ident as *mut ::core::ffi::c_char,
    );
    let mut facility_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    match g_config.syslog_facility {
        LOG_USER => {
            facility_name = b"LOG_USER\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_DAEMON => {
            facility_name = b"LOG_DAEMON\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL0 => {
            facility_name = b"LOG_LOCAL0\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL1 => {
            facility_name = b"LOG_LOCAL1\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL2 => {
            facility_name = b"LOG_LOCAL2\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL3 => {
            facility_name = b"LOG_LOCAL3\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL4 => {
            facility_name = b"LOG_LOCAL4\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL5 => {
            facility_name = b"LOG_LOCAL5\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL6 => {
            facility_name = b"LOG_LOCAL6\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LOG_LOCAL7 => {
            facility_name = b"LOG_LOCAL7\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {
            facility_name = b"LOG_USER\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    cJSON_AddStringToObject(
        settings,
        b"syslog_facility\0".as_ptr() as *const ::core::ffi::c_char,
        facility_name,
    );
    cJSON_AddStringToObject(
        settings,
        b"api_detection_url\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.api_detection_url as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        settings,
        b"api_detection_backend\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.api_detection_backend as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        settings,
        b"api_detection_format\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.api_detection_format as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        settings,
        b"api_detection_detector_name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.api_detection_detector_name as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        settings,
        b"default_detection_threshold\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.default_detection_threshold as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"pre_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.default_pre_detection_buffer as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"post_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.default_post_detection_buffer as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        settings,
        b"default_playback_transport\0".as_ptr() as *const ::core::ffi::c_char,
        if playback_transport_is_valid(
            &raw mut g_config.default_playback_transport as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        {
            &raw mut g_config.default_playback_transport as *mut ::core::ffi::c_char
        } else {
            b"auto\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddNumberToObject(
        settings,
        b"detection_grace_period\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.detection_grace_period as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        settings,
        b"detection_engine_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.detection_engine.enabled as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        settings,
        b"detection_engine_threads\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.detection_engine.num_threads as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        settings,
        b"detection_engine_delegate\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.detection_engine.delegate as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        settings,
        b"auth_timeout_hours\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.auth_timeout_hours as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"auth_absolute_timeout_hours\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.auth_absolute_timeout_hours as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"trusted_device_days\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.trusted_device_days as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        settings,
        b"trusted_proxy_cidrs\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.trusted_proxy_cidrs as *mut ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        settings,
        b"force_mfa_on_login\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.force_mfa_on_login as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        settings,
        b"login_rate_limit_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.login_rate_limit_enabled as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        settings,
        b"login_rate_limit_max_attempts\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.login_rate_limit_max_attempts as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"login_rate_limit_window_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.login_rate_limit_window_seconds as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        settings,
        b"go2rtc_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.go2rtc_enabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        settings,
        b"go2rtc_binary_path\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.go2rtc_binary_path as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        settings,
        b"go2rtc_config_dir\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.go2rtc_config_dir as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        settings,
        b"go2rtc_api_port\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.go2rtc_api_port as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        settings,
        b"go2rtc_base_path\0".as_ptr() as *const ::core::ffi::c_char,
        go2rtc_api_get_base_path(),
    );
    cJSON_AddNumberToObject(
        settings,
        b"go2rtc_rtsp_port\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.go2rtc_rtsp_port as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        settings,
        b"go2rtc_webrtc_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.go2rtc_webrtc_enabled as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        settings,
        b"go2rtc_webrtc_listen_port\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.go2rtc_webrtc_listen_port as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        settings,
        b"go2rtc_stun_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.go2rtc_stun_enabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        settings,
        b"go2rtc_stun_server\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.go2rtc_stun_server as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        settings,
        b"go2rtc_external_ip\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.go2rtc_external_ip as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        settings,
        b"go2rtc_ice_servers\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.go2rtc_ice_servers as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        settings,
        b"webrtc_connection_timeout_ms\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.webrtc_connection_timeout_ms as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"webrtc_ice_recovery_timeout_ms\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.webrtc_ice_recovery_timeout_ms as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        settings,
        b"go2rtc_force_native_hls\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.go2rtc_force_native_hls as cJSON_bool,
    );
    let mut go2rtc_config_override_buf: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut go2rtc_config_override_len: size_t = 0 as size_t;
    let mut rc_override: ::core::ffi::c_int = db_get_system_setting_alloc(
        b"go2rtc_config_override\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut go2rtc_config_override_buf,
        &raw mut go2rtc_config_override_len,
    );
    if rc_override == 0 as ::core::ffi::c_int && !go2rtc_config_override_buf.is_null() {
        cJSON_AddStringToObject(
            settings,
            b"go2rtc_config_override\0".as_ptr() as *const ::core::ffi::c_char,
            go2rtc_config_override_buf,
        );
    } else {
        cJSON_AddStringToObject(
            settings,
            b"go2rtc_config_override\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    free(go2rtc_config_override_buf as *mut ::core::ffi::c_void);
    let mut reason: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut reason_len: size_t = 0 as size_t;
    let mut rc: ::core::ffi::c_int = db_get_system_setting_alloc(
        b"go2rtc_config_override_disabled_reason\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut reason,
        &raw mut reason_len,
    );
    if rc == 0 as ::core::ffi::c_int && !reason.is_null() && reason_len > 0 as size_t {
        cJSON_AddStringToObject(
            settings,
            b"go2rtc_config_override_disabled_reason\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        );
    } else {
        cJSON_AddStringToObject(
            settings,
            b"go2rtc_config_override_disabled_reason\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    free(reason as *mut ::core::ffi::c_void);
    let mut quar: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut quar_len: size_t = 0 as size_t;
    rc = db_get_system_setting_alloc(
        b"go2rtc_config_override_quarantined\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut quar,
        &raw mut quar_len,
    );
    if rc == 0 as ::core::ffi::c_int && !quar.is_null() && quar_len > 0 as size_t {
        cJSON_AddStringToObject(
            settings,
            b"go2rtc_config_override_quarantined\0".as_ptr() as *const ::core::ffi::c_char,
            quar,
        );
    } else {
        cJSON_AddStringToObject(
            settings,
            b"go2rtc_config_override_quarantined\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    free(quar as *mut ::core::ffi::c_void);
    cJSON_AddBoolToObject(
        settings,
        b"mqtt_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.mqtt_enabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        settings,
        b"mqtt_broker_host\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.mqtt_broker_host as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        settings,
        b"mqtt_broker_port\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.mqtt_broker_port as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        settings,
        b"mqtt_username\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.mqtt_username as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        settings,
        b"mqtt_password\0".as_ptr() as *const ::core::ffi::c_char,
        if g_config.mqtt_password[0usize] as ::core::ffi::c_int != 0 {
            b"********\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddStringToObject(
        settings,
        b"mqtt_client_id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.mqtt_client_id as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        settings,
        b"mqtt_topic_prefix\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.mqtt_topic_prefix as *mut ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        settings,
        b"mqtt_tls_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.mqtt_tls_enabled as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        settings,
        b"mqtt_keepalive\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.mqtt_keepalive as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        settings,
        b"mqtt_qos\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.mqtt_qos as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        settings,
        b"mqtt_retain\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.mqtt_retain as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        settings,
        b"mqtt_ha_discovery\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.mqtt_ha_discovery as cJSON_bool,
    );
    cJSON_AddStringToObject(
        settings,
        b"mqtt_ha_discovery_prefix\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.mqtt_ha_discovery_prefix as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        settings,
        b"mqtt_ha_snapshot_interval\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.mqtt_ha_snapshot_interval as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        settings,
        b"turn_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.turn_enabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        settings,
        b"turn_server_url\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.turn_server_url as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        settings,
        b"turn_username\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.turn_username as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        settings,
        b"turn_password\0".as_ptr() as *const ::core::ffi::c_char,
        if g_config.turn_password[0usize] as ::core::ffi::c_int != 0 {
            b"********\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddBoolToObject(
        settings,
        b"onvif_discovery_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.onvif_discovery_enabled as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        settings,
        b"onvif_discovery_interval\0".as_ptr() as *const ::core::ffi::c_char,
        g_config.onvif_discovery_interval as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        settings,
        b"onvif_discovery_network\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.onvif_discovery_network as *mut ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(settings);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert settings JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(settings);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert settings JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(settings);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/settings request\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_settings(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/settings request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_SYSTEM_ADMIN) == 0 {
        return;
    }
    let mut settings: *mut cJSON = httpd_parse_json_body(req);
    if settings.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to parse settings JSON from request body\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON in request body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut format: *const cJSON = cJSON_GetObjectItem(
        settings,
        b"api_detection_format\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut parsed_format: api_detection_format_t =
        api_detection_format_t::API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT;
    if !format.is_null()
        && cJSON_IsString(format) != 0
        && !api_detection_format_parse((*format).valuestring, &raw mut parsed_format)
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Rejected invalid api_detection_format setting\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        cJSON_Delete(settings);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid api_detection_format: expected light-object-detect or doods2\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut settings_changed: bool = r#false != 0;
    let mut restart_required: bool = r#false != 0;
    let mut web_thread_pool_restart_required: bool = r#false != 0;
    let mut max_streams_restart_required: bool = r#false != 0;
    let mut go2rtc_config_changed: bool = r#false != 0;
    let mut go2rtc_becoming_enabled: bool = r#false != 0;
    let mut mqtt_config_changed: bool = r#false != 0;
    let mut old_mqtt_enabled: bool = g_config.mqtt_enabled;
    let mut old_mqtt_broker_host: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut old_mqtt_broker_host as *mut ::core::ffi::c_char,
        &raw mut g_config.mqtt_broker_host as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    let mut old_mqtt_broker_port: ::core::ffi::c_int = g_config.mqtt_broker_port;
    let mut old_mqtt_username: [::core::ffi::c_char; 128] = [0; 128];
    safe_strcpy(
        &raw mut old_mqtt_username as *mut ::core::ffi::c_char,
        &raw mut g_config.mqtt_username as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    let mut old_mqtt_password: [::core::ffi::c_char; 128] = [0; 128];
    safe_strcpy(
        &raw mut old_mqtt_password as *mut ::core::ffi::c_char,
        &raw mut g_config.mqtt_password as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    let mut old_mqtt_client_id: [::core::ffi::c_char; 128] = [0; 128];
    safe_strcpy(
        &raw mut old_mqtt_client_id as *mut ::core::ffi::c_char,
        &raw mut g_config.mqtt_client_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    let mut old_mqtt_topic_prefix: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut old_mqtt_topic_prefix as *mut ::core::ffi::c_char,
        &raw mut g_config.mqtt_topic_prefix as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    let mut old_mqtt_tls_enabled: bool = g_config.mqtt_tls_enabled;
    let mut old_mqtt_keepalive: ::core::ffi::c_int = g_config.mqtt_keepalive;
    let mut old_mqtt_qos: ::core::ffi::c_int = g_config.mqtt_qos;
    let mut old_mqtt_retain: bool = g_config.mqtt_retain;
    let mut old_mqtt_ha_discovery: bool = g_config.mqtt_ha_discovery;
    let mut old_mqtt_ha_discovery_prefix: [::core::ffi::c_char; 128] = [0; 128];
    safe_strcpy(
        &raw mut old_mqtt_ha_discovery_prefix as *mut ::core::ffi::c_char,
        &raw mut g_config.mqtt_ha_discovery_prefix as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    let mut old_mqtt_ha_snapshot_interval: ::core::ffi::c_int = g_config.mqtt_ha_snapshot_interval;
    let mut web_thread_pool_size_j: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"web_thread_pool_size\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !web_thread_pool_size_j.is_null() && cJSON_IsNumber(web_thread_pool_size_j) != 0 {
        let mut v: ::core::ffi::c_int = (*web_thread_pool_size_j).valueint;
        if v < 2 as ::core::ffi::c_int {
            v = 2 as ::core::ffi::c_int;
        }
        if v > 128 as ::core::ffi::c_int {
            v = 128 as ::core::ffi::c_int;
        }
        if v != g_config.web_thread_pool_size {
            g_config.web_thread_pool_size = v;
            settings_changed = r#true != 0;
            restart_required = r#true != 0;
            web_thread_pool_restart_required = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Updated web_thread_pool_size: %d (restart required)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                v,
            );
        }
    }
    let mut web_port: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"web_port\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !web_port.is_null() && cJSON_IsNumber(web_port) != 0 {
        g_config.web_port = (*web_port).valueint;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated web_port: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.web_port,
        );
    }
    let mut web_bind_ip: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"web_bind_ip\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !web_bind_ip.is_null() && cJSON_IsString(web_bind_ip) != 0 {
        let mut new_bind_ip: *const ::core::ffi::c_char = (*web_bind_ip).valuestring;
        let mut bind_ip_empty: bool = new_bind_ip.is_null();
        if !bind_ip_empty {
            while *(*__ctype_b_loc())
                .offset(*new_bind_ip as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
            {
                new_bind_ip = new_bind_ip.offset(1);
            }
            bind_ip_empty = *new_bind_ip as ::core::ffi::c_int == '\0' as ::core::ffi::c_int;
        }
        if bind_ip_empty {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Rejected empty web_bind_ip update\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if strcmp(
            &raw mut g_config.web_bind_ip as *mut ::core::ffi::c_char,
            new_bind_ip,
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut g_config.web_bind_ip as *mut ::core::ffi::c_char,
                new_bind_ip,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                0 as size_t,
            );
            settings_changed = r#true != 0;
            restart_required = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Updated web_bind_ip: %s (restart required)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut g_config.web_bind_ip as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut web_root: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"web_root\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !web_root.is_null() && cJSON_IsString(web_root) != 0 {
        safe_strcpy(
            &raw mut g_config.web_root as *mut ::core::ffi::c_char,
            (*web_root).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated web_root: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.web_root as *mut ::core::ffi::c_char,
        );
    }
    let mut web_auth_enabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"web_auth_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !web_auth_enabled.is_null() && cJSON_IsBool(web_auth_enabled) != 0 {
        g_config.web_auth_enabled = cJSON_IsTrue(web_auth_enabled) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated web_auth_enabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.web_auth_enabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut demo_mode: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"demo_mode\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !demo_mode.is_null() && cJSON_IsBool(demo_mode) != 0 {
        g_config.demo_mode = cJSON_IsTrue(demo_mode) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated demo_mode: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.demo_mode as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut audio_disabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"audio_disabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !audio_disabled.is_null() && cJSON_IsBool(audio_disabled) != 0 {
        let mut new_audio_disabled: bool = cJSON_IsTrue(audio_disabled) != 0;
        if new_audio_disabled as ::core::ffi::c_int != g_config.audio_disabled as ::core::ffi::c_int
        {
            g_config.audio_disabled = new_audio_disabled;
            settings_changed = r#true != 0;
            if g_config.go2rtc_enabled {
                go2rtc_config_changed = r#true != 0;
                go2rtc_becoming_enabled = r#true != 0;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Updated audio_disabled: %s%s\0".as_ptr() as *const ::core::ffi::c_char,
                if g_config.audio_disabled as ::core::ffi::c_int != 0 {
                    b"true\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"false\0".as_ptr() as *const ::core::ffi::c_char
                },
                if g_config.go2rtc_enabled as ::core::ffi::c_int != 0 {
                    b"; restarting go2rtc media workers\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
    }
    let mut auto_disabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"auto_disabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut webrtc_disabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"webrtc_disabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut hls_disabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"hls_disabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut mse_disabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mse_disabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut new_webrtc_disabled: bool =
        if !webrtc_disabled.is_null() && cJSON_IsBool(webrtc_disabled) != 0 {
            cJSON_IsTrue(webrtc_disabled)
        } else {
            g_config.webrtc_disabled as ::core::ffi::c_int
        } != 0;
    let mut new_hls_disabled: bool = if !hls_disabled.is_null() && cJSON_IsBool(hls_disabled) != 0 {
        cJSON_IsTrue(hls_disabled)
    } else {
        g_config.hls_disabled as ::core::ffi::c_int
    } != 0;
    let mut new_mse_disabled: bool = if !mse_disabled.is_null() && cJSON_IsBool(mse_disabled) != 0 {
        cJSON_IsTrue(mse_disabled)
    } else {
        g_config.mse_disabled as ::core::ffi::c_int
    } != 0;
    if new_webrtc_disabled as ::core::ffi::c_int != 0
        && new_hls_disabled as ::core::ffi::c_int != 0
        && new_mse_disabled as ::core::ffi::c_int != 0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Rejected settings: all three concrete playback transports disabled\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        cJSON_Delete(settings);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"At least one dashboard view method (WebRTC, HLS, or MSE) must remain enabled\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !auto_disabled.is_null() && cJSON_IsBool(auto_disabled) != 0 {
        g_config.auto_disabled = cJSON_IsTrue(auto_disabled) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated auto_disabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.auto_disabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    if !webrtc_disabled.is_null() && cJSON_IsBool(webrtc_disabled) != 0 {
        let mut old_webrtc_disabled: bool = g_config.webrtc_disabled;
        g_config.webrtc_disabled = cJSON_IsTrue(webrtc_disabled) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated webrtc_disabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.webrtc_disabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        if old_webrtc_disabled as ::core::ffi::c_int
            != g_config.webrtc_disabled as ::core::ffi::c_int
        {
            go2rtc_config_changed = r#true != 0;
            go2rtc_becoming_enabled = g_config.go2rtc_enabled;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"WebRTC disabled setting changed, will restart go2rtc\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if !hls_disabled.is_null() && cJSON_IsBool(hls_disabled) != 0 {
        g_config.hls_disabled = cJSON_IsTrue(hls_disabled) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated hls_disabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.hls_disabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    if !mse_disabled.is_null() && cJSON_IsBool(mse_disabled) != 0 {
        g_config.mse_disabled = cJSON_IsTrue(mse_disabled) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mse_disabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.mse_disabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut auth_timeout_hours: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"auth_timeout_hours\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !auth_timeout_hours.is_null() && cJSON_IsNumber(auth_timeout_hours) != 0 {
        let mut value: ::core::ffi::c_int = (*auth_timeout_hours).valueint;
        if value < 1 as ::core::ffi::c_int {
            value = 1 as ::core::ffi::c_int;
        }
        g_config.auth_timeout_hours = value;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated auth_timeout_hours: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.auth_timeout_hours,
        );
    }
    let mut auth_absolute_timeout_hours: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"auth_absolute_timeout_hours\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !auth_absolute_timeout_hours.is_null() && cJSON_IsNumber(auth_absolute_timeout_hours) != 0 {
        let mut value_0: ::core::ffi::c_int = (*auth_absolute_timeout_hours).valueint;
        if value_0 < 1 as ::core::ffi::c_int {
            value_0 = 1 as ::core::ffi::c_int;
        }
        g_config.auth_absolute_timeout_hours = value_0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated auth_absolute_timeout_hours: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.auth_absolute_timeout_hours,
        );
    }
    let mut trusted_device_days: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"trusted_device_days\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !trusted_device_days.is_null() && cJSON_IsNumber(trusted_device_days) != 0 {
        let mut value_1: ::core::ffi::c_int = (*trusted_device_days).valueint;
        if value_1 < 0 as ::core::ffi::c_int {
            value_1 = 0 as ::core::ffi::c_int;
        }
        g_config.trusted_device_days = value_1;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated trusted_device_days: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.trusted_device_days,
        );
    }
    let mut trusted_proxy_cidrs: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"trusted_proxy_cidrs\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !trusted_proxy_cidrs.is_null() && cJSON_IsString(trusted_proxy_cidrs) != 0 {
        if db_auth_validate_allowed_login_cidrs((*trusted_proxy_cidrs).valuestring)
            != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Rejected invalid trusted_proxy_cidrs setting\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid trusted_proxy_cidrs: expected comma- or newline-separated IPv4/IPv6 CIDRs\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        safe_strcpy(
            &raw mut g_config.trusted_proxy_cidrs as *mut ::core::ffi::c_char,
            (*trusted_proxy_cidrs).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated trusted_proxy_cidrs\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if g_config.auth_absolute_timeout_hours < g_config.auth_timeout_hours {
        g_config.auth_absolute_timeout_hours = g_config.auth_timeout_hours;
    }
    let mut force_mfa_on_login: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"force_mfa_on_login\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !force_mfa_on_login.is_null() && cJSON_IsBool(force_mfa_on_login) != 0 {
        g_config.force_mfa_on_login = cJSON_IsTrue(force_mfa_on_login) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated force_mfa_on_login: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.force_mfa_on_login as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut login_rate_limit_enabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"login_rate_limit_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !login_rate_limit_enabled.is_null() && cJSON_IsBool(login_rate_limit_enabled) != 0 {
        g_config.login_rate_limit_enabled = cJSON_IsTrue(login_rate_limit_enabled) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated login_rate_limit_enabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.login_rate_limit_enabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut login_rate_limit_max_attempts: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"login_rate_limit_max_attempts\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !login_rate_limit_max_attempts.is_null()
        && cJSON_IsNumber(login_rate_limit_max_attempts) != 0
    {
        let mut value_2: ::core::ffi::c_int = (*login_rate_limit_max_attempts).valueint;
        if value_2 < 1 as ::core::ffi::c_int {
            value_2 = 1 as ::core::ffi::c_int;
        }
        g_config.login_rate_limit_max_attempts = value_2;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated login_rate_limit_max_attempts: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.login_rate_limit_max_attempts,
        );
    }
    let mut login_rate_limit_window_seconds: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"login_rate_limit_window_seconds\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !login_rate_limit_window_seconds.is_null()
        && cJSON_IsNumber(login_rate_limit_window_seconds) != 0
    {
        let mut value_3: ::core::ffi::c_int = (*login_rate_limit_window_seconds).valueint;
        if value_3 < 10 as ::core::ffi::c_int {
            value_3 = 10 as ::core::ffi::c_int;
        }
        g_config.login_rate_limit_window_seconds = value_3;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated login_rate_limit_window_seconds: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.login_rate_limit_window_seconds,
        );
    }
    let mut storage_path: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"storage_path\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !storage_path.is_null() && cJSON_IsString(storage_path) != 0 {
        if !is_safe_storage_path((*storage_path).valuestring) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Rejected unsafe storage_path: %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*storage_path).valuestring,
            );
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid storage_path: must be an absolute path without shell metacharacters\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        safe_strcpy(
            &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
            (*storage_path).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated storage_path: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
        );
    }
    let mut storage_path_hls: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"storage_path_hls\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !storage_path_hls.is_null() && cJSON_IsString(storage_path_hls) != 0 {
        let mut hls_path_val: *const ::core::ffi::c_char = (*storage_path_hls).valuestring;
        if *hls_path_val.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && !is_safe_storage_path(hls_path_val)
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Rejected unsafe storage_path_hls: %s\0".as_ptr() as *const ::core::ffi::c_char,
                hls_path_val,
            );
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid storage_path_hls: must be an absolute path without shell metacharacters\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        safe_strcpy(
            &raw mut g_config.storage_path_hls as *mut ::core::ffi::c_char,
            hls_path_val,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated storage_path_hls: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if *hls_path_val.offset(0isize) as ::core::ffi::c_int != 0 {
                hls_path_val
            } else {
                b"(cleared, will use storage_path)\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut mp4_directory_format: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mp4_directory_format\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mp4_directory_format.is_null() && cJSON_IsString(mp4_directory_format) != 0 {
        if !mp4_directory_format_is_valid((*mp4_directory_format).valuestring) {
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"mp4_directory_format must be flat, year_month, or year_month_day\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        safe_strcpy(
            &raw mut g_config.mp4_directory_format as *mut ::core::ffi::c_char,
            (*mp4_directory_format).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mp4_directory_format: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.mp4_directory_format as *mut ::core::ffi::c_char,
        );
    }
    let mut max_storage_size: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"max_storage_size\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !max_storage_size.is_null() && cJSON_IsNumber(max_storage_size) != 0 {
        g_config.max_storage_size = (*max_storage_size).valueint as uint64_t;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated max_storage_size: %lu\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.max_storage_size,
        );
    }
    let mut retention_days: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !retention_days.is_null() && cJSON_IsNumber(retention_days) != 0 {
        if (*retention_days).valueint < 0 as ::core::ffi::c_int
            || (*retention_days).valueint > 365 as ::core::ffi::c_int
        {
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"retention_days must be between 0 and 365\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        g_config.retention_days = (*retention_days).valueint;
        set_retention_days(g_config.retention_days);
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated retention_days: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.retention_days,
        );
    }
    let mut lpr_retention_days: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"lpr_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !lpr_retention_days.is_null() {
        if cJSON_IsNumber(lpr_retention_days) == 0
            || (*lpr_retention_days).valuedouble
                != (*lpr_retention_days).valueint as ::core::ffi::c_double
            || (*lpr_retention_days).valueint < 1 as ::core::ffi::c_int
            || (*lpr_retention_days).valueint > 36500 as ::core::ffi::c_int
        {
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"lpr_retention_days must be an integer between 1 and 36500\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut value_4: [::core::ffi::c_char; 32] = [0; 32];
        snprintf(
            &raw mut value_4 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
            (*lpr_retention_days).valueint,
        );
        if db_set_system_setting(
            b"lpr_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut value_4 as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to save LPR retention setting\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated protected LPR retention policy\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut auto_delete_oldest: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"auto_delete_oldest\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !auto_delete_oldest.is_null() && cJSON_IsBool(auto_delete_oldest) != 0 {
        g_config.auto_delete_oldest = cJSON_IsTrue(auto_delete_oldest) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated auto_delete_oldest: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.auto_delete_oldest as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut mp4_faststart: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mp4_faststart\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mp4_faststart.is_null() && cJSON_IsBool(mp4_faststart) != 0 {
        let mut enabled: bool = cJSON_IsTrue(mp4_faststart) != 0;
        config_set_mp4_faststart(&raw mut g_config, enabled);
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mp4_faststart: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if enabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut min_free_pct: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"storage_min_free_pct\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !min_free_pct.is_null() && cJSON_IsNumber(min_free_pct) != 0 {
        let mut v_0: ::core::ffi::c_int = (*min_free_pct).valueint;
        if v_0 < 0 as ::core::ffi::c_int {
            v_0 = 0 as ::core::ffi::c_int;
        }
        if v_0 > 90 as ::core::ffi::c_int {
            v_0 = 90 as ::core::ffi::c_int;
        }
        g_config.storage_min_free_pct = v_0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated storage_min_free_pct: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.storage_min_free_pct,
        );
    }
    let mut warn: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"storage_pressure_warning_pct\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut crit: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"storage_pressure_critical_pct\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut emerg: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"storage_pressure_emergency_pct\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !warn.is_null()
        && !crit.is_null()
        && !emerg.is_null()
        && cJSON_IsNumber(warn) != 0
        && cJSON_IsNumber(crit) != 0
        && cJSON_IsNumber(emerg) != 0
    {
        let mut w: ::core::ffi::c_double = (*warn).valuedouble;
        let mut c: ::core::ffi::c_double = (*crit).valuedouble;
        let mut e: ::core::ffi::c_double = (*emerg).valuedouble;
        if e > 0.0f64 && c > e && w > c && w <= 100.0f64 {
            g_config.storage_pressure_warning_pct = w;
            g_config.storage_pressure_critical_pct = c;
            g_config.storage_pressure_emergency_pct = e;
            settings_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Updated disk pressure thresholds: warning=%.1f critical=%.1f emergency=%.1f\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                w,
                c,
                e,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Ignoring disk pressure thresholds (must satisfy 0 < emergency < critical < warning <= 100): warning=%.1f critical=%.1f emergency=%.1f\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                w,
                c,
                e,
            );
        }
    }
    let mut generate_thumbnails: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"generate_thumbnails\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !generate_thumbnails.is_null() && cJSON_IsBool(generate_thumbnails) != 0 {
        g_config.generate_thumbnails = cJSON_IsTrue(generate_thumbnails) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated generate_thumbnails: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.generate_thumbnails as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut tpr: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"thumbnails_per_recording\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !tpr.is_null() && cJSON_IsNumber(tpr) != 0 {
        let mut n: ::core::ffi::c_int = (*tpr).valueint;
        g_config.thumbnails_per_recording = if n <= 1 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            3 as ::core::ffi::c_int
        };
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated thumbnails_per_recording: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.thumbnails_per_recording,
        );
    }
    let mut models_path: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"models_path\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !models_path.is_null() && cJSON_IsString(models_path) != 0 {
        safe_strcpy(
            &raw mut g_config.models_path as *mut ::core::ffi::c_char,
            (*models_path).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated models_path: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.models_path as *mut ::core::ffi::c_char,
        );
    }
    let mut max_streams_j: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"max_streams\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !max_streams_j.is_null() && cJSON_IsNumber(max_streams_j) != 0 {
        let mut new_max: ::core::ffi::c_int = (*max_streams_j).valueint;
        if new_max < 1 as ::core::ffi::c_int {
            new_max = 1 as ::core::ffi::c_int;
        }
        if new_max > MAX_STREAMS {
            new_max = MAX_STREAMS;
        }
        if new_max != g_config.max_streams {
            g_config.max_streams = new_max;
            settings_changed = r#true != 0;
            restart_required = r#true != 0;
            max_streams_restart_required = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Updated max_streams: %d (restart required)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                new_max,
            );
        }
    }
    let mut log_file: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"log_file\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !log_file.is_null() && cJSON_IsString(log_file) != 0 {
        safe_strcpy(
            &raw mut g_config.log_file as *mut ::core::ffi::c_char,
            (*log_file).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated log_file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.log_file as *mut ::core::ffi::c_char,
        );
    }
    let mut log_level: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"log_level\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !log_level.is_null() && cJSON_IsNumber(log_level) != 0 {
        g_config.log_level = (*log_level).valueint;
        set_log_level(log_level_t(g_config.log_level as ::core::ffi::c_uint));
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated log_level: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.log_level,
        );
    }
    let mut syslog_enabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"syslog_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !syslog_enabled.is_null() && cJSON_IsBool(syslog_enabled) != 0 {
        g_config.syslog_enabled = cJSON_IsTrue(syslog_enabled) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated syslog_enabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.syslog_enabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut syslog_ident: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"syslog_ident\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !syslog_ident.is_null() && cJSON_IsString(syslog_ident) != 0 {
        safe_strcpy(
            &raw mut g_config.syslog_ident as *mut ::core::ffi::c_char,
            (*syslog_ident).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated syslog_ident: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.syslog_ident as *mut ::core::ffi::c_char,
        );
    }
    let mut syslog_facility: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"syslog_facility\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !syslog_facility.is_null() && cJSON_IsString(syslog_facility) != 0 {
        let mut val: *const ::core::ffi::c_char = (*syslog_facility).valuestring;
        if strcmp(val, b"LOG_DAEMON\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            g_config.syslog_facility = LOG_DAEMON;
        } else if strcmp(val, b"LOG_LOCAL0\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            g_config.syslog_facility = LOG_LOCAL0;
        } else if strcmp(val, b"LOG_LOCAL1\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            g_config.syslog_facility = LOG_LOCAL1;
        } else if strcmp(val, b"LOG_LOCAL2\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            g_config.syslog_facility = LOG_LOCAL2;
        } else if strcmp(val, b"LOG_LOCAL3\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            g_config.syslog_facility = LOG_LOCAL3;
        } else if strcmp(val, b"LOG_LOCAL4\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            g_config.syslog_facility = LOG_LOCAL4;
        } else if strcmp(val, b"LOG_LOCAL5\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            g_config.syslog_facility = LOG_LOCAL5;
        } else if strcmp(val, b"LOG_LOCAL6\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            g_config.syslog_facility = LOG_LOCAL6;
        } else if strcmp(val, b"LOG_LOCAL7\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            g_config.syslog_facility = LOG_LOCAL7;
        } else {
            g_config.syslog_facility = LOG_USER;
        }
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated syslog_facility: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.syslog_facility,
        );
    }
    let mut buffer_size: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"buffer_size\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !buffer_size.is_null() && cJSON_IsNumber(buffer_size) != 0 {
        g_config.buffer_size = (*buffer_size).valueint;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated buffer_size: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.buffer_size,
        );
    }
    let mut use_swap: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"use_swap\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !use_swap.is_null() && cJSON_IsBool(use_swap) != 0 {
        g_config.use_swap = cJSON_IsTrue(use_swap) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated use_swap: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.use_swap as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut swap_size: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"swap_size\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !swap_size.is_null() && cJSON_IsNumber(swap_size) != 0 {
        g_config.swap_size = ((*swap_size).valueint as uint64_t)
            .wrapping_mul(1024 as uint64_t)
            .wrapping_mul(1024 as uint64_t);
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated swap_size: %llu bytes\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.swap_size as ::core::ffi::c_ulonglong,
        );
    }
    let mut default_detection_threshold: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"default_detection_threshold\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !default_detection_threshold.is_null() && cJSON_IsNumber(default_detection_threshold) != 0 {
        let mut value_5: ::core::ffi::c_int = (*default_detection_threshold).valueint;
        if value_5 < 0 as ::core::ffi::c_int {
            value_5 = 0 as ::core::ffi::c_int;
        }
        if value_5 > 100 as ::core::ffi::c_int {
            value_5 = 100 as ::core::ffi::c_int;
        }
        g_config.default_detection_threshold = value_5;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated default_detection_threshold: %d%%\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.default_detection_threshold,
        );
    }
    let mut pre_detection_buffer: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"pre_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !pre_detection_buffer.is_null() && cJSON_IsNumber(pre_detection_buffer) != 0 {
        let mut value_6: ::core::ffi::c_int = (*pre_detection_buffer).valueint;
        if value_6 < 0 as ::core::ffi::c_int {
            value_6 = 0 as ::core::ffi::c_int;
        }
        if value_6 > 60 as ::core::ffi::c_int {
            value_6 = 60 as ::core::ffi::c_int;
        }
        g_config.default_pre_detection_buffer = value_6;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated default_pre_detection_buffer: %d seconds\0".as_ptr()
                as *const ::core::ffi::c_char,
            g_config.default_pre_detection_buffer,
        );
    }
    let mut post_detection_buffer: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"post_detection_buffer\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !post_detection_buffer.is_null() && cJSON_IsNumber(post_detection_buffer) != 0 {
        let mut value_7: ::core::ffi::c_int = (*post_detection_buffer).valueint;
        if value_7 < 0 as ::core::ffi::c_int {
            value_7 = 0 as ::core::ffi::c_int;
        }
        if value_7 > 300 as ::core::ffi::c_int {
            value_7 = 300 as ::core::ffi::c_int;
        }
        g_config.default_post_detection_buffer = value_7;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated default_post_detection_buffer: %d seconds\0".as_ptr()
                as *const ::core::ffi::c_char,
            g_config.default_post_detection_buffer,
        );
    }
    let mut default_playback_transport: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"default_playback_transport\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !default_playback_transport.is_null() {
        if cJSON_IsString(default_playback_transport) == 0
            || !playback_transport_is_valid((*default_playback_transport).valuestring)
        {
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"default_playback_transport is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        safe_strcpy(
            &raw mut g_config.default_playback_transport as *mut ::core::ffi::c_char,
            (*default_playback_transport).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated default_playback_transport: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.default_playback_transport as *mut ::core::ffi::c_char,
        );
    }
    let mut detection_grace_period: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"detection_grace_period\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_grace_period.is_null() && cJSON_IsNumber(detection_grace_period) != 0 {
        config_set_detection_grace_period(&raw mut g_config, (*detection_grace_period).valueint);
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated detection_grace_period: %d seconds\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.detection_grace_period,
        );
    }
    let mut api_detection_url: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"api_detection_url\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !api_detection_url.is_null() && cJSON_IsString(api_detection_url) != 0 {
        safe_strcpy(
            &raw mut g_config.api_detection_url as *mut ::core::ffi::c_char,
            (*api_detection_url).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated api_detection_url: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.api_detection_url as *mut ::core::ffi::c_char,
        );
    }
    let mut api_detection_backend: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"api_detection_backend\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !api_detection_backend.is_null() && cJSON_IsString(api_detection_backend) != 0 {
        safe_strcpy(
            &raw mut g_config.api_detection_backend as *mut ::core::ffi::c_char,
            (*api_detection_backend).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated api_detection_backend: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.api_detection_backend as *mut ::core::ffi::c_char,
        );
    }
    let mut api_detection_format: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"api_detection_format\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !api_detection_format.is_null()
        && cJSON_IsString(api_detection_format) != 0
        && config_set_api_detection_format(&raw mut g_config, (*api_detection_format).valuestring)
            as ::core::ffi::c_int
            != 0
    {
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated api_detection_format: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.api_detection_format as *mut ::core::ffi::c_char,
        );
    }
    let mut api_detection_detector_name: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"api_detection_detector_name\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !api_detection_detector_name.is_null() && cJSON_IsString(api_detection_detector_name) != 0 {
        let mut detector_name: *const ::core::ffi::c_char =
            (*api_detection_detector_name).valuestring;
        safe_strcpy(
            &raw mut g_config.api_detection_detector_name as *mut ::core::ffi::c_char,
            if *detector_name.offset(0isize) as ::core::ffi::c_int != 0 {
                detector_name
            } else {
                b"default\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated api_detection_detector_name: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.api_detection_detector_name as *mut ::core::ffi::c_char,
        );
    }
    let mut de_enabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"detection_engine_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !de_enabled.is_null() && cJSON_IsBool(de_enabled) != 0 {
        g_config.detection_engine.enabled = cJSON_IsTrue(de_enabled) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated detection_engine.enabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.detection_engine.enabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut de_threads: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"detection_engine_threads\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !de_threads.is_null() && cJSON_IsNumber(de_threads) != 0 {
        config_set_detection_engine_threads(&raw mut g_config, (*de_threads).valueint);
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated detection_engine.num_threads: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.detection_engine.num_threads,
        );
    }
    let mut de_delegate: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"detection_engine_delegate\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !de_delegate.is_null() && cJSON_IsString(de_delegate) != 0 {
        if config_set_detection_engine_delegate(&raw mut g_config, (*de_delegate).valuestring) {
            settings_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Updated detection_engine.delegate: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut g_config.detection_engine.delegate as *mut ::core::ffi::c_char,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Ignored unknown detection_engine_delegate '%s'\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*de_delegate).valuestring,
            );
        }
    }
    let mut go2rtc_enabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"go2rtc_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_enabled.is_null() && cJSON_IsBool(go2rtc_enabled) != 0 {
        let mut old_go2rtc_enabled: bool = g_config.go2rtc_enabled;
        g_config.go2rtc_enabled = cJSON_IsTrue(go2rtc_enabled) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated go2rtc_enabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.go2rtc_enabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        if old_go2rtc_enabled as ::core::ffi::c_int != g_config.go2rtc_enabled as ::core::ffi::c_int
        {
            go2rtc_config_changed = r#true != 0;
            go2rtc_becoming_enabled = g_config.go2rtc_enabled;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"go2rtc_enabled setting changed from %s to %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if old_go2rtc_enabled as ::core::ffi::c_int != 0 {
                    b"true\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"false\0".as_ptr() as *const ::core::ffi::c_char
                },
                if g_config.go2rtc_enabled as ::core::ffi::c_int != 0 {
                    b"true\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"false\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
    }
    let mut go2rtc_binary_path: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"go2rtc_binary_path\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_binary_path.is_null() && cJSON_IsString(go2rtc_binary_path) != 0 {
        safe_strcpy(
            &raw mut g_config.go2rtc_binary_path as *mut ::core::ffi::c_char,
            (*go2rtc_binary_path).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated go2rtc_binary_path: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.go2rtc_binary_path as *mut ::core::ffi::c_char,
        );
    }
    let mut go2rtc_config_dir: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"go2rtc_config_dir\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_config_dir.is_null() && cJSON_IsString(go2rtc_config_dir) != 0 {
        safe_strcpy(
            &raw mut g_config.go2rtc_config_dir as *mut ::core::ffi::c_char,
            (*go2rtc_config_dir).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated go2rtc_config_dir: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.go2rtc_config_dir as *mut ::core::ffi::c_char,
        );
    }
    let mut go2rtc_api_port: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"go2rtc_api_port\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_api_port.is_null() && cJSON_IsNumber(go2rtc_api_port) != 0 {
        g_config.go2rtc_api_port = (*go2rtc_api_port).valueint;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated go2rtc_api_port: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.go2rtc_api_port,
        );
    }
    let mut go2rtc_rtsp_port: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"go2rtc_rtsp_port\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_rtsp_port.is_null() && cJSON_IsNumber(go2rtc_rtsp_port) != 0 {
        g_config.go2rtc_rtsp_port = (*go2rtc_rtsp_port).valueint;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated go2rtc_rtsp_port: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.go2rtc_rtsp_port,
        );
    }
    let mut go2rtc_webrtc_enabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"go2rtc_webrtc_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_webrtc_enabled.is_null() && cJSON_IsBool(go2rtc_webrtc_enabled) != 0 {
        g_config.go2rtc_webrtc_enabled = cJSON_IsTrue(go2rtc_webrtc_enabled) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated go2rtc_webrtc_enabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.go2rtc_webrtc_enabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut go2rtc_webrtc_listen_port: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"go2rtc_webrtc_listen_port\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_webrtc_listen_port.is_null() && cJSON_IsNumber(go2rtc_webrtc_listen_port) != 0 {
        g_config.go2rtc_webrtc_listen_port = (*go2rtc_webrtc_listen_port).valueint;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated go2rtc_webrtc_listen_port: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.go2rtc_webrtc_listen_port,
        );
    }
    let mut webrtc_connection_timeout_ms: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"webrtc_connection_timeout_ms\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !webrtc_connection_timeout_ms.is_null() && cJSON_IsNumber(webrtc_connection_timeout_ms) != 0
    {
        let mut v_1: ::core::ffi::c_int = (*webrtc_connection_timeout_ms).valueint;
        if v_1 < 1000 as ::core::ffi::c_int {
            v_1 = 1000 as ::core::ffi::c_int;
        }
        g_config.webrtc_connection_timeout_ms = v_1;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated webrtc_connection_timeout_ms: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.webrtc_connection_timeout_ms,
        );
    }
    let mut webrtc_ice_recovery_timeout_ms: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"webrtc_ice_recovery_timeout_ms\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !webrtc_ice_recovery_timeout_ms.is_null()
        && cJSON_IsNumber(webrtc_ice_recovery_timeout_ms) != 0
    {
        let mut v_2: ::core::ffi::c_int = (*webrtc_ice_recovery_timeout_ms).valueint;
        if v_2 < 0 as ::core::ffi::c_int {
            v_2 = 0 as ::core::ffi::c_int;
        }
        g_config.webrtc_ice_recovery_timeout_ms = v_2;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated webrtc_ice_recovery_timeout_ms: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.webrtc_ice_recovery_timeout_ms,
        );
    }
    let mut go2rtc_stun_enabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"go2rtc_stun_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_stun_enabled.is_null() && cJSON_IsBool(go2rtc_stun_enabled) != 0 {
        g_config.go2rtc_stun_enabled = cJSON_IsTrue(go2rtc_stun_enabled) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated go2rtc_stun_enabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.go2rtc_stun_enabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut go2rtc_stun_server: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"go2rtc_stun_server\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_stun_server.is_null() && cJSON_IsString(go2rtc_stun_server) != 0 {
        safe_strcpy(
            &raw mut g_config.go2rtc_stun_server as *mut ::core::ffi::c_char,
            (*go2rtc_stun_server).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated go2rtc_stun_server: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.go2rtc_stun_server as *mut ::core::ffi::c_char,
        );
    }
    let mut go2rtc_external_ip: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"go2rtc_external_ip\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_external_ip.is_null() && cJSON_IsString(go2rtc_external_ip) != 0 {
        let mut candidates: [[::core::ffi::c_char; 128]; 8] = [[0; 128]; 8];
        if strlen((*go2rtc_external_ip).valuestring)
            >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
            || go2rtc_parse_external_candidates(
                (*go2rtc_external_ip).valuestring,
                8555 as ::core::ffi::c_int,
                &raw mut candidates as *mut [::core::ffi::c_char; 128],
                GO2RTC_MAX_EXTERNAL_CANDIDATES,
            ) < 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Rejected invalid go2rtc_external_ip setting\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid go2rtc_external_ip: expected up to 8 comma-separated IPs or hostnames (optionally :port), '*' for auto-detect, or stun[:port]; configure STUN servers in the STUN/ICE server settings\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        safe_strcpy(
            &raw mut g_config.go2rtc_external_ip as *mut ::core::ffi::c_char,
            (*go2rtc_external_ip).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated go2rtc_external_ip: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.go2rtc_external_ip as *mut ::core::ffi::c_char,
        );
    }
    let mut go2rtc_ice_servers: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"go2rtc_ice_servers\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_ice_servers.is_null() && cJSON_IsString(go2rtc_ice_servers) != 0 {
        safe_strcpy(
            &raw mut g_config.go2rtc_ice_servers as *mut ::core::ffi::c_char,
            (*go2rtc_ice_servers).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated go2rtc_ice_servers: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.go2rtc_ice_servers as *mut ::core::ffi::c_char,
        );
    }
    let mut go2rtc_force_native_hls: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"go2rtc_force_native_hls\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_force_native_hls.is_null() && cJSON_IsBool(go2rtc_force_native_hls) != 0 {
        g_config.go2rtc_force_native_hls = cJSON_IsTrue(go2rtc_force_native_hls) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated go2rtc_force_native_hls: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.go2rtc_force_native_hls as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut go2rtc_config_override: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"go2rtc_config_override\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !go2rtc_config_override.is_null() && cJSON_IsString(go2rtc_config_override) != 0 {
        let mut override_len: size_t = strlen((*go2rtc_config_override).valuestring);
        if override_len >= 65536 as ::core::ffi::c_int as size_t {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"go2rtc_config_override too long (%zu bytes, max 65535)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                override_len,
            );
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                413 as ::core::ffi::c_int,
                b"go2rtc_config_override exceeds 65535 byte limit\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut vr: yaml_validation_result_t = yaml_validation_result_t {
            valid: 0,
            err_message: [0; 256],
            err_line: 0,
            err_column: 0,
            duplicate_keys: 0,
            non_mapping_root: 0,
            parse_error: 0,
            warnings: [[0; 192]; 16],
            warning_count: 0,
        };
        yaml_validate_go2rtc_override(
            (*go2rtc_config_override).valuestring,
            override_len,
            &raw mut vr,
        );
        if vr.valid == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Rejecting go2rtc_config_override: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut vr.err_message as *mut ::core::ffi::c_char,
            );
            let mut result: *mut cJSON = go2rtc_validation_result_to_json(&raw mut vr);
            cJSON_Delete(settings);
            if result.is_null() {
                http_response_set_json_error(
                    res,
                    500 as ::core::ffi::c_int,
                    b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            let mut result_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(result);
            cJSON_Delete(result);
            if result_str.is_null() {
                http_response_set_json_error(
                    res,
                    500 as ::core::ffi::c_int,
                    b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            http_response_set_json(res, 400 as ::core::ffi::c_int, result_str);
            free(result_str as *mut ::core::ffi::c_void);
            return;
        }
        if db_set_system_setting(
            b"go2rtc_config_override\0".as_ptr() as *const ::core::ffi::c_char,
            (*go2rtc_config_override).valuestring,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to save go2rtc_config_override to system_settings\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Updated go2rtc_config_override (%zu bytes)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                override_len,
            );
            go2rtc_config_changed = r#true != 0;
            go2rtc_becoming_enabled = g_config.go2rtc_enabled;
            go2rtc_process_clear_override_quarantine();
        }
    }
    let mut mqtt_enabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mqtt_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mqtt_enabled.is_null() && cJSON_IsBool(mqtt_enabled) != 0 {
        g_config.mqtt_enabled = cJSON_IsTrue(mqtt_enabled) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mqtt_enabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.mqtt_enabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut mqtt_broker_host: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mqtt_broker_host\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mqtt_broker_host.is_null() && cJSON_IsString(mqtt_broker_host) != 0 {
        safe_strcpy(
            &raw mut g_config.mqtt_broker_host as *mut ::core::ffi::c_char,
            (*mqtt_broker_host).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mqtt_broker_host: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.mqtt_broker_host as *mut ::core::ffi::c_char,
        );
    }
    let mut mqtt_broker_port: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mqtt_broker_port\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mqtt_broker_port.is_null() && cJSON_IsNumber(mqtt_broker_port) != 0 {
        g_config.mqtt_broker_port = (*mqtt_broker_port).valueint;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mqtt_broker_port: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.mqtt_broker_port,
        );
    }
    let mut mqtt_username: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mqtt_username\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mqtt_username.is_null() && cJSON_IsString(mqtt_username) != 0 {
        safe_strcpy(
            &raw mut g_config.mqtt_username as *mut ::core::ffi::c_char,
            (*mqtt_username).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mqtt_username: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.mqtt_username as *mut ::core::ffi::c_char,
        );
    }
    let mut mqtt_password: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mqtt_password\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mqtt_password.is_null()
        && cJSON_IsString(mqtt_password) != 0
        && strcmp(
            (*mqtt_password).valuestring,
            b"********\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut g_config.mqtt_password as *mut ::core::ffi::c_char,
            (*mqtt_password).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mqtt_password\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut mqtt_client_id: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mqtt_client_id\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mqtt_client_id.is_null() && cJSON_IsString(mqtt_client_id) != 0 {
        safe_strcpy(
            &raw mut g_config.mqtt_client_id as *mut ::core::ffi::c_char,
            (*mqtt_client_id).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mqtt_client_id: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.mqtt_client_id as *mut ::core::ffi::c_char,
        );
    }
    let mut mqtt_topic_prefix: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mqtt_topic_prefix\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mqtt_topic_prefix.is_null() && cJSON_IsString(mqtt_topic_prefix) != 0 {
        safe_strcpy(
            &raw mut g_config.mqtt_topic_prefix as *mut ::core::ffi::c_char,
            (*mqtt_topic_prefix).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mqtt_topic_prefix: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.mqtt_topic_prefix as *mut ::core::ffi::c_char,
        );
    }
    let mut mqtt_tls_enabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mqtt_tls_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mqtt_tls_enabled.is_null() && cJSON_IsBool(mqtt_tls_enabled) != 0 {
        g_config.mqtt_tls_enabled = cJSON_IsTrue(mqtt_tls_enabled) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mqtt_tls_enabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.mqtt_tls_enabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut mqtt_keepalive: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mqtt_keepalive\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mqtt_keepalive.is_null() && cJSON_IsNumber(mqtt_keepalive) != 0 {
        g_config.mqtt_keepalive = (*mqtt_keepalive).valueint;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mqtt_keepalive: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.mqtt_keepalive,
        );
    }
    let mut mqtt_qos: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mqtt_qos\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mqtt_qos.is_null() && cJSON_IsNumber(mqtt_qos) != 0 {
        let mut qos: ::core::ffi::c_int = (*mqtt_qos).valueint;
        if qos < 0 as ::core::ffi::c_int {
            qos = 0 as ::core::ffi::c_int;
        }
        if qos > 2 as ::core::ffi::c_int {
            qos = 2 as ::core::ffi::c_int;
        }
        g_config.mqtt_qos = qos;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mqtt_qos: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.mqtt_qos,
        );
    }
    let mut mqtt_retain: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mqtt_retain\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mqtt_retain.is_null() && cJSON_IsBool(mqtt_retain) != 0 {
        g_config.mqtt_retain = cJSON_IsTrue(mqtt_retain) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mqtt_retain: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.mqtt_retain as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut mqtt_ha_discovery: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mqtt_ha_discovery\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mqtt_ha_discovery.is_null() && cJSON_IsBool(mqtt_ha_discovery) != 0 {
        g_config.mqtt_ha_discovery = cJSON_IsTrue(mqtt_ha_discovery) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mqtt_ha_discovery: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.mqtt_ha_discovery as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut mqtt_ha_discovery_prefix: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mqtt_ha_discovery_prefix\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mqtt_ha_discovery_prefix.is_null() && cJSON_IsString(mqtt_ha_discovery_prefix) != 0 {
        safe_strcpy(
            &raw mut g_config.mqtt_ha_discovery_prefix as *mut ::core::ffi::c_char,
            (*mqtt_ha_discovery_prefix).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mqtt_ha_discovery_prefix: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.mqtt_ha_discovery_prefix as *mut ::core::ffi::c_char,
        );
    }
    let mut mqtt_ha_snapshot_interval: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"mqtt_ha_snapshot_interval\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !mqtt_ha_snapshot_interval.is_null() && cJSON_IsNumber(mqtt_ha_snapshot_interval) != 0 {
        let mut interval: ::core::ffi::c_int = (*mqtt_ha_snapshot_interval).valueint;
        if interval < 0 as ::core::ffi::c_int {
            interval = 0 as ::core::ffi::c_int;
        }
        if interval > 300 as ::core::ffi::c_int {
            interval = 300 as ::core::ffi::c_int;
        }
        g_config.mqtt_ha_snapshot_interval = interval;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated mqtt_ha_snapshot_interval: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.mqtt_ha_snapshot_interval,
        );
    }
    if old_mqtt_enabled as ::core::ffi::c_int != g_config.mqtt_enabled as ::core::ffi::c_int
        || strcmp(
            &raw mut old_mqtt_broker_host as *mut ::core::ffi::c_char,
            &raw mut g_config.mqtt_broker_host as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        || old_mqtt_broker_port != g_config.mqtt_broker_port
        || strcmp(
            &raw mut old_mqtt_username as *mut ::core::ffi::c_char,
            &raw mut g_config.mqtt_username as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        || strcmp(
            &raw mut old_mqtt_password as *mut ::core::ffi::c_char,
            &raw mut g_config.mqtt_password as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        || strcmp(
            &raw mut old_mqtt_client_id as *mut ::core::ffi::c_char,
            &raw mut g_config.mqtt_client_id as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        || strcmp(
            &raw mut old_mqtt_topic_prefix as *mut ::core::ffi::c_char,
            &raw mut g_config.mqtt_topic_prefix as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        || old_mqtt_tls_enabled as ::core::ffi::c_int
            != g_config.mqtt_tls_enabled as ::core::ffi::c_int
        || old_mqtt_keepalive != g_config.mqtt_keepalive
        || old_mqtt_qos != g_config.mqtt_qos
        || old_mqtt_retain as ::core::ffi::c_int != g_config.mqtt_retain as ::core::ffi::c_int
        || old_mqtt_ha_discovery as ::core::ffi::c_int
            != g_config.mqtt_ha_discovery as ::core::ffi::c_int
        || strcmp(
            &raw mut old_mqtt_ha_discovery_prefix as *mut ::core::ffi::c_char,
            &raw mut g_config.mqtt_ha_discovery_prefix as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        || old_mqtt_ha_snapshot_interval != g_config.mqtt_ha_snapshot_interval
    {
        mqtt_config_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"MQTT settings changed, will reinitialize MQTT client\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let mut turn_enabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"turn_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !turn_enabled.is_null() && cJSON_IsBool(turn_enabled) != 0 {
        let mut old_turn_enabled: bool = g_config.turn_enabled;
        g_config.turn_enabled = cJSON_IsTrue(turn_enabled) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated turn_enabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.turn_enabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        if old_turn_enabled as ::core::ffi::c_int != g_config.turn_enabled as ::core::ffi::c_int
            && g_config.go2rtc_enabled as ::core::ffi::c_int != 0
        {
            go2rtc_config_changed = r#true != 0;
            go2rtc_becoming_enabled = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"TURN enabled setting changed, will restart go2rtc\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    let mut turn_server_url: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"turn_server_url\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !turn_server_url.is_null() && cJSON_IsString(turn_server_url) != 0 {
        let mut old_turn_server_url: [::core::ffi::c_char; 256] = [0; 256];
        safe_strcpy(
            &raw mut old_turn_server_url as *mut ::core::ffi::c_char,
            &raw mut g_config.turn_server_url as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut g_config.turn_server_url as *mut ::core::ffi::c_char,
            (*turn_server_url).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated turn_server_url: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.turn_server_url as *mut ::core::ffi::c_char,
        );
        if strcmp(
            &raw mut old_turn_server_url as *mut ::core::ffi::c_char,
            &raw mut g_config.turn_server_url as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            && g_config.go2rtc_enabled as ::core::ffi::c_int != 0
            && g_config.turn_enabled as ::core::ffi::c_int != 0
        {
            go2rtc_config_changed = r#true != 0;
            go2rtc_becoming_enabled = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"TURN server URL changed, will restart go2rtc\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    let mut turn_username: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"turn_username\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !turn_username.is_null() && cJSON_IsString(turn_username) != 0 {
        let mut old_turn_username: [::core::ffi::c_char; 128] = [0; 128];
        safe_strcpy(
            &raw mut old_turn_username as *mut ::core::ffi::c_char,
            &raw mut g_config.turn_username as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut g_config.turn_username as *mut ::core::ffi::c_char,
            (*turn_username).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated turn_username: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.turn_username as *mut ::core::ffi::c_char,
        );
        if strcmp(
            &raw mut old_turn_username as *mut ::core::ffi::c_char,
            &raw mut g_config.turn_username as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            && g_config.go2rtc_enabled as ::core::ffi::c_int != 0
            && g_config.turn_enabled as ::core::ffi::c_int != 0
        {
            go2rtc_config_changed = r#true != 0;
            go2rtc_becoming_enabled = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"TURN username changed, will restart go2rtc\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    let mut turn_password: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"turn_password\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !turn_password.is_null() && cJSON_IsString(turn_password) != 0 {
        if strcmp(
            (*turn_password).valuestring,
            b"********\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            let mut old_turn_password: [::core::ffi::c_char; 128] = [0; 128];
            safe_strcpy(
                &raw mut old_turn_password as *mut ::core::ffi::c_char,
                &raw mut g_config.turn_password as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut g_config.turn_password as *mut ::core::ffi::c_char,
                (*turn_password).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
            settings_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Updated turn_password\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if strcmp(
                &raw mut old_turn_password as *mut ::core::ffi::c_char,
                &raw mut g_config.turn_password as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
                && g_config.go2rtc_enabled as ::core::ffi::c_int != 0
                && g_config.turn_enabled as ::core::ffi::c_int != 0
            {
                go2rtc_config_changed = r#true != 0;
                go2rtc_becoming_enabled = r#true != 0;
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"TURN password changed, will restart go2rtc\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        }
    }
    let mut onvif_discovery_enabled: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"onvif_discovery_enabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !onvif_discovery_enabled.is_null() && cJSON_IsBool(onvif_discovery_enabled) != 0 {
        g_config.onvif_discovery_enabled = cJSON_IsTrue(onvif_discovery_enabled) != 0;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated onvif_discovery_enabled: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.onvif_discovery_enabled as ::core::ffi::c_int != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut onvif_discovery_interval: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"onvif_discovery_interval\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !onvif_discovery_interval.is_null() && cJSON_IsNumber(onvif_discovery_interval) != 0 {
        let mut value_8: ::core::ffi::c_int = (*onvif_discovery_interval).valueint;
        if value_8 < 30 as ::core::ffi::c_int {
            value_8 = 30 as ::core::ffi::c_int;
        }
        if value_8 > 3600 as ::core::ffi::c_int {
            value_8 = 3600 as ::core::ffi::c_int;
        }
        g_config.onvif_discovery_interval = value_8;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated onvif_discovery_interval: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.onvif_discovery_interval,
        );
    }
    let mut onvif_discovery_network: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"onvif_discovery_network\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !onvif_discovery_network.is_null() && cJSON_IsString(onvif_discovery_network) != 0 {
        safe_strcpy(
            &raw mut g_config.onvif_discovery_network as *mut ::core::ffi::c_char,
            (*onvif_discovery_network).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated onvif_discovery_network: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.onvif_discovery_network as *mut ::core::ffi::c_char,
        );
    }
    let mut db_backup_interval_minutes: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"db_backup_interval_minutes\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !db_backup_interval_minutes.is_null() && cJSON_IsNumber(db_backup_interval_minutes) != 0 {
        let mut value_9: ::core::ffi::c_int = (*db_backup_interval_minutes).valueint;
        if value_9 < 0 as ::core::ffi::c_int {
            value_9 = 0 as ::core::ffi::c_int;
        }
        g_config.db_backup_interval_minutes = value_9;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated db_backup_interval_minutes: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.db_backup_interval_minutes,
        );
    }
    let mut db_backup_retention_count: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"db_backup_retention_count\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !db_backup_retention_count.is_null() && cJSON_IsNumber(db_backup_retention_count) != 0 {
        let mut value_10: ::core::ffi::c_int = (*db_backup_retention_count).valueint;
        if value_10 < 0 as ::core::ffi::c_int {
            value_10 = 0 as ::core::ffi::c_int;
        }
        g_config.db_backup_retention_count = value_10;
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated db_backup_retention_count: %d\0".as_ptr() as *const ::core::ffi::c_char,
            g_config.db_backup_retention_count,
        );
    }
    let mut db_backup_verify: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"db_backup_verify\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !db_backup_verify.is_null() && cJSON_IsString(db_backup_verify) != 0 {
        let mut mode: *const ::core::ffi::c_char = (*db_backup_verify).valuestring;
        let mut value_11: ::core::ffi::c_int = 0;
        if strcasecmp(mode, b"full\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            value_11 = DB_BACKUP_VERIFY_FULL;
        } else if strcasecmp(mode, b"quick\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            value_11 = DB_BACKUP_VERIFY_QUICK;
        } else if strcasecmp(mode, b"off\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            value_11 = DB_BACKUP_VERIFY_OFF;
        } else {
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid db_backup_verify: expected \"full\", \"quick\" or \"off\"\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        if value_11 != g_config.db_backup_verify {
            g_config.db_backup_verify = value_11;
            settings_changed = r#true != 0;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Updated db_backup_verify: %s\0".as_ptr() as *const ::core::ffi::c_char,
                mode,
            );
        }
    }
    let mut db_post_backup_script: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"db_post_backup_script\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !db_post_backup_script.is_null() && cJSON_IsString(db_post_backup_script) != 0 {
        let mut script_path: *const ::core::ffi::c_char = (*db_post_backup_script).valuestring;
        if *script_path.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && !is_safe_storage_path(script_path)
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Rejected unsafe db_post_backup_script: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                script_path,
            );
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid db_post_backup_script: must be an absolute path without shell metacharacters\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        safe_strcpy(
            &raw mut g_config.db_post_backup_script as *mut ::core::ffi::c_char,
            script_path,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Updated db_post_backup_script: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if g_config.db_post_backup_script[0usize] as ::core::ffi::c_int != 0 {
                &raw mut g_config.db_post_backup_script as *mut ::core::ffi::c_char
            } else {
                b"(disabled)\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    let mut db_path: *mut cJSON = cJSON_GetObjectItem(
        settings,
        b"db_path\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !db_path.is_null()
        && cJSON_IsString(db_path) != 0
        && strcmp(
            &raw mut g_config.db_path as *mut ::core::ffi::c_char,
            (*db_path).valuestring,
        ) != 0 as ::core::ffi::c_int
    {
        let mut old_db_path: [::core::ffi::c_char; 512] = [0; 512];
        safe_strcpy(
            &raw mut old_db_path as *mut ::core::ffi::c_char,
            &raw mut g_config.db_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut g_config.db_path as *mut ::core::ffi::c_char,
            (*db_path).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        settings_changed = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Database path changed from %s to %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut old_db_path as *mut ::core::ffi::c_char,
            &raw mut g_config.db_path as *mut ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Stopping all HLS streams before changing database path...\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        let mut active_streams: *mut [::core::ffi::c_char; 256] =
            calloc(g_config.max_streams as size_t, MAX_STREAM_NAME as size_t)
                as *mut [::core::ffi::c_char; 256];
        if active_streams.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"handle_post_settings: out of memory for active_streams\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Internal error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut active_stream_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Scanning for active streams...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < g_config.max_streams {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Checking stream slot %d: name='%s', enabled=%d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                i,
                &raw mut (*g_config.streams.offset(i as isize)).name as *mut ::core::ffi::c_char,
                (*g_config.streams.offset(i as isize)).enabled as ::core::ffi::c_int,
            );
            if (*g_config.streams.offset(i as isize)).name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Explicitly stopping HLS streaming for stream: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*g_config.streams.offset(i as isize)).name
                        as *mut ::core::ffi::c_char,
                );
                stream_stop_hls(
                    &raw mut (*g_config.streams.offset(i as isize)).name
                        as *mut ::core::ffi::c_char,
                );
                if (*g_config.streams.offset(i as isize)).enabled {
                    safe_strcpy(
                        &raw mut *active_streams.offset(active_stream_count as isize)
                            as *mut ::core::ffi::c_char,
                        &raw mut (*g_config.streams.offset(i as isize)).name
                            as *mut ::core::ffi::c_char,
                        MAX_STREAM_NAME as size_t,
                        0 as size_t,
                    );
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Added active stream %d: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        active_stream_count,
                        &raw mut *active_streams.offset(active_stream_count as isize)
                            as *mut ::core::ffi::c_char,
                    );
                    active_stream_count += 1;
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Stopping stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*g_config.streams.offset(i as isize)).name
                            as *mut ::core::ffi::c_char,
                    );
                    let mut stream: stream_handle_t = get_stream_by_name(
                        &raw mut (*g_config.streams.offset(i as isize)).name
                            as *mut ::core::ffi::c_char,
                    );
                    if !stream.is_null() {
                        if stop_stream(stream) == 0 as ::core::ffi::c_int {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT.as_ptr(),
                                _log_stream_name,
                                b"Stream stopped: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut (*g_config.streams.offset(i as isize)).name
                                    as *mut ::core::ffi::c_char,
                            );
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_WARN,
                                LOG_COMPONENT.as_ptr(),
                                _log_stream_name,
                                b"Failed to stop stream: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut (*g_config.streams.offset(i as isize)).name
                                    as *mut ::core::ffi::c_char,
                            );
                        }
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Failed to get stream handle for: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*g_config.streams.offset(i as isize)).name
                                as *mut ::core::ffi::c_char,
                        );
                    }
                }
            }
            i += 1;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Found %d active streams\0".as_ptr() as *const ::core::ffi::c_char,
            active_stream_count,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Waiting for streams to fully stop...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sleep(2 as ::core::ffi::c_uint);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Shutting down stream manager to change database path...\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        shutdown_stream_manager();
        audit_log_flush_summaries(r#false != 0);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Shutting down database...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        shutdown_database();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Initializing database with new path: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.db_path as *mut ::core::ffi::c_char,
        );
        if init_database(&raw mut g_config.db_path as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to initialize database with new path, reverting to old path\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            safe_strcpy(
                &raw mut g_config.db_path as *mut ::core::ffi::c_char,
                &raw mut old_db_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
            if init_database(&raw mut g_config.db_path as *mut ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to reinitialize database with old path, database may be unavailable\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Successfully reinitialized database with old path\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                audit_log_decision_modes_init();
            }
            if init_stream_manager(g_config.max_streams) != 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to reinitialize stream manager\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Successfully reinitialized stream manager\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            free(active_streams as *mut ::core::ffi::c_void);
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to initialize database with new path\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        audit_log_decision_modes_init();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Reinitializing stream manager...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if init_stream_manager(g_config.max_streams) != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to reinitialize stream manager\0".as_ptr() as *const ::core::ffi::c_char,
            );
            free(active_streams as *mut ::core::ffi::c_void);
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to reinitialize stream manager\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Restarting streams from configuration...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Active stream count: %d\0".as_ptr() as *const ::core::ffi::c_char,
            active_stream_count,
        );
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < active_stream_count {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Processing active stream %d: %s\0".as_ptr() as *const ::core::ffi::c_char,
                i_0,
                &raw mut *active_streams.offset(i_0 as isize) as *mut ::core::ffi::c_char,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Restarting stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut *active_streams.offset(i_0 as isize) as *mut ::core::ffi::c_char,
            );
            let mut stream_0: stream_handle_t = get_stream_by_name(
                &raw mut *active_streams.offset(i_0 as isize) as *mut ::core::ffi::c_char,
            );
            if !stream_0.is_null() {
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
                if get_stream_config(stream_0, &raw mut config) != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Failed to get stream configuration for %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut *active_streams.offset(i_0 as isize) as *mut ::core::ffi::c_char,
                    );
                } else if start_stream(stream_0) == 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Stream restarted: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut *active_streams.offset(i_0 as isize) as *mut ::core::ffi::c_char,
                    );
                    if config.streaming_enabled {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Starting HLS streaming for stream: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut *active_streams.offset(i_0 as isize)
                                as *mut ::core::ffi::c_char,
                        );
                        let mut hls_started: bool = r#false != 0;
                        let mut retry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while retry < 3 as ::core::ffi::c_int && !hls_started {
                            if retry > 0 as ::core::ffi::c_int {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_INFO,
                                    LOG_COMPONENT.as_ptr(),
                                    _log_stream_name,
                                    b"Retry %d starting HLS streaming for stream: %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    retry,
                                    &raw mut *active_streams.offset(i_0 as isize)
                                        as *mut ::core::ffi::c_char,
                                );
                                usleep(500000 as ::core::ffi::c_int as __useconds_t);
                            }
                            if stream_start_hls(&raw mut *active_streams.offset(i_0 as isize)
                                as *mut ::core::ffi::c_char)
                                == 0 as ::core::ffi::c_int
                            {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_INFO,
                                    LOG_COMPONENT.as_ptr(),
                                    _log_stream_name,
                                    b"HLS streaming started for stream: %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut *active_streams.offset(i_0 as isize)
                                        as *mut ::core::ffi::c_char,
                                );
                                hls_started = r#true != 0;
                            } else {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_WARN,
                                    LOG_COMPONENT.as_ptr(),
                                    _log_stream_name,
                                    b"Failed to start HLS streaming for stream: %s (attempt %d/3)\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut *active_streams.offset(i_0 as isize)
                                        as *mut ::core::ffi::c_char,
                                    retry + 1 as ::core::ffi::c_int,
                                );
                            }
                            retry += 1;
                        }
                        if !hls_started {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_ERROR,
                                LOG_COMPONENT.as_ptr(),
                                _log_stream_name,
                                b"Failed to start HLS streaming for stream: %s after multiple attempts\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                &raw mut *active_streams.offset(i_0 as isize)
                                    as *mut ::core::ffi::c_char,
                            );
                        }
                    }
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Failed to restart stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut *active_streams.offset(i_0 as isize) as *mut ::core::ffi::c_char,
                    );
                }
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to get stream handle for: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut *active_streams.offset(i_0 as isize) as *mut ::core::ffi::c_char,
                );
                let mut config_0: *const stream_config_t = ::core::ptr::null::<stream_config_t>();
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j < g_config.max_streams {
                    if strcmp(
                        &raw mut (*g_config.streams.offset(j as isize)).name
                            as *mut ::core::ffi::c_char,
                        &raw mut *active_streams.offset(i_0 as isize) as *mut ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        config_0 = g_config.streams.offset(j as isize);
                        break;
                    } else {
                        j += 1;
                    }
                }
                if !config_0.is_null() {
                    stream_0 = add_stream(config_0);
                    if !stream_0.is_null() {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Added stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut *active_streams.offset(i_0 as isize)
                                as *mut ::core::ffi::c_char,
                        );
                        if start_stream(stream_0) == 0 as ::core::ffi::c_int {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT.as_ptr(),
                                _log_stream_name,
                                b"Stream started: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut *active_streams.offset(i_0 as isize)
                                    as *mut ::core::ffi::c_char,
                            );
                            if (*config_0).streaming_enabled {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_INFO,
                                    LOG_COMPONENT.as_ptr(),
                                    _log_stream_name,
                                    b"Starting HLS streaming for stream: %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut *active_streams.offset(i_0 as isize)
                                        as *mut ::core::ffi::c_char,
                                );
                                let mut hls_started_0: bool = r#false != 0;
                                let mut retry_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while retry_0 < 3 as ::core::ffi::c_int && !hls_started_0 {
                                    if retry_0 > 0 as ::core::ffi::c_int {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_INFO,
                                            LOG_COMPONENT.as_ptr(),
                                            _log_stream_name,
                                            b"Retry %d starting HLS streaming for stream: %s\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            retry_0,
                                            &raw mut *active_streams.offset(i_0 as isize)
                                                as *mut ::core::ffi::c_char,
                                        );
                                        usleep(500000 as ::core::ffi::c_int as __useconds_t);
                                    }
                                    if stream_start_hls(
                                        &raw mut *active_streams.offset(i_0 as isize)
                                            as *mut ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_INFO,
                                            LOG_COMPONENT.as_ptr(),
                                            _log_stream_name,
                                            b"HLS streaming started for stream: %s\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            &raw mut *active_streams.offset(i_0 as isize)
                                                as *mut ::core::ffi::c_char,
                                        );
                                        hls_started_0 = r#true != 0;
                                    } else {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_WARN,
                                            LOG_COMPONENT.as_ptr(),
                                            _log_stream_name,
                                            b"Failed to start HLS streaming for stream: %s (attempt %d/3)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            &raw mut *active_streams.offset(i_0 as isize)
                                                as *mut ::core::ffi::c_char,
                                            retry_0 + 1 as ::core::ffi::c_int,
                                        );
                                    }
                                    retry_0 += 1;
                                }
                                if !hls_started_0 {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_ERROR,
                                        LOG_COMPONENT.as_ptr(),
                                        _log_stream_name,
                                        b"Failed to start HLS streaming for stream: %s after multiple attempts\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        &raw mut *active_streams.offset(i_0 as isize)
                                            as *mut ::core::ffi::c_char,
                                    );
                                }
                            }
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_WARN,
                                LOG_COMPONENT.as_ptr(),
                                _log_stream_name,
                                b"Failed to start stream: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut *active_streams.offset(i_0 as isize)
                                    as *mut ::core::ffi::c_char,
                            );
                        }
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Failed to add stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut *active_streams.offset(i_0 as isize)
                                as *mut ::core::ffi::c_char,
                        );
                    }
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Failed to find configuration for stream: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut *active_streams.offset(i_0 as isize) as *mut ::core::ffi::c_char,
                    );
                }
            }
            i_0 += 1;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Waiting for streams to fully start...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        sleep(2 as ::core::ffi::c_uint);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Force restarting all HLS streams to ensure they're properly started...\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < active_stream_count {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Force restarting HLS for active stream %d: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                i_1,
                &raw mut *active_streams.offset(i_1 as isize) as *mut ::core::ffi::c_char,
            );
            let mut stream_1: stream_handle_t = get_stream_by_name(
                &raw mut *active_streams.offset(i_1 as isize) as *mut ::core::ffi::c_char,
            );
            if !stream_1.is_null() {
                let mut config_1: stream_config_t = stream_config_t {
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
                if get_stream_config(stream_1, &raw mut config_1) != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Failed to get stream configuration for %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut *active_streams.offset(i_1 as isize) as *mut ::core::ffi::c_char,
                    );
                } else if config_1.streaming_enabled {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Force restarting HLS streaming for stream: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut *active_streams.offset(i_1 as isize) as *mut ::core::ffi::c_char,
                    );
                    if stream_stop_hls(
                        &raw mut *active_streams.offset(i_1 as isize) as *mut ::core::ffi::c_char
                    ) != 0 as ::core::ffi::c_int
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Failed to stop HLS stream for restart: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut *active_streams.offset(i_1 as isize)
                                as *mut ::core::ffi::c_char,
                        );
                    }
                    usleep(500000 as ::core::ffi::c_int as __useconds_t);
                    if stream_start_hls(
                        &raw mut *active_streams.offset(i_1 as isize) as *mut ::core::ffi::c_char
                    ) == 0 as ::core::ffi::c_int
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"HLS streaming force restarted for stream: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut *active_streams.offset(i_1 as isize)
                                as *mut ::core::ffi::c_char,
                        );
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Failed to force restart HLS streaming for stream: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut *active_streams.offset(i_1 as isize)
                                as *mut ::core::ffi::c_char,
                        );
                    }
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Streaming not enabled for stream: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut *active_streams.offset(i_1 as isize) as *mut ::core::ffi::c_char,
                    );
                }
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to get stream handle for force restart: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut *active_streams.offset(i_1 as isize) as *mut ::core::ffi::c_char,
                );
            }
            i_1 += 1;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Starting all streams from the database after changing database path...\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        let mut db_streams: *mut stream_config_t = calloc(
            g_config.max_streams as size_t,
            ::core::mem::size_of::<stream_config_t>(),
        ) as *mut stream_config_t;
        if db_streams.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"handle_post_settings: out of memory for db_streams\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            free(active_streams as *mut ::core::ffi::c_void);
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Internal error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut count: ::core::ffi::c_int =
            get_all_stream_configs(db_streams, g_config.max_streams);
        if count > 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Found %d streams in the database\0".as_ptr() as *const ::core::ffi::c_char,
                count,
            );
            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_2 < count {
                if (*db_streams.offset(i_2 as isize)).name[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                    && (*db_streams.offset(i_2 as isize)).enabled as ::core::ffi::c_int != 0
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Starting stream from database: %s (streaming_enabled=%d)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*db_streams.offset(i_2 as isize)).name
                            as *mut ::core::ffi::c_char,
                        (*db_streams.offset(i_2 as isize)).streaming_enabled as ::core::ffi::c_int,
                    );
                    let mut stream_2: stream_handle_t = add_stream(db_streams.offset(i_2 as isize));
                    if !stream_2.is_null() {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Added stream from database: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*db_streams.offset(i_2 as isize)).name
                                as *mut ::core::ffi::c_char,
                        );
                        if start_stream(stream_2) == 0 as ::core::ffi::c_int {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT.as_ptr(),
                                _log_stream_name,
                                b"Started stream from database: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut (*db_streams.offset(i_2 as isize)).name
                                    as *mut ::core::ffi::c_char,
                            );
                            if (*db_streams.offset(i_2 as isize)).streaming_enabled {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_INFO,
                                    LOG_COMPONENT.as_ptr(),
                                    _log_stream_name,
                                    b"Starting HLS streaming for database stream: %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut (*db_streams.offset(i_2 as isize)).name
                                        as *mut ::core::ffi::c_char,
                                );
                                let mut hls_started_1: bool = r#false != 0;
                                let mut retry_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while retry_1 < 3 as ::core::ffi::c_int && !hls_started_1 {
                                    if retry_1 > 0 as ::core::ffi::c_int {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_INFO,
                                            LOG_COMPONENT.as_ptr(),
                                            _log_stream_name,
                                            b"Retry %d starting HLS streaming for database stream: %s\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            retry_1,
                                            &raw mut (*db_streams.offset(i_2 as isize)).name
                                                as *mut ::core::ffi::c_char,
                                        );
                                        usleep(500000 as ::core::ffi::c_int as __useconds_t);
                                    }
                                    if stream_start_hls(
                                        &raw mut (*db_streams.offset(i_2 as isize)).name
                                            as *mut ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_INFO,
                                            LOG_COMPONENT.as_ptr(),
                                            _log_stream_name,
                                            b"HLS streaming started for database stream: %s\0"
                                                .as_ptr()
                                                as *const ::core::ffi::c_char,
                                            &raw mut (*db_streams.offset(i_2 as isize)).name
                                                as *mut ::core::ffi::c_char,
                                        );
                                        hls_started_1 = r#true != 0;
                                    } else {
                                        _log_message_ctx(
                                            log_level_t::LOG_LEVEL_WARN,
                                            LOG_COMPONENT.as_ptr(),
                                            _log_stream_name,
                                            b"Failed to start HLS streaming for database stream: %s (attempt %d/3)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            &raw mut (*db_streams.offset(i_2 as isize)).name
                                                as *mut ::core::ffi::c_char,
                                            retry_1 + 1 as ::core::ffi::c_int,
                                        );
                                    }
                                    retry_1 += 1;
                                }
                                if !hls_started_1 {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_ERROR,
                                        LOG_COMPONENT.as_ptr(),
                                        _log_stream_name,
                                        b"Failed to start HLS streaming for database stream: %s after multiple attempts\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        &raw mut (*db_streams.offset(i_2 as isize)).name
                                            as *mut ::core::ffi::c_char,
                                    );
                                }
                            } else {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_WARN,
                                    LOG_COMPONENT.as_ptr(),
                                    _log_stream_name,
                                    b"HLS streaming not enabled for database stream: %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut (*db_streams.offset(i_2 as isize)).name
                                        as *mut ::core::ffi::c_char,
                                );
                            }
                        } else {
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_WARN,
                                LOG_COMPONENT.as_ptr(),
                                _log_stream_name,
                                b"Failed to start stream from database: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut (*db_streams.offset(i_2 as isize)).name
                                    as *mut ::core::ffi::c_char,
                            );
                        }
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Failed to add stream from database: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*db_streams.offset(i_2 as isize)).name
                                as *mut ::core::ffi::c_char,
                        );
                    }
                }
                i_2 += 1;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Waiting for database streams to fully start...\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            sleep(2 as ::core::ffi::c_uint);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Force restarting all HLS streams from database to ensure they're properly started...\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_3 < count {
                if (*db_streams.offset(i_3 as isize)).name[0usize] as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                    && (*db_streams.offset(i_3 as isize)).enabled as ::core::ffi::c_int != 0
                    && (*db_streams.offset(i_3 as isize)).streaming_enabled as ::core::ffi::c_int
                        != 0
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Force restarting HLS for database stream: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*db_streams.offset(i_3 as isize)).name
                            as *mut ::core::ffi::c_char,
                    );
                    if stream_stop_hls(
                        &raw mut (*db_streams.offset(i_3 as isize)).name
                            as *mut ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Failed to stop HLS stream for restart: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*db_streams.offset(i_3 as isize)).name
                                as *mut ::core::ffi::c_char,
                        );
                    }
                    usleep(500000 as ::core::ffi::c_int as __useconds_t);
                    if stream_start_hls(
                        &raw mut (*db_streams.offset(i_3 as isize)).name
                            as *mut ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"HLS streaming force restarted for database stream: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*db_streams.offset(i_3 as isize)).name
                                as *mut ::core::ffi::c_char,
                        );
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Failed to force restart HLS streaming for database stream: %s\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            &raw mut (*db_streams.offset(i_3 as isize)).name
                                as *mut ::core::ffi::c_char,
                        );
                    }
                }
                i_3 += 1;
            }
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"No streams found in the database\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        free(db_streams as *mut ::core::ffi::c_void);
        free(active_streams as *mut ::core::ffi::c_void);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Database path changed successfully\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut health_candidate: system_health_policy_t = system_health_policy_t {
        generation: 0,
        settings: system_health_policy_settings_t {
            enabled: false,
            profile: [0; 16],
            fast_interval_seconds: 0,
            normal_interval_seconds: 0,
            slow_interval_seconds: 0,
            device_interval_seconds: 0,
            write_probe_enabled: false,
            hardware_provider: [0; 16],
            presence_interval_seconds: 0,
            incident_retention_days: 0,
        },
        conditions: [system_health_condition_policy_t {
            condition: system_health_condition_t::SYSTEM_HEALTH_CONDITION_MEMORY_AVAILABLE_LOW,
            profile: system_health_profile_t::SYSTEM_HEALTH_PROFILE_BALANCED,
            enabled: false,
            overridden: false,
            direction: system_health_threshold_direction_t::SYSTEM_HEALTH_THRESHOLD_NONE,
            unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
            warning_threshold: 0.,
            critical_threshold: 0.,
            recovery_threshold: 0.,
            warning_for_seconds: 0,
            critical_for_seconds: 0,
            recovery_for_seconds: 0,
        }; 27],
    };
    let mut prior_health_settings: system_health_policy_settings_t = g_config.health;
    let mut health_canonical: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut persist_health_overrides: bool = r#false != 0;
    let mut health_error: [::core::ffi::c_char; 256] = [0; 256];
    let mut health_update: ::core::ffi::c_int = prepare_health_policy_update(
        settings,
        &raw mut health_candidate,
        &raw mut health_canonical,
        &raw mut persist_health_overrides,
        &raw mut health_error as *mut ::core::ffi::c_char,
    );
    if health_update < 0 as ::core::ffi::c_int {
        cJSON_Delete(settings);
        http_response_set_json_error(
            res,
            if health_update == -1 as ::core::ffi::c_int {
                400 as ::core::ffi::c_int
            } else {
                500 as ::core::ffi::c_int
            },
            &raw mut health_error as *mut ::core::ffi::c_char,
        );
        return;
    }
    if health_update == 1 as ::core::ffi::c_int {
        g_config.health = health_candidate.settings;
        settings_changed = r#true != 0;
    }
    if settings_changed {
        let mut config_path: *const ::core::ffi::c_char = get_loaded_config_path();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Saving configuration to file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if !config_path.is_null() {
                config_path
            } else {
                b"default path\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Current database path before saving: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.db_path as *mut ::core::ffi::c_char,
        );
        let mut save_result: ::core::ffi::c_int = 0;
        if !config_path.is_null() {
            save_result = save_config(&raw mut g_config, config_path);
        } else {
            save_result = save_config(
                &raw mut g_config,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        if save_result != 0 as ::core::ffi::c_int {
            if health_update == 1 as ::core::ffi::c_int {
                g_config.health = prior_health_settings;
            }
            free(health_canonical as *mut ::core::ffi::c_void);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to save configuration, error code: %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                save_result,
            );
            cJSON_Delete(settings);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to save configuration\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Configuration saved successfully\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if health_update == 1 as ::core::ffi::c_int {
            let mut previous_overrides: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut previous_overrides_length: size_t = 0 as size_t;
            let mut previous_result: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            if persist_health_overrides {
                previous_result = db_get_system_setting_alloc(
                    SYSTEM_HEALTH_OVERRIDES_SETTING_KEY.as_ptr(),
                    &raw mut previous_overrides,
                    &raw mut previous_overrides_length,
                );
                if previous_result < 0 as ::core::ffi::c_int
                    || db_set_system_setting(
                        SYSTEM_HEALTH_OVERRIDES_SETTING_KEY.as_ptr(),
                        health_canonical,
                    ) != 0 as ::core::ffi::c_int
                {
                    free(previous_overrides as *mut ::core::ffi::c_void);
                    g_config.health = prior_health_settings;
                    save_config(&raw mut g_config, config_path);
                    free(health_canonical as *mut ::core::ffi::c_void);
                    cJSON_Delete(settings);
                    http_response_set_json_error(
                        res,
                        500 as ::core::ffi::c_int,
                        b"Failed to persist health overrides\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return;
                }
            }
            if system_health_policy_replace(
                &raw mut health_candidate,
                &raw mut health_error as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                if persist_health_overrides {
                    db_set_system_setting(
                        SYSTEM_HEALTH_OVERRIDES_SETTING_KEY.as_ptr(),
                        if previous_result == 0 as ::core::ffi::c_int
                            && !previous_overrides.is_null()
                        {
                            previous_overrides
                        } else {
                            b"{\"version\":1,\"conditions\":[]}\0".as_ptr()
                                as *const ::core::ffi::c_char
                        },
                    );
                }
                free(previous_overrides as *mut ::core::ffi::c_void);
                g_config.health = prior_health_settings;
                save_config(&raw mut g_config, config_path);
                free(health_canonical as *mut ::core::ffi::c_void);
                cJSON_Delete(settings);
                http_response_set_json_error(
                    res,
                    500 as ::core::ffi::c_int,
                    b"Failed to activate health policy\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            free(previous_overrides as *mut ::core::ffi::c_void);
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"No settings changed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    free(health_canonical as *mut ::core::ffi::c_void);
    if go2rtc_config_changed {
        let mut task: *mut go2rtc_settings_task_t = calloc(
            1 as size_t,
            ::core::mem::size_of::<go2rtc_settings_task_t>(),
        ) as *mut go2rtc_settings_task_t;
        if !task.is_null() {
            (*task).becoming_enabled = go2rtc_becoming_enabled;
            let mut thread_id: pthread_t = 0;
            let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
            pthread_attr_init(&raw mut attr);
            pthread_attr_setdetachstate(
                &raw mut attr,
                C2Rust_Unnamed_0::PTHREAD_CREATE_DETACHED.0 as ::core::ffi::c_int,
            );
            if pthread_create(
                &raw mut thread_id,
                &raw mut attr,
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut go2rtc_settings_task_t) -> ()>,
                    Option<
                        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
                    >,
                >(Some(
                    go2rtc_settings_worker
                        as unsafe extern "C" fn(*mut go2rtc_settings_task_t) -> (),
                )),
                task as *mut ::core::ffi::c_void,
            ) != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to create go2rtc settings worker thread\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                free(task as *mut ::core::ffi::c_void);
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"go2rtc settings change dispatched to background thread (becoming_%s)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    if go2rtc_becoming_enabled as ::core::ffi::c_int != 0 {
                        b"enabled\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"disabled\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
            }
            pthread_attr_destroy(&raw mut attr);
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to allocate go2rtc settings task\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    if mqtt_config_changed {
        let mut task_0: *mut mqtt_settings_task_t =
            calloc(1 as size_t, ::core::mem::size_of::<mqtt_settings_task_t>())
                as *mut mqtt_settings_task_t;
        if !task_0.is_null() {
            (*task_0).mqtt_now_enabled = g_config.mqtt_enabled;
            let mut thread_id_0: pthread_t = 0;
            let mut attr_0: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
            pthread_attr_init(&raw mut attr_0);
            pthread_attr_setdetachstate(
                &raw mut attr_0,
                C2Rust_Unnamed_0::PTHREAD_CREATE_DETACHED.0 as ::core::ffi::c_int,
            );
            if pthread_create(
                &raw mut thread_id_0,
                &raw mut attr_0,
                Some(
                    mqtt_settings_worker
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                        )
                            -> *mut ::core::ffi::c_void,
                ),
                task_0 as *mut ::core::ffi::c_void,
            ) != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to create MQTT settings worker thread\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                free(task_0 as *mut ::core::ffi::c_void);
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"MQTT settings change dispatched to background thread\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            pthread_attr_destroy(&raw mut attr_0);
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to allocate MQTT settings task\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    cJSON_Delete(settings);
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
    cJSON_AddBoolToObject(
        success,
        b"restart_required\0".as_ptr() as *const ::core::ffi::c_char,
        restart_required as cJSON_bool,
    );
    if restart_required {
        if web_thread_pool_restart_required as ::core::ffi::c_int != 0
            && max_streams_restart_required as ::core::ffi::c_int != 0
        {
            cJSON_AddStringToObject(
                success,
                b"restart_required_message\0".as_ptr() as *const ::core::ffi::c_char,
                b"Worker thread pool size and Max Streams were saved, but LightNVR must be restarted before the new runtime capacity takes effect.\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        } else if web_thread_pool_restart_required {
            cJSON_AddStringToObject(
                success,
                b"restart_required_message\0".as_ptr() as *const ::core::ffi::c_char,
                b"Worker thread pool size was saved, but LightNVR must be restarted before the new thread pool takes effect.\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        } else if max_streams_restart_required {
            cJSON_AddStringToObject(
                success,
                b"restart_required_message\0".as_ptr() as *const ::core::ffi::c_char,
                b"Max Streams was saved, but LightNVR must be restarted before the new camera capacity takes effect.\0"
                    .as_ptr() as *const ::core::ffi::c_char,
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
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(success);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled POST /api/settings request\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
