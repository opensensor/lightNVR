unsafe extern "C" {
    pub type stream_handle_s;
    pub type AVOptionRanges;
    pub type AVOption;
    pub type AVBuffer;
    pub type AVDictionary;
    pub type AVIAMFMixPresentation;
    pub type AVIAMFAudioElement;
    pub type AVCodecTag;
    unsafe fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_detach(__th: pthread_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
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
    static mut g_config: config_t;
    unsafe fn url_apply_credentials(
        url: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        out_url: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mkdir_recursive(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn chmod_path(path: *const ::core::ffi::c_char, mode: mode_t) -> ::core::ffi::c_int;
    unsafe fn is_shutdown_initiated() -> bool;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcat(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_config(
        handle: stream_handle_t,
        config: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_by_name(name: *const ::core::ffi::c_char) -> stream_handle_t;
    unsafe fn mp4_writer_create(
        output_path: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
    ) -> *mut mp4_writer_t;
    unsafe fn mp4_writer_close(writer: *mut mp4_writer_t);
    unsafe fn mp4_writer_set_audio(writer: *mut mp4_writer_t, enable: ::core::ffi::c_int);
    unsafe fn mp4_writer_set_segment_duration(
        writer: *mut mp4_writer_t,
        segment_duration: ::core::ffi::c_int,
    );
    unsafe fn mp4_writer_start_recording_thread(
        writer: *mut mp4_writer_t,
        rtsp_url: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn mp4_writer_stop_recording_thread(writer: *mut mp4_writer_t);
    unsafe fn mp4_writer_is_recording(writer: *mut mp4_writer_t) -> ::core::ffi::c_int;
    unsafe fn mp4_writer_signal_reconnect(writer: *mut mp4_writer_t);
    unsafe fn register_mp4_writer_for_stream(
        stream_name: *const ::core::ffi::c_char,
        writer: *mut mp4_writer_t,
    ) -> ::core::ffi::c_int;
    unsafe fn unregister_mp4_writer_for_stream(stream_name: *const ::core::ffi::c_char);
    unsafe fn pthread_join_with_timeout(
        thread: pthread_t,
        retval: *mut *mut ::core::ffi::c_void,
        timeout_sec: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn get_streaming_config() -> *mut config_t;
    unsafe fn prepare_placed_mp4_recording_path(
        config: *const config_t,
        stream_name: *const ::core::ffi::c_char,
        timestamp: time_t,
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
        placement: *mut storage_placement_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mp4_segment_recorder_init();
    unsafe fn mp4_segment_recorder_cleanup();
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
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
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
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type __sig_atomic_t = ::core::ffi::c_int;
pub type mode_t = __mode_t;
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
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
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
pub type atomic_int = ::core::ffi::c_int;
pub type stream_handle_t = *mut stream_handle_s;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVMediaType(pub ::core::ffi::c_int);
impl AVMediaType {
    pub const AVMEDIA_TYPE_UNKNOWN: Self = Self(-1);
    pub const AVMEDIA_TYPE_VIDEO: Self = Self(0);
    pub const AVMEDIA_TYPE_AUDIO: Self = Self(1);
    pub const AVMEDIA_TYPE_DATA: Self = Self(2);
    pub const AVMEDIA_TYPE_SUBTITLE: Self = Self(3);
    pub const AVMEDIA_TYPE_ATTACHMENT: Self = Self(4);
    pub const AVMEDIA_TYPE_NB: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVClassCategory(pub ::core::ffi::c_uint);
impl AVClassCategory {
    pub const AV_CLASS_CATEGORY_NA: Self = Self(0);
    pub const AV_CLASS_CATEGORY_INPUT: Self = Self(1);
    pub const AV_CLASS_CATEGORY_OUTPUT: Self = Self(2);
    pub const AV_CLASS_CATEGORY_MUXER: Self = Self(3);
    pub const AV_CLASS_CATEGORY_DEMUXER: Self = Self(4);
    pub const AV_CLASS_CATEGORY_ENCODER: Self = Self(5);
    pub const AV_CLASS_CATEGORY_DECODER: Self = Self(6);
    pub const AV_CLASS_CATEGORY_FILTER: Self = Self(7);
    pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: Self = Self(8);
    pub const AV_CLASS_CATEGORY_SWSCALER: Self = Self(9);
    pub const AV_CLASS_CATEGORY_SWRESAMPLER: Self = Self(10);
    pub const AV_CLASS_CATEGORY_HWDEVICE: Self = Self(11);
    pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: Self = Self(40);
    pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: Self = Self(41);
    pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: Self = Self(42);
    pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: Self = Self(43);
    pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: Self = Self(44);
    pub const AV_CLASS_CATEGORY_DEVICE_INPUT: Self = Self(45);
    pub const AV_CLASS_CATEGORY_NB: Self = Self(46);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVClass {
    pub class_name: *const ::core::ffi::c_char,
    pub item_name:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const ::core::ffi::c_char>,
    pub option: *const AVOption,
    pub version: ::core::ffi::c_int,
    pub log_level_offset_offset: ::core::ffi::c_int,
    pub parent_log_context_offset: ::core::ffi::c_int,
    pub category: AVClassCategory,
    pub get_category: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> AVClassCategory>,
    pub query_ranges: Option<
        unsafe extern "C" fn(
            *mut *mut AVOptionRanges,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub child_next: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub child_class_iterate:
        Option<unsafe extern "C" fn(*mut *mut ::core::ffi::c_void) -> *const AVClass>,
    pub state_flags_offset: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVColorPrimaries(pub ::core::ffi::c_uint);
impl AVColorPrimaries {
    pub const AVCOL_PRI_RESERVED0: Self = Self(0);
    pub const AVCOL_PRI_BT709: Self = Self(1);
    pub const AVCOL_PRI_UNSPECIFIED: Self = Self(2);
    pub const AVCOL_PRI_RESERVED: Self = Self(3);
    pub const AVCOL_PRI_BT470M: Self = Self(4);
    pub const AVCOL_PRI_BT470BG: Self = Self(5);
    pub const AVCOL_PRI_SMPTE170M: Self = Self(6);
    pub const AVCOL_PRI_SMPTE240M: Self = Self(7);
    pub const AVCOL_PRI_FILM: Self = Self(8);
    pub const AVCOL_PRI_BT2020: Self = Self(9);
    pub const AVCOL_PRI_SMPTE428: Self = Self(10);
    pub const AVCOL_PRI_SMPTEST428_1: Self = Self(10);
    pub const AVCOL_PRI_SMPTE431: Self = Self(11);
    pub const AVCOL_PRI_SMPTE432: Self = Self(12);
    pub const AVCOL_PRI_EBU3213: Self = Self(22);
    pub const AVCOL_PRI_JEDEC_P22: Self = Self(22);
    pub const AVCOL_PRI_NB: Self = Self(23);
    pub const AVCOL_PRI_EXT_BASE: Self = Self(256);
    pub const AVCOL_PRI_V_GAMUT: Self = Self(256);
    pub const AVCOL_PRI_EXT_NB: Self = Self(257);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVColorTransferCharacteristic(pub ::core::ffi::c_uint);
impl AVColorTransferCharacteristic {
    pub const AVCOL_TRC_RESERVED0: Self = Self(0);
    pub const AVCOL_TRC_BT709: Self = Self(1);
    pub const AVCOL_TRC_UNSPECIFIED: Self = Self(2);
    pub const AVCOL_TRC_RESERVED: Self = Self(3);
    pub const AVCOL_TRC_GAMMA22: Self = Self(4);
    pub const AVCOL_TRC_GAMMA28: Self = Self(5);
    pub const AVCOL_TRC_SMPTE170M: Self = Self(6);
    pub const AVCOL_TRC_SMPTE240M: Self = Self(7);
    pub const AVCOL_TRC_LINEAR: Self = Self(8);
    pub const AVCOL_TRC_LOG: Self = Self(9);
    pub const AVCOL_TRC_LOG_SQRT: Self = Self(10);
    pub const AVCOL_TRC_IEC61966_2_4: Self = Self(11);
    pub const AVCOL_TRC_BT1361_ECG: Self = Self(12);
    pub const AVCOL_TRC_IEC61966_2_1: Self = Self(13);
    pub const AVCOL_TRC_BT2020_10: Self = Self(14);
    pub const AVCOL_TRC_BT2020_12: Self = Self(15);
    pub const AVCOL_TRC_SMPTE2084: Self = Self(16);
    pub const AVCOL_TRC_SMPTEST2084: Self = Self(16);
    pub const AVCOL_TRC_SMPTE428: Self = Self(17);
    pub const AVCOL_TRC_SMPTEST428_1: Self = Self(17);
    pub const AVCOL_TRC_ARIB_STD_B67: Self = Self(18);
    pub const AVCOL_TRC_NB: Self = Self(19);
    pub const AVCOL_TRC_EXT_BASE: Self = Self(256);
    pub const AVCOL_TRC_V_LOG: Self = Self(256);
    pub const AVCOL_TRC_EXT_NB: Self = Self(257);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVColorSpace(pub ::core::ffi::c_uint);
impl AVColorSpace {
    pub const AVCOL_SPC_RGB: Self = Self(0);
    pub const AVCOL_SPC_BT709: Self = Self(1);
    pub const AVCOL_SPC_UNSPECIFIED: Self = Self(2);
    pub const AVCOL_SPC_RESERVED: Self = Self(3);
    pub const AVCOL_SPC_FCC: Self = Self(4);
    pub const AVCOL_SPC_BT470BG: Self = Self(5);
    pub const AVCOL_SPC_SMPTE170M: Self = Self(6);
    pub const AVCOL_SPC_SMPTE240M: Self = Self(7);
    pub const AVCOL_SPC_YCGCO: Self = Self(8);
    pub const AVCOL_SPC_YCOCG: Self = Self(8);
    pub const AVCOL_SPC_BT2020_NCL: Self = Self(9);
    pub const AVCOL_SPC_BT2020_CL: Self = Self(10);
    pub const AVCOL_SPC_SMPTE2085: Self = Self(11);
    pub const AVCOL_SPC_CHROMA_DERIVED_NCL: Self = Self(12);
    pub const AVCOL_SPC_CHROMA_DERIVED_CL: Self = Self(13);
    pub const AVCOL_SPC_ICTCP: Self = Self(14);
    pub const AVCOL_SPC_IPT_C2: Self = Self(15);
    pub const AVCOL_SPC_YCGCO_RE: Self = Self(16);
    pub const AVCOL_SPC_YCGCO_RO: Self = Self(17);
    pub const AVCOL_SPC_NB: Self = Self(18);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVColorRange(pub ::core::ffi::c_uint);
impl AVColorRange {
    pub const AVCOL_RANGE_UNSPECIFIED: Self = Self(0);
    pub const AVCOL_RANGE_MPEG: Self = Self(1);
    pub const AVCOL_RANGE_JPEG: Self = Self(2);
    pub const AVCOL_RANGE_NB: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVChromaLocation(pub ::core::ffi::c_uint);
impl AVChromaLocation {
    pub const AVCHROMA_LOC_UNSPECIFIED: Self = Self(0);
    pub const AVCHROMA_LOC_LEFT: Self = Self(1);
    pub const AVCHROMA_LOC_CENTER: Self = Self(2);
    pub const AVCHROMA_LOC_TOPLEFT: Self = Self(3);
    pub const AVCHROMA_LOC_TOP: Self = Self(4);
    pub const AVCHROMA_LOC_BOTTOMLEFT: Self = Self(5);
    pub const AVCHROMA_LOC_BOTTOM: Self = Self(6);
    pub const AVCHROMA_LOC_NB: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVAlphaMode(pub ::core::ffi::c_uint);
impl AVAlphaMode {
    pub const AVALPHA_MODE_UNSPECIFIED: Self = Self(0);
    pub const AVALPHA_MODE_PREMULTIPLIED: Self = Self(1);
    pub const AVALPHA_MODE_STRAIGHT: Self = Self(2);
    pub const AVALPHA_MODE_NB: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVChannel(pub ::core::ffi::c_int);
impl AVChannel {
    pub const AV_CHAN_NONE: Self = Self(-1);
    pub const AV_CHAN_FRONT_LEFT: Self = Self(0);
    pub const AV_CHAN_FRONT_RIGHT: Self = Self(1);
    pub const AV_CHAN_FRONT_CENTER: Self = Self(2);
    pub const AV_CHAN_LOW_FREQUENCY: Self = Self(3);
    pub const AV_CHAN_BACK_LEFT: Self = Self(4);
    pub const AV_CHAN_BACK_RIGHT: Self = Self(5);
    pub const AV_CHAN_FRONT_LEFT_OF_CENTER: Self = Self(6);
    pub const AV_CHAN_FRONT_RIGHT_OF_CENTER: Self = Self(7);
    pub const AV_CHAN_BACK_CENTER: Self = Self(8);
    pub const AV_CHAN_SIDE_LEFT: Self = Self(9);
    pub const AV_CHAN_SIDE_RIGHT: Self = Self(10);
    pub const AV_CHAN_TOP_CENTER: Self = Self(11);
    pub const AV_CHAN_TOP_FRONT_LEFT: Self = Self(12);
    pub const AV_CHAN_TOP_FRONT_CENTER: Self = Self(13);
    pub const AV_CHAN_TOP_FRONT_RIGHT: Self = Self(14);
    pub const AV_CHAN_TOP_BACK_LEFT: Self = Self(15);
    pub const AV_CHAN_TOP_BACK_CENTER: Self = Self(16);
    pub const AV_CHAN_TOP_BACK_RIGHT: Self = Self(17);
    pub const AV_CHAN_STEREO_LEFT: Self = Self(29);
    pub const AV_CHAN_STEREO_RIGHT: Self = Self(30);
    pub const AV_CHAN_WIDE_LEFT: Self = Self(31);
    pub const AV_CHAN_WIDE_RIGHT: Self = Self(32);
    pub const AV_CHAN_SURROUND_DIRECT_LEFT: Self = Self(33);
    pub const AV_CHAN_SURROUND_DIRECT_RIGHT: Self = Self(34);
    pub const AV_CHAN_LOW_FREQUENCY_2: Self = Self(35);
    pub const AV_CHAN_TOP_SIDE_LEFT: Self = Self(36);
    pub const AV_CHAN_TOP_SIDE_RIGHT: Self = Self(37);
    pub const AV_CHAN_BOTTOM_FRONT_CENTER: Self = Self(38);
    pub const AV_CHAN_BOTTOM_FRONT_LEFT: Self = Self(39);
    pub const AV_CHAN_BOTTOM_FRONT_RIGHT: Self = Self(40);
    pub const AV_CHAN_SIDE_SURROUND_LEFT: Self = Self(41);
    pub const AV_CHAN_SIDE_SURROUND_RIGHT: Self = Self(42);
    pub const AV_CHAN_TOP_SURROUND_LEFT: Self = Self(43);
    pub const AV_CHAN_TOP_SURROUND_RIGHT: Self = Self(44);
    pub const AV_CHAN_BINAURAL_LEFT: Self = Self(61);
    pub const AV_CHAN_BINAURAL_RIGHT: Self = Self(62);
    pub const AV_CHAN_UNUSED: Self = Self(512);
    pub const AV_CHAN_UNKNOWN: Self = Self(768);
    pub const AV_CHAN_AMBISONIC_BASE: Self = Self(1024);
    pub const AV_CHAN_AMBISONIC_END: Self = Self(2047);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVChannelOrder(pub ::core::ffi::c_uint);
impl AVChannelOrder {
    pub const AV_CHANNEL_ORDER_UNSPEC: Self = Self(0);
    pub const AV_CHANNEL_ORDER_NATIVE: Self = Self(1);
    pub const AV_CHANNEL_ORDER_CUSTOM: Self = Self(2);
    pub const AV_CHANNEL_ORDER_AMBISONIC: Self = Self(3);
    pub const FF_CHANNEL_ORDER_NB: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChannelCustom {
    pub id: AVChannel,
    pub name: [::core::ffi::c_char; 16],
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChannelLayout {
    pub order: AVChannelOrder,
    pub nb_channels: ::core::ffi::c_int,
    pub u: C2Rust_Unnamed_0,
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub mask: uint64_t,
    pub map: *mut AVChannelCustom,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVCodecID(pub ::core::ffi::c_uint);
impl AVCodecID {
    pub const AV_CODEC_ID_NONE: Self = Self(0);
    pub const AV_CODEC_ID_MPEG1VIDEO: Self = Self(1);
    pub const AV_CODEC_ID_MPEG2VIDEO: Self = Self(2);
    pub const AV_CODEC_ID_H261: Self = Self(3);
    pub const AV_CODEC_ID_H263: Self = Self(4);
    pub const AV_CODEC_ID_RV10: Self = Self(5);
    pub const AV_CODEC_ID_RV20: Self = Self(6);
    pub const AV_CODEC_ID_MJPEG: Self = Self(7);
    pub const AV_CODEC_ID_MJPEGB: Self = Self(8);
    pub const AV_CODEC_ID_LJPEG: Self = Self(9);
    pub const AV_CODEC_ID_SP5X: Self = Self(10);
    pub const AV_CODEC_ID_JPEGLS: Self = Self(11);
    pub const AV_CODEC_ID_MPEG4: Self = Self(12);
    pub const AV_CODEC_ID_RAWVIDEO: Self = Self(13);
    pub const AV_CODEC_ID_MSMPEG4V1: Self = Self(14);
    pub const AV_CODEC_ID_MSMPEG4V2: Self = Self(15);
    pub const AV_CODEC_ID_MSMPEG4V3: Self = Self(16);
    pub const AV_CODEC_ID_WMV1: Self = Self(17);
    pub const AV_CODEC_ID_WMV2: Self = Self(18);
    pub const AV_CODEC_ID_H263P: Self = Self(19);
    pub const AV_CODEC_ID_H263I: Self = Self(20);
    pub const AV_CODEC_ID_FLV1: Self = Self(21);
    pub const AV_CODEC_ID_SVQ1: Self = Self(22);
    pub const AV_CODEC_ID_SVQ3: Self = Self(23);
    pub const AV_CODEC_ID_DVVIDEO: Self = Self(24);
    pub const AV_CODEC_ID_HUFFYUV: Self = Self(25);
    pub const AV_CODEC_ID_CYUV: Self = Self(26);
    pub const AV_CODEC_ID_H264: Self = Self(27);
    pub const AV_CODEC_ID_INDEO3: Self = Self(28);
    pub const AV_CODEC_ID_VP3: Self = Self(29);
    pub const AV_CODEC_ID_THEORA: Self = Self(30);
    pub const AV_CODEC_ID_ASV1: Self = Self(31);
    pub const AV_CODEC_ID_ASV2: Self = Self(32);
    pub const AV_CODEC_ID_FFV1: Self = Self(33);
    pub const AV_CODEC_ID_4XM: Self = Self(34);
    pub const AV_CODEC_ID_VCR1: Self = Self(35);
    pub const AV_CODEC_ID_CLJR: Self = Self(36);
    pub const AV_CODEC_ID_MDEC: Self = Self(37);
    pub const AV_CODEC_ID_ROQ: Self = Self(38);
    pub const AV_CODEC_ID_INTERPLAY_VIDEO: Self = Self(39);
    pub const AV_CODEC_ID_XAN_WC3: Self = Self(40);
    pub const AV_CODEC_ID_XAN_WC4: Self = Self(41);
    pub const AV_CODEC_ID_RPZA: Self = Self(42);
    pub const AV_CODEC_ID_CINEPAK: Self = Self(43);
    pub const AV_CODEC_ID_WS_VQA: Self = Self(44);
    pub const AV_CODEC_ID_MSRLE: Self = Self(45);
    pub const AV_CODEC_ID_MSVIDEO1: Self = Self(46);
    pub const AV_CODEC_ID_IDCIN: Self = Self(47);
    pub const AV_CODEC_ID_8BPS: Self = Self(48);
    pub const AV_CODEC_ID_SMC: Self = Self(49);
    pub const AV_CODEC_ID_FLIC: Self = Self(50);
    pub const AV_CODEC_ID_TRUEMOTION1: Self = Self(51);
    pub const AV_CODEC_ID_VMDVIDEO: Self = Self(52);
    pub const AV_CODEC_ID_MSZH: Self = Self(53);
    pub const AV_CODEC_ID_ZLIB: Self = Self(54);
    pub const AV_CODEC_ID_QTRLE: Self = Self(55);
    pub const AV_CODEC_ID_TSCC: Self = Self(56);
    pub const AV_CODEC_ID_ULTI: Self = Self(57);
    pub const AV_CODEC_ID_QDRAW: Self = Self(58);
    pub const AV_CODEC_ID_VIXL: Self = Self(59);
    pub const AV_CODEC_ID_QPEG: Self = Self(60);
    pub const AV_CODEC_ID_PNG: Self = Self(61);
    pub const AV_CODEC_ID_PPM: Self = Self(62);
    pub const AV_CODEC_ID_PBM: Self = Self(63);
    pub const AV_CODEC_ID_PGM: Self = Self(64);
    pub const AV_CODEC_ID_PGMYUV: Self = Self(65);
    pub const AV_CODEC_ID_PAM: Self = Self(66);
    pub const AV_CODEC_ID_FFVHUFF: Self = Self(67);
    pub const AV_CODEC_ID_RV30: Self = Self(68);
    pub const AV_CODEC_ID_RV40: Self = Self(69);
    pub const AV_CODEC_ID_VC1: Self = Self(70);
    pub const AV_CODEC_ID_WMV3: Self = Self(71);
    pub const AV_CODEC_ID_LOCO: Self = Self(72);
    pub const AV_CODEC_ID_WNV1: Self = Self(73);
    pub const AV_CODEC_ID_AASC: Self = Self(74);
    pub const AV_CODEC_ID_INDEO2: Self = Self(75);
    pub const AV_CODEC_ID_FRAPS: Self = Self(76);
    pub const AV_CODEC_ID_TRUEMOTION2: Self = Self(77);
    pub const AV_CODEC_ID_BMP: Self = Self(78);
    pub const AV_CODEC_ID_CSCD: Self = Self(79);
    pub const AV_CODEC_ID_MMVIDEO: Self = Self(80);
    pub const AV_CODEC_ID_ZMBV: Self = Self(81);
    pub const AV_CODEC_ID_AVS: Self = Self(82);
    pub const AV_CODEC_ID_SMACKVIDEO: Self = Self(83);
    pub const AV_CODEC_ID_NUV: Self = Self(84);
    pub const AV_CODEC_ID_KMVC: Self = Self(85);
    pub const AV_CODEC_ID_FLASHSV: Self = Self(86);
    pub const AV_CODEC_ID_CAVS: Self = Self(87);
    pub const AV_CODEC_ID_JPEG2000: Self = Self(88);
    pub const AV_CODEC_ID_VMNC: Self = Self(89);
    pub const AV_CODEC_ID_VP5: Self = Self(90);
    pub const AV_CODEC_ID_VP6: Self = Self(91);
    pub const AV_CODEC_ID_VP6F: Self = Self(92);
    pub const AV_CODEC_ID_TARGA: Self = Self(93);
    pub const AV_CODEC_ID_DSICINVIDEO: Self = Self(94);
    pub const AV_CODEC_ID_TIERTEXSEQVIDEO: Self = Self(95);
    pub const AV_CODEC_ID_TIFF: Self = Self(96);
    pub const AV_CODEC_ID_GIF: Self = Self(97);
    pub const AV_CODEC_ID_DXA: Self = Self(98);
    pub const AV_CODEC_ID_DNXHD: Self = Self(99);
    pub const AV_CODEC_ID_THP: Self = Self(100);
    pub const AV_CODEC_ID_SGI: Self = Self(101);
    pub const AV_CODEC_ID_C93: Self = Self(102);
    pub const AV_CODEC_ID_BETHSOFTVID: Self = Self(103);
    pub const AV_CODEC_ID_PTX: Self = Self(104);
    pub const AV_CODEC_ID_TXD: Self = Self(105);
    pub const AV_CODEC_ID_VP6A: Self = Self(106);
    pub const AV_CODEC_ID_AMV: Self = Self(107);
    pub const AV_CODEC_ID_VB: Self = Self(108);
    pub const AV_CODEC_ID_PCX: Self = Self(109);
    pub const AV_CODEC_ID_SUNRAST: Self = Self(110);
    pub const AV_CODEC_ID_INDEO4: Self = Self(111);
    pub const AV_CODEC_ID_INDEO5: Self = Self(112);
    pub const AV_CODEC_ID_MIMIC: Self = Self(113);
    pub const AV_CODEC_ID_RL2: Self = Self(114);
    pub const AV_CODEC_ID_ESCAPE124: Self = Self(115);
    pub const AV_CODEC_ID_DIRAC: Self = Self(116);
    pub const AV_CODEC_ID_BFI: Self = Self(117);
    pub const AV_CODEC_ID_CMV: Self = Self(118);
    pub const AV_CODEC_ID_MOTIONPIXELS: Self = Self(119);
    pub const AV_CODEC_ID_TGV: Self = Self(120);
    pub const AV_CODEC_ID_TGQ: Self = Self(121);
    pub const AV_CODEC_ID_TQI: Self = Self(122);
    pub const AV_CODEC_ID_AURA: Self = Self(123);
    pub const AV_CODEC_ID_AURA2: Self = Self(124);
    pub const AV_CODEC_ID_V210X: Self = Self(125);
    pub const AV_CODEC_ID_TMV: Self = Self(126);
    pub const AV_CODEC_ID_V210: Self = Self(127);
    pub const AV_CODEC_ID_DPX: Self = Self(128);
    pub const AV_CODEC_ID_MAD: Self = Self(129);
    pub const AV_CODEC_ID_FRWU: Self = Self(130);
    pub const AV_CODEC_ID_FLASHSV2: Self = Self(131);
    pub const AV_CODEC_ID_CDGRAPHICS: Self = Self(132);
    pub const AV_CODEC_ID_R210: Self = Self(133);
    pub const AV_CODEC_ID_ANM: Self = Self(134);
    pub const AV_CODEC_ID_BINKVIDEO: Self = Self(135);
    pub const AV_CODEC_ID_IFF_ILBM: Self = Self(136);
    pub const AV_CODEC_ID_KGV1: Self = Self(137);
    pub const AV_CODEC_ID_YOP: Self = Self(138);
    pub const AV_CODEC_ID_VP8: Self = Self(139);
    pub const AV_CODEC_ID_PICTOR: Self = Self(140);
    pub const AV_CODEC_ID_ANSI: Self = Self(141);
    pub const AV_CODEC_ID_A64_MULTI: Self = Self(142);
    pub const AV_CODEC_ID_A64_MULTI5: Self = Self(143);
    pub const AV_CODEC_ID_R10K: Self = Self(144);
    pub const AV_CODEC_ID_MXPEG: Self = Self(145);
    pub const AV_CODEC_ID_LAGARITH: Self = Self(146);
    pub const AV_CODEC_ID_PRORES: Self = Self(147);
    pub const AV_CODEC_ID_JV: Self = Self(148);
    pub const AV_CODEC_ID_DFA: Self = Self(149);
    pub const AV_CODEC_ID_WMV3IMAGE: Self = Self(150);
    pub const AV_CODEC_ID_VC1IMAGE: Self = Self(151);
    pub const AV_CODEC_ID_UTVIDEO: Self = Self(152);
    pub const AV_CODEC_ID_BMV_VIDEO: Self = Self(153);
    pub const AV_CODEC_ID_VBLE: Self = Self(154);
    pub const AV_CODEC_ID_DXTORY: Self = Self(155);
    pub const AV_CODEC_ID_XWD: Self = Self(156);
    pub const AV_CODEC_ID_CDXL: Self = Self(157);
    pub const AV_CODEC_ID_XBM: Self = Self(158);
    pub const AV_CODEC_ID_ZEROCODEC: Self = Self(159);
    pub const AV_CODEC_ID_MSS1: Self = Self(160);
    pub const AV_CODEC_ID_MSA1: Self = Self(161);
    pub const AV_CODEC_ID_TSCC2: Self = Self(162);
    pub const AV_CODEC_ID_MTS2: Self = Self(163);
    pub const AV_CODEC_ID_CLLC: Self = Self(164);
    pub const AV_CODEC_ID_MSS2: Self = Self(165);
    pub const AV_CODEC_ID_VP9: Self = Self(166);
    pub const AV_CODEC_ID_AIC: Self = Self(167);
    pub const AV_CODEC_ID_ESCAPE130: Self = Self(168);
    pub const AV_CODEC_ID_G2M: Self = Self(169);
    pub const AV_CODEC_ID_WEBP: Self = Self(170);
    pub const AV_CODEC_ID_HNM4_VIDEO: Self = Self(171);
    pub const AV_CODEC_ID_HEVC: Self = Self(172);
    pub const AV_CODEC_ID_FIC: Self = Self(173);
    pub const AV_CODEC_ID_ALIAS_PIX: Self = Self(174);
    pub const AV_CODEC_ID_BRENDER_PIX: Self = Self(175);
    pub const AV_CODEC_ID_PAF_VIDEO: Self = Self(176);
    pub const AV_CODEC_ID_EXR: Self = Self(177);
    pub const AV_CODEC_ID_VP7: Self = Self(178);
    pub const AV_CODEC_ID_SANM: Self = Self(179);
    pub const AV_CODEC_ID_SGIRLE: Self = Self(180);
    pub const AV_CODEC_ID_MVC1: Self = Self(181);
    pub const AV_CODEC_ID_MVC2: Self = Self(182);
    pub const AV_CODEC_ID_HQX: Self = Self(183);
    pub const AV_CODEC_ID_TDSC: Self = Self(184);
    pub const AV_CODEC_ID_HQ_HQA: Self = Self(185);
    pub const AV_CODEC_ID_HAP: Self = Self(186);
    pub const AV_CODEC_ID_DDS: Self = Self(187);
    pub const AV_CODEC_ID_DXV: Self = Self(188);
    pub const AV_CODEC_ID_SCREENPRESSO: Self = Self(189);
    pub const AV_CODEC_ID_RSCC: Self = Self(190);
    pub const AV_CODEC_ID_AVS2: Self = Self(191);
    pub const AV_CODEC_ID_PGX: Self = Self(192);
    pub const AV_CODEC_ID_AVS3: Self = Self(193);
    pub const AV_CODEC_ID_MSP2: Self = Self(194);
    pub const AV_CODEC_ID_VVC: Self = Self(195);
    pub const AV_CODEC_ID_Y41P: Self = Self(196);
    pub const AV_CODEC_ID_AVRP: Self = Self(197);
    pub const AV_CODEC_ID_012V: Self = Self(198);
    pub const AV_CODEC_ID_AVUI: Self = Self(199);
    pub const AV_CODEC_ID_TARGA_Y216: Self = Self(200);
    pub const AV_CODEC_ID_YUV4: Self = Self(201);
    pub const AV_CODEC_ID_AVRN: Self = Self(202);
    pub const AV_CODEC_ID_CPIA: Self = Self(203);
    pub const AV_CODEC_ID_XFACE: Self = Self(204);
    pub const AV_CODEC_ID_SNOW: Self = Self(205);
    pub const AV_CODEC_ID_SMVJPEG: Self = Self(206);
    pub const AV_CODEC_ID_APNG: Self = Self(207);
    pub const AV_CODEC_ID_DAALA: Self = Self(208);
    pub const AV_CODEC_ID_CFHD: Self = Self(209);
    pub const AV_CODEC_ID_TRUEMOTION2RT: Self = Self(210);
    pub const AV_CODEC_ID_M101: Self = Self(211);
    pub const AV_CODEC_ID_MAGICYUV: Self = Self(212);
    pub const AV_CODEC_ID_SHEERVIDEO: Self = Self(213);
    pub const AV_CODEC_ID_YLC: Self = Self(214);
    pub const AV_CODEC_ID_PSD: Self = Self(215);
    pub const AV_CODEC_ID_PIXLET: Self = Self(216);
    pub const AV_CODEC_ID_SPEEDHQ: Self = Self(217);
    pub const AV_CODEC_ID_FMVC: Self = Self(218);
    pub const AV_CODEC_ID_SCPR: Self = Self(219);
    pub const AV_CODEC_ID_CLEARVIDEO: Self = Self(220);
    pub const AV_CODEC_ID_XPM: Self = Self(221);
    pub const AV_CODEC_ID_AV1: Self = Self(222);
    pub const AV_CODEC_ID_BITPACKED: Self = Self(223);
    pub const AV_CODEC_ID_MSCC: Self = Self(224);
    pub const AV_CODEC_ID_SRGC: Self = Self(225);
    pub const AV_CODEC_ID_SVG: Self = Self(226);
    pub const AV_CODEC_ID_GDV: Self = Self(227);
    pub const AV_CODEC_ID_FITS: Self = Self(228);
    pub const AV_CODEC_ID_IMM4: Self = Self(229);
    pub const AV_CODEC_ID_PROSUMER: Self = Self(230);
    pub const AV_CODEC_ID_MWSC: Self = Self(231);
    pub const AV_CODEC_ID_WCMV: Self = Self(232);
    pub const AV_CODEC_ID_RASC: Self = Self(233);
    pub const AV_CODEC_ID_HYMT: Self = Self(234);
    pub const AV_CODEC_ID_ARBC: Self = Self(235);
    pub const AV_CODEC_ID_AGM: Self = Self(236);
    pub const AV_CODEC_ID_LSCR: Self = Self(237);
    pub const AV_CODEC_ID_VP4: Self = Self(238);
    pub const AV_CODEC_ID_IMM5: Self = Self(239);
    pub const AV_CODEC_ID_MVDV: Self = Self(240);
    pub const AV_CODEC_ID_MVHA: Self = Self(241);
    pub const AV_CODEC_ID_CDTOONS: Self = Self(242);
    pub const AV_CODEC_ID_MV30: Self = Self(243);
    pub const AV_CODEC_ID_NOTCHLC: Self = Self(244);
    pub const AV_CODEC_ID_PFM: Self = Self(245);
    pub const AV_CODEC_ID_MOBICLIP: Self = Self(246);
    pub const AV_CODEC_ID_PHOTOCD: Self = Self(247);
    pub const AV_CODEC_ID_IPU: Self = Self(248);
    pub const AV_CODEC_ID_ARGO: Self = Self(249);
    pub const AV_CODEC_ID_CRI: Self = Self(250);
    pub const AV_CODEC_ID_SIMBIOSIS_IMX: Self = Self(251);
    pub const AV_CODEC_ID_SGA_VIDEO: Self = Self(252);
    pub const AV_CODEC_ID_GEM: Self = Self(253);
    pub const AV_CODEC_ID_VBN: Self = Self(254);
    pub const AV_CODEC_ID_JPEGXL: Self = Self(255);
    pub const AV_CODEC_ID_QOI: Self = Self(256);
    pub const AV_CODEC_ID_PHM: Self = Self(257);
    pub const AV_CODEC_ID_RADIANCE_HDR: Self = Self(258);
    pub const AV_CODEC_ID_WBMP: Self = Self(259);
    pub const AV_CODEC_ID_MEDIA100: Self = Self(260);
    pub const AV_CODEC_ID_VQC: Self = Self(261);
    pub const AV_CODEC_ID_PDV: Self = Self(262);
    pub const AV_CODEC_ID_EVC: Self = Self(263);
    pub const AV_CODEC_ID_RTV1: Self = Self(264);
    pub const AV_CODEC_ID_VMIX: Self = Self(265);
    pub const AV_CODEC_ID_LEAD: Self = Self(266);
    pub const AV_CODEC_ID_DNXUC: Self = Self(267);
    pub const AV_CODEC_ID_RV60: Self = Self(268);
    pub const AV_CODEC_ID_JPEGXL_ANIM: Self = Self(269);
    pub const AV_CODEC_ID_APV: Self = Self(270);
    pub const AV_CODEC_ID_PRORES_RAW: Self = Self(271);
    pub const AV_CODEC_ID_JPEGXS: Self = Self(272);
    pub const AV_CODEC_ID_WEBP_ANIM: Self = Self(273);
    pub const AV_CODEC_ID_FIRST_AUDIO: Self = Self(65536);
    pub const AV_CODEC_ID_PCM_S16LE: Self = Self(65536);
    pub const AV_CODEC_ID_PCM_S16BE: Self = Self(65537);
    pub const AV_CODEC_ID_PCM_U16LE: Self = Self(65538);
    pub const AV_CODEC_ID_PCM_U16BE: Self = Self(65539);
    pub const AV_CODEC_ID_PCM_S8: Self = Self(65540);
    pub const AV_CODEC_ID_PCM_U8: Self = Self(65541);
    pub const AV_CODEC_ID_PCM_MULAW: Self = Self(65542);
    pub const AV_CODEC_ID_PCM_ALAW: Self = Self(65543);
    pub const AV_CODEC_ID_PCM_S32LE: Self = Self(65544);
    pub const AV_CODEC_ID_PCM_S32BE: Self = Self(65545);
    pub const AV_CODEC_ID_PCM_U32LE: Self = Self(65546);
    pub const AV_CODEC_ID_PCM_U32BE: Self = Self(65547);
    pub const AV_CODEC_ID_PCM_S24LE: Self = Self(65548);
    pub const AV_CODEC_ID_PCM_S24BE: Self = Self(65549);
    pub const AV_CODEC_ID_PCM_U24LE: Self = Self(65550);
    pub const AV_CODEC_ID_PCM_U24BE: Self = Self(65551);
    pub const AV_CODEC_ID_PCM_S24DAUD: Self = Self(65552);
    pub const AV_CODEC_ID_PCM_ZORK: Self = Self(65553);
    pub const AV_CODEC_ID_PCM_S16LE_PLANAR: Self = Self(65554);
    pub const AV_CODEC_ID_PCM_DVD: Self = Self(65555);
    pub const AV_CODEC_ID_PCM_F32BE: Self = Self(65556);
    pub const AV_CODEC_ID_PCM_F32LE: Self = Self(65557);
    pub const AV_CODEC_ID_PCM_F64BE: Self = Self(65558);
    pub const AV_CODEC_ID_PCM_F64LE: Self = Self(65559);
    pub const AV_CODEC_ID_PCM_BLURAY: Self = Self(65560);
    pub const AV_CODEC_ID_PCM_LXF: Self = Self(65561);
    pub const AV_CODEC_ID_S302M: Self = Self(65562);
    pub const AV_CODEC_ID_PCM_S8_PLANAR: Self = Self(65563);
    pub const AV_CODEC_ID_PCM_S24LE_PLANAR: Self = Self(65564);
    pub const AV_CODEC_ID_PCM_S32LE_PLANAR: Self = Self(65565);
    pub const AV_CODEC_ID_PCM_S16BE_PLANAR: Self = Self(65566);
    pub const AV_CODEC_ID_PCM_S64LE: Self = Self(65567);
    pub const AV_CODEC_ID_PCM_S64BE: Self = Self(65568);
    pub const AV_CODEC_ID_PCM_F16LE: Self = Self(65569);
    pub const AV_CODEC_ID_PCM_F24LE: Self = Self(65570);
    pub const AV_CODEC_ID_PCM_VIDC: Self = Self(65571);
    pub const AV_CODEC_ID_PCM_SGA: Self = Self(65572);
    pub const AV_CODEC_ID_ADPCM_IMA_QT: Self = Self(69632);
    pub const AV_CODEC_ID_ADPCM_IMA_WAV: Self = Self(69633);
    pub const AV_CODEC_ID_ADPCM_IMA_DK3: Self = Self(69634);
    pub const AV_CODEC_ID_ADPCM_IMA_DK4: Self = Self(69635);
    pub const AV_CODEC_ID_ADPCM_IMA_WS: Self = Self(69636);
    pub const AV_CODEC_ID_ADPCM_IMA_SMJPEG: Self = Self(69637);
    pub const AV_CODEC_ID_ADPCM_MS: Self = Self(69638);
    pub const AV_CODEC_ID_ADPCM_4XM: Self = Self(69639);
    pub const AV_CODEC_ID_ADPCM_XA: Self = Self(69640);
    pub const AV_CODEC_ID_ADPCM_ADX: Self = Self(69641);
    pub const AV_CODEC_ID_ADPCM_EA: Self = Self(69642);
    pub const AV_CODEC_ID_ADPCM_G726: Self = Self(69643);
    pub const AV_CODEC_ID_ADPCM_CT: Self = Self(69644);
    pub const AV_CODEC_ID_ADPCM_SWF: Self = Self(69645);
    pub const AV_CODEC_ID_ADPCM_YAMAHA: Self = Self(69646);
    pub const AV_CODEC_ID_ADPCM_SBPRO_4: Self = Self(69647);
    pub const AV_CODEC_ID_ADPCM_SBPRO_3: Self = Self(69648);
    pub const AV_CODEC_ID_ADPCM_SBPRO_2: Self = Self(69649);
    pub const AV_CODEC_ID_ADPCM_THP: Self = Self(69650);
    pub const AV_CODEC_ID_ADPCM_IMA_AMV: Self = Self(69651);
    pub const AV_CODEC_ID_ADPCM_EA_R1: Self = Self(69652);
    pub const AV_CODEC_ID_ADPCM_EA_R3: Self = Self(69653);
    pub const AV_CODEC_ID_ADPCM_EA_R2: Self = Self(69654);
    pub const AV_CODEC_ID_ADPCM_IMA_EA_SEAD: Self = Self(69655);
    pub const AV_CODEC_ID_ADPCM_IMA_EA_EACS: Self = Self(69656);
    pub const AV_CODEC_ID_ADPCM_EA_XAS: Self = Self(69657);
    pub const AV_CODEC_ID_ADPCM_EA_MAXIS_XA: Self = Self(69658);
    pub const AV_CODEC_ID_ADPCM_IMA_ISS: Self = Self(69659);
    pub const AV_CODEC_ID_ADPCM_G722: Self = Self(69660);
    pub const AV_CODEC_ID_ADPCM_IMA_APC: Self = Self(69661);
    pub const AV_CODEC_ID_ADPCM_VIMA: Self = Self(69662);
    pub const AV_CODEC_ID_ADPCM_AFC: Self = Self(69663);
    pub const AV_CODEC_ID_ADPCM_IMA_OKI: Self = Self(69664);
    pub const AV_CODEC_ID_ADPCM_DTK: Self = Self(69665);
    pub const AV_CODEC_ID_ADPCM_IMA_RAD: Self = Self(69666);
    pub const AV_CODEC_ID_ADPCM_G726LE: Self = Self(69667);
    pub const AV_CODEC_ID_ADPCM_THP_LE: Self = Self(69668);
    pub const AV_CODEC_ID_ADPCM_PSX: Self = Self(69669);
    pub const AV_CODEC_ID_ADPCM_AICA: Self = Self(69670);
    pub const AV_CODEC_ID_ADPCM_IMA_DAT4: Self = Self(69671);
    pub const AV_CODEC_ID_ADPCM_MTAF: Self = Self(69672);
    pub const AV_CODEC_ID_ADPCM_AGM: Self = Self(69673);
    pub const AV_CODEC_ID_ADPCM_ARGO: Self = Self(69674);
    pub const AV_CODEC_ID_ADPCM_IMA_SSI: Self = Self(69675);
    pub const AV_CODEC_ID_ADPCM_ZORK: Self = Self(69676);
    pub const AV_CODEC_ID_ADPCM_IMA_APM: Self = Self(69677);
    pub const AV_CODEC_ID_ADPCM_IMA_ALP: Self = Self(69678);
    pub const AV_CODEC_ID_ADPCM_IMA_MTF: Self = Self(69679);
    pub const AV_CODEC_ID_ADPCM_IMA_CUNNING: Self = Self(69680);
    pub const AV_CODEC_ID_ADPCM_IMA_MOFLEX: Self = Self(69681);
    pub const AV_CODEC_ID_ADPCM_IMA_ACORN: Self = Self(69682);
    pub const AV_CODEC_ID_ADPCM_XMD: Self = Self(69683);
    pub const AV_CODEC_ID_ADPCM_IMA_XBOX: Self = Self(69684);
    pub const AV_CODEC_ID_ADPCM_SANYO: Self = Self(69685);
    pub const AV_CODEC_ID_ADPCM_IMA_HVQM4: Self = Self(69686);
    pub const AV_CODEC_ID_ADPCM_IMA_PDA: Self = Self(69687);
    pub const AV_CODEC_ID_ADPCM_N64: Self = Self(69688);
    pub const AV_CODEC_ID_ADPCM_IMA_HVQM2: Self = Self(69689);
    pub const AV_CODEC_ID_ADPCM_IMA_MAGIX: Self = Self(69690);
    pub const AV_CODEC_ID_ADPCM_PSXC: Self = Self(69691);
    pub const AV_CODEC_ID_ADPCM_CIRCUS: Self = Self(69692);
    pub const AV_CODEC_ID_ADPCM_IMA_ESCAPE: Self = Self(69693);
    pub const AV_CODEC_ID_AMR_NB: Self = Self(73728);
    pub const AV_CODEC_ID_AMR_WB: Self = Self(73729);
    pub const AV_CODEC_ID_RA_144: Self = Self(77824);
    pub const AV_CODEC_ID_RA_288: Self = Self(77825);
    pub const AV_CODEC_ID_ROQ_DPCM: Self = Self(81920);
    pub const AV_CODEC_ID_INTERPLAY_DPCM: Self = Self(81921);
    pub const AV_CODEC_ID_XAN_DPCM: Self = Self(81922);
    pub const AV_CODEC_ID_SOL_DPCM: Self = Self(81923);
    pub const AV_CODEC_ID_SDX2_DPCM: Self = Self(81924);
    pub const AV_CODEC_ID_GREMLIN_DPCM: Self = Self(81925);
    pub const AV_CODEC_ID_DERF_DPCM: Self = Self(81926);
    pub const AV_CODEC_ID_WADY_DPCM: Self = Self(81927);
    pub const AV_CODEC_ID_CBD2_DPCM: Self = Self(81928);
    pub const AV_CODEC_ID_MP2: Self = Self(86016);
    pub const AV_CODEC_ID_MP3: Self = Self(86017);
    pub const AV_CODEC_ID_AAC: Self = Self(86018);
    pub const AV_CODEC_ID_AC3: Self = Self(86019);
    pub const AV_CODEC_ID_DTS: Self = Self(86020);
    pub const AV_CODEC_ID_VORBIS: Self = Self(86021);
    pub const AV_CODEC_ID_DVAUDIO: Self = Self(86022);
    pub const AV_CODEC_ID_WMAV1: Self = Self(86023);
    pub const AV_CODEC_ID_WMAV2: Self = Self(86024);
    pub const AV_CODEC_ID_MACE3: Self = Self(86025);
    pub const AV_CODEC_ID_MACE6: Self = Self(86026);
    pub const AV_CODEC_ID_VMDAUDIO: Self = Self(86027);
    pub const AV_CODEC_ID_FLAC: Self = Self(86028);
    pub const AV_CODEC_ID_MP3ADU: Self = Self(86029);
    pub const AV_CODEC_ID_MP3ON4: Self = Self(86030);
    pub const AV_CODEC_ID_SHORTEN: Self = Self(86031);
    pub const AV_CODEC_ID_ALAC: Self = Self(86032);
    pub const AV_CODEC_ID_WESTWOOD_SND1: Self = Self(86033);
    pub const AV_CODEC_ID_GSM: Self = Self(86034);
    pub const AV_CODEC_ID_QDM2: Self = Self(86035);
    pub const AV_CODEC_ID_COOK: Self = Self(86036);
    pub const AV_CODEC_ID_TRUESPEECH: Self = Self(86037);
    pub const AV_CODEC_ID_TTA: Self = Self(86038);
    pub const AV_CODEC_ID_SMACKAUDIO: Self = Self(86039);
    pub const AV_CODEC_ID_QCELP: Self = Self(86040);
    pub const AV_CODEC_ID_WAVPACK: Self = Self(86041);
    pub const AV_CODEC_ID_DSICINAUDIO: Self = Self(86042);
    pub const AV_CODEC_ID_IMC: Self = Self(86043);
    pub const AV_CODEC_ID_MUSEPACK7: Self = Self(86044);
    pub const AV_CODEC_ID_MLP: Self = Self(86045);
    pub const AV_CODEC_ID_GSM_MS: Self = Self(86046);
    pub const AV_CODEC_ID_ATRAC3: Self = Self(86047);
    pub const AV_CODEC_ID_APE: Self = Self(86048);
    pub const AV_CODEC_ID_NELLYMOSER: Self = Self(86049);
    pub const AV_CODEC_ID_MUSEPACK8: Self = Self(86050);
    pub const AV_CODEC_ID_SPEEX: Self = Self(86051);
    pub const AV_CODEC_ID_WMAVOICE: Self = Self(86052);
    pub const AV_CODEC_ID_WMAPRO: Self = Self(86053);
    pub const AV_CODEC_ID_WMALOSSLESS: Self = Self(86054);
    pub const AV_CODEC_ID_ATRAC3P: Self = Self(86055);
    pub const AV_CODEC_ID_EAC3: Self = Self(86056);
    pub const AV_CODEC_ID_SIPR: Self = Self(86057);
    pub const AV_CODEC_ID_MP1: Self = Self(86058);
    pub const AV_CODEC_ID_TWINVQ: Self = Self(86059);
    pub const AV_CODEC_ID_TRUEHD: Self = Self(86060);
    pub const AV_CODEC_ID_MP4ALS: Self = Self(86061);
    pub const AV_CODEC_ID_ATRAC1: Self = Self(86062);
    pub const AV_CODEC_ID_BINKAUDIO_RDFT: Self = Self(86063);
    pub const AV_CODEC_ID_BINKAUDIO_DCT: Self = Self(86064);
    pub const AV_CODEC_ID_AAC_LATM: Self = Self(86065);
    pub const AV_CODEC_ID_QDMC: Self = Self(86066);
    pub const AV_CODEC_ID_CELT: Self = Self(86067);
    pub const AV_CODEC_ID_G723_1: Self = Self(86068);
    pub const AV_CODEC_ID_G729: Self = Self(86069);
    pub const AV_CODEC_ID_8SVX_EXP: Self = Self(86070);
    pub const AV_CODEC_ID_8SVX_FIB: Self = Self(86071);
    pub const AV_CODEC_ID_BMV_AUDIO: Self = Self(86072);
    pub const AV_CODEC_ID_RALF: Self = Self(86073);
    pub const AV_CODEC_ID_IAC: Self = Self(86074);
    pub const AV_CODEC_ID_ILBC: Self = Self(86075);
    pub const AV_CODEC_ID_OPUS: Self = Self(86076);
    pub const AV_CODEC_ID_COMFORT_NOISE: Self = Self(86077);
    pub const AV_CODEC_ID_TAK: Self = Self(86078);
    pub const AV_CODEC_ID_METASOUND: Self = Self(86079);
    pub const AV_CODEC_ID_PAF_AUDIO: Self = Self(86080);
    pub const AV_CODEC_ID_ON2AVC: Self = Self(86081);
    pub const AV_CODEC_ID_DSS_SP: Self = Self(86082);
    pub const AV_CODEC_ID_CODEC2: Self = Self(86083);
    pub const AV_CODEC_ID_FFWAVESYNTH: Self = Self(86084);
    pub const AV_CODEC_ID_SONIC: Self = Self(86085);
    pub const AV_CODEC_ID_SONIC_LS: Self = Self(86086);
    pub const AV_CODEC_ID_EVRC: Self = Self(86087);
    pub const AV_CODEC_ID_SMV: Self = Self(86088);
    pub const AV_CODEC_ID_DSD_LSBF: Self = Self(86089);
    pub const AV_CODEC_ID_DSD_MSBF: Self = Self(86090);
    pub const AV_CODEC_ID_DSD_LSBF_PLANAR: Self = Self(86091);
    pub const AV_CODEC_ID_DSD_MSBF_PLANAR: Self = Self(86092);
    pub const AV_CODEC_ID_4GV: Self = Self(86093);
    pub const AV_CODEC_ID_INTERPLAY_ACM: Self = Self(86094);
    pub const AV_CODEC_ID_XMA1: Self = Self(86095);
    pub const AV_CODEC_ID_XMA2: Self = Self(86096);
    pub const AV_CODEC_ID_DST: Self = Self(86097);
    pub const AV_CODEC_ID_ATRAC3AL: Self = Self(86098);
    pub const AV_CODEC_ID_ATRAC3PAL: Self = Self(86099);
    pub const AV_CODEC_ID_DOLBY_E: Self = Self(86100);
    pub const AV_CODEC_ID_APTX: Self = Self(86101);
    pub const AV_CODEC_ID_APTX_HD: Self = Self(86102);
    pub const AV_CODEC_ID_SBC: Self = Self(86103);
    pub const AV_CODEC_ID_ATRAC9: Self = Self(86104);
    pub const AV_CODEC_ID_HCOM: Self = Self(86105);
    pub const AV_CODEC_ID_ACELP_KELVIN: Self = Self(86106);
    pub const AV_CODEC_ID_MPEGH_3D_AUDIO: Self = Self(86107);
    pub const AV_CODEC_ID_SIREN: Self = Self(86108);
    pub const AV_CODEC_ID_HCA: Self = Self(86109);
    pub const AV_CODEC_ID_FASTAUDIO: Self = Self(86110);
    pub const AV_CODEC_ID_MSNSIREN: Self = Self(86111);
    pub const AV_CODEC_ID_DFPWM: Self = Self(86112);
    pub const AV_CODEC_ID_BONK: Self = Self(86113);
    pub const AV_CODEC_ID_MISC4: Self = Self(86114);
    pub const AV_CODEC_ID_APAC: Self = Self(86115);
    pub const AV_CODEC_ID_FTR: Self = Self(86116);
    pub const AV_CODEC_ID_WAVARC: Self = Self(86117);
    pub const AV_CODEC_ID_RKA: Self = Self(86118);
    pub const AV_CODEC_ID_AC4: Self = Self(86119);
    pub const AV_CODEC_ID_OSQ: Self = Self(86120);
    pub const AV_CODEC_ID_QOA: Self = Self(86121);
    pub const AV_CODEC_ID_LC3: Self = Self(86122);
    pub const AV_CODEC_ID_G728: Self = Self(86123);
    pub const AV_CODEC_ID_AHX: Self = Self(86124);
    pub const AV_CODEC_ID_APPLE_APAC: Self = Self(86125);
    pub const AV_CODEC_ID_FIRST_SUBTITLE: Self = Self(94208);
    pub const AV_CODEC_ID_DVD_SUBTITLE: Self = Self(94208);
    pub const AV_CODEC_ID_DVB_SUBTITLE: Self = Self(94209);
    pub const AV_CODEC_ID_TEXT: Self = Self(94210);
    pub const AV_CODEC_ID_XSUB: Self = Self(94211);
    pub const AV_CODEC_ID_SSA: Self = Self(94212);
    pub const AV_CODEC_ID_MOV_TEXT: Self = Self(94213);
    pub const AV_CODEC_ID_HDMV_PGS_SUBTITLE: Self = Self(94214);
    pub const AV_CODEC_ID_DVB_TELETEXT: Self = Self(94215);
    pub const AV_CODEC_ID_SRT: Self = Self(94216);
    pub const AV_CODEC_ID_MICRODVD: Self = Self(94217);
    pub const AV_CODEC_ID_EIA_608: Self = Self(94218);
    pub const AV_CODEC_ID_JACOSUB: Self = Self(94219);
    pub const AV_CODEC_ID_SAMI: Self = Self(94220);
    pub const AV_CODEC_ID_REALTEXT: Self = Self(94221);
    pub const AV_CODEC_ID_STL: Self = Self(94222);
    pub const AV_CODEC_ID_SUBVIEWER1: Self = Self(94223);
    pub const AV_CODEC_ID_SUBVIEWER: Self = Self(94224);
    pub const AV_CODEC_ID_SUBRIP: Self = Self(94225);
    pub const AV_CODEC_ID_WEBVTT: Self = Self(94226);
    pub const AV_CODEC_ID_MPL2: Self = Self(94227);
    pub const AV_CODEC_ID_VPLAYER: Self = Self(94228);
    pub const AV_CODEC_ID_PJS: Self = Self(94229);
    pub const AV_CODEC_ID_ASS: Self = Self(94230);
    pub const AV_CODEC_ID_HDMV_TEXT_SUBTITLE: Self = Self(94231);
    pub const AV_CODEC_ID_TTML: Self = Self(94232);
    pub const AV_CODEC_ID_ARIB_CAPTION: Self = Self(94233);
    pub const AV_CODEC_ID_IVTV_VBI: Self = Self(94234);
    pub const AV_CODEC_ID_FIRST_UNKNOWN: Self = Self(98304);
    pub const AV_CODEC_ID_TTF: Self = Self(98304);
    pub const AV_CODEC_ID_SCTE_35: Self = Self(98305);
    pub const AV_CODEC_ID_EPG: Self = Self(98306);
    pub const AV_CODEC_ID_BINTEXT: Self = Self(98307);
    pub const AV_CODEC_ID_XBIN: Self = Self(98308);
    pub const AV_CODEC_ID_IDF: Self = Self(98309);
    pub const AV_CODEC_ID_OTF: Self = Self(98310);
    pub const AV_CODEC_ID_SMPTE_KLV: Self = Self(98311);
    pub const AV_CODEC_ID_DVD_NAV: Self = Self(98312);
    pub const AV_CODEC_ID_TIMED_ID3: Self = Self(98313);
    pub const AV_CODEC_ID_BIN_DATA: Self = Self(98314);
    pub const AV_CODEC_ID_SMPTE_2038: Self = Self(98315);
    pub const AV_CODEC_ID_LCEVC: Self = Self(98316);
    pub const AV_CODEC_ID_SMPTE_436M_ANC: Self = Self(98317);
    pub const AV_CODEC_ID_PROBE: Self = Self(102400);
    pub const AV_CODEC_ID_MPEG2TS: Self = Self(131072);
    pub const AV_CODEC_ID_MPEG4SYSTEMS: Self = Self(131073);
    pub const AV_CODEC_ID_FFMETADATA: Self = Self(135168);
    pub const AV_CODEC_ID_WRAPPED_AVFRAME: Self = Self(135169);
    pub const AV_CODEC_ID_VNULL: Self = Self(135170);
    pub const AV_CODEC_ID_ANULL: Self = Self(135171);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVFieldOrder(pub ::core::ffi::c_uint);
impl AVFieldOrder {
    pub const AV_FIELD_UNKNOWN: Self = Self(0);
    pub const AV_FIELD_PROGRESSIVE: Self = Self(1);
    pub const AV_FIELD_TT: Self = Self(2);
    pub const AV_FIELD_BB: Self = Self(3);
    pub const AV_FIELD_TB: Self = Self(4);
    pub const AV_FIELD_BT: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVDiscard(pub ::core::ffi::c_int);
impl AVDiscard {
    pub const AVDISCARD_NONE: Self = Self(-16);
    pub const AVDISCARD_DEFAULT: Self = Self(0);
    pub const AVDISCARD_NONREF: Self = Self(8);
    pub const AVDISCARD_BIDIR: Self = Self(16);
    pub const AVDISCARD_NONINTRA: Self = Self(24);
    pub const AVDISCARD_NONKEY: Self = Self(32);
    pub const AVDISCARD_ALL: Self = Self(48);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVPacketSideDataType(pub ::core::ffi::c_uint);
impl AVPacketSideDataType {
    pub const AV_PKT_DATA_PALETTE: Self = Self(0);
    pub const AV_PKT_DATA_NEW_EXTRADATA: Self = Self(1);
    pub const AV_PKT_DATA_PARAM_CHANGE: Self = Self(2);
    pub const AV_PKT_DATA_H263_MB_INFO: Self = Self(3);
    pub const AV_PKT_DATA_REPLAYGAIN: Self = Self(4);
    pub const AV_PKT_DATA_DISPLAYMATRIX: Self = Self(5);
    pub const AV_PKT_DATA_STEREO3D: Self = Self(6);
    pub const AV_PKT_DATA_AUDIO_SERVICE_TYPE: Self = Self(7);
    pub const AV_PKT_DATA_QUALITY_STATS: Self = Self(8);
    pub const AV_PKT_DATA_FALLBACK_TRACK: Self = Self(9);
    pub const AV_PKT_DATA_CPB_PROPERTIES: Self = Self(10);
    pub const AV_PKT_DATA_SKIP_SAMPLES: Self = Self(11);
    pub const AV_PKT_DATA_JP_DUALMONO: Self = Self(12);
    pub const AV_PKT_DATA_STRINGS_METADATA: Self = Self(13);
    pub const AV_PKT_DATA_SUBTITLE_POSITION: Self = Self(14);
    pub const AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL: Self = Self(15);
    pub const AV_PKT_DATA_WEBVTT_IDENTIFIER: Self = Self(16);
    pub const AV_PKT_DATA_WEBVTT_SETTINGS: Self = Self(17);
    pub const AV_PKT_DATA_METADATA_UPDATE: Self = Self(18);
    pub const AV_PKT_DATA_MPEGTS_STREAM_ID: Self = Self(19);
    pub const AV_PKT_DATA_MASTERING_DISPLAY_METADATA: Self = Self(20);
    pub const AV_PKT_DATA_SPHERICAL: Self = Self(21);
    pub const AV_PKT_DATA_CONTENT_LIGHT_LEVEL: Self = Self(22);
    pub const AV_PKT_DATA_A53_CC: Self = Self(23);
    pub const AV_PKT_DATA_ENCRYPTION_INIT_INFO: Self = Self(24);
    pub const AV_PKT_DATA_ENCRYPTION_INFO: Self = Self(25);
    pub const AV_PKT_DATA_AFD: Self = Self(26);
    pub const AV_PKT_DATA_PRFT: Self = Self(27);
    pub const AV_PKT_DATA_ICC_PROFILE: Self = Self(28);
    pub const AV_PKT_DATA_DOVI_CONF: Self = Self(29);
    pub const AV_PKT_DATA_S12M_TIMECODE: Self = Self(30);
    pub const AV_PKT_DATA_DYNAMIC_HDR10_PLUS: Self = Self(31);
    pub const AV_PKT_DATA_IAMF_MIX_GAIN_PARAM: Self = Self(32);
    pub const AV_PKT_DATA_IAMF_DEMIXING_INFO_PARAM: Self = Self(33);
    pub const AV_PKT_DATA_IAMF_RECON_GAIN_INFO_PARAM: Self = Self(34);
    pub const AV_PKT_DATA_AMBIENT_VIEWING_ENVIRONMENT: Self = Self(35);
    pub const AV_PKT_DATA_FRAME_CROPPING: Self = Self(36);
    pub const AV_PKT_DATA_LCEVC: Self = Self(37);
    pub const AV_PKT_DATA_3D_REFERENCE_DISPLAYS: Self = Self(38);
    pub const AV_PKT_DATA_RTCP_SR: Self = Self(39);
    pub const AV_PKT_DATA_EXIF: Self = Self(40);
    pub const AV_PKT_DATA_DYNAMIC_HDR_SMPTE_2094_APP5: Self = Self(41);
    pub const AV_PKT_DATA_HEVC_CONF: Self = Self(42);
    pub const AV_PKT_DATA_NB: Self = Self(43);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacketSideData {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub r#type: AVPacketSideDataType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacket {
    pub buf: *mut AVBufferRef,
    pub pts: int64_t,
    pub dts: int64_t,
    pub data: *mut uint8_t,
    pub size: ::core::ffi::c_int,
    pub stream_index: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: ::core::ffi::c_int,
    pub duration: int64_t,
    pub pos: int64_t,
    pub opaque: *mut ::core::ffi::c_void,
    pub opaque_ref: *mut AVBufferRef,
    pub time_base: AVRational,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParameters {
    pub codec_type: AVMediaType,
    pub codec_id: AVCodecID,
    pub codec_tag: uint32_t,
    pub extradata: *mut uint8_t,
    pub extradata_size: ::core::ffi::c_int,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: ::core::ffi::c_int,
    pub format: ::core::ffi::c_int,
    pub bit_rate: int64_t,
    pub bits_per_coded_sample: ::core::ffi::c_int,
    pub bits_per_raw_sample: ::core::ffi::c_int,
    pub profile: ::core::ffi::c_int,
    pub level: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub sample_aspect_ratio: AVRational,
    pub framerate: AVRational,
    pub field_order: AVFieldOrder,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub color_space: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub video_delay: ::core::ffi::c_int,
    pub ch_layout: AVChannelLayout,
    pub sample_rate: ::core::ffi::c_int,
    pub block_align: ::core::ffi::c_int,
    pub frame_size: ::core::ffi::c_int,
    pub initial_padding: ::core::ffi::c_int,
    pub trailing_padding: ::core::ffi::c_int,
    pub seek_preroll: ::core::ffi::c_int,
    pub alpha_mode: AVAlphaMode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVIOInterruptCB {
    pub callback: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVIODataMarkerType(pub ::core::ffi::c_uint);
impl AVIODataMarkerType {
    pub const AVIO_DATA_MARKER_HEADER: Self = Self(0);
    pub const AVIO_DATA_MARKER_SYNC_POINT: Self = Self(1);
    pub const AVIO_DATA_MARKER_BOUNDARY_POINT: Self = Self(2);
    pub const AVIO_DATA_MARKER_UNKNOWN: Self = Self(3);
    pub const AVIO_DATA_MARKER_TRAILER: Self = Self(4);
    pub const AVIO_DATA_MARKER_FLUSH_POINT: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVIOContext {
    pub av_class: *const AVClass,
    pub buffer: *mut ::core::ffi::c_uchar,
    pub buffer_size: ::core::ffi::c_int,
    pub buf_ptr: *mut ::core::ffi::c_uchar,
    pub buf_end: *mut ::core::ffi::c_uchar,
    pub opaque: *mut ::core::ffi::c_void,
    pub read_packet: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut uint8_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub write_packet: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const uint8_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub seek: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, int64_t, ::core::ffi::c_int) -> int64_t,
    >,
    pub pos: int64_t,
    pub eof_reached: ::core::ffi::c_int,
    pub error: ::core::ffi::c_int,
    pub write_flag: ::core::ffi::c_int,
    pub max_packet_size: ::core::ffi::c_int,
    pub min_packet_size: ::core::ffi::c_int,
    pub checksum: ::core::ffi::c_ulong,
    pub checksum_ptr: *mut ::core::ffi::c_uchar,
    pub update_checksum: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_ulong,
            *const uint8_t,
            ::core::ffi::c_uint,
        ) -> ::core::ffi::c_ulong,
    >,
    pub read_pause: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub read_seek: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            int64_t,
            ::core::ffi::c_int,
        ) -> int64_t,
    >,
    pub seekable: ::core::ffi::c_int,
    pub direct: ::core::ffi::c_int,
    pub protocol_whitelist: *const ::core::ffi::c_char,
    pub protocol_blacklist: *const ::core::ffi::c_char,
    pub write_data_type: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const uint8_t,
            ::core::ffi::c_int,
            AVIODataMarkerType,
            int64_t,
        ) -> ::core::ffi::c_int,
    >,
    pub ignore_boundary_point: ::core::ffi::c_int,
    pub buf_ptr_max: *mut ::core::ffi::c_uchar,
    pub bytes_read: int64_t,
    pub bytes_written: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProfile {
    pub profile: ::core::ffi::c_int,
    pub name: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodec {
    pub name: *const ::core::ffi::c_char,
    pub long_name: *const ::core::ffi::c_char,
    pub r#type: AVMediaType,
    pub id: AVCodecID,
    pub capabilities: ::core::ffi::c_int,
    pub max_lowres: uint8_t,
    pub priv_class: *const AVClass,
    pub profiles: *const AVProfile,
    pub wrapper_name: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFormatContext {
    pub av_class: *const AVClass,
    pub iformat: *const AVInputFormat,
    pub oformat: *const AVOutputFormat,
    pub priv_data: *mut ::core::ffi::c_void,
    pub pb: *mut AVIOContext,
    pub ctx_flags: ::core::ffi::c_int,
    pub nb_streams: ::core::ffi::c_uint,
    pub streams: *mut *mut AVStream,
    pub nb_stream_groups: ::core::ffi::c_uint,
    pub stream_groups: *mut *mut AVStreamGroup,
    pub nb_chapters: ::core::ffi::c_uint,
    pub chapters: *mut *mut AVChapter,
    pub url: *mut ::core::ffi::c_char,
    pub start_time: int64_t,
    pub duration: int64_t,
    pub bit_rate: int64_t,
    pub packet_size: ::core::ffi::c_uint,
    pub max_delay: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub probesize: int64_t,
    pub max_analyze_duration: int64_t,
    pub key: *const uint8_t,
    pub keylen: ::core::ffi::c_int,
    pub nb_programs: ::core::ffi::c_uint,
    pub programs: *mut *mut AVProgram,
    pub video_codec_id: AVCodecID,
    pub audio_codec_id: AVCodecID,
    pub subtitle_codec_id: AVCodecID,
    pub data_codec_id: AVCodecID,
    pub metadata: *mut AVDictionary,
    pub start_time_realtime: int64_t,
    pub fps_probe_size: ::core::ffi::c_int,
    pub error_recognition: ::core::ffi::c_int,
    pub interrupt_callback: AVIOInterruptCB,
    pub debug: ::core::ffi::c_int,
    pub max_streams: ::core::ffi::c_int,
    pub max_index_size: ::core::ffi::c_uint,
    pub max_picture_buffer: ::core::ffi::c_uint,
    pub max_interleave_delta: int64_t,
    pub max_ts_probe: ::core::ffi::c_int,
    pub max_chunk_duration: ::core::ffi::c_int,
    pub max_chunk_size: ::core::ffi::c_int,
    pub max_probe_packets: ::core::ffi::c_int,
    pub strict_std_compliance: ::core::ffi::c_int,
    pub event_flags: ::core::ffi::c_int,
    pub avoid_negative_ts: ::core::ffi::c_int,
    pub audio_preload: ::core::ffi::c_int,
    pub use_wallclock_as_timestamps: ::core::ffi::c_int,
    pub skip_estimate_duration_from_pts: ::core::ffi::c_int,
    pub avio_flags: ::core::ffi::c_int,
    pub duration_estimation_method: AVDurationEstimationMethod,
    pub skip_initial_bytes: int64_t,
    pub correct_ts_overflow: ::core::ffi::c_uint,
    pub seek2any: ::core::ffi::c_int,
    pub flush_packets: ::core::ffi::c_int,
    pub probe_score: ::core::ffi::c_int,
    pub format_probesize: ::core::ffi::c_int,
    pub codec_whitelist: *mut ::core::ffi::c_char,
    pub format_whitelist: *mut ::core::ffi::c_char,
    pub protocol_whitelist: *mut ::core::ffi::c_char,
    pub protocol_blacklist: *mut ::core::ffi::c_char,
    pub io_repositioned: ::core::ffi::c_int,
    pub video_codec: *const AVCodec,
    pub audio_codec: *const AVCodec,
    pub subtitle_codec: *const AVCodec,
    pub data_codec: *const AVCodec,
    pub metadata_header_padding: ::core::ffi::c_int,
    pub opaque: *mut ::core::ffi::c_void,
    pub control_message_cb: av_format_control_message,
    pub output_ts_offset: int64_t,
    pub dump_separator: *mut uint8_t,
    pub io_open: Option<
        unsafe extern "C" fn(
            *mut AVFormatContext,
            *mut *mut AVIOContext,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut AVDictionary,
        ) -> ::core::ffi::c_int,
    >,
    pub io_close2:
        Option<unsafe extern "C" fn(*mut AVFormatContext, *mut AVIOContext) -> ::core::ffi::c_int>,
    pub duration_probesize: int64_t,
    pub name: *mut ::core::ffi::c_char,
}
pub type av_format_control_message = Option<
    unsafe extern "C" fn(
        *mut AVFormatContext,
        ::core::ffi::c_int,
        *mut ::core::ffi::c_void,
        size_t,
    ) -> ::core::ffi::c_int,
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVDurationEstimationMethod(pub ::core::ffi::c_uint);
impl AVDurationEstimationMethod {
    pub const AVFMT_DURATION_FROM_PTS: Self = Self(0);
    pub const AVFMT_DURATION_FROM_STREAM: Self = Self(1);
    pub const AVFMT_DURATION_FROM_BITRATE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProgram {
    pub id: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub discard: AVDiscard,
    pub stream_index: *mut ::core::ffi::c_uint,
    pub nb_stream_indexes: ::core::ffi::c_uint,
    pub metadata: *mut AVDictionary,
    pub program_num: ::core::ffi::c_int,
    pub pmt_pid: ::core::ffi::c_int,
    pub pcr_pid: ::core::ffi::c_int,
    pub pmt_version: ::core::ffi::c_int,
    pub start_time: int64_t,
    pub end_time: int64_t,
    pub pts_wrap_reference: int64_t,
    pub pts_wrap_behavior: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChapter {
    pub id: int64_t,
    pub time_base: AVRational,
    pub start: int64_t,
    pub end: int64_t,
    pub metadata: *mut AVDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStreamGroup {
    pub av_class: *const AVClass,
    pub priv_data: *mut ::core::ffi::c_void,
    pub index: ::core::ffi::c_uint,
    pub id: int64_t,
    pub r#type: AVStreamGroupParamsType,
    pub params: C2Rust_Unnamed_1,
    pub metadata: *mut AVDictionary,
    pub nb_streams: ::core::ffi::c_uint,
    pub streams: *mut *mut AVStream,
    pub disposition: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStream {
    pub av_class: *const AVClass,
    pub index: ::core::ffi::c_int,
    pub id: ::core::ffi::c_int,
    pub codecpar: *mut AVCodecParameters,
    pub priv_data: *mut ::core::ffi::c_void,
    pub time_base: AVRational,
    pub start_time: int64_t,
    pub duration: int64_t,
    pub nb_frames: int64_t,
    pub disposition: ::core::ffi::c_int,
    pub discard: AVDiscard,
    pub sample_aspect_ratio: AVRational,
    pub metadata: *mut AVDictionary,
    pub avg_frame_rate: AVRational,
    pub attached_pic: AVPacket,
    pub event_flags: ::core::ffi::c_int,
    pub r_frame_rate: AVRational,
    pub pts_wrap_bits: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub iamf_audio_element: *mut AVIAMFAudioElement,
    pub iamf_mix_presentation: *mut AVIAMFMixPresentation,
    pub tile_grid: *mut AVStreamGroupTileGrid,
    pub layered_video: *mut AVStreamGroupLayeredVideo,
    pub lcevc: *mut AVStreamGroupLayeredVideo,
    pub tref: *mut AVStreamGroupTREF,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStreamGroupTREF {
    pub av_class: *const AVClass,
    pub metadata_index: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStreamGroupLayeredVideo {
    pub av_class: *const AVClass,
    pub c2rust_unnamed: C2Rust_Unnamed_2,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub el_index: ::core::ffi::c_uint,
    pub lcevc_index: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStreamGroupTileGrid {
    pub av_class: *const AVClass,
    pub nb_tiles: ::core::ffi::c_uint,
    pub coded_width: ::core::ffi::c_int,
    pub coded_height: ::core::ffi::c_int,
    pub offsets: *mut C2Rust_Unnamed_3,
    pub background: [uint8_t; 4],
    pub horizontal_offset: ::core::ffi::c_int,
    pub vertical_offset: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub idx: ::core::ffi::c_uint,
    pub horizontal: ::core::ffi::c_int,
    pub vertical: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AVStreamGroupParamsType(pub ::core::ffi::c_uint);
impl AVStreamGroupParamsType {
    pub const AV_STREAM_GROUP_PARAMS_NONE: Self = Self(0);
    pub const AV_STREAM_GROUP_PARAMS_IAMF_AUDIO_ELEMENT: Self = Self(1);
    pub const AV_STREAM_GROUP_PARAMS_IAMF_MIX_PRESENTATION: Self = Self(2);
    pub const AV_STREAM_GROUP_PARAMS_TILE_GRID: Self = Self(3);
    pub const AV_STREAM_GROUP_PARAMS_LCEVC: Self = Self(4);
    pub const AV_STREAM_GROUP_PARAMS_TREF: Self = Self(5);
    pub const AV_STREAM_GROUP_PARAMS_DOLBY_VISION: Self = Self(6);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOutputFormat {
    pub name: *const ::core::ffi::c_char,
    pub long_name: *const ::core::ffi::c_char,
    pub mime_type: *const ::core::ffi::c_char,
    pub extensions: *const ::core::ffi::c_char,
    pub audio_codec: AVCodecID,
    pub video_codec: AVCodecID,
    pub subtitle_codec: AVCodecID,
    pub flags: ::core::ffi::c_int,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVInputFormat {
    pub name: *const ::core::ffi::c_char,
    pub long_name: *const ::core::ffi::c_char,
    pub flags: ::core::ffi::c_int,
    pub extensions: *const ::core::ffi::c_char,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
    pub mime_type: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_frame_rate_measurement_t {
    pub started: bool,
    pub warmup_until_us: int64_t,
    pub window_start_us: int64_t,
    pub packets: uint64_t,
    pub fps: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp4_writer {
    pub output_path: [::core::ffi::c_char; 512],
    pub stream_name: [::core::ffi::c_char; 256],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub output_ctx: *mut AVFormatContext,
    pub reserved_output: *mut AVIOContext,
    pub video_stream_idx: ::core::ffi::c_int,
    pub has_audio: ::core::ffi::c_int,
    pub first_dts: int64_t,
    pub first_pts: int64_t,
    pub last_dts: int64_t,
    pub time_base: AVRational,
    pub is_initialized: ::core::ffi::c_int,
    pub creation_time: time_t,
    pub last_packet_time: time_t,
    pub audio: mp4_audio_state_t,
    pub mutex: pthread_mutex_t,
    pub current_recording_id: uint64_t,
    pub placement: storage_placement_t,
    pub segment_duration: ::core::ffi::c_int,
    pub last_rotation_time: time_t,
    pub waiting_for_keyframe: ::core::ffi::c_int,
    pub is_rotating: ::core::ffi::c_int,
    pub output_dir: [::core::ffi::c_char; 512],
    pub trigger_type: [::core::ffi::c_char; 16],
    pub schedule_restricted: ::core::ffi::c_int,
    pub start_time_corrected: bool,
    pub thread_ctx: *mut mp4_writer_thread_t,
    pub shutdown_component_id: ::core::ffi::c_int,
    pub pre_buffer_seconds: ::core::ffi::c_int,
    pub pending_audio_codecpar: *mut AVCodecParameters,
    pub pending_audio_time_base: AVRational,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp4_writer_thread_t {
    pub thread: pthread_t,
    pub running: ::core::ffi::c_int,
    pub rtsp_url: [::core::ffi::c_char; 512],
    pub shutdown_requested: atomic_int,
    pub writer: *mut mp4_writer_t,
    pub segment_duration: ::core::ffi::c_int,
    pub last_segment_time: time_t,
    pub retry_count: ::core::ffi::c_int,
    pub last_retry_time: time_t,
    pub auto_restart: bool,
    pub force_reconnect: atomic_int,
    pub input_ctx: *mut AVFormatContext,
    pub segment_info: segment_info_t,
    pub context_mutex: pthread_mutex_t,
    pub video_params_detected: bool,
    pub segment_retry_count: ::core::ffi::c_int,
    pub last_segment_retry_time: time_t,
    pub segment_count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segment_info_t {
    pub segment_index: ::core::ffi::c_int,
    pub has_audio: bool,
    pub last_frame_was_key: bool,
    pub pending_video_keyframe: *mut AVPacket,
    pub stream_name: [::core::ffi::c_char; 256],
    pub fps_measurement: stream_frame_rate_measurement_t,
}
pub type mp4_writer_t = mp4_writer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_placement_t {
    pub status: storage_placement_status_t,
    pub target_is_default: bool,
    pub target_uuid: [::core::ffi::c_char; 37],
    pub target_root: [::core::ffi::c_char; 512],
    pub object_key: [::core::ffi::c_char; 512],
    pub policy_uuid: [::core::ffi::c_char; 37],
    pub policy_version: int64_t,
    pub reason: [::core::ffi::c_char; 64],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct storage_placement_status_t(pub ::core::ffi::c_uint);
impl storage_placement_status_t {
    pub const STORAGE_PLACEMENT_READY: Self = Self(0);
    pub const STORAGE_PLACEMENT_PAUSED: Self = Self(1);
    pub const STORAGE_PLACEMENT_FAILED: Self = Self(2);
    pub const STORAGE_PLACEMENT_ERROR: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp4_audio_state_t {
    pub stream_idx: ::core::ffi::c_int,
    pub first_dts: int64_t,
    pub last_pts: int64_t,
    pub last_dts: int64_t,
    pub initialized: ::core::ffi::c_int,
    pub time_base: AVRational,
    pub frame_size: ::core::ffi::c_int,
    pub mutex: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp4_recording_ctx_t {
    pub config: stream_config_t,
    pub running: ::core::ffi::c_int,
    pub reaping: ::core::ffi::c_int,
    pub thread: pthread_t,
    pub output_path: [::core::ffi::c_char; 512],
    pub placement: storage_placement_t,
    pub mp4_writer: *mut mp4_writer_t,
    pub trigger_type: [::core::ffi::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_runtime_info_t {
    pub active: bool,
    pub initializing: bool,
    pub trigger_type: [::core::ffi::c_char; 16],
    pub recording_id: uint64_t,
}
pub const W_OK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
#[unsafe(no_mangle)]
pub static mut recording_contexts: [*mut mp4_recording_ctx_t; 1024] =
    [::core::ptr::null_mut::<mp4_recording_ctx_t>(); 1024];
static mut recording_contexts_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut recording_slot_reserved: [bool; 1024] = [false; 1024];
unsafe extern "C" fn reserve_recording_slot_locked() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        if recording_contexts[i as usize].is_null() && !recording_slot_reserved[i as usize] {
            recording_slot_reserved[i as usize] = r#true != 0;
            return i;
        }
        i += 1;
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn publish_recording_slot(
    mut slot: ::core::ffi::c_int,
    mut ctx: *mut mp4_recording_ctx_t,
) {
    if slot < 0 as ::core::ffi::c_int {
        return;
    }
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    if !ctx.is_null() {
        recording_contexts[slot as usize] = ctx;
    }
    recording_slot_reserved[slot as usize] = r#false != 0;
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
}
#[unsafe(no_mangle)]
pub static mut shutdown_in_progress: sig_atomic_t = 0 as sig_atomic_t;
unsafe extern "C" fn claim_start_for_stream(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut stopped: *mut mp4_recording_ctx_t = ::core::ptr::null_mut::<mp4_recording_ctx_t>();
    let mut slot: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        let mut ctx: *mut mp4_recording_ctx_t = recording_contexts[i as usize];
        if ctx.is_null()
            || strcmp(
                &raw mut (*ctx).config.name as *mut ::core::ffi::c_char,
                stream_name,
            ) != 0 as ::core::ffi::c_int
        {
            i += 1;
        } else {
            if (*ctx).running != 0 {
                let mut writer: *mut mp4_writer_t = (*ctx).mp4_writer;
                let mut healthy: bool = !writer.is_null() && mp4_writer_is_recording(writer) != 0;
                pthread_mutex_unlock(&raw mut recording_contexts_mutex);
                if healthy {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"MP4 recording for stream %s already running and healthy\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        stream_name,
                    );
                } else if writer.is_null() {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"MP4 recording for stream %s is initializing, skipping duplicate start\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        stream_name,
                    );
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"MP4 recording thread for stream %s is alive and recovering its RTSP reader; leaving it to finish\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        stream_name,
                    );
                }
                return 1 as ::core::ffi::c_int;
            }
            if (*ctx).reaping != 0 {
                pthread_mutex_unlock(&raw mut recording_contexts_mutex);
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"MP4 recording for stream %s is being reaped by another starter\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_name,
                );
                return -1 as ::core::ffi::c_int;
            }
            (*ctx).reaping = 1 as ::core::ffi::c_int;
            stopped = ctx;
            slot = i;
            break;
        }
    }
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
    if stopped.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut join_result: ::core::ffi::c_int = pthread_join_with_timeout(
        (*stopped).thread,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        15 as ::core::ffi::c_int,
    );
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    if join_result != 0 as ::core::ffi::c_int {
        (*stopped).reaping = 0 as ::core::ffi::c_int;
        pthread_mutex_unlock(&raw mut recording_contexts_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MP4 recording thread for stream %s has stopped but not exited within 15 s; will retry on the next check\0"
                .as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    if recording_contexts[slot as usize] == stopped {
        recording_contexts[slot as usize] = ::core::ptr::null_mut::<mp4_recording_ctx_t>();
    }
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
    if !(*stopped).mp4_writer.is_null() {
        let mut writer_0: *mut mp4_writer_t = (*stopped).mp4_writer;
        (*stopped).mp4_writer = ::core::ptr::null_mut::<mp4_writer_t>();
        unregister_mp4_writer_for_stream(stream_name);
        mp4_writer_close(writer_0);
    }
    free(stopped as *mut ::core::ffi::c_void);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Reaped exited MP4 recording thread for stream %s, restarting\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mp4_recording_thread_alive(
    mut stream_name: *const ::core::ffi::c_char,
) -> bool {
    if stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut alive: bool = r#false != 0;
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        let mut ctx: *mut mp4_recording_ctx_t = recording_contexts[i as usize];
        if !ctx.is_null()
            && (*ctx).running != 0
            && strcmp(
                &raw mut (*ctx).config.name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            alive = r#true != 0;
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
    return alive;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_mp4_recording_runtime_info(
    mut stream_name: *const ::core::ffi::c_char,
    mut info: *mut recording_runtime_info_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null() || info.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        info as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<recording_runtime_info_t>(),
    );
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        let mut ctx: *mut mp4_recording_ctx_t = recording_contexts[i as usize];
        if ctx.is_null()
            || strcmp(
                &raw mut (*ctx).config.name as *mut ::core::ffi::c_char,
                stream_name,
            ) != 0 as ::core::ffi::c_int
        {
            i += 1;
        } else {
            (*info).initializing = (*ctx).running != 0 && (*ctx).mp4_writer.is_null();
            (*info).active = (*ctx).running != 0
                && !(*ctx).mp4_writer.is_null()
                && mp4_writer_is_recording((*ctx).mp4_writer) != 0;
            safe_strcpy(
                &raw mut (*info).trigger_type as *mut ::core::ffi::c_char,
                if (*ctx).trigger_type[0usize] as ::core::ffi::c_int != 0 {
                    &raw mut (*ctx).trigger_type as *mut ::core::ffi::c_char
                } else {
                    b"scheduled\0".as_ptr() as *const ::core::ffi::c_char
                },
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
            if !(*ctx).mp4_writer.is_null() {
                (*info).recording_id = (*(*ctx).mp4_writer).current_recording_id;
            }
            pthread_mutex_unlock(&raw mut recording_contexts_mutex);
            return if (*info).active as ::core::ffi::c_int != 0
                || (*info).initializing as ::core::ffi::c_int != 0
            {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
        }
    }
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn mp4_recording_thread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut ctx: *mut mp4_recording_ctx_t = arg as *mut mp4_recording_ctx_t;
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut (*ctx).config.name as *mut ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
        0 as size_t,
    );
    log_set_thread_context(
        b"MP4Recorder\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting MP4 recording thread for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    if (*ctx).running == 0
        || ::core::ptr::read_volatile::<sig_atomic_t>(&raw const shutdown_in_progress) != 0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MP4 recording thread for %s exiting early due to shutdown\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        return NULL;
    }
    let mut mp4_dir: [::core::ffi::c_char; 512] = [0; 512];
    safe_strcpy(
        &raw mut mp4_dir as *mut ::core::ffi::c_char,
        &raw mut (*ctx).output_path as *mut ::core::ffi::c_char,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    let mut last_slash: *mut ::core::ffi::c_char = strrchr(
        &raw mut mp4_dir as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if !last_slash.is_null() {
        *last_slash = '\0' as ::core::ffi::c_char;
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
    if stat(&raw mut mp4_dir as *mut ::core::ffi::c_char, &raw mut st) != 0 as ::core::ffi::c_int
        || !(st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Output directory does not exist or is not a directory: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut mp4_dir as *mut ::core::ffi::c_char,
        );
        let mut ret_mkdir: ::core::ffi::c_int =
            mkdir_recursive(&raw mut mp4_dir as *mut ::core::ffi::c_char);
        if ret_mkdir != 0 as ::core::ffi::c_int
            || stat(&raw mut mp4_dir as *mut ::core::ffi::c_char, &raw mut st)
                != 0 as ::core::ffi::c_int
            || !(st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to create output directory: %s (return code: %d)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut mp4_dir as *mut ::core::ffi::c_char,
                ret_mkdir,
            );
            (*ctx).running = 0 as ::core::ffi::c_int;
            return NULL;
        }
        if chmod_path(
            &raw mut mp4_dir as *mut ::core::ffi::c_char,
            0o755 as mode_t,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to set permissions on directory: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut mp4_dir as *mut ::core::ffi::c_char,
            );
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Successfully created output directory: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut mp4_dir as *mut ::core::ffi::c_char,
        );
    }
    if access(&raw mut mp4_dir as *mut ::core::ffi::c_char, W_OK) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Output directory is not writable: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut mp4_dir as *mut ::core::ffi::c_char,
        );
        if chmod_path(
            &raw mut mp4_dir as *mut ::core::ffi::c_char,
            0o755 as mode_t,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to set permissions on directory: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut mp4_dir as *mut ::core::ffi::c_char,
            );
        }
        if access(&raw mut mp4_dir as *mut ::core::ffi::c_char, W_OK) != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Still unable to write to output directory: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut mp4_dir as *mut ::core::ffi::c_char,
            );
            (*ctx).running = 0 as ::core::ffi::c_int;
            return NULL;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Successfully fixed permissions for output directory: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut mp4_dir as *mut ::core::ffi::c_char,
        );
    }
    if (*ctx).running == 0
        || ::core::ptr::read_volatile::<sig_atomic_t>(&raw const shutdown_in_progress) != 0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"MP4 recording thread for %s exiting after directory checks due to shutdown\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        return NULL;
    }
    (*ctx).mp4_writer = mp4_writer_create(
        &raw mut (*ctx).output_path as *mut ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    if (*ctx).mp4_writer.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create MP4 writer for %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        (*ctx).running = 0 as ::core::ffi::c_int;
        return NULL;
    }
    (*(*ctx).mp4_writer).placement = (*ctx).placement;
    safe_strcpy(
        &raw mut (*(*ctx).mp4_writer).camera_uuid as *mut ::core::ffi::c_char,
        &raw mut (*ctx).config.camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    mp4_writer_set_audio(
        (*ctx).mp4_writer,
        if (*ctx).config.record_audio as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    if (*ctx).trigger_type[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        safe_strcpy(
            &raw mut (*(*ctx).mp4_writer).trigger_type as *mut ::core::ffi::c_char,
            &raw mut (*ctx).trigger_type as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
    }
    (*(*ctx).mp4_writer).schedule_restricted = (strcmp(
        &raw mut (*(*ctx).mp4_writer).trigger_type as *mut ::core::ffi::c_char,
        b"scheduled\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && (*ctx).config.record_on_schedule as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Created MP4 writer for %s at %s (trigger_type: %s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut (*ctx).output_path as *mut ::core::ffi::c_char,
        &raw mut (*(*ctx).mp4_writer).trigger_type as *mut ::core::ffi::c_char,
    );
    if register_mp4_writer_for_stream(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        (*ctx).mp4_writer,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Not starting MP4 recording for %s: its previous writer is still registered\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        let mut unregistered: *mut mp4_writer_t = (*ctx).mp4_writer;
        (*ctx).mp4_writer = ::core::ptr::null_mut::<mp4_writer_t>();
        mp4_writer_close(unregistered);
        (*ctx).running = 0 as ::core::ffi::c_int;
        return NULL;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Registered MP4 writer for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut segment_duration: ::core::ffi::c_int =
        if (*ctx).config.segment_duration > 0 as ::core::ffi::c_int {
            (*ctx).config.segment_duration
        } else {
            30 as ::core::ffi::c_int
        };
    mp4_writer_set_segment_duration((*ctx).mp4_writer, segment_duration);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Set segment duration to %d seconds for MP4 writer for stream %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        segment_duration,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut actual_url: [::core::ffi::c_char; 512] = [0; 512];
    let mut using_go2rtc: bool = r#false != 0;
    unsafe extern "C" {
        #[link_name = "go2rtc_integration_is_using_go2rtc_for_recording"]
        unsafe fn go2rtc_integration_is_using_go2rtc_for_recording_0(
            stream_name_0: *const ::core::ffi::c_char,
        ) -> bool;
    }
    unsafe extern "C" {
        #[link_name = "go2rtc_get_rtsp_url"]
        unsafe fn go2rtc_get_rtsp_url_0(
            stream_name_0: *const ::core::ffi::c_char,
            url: *mut ::core::ffi::c_char,
            url_size: size_t,
        ) -> bool;
    }
    if go2rtc_integration_is_using_go2rtc_for_recording_0(
        &raw mut stream_name as *mut ::core::ffi::c_char,
    ) {
        let mut retries: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
        let mut success: bool = r#false != 0;
        while retries > 0 as ::core::ffi::c_int && !success {
            if go2rtc_get_rtsp_url_0(
                &raw mut stream_name as *mut ::core::ffi::c_char,
                &raw mut actual_url as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            ) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Using go2rtc RTSP URL for MP4 recording on stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
                using_go2rtc = r#true != 0;
                success = r#true != 0;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to get go2rtc RTSP URL for stream %s, retrying in 2 seconds (%d retries left)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                    retries,
                );
                sleep(2 as ::core::ffi::c_uint);
                retries -= 1;
            }
        }
        if !success {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to get go2rtc RTSP URL for stream %s after multiple retries, falling back to original URL\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
            safe_strcpy(
                &raw mut actual_url as *mut ::core::ffi::c_char,
                &raw mut (*ctx).config.url as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        }
        if success as ::core::ffi::c_int != 0
            && (!(*ctx).config.record_audio || g_config.audio_disabled as ::core::ffi::c_int != 0)
        {
            let mut url_len: size_t = strlen(&raw mut actual_url as *mut ::core::ffi::c_char);
            let mut suffix: *const ::core::ffi::c_char =
                b"?video\0".as_ptr() as *const ::core::ffi::c_char;
            let mut suffix_len: size_t = strlen(suffix);
            if url_len.wrapping_add(suffix_len)
                < ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
            {
                safe_strcat(
                    &raw mut actual_url as *mut ::core::ffi::c_char,
                    suffix,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Audio recording disabled for %s, using video-only go2rtc RTSP URL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"RTSP URL too long to append ?video selector for stream %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
            }
        }
    } else if url_apply_credentials(
        &raw mut (*ctx).config.url as *mut ::core::ffi::c_char,
        if (*ctx).config.onvif_username[0usize] as ::core::ffi::c_int != 0 {
            &raw mut (*ctx).config.onvif_username as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        if (*ctx).config.onvif_password[0usize] as ::core::ffi::c_int != 0 {
            &raw mut (*ctx).config.onvif_password as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        &raw mut actual_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to inject credentials into URL for stream %s, using original URL\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        safe_strcpy(
            &raw mut actual_url as *mut ::core::ffi::c_char,
            &raw mut (*ctx).config.url as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    let mut ret: ::core::ffi::c_int = mp4_writer_start_recording_thread(
        (*ctx).mp4_writer,
        &raw mut actual_url as *mut ::core::ffi::c_char,
    );
    if ret < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to start RTSP recording thread for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        unregister_mp4_writer_for_stream(&raw mut stream_name as *mut ::core::ffi::c_char);
        mp4_writer_close((*ctx).mp4_writer);
        (*ctx).mp4_writer = ::core::ptr::null_mut::<mp4_writer_t>();
        (*ctx).running = 0 as ::core::ffi::c_int;
        return NULL;
    }
    if using_go2rtc {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Started MP4 recording for stream %s using go2rtc's RTSP output\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Started self-managing RTSP recording thread for %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut restart_url: [::core::ffi::c_char; 512] = [0; 512];
    safe_strcpy(
        &raw mut restart_url as *mut ::core::ffi::c_char,
        &raw mut actual_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    let mut dead_check_seconds: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut self_restart_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut healthy_seconds: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let DEAD_RESTART_THRESHOLD_SECS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let MAX_SELF_RESTARTS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let COOLDOWN_SECS: ::core::ffi::c_int = 300 as ::core::ffi::c_int;
    while (*ctx).running != 0
        && ::core::ptr::read_volatile::<sig_atomic_t>(&raw const shutdown_in_progress) == 0
    {
        if is_shutdown_initiated() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"MP4 recording thread for %s stopping due to system shutdown\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
            (*ctx).running = 0 as ::core::ffi::c_int;
            break;
        } else {
            sleep(1 as ::core::ffi::c_uint);
            if (*ctx).mp4_writer.is_null() {
                continue;
            }
            if mp4_writer_is_recording((*ctx).mp4_writer) == 0 {
                dead_check_seconds += 1;
                if dead_check_seconds < DEAD_RESTART_THRESHOLD_SECS {
                    continue;
                }
                if self_restart_count >= MAX_SELF_RESTARTS {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Stream %s: inner RTSP thread failed %d times in a row. Entering %d-second cooldown before next attempt.\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                        self_restart_count,
                        COOLDOWN_SECS,
                    );
                    mp4_writer_stop_recording_thread((*ctx).mp4_writer);
                    let mut cd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while cd < COOLDOWN_SECS {
                        if (*ctx).running == 0
                            || ::core::ptr::read_volatile::<sig_atomic_t>(
                                &raw const shutdown_in_progress,
                            ) != 0
                            || is_shutdown_initiated() as ::core::ffi::c_int != 0
                        {
                            (*ctx).running = 0 as ::core::ffi::c_int;
                            break;
                        } else {
                            sleep(1 as ::core::ffi::c_uint);
                            cd += 1;
                        }
                    }
                    if (*ctx).running == 0
                        || ::core::ptr::read_volatile::<sig_atomic_t>(
                            &raw const shutdown_in_progress,
                        ) != 0
                    {
                        break;
                    }
                    self_restart_count = 0 as ::core::ffi::c_int;
                    dead_check_seconds = 0 as ::core::ffi::c_int;
                    healthy_seconds = 0 as ::core::ffi::c_int;
                    if using_go2rtc {
                        let mut fresh_url: [::core::ffi::c_char; 512] = [0; 512];
                        if go2rtc_get_rtsp_url_0(
                            &raw mut stream_name as *mut ::core::ffi::c_char,
                            &raw mut fresh_url as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                        ) {
                            safe_strcpy(
                                &raw mut restart_url as *mut ::core::ffi::c_char,
                                &raw mut fresh_url as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                                0 as size_t,
                            );
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Refreshed go2rtc URL for stream %s after cooldown\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut stream_name as *mut ::core::ffi::c_char,
                            );
                        }
                    }
                    let mut cooldown_ret: ::core::ffi::c_int = mp4_writer_start_recording_thread(
                        (*ctx).mp4_writer,
                        &raw mut restart_url as *mut ::core::ffi::c_char,
                    );
                    if cooldown_ret < 0 as ::core::ffi::c_int {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to restart inner RTSP thread for stream %s after cooldown \xE2\x80\x94 giving up\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            &raw mut stream_name as *mut ::core::ffi::c_char,
                        );
                        (*ctx).running = 0 as ::core::ffi::c_int;
                        break;
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Stream %s: inner RTSP thread restarted after cooldown\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut stream_name as *mut ::core::ffi::c_char,
                        );
                    }
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Inner RTSP thread for stream %s dead for %d+ s, restarting (attempt %d/%d)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                        dead_check_seconds,
                        self_restart_count + 1 as ::core::ffi::c_int,
                        MAX_SELF_RESTARTS,
                    );
                    if using_go2rtc {
                        let mut fresh_url_0: [::core::ffi::c_char; 512] = [0; 512];
                        if go2rtc_get_rtsp_url_0(
                            &raw mut stream_name as *mut ::core::ffi::c_char,
                            &raw mut fresh_url_0 as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                        ) {
                            safe_strcpy(
                                &raw mut restart_url as *mut ::core::ffi::c_char,
                                &raw mut fresh_url_0 as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                                0 as size_t,
                            );
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_INFO,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Refreshed go2rtc URL for stream %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut stream_name as *mut ::core::ffi::c_char,
                            );
                        }
                    }
                    mp4_writer_stop_recording_thread((*ctx).mp4_writer);
                    let mut restart_ret: ::core::ffi::c_int = mp4_writer_start_recording_thread(
                        (*ctx).mp4_writer,
                        &raw mut restart_url as *mut ::core::ffi::c_char,
                    );
                    if restart_ret < 0 as ::core::ffi::c_int {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Failed to restart inner RTSP thread for stream %s, exiting outer thread\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            &raw mut stream_name as *mut ::core::ffi::c_char,
                        );
                        (*ctx).running = 0 as ::core::ffi::c_int;
                        break;
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Inner RTSP thread for stream %s restarted successfully\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut stream_name as *mut ::core::ffi::c_char,
                        );
                        self_restart_count += 1;
                        dead_check_seconds = 0 as ::core::ffi::c_int;
                        healthy_seconds = 0 as ::core::ffi::c_int;
                    }
                }
            } else {
                if dead_check_seconds > 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Stream %s: recording thread responsive after %d dead seconds (packet or retry activity; not confirmation of a completed segment)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                        dead_check_seconds,
                    );
                }
                dead_check_seconds = 0 as ::core::ffi::c_int;
                healthy_seconds += 1;
                if healthy_seconds >= 120 as ::core::ffi::c_int
                    && self_restart_count > 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Stream %s: stable for %d s, resetting restart counter (was %d)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                        healthy_seconds,
                        self_restart_count,
                    );
                    self_restart_count = 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    if !(*ctx).mp4_writer.is_null() {
        let mut writer: *mut mp4_writer_t = (*ctx).mp4_writer;
        (*ctx).mp4_writer = ::core::ptr::null_mut::<mp4_writer_t>();
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stopping RTSP recording thread for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        mp4_writer_stop_recording_thread(writer);
        unregister_mp4_writer_for_stream(&raw mut stream_name as *mut ::core::ffi::c_char);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Closing MP4 writer for stream %s during thread exit\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        mp4_writer_close(writer);
    }
    (*ctx).running = 0 as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MP4 recording thread for stream %s exited\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_mp4_recording_backend() {
    memset(
        &raw mut recording_contexts as *mut *mut mp4_recording_ctx_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[*mut mp4_recording_ctx_t; 1024]>(),
    );
    ::core::ptr::write_volatile(
        &raw mut shutdown_in_progress,
        0 as ::core::ffi::c_int as sig_atomic_t,
    );
    mp4_segment_recorder_init();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MP4 recording backend initialized\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cleanup_mp4_recording_backend() {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting MP4 recording backend cleanup\0".as_ptr() as *const ::core::ffi::c_char,
    );
    ::core::ptr::write_volatile(
        &raw mut shutdown_in_progress,
        1 as ::core::ffi::c_int as sig_atomic_t,
    );
    let mut capacity: ::core::ffi::c_int = g_config.max_streams;
    if capacity < 1 as ::core::ffi::c_int || capacity > MAX_STREAMS {
        capacity = MAX_STREAMS;
    }
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < capacity {
        if !recording_contexts[i as usize].is_null() {
            (*recording_contexts[i as usize]).running = 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < capacity {
        let mut ctx: *mut mp4_recording_ctx_t = ::core::ptr::null_mut::<mp4_recording_ctx_t>();
        let mut thread: pthread_t = 0;
        let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
        pthread_mutex_lock(&raw mut recording_contexts_mutex);
        if !recording_contexts[i_0 as usize].is_null() {
            ctx = recording_contexts[i_0 as usize];
            thread = (*ctx).thread;
            safe_strcpy(
                &raw mut stream_name as *mut ::core::ffi::c_char,
                &raw mut (*ctx).config.name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
            recording_contexts[i_0 as usize] = ::core::ptr::null_mut::<mp4_recording_ctx_t>();
        }
        pthread_mutex_unlock(&raw mut recording_contexts_mutex);
        if !ctx.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Waiting for MP4 recording thread for %s to exit\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
            let mut join_result: ::core::ffi::c_int = pthread_join_with_timeout(
                thread,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
                15 as ::core::ffi::c_int,
            );
            if join_result != 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Could not join MP4 recording thread for %s within timeout: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                    strerror(join_result),
                );
                pthread_detach(thread);
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Detached MP4 recording thread for %s, skipping context free to avoid use-after-free\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Successfully joined MP4 recording thread for %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
                free(ctx as *mut ::core::ffi::c_void);
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Freed MP4 recording context for %s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
            }
        }
        i_0 += 1;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Cleaning up MP4 segment recorder resources\0".as_ptr() as *const ::core::ffi::c_char,
    );
    mp4_segment_recorder_cleanup();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"MP4 recording backend cleanup complete\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_mp4_recording(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if ::core::ptr::read_volatile::<sig_atomic_t>(&raw const shutdown_in_progress) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot start MP4 recording for %s during shutdown\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: stream_handle_t = get_stream_by_name(stream_name);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s not found for MP4 recording\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
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
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get config for stream %s for MP4 recording\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut claim: ::core::ffi::c_int = claim_start_for_stream(stream_name);
    if claim != 0 as ::core::ffi::c_int {
        return if claim > 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Using standalone recording thread for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    let mut slot: ::core::ffi::c_int = reserve_recording_slot_locked();
    if slot == -1 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut recording_contexts_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"No slot available for new MP4 recording\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
    let mut ctx: *mut mp4_recording_ctx_t =
        malloc(::core::mem::size_of::<mp4_recording_ctx_t>()) as *mut mp4_recording_ctx_t;
    if ctx.is_null() {
        publish_recording_slot(slot, ::core::ptr::null_mut::<mp4_recording_ctx_t>());
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory allocation failed for MP4 recording context\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<mp4_recording_ctx_t>(),
    );
    memcpy(
        &raw mut (*ctx).config as *mut ::core::ffi::c_void,
        &raw mut config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    (*ctx).running = 1 as ::core::ffi::c_int;
    safe_strcpy(
        &raw mut (*ctx).trigger_type as *mut ::core::ffi::c_char,
        if config.record_on_schedule as ::core::ffi::c_int != 0 {
            b"scheduled\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"continuous\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    let mut global_config: *const config_t = get_streaming_config();
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if prepare_placed_mp4_recording_path(
        global_config,
        stream_name,
        now,
        &raw mut (*ctx).output_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        &raw mut (*ctx).placement,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Storage placement blocked MP4 output for %s (reason: %s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
            &raw mut (*ctx).placement.reason as *mut ::core::ffi::c_char,
        );
        publish_recording_slot(slot, ::core::ptr::null_mut::<mp4_recording_ctx_t>());
        free(ctx as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    if pthread_create(
        &raw mut (*ctx).thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            mp4_recording_thread
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        ctx as *mut ::core::ffi::c_void,
    ) != 0 as ::core::ffi::c_int
    {
        recording_slot_reserved[slot as usize] = r#false != 0;
        pthread_mutex_unlock(&raw mut recording_contexts_mutex);
        free(ctx as *mut ::core::ffi::c_void);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create MP4 recording thread for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    recording_contexts[slot as usize] = ctx;
    recording_slot_reserved[slot as usize] = r#false != 0;
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Started MP4 recording for %s in slot %d\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        slot,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_mp4_recording_with_url(
    mut stream_name: *const ::core::ffi::c_char,
    mut url: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if ::core::ptr::read_volatile::<sig_atomic_t>(&raw const shutdown_in_progress) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot start MP4 recording for %s during shutdown\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: stream_handle_t = get_stream_by_name(stream_name);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s not found for MP4 recording\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
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
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get config for stream %s for MP4 recording\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut claim: ::core::ffi::c_int = claim_start_for_stream(stream_name);
    if claim != 0 as ::core::ffi::c_int {
        return if claim > 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Using standalone recording thread for stream %s with custom URL\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
    );
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    let mut slot: ::core::ffi::c_int = reserve_recording_slot_locked();
    if slot == -1 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut recording_contexts_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"No slot available for new MP4 recording\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
    let mut ctx: *mut mp4_recording_ctx_t =
        malloc(::core::mem::size_of::<mp4_recording_ctx_t>()) as *mut mp4_recording_ctx_t;
    if ctx.is_null() {
        publish_recording_slot(slot, ::core::ptr::null_mut::<mp4_recording_ctx_t>());
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory allocation failed for MP4 recording context\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<mp4_recording_ctx_t>(),
    );
    memcpy(
        &raw mut (*ctx).config as *mut ::core::ffi::c_void,
        &raw mut config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    safe_strcpy(
        &raw mut (*ctx).config.url as *mut ::core::ffi::c_char,
        url,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
    (*ctx).running = 1 as ::core::ffi::c_int;
    safe_strcpy(
        &raw mut (*ctx).trigger_type as *mut ::core::ffi::c_char,
        if config.record_on_schedule as ::core::ffi::c_int != 0 {
            b"scheduled\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"continuous\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    let mut global_config: *const config_t = get_streaming_config();
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if prepare_placed_mp4_recording_path(
        global_config,
        stream_name,
        now,
        &raw mut (*ctx).output_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        &raw mut (*ctx).placement,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Storage placement blocked MP4 output for %s (reason: %s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
            &raw mut (*ctx).placement.reason as *mut ::core::ffi::c_char,
        );
        publish_recording_slot(slot, ::core::ptr::null_mut::<mp4_recording_ctx_t>());
        free(ctx as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    if pthread_create(
        &raw mut (*ctx).thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            mp4_recording_thread
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        ctx as *mut ::core::ffi::c_void,
    ) != 0 as ::core::ffi::c_int
    {
        recording_slot_reserved[slot as usize] = r#false != 0;
        pthread_mutex_unlock(&raw mut recording_contexts_mutex);
        free(ctx as *mut ::core::ffi::c_void);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create MP4 recording thread for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    recording_contexts[slot as usize] = ctx;
    recording_slot_reserved[slot as usize] = r#false != 0;
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Started MP4 recording for %s in slot %d\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        slot,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_mp4_recording(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Attempting to stop MP4 recording: %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    let mut ctx: *mut mp4_recording_ctx_t = ::core::ptr::null_mut::<mp4_recording_ctx_t>();
    let mut index: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        if !recording_contexts[i as usize].is_null()
            && strcmp(
                &raw mut (**(&raw mut recording_contexts as *mut *mut mp4_recording_ctx_t)
                    .offset(i as isize))
                .config
                .name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            ctx = recording_contexts[i as usize];
            index = i;
            break;
        } else {
            i += 1;
        }
    }
    if ctx.is_null() {
        pthread_mutex_unlock(&raw mut recording_contexts_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"MP4 recording for stream %s not found for stopping\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    (*ctx).running = 0 as ::core::ffi::c_int;
    recording_contexts[index as usize] = ::core::ptr::null_mut::<mp4_recording_ctx_t>();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Marked MP4 recording for stream %s as stopping (index: %d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
        index,
    );
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
    let mut join_result: ::core::ffi::c_int = pthread_join_with_timeout(
        (*ctx).thread,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        15 as ::core::ffi::c_int,
    );
    if join_result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to join recording thread for stream %s (error: %d), detaching\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
            join_result,
        );
        pthread_detach((*ctx).thread);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stopped MP4 recording for stream %s (thread detached)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return 0 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Successfully joined thread for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    if !(*ctx).mp4_writer.is_null() {
        let mut writer: *mut mp4_writer_t = (*ctx).mp4_writer;
        (*ctx).mp4_writer = ::core::ptr::null_mut::<mp4_writer_t>();
        unregister_mp4_writer_for_stream(stream_name);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Closing MP4 writer for stream %s (thread exited early)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        mp4_writer_close(writer);
    }
    free(ctx as *mut ::core::ffi::c_void);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stopped MP4 recording for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_mp4_recording_with_trigger(
    mut stream_name: *const ::core::ffi::c_char,
    mut trigger_type: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if ::core::ptr::read_volatile::<sig_atomic_t>(&raw const shutdown_in_progress) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot start MP4 recording for %s during shutdown\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: stream_handle_t = get_stream_by_name(stream_name);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s not found for MP4 recording\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
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
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get config for stream %s for MP4 recording\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut claim: ::core::ffi::c_int = claim_start_for_stream(stream_name);
    if claim != 0 as ::core::ffi::c_int {
        return if claim > 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    let mut effective_trigger_type: *const ::core::ffi::c_char = if !trigger_type.is_null()
        && *trigger_type.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        trigger_type
    } else if config.record_on_schedule as ::core::ffi::c_int != 0 {
        b"scheduled\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"continuous\0".as_ptr() as *const ::core::ffi::c_char
    };
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Using standalone recording thread for stream %s with trigger_type: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
        effective_trigger_type,
    );
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    let mut slot: ::core::ffi::c_int = reserve_recording_slot_locked();
    if slot == -1 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut recording_contexts_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"No slot available for new MP4 recording\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
    let mut ctx: *mut mp4_recording_ctx_t =
        malloc(::core::mem::size_of::<mp4_recording_ctx_t>()) as *mut mp4_recording_ctx_t;
    if ctx.is_null() {
        publish_recording_slot(slot, ::core::ptr::null_mut::<mp4_recording_ctx_t>());
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory allocation failed for MP4 recording context\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<mp4_recording_ctx_t>(),
    );
    memcpy(
        &raw mut (*ctx).config as *mut ::core::ffi::c_void,
        &raw mut config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    (*ctx).running = 1 as ::core::ffi::c_int;
    safe_strcpy(
        &raw mut (*ctx).trigger_type as *mut ::core::ffi::c_char,
        effective_trigger_type,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    let mut global_config: *const config_t = get_streaming_config();
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if prepare_placed_mp4_recording_path(
        global_config,
        stream_name,
        now,
        &raw mut (*ctx).output_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        &raw mut (*ctx).placement,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Storage placement blocked MP4 output for %s (reason: %s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
            &raw mut (*ctx).placement.reason as *mut ::core::ffi::c_char,
        );
        publish_recording_slot(slot, ::core::ptr::null_mut::<mp4_recording_ctx_t>());
        free(ctx as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    if pthread_create(
        &raw mut (*ctx).thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            mp4_recording_thread
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        ctx as *mut ::core::ffi::c_void,
    ) != 0 as ::core::ffi::c_int
    {
        recording_slot_reserved[slot as usize] = r#false != 0;
        pthread_mutex_unlock(&raw mut recording_contexts_mutex);
        free(ctx as *mut ::core::ffi::c_void);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create MP4 recording thread for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    recording_contexts[slot as usize] = ctx;
    recording_slot_reserved[slot as usize] = r#false != 0;
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Started MP4 recording for %s in slot %d with trigger_type: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
        slot,
        effective_trigger_type,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_mp4_recording_with_url_and_trigger(
    mut stream_name: *const ::core::ffi::c_char,
    mut url: *const ::core::ffi::c_char,
    mut trigger_type: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if ::core::ptr::read_volatile::<sig_atomic_t>(&raw const shutdown_in_progress) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot start MP4 recording for %s during shutdown\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: stream_handle_t = get_stream_by_name(stream_name);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream %s not found for MP4 recording\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
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
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get config for stream %s for MP4 recording\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut config.url as *mut ::core::ffi::c_char,
        url,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    let mut claim: ::core::ffi::c_int = claim_start_for_stream(stream_name);
    if claim != 0 as ::core::ffi::c_int {
        return if claim > 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    let mut effective_trigger_type: *const ::core::ffi::c_char = if !trigger_type.is_null()
        && *trigger_type.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        trigger_type
    } else if config.record_on_schedule as ::core::ffi::c_int != 0 {
        b"scheduled\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"continuous\0".as_ptr() as *const ::core::ffi::c_char
    };
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Using standalone recording thread for stream %s with trigger_type: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
        effective_trigger_type,
    );
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    let mut slot: ::core::ffi::c_int = reserve_recording_slot_locked();
    if slot == -1 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut recording_contexts_mutex);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"No slot available for new MP4 recording\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
    let mut ctx: *mut mp4_recording_ctx_t =
        malloc(::core::mem::size_of::<mp4_recording_ctx_t>()) as *mut mp4_recording_ctx_t;
    if ctx.is_null() {
        publish_recording_slot(slot, ::core::ptr::null_mut::<mp4_recording_ctx_t>());
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory allocation failed for MP4 recording context\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<mp4_recording_ctx_t>(),
    );
    memcpy(
        &raw mut (*ctx).config as *mut ::core::ffi::c_void,
        &raw mut config as *const ::core::ffi::c_void,
        ::core::mem::size_of::<stream_config_t>(),
    );
    (*ctx).running = 1 as ::core::ffi::c_int;
    safe_strcpy(
        &raw mut (*ctx).trigger_type as *mut ::core::ffi::c_char,
        effective_trigger_type,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    let mut global_config: *const config_t = get_streaming_config();
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if prepare_placed_mp4_recording_path(
        global_config,
        stream_name,
        now,
        &raw mut (*ctx).output_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        &raw mut (*ctx).placement,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Storage placement blocked MP4 output for %s (reason: %s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
            &raw mut (*ctx).placement.reason as *mut ::core::ffi::c_char,
        );
        publish_recording_slot(slot, ::core::ptr::null_mut::<mp4_recording_ctx_t>());
        free(ctx as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    if pthread_create(
        &raw mut (*ctx).thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            mp4_recording_thread
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        ctx as *mut ::core::ffi::c_void,
    ) != 0 as ::core::ffi::c_int
    {
        recording_slot_reserved[slot as usize] = r#false != 0;
        pthread_mutex_unlock(&raw mut recording_contexts_mutex);
        free(ctx as *mut ::core::ffi::c_void);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create MP4 recording thread for %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    recording_contexts[slot as usize] = ctx;
    recording_slot_reserved[slot as usize] = r#false != 0;
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Started MP4 recording for %s in slot %d with trigger_type: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
        slot,
        effective_trigger_type,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signal_all_mp4_recordings_reconnect() {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Signaling all active MP4 recordings to reconnect\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut signaled_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        if !recording_contexts[i as usize].is_null()
            && (*recording_contexts[i as usize]).running != 0
        {
            let mut writer: *mut mp4_writer_t = (*recording_contexts[i as usize]).mp4_writer;
            if !writer.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Signaling reconnect for recording: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (**(&raw mut recording_contexts as *mut *mut mp4_recording_ctx_t)
                        .offset(i as isize))
                    .config
                    .name as *mut ::core::ffi::c_char,
                );
                mp4_writer_signal_reconnect(writer);
                signaled_count += 1;
            }
        }
        i += 1;
    }
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Signaled %d active MP4 recordings to reconnect\0".as_ptr() as *const ::core::ffi::c_char,
        signaled_count,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signal_mp4_recording_reconnect(
    mut stream_name: *const ::core::ffi::c_char,
) {
    if stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Signaling reconnect for MP4 recording of stream: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
    );
    pthread_mutex_lock(&raw mut recording_contexts_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_config.max_streams {
        if !recording_contexts[i as usize].is_null()
            && (*recording_contexts[i as usize]).running != 0
            && strcmp(
                &raw mut (**(&raw mut recording_contexts as *mut *mut mp4_recording_ctx_t)
                    .offset(i as isize))
                .config
                .name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            let mut writer: *mut mp4_writer_t = (*recording_contexts[i as usize]).mp4_writer;
            if !writer.is_null() {
                mp4_writer_signal_reconnect(writer);
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Signaled reconnect for recording: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_name,
                );
            }
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut recording_contexts_mutex);
}
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
