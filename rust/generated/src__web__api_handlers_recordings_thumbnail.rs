unsafe extern "C" {
    pub type __dirstream;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn rmdir(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn difftime(__time1: time_t, __time0: time_t) -> ::core::ffi::c_double;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
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
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
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
    unsafe fn thumbnail_thread_submit(
        recording_id: uint64_t,
        sample_key: int64_t,
        input_path: *const ::core::ffi::c_char,
        output_path: *const ::core::ffi::c_char,
        seek_seconds: ::core::ffi::c_double,
        deferred_action: deferred_action_handle_t,
        callback: deferred_response_callback_t,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn ensure_dir(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn libuv_serve_file(
        conn: *mut libuv_connection_t,
        path: *const ::core::ffi::c_char,
        content_type: *const ::core::ffi::c_char,
        extra_headers: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn libuv_send_response_ex(
        conn: *mut libuv_connection_t,
        response: *const http_response_t,
        action: write_complete_action_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_recording_metadata_by_id(
        id: uint64_t,
        metadata: *mut recording_metadata_t,
    ) -> ::core::ffi::c_int;
    unsafe fn storage_source_resolve(
        recording_id: uint64_t,
        path: *mut ::core::ffi::c_char,
        error: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn recording_source_add_status(object: *mut cJSON, recording_id: uint64_t);
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    unsafe fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
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
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ssize_t = isize;
pub type time_t = __time_t;
pub type int32_t = i32;
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
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::core::ffi::c_uint,
    pub __writers: ::core::ffi::c_uint,
    pub __wrphase_futex: ::core::ffi::c_uint,
    pub __writers_futex: ::core::ffi::c_uint,
    pub __pad3: ::core::ffi::c_uint,
    pub __pad4: ::core::ffi::c_uint,
    pub __cur_writer: ::core::ffi::c_int,
    pub __shared: ::core::ffi::c_int,
    pub __pad1: ::core::ffi::c_ulong,
    pub __pad2: ::core::ffi::c_ulong,
    pub __flags: ::core::ffi::c_uint,
}
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type DIR = __dirstream;
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
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type uintptr_t = usize;
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
pub type request_handler_t =
    Option<unsafe extern "C" fn(*const http_request_t, *mut http_response_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thumbnail_request_t {
    pub recording_id: uint64_t,
    pub sample_key: int64_t,
    pub thumb_path: *const ::core::ffi::c_char,
    pub cache_directory: *const ::core::ffi::c_char,
    pub seek_seconds: ::core::ffi::c_double,
    pub allow_generation: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_metadata_t {
    pub id: uint64_t,
    pub stream_name: [::core::ffi::c_char; 64],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub file_path: [::core::ffi::c_char; 512],
    pub storage_target_uuid: [::core::ffi::c_char; 37],
    pub object_key: [::core::ffi::c_char; 512],
    pub placement_reason: [::core::ffi::c_char; 64],
    pub storage_policy_version: int64_t,
    pub start_time: time_t,
    pub end_time: time_t,
    pub size_bytes: uint64_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub fps: ::core::ffi::c_int,
    pub codec: [::core::ffi::c_char; 16],
    pub is_complete: bool,
    pub playback_issue: [::core::ffi::c_char; 32],
    pub trigger_type: [::core::ffi::c_char; 16],
    pub protected: bool,
    pub retention_override_days: ::core::ffi::c_int,
    pub retention_tier: ::core::ffi::c_int,
    pub disk_pressure_eligible: bool,
    pub schedule_restricted: ::core::ffi::c_int,
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
pub type libuv_connection_t = libuv_connection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libuv_connection {
    pub handle: uv_tcp_t,
    pub read_buf: uv_buf_t,
    pub recv_buffer: *mut ::core::ffi::c_char,
    pub recv_buffer_size: size_t,
    pub recv_buffer_used: size_t,
    pub parser: llhttp_t,
    pub settings: llhttp_settings_t,
    pub request: http_request_t,
    pub response: http_response_t,
    pub body_offset: size_t,
    pub body_present: bool,
    pub server: *mut libuv_server_t,
    pub tls_session: *mut ::core::ffi::c_void,
    pub headers_complete: bool,
    pub message_complete: bool,
    pub current_header_field: [::core::ffi::c_char; 128],
    pub current_header_field_len: size_t,
    pub keep_alive: bool,
    pub requests_handled: ::core::ffi::c_int,
    pub async_response_pending: bool,
    pub handler_on_worker: bool,
    pub close_completed: bool,
    pub go2rtc_websocket: *mut ::core::ffi::c_void,
    pub archive_stream: *mut ::core::ffi::c_void,
    pub deferred_file_serve: bool,
    pub deferred_file_path: [::core::ffi::c_char; 512],
    pub deferred_content_type: [::core::ffi::c_char; 128],
    pub deferred_extra_headers: [::core::ffi::c_char; 512],
    pub deferred_action: write_complete_action_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct write_complete_action_t(pub ::core::ffi::c_uint);
impl write_complete_action_t {
    pub const WRITE_ACTION_NONE: Self = Self(0);
    pub const WRITE_ACTION_KEEP_ALIVE: Self = Self(1);
    pub const WRITE_ACTION_CLOSE: Self = Self(2);
}
pub type libuv_server_t = libuv_server;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libuv_server {
    pub r#loop: *mut uv_loop_t,
    pub listener: uv_tcp_t,
    pub stop_async: uv_async_t,
    pub config: http_server_config_t,
    pub running: bool,
    pub owns_loop: bool,
    pub shutting_down: bool,
    pub handlers: *mut C2Rust_Unnamed,
    pub handler_count: ::core::ffi::c_int,
    pub handler_capacity: ::core::ffi::c_int,
    pub tls_ctx: *mut ::core::ffi::c_void,
    pub thread: uv_thread_t,
    pub thread_running: bool,
}
pub type uv_thread_t = pthread_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub path: [::core::ffi::c_char; 256],
    pub method: [::core::ffi::c_char; 16],
    pub handler: request_handler_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_server_config_t {
    pub port: ::core::ffi::c_int,
    pub bind_ip: *const ::core::ffi::c_char,
    pub web_root: *const ::core::ffi::c_char,
    pub auth_enabled: bool,
    pub username: [::core::ffi::c_char; 32],
    pub password: [::core::ffi::c_char; 32],
    pub cors_enabled: bool,
    pub allowed_origins: [::core::ffi::c_char; 256],
    pub allowed_methods: [::core::ffi::c_char; 256],
    pub allowed_headers: [::core::ffi::c_char; 256],
    pub ssl_enabled: bool,
    pub cert_path: [::core::ffi::c_char; 512],
    pub key_path: [::core::ffi::c_char; 512],
    pub max_connections: ::core::ffi::c_int,
    pub connection_timeout: ::core::ffi::c_int,
    pub daemon_mode: bool,
    pub pid_file: [::core::ffi::c_char; 512],
}
pub type uv_async_t = uv_async_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_async_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#loop: *mut uv_loop_t,
    pub r#type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2Rust_Unnamed_5,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub async_cb: uv_async_cb,
    pub queue: uv__queue,
    pub pending: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__queue {
    pub next: *mut uv__queue,
    pub prev: *mut uv__queue,
}
pub type uv_async_cb = Option<unsafe extern "C" fn(*mut uv_async_t) -> ()>;
pub type uv_handle_t = uv_handle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_handle_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#loop: *mut uv_loop_t,
    pub r#type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2Rust_Unnamed_0,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_close_cb = Option<unsafe extern "C" fn(*mut uv_handle_t) -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct uv_handle_type(pub ::core::ffi::c_uint);
impl uv_handle_type {
    pub const UV_UNKNOWN_HANDLE: Self = Self(0);
    pub const UV_ASYNC: Self = Self(1);
    pub const UV_CHECK: Self = Self(2);
    pub const UV_FS_EVENT: Self = Self(3);
    pub const UV_FS_POLL: Self = Self(4);
    pub const UV_HANDLE: Self = Self(5);
    pub const UV_IDLE: Self = Self(6);
    pub const UV_NAMED_PIPE: Self = Self(7);
    pub const UV_POLL: Self = Self(8);
    pub const UV_PREPARE: Self = Self(9);
    pub const UV_PROCESS: Self = Self(10);
    pub const UV_STREAM: Self = Self(11);
    pub const UV_TCP: Self = Self(12);
    pub const UV_TIMER: Self = Self(13);
    pub const UV_TTY: Self = Self(14);
    pub const UV_UDP: Self = Self(15);
    pub const UV_SIGNAL: Self = Self(16);
    pub const UV_FILE: Self = Self(17);
    pub const UV_HANDLE_TYPE_MAX: Self = Self(18);
}
pub type uv_loop_t = uv_loop_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_loop_s {
    pub data: *mut ::core::ffi::c_void,
    pub active_handles: ::core::ffi::c_uint,
    pub handle_queue: uv__queue,
    pub active_reqs: C2Rust_Unnamed_4,
    pub internal_fields: *mut ::core::ffi::c_void,
    pub stop_flag: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_ulong,
    pub backend_fd: ::core::ffi::c_int,
    pub pending_queue: uv__queue,
    pub watcher_queue: uv__queue,
    pub watchers: *mut *mut uv__io_t,
    pub nwatchers: ::core::ffi::c_uint,
    pub nfds: ::core::ffi::c_uint,
    pub wq: uv__queue,
    pub wq_mutex: uv_mutex_t,
    pub wq_async: uv_async_t,
    pub cloexec_lock: uv_rwlock_t,
    pub closing_handles: *mut uv_handle_t,
    pub process_handles: uv__queue,
    pub prepare_handles: uv__queue,
    pub check_handles: uv__queue,
    pub idle_handles: uv__queue,
    pub async_handles: uv__queue,
    pub async_unused: Option<unsafe extern "C" fn() -> ()>,
    pub async_io_watcher: uv__io_t,
    pub async_wfd: ::core::ffi::c_int,
    pub timer_heap: C2Rust_Unnamed_3,
    pub timer_counter: uint64_t,
    pub time: uint64_t,
    pub signal_pipefd: [::core::ffi::c_int; 2],
    pub signal_io_watcher: uv__io_t,
    pub child_watcher: uv_signal_t,
    pub emfile_fd: ::core::ffi::c_int,
    pub inotify_read_watcher: uv__io_t,
    pub inotify_watchers: *mut ::core::ffi::c_void,
    pub inotify_fd: ::core::ffi::c_int,
}
pub type uv__io_t = uv__io_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__io_s {
    pub bits: uintptr_t,
    pub pending_queue: uv__queue,
    pub watcher_queue: uv__queue,
    pub pevents: ::core::ffi::c_uint,
    pub events: ::core::ffi::c_uint,
    pub fd: ::core::ffi::c_int,
}
pub type uv_signal_t = uv_signal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_signal_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#loop: *mut uv_loop_t,
    pub r#type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2Rust_Unnamed_2,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub signal_cb: uv_signal_cb,
    pub signum: ::core::ffi::c_int,
    pub tree_entry: C2Rust_Unnamed_1,
    pub caught_signals: ::core::ffi::c_uint,
    pub dispatched_signals: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub rbe_left: *mut uv_signal_s,
    pub rbe_right: *mut uv_signal_s,
    pub rbe_parent: *mut uv_signal_s,
    pub rbe_color: ::core::ffi::c_int,
}
pub type uv_signal_cb = Option<unsafe extern "C" fn(*mut uv_signal_t, ::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub min: *mut ::core::ffi::c_void,
    pub nelts: ::core::ffi::c_uint,
}
pub type uv_rwlock_t = pthread_rwlock_t;
pub type uv_mutex_t = pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub unused: *mut ::core::ffi::c_void,
    pub count: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_tcp_t = uv_tcp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_tcp_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#loop: *mut uv_loop_t,
    pub r#type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2Rust_Unnamed_9,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub write_queue_size: size_t,
    pub alloc_cb: uv_alloc_cb,
    pub read_cb: uv_read_cb,
    pub connect_req: *mut uv_connect_t,
    pub shutdown_req: *mut uv_shutdown_t,
    pub io_watcher: uv__io_t,
    pub write_queue: uv__queue,
    pub write_completed_queue: uv__queue,
    pub connection_cb: uv_connection_cb,
    pub delayed_error: ::core::ffi::c_int,
    pub accepted_fd: ::core::ffi::c_int,
    pub queued_fds: *mut ::core::ffi::c_void,
}
pub type uv_connection_cb =
    Option<unsafe extern "C" fn(*mut uv_stream_t, ::core::ffi::c_int) -> ()>;
pub type uv_stream_t = uv_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_stream_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#loop: *mut uv_loop_t,
    pub r#type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2Rust_Unnamed_8,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub write_queue_size: size_t,
    pub alloc_cb: uv_alloc_cb,
    pub read_cb: uv_read_cb,
    pub connect_req: *mut uv_connect_t,
    pub shutdown_req: *mut uv_shutdown_t,
    pub io_watcher: uv__io_t,
    pub write_queue: uv__queue,
    pub write_completed_queue: uv__queue,
    pub connection_cb: uv_connection_cb,
    pub delayed_error: ::core::ffi::c_int,
    pub accepted_fd: ::core::ffi::c_int,
    pub queued_fds: *mut ::core::ffi::c_void,
}
pub type uv_shutdown_t = uv_shutdown_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_shutdown_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#type: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 5],
    pub c2rust_unnamed: C2Rust_Unnamed_6,
    pub handle: *mut uv_stream_t,
    pub cb: uv_shutdown_cb,
}
pub type uv_shutdown_cb =
    Option<unsafe extern "C" fn(*mut uv_shutdown_t, ::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_6 {
    pub reserved2: [*mut ::core::ffi::c_void; 1],
    pub write_extra: uv__req_write_extra_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__req_write_extra_s {
    pub nwritten: size_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct uv_req_type(pub ::core::ffi::c_uint);
impl uv_req_type {
    pub const UV_UNKNOWN_REQ: Self = Self(0);
    pub const UV_REQ: Self = Self(1);
    pub const UV_CONNECT: Self = Self(2);
    pub const UV_WRITE: Self = Self(3);
    pub const UV_SHUTDOWN: Self = Self(4);
    pub const UV_UDP_SEND: Self = Self(5);
    pub const UV_FS: Self = Self(6);
    pub const UV_WORK: Self = Self(7);
    pub const UV_GETADDRINFO: Self = Self(8);
    pub const UV_GETNAMEINFO: Self = Self(9);
    pub const UV_RANDOM: Self = Self(10);
    pub const UV_REQ_TYPE_MAX: Self = Self(11);
}
pub type uv_connect_t = uv_connect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_connect_s {
    pub data: *mut ::core::ffi::c_void,
    pub r#type: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 5],
    pub c2rust_unnamed: C2Rust_Unnamed_7,
    pub cb: uv_connect_cb,
    pub handle: *mut uv_stream_t,
    pub queue: uv__queue,
}
pub type uv_connect_cb = Option<unsafe extern "C" fn(*mut uv_connect_t, ::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub reserved2: [*mut ::core::ffi::c_void; 1],
    pub write_extra: uv__req_write_extra_s,
}
pub type uv_read_cb =
    Option<unsafe extern "C" fn(*mut uv_stream_t, ssize_t, *const uv_buf_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_buf_t {
    pub base: *mut ::core::ffi::c_char,
    pub len: size_t,
}
pub type uv_alloc_cb = Option<unsafe extern "C" fn(*mut uv_handle_t, size_t, *mut uv_buf_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type llhttp_settings_t = llhttp_settings_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct llhttp_settings_s {
    pub on_message_begin: llhttp_cb,
    pub on_protocol: llhttp_data_cb,
    pub on_url: llhttp_data_cb,
    pub on_status: llhttp_data_cb,
    pub on_method: llhttp_data_cb,
    pub on_version: llhttp_data_cb,
    pub on_header_field: llhttp_data_cb,
    pub on_header_value: llhttp_data_cb,
    pub on_chunk_extension_name: llhttp_data_cb,
    pub on_chunk_extension_value: llhttp_data_cb,
    pub on_headers_complete: llhttp_cb,
    pub on_body: llhttp_data_cb,
    pub on_message_complete: llhttp_cb,
    pub on_protocol_complete: llhttp_cb,
    pub on_url_complete: llhttp_cb,
    pub on_status_complete: llhttp_cb,
    pub on_method_complete: llhttp_cb,
    pub on_version_complete: llhttp_cb,
    pub on_header_field_complete: llhttp_cb,
    pub on_header_value_complete: llhttp_cb,
    pub on_chunk_extension_name_complete: llhttp_cb,
    pub on_chunk_extension_value_complete: llhttp_cb,
    pub on_chunk_header: llhttp_cb,
    pub on_chunk_complete: llhttp_cb,
    pub on_reset: llhttp_cb,
}
pub type llhttp_cb = Option<unsafe extern "C" fn(*mut llhttp_t) -> ::core::ffi::c_int>;
pub type llhttp_t = llhttp__internal_t;
pub type llhttp__internal_t = llhttp__internal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct llhttp__internal_s {
    pub _index: int32_t,
    pub _span_pos0: *mut ::core::ffi::c_void,
    pub _span_cb0: *mut ::core::ffi::c_void,
    pub error: int32_t,
    pub reason: *const ::core::ffi::c_char,
    pub error_pos: *const ::core::ffi::c_char,
    pub data: *mut ::core::ffi::c_void,
    pub _current: *mut ::core::ffi::c_void,
    pub content_length: uint64_t,
    pub r#type: uint8_t,
    pub method: uint8_t,
    pub http_major: uint8_t,
    pub http_minor: uint8_t,
    pub header_state: uint8_t,
    pub lenient_flags: uint16_t,
    pub upgrade: uint8_t,
    pub finish: uint8_t,
    pub flags: uint16_t,
    pub status_code: uint16_t,
    pub initial_message_completed: uint8_t,
    pub settings: *mut ::core::ffi::c_void,
}
pub type llhttp_data_cb = Option<
    unsafe extern "C" fn(*mut llhttp_t, *const ::core::ffi::c_char, size_t) -> ::core::ffi::c_int,
>;
pub type deferred_action_handle_t = *mut ::core::ffi::c_void;
pub type deferred_response_callback_t = Option<
    unsafe extern "C" fn(
        deferred_action_handle_t,
        *const ::core::ffi::c_char,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type cJSON_bool = ::core::ffi::c_int;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_10 {
    pub const STORAGE_SOURCE_READY: Self = Self(0);
    pub const STORAGE_SOURCE_PREPARING: Self = Self(1);
    pub const STORAGE_SOURCE_ERROR: Self = Self(-1);
    pub const STORAGE_SOURCE_MISSING: Self = Self(-2);
    pub const STORAGE_SOURCE_DELETING: Self = Self(-3);
}
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const LOG_COMPONENT: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"RecordingsAPI\0") };
#[inline]
unsafe extern "C" fn recording_source_for_request(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut id: uint64_t,
    mut path: *mut ::core::ffi::c_char,
) -> bool {
    let mut error: [::core::ffi::c_char; 256] = [
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
    let mut result: ::core::ffi::c_int =
        storage_source_resolve(id, path, &raw mut error as *mut ::core::ffi::c_char);
    if result == C2Rust_Unnamed_10::STORAGE_SOURCE_READY.0 {
        return r#true != 0;
    }
    if result == C2Rust_Unnamed_10::STORAGE_SOURCE_PREPARING.0 {
        let mut prepare: [::core::ffi::c_char; 8] = [0; 8];
        let mut preparing: bool = http_request_get_query_param(
            req,
            b"prepare\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut prepare as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
        ) > 0 as ::core::ffi::c_int
            && strcmp(
                &raw mut prepare as *mut ::core::ffi::c_char,
                b"1\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0;
        http_response_add_header(
            res,
            b"Retry-After\0".as_ptr() as *const ::core::ffi::c_char,
            b"2\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json(
            res,
            if preparing as ::core::ffi::c_int != 0 {
                202 as ::core::ffi::c_int
            } else {
                503 as ::core::ffi::c_int
            },
            b"{\"status\":\"preparing\",\"source\":\"archive\",\"error\":\"Preparing archived recording\"}\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        http_response_set_json_error(
            res,
            if result == C2Rust_Unnamed_10::STORAGE_SOURCE_MISSING.0 {
                404 as ::core::ffi::c_int
            } else if result == C2Rust_Unnamed_10::STORAGE_SOURCE_DELETING.0 {
                409 as ::core::ffi::c_int
            } else {
                503 as ::core::ffi::c_int
            },
            if error[0usize] as ::core::ffi::c_int != 0 {
                &raw mut error as *mut ::core::ffi::c_char
            } else {
                b"Recording source is unavailable\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    return r#false != 0;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INVESTIGATION_THUMBNAIL_OFFSET_GRANULARITY_MS: ::core::ffi::c_int =
    1000 as ::core::ffi::c_int;
pub const INVESTIGATION_THUMBNAIL_CACHE_MAX_FILES: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
unsafe extern "C" fn ensure_thumbnails_dir(
    mut storage_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut dir_path: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut dir_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/thumbnails\0".as_ptr() as *const ::core::ffi::c_char,
        storage_path,
    );
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
    if stat(&raw mut dir_path as *mut ::core::ffi::c_char, &raw mut st) == 0 as ::core::ffi::c_int
        && st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
    {
        return 0 as ::core::ffi::c_int;
    }
    if ensure_dir(&raw mut dir_path as *mut ::core::ffi::c_char) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create thumbnails directory: %s (error: %s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut dir_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn investigation_thumbnail_dir_path(
    mut storage_path: *const ::core::ffi::c_char,
    mut recording_id: uint64_t,
    mut directory: *mut ::core::ffi::c_char,
    mut directory_size: size_t,
) {
    snprintf(
        directory,
        directory_size,
        b"%s/thumbnails/investigation/%llu\0".as_ptr() as *const ::core::ffi::c_char,
        storage_path,
        recording_id as ::core::ffi::c_ulonglong,
    );
}
unsafe extern "C" fn is_investigation_thumbnail_name(mut name: *const ::core::ffi::c_char) -> bool {
    let length: size_t = strlen(name);
    if length <= 4 as size_t
        || strcmp(
            name.offset(length as isize)
                .offset(-(4 as ::core::ffi::c_int as isize)),
            b".jpg\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut i: size_t = 0 as size_t;
    while i < length.wrapping_sub(4 as size_t) {
        if (*name.offset(i as isize) as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || *name.offset(i as isize) as ::core::ffi::c_int > '9' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn evict_investigation_thumbnail_if_full(
    mut directory: *const ::core::ffi::c_char,
) {
    let mut dir: *mut DIR = opendir(directory);
    if dir.is_null() {
        return;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut oldest_name: [::core::ffi::c_char; 256] = [
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
    let mut oldest_mtime: time_t = 0 as time_t;
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        if !is_investigation_thumbnail_name(&raw mut (*entry).d_name as *mut ::core::ffi::c_char) {
            continue;
        }
        count += 1;
        let mut candidate: [::core::ffi::c_char; 512] = [0; 512];
        snprintf(
            &raw mut candidate as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            directory,
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
        );
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
        if stat(&raw mut candidate as *mut ::core::ffi::c_char, &raw mut st)
            != 0 as ::core::ffi::c_int
        {
            continue;
        }
        if oldest_name[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            || st.st_mtim.tv_sec < oldest_mtime
        {
            oldest_mtime = st.st_mtim.tv_sec as time_t;
            safe_strcpy(
                &raw mut oldest_name as *mut ::core::ffi::c_char,
                &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        }
    }
    closedir(dir);
    if count < INVESTIGATION_THUMBNAIL_CACHE_MAX_FILES
        || oldest_name[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return;
    }
    let mut victim: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut victim as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
        directory,
        &raw mut oldest_name as *mut ::core::ffi::c_char,
    );
    if unlink(&raw mut victim as *mut ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Evicted cached investigation thumbnail: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut victim as *mut ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn prepare_investigation_thumbnail_dir(
    mut storage_path: *const ::core::ffi::c_char,
    mut directory: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut root: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut root as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/thumbnails/investigation\0".as_ptr() as *const ::core::ffi::c_char,
        storage_path,
    );
    if ensure_dir(&raw mut root as *mut ::core::ffi::c_char) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create investigation thumbnail directory: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut root as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if ensure_dir(directory) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create recording thumbnail directory: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            directory,
        );
        return -1 as ::core::ffi::c_int;
    }
    evict_investigation_thumbnail_if_full(directory);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn thumbnail_complete_callback(
    mut handle: deferred_action_handle_t,
    mut output_path: *const ::core::ffi::c_char,
    mut result: ::core::ffi::c_int,
) {
    let mut conn: *mut libuv_connection_t = handle as *mut libuv_connection_t;
    if result == 0 as ::core::ffi::c_int && !output_path.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Serving generated thumbnail: %s\0".as_ptr() as *const ::core::ffi::c_char,
            output_path,
        );
        if libuv_serve_file(
            conn,
            output_path,
            b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char,
            b"Cache-Control: private, max-age=86400\r\n\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            http_response_set_json_error(
                &raw mut (*conn).response,
                500 as ::core::ffi::c_int,
                b"Failed to serve thumbnail\0".as_ptr() as *const ::core::ffi::c_char,
            );
            libuv_send_response_ex(conn, &raw mut (*conn).response, (*conn).deferred_action);
        }
    } else {
        http_response_set_json_error(
            &raw mut (*conn).response,
            500 as ::core::ffi::c_int,
            b"Failed to generate thumbnail\0".as_ptr() as *const ::core::ffi::c_char,
        );
        libuv_send_response_ex(conn, &raw mut (*conn).response, (*conn).deferred_action);
    };
}
unsafe extern "C" fn serve_or_generate_thumbnail(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut recording: *const recording_metadata_t,
    mut request: *const thumbnail_request_t,
) {
    let mut state: *mut cJSON = cJSON_CreateObject();
    recording_source_add_status(state, (*request).recording_id);
    let mut deleting: bool = cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        state,
        b"deletion_pending\0".as_ptr() as *const ::core::ffi::c_char,
    )) != 0;
    cJSON_Delete(state);
    if deleting {
        http_response_set_json_error(
            res,
            409 as ::core::ffi::c_int,
            b"Recording deletion is pending\0".as_ptr() as *const ::core::ffi::c_char,
        );
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
    if stat((*request).thumb_path, &raw mut st) == 0 as ::core::ffi::c_int
        && st.st_size > 0 as __off_t
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Serving cached thumbnail: %s\0".as_ptr() as *const ::core::ffi::c_char,
            (*request).thumb_path,
        );
        if http_serve_file(
            req,
            res,
            (*request).thumb_path,
            b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char,
            b"Cache-Control: private, max-age=86400\r\n\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to serve thumbnail\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    if !(*request).allow_generation {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Thumbnail generation is not permitted for this session\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut source_path: [::core::ffi::c_char; 512] = [0; 512];
    if !recording_source_for_request(
        req,
        res,
        (*request).recording_id,
        &raw mut source_path as *mut ::core::ffi::c_char,
    ) {
        return;
    }
    if stat(
        &raw mut source_path as *mut ::core::ffi::c_char,
        &raw mut st,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Recording file not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if ensure_thumbnails_dir(&raw mut g_config.storage_path as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create thumbnails directory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*request).cache_directory.is_null()
        && prepare_investigation_thumbnail_dir(
            &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
            (*request).cache_directory,
        ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create thumbnail cache directory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut conn: *mut libuv_connection_t = (*req).user_data as *mut libuv_connection_t;
    if conn.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Thumbnail request has no connection in request user_data\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Internal server error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if thumbnail_thread_submit(
        (*request).recording_id,
        (*request).sample_key,
        &raw mut source_path as *mut ::core::ffi::c_char,
        (*request).thumb_path,
        (*request).seek_seconds,
        conn as deferred_action_handle_t,
        Some(
            thumbnail_complete_callback
                as unsafe extern "C" fn(
                    deferred_action_handle_t,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_add_header(
            res,
            b"Retry-After\0".as_ptr() as *const ::core::ffi::c_char,
            b"2\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            503 as ::core::ffi::c_int,
            b"Thumbnail generation busy, try again later\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    (*conn).async_response_pending = r#true != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Submitted thumbnail generation for recording %llu sample %lld\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*request).recording_id as ::core::ffi::c_ulonglong,
        (*request).sample_key as ::core::ffi::c_longlong,
    );
}
unsafe extern "C" fn parse_thumbnail_path(
    mut req: *const http_request_t,
    mut prefix: *const ::core::ffi::c_char,
    mut recording_id: *mut uint64_t,
    mut sample_key: *mut int64_t,
) -> bool {
    let mut params: [::core::ffi::c_char; 96] = [0; 96];
    if http_request_extract_path_param(
        req,
        prefix,
        &raw mut params as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut slash: *mut ::core::ffi::c_char = strchr(
        &raw mut params as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if slash.is_null()
        || slash == &raw mut params as *mut ::core::ffi::c_char
        || *slash.offset(1isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || !strchr(
            slash.offset(1 as ::core::ffi::c_int as isize),
            '/' as ::core::ffi::c_int,
        )
        .is_null()
    {
        return r#false != 0;
    }
    *slash = '\0' as ::core::ffi::c_char;
    let mut cursor: *const ::core::ffi::c_char = &raw mut params as *mut ::core::ffi::c_char;
    while *cursor != 0 {
        if (*cursor as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || *cursor as ::core::ffi::c_int > '9' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    let mut cursor_0: *const ::core::ffi::c_char = slash.offset(1 as ::core::ffi::c_int as isize);
    while *cursor_0 != 0 {
        if (*cursor_0 as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || *cursor_0 as ::core::ffi::c_int > '9' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        cursor_0 = cursor_0.offset(1);
    }
    let mut id_end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut key_end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut parsed_id: ::core::ffi::c_ulonglong = strtoull(
        &raw mut params as *mut ::core::ffi::c_char,
        &raw mut id_end,
        10 as ::core::ffi::c_int,
    );
    if *__errno_location() != 0 as ::core::ffi::c_int
        || id_end.is_null()
        || *id_end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || parsed_id == 0 as ::core::ffi::c_ulonglong
    {
        return r#false != 0;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut parsed_key: ::core::ffi::c_longlong = strtoll(
        slash.offset(1 as ::core::ffi::c_int as isize),
        &raw mut key_end,
        10 as ::core::ffi::c_int,
    );
    if *__errno_location() != 0 as ::core::ffi::c_int
        || key_end.is_null()
        || *key_end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || parsed_key < 0 as ::core::ffi::c_longlong
    {
        return r#false != 0;
    }
    *recording_id = parsed_id as uint64_t;
    *sample_key = parsed_key as int64_t;
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_recordings_thumbnail(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if req.is_null() || res.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid parameters for handle_recordings_thumbnail\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if !g_config.generate_thumbnails {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Thumbnail generation is disabled\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut id: uint64_t = 0 as uint64_t;
    let mut parsed_index: int64_t = -1 as int64_t;
    if !parse_thumbnail_path(
        req,
        b"/api/recordings/thumbnail/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut id,
        &raw mut parsed_index,
    ) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid thumbnail path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if parsed_index > 2 as int64_t {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid thumbnail index (must be 0, 1, or 2)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut index: ::core::ffi::c_int = parsed_index as ::core::ffi::c_int;
    if index > 0 as ::core::ffi::c_int
        && g_config.thumbnails_per_recording <= 1 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Hover frames disabled (thumbnails_per_recording=1)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut recording: recording_metadata_t = recording_metadata_t {
        id: 0 as uint64_t,
        stream_name: [0; 64],
        camera_uuid: [0; 37],
        file_path: [0; 512],
        storage_target_uuid: [0; 37],
        object_key: [0; 512],
        placement_reason: [0; 64],
        storage_policy_version: 0,
        start_time: 0,
        end_time: 0,
        size_bytes: 0,
        width: 0,
        height: 0,
        fps: 0,
        codec: [0; 16],
        is_complete: false,
        playback_issue: [0; 32],
        trigger_type: [0; 16],
        protected: false,
        retention_override_days: 0,
        retention_tier: 0,
        disk_pressure_eligible: false,
        schedule_restricted: 0,
    };
    if get_recording_metadata_by_id(id, &raw mut recording) != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Recording not found\0".as_ptr() as *const ::core::ffi::c_char,
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
        authorization_action_t::AUTHZ_RECORDINGS_REPLAY,
        &raw mut recording.camera_uuid as *mut ::core::ffi::c_char,
        &raw mut recording.stream_name as *mut ::core::ffi::c_char,
        &raw mut user,
        &raw mut camera,
        &raw mut evaluation,
    ) == 0
    {
        return;
    }
    let mut thumb_path: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut thumb_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/thumbnails/%llu_%d.jpg\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
        index,
    );
    let mut duration: ::core::ffi::c_double = difftime(recording.end_time, recording.start_time);
    if duration <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        duration = 10.0f64;
    }
    let mut seek_seconds: ::core::ffi::c_double = 0.;
    match index {
        0 => {
            seek_seconds = 0.0f64;
        }
        1 => {
            seek_seconds = duration / 2.0f64;
        }
        2 => {
            seek_seconds = if duration > 2.0f64 {
                duration - 1.0f64
            } else {
                duration * 0.9f64
            };
        }
        _ => {
            seek_seconds = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    if seek_seconds >= duration {
        seek_seconds = if duration > 1.0f64 {
            duration - 1.0f64
        } else {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        };
    }
    let mut request: thumbnail_request_t = thumbnail_request_t {
        recording_id: id,
        sample_key: index as int64_t,
        thumb_path: &raw mut thumb_path as *mut ::core::ffi::c_char,
        cache_directory: ::core::ptr::null::<::core::ffi::c_char>(),
        seek_seconds: seek_seconds,
        allow_generation: r#true != 0,
    };
    serve_or_generate_thumbnail(req, res, &raw mut recording, &raw mut request);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_investigation_thumbnail(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if req.is_null() || res.is_null() {
        return;
    }
    if !g_config.generate_thumbnails {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Thumbnail generation is disabled\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_config.thumbnails_per_recording <= 1 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Thumbnail drill-down disabled (thumbnails_per_recording=1)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut id: uint64_t = 0 as uint64_t;
    let mut offset_ms: int64_t = -1 as int64_t;
    if !parse_thumbnail_path(
        req,
        b"/api/investigations/thumbnail/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut id,
        &raw mut offset_ms,
    ) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid thumbnail path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut recording: recording_metadata_t = recording_metadata_t {
        id: 0 as uint64_t,
        stream_name: [0; 64],
        camera_uuid: [0; 37],
        file_path: [0; 512],
        storage_target_uuid: [0; 37],
        object_key: [0; 512],
        placement_reason: [0; 64],
        storage_policy_version: 0,
        start_time: 0,
        end_time: 0,
        size_bytes: 0,
        width: 0,
        height: 0,
        fps: 0,
        codec: [0; 16],
        is_complete: false,
        playback_issue: [0; 32],
        trigger_type: [0; 16],
        protected: false,
        retention_override_days: 0,
        retention_tier: 0,
        disk_pressure_eligible: false,
        schedule_restricted: 0,
    };
    if get_recording_metadata_by_id(id, &raw mut recording) != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Recording not found\0".as_ptr() as *const ::core::ffi::c_char,
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
        authorization_action_t::AUTHZ_RECORDINGS_REPLAY,
        &raw mut recording.camera_uuid as *mut ::core::ffi::c_char,
        &raw mut recording.stream_name as *mut ::core::ffi::c_char,
        &raw mut user,
        &raw mut camera,
        &raw mut evaluation,
    ) == 0
    {
        return;
    }
    let mut duration: ::core::ffi::c_double = difftime(recording.end_time, recording.start_time);
    let mut duration_ms: int64_t = if duration > 0.0f64 {
        (duration * 1000.0f64) as int64_t
    } else {
        0 as int64_t
    };
    if duration_ms <= 0 as int64_t || offset_ms > duration_ms {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Thumbnail offset is outside the recording\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    offset_ms = offset_ms / INVESTIGATION_THUMBNAIL_OFFSET_GRANULARITY_MS as int64_t
        * INVESTIGATION_THUMBNAIL_OFFSET_GRANULARITY_MS as int64_t;
    let mut seek_seconds: ::core::ffi::c_double = offset_ms as ::core::ffi::c_double / 1000.0f64;
    if seek_seconds >= duration {
        seek_seconds = if duration > 1.0f64 {
            duration - 1.0f64
        } else {
            0.0f64
        };
    }
    let mut thumb_directory: [::core::ffi::c_char; 512] = [0; 512];
    investigation_thumbnail_dir_path(
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
        id,
        &raw mut thumb_directory as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    );
    let mut thumb_path: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut thumb_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/%lld.jpg\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut thumb_directory as *mut ::core::ffi::c_char,
        offset_ms as ::core::ffi::c_longlong,
    );
    let mut request: thumbnail_request_t = thumbnail_request_t {
        recording_id: id,
        sample_key: offset_ms,
        thumb_path: &raw mut thumb_path as *mut ::core::ffi::c_char,
        cache_directory: &raw mut thumb_directory as *mut ::core::ffi::c_char,
        seek_seconds: seek_seconds,
        allow_generation: strcmp(
            &raw mut user.authentication_method as *mut ::core::ffi::c_char,
            b"demo\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int,
    };
    serve_or_generate_thumbnail(req, res, &raw mut recording, &raw mut request);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_recording_thumbnails(mut recording_id: uint64_t) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int {
        let mut thumb_path: [::core::ffi::c_char; 512] = [0; 512];
        snprintf(
            &raw mut thumb_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/thumbnails/%llu_%d.jpg\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
            recording_id as ::core::ffi::c_ulonglong,
            i,
        );
        if unlink(&raw mut thumb_path as *mut ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Deleted thumbnail: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut thumb_path as *mut ::core::ffi::c_char,
            );
        }
        i += 1;
    }
    let mut directory: [::core::ffi::c_char; 512] = [0; 512];
    investigation_thumbnail_dir_path(
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
        recording_id,
        &raw mut directory as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    );
    let mut dir: *mut DIR = opendir(&raw mut directory as *mut ::core::ffi::c_char);
    if dir.is_null() {
        return;
    }
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        if !is_investigation_thumbnail_name(&raw mut (*entry).d_name as *mut ::core::ffi::c_char) {
            continue;
        }
        let mut thumb_path_0: [::core::ffi::c_char; 512] = [0; 512];
        snprintf(
            &raw mut thumb_path_0 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut directory as *mut ::core::ffi::c_char,
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
        );
        if unlink(&raw mut thumb_path_0 as *mut ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Deleted thumbnail: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut thumb_path_0 as *mut ::core::ffi::c_char,
            );
        }
    }
    closedir(dir);
    if rmdir(&raw mut directory as *mut ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Deleted thumbnail directory: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut directory as *mut ::core::ffi::c_char,
        );
    }
}
