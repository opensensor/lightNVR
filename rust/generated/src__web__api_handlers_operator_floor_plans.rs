use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut g_config: config_t;
    unsafe fn http_request_get_header(
        req: *const http_request_t,
        name: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
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
    unsafe fn http_serve_file(
        req: *const http_request_t,
        res: *const http_response_t,
        file_path: *const ::core::ffi::c_char,
        content_type: *const ::core::ffi::c_char,
        extra_headers: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_ParseWithLength(
        value: *const ::core::ffi::c_char,
        buffer_length: size_t,
    ) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
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
    unsafe fn cJSON_CreateNumber(num: ::core::ffi::c_double) -> *mut cJSON;
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
    unsafe fn cJSON_AddArrayToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...)
    -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn openat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn fmin(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double)
    -> ::core::ffi::c_double;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn fstatat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __buf: *mut stat,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn geteuid() -> __uid_t;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn unlinkat(
        __fd: ::core::ffi::c_int,
        __name: *const ::core::ffi::c_char,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn rmdir(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn syscall(__sysno: ::core::ffi::c_long, ...) -> ::core::ffi::c_long;
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
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn mkdir_recursive(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_operator_floor_plan_list(
        plans: *mut operator_floor_plan_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_operator_floor_plan_get(
        uuid: *const ::core::ffi::c_char,
        plan: *mut operator_floor_plan_t,
    ) -> db_operator_floor_plan_result_t;
    unsafe fn db_operator_floor_plan_camera_list(
        plan_uuid: *const ::core::ffi::c_char,
        cameras: *mut operator_floor_plan_camera_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_operator_floor_plan_create(
        plan: *mut operator_floor_plan_t,
        cameras: *const operator_floor_plan_camera_t,
        camera_count: ::core::ffi::c_int,
        sketch_json: *const ::core::ffi::c_char,
    ) -> db_operator_floor_plan_result_t;
    unsafe fn db_operator_floor_plan_update(
        plan: *mut operator_floor_plan_t,
        cameras: *const operator_floor_plan_camera_t,
        camera_count: ::core::ffi::c_int,
        expected_revision: int64_t,
        sketch_json: *const ::core::ffi::c_char,
    ) -> db_operator_floor_plan_result_t;
    unsafe fn db_operator_floor_plan_sketch_load(
        uuid: *const ::core::ffi::c_char,
        sketch_json: *mut *mut ::core::ffi::c_char,
    ) -> db_operator_floor_plan_result_t;
    unsafe fn db_operator_floor_plan_delete(
        uuid: *const ::core::ffi::c_char,
        expected_revision: int64_t,
    ) -> db_operator_floor_plan_result_t;
    unsafe fn db_operator_floor_plan_set_background(
        uuid: *const ::core::ffi::c_char,
        background_mime: *const ::core::ffi::c_char,
    ) -> db_operator_floor_plan_result_t;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
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
    unsafe fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn mkdtemp(__template: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub struct operator_floor_plan_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub location_uuid: [::core::ffi::c_char; 37],
    pub parent_plan_uuid: [::core::ffi::c_char; 37],
    pub canvas_width: ::core::ffi::c_int,
    pub canvas_height: ::core::ffi::c_int,
    pub background_mime: [::core::ffi::c_char; 16],
    pub sketch_bytes: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct operator_floor_plan_camera_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub rotation: ::core::ffi::c_double,
    pub fov: ::core::ffi::c_double,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_operator_floor_plan_result_t(pub ::core::ffi::c_int);
impl db_operator_floor_plan_result_t {
    pub const DB_OPERATOR_FLOOR_PLAN_OK: Self = Self(0);
    pub const DB_OPERATOR_FLOOR_PLAN_NOT_FOUND: Self = Self(-1);
    pub const DB_OPERATOR_FLOOR_PLAN_INVALID: Self = Self(-2);
    pub const DB_OPERATOR_FLOOR_PLAN_CONFLICT: Self = Self(-3);
    pub const DB_OPERATOR_FLOOR_PLAN_STALE: Self = Self(-4);
    pub const DB_OPERATOR_FLOOR_PLAN_LIMIT: Self = Self(-5);
    pub const DB_OPERATOR_FLOOR_PLAN_ERROR: Self = Self(-6);
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
pub struct background_backup_t {
    pub directory: [::core::ffi::c_char; 672],
    pub path: [::core::ffi::c_char; 688],
    pub descriptor: ::core::ffi::c_int,
    pub parent: ::core::ffi::c_int,
    pub old_file: ::core::ffi::c_int,
    pub upload_file: ::core::ffi::c_int,
    pub upload_name: [::core::ffi::c_char; 64],
    pub upload_identity: stat,
    pub parent_identity: stat,
    pub created: bool,
    pub validated: bool,
    pub identity: stat,
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
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __O_DIRECTORY: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const __O_PATH: ::core::ffi::c_int = 0o10000000 as ::core::ffi::c_int;
pub const O_DIRECTORY: ::core::ffi::c_int = __O_DIRECTORY;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const O_PATH: ::core::ffi::c_int = __O_PATH;
pub const F_DUPFD_CLOEXEC: ::core::ffi::c_int = 1030 as ::core::ffi::c_int;
pub const AT_SYMLINK_NOFOLLOW: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const OPERATOR_FLOOR_PLAN_MAX_VISIBLE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const OPERATOR_FLOOR_PLAN_MAX_CAMERAS: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const OPERATOR_FLOOR_PLAN_SKETCH_MAX: ::core::ffi::c_int =
    256 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const __NR_renameat2: ::core::ffi::c_int = 316 as ::core::ffi::c_int;
pub const SYS_renameat2: ::core::ffi::c_int = __NR_renameat2;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const OPERATOR_FLOOR_PLAN_BACKGROUND_MAX: ::core::ffi::c_int =
    768 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
pub const SKETCH_MAX_SHAPES: ::core::ffi::c_int = 400 as ::core::ffi::c_int;
pub const SKETCH_MAX_POINTS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const SKETCH_MIN_POLYLINE_POINTS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SKETCH_MIN_POLYGON_POINTS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SKETCH_MIN_RECT_SIDE: ::core::ffi::c_double = 0.005f64;
pub const SKETCH_ID_MAX: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const SKETCH_TEXT_MAX: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
static mut background_files_mutex: pthread_mutex_t = pthread_mutex_t {
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
unsafe extern "C" fn read_only_identity(mut user: *const user_t) -> bool {
    return (*user).authenticated_via_scoped_token as ::core::ffi::c_int != 0
        || strcmp(
            &raw const (*user).authentication_method as *const ::core::ffi::c_char,
            b"demo\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn can_modify(mut user: *const user_t) -> bool {
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
    return !read_only_identity(user)
        && authorization_evaluate(
            user,
            authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
            ::core::ptr::null::<fleet_camera_t>(),
            &raw mut evaluation,
        ) == 0 as ::core::ffi::c_int
        && evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0;
}
unsafe extern "C" fn load_authorized_cameras(
    mut user: *const user_t,
    mut action: authorization_action_t,
    mut cameras: *mut *mut fleet_camera_t,
    mut camera_count: *mut ::core::ffi::c_int,
) -> bool {
    *cameras = ::core::ptr::null_mut::<fleet_camera_t>();
    *camera_count = 0 as ::core::ffi::c_int;
    return db_fleet_camera_load(cameras, camera_count) == 0 as ::core::ffi::c_int
        && authorization_filter_cameras(user, action, *cameras, camera_count)
            == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn camera_allowed(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut cameras: *const fleet_camera_t,
    mut camera_count: ::core::ffi::c_int,
) -> bool {
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < camera_count {
        if strcmp(
            &raw const (*cameras.offset(index as isize)).camera_uuid as *const ::core::ffi::c_char,
            camera_uuid,
        ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        index += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn copy_string(
    mut body: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut destination: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut required: bool,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, key);
    if item.is_null() {
        return !required;
    }
    if cJSON_IsString(item) == 0
        || (*item).valuestring.is_null()
        || required as ::core::ffi::c_int != 0
            && *(*item).valuestring.offset(0isize) as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int
        || strlen((*item).valuestring) >= size
    {
        return r#false != 0;
    }
    safe_strcpy(destination, (*item).valuestring, size, 0 as size_t);
    return r#true != 0;
}
unsafe extern "C" fn optional_uuid(
    mut body: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut destination: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, key);
    if item.is_null() || cJSON_IsNull(item) != 0 {
        *destination.offset(0isize) = '\0' as ::core::ffi::c_char;
        return r#true != 0;
    }
    if cJSON_IsString(item) == 0
        || (*item).valuestring.is_null()
        || !lightnvr_uuid_is_valid((*item).valuestring)
    {
        return r#false != 0;
    }
    safe_strcpy(destination, (*item).valuestring, size, 0 as size_t);
    return r#true != 0;
}
unsafe extern "C" fn integer_field(
    mut body: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut minimum: ::core::ffi::c_int,
    mut maximum: ::core::ffi::c_int,
    mut fallback: ::core::ffi::c_int,
    mut value: *mut ::core::ffi::c_int,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, key);
    if item.is_null() {
        *value = fallback;
        return r#true != 0;
    }
    if cJSON_IsNumber(item) == 0
        || (*item).valuedouble.is_finite() as ::core::ffi::c_int == 0
        || floor((*item).valuedouble) != (*item).valuedouble
        || (*item).valueint < minimum
        || (*item).valueint > maximum
    {
        return r#false != 0;
    }
    *value = (*item).valueint;
    return r#true != 0;
}
unsafe extern "C" fn number_field(
    mut body: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut minimum: ::core::ffi::c_double,
    mut maximum: ::core::ffi::c_double,
    mut fallback: ::core::ffi::c_double,
    mut value: *mut ::core::ffi::c_double,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, key);
    if item.is_null() {
        *value = fallback;
        return r#true != 0;
    }
    if cJSON_IsNumber(item) == 0
        || (*item).valuedouble.is_finite() as ::core::ffi::c_int == 0
        || (*item).valuedouble < minimum
        || (*item).valuedouble > maximum
    {
        return r#false != 0;
    }
    *value = (*item).valuedouble;
    return r#true != 0;
}
unsafe extern "C" fn parse_revision(mut body: *const cJSON, mut revision: *mut int64_t) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsNumber(item) == 0
        || (*item).valuedouble.is_finite() as ::core::ffi::c_int == 0
        || floor((*item).valuedouble) != (*item).valuedouble
        || (*item).valuedouble < 1 as ::core::ffi::c_int as ::core::ffi::c_double
        || (*item).valuedouble > INT64_MAX as ::core::ffi::c_double
    {
        return r#false != 0;
    }
    *revision = (*item).valuedouble as int64_t;
    return r#true != 0;
}
static mut SKETCH_TONES: [*const ::core::ffi::c_char; 6] = [
    b"slate\0".as_ptr() as *const ::core::ffi::c_char,
    b"blue\0".as_ptr() as *const ::core::ffi::c_char,
    b"green\0".as_ptr() as *const ::core::ffi::c_char,
    b"amber\0".as_ptr() as *const ::core::ffi::c_char,
    b"red\0".as_ptr() as *const ::core::ffi::c_char,
    b"violet\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut SKETCH_LABEL_SIZES: [*const ::core::ffi::c_char; 3] = [
    b"sm\0".as_ptr() as *const ::core::ffi::c_char,
    b"md\0".as_ptr() as *const ::core::ffi::c_char,
    b"lg\0".as_ptr() as *const ::core::ffi::c_char,
];
unsafe extern "C" fn sketch_enum_field(
    mut item: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut allowed: *const *const ::core::ffi::c_char,
    mut allowed_count: size_t,
    mut value: *mut *const ::core::ffi::c_char,
) -> bool {
    let mut field: *const cJSON = cJSON_GetObjectItemCaseSensitive(item, key);
    if field.is_null() {
        *value = *allowed.offset(0isize);
        return r#true != 0;
    }
    if cJSON_IsString(field) == 0 || (*field).valuestring.is_null() {
        return r#false != 0;
    }
    let mut index: size_t = 0 as size_t;
    while index < allowed_count {
        if strcmp((*field).valuestring, *allowed.offset(index as isize)) == 0 as ::core::ffi::c_int
        {
            *value = *allowed.offset(index as isize);
            return r#true != 0;
        }
        index = index.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn sketch_text_valid(
    mut text: *const ::core::ffi::c_char,
    mut minimum: size_t,
    mut maximum: size_t,
) -> bool {
    let mut length: size_t = strnlen(text, maximum.wrapping_add(1 as size_t));
    if length < minimum || length > maximum {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = text as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if (*cursor as ::core::ffi::c_int) < 0x20 as ::core::ffi::c_int
            || *cursor as ::core::ffi::c_int == 0x7f as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn sketch_optional_text(
    mut item: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut value: *mut *const ::core::ffi::c_char,
) -> bool {
    let mut field: *const cJSON = cJSON_GetObjectItemCaseSensitive(item, key);
    *value = ::core::ptr::null::<::core::ffi::c_char>();
    if field.is_null() || cJSON_IsNull(field) != 0 {
        return r#true != 0;
    }
    if cJSON_IsString(field) == 0 || (*field).valuestring.is_null() {
        return r#false != 0;
    }
    if *(*field).valuestring.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return r#true != 0;
    }
    if !sketch_text_valid((*field).valuestring, 1 as size_t, SKETCH_TEXT_MAX as size_t) {
        return r#false != 0;
    }
    *value = (*field).valuestring;
    return r#true != 0;
}
unsafe extern "C" fn sketch_id_valid(mut id: *const ::core::ffi::c_char) -> bool {
    let mut length: size_t = strnlen(id, (SKETCH_ID_MAX + 1 as ::core::ffi::c_int) as size_t);
    if length == 0 as size_t || length > SKETCH_ID_MAX as size_t {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_char = id;
    while *cursor != 0 {
        if *(*__ctype_b_loc())
            .offset(*cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
            && *cursor as ::core::ffi::c_int != '_' as ::core::ffi::c_int
            && *cursor as ::core::ffi::c_int != '-' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn sketch_round(mut value: ::core::ffi::c_double) -> ::core::ffi::c_double {
    return round(value * 10000.0f64) / 10000.0f64;
}
unsafe extern "C" fn sketch_unit_field(
    mut item: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut minimum: ::core::ffi::c_double,
    mut value: *mut ::core::ffi::c_double,
) -> bool {
    let mut field: *const cJSON = cJSON_GetObjectItemCaseSensitive(item, key);
    if cJSON_IsNumber(field) == 0
        || (*field).valuedouble.is_finite() as ::core::ffi::c_int == 0
        || (*field).valuedouble < minimum
        || (*field).valuedouble > 1.0f64
    {
        return r#false != 0;
    }
    *value = sketch_round((*field).valuedouble);
    return r#true != 0;
}
unsafe extern "C" fn sketch_points(
    mut item: *const cJSON,
    mut minimum: ::core::ffi::c_int,
    mut out: *mut cJSON,
) -> bool {
    let mut points: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(item, b"points\0".as_ptr() as *const ::core::ffi::c_char);
    if cJSON_IsArray(points) == 0 {
        return r#false != 0;
    }
    let mut count: ::core::ffi::c_int = cJSON_GetArraySize(points);
    if count < minimum || count > SKETCH_MAX_POINTS {
        return r#false != 0;
    }
    let mut canonical: *mut cJSON =
        cJSON_AddArrayToObject(out, b"points\0".as_ptr() as *const ::core::ffi::c_char);
    if canonical.is_null() {
        return r#false != 0;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut point: *const cJSON = cJSON_GetArrayItem(points, index);
        if cJSON_IsArray(point) == 0 || cJSON_GetArraySize(point) != 2 as ::core::ffi::c_int {
            return r#false != 0;
        }
        let mut x: *const cJSON = cJSON_GetArrayItem(point, 0 as ::core::ffi::c_int);
        let mut y: *const cJSON = cJSON_GetArrayItem(point, 1 as ::core::ffi::c_int);
        if cJSON_IsNumber(x) == 0
            || cJSON_IsNumber(y) == 0
            || (*x).valuedouble.is_finite() as ::core::ffi::c_int == 0
            || (*y).valuedouble.is_finite() as ::core::ffi::c_int == 0
            || (*x).valuedouble < 0.0f64
            || (*x).valuedouble > 1.0f64
            || (*y).valuedouble < 0.0f64
            || (*y).valuedouble > 1.0f64
        {
            return r#false != 0;
        }
        let mut pair: *mut cJSON = cJSON_CreateArray();
        if pair.is_null() {
            return r#false != 0;
        }
        cJSON_AddItemToArray(pair, cJSON_CreateNumber(sketch_round((*x).valuedouble)));
        cJSON_AddItemToArray(pair, cJSON_CreateNumber(sketch_round((*y).valuedouble)));
        cJSON_AddItemToArray(canonical, pair);
        index += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn sketch_shape(mut item: *const cJSON, mut shapes: *mut cJSON) -> bool {
    if cJSON_IsObject(item) == 0 {
        return r#false != 0;
    }
    let mut r#type: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(item, b"type\0".as_ptr() as *const ::core::ffi::c_char);
    let mut id: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(item, b"id\0".as_ptr() as *const ::core::ffi::c_char);
    let mut tone: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if cJSON_IsString(r#type) == 0
        || (*r#type).valuestring.is_null()
        || cJSON_IsString(id) == 0
        || (*id).valuestring.is_null()
        || !sketch_id_valid((*id).valuestring)
        || !sketch_enum_field(
            item,
            b"tone\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const SKETCH_TONES as *const *const ::core::ffi::c_char,
            ::core::mem::size_of::<[*const ::core::ffi::c_char; 6]>()
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
            &raw mut tone,
        )
    {
        return r#false != 0;
    }
    let mut out: *mut cJSON = cJSON_CreateObject();
    if out.is_null() {
        return r#false != 0;
    }
    cJSON_AddItemToArray(shapes, out);
    cJSON_AddStringToObject(
        out,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        (*id).valuestring,
    );
    cJSON_AddStringToObject(
        out,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        (*r#type).valuestring,
    );
    cJSON_AddStringToObject(out, b"tone\0".as_ptr() as *const ::core::ffi::c_char, tone);
    let mut text: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if strcmp(
        (*r#type).valuestring,
        b"rect\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut x: ::core::ffi::c_double = 0.;
        let mut y: ::core::ffi::c_double = 0.;
        let mut w: ::core::ffi::c_double = 0.;
        let mut h: ::core::ffi::c_double = 0.;
        let mut filled: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"filled\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !sketch_unit_field(
            item,
            b"x\0".as_ptr() as *const ::core::ffi::c_char,
            0.0f64,
            &raw mut x,
        ) || !sketch_unit_field(
            item,
            b"y\0".as_ptr() as *const ::core::ffi::c_char,
            0.0f64,
            &raw mut y,
        ) || !sketch_unit_field(
            item,
            b"w\0".as_ptr() as *const ::core::ffi::c_char,
            SKETCH_MIN_RECT_SIDE,
            &raw mut w,
        ) || !sketch_unit_field(
            item,
            b"h\0".as_ptr() as *const ::core::ffi::c_char,
            SKETCH_MIN_RECT_SIDE,
            &raw mut h,
        ) || x + w > 1.0f64 + 1e-6f64
            || y + h > 1.0f64 + 1e-6f64
            || !filled.is_null() && cJSON_IsBool(filled) == 0
            || !sketch_optional_text(
                item,
                b"label\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut text,
            )
        {
            return r#false != 0;
        }
        cJSON_AddNumberToObject(out, b"x\0".as_ptr() as *const ::core::ffi::c_char, x);
        cJSON_AddNumberToObject(out, b"y\0".as_ptr() as *const ::core::ffi::c_char, y);
        cJSON_AddNumberToObject(
            out,
            b"w\0".as_ptr() as *const ::core::ffi::c_char,
            fmin(w, 1.0f64 - x),
        );
        cJSON_AddNumberToObject(
            out,
            b"h\0".as_ptr() as *const ::core::ffi::c_char,
            fmin(h, 1.0f64 - y),
        );
        cJSON_AddBoolToObject(
            out,
            b"filled\0".as_ptr() as *const ::core::ffi::c_char,
            (filled.is_null() || cJSON_IsTrue(filled) != 0) as ::core::ffi::c_int,
        );
        if !text.is_null() {
            cJSON_AddStringToObject(out, b"label\0".as_ptr() as *const ::core::ffi::c_char, text);
        }
        return r#true != 0;
    }
    if strcmp(
        (*r#type).valuestring,
        b"wall\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return sketch_points(item, SKETCH_MIN_POLYLINE_POINTS, out);
    }
    if strcmp(
        (*r#type).valuestring,
        b"area\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if !sketch_points(item, SKETCH_MIN_POLYGON_POINTS, out)
            || !sketch_optional_text(
                item,
                b"label\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut text,
            )
        {
            return r#false != 0;
        }
        if !text.is_null() {
            cJSON_AddStringToObject(out, b"label\0".as_ptr() as *const ::core::ffi::c_char, text);
        }
        return r#true != 0;
    }
    if strcmp(
        (*r#type).valuestring,
        b"label\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut x_0: ::core::ffi::c_double = 0.;
        let mut y_0: ::core::ffi::c_double = 0.;
        let mut size: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut label: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"text\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !sketch_unit_field(
            item,
            b"x\0".as_ptr() as *const ::core::ffi::c_char,
            0.0f64,
            &raw mut x_0,
        ) || !sketch_unit_field(
            item,
            b"y\0".as_ptr() as *const ::core::ffi::c_char,
            0.0f64,
            &raw mut y_0,
        ) || cJSON_IsString(label) == 0
            || (*label).valuestring.is_null()
            || !sketch_text_valid((*label).valuestring, 1 as size_t, SKETCH_TEXT_MAX as size_t)
            || !sketch_enum_field(
                item,
                b"size\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const SKETCH_LABEL_SIZES as *const *const ::core::ffi::c_char,
                ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
                    .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
                &raw mut size,
            )
        {
            return r#false != 0;
        }
        cJSON_AddNumberToObject(out, b"x\0".as_ptr() as *const ::core::ffi::c_char, x_0);
        cJSON_AddNumberToObject(out, b"y\0".as_ptr() as *const ::core::ffi::c_char, y_0);
        cJSON_AddStringToObject(
            out,
            b"text\0".as_ptr() as *const ::core::ffi::c_char,
            (*label).valuestring,
        );
        cJSON_AddStringToObject(out, b"size\0".as_ptr() as *const ::core::ffi::c_char, size);
        return r#true != 0;
    }
    return r#false != 0;
}
unsafe extern "C" fn parse_sketch(
    mut body: *const cJSON,
    mut sketch_json: *mut *mut ::core::ffi::c_char,
    mut res: *mut http_response_t,
) -> bool {
    *sketch_json = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sketch: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"sketch\0".as_ptr() as *const ::core::ffi::c_char);
    if sketch.is_null() {
        return r#true != 0;
    }
    let mut shapes: *const cJSON = if cJSON_IsObject(sketch) != 0 {
        cJSON_GetObjectItemCaseSensitive(sketch, b"shapes\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if cJSON_IsNull(sketch) != 0
        || cJSON_IsObject(sketch) != 0 && shapes.is_null()
        || cJSON_IsArray(shapes) != 0 && cJSON_GetArraySize(shapes) == 0 as ::core::ffi::c_int
    {
        *sketch_json = strdup(b"\0".as_ptr() as *const ::core::ffi::c_char);
        if (*sketch_json).is_null() {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Memory allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        return r#true != 0;
    }
    let mut version: *const cJSON = if cJSON_IsObject(sketch) != 0 {
        cJSON_GetObjectItemCaseSensitive(
            sketch,
            b"version\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if cJSON_IsArray(shapes) == 0
        || !version.is_null()
            && (cJSON_IsNumber(version) == 0 || (*version).valueint != 1 as ::core::ffi::c_int)
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid floor plan sketch\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut count: ::core::ffi::c_int = cJSON_GetArraySize(shapes);
    if count > SKETCH_MAX_SHAPES {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Too many floor plan sketch shapes\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut canonical: *mut cJSON = cJSON_CreateObject();
    let mut out_shapes: *mut cJSON = if !canonical.is_null() {
        cJSON_AddArrayToObject(
            canonical,
            b"shapes\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if canonical.is_null()
        || out_shapes.is_null()
        || cJSON_AddNumberToObject(
            canonical,
            b"version\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int as ::core::ffi::c_double,
        )
        .is_null()
    {
        cJSON_Delete(canonical);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Memory allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut valid: bool = r#true != 0;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while valid as ::core::ffi::c_int != 0 && index < count {
        let mut item: *const cJSON = cJSON_GetArrayItem(shapes, index);
        valid = sketch_shape(item, out_shapes);
        let mut id: *const cJSON = if valid as ::core::ffi::c_int != 0 {
            cJSON_GetObjectItemCaseSensitive(item, b"id\0".as_ptr() as *const ::core::ffi::c_char)
        } else {
            ::core::ptr::null_mut::<cJSON>()
        };
        let mut previous: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while valid as ::core::ffi::c_int != 0 && previous < index {
            let mut other: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                cJSON_GetArrayItem(out_shapes, previous),
                b"id\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !other.is_null()
                && strcmp((*other).valuestring, (*id).valuestring) == 0 as ::core::ffi::c_int
            {
                valid = r#false != 0;
            }
            previous += 1;
        }
        index += 1;
    }
    if !valid {
        cJSON_Delete(canonical);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid floor plan sketch\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut encoded: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(canonical);
    cJSON_Delete(canonical);
    if encoded.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Memory allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if strlen(encoded) > OPERATOR_FLOOR_PLAN_SKETCH_MAX as size_t {
        free(encoded as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Floor plan sketch too large\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *sketch_json = encoded;
    return r#true != 0;
}
unsafe extern "C" fn parse_plan(
    mut body: *const cJSON,
    mut creating: bool,
    mut plan: *mut operator_floor_plan_t,
    mut cameras: *mut *mut operator_floor_plan_camera_t,
    mut camera_count: *mut ::core::ffi::c_int,
    mut res: *mut http_response_t,
) -> bool {
    memset(
        plan as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<operator_floor_plan_t>(),
    );
    *cameras = ::core::ptr::null_mut::<operator_floor_plan_camera_t>();
    *camera_count = 0 as ::core::ffi::c_int;
    if cJSON_IsObject(body) == 0
        || !copy_string(
            body,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*plan).name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            r#true != 0,
        )
        || !optional_uuid(
            body,
            b"location_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*plan).location_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        )
        || !optional_uuid(
            body,
            b"parent_plan_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*plan).parent_plan_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        )
        || !integer_field(
            body,
            b"canvas_width\0".as_ptr() as *const ::core::ffi::c_char,
            400 as ::core::ffi::c_int,
            4000 as ::core::ffi::c_int,
            1200 as ::core::ffi::c_int,
            &raw mut (*plan).canvas_width,
        )
        || !integer_field(
            body,
            b"canvas_height\0".as_ptr() as *const ::core::ffi::c_char,
            300 as ::core::ffi::c_int,
            4000 as ::core::ffi::c_int,
            800 as ::core::ffi::c_int,
            &raw mut (*plan).canvas_height,
        )
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid floor plan fields\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut items: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"cameras\0".as_ptr() as *const ::core::ffi::c_char);
    if items.is_null() && creating as ::core::ffi::c_int != 0 {
        return r#true != 0;
    }
    if cJSON_IsArray(items) == 0 {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"cameras must be an array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut count: ::core::ffi::c_int = cJSON_GetArraySize(items);
    if count < 0 as ::core::ffi::c_int || count > OPERATOR_FLOOR_PLAN_MAX_CAMERAS {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Too many floor plan cameras\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut parsed: *mut operator_floor_plan_camera_t = (if count > 0 as ::core::ffi::c_int {
        calloc(
            count as size_t,
            ::core::mem::size_of::<operator_floor_plan_camera_t>(),
        )
    } else {
        NULL
    }) as *mut operator_floor_plan_camera_t;
    if count > 0 as ::core::ffi::c_int && parsed.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Memory allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut valid: bool = r#true != 0;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while valid as ::core::ffi::c_int != 0 && index < count {
        let mut item: *const cJSON = cJSON_GetArrayItem(items, index);
        valid = cJSON_IsObject(item) != 0
            && copy_string(
                item,
                b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*parsed.offset(index as isize)).camera_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                r#true != 0,
            ) as ::core::ffi::c_int
                != 0
            && lightnvr_uuid_is_valid(
                &raw mut (*parsed.offset(index as isize)).camera_uuid as *mut ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            && number_field(
                item,
                b"x\0".as_ptr() as *const ::core::ffi::c_char,
                0.0f64,
                1.0f64,
                0.5f64,
                &raw mut (*parsed.offset(index as isize)).x,
            ) as ::core::ffi::c_int
                != 0
            && number_field(
                item,
                b"y\0".as_ptr() as *const ::core::ffi::c_char,
                0.0f64,
                1.0f64,
                0.5f64,
                &raw mut (*parsed.offset(index as isize)).y,
            ) as ::core::ffi::c_int
                != 0
            && number_field(
                item,
                b"rotation\0".as_ptr() as *const ::core::ffi::c_char,
                -180.0f64,
                180.0f64,
                0.0f64,
                &raw mut (*parsed.offset(index as isize)).rotation,
            ) as ::core::ffi::c_int
                != 0
            && number_field(
                item,
                b"fov\0".as_ptr() as *const ::core::ffi::c_char,
                1.0f64,
                180.0f64,
                65.0f64,
                &raw mut (*parsed.offset(index as isize)).fov,
            ) as ::core::ffi::c_int
                != 0;
        let mut previous: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while valid as ::core::ffi::c_int != 0 && previous < index {
            if strcmp(
                &raw mut (*parsed.offset(previous as isize)).camera_uuid
                    as *mut ::core::ffi::c_char,
                &raw mut (*parsed.offset(index as isize)).camera_uuid as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                valid = r#false != 0;
            }
            previous += 1;
        }
        index += 1;
    }
    if !valid {
        free(parsed as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid floor plan camera placement\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *cameras = parsed;
    *camera_count = count;
    return r#true != 0;
}
unsafe extern "C" fn plan_json(
    mut plan: *const operator_floor_plan_t,
    mut placements: *const operator_floor_plan_camera_t,
    mut placement_count: ::core::ffi::c_int,
    mut authorized: *const fleet_camera_t,
    mut authorized_count: ::core::ffi::c_int,
    mut sketch_json: *const ::core::ffi::c_char,
) -> *mut cJSON {
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut cameras: *mut cJSON = if !root.is_null() {
        cJSON_AddArrayToObject(root, b"cameras\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if root.is_null() || cameras.is_null() {
        cJSON_Delete(root);
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut sketch: *mut cJSON =
        if !sketch_json.is_null() && *sketch_json.offset(0isize) as ::core::ffi::c_int != 0 {
            cJSON_Parse(sketch_json)
        } else {
            ::core::ptr::null_mut::<cJSON>()
        };
    if !sketch.is_null() {
        cJSON_AddItemToObject(
            root,
            b"sketch\0".as_ptr() as *const ::core::ffi::c_char,
            sketch,
        );
    } else {
        cJSON_AddNullToObject(root, b"sketch\0".as_ptr() as *const ::core::ffi::c_char);
    }
    cJSON_AddStringToObject(
        root,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*plan).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*plan).name as *const ::core::ffi::c_char,
    );
    if (*plan).location_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            root,
            b"location_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*plan).location_uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            root,
            b"location_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*plan).parent_plan_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            root,
            b"parent_plan_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*plan).parent_plan_uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            root,
            b"parent_plan_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddNumberToObject(
        root,
        b"canvas_width\0".as_ptr() as *const ::core::ffi::c_char,
        (*plan).canvas_width as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"canvas_height\0".as_ptr() as *const ::core::ffi::c_char,
        (*plan).canvas_height as ::core::ffi::c_double,
    );
    if (*plan).background_mime[0usize] != 0 {
        cJSON_AddStringToObject(
            root,
            b"background_mime\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*plan).background_mime as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            root,
            b"background_mime\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut partially_authorized: bool = r#false != 0;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < placement_count {
        if !camera_allowed(
            &raw const (*placements.offset(index as isize)).camera_uuid
                as *const ::core::ffi::c_char,
            authorized,
            authorized_count,
        ) {
            partially_authorized = r#true != 0;
        } else {
            let mut camera: *mut cJSON = cJSON_CreateObject();
            if !camera.is_null() {
                cJSON_AddStringToObject(
                    camera,
                    b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*placements.offset(index as isize)).camera_uuid
                        as *const ::core::ffi::c_char,
                );
                cJSON_AddNumberToObject(
                    camera,
                    b"x\0".as_ptr() as *const ::core::ffi::c_char,
                    (*placements.offset(index as isize)).x,
                );
                cJSON_AddNumberToObject(
                    camera,
                    b"y\0".as_ptr() as *const ::core::ffi::c_char,
                    (*placements.offset(index as isize)).y,
                );
                cJSON_AddNumberToObject(
                    camera,
                    b"rotation\0".as_ptr() as *const ::core::ffi::c_char,
                    (*placements.offset(index as isize)).rotation,
                );
                cJSON_AddNumberToObject(
                    camera,
                    b"fov\0".as_ptr() as *const ::core::ffi::c_char,
                    (*placements.offset(index as isize)).fov,
                );
                cJSON_AddItemToArray(cameras, camera);
            }
        }
        index += 1;
    }
    cJSON_AddBoolToObject(
        root,
        b"partially_authorized\0".as_ptr() as *const ::core::ffi::c_char,
        partially_authorized as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        root,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        (*plan).revision as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*plan).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*plan).updated_at as ::core::ffi::c_double,
    );
    return root;
}
unsafe extern "C" fn send_json(
    mut res: *mut http_response_t,
    mut status: ::core::ffi::c_int,
    mut json: *mut cJSON,
) {
    let mut encoded: *mut ::core::ffi::c_char = if !json.is_null() {
        cJSON_PrintUnformatted(json)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    if encoded.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, status, encoded);
    free(encoded as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn set_db_error(
    mut res: *mut http_response_t,
    mut result: db_operator_floor_plan_result_t,
) {
    match result {
        db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Floor plan not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid floor plan\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_CONFLICT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Floor plan name or relationship conflicts\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_STALE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Floor plan changed; reload and retry\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_LIMIT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Floor plan limit reached\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Floor plan database operation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn extract_uuid(
    mut req: *const http_request_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut uuid_size: size_t,
    mut res: *mut http_response_t,
) -> bool {
    if http_request_extract_path_param(
        req,
        b"/api/live/plans/\0".as_ptr() as *const ::core::ffi::c_char,
        uuid,
        uuid_size,
    ) != 0 as ::core::ffi::c_int
        || !strchr(uuid, '/' as ::core::ffi::c_int).is_null()
        || !lightnvr_uuid_is_valid(uuid)
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid floor plan path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn require_modify(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut user: *mut user_t,
) -> bool {
    if !authenticate(req, res, user) {
        return r#false != 0;
    }
    if !can_modify(user) {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Floor plan editing is not allowed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn load_one_json(
    mut uuid: *const ::core::ffi::c_char,
    mut user: *const user_t,
    mut res: *mut http_response_t,
) -> *mut cJSON {
    let mut plan: operator_floor_plan_t = operator_floor_plan_t {
        uuid: [0; 37],
        name: [0; 128],
        location_uuid: [0; 37],
        parent_plan_uuid: [0; 37],
        canvas_width: 0,
        canvas_height: 0,
        background_mime: [0; 16],
        sketch_bytes: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_operator_floor_plan_result_t =
        db_operator_floor_plan_get(uuid, &raw mut plan);
    if result.0 != db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK.0 {
        set_db_error(res, result);
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut placements: *mut operator_floor_plan_camera_t = calloc(
        OPERATOR_FLOOR_PLAN_MAX_CAMERAS as size_t,
        ::core::mem::size_of::<operator_floor_plan_camera_t>(),
    )
        as *mut operator_floor_plan_camera_t;
    let mut authorized: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut authorized_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut placement_count: ::core::ffi::c_int = if !placements.is_null() {
        db_operator_floor_plan_camera_list(uuid, placements, OPERATOR_FLOOR_PLAN_MAX_CAMERAS)
    } else {
        -1 as ::core::ffi::c_int
    };
    if placement_count < 0 as ::core::ffi::c_int
        || !load_authorized_cameras(
            user,
            authorization_action_t::AUTHZ_LIVE_VIEW,
            &raw mut authorized,
            &raw mut authorized_count,
        )
    {
        free(placements as *mut ::core::ffi::c_void);
        free(authorized as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load floor plan\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut sketch_json: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    db_operator_floor_plan_sketch_load(uuid, &raw mut sketch_json);
    let mut json: *mut cJSON = plan_json(
        &raw mut plan,
        placements,
        placement_count,
        authorized,
        authorized_count,
        sketch_json,
    );
    free(sketch_json as *mut ::core::ffi::c_void);
    free(placements as *mut ::core::ffi::c_void);
    free(authorized as *mut ::core::ffi::c_void);
    return json;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_operator_floor_plans(
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
    let mut plans: *mut operator_floor_plan_t = calloc(
        OPERATOR_FLOOR_PLAN_MAX_VISIBLE as size_t,
        ::core::mem::size_of::<operator_floor_plan_t>(),
    ) as *mut operator_floor_plan_t;
    let mut authorized: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut authorized_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut plan_count: ::core::ffi::c_int = if !plans.is_null() {
        db_operator_floor_plan_list(plans, OPERATOR_FLOOR_PLAN_MAX_VISIBLE)
    } else {
        -1 as ::core::ffi::c_int
    };
    if plan_count < 0 as ::core::ffi::c_int
        || !load_authorized_cameras(
            &raw mut user,
            authorization_action_t::AUTHZ_LIVE_VIEW,
            &raw mut authorized,
            &raw mut authorized_count,
        )
    {
        free(plans as *mut ::core::ffi::c_void);
        free(authorized as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to list floor plans\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = if !root.is_null() {
        cJSON_AddArrayToObject(root, b"plans\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut placements: *mut operator_floor_plan_camera_t = calloc(
        OPERATOR_FLOOR_PLAN_MAX_CAMERAS as size_t,
        ::core::mem::size_of::<operator_floor_plan_camera_t>(),
    )
        as *mut operator_floor_plan_camera_t;
    if root.is_null() || items.is_null() || placements.is_null() {
        cJSON_Delete(root);
        free(plans as *mut ::core::ffi::c_void);
        free(authorized as *mut ::core::ffi::c_void);
        free(placements as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Memory allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < plan_count {
        let mut placement_count: ::core::ffi::c_int = db_operator_floor_plan_camera_list(
            &raw mut (*plans.offset(index as isize)).uuid as *mut ::core::ffi::c_char,
            placements,
            OPERATOR_FLOOR_PLAN_MAX_CAMERAS,
        );
        if placement_count >= 0 as ::core::ffi::c_int {
            let mut sketch_json: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            if (*plans.offset(index as isize)).sketch_bytes > 0 as ::core::ffi::c_int {
                db_operator_floor_plan_sketch_load(
                    &raw mut (*plans.offset(index as isize)).uuid as *mut ::core::ffi::c_char,
                    &raw mut sketch_json,
                );
            }
            let mut item: *mut cJSON = plan_json(
                plans.offset(index as isize),
                placements,
                placement_count,
                authorized,
                authorized_count,
                sketch_json,
            );
            free(sketch_json as *mut ::core::ffi::c_void);
            if !item.is_null() {
                cJSON_AddItemToArray(items, item);
            }
        }
        index += 1;
    }
    cJSON_AddBoolToObject(
        root,
        b"can_modify\0".as_ptr() as *const ::core::ffi::c_char,
        can_modify(&raw mut user) as cJSON_bool,
    );
    send_json(res, 200 as ::core::ffi::c_int, root);
    cJSON_Delete(root);
    free(placements as *mut ::core::ffi::c_void);
    free(plans as *mut ::core::ffi::c_void);
    free(authorized as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_operator_floor_plan(
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
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !authenticate(req, res, &raw mut user)
        || !extract_uuid(
            req,
            &raw mut uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            res,
        )
    {
        return;
    }
    let mut json: *mut cJSON = load_one_json(
        &raw mut uuid as *mut ::core::ffi::c_char,
        &raw mut user,
        res,
    );
    if json.is_null() {
        return;
    }
    cJSON_AddBoolToObject(
        json,
        b"can_modify\0".as_ptr() as *const ::core::ffi::c_char,
        can_modify(&raw mut user) as cJSON_bool,
    );
    send_json(res, 200 as ::core::ffi::c_int, json);
    cJSON_Delete(json);
}
unsafe extern "C" fn placements_allowed(
    mut user: *const user_t,
    mut placements: *const operator_floor_plan_camera_t,
    mut placement_count: ::core::ffi::c_int,
    mut res: *mut http_response_t,
) -> bool {
    let mut authorized: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut authorized_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !load_authorized_cameras(
        user,
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        &raw mut authorized,
        &raw mut authorized_count,
    ) {
        free(authorized as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Authorization evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut allowed: bool = r#true != 0;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < placement_count {
        if !camera_allowed(
            &raw const (*placements.offset(index as isize)).camera_uuid
                as *const ::core::ffi::c_char,
            authorized,
            authorized_count,
        ) {
            allowed = r#false != 0;
            break;
        } else {
            index += 1;
        }
    }
    free(authorized as *mut ::core::ffi::c_void);
    if !allowed {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Floor plan contains an unavailable camera\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return allowed;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_operator_floor_plan(
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
    if !require_modify(req, res, &raw mut user) {
        return;
    }
    let mut body: *mut cJSON =
        cJSON_ParseWithLength((*req).body as *const ::core::ffi::c_char, (*req).body_len);
    let mut plan: operator_floor_plan_t = operator_floor_plan_t {
        uuid: [0; 37],
        name: [0; 128],
        location_uuid: [0; 37],
        parent_plan_uuid: [0; 37],
        canvas_width: 0,
        canvas_height: 0,
        background_mime: [0; 16],
        sketch_bytes: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut placements: *mut operator_floor_plan_camera_t =
        ::core::ptr::null_mut::<operator_floor_plan_camera_t>();
    let mut placement_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sketch_json: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !parse_plan(
        body,
        r#true != 0,
        &raw mut plan,
        &raw mut placements,
        &raw mut placement_count,
        res,
    ) || !parse_sketch(body, &raw mut sketch_json, res)
        || !placements_allowed(&raw mut user, placements, placement_count, res)
    {
        free(sketch_json as *mut ::core::ffi::c_void);
        free(placements as *mut ::core::ffi::c_void);
        cJSON_Delete(body);
        return;
    }
    let mut result: db_operator_floor_plan_result_t =
        db_operator_floor_plan_create(&raw mut plan, placements, placement_count, sketch_json);
    free(sketch_json as *mut ::core::ffi::c_void);
    free(placements as *mut ::core::ffi::c_void);
    cJSON_Delete(body);
    if result.0 != db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK.0 {
        set_db_error(res, result);
        return;
    }
    let mut json: *mut cJSON = load_one_json(
        &raw mut plan.uuid as *mut ::core::ffi::c_char,
        &raw mut user,
        res,
    );
    if json.is_null() {
        return;
    }
    send_json(res, 201 as ::core::ffi::c_int, json);
    cJSON_Delete(json);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_operator_floor_plan(
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
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !require_modify(req, res, &raw mut user)
        || !extract_uuid(
            req,
            &raw mut uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            res,
        )
    {
        return;
    }
    let mut body: *mut cJSON =
        cJSON_ParseWithLength((*req).body as *const ::core::ffi::c_char, (*req).body_len);
    let mut plan: operator_floor_plan_t = operator_floor_plan_t {
        uuid: [0; 37],
        name: [0; 128],
        location_uuid: [0; 37],
        parent_plan_uuid: [0; 37],
        canvas_width: 0,
        canvas_height: 0,
        background_mime: [0; 16],
        sketch_bytes: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut placements: *mut operator_floor_plan_camera_t =
        ::core::ptr::null_mut::<operator_floor_plan_camera_t>();
    let mut placement_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut revision: int64_t = 0 as int64_t;
    let mut sketch_json: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !parse_plan(
        body,
        r#false != 0,
        &raw mut plan,
        &raw mut placements,
        &raw mut placement_count,
        res,
    ) || !parse_revision(body, &raw mut revision)
        || !parse_sketch(body, &raw mut sketch_json, res)
        || !placements_allowed(&raw mut user, placements, placement_count, res)
    {
        if (*res).status_code < 400 as ::core::ffi::c_int {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid revision\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        free(sketch_json as *mut ::core::ffi::c_void);
        free(placements as *mut ::core::ffi::c_void);
        cJSON_Delete(body);
        return;
    }
    safe_strcpy(
        &raw mut plan.uuid as *mut ::core::ffi::c_char,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    let mut result: db_operator_floor_plan_result_t = db_operator_floor_plan_update(
        &raw mut plan,
        placements,
        placement_count,
        revision,
        sketch_json,
    );
    free(sketch_json as *mut ::core::ffi::c_void);
    free(placements as *mut ::core::ffi::c_void);
    cJSON_Delete(body);
    if result.0 != db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK.0 {
        set_db_error(res, result);
        return;
    }
    let mut json: *mut cJSON = load_one_json(
        &raw mut uuid as *mut ::core::ffi::c_char,
        &raw mut user,
        res,
    );
    if json.is_null() {
        return;
    }
    send_json(res, 200 as ::core::ffi::c_int, json);
    cJSON_Delete(json);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_operator_floor_plan(
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
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !require_modify(req, res, &raw mut user)
        || !extract_uuid(
            req,
            &raw mut uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            res,
        )
    {
        return;
    }
    let mut body: *mut cJSON =
        cJSON_ParseWithLength((*req).body as *const ::core::ffi::c_char, (*req).body_len);
    let mut revision: int64_t = 0 as int64_t;
    if cJSON_IsObject(body) == 0 || !parse_revision(body, &raw mut revision) {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid revision\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_Delete(body);
    let mut result: db_operator_floor_plan_result_t =
        db_operator_floor_plan_delete(&raw mut uuid as *mut ::core::ffi::c_char, revision);
    if result.0 != db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK.0 {
        set_db_error(res, result);
        return;
    }
    remove_background_files(&raw mut uuid as *mut ::core::ffi::c_char);
    let mut json: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        json,
        b"deleted\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    send_json(res, 200 as ::core::ffi::c_int, json);
    cJSON_Delete(json);
}
unsafe extern "C" fn background_extension(
    mut mime: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if strcmp(mime, b"image/png\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return b".png\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcmp(mime, b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return b".jpg\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn background_mime_from_content_type(
    mut value: *const ::core::ffi::c_char,
    mut mime: *mut *const ::core::ffi::c_char,
) -> bool {
    let mut length: size_t = 0;
    if value.is_null() {
        return r#false != 0;
    }
    if strncasecmp(
        value,
        b"image/png\0".as_ptr() as *const ::core::ffi::c_char,
        9 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        *mime = b"image/png\0".as_ptr() as *const ::core::ffi::c_char;
        length = 9 as size_t;
    } else if strncasecmp(
        value,
        b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char,
        10 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        *mime = b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char;
        length = 10 as size_t;
    } else {
        return r#false != 0;
    }
    let mut suffix: *const ::core::ffi::c_char = value.offset(length as isize);
    while *(*__ctype_b_loc()).offset(*suffix as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        != 0
    {
        suffix = suffix.offset(1);
    }
    return *suffix as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || *suffix as ::core::ffi::c_int == ';' as ::core::ffi::c_int;
}
unsafe extern "C" fn background_matches_mime(
    mut mime: *const ::core::ffi::c_char,
    mut body: *const ::core::ffi::c_uchar,
    mut body_len: size_t,
) -> bool {
    static mut png_signature: [::core::ffi::c_uchar; 8] = [
        0x89 as ::core::ffi::c_uchar,
        'P' as ::core::ffi::c_uchar,
        'N' as ::core::ffi::c_uchar,
        'G' as ::core::ffi::c_uchar,
        0xd as ::core::ffi::c_uchar,
        0xa as ::core::ffi::c_uchar,
        0x1a as ::core::ffi::c_uchar,
        0xa as ::core::ffi::c_uchar,
    ];
    if strcmp(mime, b"image/png\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return body_len >= ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>()
            && memcmp(
                body as *const ::core::ffi::c_void,
                &raw const png_signature as *const ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>(),
            ) == 0 as ::core::ffi::c_int;
    }
    return body_len >= 3 as size_t
        && *body.offset(0isize) as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
        && *body.offset(1isize) as ::core::ffi::c_int == 0xd8 as ::core::ffi::c_int
        && *body.offset(2isize) as ::core::ffi::c_int == 0xff as ::core::ffi::c_int;
}
unsafe extern "C" fn create_background_backup(
    mut destination: *const ::core::ffi::c_char,
    mut backup: *mut background_backup_t,
) -> bool {
    let mut length: ::core::ffi::c_int = snprintf(
        &raw mut (*backup).directory as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 672]>(),
        b"%s.backup.XXXXXX\0".as_ptr() as *const ::core::ffi::c_char,
        destination,
    );
    if length < 0 as ::core::ffi::c_int
        || length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 672]>()
        || mkdtemp(&raw mut (*backup).directory as *mut ::core::ffi::c_char).is_null()
    {
        return r#false != 0;
    }
    (*backup).created = r#true != 0;
    (*backup).descriptor = open(
        &raw mut (*backup).directory as *mut ::core::ffi::c_char,
        O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC,
    );
    let mut named: stat = stat {
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
    if (*backup).descriptor < 0 as ::core::ffi::c_int
        || fstat((*backup).descriptor, &raw mut (*backup).identity) != 0 as ::core::ffi::c_int
        || lstat(
            &raw mut (*backup).directory as *mut ::core::ffi::c_char,
            &raw mut named,
        ) != 0 as ::core::ffi::c_int
        || !((*backup).identity.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
        || (*backup).identity.st_uid != geteuid()
        || (*backup).identity.st_mode & 0o777 as __mode_t != 0o700 as __mode_t
        || named.st_dev != (*backup).identity.st_dev
        || named.st_ino != (*backup).identity.st_ino
    {
        return r#false != 0;
    }
    (*backup).validated = r#true != 0;
    length = snprintf(
        &raw mut (*backup).path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 688]>(),
        b"%s/original\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*backup).directory as *mut ::core::ffi::c_char,
    );
    return length > 0 as ::core::ffi::c_int
        && (length as size_t) < ::core::mem::size_of::<[::core::ffi::c_char; 688]>();
}
unsafe extern "C" fn finish_background_backup(mut backup: *mut background_backup_t) {
    if (*backup).created {
        let mut current: stat = stat {
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
        if !(*backup).validated
            || (*backup).descriptor < 0 as ::core::ffi::c_int
            || lstat(
                &raw mut (*backup).directory as *mut ::core::ffi::c_char,
                &raw mut current,
            ) != 0 as ::core::ffi::c_int
            || current.st_dev != (*backup).identity.st_dev
            || current.st_ino != (*backup).identity.st_ino
            || rmdir(&raw mut (*backup).directory as *mut ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Floor-plan backup directory retained for inspection: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*backup).directory as *mut ::core::ffi::c_char,
            );
        }
    }
    if (*backup).descriptor >= 0 as ::core::ffi::c_int {
        close((*backup).descriptor);
    }
    (*backup).descriptor = -1 as ::core::ffi::c_int;
    if (*backup).parent >= 0 as ::core::ffi::c_int {
        close((*backup).parent);
    }
    if (*backup).old_file >= 0 as ::core::ffi::c_int {
        close((*backup).old_file);
    }
    if (*backup).upload_file >= 0 as ::core::ffi::c_int {
        close((*backup).upload_file);
    }
    (*backup).upload_file = -1 as ::core::ffi::c_int;
    (*backup).old_file = (*backup).upload_file;
    (*backup).parent = (*backup).old_file;
}
unsafe extern "C" fn background_same_inode(mut a: *const stat, mut b: *const stat) -> bool {
    return (*a).st_dev == (*b).st_dev && (*a).st_ino == (*b).st_ino;
}
unsafe extern "C" fn background_move_noreplace(
    mut from: ::core::ffi::c_int,
    mut source: *const ::core::ffi::c_char,
    mut to: ::core::ffi::c_int,
    mut destination: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return syscall(
        SYS_renameat2 as ::core::ffi::c_long,
        from,
        source,
        to,
        destination,
        1 as ::core::ffi::c_uint,
    ) as ::core::ffi::c_int;
}
unsafe extern "C" fn bind_background_original(
    mut destination: *const ::core::ffi::c_char,
    mut directory: *const ::core::ffi::c_char,
    mut backup: *mut background_backup_t,
) -> ::core::ffi::c_int {
    let mut leaf: *const ::core::ffi::c_char = strrchr(destination, '/' as ::core::ffi::c_int);
    leaf = if !leaf.is_null() {
        leaf.offset(1 as ::core::ffi::c_int as isize)
    } else {
        destination
    };
    if (*backup).parent < 0 as ::core::ffi::c_int {
        (*backup).parent = open(directory, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
    }
    let mut parent_identity: stat = stat {
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
    let mut named: stat = stat {
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
    let mut original: stat = stat {
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
    if (*backup).parent < 0 as ::core::ffi::c_int
        || fstat((*backup).parent, &raw mut parent_identity) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    (*backup).old_file = openat((*backup).parent, leaf, O_PATH | O_NOFOLLOW | O_CLOEXEC);
    if (*backup).old_file < 0 as ::core::ffi::c_int {
        return if *__errno_location() == ENOENT {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    if fstat((*backup).old_file, &raw mut original) != 0 as ::core::ffi::c_int
        || !(original.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
    {
        return -1 as ::core::ffi::c_int;
    }
    if !(*backup).created && !create_background_backup(destination, backup) {
        return -1 as ::core::ffi::c_int;
    }
    let mut private_leaf: *const ::core::ffi::c_char = strrchr(
        &raw mut (*backup).directory as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    private_leaf = if !private_leaf.is_null() {
        private_leaf.offset(1 as ::core::ffi::c_int as isize)
    } else {
        &raw mut (*backup).directory as *mut ::core::ffi::c_char
    };
    if lstat(directory, &raw mut named) != 0 as ::core::ffi::c_int
        || !background_same_inode(&raw mut parent_identity, &raw mut named)
        || fstatat(
            (*backup).parent,
            private_leaf,
            &raw mut named,
            AT_SYMLINK_NOFOLLOW,
        ) != 0 as ::core::ffi::c_int
        || !background_same_inode(&raw mut (*backup).identity, &raw mut named)
    {
        return -1 as ::core::ffi::c_int;
    }
    if background_move_noreplace(
        (*backup).parent,
        leaf,
        (*backup).descriptor,
        b"original\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if fstatat(
        (*backup).descriptor,
        b"original\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut named,
        AT_SYMLINK_NOFOLLOW,
    ) != 0 as ::core::ffi::c_int
        || !(named.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
        || !background_same_inode(&raw mut original, &raw mut named)
        || original.st_size != named.st_size
        || original.st_mtim.tv_sec != named.st_mtim.tv_sec
        || original.st_mtim.tv_nsec != named.st_mtim.tv_nsec
    {
        if background_move_noreplace(
            (*backup).descriptor,
            b"original\0".as_ptr() as *const ::core::ffi::c_char,
            (*backup).parent,
            leaf,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Changed floor-plan entry retained in private backup: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*backup).path as *mut ::core::ffi::c_char,
            );
        }
        return -1 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn background_private_matches(
    mut backup: *const background_backup_t,
    mut name: *const ::core::ffi::c_char,
    mut fd: ::core::ffi::c_int,
) -> bool {
    let mut held: stat = stat {
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
    let mut named: stat = stat {
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
    return fd >= 0 as ::core::ffi::c_int
        && fstat(fd, &raw mut held) == 0 as ::core::ffi::c_int
        && held.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
        && fstatat(
            (*backup).descriptor,
            name,
            &raw mut named,
            AT_SYMLINK_NOFOLLOW,
        ) == 0 as ::core::ffi::c_int
        && named.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
        && background_same_inode(&raw mut held, &raw mut named) as ::core::ffi::c_int != 0;
}
unsafe extern "C" fn background_remove_private(
    mut backup: *const background_backup_t,
    mut name: *const ::core::ffi::c_char,
    mut fd: ::core::ffi::c_int,
) -> bool {
    let mut named: stat = stat {
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
    if fstatat(
        (*backup).descriptor,
        name,
        &raw mut named,
        AT_SYMLINK_NOFOLLOW,
    ) != 0 as ::core::ffi::c_int
    {
        return *__errno_location() == ENOENT;
    }
    if background_private_matches(backup, name, fd) as ::core::ffi::c_int != 0
        && unlinkat((*backup).descriptor, name, 0 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Unknown or unavailable floor-plan private artifact retained: %s/%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw const (*backup).directory as *const ::core::ffi::c_char,
        name,
    );
    return r#false != 0;
}
unsafe extern "C" fn background_parent_matches(
    mut backup: *const background_backup_t,
    mut directory: *const ::core::ffi::c_char,
) -> bool {
    let mut named: stat = stat {
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
    return (*backup).parent >= 0 as ::core::ffi::c_int
        && lstat(directory, &raw mut named) == 0 as ::core::ffi::c_int
        && named.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
        && background_same_inode(&raw const (*backup).parent_identity, &raw mut named)
            as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn background_publication_matches(
    mut backup: *const background_backup_t,
    mut leaf: *const ::core::ffi::c_char,
    mut directory: *const ::core::ffi::c_char,
) -> bool {
    let mut held: stat = stat {
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
    let mut named: stat = stat {
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
    return background_parent_matches(backup, directory) as ::core::ffi::c_int != 0
        && fstat((*backup).upload_file, &raw mut held) == 0 as ::core::ffi::c_int
        && fstatat((*backup).parent, leaf, &raw mut named, AT_SYMLINK_NOFOLLOW)
            == 0 as ::core::ffi::c_int
        && named.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
        && background_same_inode(&raw mut held, &raw mut named) as ::core::ffi::c_int != 0
        && held.st_size == (*backup).upload_identity.st_size
        && held.st_mtim.tv_sec == (*backup).upload_identity.st_mtim.tv_sec
        && held.st_mtim.tv_nsec == (*backup).upload_identity.st_mtim.tv_nsec;
}
unsafe extern "C" fn restore_background_original(
    mut backup: *const background_backup_t,
    mut leaf: *const ::core::ffi::c_char,
) -> bool {
    if background_private_matches(
        backup,
        b"original\0".as_ptr() as *const ::core::ffi::c_char,
        (*backup).old_file,
    ) as ::core::ffi::c_int
        != 0
        && background_move_noreplace(
            (*backup).descriptor,
            b"original\0".as_ptr() as *const ::core::ffi::c_char,
            (*backup).parent,
            leaf,
        ) == 0 as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Floor-plan original retained after restoration conflict/failure: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw const (*backup).path as *const ::core::ffi::c_char,
    );
    return r#false != 0;
}
unsafe extern "C" fn recover_background_publication(
    mut backup: *const background_backup_t,
    mut leaf: *const ::core::ffi::c_char,
) {
    if background_move_noreplace(
        (*backup).parent,
        leaf,
        (*backup).descriptor,
        b"failed-upload\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        if *__errno_location() != ENOENT {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Floor-plan failed publication retained at public entry: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                leaf,
            );
        }
        return;
    }
    if background_private_matches(
        backup,
        b"failed-upload\0".as_ptr() as *const ::core::ffi::c_char,
        (*backup).upload_file,
    ) {
        background_remove_private(
            backup,
            b"failed-upload\0".as_ptr() as *const ::core::ffi::c_char,
            (*backup).upload_file,
        );
    } else if background_move_noreplace(
        (*backup).descriptor,
        b"failed-upload\0".as_ptr() as *const ::core::ffi::c_char,
        (*backup).parent,
        leaf,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Changed floor-plan publication retained privately: %s/failed-upload\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*backup).directory as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn background_file_path(
    mut uuid: *const ::core::ffi::c_char,
    mut mime: *const ::core::ffi::c_char,
    mut path: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> bool {
    let mut extension: *const ::core::ffi::c_char = background_extension(mime);
    if extension.is_null() {
        return r#false != 0;
    }
    let mut length: ::core::ffi::c_int = snprintf(
        path,
        size,
        b"%s/floor_plans/%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
        uuid,
        extension,
    );
    return length >= 0 as ::core::ffi::c_int && (length as size_t) < size;
}
unsafe extern "C" fn remove_background_files_locked(mut uuid: *const ::core::ffi::c_char) {
    let mut path: [::core::ffi::c_char; 640] = [0; 640];
    if background_file_path(
        uuid,
        b"image/png\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 640]>(),
    ) {
        unlink(&raw mut path as *mut ::core::ffi::c_char);
        let mut length: size_t = strlen(&raw mut path as *mut ::core::ffi::c_char);
        if length.wrapping_add(4 as size_t) < ::core::mem::size_of::<[::core::ffi::c_char; 640]>() {
            memcpy(
                (&raw mut path as *mut ::core::ffi::c_char).offset(length as isize)
                    as *mut ::core::ffi::c_void,
                b".bak\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                5 as size_t,
            );
            unlink(&raw mut path as *mut ::core::ffi::c_char);
        }
    }
    if background_file_path(
        uuid,
        b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 640]>(),
    ) {
        unlink(&raw mut path as *mut ::core::ffi::c_char);
        let mut length_0: size_t = strlen(&raw mut path as *mut ::core::ffi::c_char);
        if length_0.wrapping_add(4 as size_t) < ::core::mem::size_of::<[::core::ffi::c_char; 640]>()
        {
            memcpy(
                (&raw mut path as *mut ::core::ffi::c_char).offset(length_0 as isize)
                    as *mut ::core::ffi::c_void,
                b".bak\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                5 as size_t,
            );
            unlink(&raw mut path as *mut ::core::ffi::c_char);
        }
    }
}
unsafe extern "C" fn remove_background_files(mut uuid: *const ::core::ffi::c_char) {
    pthread_mutex_lock(&raw mut background_files_mutex);
    remove_background_files_locked(uuid);
    pthread_mutex_unlock(&raw mut background_files_mutex);
}
unsafe extern "C" fn write_background_temporary(
    mut backup: *mut background_backup_t,
    mut body: *const ::core::ffi::c_uchar,
    mut body_len: size_t,
) -> bool {
    let mut descriptor: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut attempt: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while attempt < 8 as ::core::ffi::c_uint {
        let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
        if lightnvr_uuid_generate_v4(&raw mut uuid as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        let mut length: ::core::ffi::c_int = snprintf(
            &raw mut (*backup).upload_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"upload.tmp.%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut uuid as *mut ::core::ffi::c_char,
        );
        if length < 0 as ::core::ffi::c_int
            || length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 64]>()
        {
            return r#false != 0;
        }
        descriptor = openat(
            (*backup).descriptor,
            &raw mut (*backup).upload_name as *mut ::core::ffi::c_char,
            O_WRONLY | O_CREAT | O_EXCL | O_NOFOLLOW | O_CLOEXEC,
            0o600 as ::core::ffi::c_int,
        );
        if descriptor >= 0 as ::core::ffi::c_int || *__errno_location() != EEXIST {
            break;
        }
        attempt = attempt.wrapping_add(1);
    }
    if descriptor < 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut held: ::core::ffi::c_int = fcntl(descriptor, F_DUPFD_CLOEXEC, 0 as ::core::ffi::c_int);
    if held < 0 as ::core::ffi::c_int {
        background_remove_private(
            backup,
            &raw mut (*backup).upload_name as *mut ::core::ffi::c_char,
            descriptor,
        );
        close(descriptor);
        return r#false != 0;
    }
    let mut file: *mut FILE = fdopen(descriptor, b"wb\0".as_ptr() as *const ::core::ffi::c_char);
    if file.is_null() {
        background_remove_private(
            backup,
            &raw mut (*backup).upload_name as *mut ::core::ffi::c_char,
            held,
        );
        close(descriptor);
        close(held);
        return r#false != 0;
    }
    let mut written: bool = fwrite(
        body as *const ::core::ffi::c_void,
        1 as size_t,
        body_len,
        file,
    ) as size_t
        == body_len
        && fflush(file) == 0 as ::core::ffi::c_int
        && fsync(fileno(file)) == 0 as ::core::ffi::c_int;
    if fclose(file) != 0 as ::core::ffi::c_int {
        written = r#false != 0;
    }
    if !written {
        background_remove_private(
            backup,
            &raw mut (*backup).upload_name as *mut ::core::ffi::c_char,
            held,
        );
        close(held);
    } else {
        (*backup).upload_file = held;
    }
    return written;
}
unsafe extern "C" fn extract_background_target(
    mut req: *const http_request_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut uuid_size: size_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut param: [::core::ffi::c_char; 512] = [0; 512];
    if http_request_extract_path_param(
        req,
        b"/api/live/plans/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut param as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid floor plan path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut slash: *mut ::core::ffi::c_char = strchr(
        &raw mut param as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if slash.is_null()
        || slash == &raw mut param as *mut ::core::ffi::c_char
        || strcmp(
            slash.offset(1 as ::core::ffi::c_int as isize),
            b"background\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid floor plan path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *slash = '\0' as ::core::ffi::c_char;
    if !lightnvr_uuid_is_valid(&raw mut param as *mut ::core::ffi::c_char) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid floor plan path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        uuid,
        &raw mut param as *mut ::core::ffi::c_char,
        uuid_size,
        0 as size_t,
    );
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_operator_floor_plan_background(
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
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !authenticate(req, res, &raw mut user)
        || !extract_background_target(
            req,
            &raw mut uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            res,
        )
    {
        return;
    }
    let mut plan: operator_floor_plan_t = operator_floor_plan_t {
        uuid: [0; 37],
        name: [0; 128],
        location_uuid: [0; 37],
        parent_plan_uuid: [0; 37],
        canvas_width: 0,
        canvas_height: 0,
        background_mime: [0; 16],
        sketch_bytes: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_operator_floor_plan_result_t =
        db_operator_floor_plan_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut plan);
    if result.0 != db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK.0 {
        set_db_error(res, result);
        return;
    }
    if plan.background_mime[0usize] == 0 {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Floor plan background not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut path: [::core::ffi::c_char; 640] = [0; 640];
    let mut info: stat = stat {
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
    if !background_file_path(
        &raw mut uuid as *mut ::core::ffi::c_char,
        &raw mut plan.background_mime as *mut ::core::ffi::c_char,
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 640]>(),
    ) || lstat(&raw mut path as *mut ::core::ffi::c_char, &raw mut info)
        != 0 as ::core::ffi::c_int
        || !(info.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
    {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Floor plan background not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if http_serve_file(
        req,
        res,
        &raw mut path as *mut ::core::ffi::c_char,
        &raw mut plan.background_mime as *mut ::core::ffi::c_char,
        b"Cache-Control: private, no-store\r\nX-Content-Type-Options: nosniff\r\nContent-Security-Policy: sandbox; default-src 'none'\r\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serve floor plan background\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_operator_floor_plan_background(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut obsolete: [::core::ffi::c_char; 640] = [0; 640];
    let mut other: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut json: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
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
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !require_modify(req, res, &raw mut user)
        || !extract_background_target(
            req,
            &raw mut uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            res,
        )
    {
        return;
    }
    let mut mime: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !background_mime_from_content_type(
        http_request_get_header(
            req,
            b"Content-Type\0".as_ptr() as *const ::core::ffi::c_char,
        ),
        &raw mut mime,
    ) {
        http_response_set_json_error(
            res,
            415 as ::core::ffi::c_int,
            b"Background must be an image/png or image/jpeg file\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*req).body.is_null() || (*req).body_len < 4 as size_t {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Background image is empty\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*req).body_len > OPERATOR_FLOOR_PLAN_BACKGROUND_MAX as size_t {
        http_response_set_json_error(
            res,
            413 as ::core::ffi::c_int,
            b"Background image too large; maximum is 768 KB\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if !background_matches_mime(
        mime,
        (*req).body as *const ::core::ffi::c_uchar,
        (*req).body_len,
    ) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Background image does not match its declared type\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut directory: [::core::ffi::c_char; 640] = [0; 640];
    let mut path: [::core::ffi::c_char; 640] = [0; 640];
    let mut directory_length: ::core::ffi::c_int = snprintf(
        &raw mut directory as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 640]>(),
        b"%s/floor_plans\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
    );
    if directory_length < 0 as ::core::ffi::c_int
        || directory_length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 640]>()
        || !background_file_path(
            &raw mut uuid as *mut ::core::ffi::c_char,
            mime,
            &raw mut path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 640]>(),
        )
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to store floor plan background\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    pthread_mutex_lock(&raw mut background_files_mutex);
    let mut existing: operator_floor_plan_t = operator_floor_plan_t {
        uuid: [0; 37],
        name: [0; 128],
        location_uuid: [0; 37],
        parent_plan_uuid: [0; 37],
        canvas_width: 0,
        canvas_height: 0,
        background_mime: [0; 16],
        sketch_bytes: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_operator_floor_plan_result_t =
        db_operator_floor_plan_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut existing);
    if result.0 != db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK.0 {
        pthread_mutex_unlock(&raw mut background_files_mutex);
        set_db_error(res, result);
        return;
    }
    let mut backup: background_backup_t = background_backup_t {
        directory: [0; 672],
        path: [0; 688],
        descriptor: -1 as ::core::ffi::c_int,
        parent: -1 as ::core::ffi::c_int,
        old_file: -1 as ::core::ffi::c_int,
        upload_file: -1 as ::core::ffi::c_int,
        upload_name: [0; 64],
        upload_identity: stat {
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
        },
        parent_identity: stat {
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
        },
        created: false,
        validated: false,
        identity: stat {
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
        },
    };
    let mut backed_up: bool = r#false != 0;
    let mut published: bool = r#false != 0;
    let mut leaf: *const ::core::ffi::c_char = strrchr(
        &raw mut path as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    leaf = if !leaf.is_null() {
        leaf.offset(1 as ::core::ffi::c_int as isize)
    } else {
        &raw mut path as *mut ::core::ffi::c_char
    };
    '_background_failure: {
        if mkdir_recursive(&raw mut directory as *mut ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            backup.parent = open(
                &raw mut directory as *mut ::core::ffi::c_char,
                O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC,
            );
            if !(backup.parent < 0 as ::core::ffi::c_int
                || fstat(backup.parent, &raw mut backup.parent_identity) != 0 as ::core::ffi::c_int
                || !create_background_backup(
                    &raw mut path as *mut ::core::ffi::c_char,
                    &raw mut backup,
                )
                || !background_parent_matches(
                    &raw mut backup,
                    &raw mut directory as *mut ::core::ffi::c_char,
                ))
            {
                if write_background_temporary(
                    &raw mut backup,
                    (*req).body as *const ::core::ffi::c_uchar,
                    (*req).body_len,
                ) {
                    if !(!background_private_matches(
                        &raw mut backup,
                        &raw mut backup.upload_name as *mut ::core::ffi::c_char,
                        backup.upload_file,
                    ) || fstat(backup.upload_file, &raw mut backup.upload_identity)
                        != 0 as ::core::ffi::c_int)
                    {
                        if strcmp(
                            &raw mut existing.background_mime as *mut ::core::ffi::c_char,
                            mime,
                        ) == 0 as ::core::ffi::c_int
                        {
                            let mut bound: ::core::ffi::c_int = bind_background_original(
                                &raw mut path as *mut ::core::ffi::c_char,
                                &raw mut directory as *mut ::core::ffi::c_char,
                                &raw mut backup,
                            );
                            if bound < 0 as ::core::ffi::c_int {
                                break '_background_failure;
                            } else {
                                backed_up = bound > 0 as ::core::ffi::c_int;
                            }
                        }
                        if !(!background_parent_matches(
                            &raw mut backup,
                            &raw mut directory as *mut ::core::ffi::c_char,
                        ) || background_move_noreplace(
                            backup.descriptor,
                            &raw mut backup.upload_name as *mut ::core::ffi::c_char,
                            backup.parent,
                            leaf,
                        ) != 0 as ::core::ffi::c_int)
                        {
                            published = r#true != 0;
                            if !(fsync(backup.parent) != 0 as ::core::ffi::c_int
                                || !background_publication_matches(
                                    &raw mut backup,
                                    leaf,
                                    &raw mut directory as *mut ::core::ffi::c_char,
                                ))
                            {
                                result = db_operator_floor_plan_set_background(
                                    &raw mut uuid as *mut ::core::ffi::c_char,
                                    mime,
                                );
                                if result.0
                                    == db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK.0
                                {
                                    if backed_up {
                                        background_remove_private(
                                            &raw mut backup,
                                            b"original\0".as_ptr() as *const ::core::ffi::c_char,
                                            backup.old_file,
                                        );
                                    }
                                    obsolete = [0; 640];
                                    other = if strcmp(
                                        mime,
                                        b"image/png\0".as_ptr() as *const ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char
                                    } else {
                                        b"image/png\0".as_ptr() as *const ::core::ffi::c_char
                                    };
                                    if background_file_path(
                                        &raw mut uuid as *mut ::core::ffi::c_char,
                                        other,
                                        &raw mut obsolete as *mut ::core::ffi::c_char,
                                        ::core::mem::size_of::<[::core::ffi::c_char; 640]>(),
                                    ) {
                                        unlink(&raw mut obsolete as *mut ::core::ffi::c_char);
                                    }
                                    finish_background_backup(&raw mut backup);
                                    pthread_mutex_unlock(&raw mut background_files_mutex);
                                    json = load_one_json(
                                        &raw mut uuid as *mut ::core::ffi::c_char,
                                        &raw mut user,
                                        res,
                                    );
                                    if json.is_null() {
                                        return;
                                    }
                                    send_json(res, 200 as ::core::ffi::c_int, json);
                                    cJSON_Delete(json);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if published {
        recover_background_publication(&raw mut backup, leaf);
    }
    let mut restored: bool =
        !backed_up || restore_background_original(&raw mut backup, leaf) as ::core::ffi::c_int != 0;
    if restored as ::core::ffi::c_int != 0 && backup.upload_name[0usize] as ::core::ffi::c_int != 0
    {
        background_remove_private(
            &raw mut backup,
            &raw mut backup.upload_name as *mut ::core::ffi::c_char,
            backup.upload_file,
        );
    }
    finish_background_backup(&raw mut backup);
    pthread_mutex_unlock(&raw mut background_files_mutex);
    if result.0 != db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK.0 {
        set_db_error(res, result);
    } else {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to store floor plan background\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_operator_floor_plan_background(
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
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !require_modify(req, res, &raw mut user)
        || !extract_background_target(
            req,
            &raw mut uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            res,
        )
    {
        return;
    }
    pthread_mutex_lock(&raw mut background_files_mutex);
    let mut existing: operator_floor_plan_t = operator_floor_plan_t {
        uuid: [0; 37],
        name: [0; 128],
        location_uuid: [0; 37],
        parent_plan_uuid: [0; 37],
        canvas_width: 0,
        canvas_height: 0,
        background_mime: [0; 16],
        sketch_bytes: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_operator_floor_plan_result_t =
        db_operator_floor_plan_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut existing);
    if result.0 != db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK.0 {
        pthread_mutex_unlock(&raw mut background_files_mutex);
        set_db_error(res, result);
        return;
    }
    result = db_operator_floor_plan_set_background(
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if result.0 != db_operator_floor_plan_result_t::DB_OPERATOR_FLOOR_PLAN_OK.0 {
        pthread_mutex_unlock(&raw mut background_files_mutex);
        set_db_error(res, result);
        return;
    }
    remove_background_files_locked(&raw mut uuid as *mut ::core::ffi::c_char);
    pthread_mutex_unlock(&raw mut background_files_mutex);
    let mut json: *mut cJSON = load_one_json(
        &raw mut uuid as *mut ::core::ffi::c_char,
        &raw mut user,
        res,
    );
    if json.is_null() {
        return;
    }
    send_json(res, 200 as ::core::ffi::c_int, json);
    cJSON_Delete(json);
}
