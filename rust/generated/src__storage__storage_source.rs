use ::libc;
unsafe extern "C" {
    pub type sqlite3_stmt;
    pub type sqlite3;
    pub type __dirstream;
    static mut g_config: config_t;
    unsafe fn db_storage_target_get(
        uuid: *const ::core::ffi::c_char,
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
    unsafe fn db_storage_target_resolve_path(
        target_uuid: *const ::core::ffi::c_char,
        object_key: *const ::core::ffi::c_char,
        absolute_path: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_storage_target_mount_guard_active(target: *const storage_target_t) -> bool;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn fdopendir(__fd: ::core::ffi::c_int) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
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
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_changes(_: *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_int(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_int64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_int64(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int)
    -> sqlite3_int64;
    unsafe fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn fstatat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __buf: *mut stat,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn utimensat(
        __fd: ::core::ffi::c_int,
        __path: *const ::core::ffi::c_char,
        __times: *const timespec,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn statvfs(
        __file: *const ::core::ffi::c_char,
        __buf: *mut statvfs,
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn unlinkat(
        __fd: ::core::ffi::c_int,
        __name: *const ::core::ffi::c_char,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn ensure_path(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn storage_s3_download(
        target: *const storage_target_t,
        key: *const ::core::ffi::c_char,
        path: *const ::core::ffi::c_char,
        expected_size: uint64_t,
        expected_checksum: *const ::core::ffi::c_char,
        control: *const storage_transfer_control_t,
        error: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
    unsafe fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
pub type __fsblkcnt_t = ::core::ffi::c_ulong;
pub type __fsfilcnt_t = ::core::ffi::c_ulong;
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
pub union __atomic_wide_counter {
    pub __value64: ::core::ffi::c_ulonglong,
    pub __value32: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
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
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_size: [::core::ffi::c_uint; 2],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2],
    pub __unused_initialized_1: ::core::ffi::c_uint,
    pub __unused_initialized_2: ::core::ffi::c_uint,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::core::ffi::c_char; 48],
    pub __align: ::core::ffi::c_longlong,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_target_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub target_type: [::core::ffi::c_char; 24],
    pub root_path: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub is_default: bool,
    pub storage_class: [::core::ffi::c_char; 16],
    pub reserve_bytes: uint64_t,
    pub high_watermark_pct: ::core::ffi::c_double,
    pub low_watermark_pct: ::core::ffi::c_double,
    pub health_status: [::core::ffi::c_char; 16],
    pub capacity_bytes: uint64_t,
    pub available_bytes: uint64_t,
    pub filesystem_device: uint64_t,
    pub last_probe_at: int64_t,
    pub last_success_at: int64_t,
    pub last_error: [::core::ffi::c_char; 256],
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub recording_count: uint64_t,
    pub recording_bytes: uint64_t,
    pub mount_required: bool,
    pub mount_guard_path: [::core::ffi::c_char; 512],
    pub migration_bandwidth_bps: uint64_t,
    pub archival_window_start_minute: ::core::ffi::c_int,
    pub archival_window_end_minute: ::core::ffi::c_int,
    pub replica_count: uint64_t,
    pub replica_bytes: uint64_t,
    pub endpoint: [::core::ffi::c_char; 512],
    pub region: [::core::ffi::c_char; 64],
    pub bucket: [::core::ffi::c_char; 128],
    pub credential_ref: [::core::ffi::c_char; 128],
    pub archive_budget_bytes: uint64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_storage_target_result_t(pub ::core::ffi::c_int);
impl db_storage_target_result_t {
    pub const DB_STORAGE_TARGET_OK: Self = Self(0);
    pub const DB_STORAGE_TARGET_NOT_FOUND: Self = Self(-1);
    pub const DB_STORAGE_TARGET_CONFLICT: Self = Self(-2);
    pub const DB_STORAGE_TARGET_INVALID: Self = Self(-3);
    pub const DB_STORAGE_TARGET_STALE: Self = Self(-4);
    pub const DB_STORAGE_TARGET_LIMIT: Self = Self(-5);
    pub const DB_STORAGE_TARGET_IN_USE: Self = Self(-6);
    pub const DB_STORAGE_TARGET_UNAVAILABLE: Self = Self(-7);
    pub const DB_STORAGE_TARGET_ERROR: Self = Self(-8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_0 {
    pub const STORAGE_SOURCE_READY: Self = Self(0);
    pub const STORAGE_SOURCE_PREPARING: Self = Self(1);
    pub const STORAGE_SOURCE_ERROR: Self = Self(-1);
    pub const STORAGE_SOURCE_MISSING: Self = Self(-2);
    pub const STORAGE_SOURCE_DELETING: Self = Self(-3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_remote_source_t {
    pub recording_id: uint64_t,
    pub target: storage_target_t,
    pub key: [::core::ffi::c_char; 512],
    pub size: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_source_t {
    pub target: [::core::ffi::c_char; 37],
    pub key: [::core::ffi::c_char; 512],
    pub path: [::core::ffi::c_char; 512],
    pub checksum: [::core::ffi::c_char; 65],
    pub bytes: uint64_t,
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
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statvfs {
    pub f_bsize: ::core::ffi::c_ulong,
    pub f_frsize: ::core::ffi::c_ulong,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_favail: __fsfilcnt_t,
    pub f_fsid: ::core::ffi::c_ulong,
    pub f_flag: ::core::ffi::c_ulong,
    pub f_namemax: ::core::ffi::c_ulong,
    pub f_type: ::core::ffi::c_uint,
    pub __f_spare: [::core::ffi::c_int; 5],
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fetch_context_t {
    pub recording_id: uint64_t,
    pub reserve_bytes: uint64_t,
    pub space_checked_at: time_t,
}
pub type atomic_bool = bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_transfer_control_t {
    pub cancelled: storage_transfer_cancel_fn,
    pub context: *mut ::core::ffi::c_void,
    pub bandwidth_bps: uint64_t,
    pub job_uuid: *const ::core::ffi::c_char,
}
pub type storage_transfer_cancel_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> bool>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const UTIME_NOW: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_long) << 30 as ::core::ffi::c_int) - 1 as ::core::ffi::c_long;
pub const UTIME_OMIT: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_long) << 30 as ::core::ffi::c_int) - 2 as ::core::ffi::c_long;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __O_DIRECTORY: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const O_DIRECTORY: ::core::ffi::c_int = __O_DIRECTORY;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const AT_FDCWD: ::core::ffi::c_int = -100 as ::core::ffi::c_int;
pub const AT_SYMLINK_NOFOLLOW: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __data: __pthread_cond_s {
        __wseq: __atomic_wide_counter {
            __value64: 0 as ::core::ffi::c_ulonglong,
        },
        __g1_start: __atomic_wide_counter {
            __value64: 0 as ::core::ffi::c_ulonglong,
        },
        __g_size: [0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint],
        __g1_orig_size: 0 as ::core::ffi::c_uint,
        __wrefs: 0 as ::core::ffi::c_uint,
        __g_signals: [0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint],
        __unused_initialized_1: 0 as ::core::ffi::c_uint,
        __unused_initialized_2: 0 as ::core::ffi::c_uint,
    },
};
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SOURCE_LEASE_SECONDS: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
pub const SOURCE_QUEUE_LIMIT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
static mut source_thread: pthread_t = 0;
static mut source_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_1::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut source_condition: pthread_cond_t = PTHREAD_COND_INITIALIZER;
static mut source_running: bool = false;
static mut source_stop: atomic_bool = false;
unsafe extern "C" fn capture_space(
    mut available: *mut uint64_t,
    mut reserve: *mut uint64_t,
) -> bool {
    let mut fs: statvfs = statvfs {
        f_bsize: 0,
        f_frsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_favail: 0,
        f_fsid: 0,
        f_flag: 0,
        f_namemax: 0,
        f_type: 0,
        __f_spare: [0; 5],
    };
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
    if statvfs(
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
        &raw mut fs,
    ) != 0
        || stat(
            &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
            &raw mut info,
        ) != 0
    {
        return r#false != 0;
    }
    let mut pct: ::core::ffi::c_uint = if g_config.storage_min_free_pct > 0 as ::core::ffi::c_int {
        g_config.storage_min_free_pct as ::core::ffi::c_uint
    } else {
        10 as ::core::ffi::c_uint
    };
    if pct > 100 as ::core::ffi::c_uint {
        pct = 100 as ::core::ffi::c_uint;
    }
    *available = (fs.f_bavail as uint64_t).wrapping_mul(fs.f_frsize as uint64_t);
    *reserve = (fs.f_blocks as uint64_t)
        .wrapping_mul(fs.f_frsize as uint64_t)
        .wrapping_div(100 as uint64_t)
        .wrapping_mul(pct as uint64_t);
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return r#false != 0;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut found: bool = r#false != 0;
    if sqlite3_prepare_v2(
        db,
        b"SELECT COALESCE(MAX(reserve_bytes),0) FROM storage_targets WHERE target_type='filesystem' AND (filesystem_device=? OR filesystem_device=0);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, info.st_dev as sqlite3_int64);
        if sqlite3_step(stmt) == SQLITE_ROW {
            let mut target_reserve: uint64_t = sqlite3_column_int64(
                stmt,
                0 as ::core::ffi::c_int,
            ) as uint64_t;
            if target_reserve > *reserve {
                *reserve = target_reserve;
            }
            found = r#true != 0;
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return found;
}
unsafe extern "C" fn cache_limit() -> uint64_t {
    let mut value: *const ::core::ffi::c_char =
        getenv(b"LIGHTNVR_ARCHIVE_CACHE_MB\0".as_ptr() as *const ::core::ffi::c_char);
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut mb: ::core::ffi::c_ulonglong = if !value.is_null() {
        strtoull(value, &raw mut end, 10 as ::core::ffi::c_int)
    } else {
        2048 as ::core::ffi::c_ulonglong
    };
    if !value.is_null()
        && (end.is_null()
            || *end as ::core::ffi::c_int != 0
            || mb > 1048576 as ::core::ffi::c_ulonglong)
    {
        mb = 2048 as ::core::ffi::c_ulonglong;
    }
    return mb
        .wrapping_mul(1024 as ::core::ffi::c_ulonglong)
        .wrapping_mul(1024 as ::core::ffi::c_ulonglong) as uint64_t;
}
unsafe extern "C" fn source_lease(mut db: *mut sqlite3, mut id: uint64_t) -> bool {
    let mut saved: bool = r#false != 0;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        b"INSERT INTO storage_read_leases(recording_id,expires_at) VALUES(?,strftime('%s','now')+120) ON CONFLICT(recording_id) DO UPDATE SET expires_at=excluded.expires_at;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        saved = sqlite3_step(statement) == SQLITE_DONE
            && sqlite3_changes(db) == 1 as ::core::ffi::c_int;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return saved;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_source_renew_lease(mut id: uint64_t) -> bool {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return r#false != 0;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut renewed: bool = r#false != 0;
    if sqlite3_prepare_v2(
        db,
        b"UPDATE storage_read_leases SET expires_at=strftime('%s','now')+120 WHERE recording_id=? AND EXISTS(SELECT 1 FROM recordings WHERE id=recording_id);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        renewed = sqlite3_step(stmt) == SQLITE_DONE
            && sqlite3_changes(db) == 1 as ::core::ffi::c_int;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return renewed;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_source_touch(mut id: uint64_t) -> bool {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return r#false != 0;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut ready: bool = r#false != 0;
    if sqlite3_prepare_v2(
        db,
        b"SELECT 1 FROM recordings WHERE id=? AND deletion_pending=0;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        ready = sqlite3_step(stmt) == SQLITE_ROW;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if ready {
        ready = source_lease(db, id);
    }
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if ready as ::core::ffi::c_int != 0
        && sqlite3_prepare_v2(
            db,
            b"SELECT file_path FROM storage_retrieval_jobs WHERE recording_id=? AND state='ready';\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
    {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        if sqlite3_step(stmt) == SQLITE_ROW {
            let mut times: [timespec; 2] = [
                timespec {
                    tv_sec: 0,
                    tv_nsec: UTIME_NOW,
                },
                timespec {
                    tv_sec: 0,
                    tv_nsec: UTIME_OMIT,
                },
            ];
            utimensat(
                AT_FDCWD,
                sqlite3_column_text(stmt, 0 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                &raw mut times as *mut timespec,
                AT_SYMLINK_NOFOLLOW,
            );
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if ready as ::core::ffi::c_int != 0
        && sqlite3_prepare_v2(
            db,
            b"UPDATE storage_retrieval_jobs SET last_access_at=strftime('%s','now') WHERE recording_id=?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
    {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return ready;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_source_has_lease(mut id: uint64_t) -> bool {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return r#true != 0;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut active: bool = r#true != 0;
    if sqlite3_prepare_v2(
        db,
        b"SELECT 1 FROM storage_read_leases WHERE recording_id=? AND expires_at>strftime('%s','now');\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        active = sqlite3_step(statement) != SQLITE_DONE;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return active;
}
unsafe extern "C" fn trim_cache_without_catalog(mut needed: uint64_t) -> uint64_t {
    let mut root: ::core::ffi::c_int = open(
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
        O_RDONLY | O_DIRECTORY | O_CLOEXEC | O_NOFOLLOW,
    );
    if root < 0 as ::core::ffi::c_int {
        return 0 as uint64_t;
    }
    let mut fd: ::core::ffi::c_int = openat(
        root,
        b"archive-cache\0".as_ptr() as *const ::core::ffi::c_char,
        O_RDONLY | O_DIRECTORY | O_CLOEXEC | O_NOFOLLOW,
    );
    close(root);
    if fd < 0 as ::core::ffi::c_int {
        return 0 as uint64_t;
    }
    let mut directory: *mut DIR = fdopendir(fd);
    if directory.is_null() {
        close(fd);
        return 0 as uint64_t;
    }
    let mut removed: uint64_t = 0 as uint64_t;
    let mut cutoff: time_t = time(::core::ptr::null_mut::<time_t>())
        - (if needed != 0 {
            SOURCE_LEASE_SECONDS
        } else {
            3600 as ::core::ffi::c_int
        }) as time_t;
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    let mut scanned: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut deleted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let c2rust_fresh1 = scanned;
        scanned += 1;
        if !(c2rust_fresh1 < 4096 as ::core::ffi::c_int && deleted < 32 as ::core::ffi::c_int && {
            entry = readdir(directory);
            !entry.is_null()
        }) {
            break;
        }
        let mut name: *const ::core::ffi::c_char =
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char;
        let mut p: *const ::core::ffi::c_char = name;
        while *p as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *p as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        if p == name || p.offset_from(name) > 20isize || {
            let c2rust_fresh2 = p;
            p = p.offset(1);
            *c2rust_fresh2 as ::core::ffi::c_int != '-' as ::core::ffi::c_int
        } {
            continue;
        }
        if strlen(p) != 68 as size_t
            || strcmp(
                p.offset(64 as ::core::ffi::c_int as isize),
                b".mp4\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0
        {
            continue;
        }
        let mut valid: bool = r#true != 0;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 64 as ::core::ffi::c_int {
            if !(*p.offset(i as isize) as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && *p.offset(i as isize) as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                || *p.offset(i as isize) as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                    && *p.offset(i as isize) as ::core::ffi::c_int <= 'f' as ::core::ffi::c_int)
            {
                valid = r#false != 0;
            }
            i += 1;
        }
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
        if !valid
            || fstatat(fd, name, &raw mut info, AT_SYMLINK_NOFOLLOW) != 0
            || !(info.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
            || info.st_mtim.tv_sec >= cutoff
            || info.st_atim.tv_sec >= cutoff
        {
            continue;
        }
        if unlinkat(fd, name, 0 as ::core::ffi::c_int) != 0 {
            continue;
        }
        removed = removed.wrapping_add(info.st_size as uint64_t);
        deleted += 1;
        if needed != 0 && removed >= needed {
            break;
        }
    }
    closedir(directory);
    return removed;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_source_trim(mut needed: uint64_t) -> uint64_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return trim_cache_without_catalog(needed);
    }
    let mut removed: uint64_t = 0 as uint64_t;
    let mut catalog_failed: bool = r#false != 0;
    pthread_mutex_lock(mutex);
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while n < 32 as ::core::ffi::c_int {
        let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut id: uint64_t = 0 as uint64_t;
        let mut bytes: uint64_t = 0 as uint64_t;
        let mut path: [::core::ffi::c_char; 512] = [
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
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        let mut sql: *const ::core::ffi::c_char = b"SELECT recording_id,file_path,size_bytes FROM storage_retrieval_jobs j WHERE state IN('ready','failed') AND last_access_at<strftime('%s','now')-? AND NOT EXISTS(SELECT 1 FROM storage_read_leases l WHERE l.recording_id=j.recording_id AND l.expires_at>strftime('%s','now')) ORDER BY last_access_at LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char;
        if sqlite3_prepare_v2(
            db,
            sql,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
        {
            sqlite3_bind_int(
                statement,
                1 as ::core::ffi::c_int,
                if needed != 0 {
                    SOURCE_LEASE_SECONDS
                } else {
                    3600 as ::core::ffi::c_int
                },
            );
            let mut rc: ::core::ffi::c_int = sqlite3_step(statement);
            catalog_failed = rc != SQLITE_ROW && rc != SQLITE_DONE;
            if rc == SQLITE_ROW {
                id = sqlite3_column_int64(statement, 0 as ::core::ffi::c_int) as uint64_t;
                safe_strcpy(
                    &raw mut path as *mut ::core::ffi::c_char,
                    sqlite3_column_text(statement, 1 as ::core::ffi::c_int)
                        as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    0 as size_t,
                );
                bytes = sqlite3_column_int64(statement, 2 as ::core::ffi::c_int) as uint64_t;
            }
        } else {
            catalog_failed = r#true != 0;
        }
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
        if id == 0 {
            break;
        }
        let mut deleted: ::core::ffi::c_int = unlink(&raw mut path as *mut ::core::ffi::c_char);
        if deleted != 0 as ::core::ffi::c_int && *__errno_location() != ENOENT {
            break;
        }
        if deleted == 0 as ::core::ffi::c_int {
            removed = removed.wrapping_add(bytes);
        }
        statement = ::core::ptr::null_mut::<sqlite3_stmt>();
        if sqlite3_prepare_v2(
            db,
            b"DELETE FROM storage_retrieval_jobs WHERE recording_id=?;\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
        {
            sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
            sqlite3_step(statement);
        }
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
        if needed != 0 && removed >= needed {
            break;
        }
        n += 1;
    }
    pthread_mutex_unlock(mutex);
    if catalog_failed {
        removed = removed.wrapping_add(trim_cache_without_catalog(if needed > removed {
            needed.wrapping_sub(removed)
        } else {
            0 as uint64_t
        }));
    }
    return removed;
}
unsafe extern "C" fn select_source(
    mut id: uint64_t,
    mut selected: *mut recording_source_t,
    mut available: *mut bool,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    *available = r#false != 0;
    if db.is_null() || mutex.is_null() {
        return C2Rust_Unnamed_0::STORAGE_SOURCE_ERROR.0;
    }
    let mut sources: [recording_source_t; 9] = [recording_source_t {
        target: [0; 37],
        key: [0; 512],
        path: [0; 512],
        checksum: [0; 65],
        bytes: 0,
    }; 9];
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char = b"SELECT COALESCE(storage_target_uuid,''),COALESCE(object_key,''),file_path,archive_checksum,size_bytes,deletion_pending FROM recordings WHERE id=?1 UNION ALL SELECT c.target_uuid,c.object_key,'',c.checksum,c.size_bytes,r.deletion_pending FROM storage_recording_copies c JOIN recordings r ON r.id=c.recording_id WHERE r.id=?1 UNION ALL SELECT j.source_target_uuid,j.source_object_key,'',j.checksum,j.bytes_total,r.deletion_pending FROM storage_migration_jobs j JOIN recordings r ON r.id=j.recording_id WHERE r.id=?1 AND j.state='cleanup_pending' LIMIT 9;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        while count < 9 as ::core::ffi::c_int && {
            rc = sqlite3_step(statement);
            rc == SQLITE_ROW
        } {
            if sqlite3_column_int(statement, 5 as ::core::ffi::c_int) != 0 {
                sqlite3_finalize(statement);
                pthread_mutex_unlock(mutex);
                safe_strcpy(
                    error,
                    b"Recording deletion is pending\0".as_ptr() as *const ::core::ffi::c_char,
                    256 as size_t,
                    0 as size_t,
                );
                return C2Rust_Unnamed_0::STORAGE_SOURCE_DELETING.0;
            }
            let c2rust_fresh0 = count;
            count += 1;
            let mut source: *mut recording_source_t =
                (&raw mut sources as *mut recording_source_t).offset(c2rust_fresh0 as isize);
            memset(
                source as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<recording_source_t>(),
            );
            safe_strcpy(
                &raw mut (*source).target as *mut ::core::ffi::c_char,
                sqlite3_column_text(statement, 0 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*source).key as *mut ::core::ffi::c_char,
                sqlite3_column_text(statement, 1 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*source).path as *mut ::core::ffi::c_char,
                sqlite3_column_text(statement, 2 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*source).checksum as *mut ::core::ffi::c_char,
                sqlite3_column_text(statement, 3 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
                0 as size_t,
            );
            (*source).bytes = sqlite3_column_int64(statement, 4 as ::core::ffi::c_int) as uint64_t;
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    let mut job_target: [::core::ffi::c_char; 37] = [
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
    ];
    let mut job_key: [::core::ffi::c_char; 512] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut job_state: [::core::ffi::c_char; 16] = [
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
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        b"SELECT target_uuid,object_key,state FROM storage_retrieval_jobs WHERE recording_id=?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        if sqlite3_step(statement) == SQLITE_ROW {
            safe_strcpy(
                &raw mut job_target as *mut ::core::ffi::c_char,
                sqlite3_column_text(statement, 0 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut job_key as *mut ::core::ffi::c_char,
                sqlite3_column_text(statement, 1 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut job_state as *mut ::core::ffi::c_char,
                sqlite3_column_text(statement, 2 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    if count == 0 {
        return if rc == SQLITE_DONE {
            C2Rust_Unnamed_0::STORAGE_SOURCE_MISSING.0
        } else {
            C2Rust_Unnamed_0::STORAGE_SOURCE_ERROR.0
        };
    }
    let mut best: ::core::ffi::c_int = -10000 as ::core::ffi::c_int;
    let mut external: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut source_0: *mut recording_source_t =
            (&raw mut sources as *mut recording_source_t).offset(i as isize);
        's_157: {
            if (*source_0).target[0usize] != 0 {
                let mut target: storage_target_t = storage_target_t {
                    uuid: [0; 37],
                    name: [0; 128],
                    target_type: [0; 24],
                    root_path: [0; 512],
                    enabled: false,
                    is_default: false,
                    storage_class: [0; 16],
                    reserve_bytes: 0,
                    high_watermark_pct: 0.,
                    low_watermark_pct: 0.,
                    health_status: [0; 16],
                    capacity_bytes: 0,
                    available_bytes: 0,
                    filesystem_device: 0,
                    last_probe_at: 0,
                    last_success_at: 0,
                    last_error: [0; 256],
                    revision: 0,
                    created_at: 0,
                    updated_at: 0,
                    recording_count: 0,
                    recording_bytes: 0,
                    mount_required: false,
                    mount_guard_path: [0; 512],
                    migration_bandwidth_bps: 0,
                    archival_window_start_minute: 0,
                    archival_window_end_minute: 0,
                    replica_count: 0,
                    replica_bytes: 0,
                    endpoint: [0; 512],
                    region: [0; 64],
                    bucket: [0; 128],
                    credential_ref: [0; 128],
                    archive_budget_bytes: 0,
                };
                if db_storage_target_get(
                    &raw mut (*source_0).target as *mut ::core::ffi::c_char,
                    &raw mut target,
                )
                .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
                {
                    break 's_157;
                } else if strcmp(
                    &raw mut target.target_type as *mut ::core::ffi::c_char,
                    b"s3\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0
                {
                    if strlen(&raw mut (*source_0).checksum as *mut ::core::ffi::c_char)
                        != 64 as size_t
                    {
                        break 's_157;
                    } else {
                        let mut current: bool = strcmp(
                            &raw mut job_target as *mut ::core::ffi::c_char,
                            &raw mut (*source_0).target as *mut ::core::ffi::c_char,
                        ) == 0
                            && strcmp(
                                &raw mut job_key as *mut ::core::ffi::c_char,
                                &raw mut (*source_0).key as *mut ::core::ffi::c_char,
                            ) == 0;
                        let mut failed: bool = current as ::core::ffi::c_int != 0
                            && strcmp(
                                &raw mut job_state as *mut ::core::ffi::c_char,
                                b"failed\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0;
                        let mut healthy: bool = strcmp(
                            &raw mut target.health_status as *mut ::core::ffi::c_char,
                            b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
                        ) != 0 as ::core::ffi::c_int;
                        let mut score: ::core::ffi::c_int = if healthy as ::core::ffi::c_int != 0 {
                            100 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } - if failed as ::core::ffi::c_int != 0
                        {
                            200 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        };
                        if current as ::core::ffi::c_int != 0 && !failed {
                            score += 500 as ::core::ffi::c_int;
                        }
                        if current as ::core::ffi::c_int != 0
                            && strcmp(
                                &raw mut job_state as *mut ::core::ffi::c_char,
                                b"fetching\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0
                        {
                            score = 1000 as ::core::ffi::c_int;
                        }
                        if score > best {
                            *selected = *source_0;
                            *available = healthy as ::core::ffi::c_int != 0 && !failed;
                            external = r#true != 0;
                            best = score;
                        }
                        break 's_157;
                    }
                } else if !db_storage_target_mount_guard_active(&raw mut target)
                    || db_storage_target_resolve_path(
                        &raw mut (*source_0).target as *mut ::core::ffi::c_char,
                        &raw mut (*source_0).key as *mut ::core::ffi::c_char,
                        &raw mut (*source_0).path as *mut ::core::ffi::c_char,
                    ) != 0
                {
                    break 's_157;
                }
            }
            let mut status: stat = stat {
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
            if (*source_0).path[0usize] as ::core::ffi::c_int != 0
                && stat(
                    &raw mut (*source_0).path as *mut ::core::ffi::c_char,
                    &raw mut status,
                ) == 0 as ::core::ffi::c_int
                && status.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
            {
                *selected = *source_0;
                *available = r#true != 0;
                return C2Rust_Unnamed_0::STORAGE_SOURCE_READY.0;
            }
        }
        i += 1;
    }
    if external {
        if strcmp(
            &raw mut job_state as *mut ::core::ffi::c_char,
            b"ready\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
            && strcmp(
                &raw mut job_target as *mut ::core::ffi::c_char,
                &raw mut (*selected).target as *mut ::core::ffi::c_char,
            ) == 0
            && strcmp(
                &raw mut job_key as *mut ::core::ffi::c_char,
                &raw mut (*selected).key as *mut ::core::ffi::c_char,
            ) == 0
        {
            let mut cached: [::core::ffi::c_char; 512] = [0; 512];
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
            let mut n: ::core::ffi::c_int = snprintf(
                &raw mut cached as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                b"%s/archive-cache/%llu-%s.mp4\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
                id as ::core::ffi::c_ulonglong,
                &raw mut (*selected).checksum as *mut ::core::ffi::c_char,
            );
            if n > 0 as ::core::ffi::c_int
                && n < ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int
                && stat(&raw mut cached as *mut ::core::ffi::c_char, &raw mut info)
                    == 0 as ::core::ffi::c_int
                && info.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
                && info.st_size as uint64_t == (*selected).bytes
            {
                safe_strcpy(
                    &raw mut (*selected).path as *mut ::core::ffi::c_char,
                    &raw mut cached as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    0 as size_t,
                );
                *available = r#true != 0;
                return C2Rust_Unnamed_0::STORAGE_SOURCE_READY.0;
            }
        }
        return C2Rust_Unnamed_0::STORAGE_SOURCE_PREPARING.0;
    }
    safe_strcpy(
        error,
        b"No readable recording copy is currently available\0".as_ptr()
            as *const ::core::ffi::c_char,
        256 as size_t,
        0 as size_t,
    );
    return C2Rust_Unnamed_0::STORAGE_SOURCE_MISSING.0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_source_available(mut id: uint64_t) -> bool {
    let mut source: recording_source_t = recording_source_t {
        target: [0; 37],
        key: [0; 512],
        path: [0; 512],
        checksum: [0; 65],
        bytes: 0,
    };
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
    let mut available: bool = false;
    select_source(
        id,
        &raw mut source,
        &raw mut available,
        &raw mut error as *mut ::core::ffi::c_char,
    );
    return available;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_source_remote(
    mut id: uint64_t,
    mut source: *mut storage_remote_source_t,
) -> ::core::ffi::c_int {
    if source.is_null() {
        return C2Rust_Unnamed_0::STORAGE_SOURCE_ERROR.0;
    }
    let mut selected: recording_source_t = recording_source_t {
        target: [0; 37],
        key: [0; 512],
        path: [0; 512],
        checksum: [0; 65],
        bytes: 0,
    };
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
    let mut available: bool = false;
    let mut result: ::core::ffi::c_int = select_source(
        id,
        &raw mut selected,
        &raw mut available,
        &raw mut error as *mut ::core::ffi::c_char,
    );
    if result == C2Rust_Unnamed_0::STORAGE_SOURCE_READY.0 {
        return C2Rust_Unnamed_0::STORAGE_SOURCE_MISSING.0;
    }
    if result != C2Rust_Unnamed_0::STORAGE_SOURCE_PREPARING.0 {
        return result;
    }
    if !storage_source_touch(id) {
        return C2Rust_Unnamed_0::STORAGE_SOURCE_DELETING.0;
    }
    memset(
        source as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_remote_source_t>(),
    );
    (*source).recording_id = id;
    safe_strcpy(
        &raw mut (*source).key as *mut ::core::ffi::c_char,
        &raw mut selected.key as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    (*source).size = selected.bytes;
    return if db_storage_target_get(
        &raw mut selected.target as *mut ::core::ffi::c_char,
        &raw mut (*source).target,
    )
    .0 == db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        C2Rust_Unnamed_0::STORAGE_SOURCE_READY.0
    } else {
        C2Rust_Unnamed_0::STORAGE_SOURCE_ERROR.0
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_source_resolve(
    mut id: uint64_t,
    mut path: *mut ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if id == 0 || path.is_null() || error.is_null() {
        return C2Rust_Unnamed_0::STORAGE_SOURCE_ERROR.0;
    }
    *path.offset(0isize) = 0 as ::core::ffi::c_char;
    *error.offset(0isize) = 0 as ::core::ffi::c_char;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return C2Rust_Unnamed_0::STORAGE_SOURCE_ERROR.0;
    }
    let mut selected: recording_source_t = recording_source_t {
        target: [0; 37],
        key: [0; 512],
        path: [0; 512],
        checksum: [0; 65],
        bytes: 0,
    };
    let mut available: bool = false;
    let mut result: ::core::ffi::c_int =
        select_source(id, &raw mut selected, &raw mut available, error);
    if result == C2Rust_Unnamed_0::STORAGE_SOURCE_READY.0 {
        if !storage_source_touch(id) {
            return C2Rust_Unnamed_0::STORAGE_SOURCE_DELETING.0;
        }
        safe_strcpy(
            path,
            &raw mut selected.path as *mut ::core::ffi::c_char,
            MAX_PATH_LENGTH as size_t,
            0 as size_t,
        );
        return C2Rust_Unnamed_0::STORAGE_SOURCE_READY.0;
    }
    if result != C2Rust_Unnamed_0::STORAGE_SOURCE_PREPARING.0 {
        return result;
    }
    if !storage_source_touch(id) {
        return C2Rust_Unnamed_0::STORAGE_SOURCE_DELETING.0;
    }
    let mut source: *mut recording_source_t = &raw mut selected;
    if strlen(&raw mut (*source).checksum as *mut ::core::ffi::c_char) != 64 as size_t {
        safe_strcpy(
            error,
            b"Archive copy has no verified checksum\0".as_ptr() as *const ::core::ffi::c_char,
            256 as size_t,
            0 as size_t,
        );
        return C2Rust_Unnamed_0::STORAGE_SOURCE_ERROR.0;
    }
    let mut n: ::core::ffi::c_int = snprintf(
        path,
        MAX_PATH_LENGTH as size_t,
        b"%s/archive-cache/%llu-%s.mp4\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
        &raw mut (*source).checksum as *mut ::core::ffi::c_char,
    );
    if n < 0 as ::core::ffi::c_int || n >= MAX_PATH_LENGTH || ensure_path(path) != 0 {
        safe_strcpy(
            error,
            b"Cannot create recording retrieval cache\0".as_ptr() as *const ::core::ffi::c_char,
            256 as size_t,
            0 as size_t,
        );
        return C2Rust_Unnamed_0::STORAGE_SOURCE_ERROR.0;
    }
    storage_source_trim((*source).bytes);
    let mut free_bytes: uint64_t = 0;
    let mut reserve: uint64_t = 0;
    if !capture_space(&raw mut free_bytes, &raw mut reserve) {
        return C2Rust_Unnamed_0::STORAGE_SOURCE_ERROR.0;
    }
    let mut limit: uint64_t = cache_limit();
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut state: ::core::ffi::c_int = C2Rust_Unnamed_0::STORAGE_SOURCE_PREPARING.0;
    if sqlite3_prepare_v2(
        db,
        b"SELECT state,last_error,next_attempt_at FROM storage_retrieval_jobs WHERE recording_id=? AND target_uuid=? AND object_key=?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            &raw mut (*source).target as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            &raw mut (*source).key as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if sqlite3_step(statement) == SQLITE_ROW {
            let mut value: *const ::core::ffi::c_char = sqlite3_column_text(
                statement,
                0 as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char;
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
                st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                __glibc_reserved: [0; 3],
            };
            if strcmp(value, b"ready\0".as_ptr() as *const ::core::ffi::c_char) == 0
                && stat(path, &raw mut info) == 0 as ::core::ffi::c_int
                && info.st_size as uint64_t == (*source).bytes
            {
                state = C2Rust_Unnamed_0::STORAGE_SOURCE_READY.0;
            } else if strcmp(value, b"failed\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
                && sqlite3_column_int64(statement, 2 as ::core::ffi::c_int)
                    > time(::core::ptr::null_mut::<time_t>()) as sqlite3_int64
            {
                safe_strcpy(
                    error,
                    sqlite3_column_text(statement, 1 as ::core::ffi::c_int)
                        as *const ::core::ffi::c_char,
                    256 as size_t,
                    0 as size_t,
                );
                state = C2Rust_Unnamed_0::STORAGE_SOURCE_ERROR.0;
            }
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if state == C2Rust_Unnamed_0::STORAGE_SOURCE_PREPARING.0 {
        let mut used: uint64_t = 0 as uint64_t;
        let mut pending_bytes: uint64_t = 0 as uint64_t;
        let mut jobs: ::core::ffi::c_int = SOURCE_QUEUE_LIMIT;
        statement = ::core::ptr::null_mut::<sqlite3_stmt>();
        if sqlite3_prepare_v2(
            db,
            b"SELECT COALESCE(sum(size_bytes),0),COALESCE(sum(CASE WHEN state IN('queued','fetching') THEN size_bytes ELSE 0 END),0),COALESCE(sum(state IN('queued','fetching')),0) FROM storage_retrieval_jobs WHERE recording_id<>?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
        {
            sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
            if sqlite3_step(statement) == SQLITE_ROW {
                used = sqlite3_column_int64(statement, 0 as ::core::ffi::c_int)
                    as uint64_t;
                pending_bytes = sqlite3_column_int64(statement, 1 as ::core::ffi::c_int)
                    as uint64_t;
                jobs = sqlite3_column_int(statement, 2 as ::core::ffi::c_int);
            }
        }
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
        if (*source).bytes > limit
            || used > limit.wrapping_sub((*source).bytes)
            || free_bytes <= reserve
            || pending_bytes > free_bytes.wrapping_sub(reserve)
            || (*source).bytes > free_bytes.wrapping_sub(reserve).wrapping_sub(pending_bytes)
            || jobs >= SOURCE_QUEUE_LIMIT
        {
            safe_strcpy(
                error,
                b"Archive playback cache is full or capture reserve would be exceeded\0".as_ptr()
                    as *const ::core::ffi::c_char,
                256 as size_t,
                0 as size_t,
            );
            state = C2Rust_Unnamed_0::STORAGE_SOURCE_ERROR.0;
        } else {
            statement = ::core::ptr::null_mut::<sqlite3_stmt>();
            let mut insert: *const ::core::ffi::c_char = b"INSERT INTO storage_retrieval_jobs(recording_id,target_uuid,object_key,checksum,size_bytes,file_path) SELECT ?1,?2,?3,?4,?5,?6 FROM recordings WHERE id=?1 AND deletion_pending=0 ON CONFLICT(recording_id) DO UPDATE SET state=CASE WHEN storage_retrieval_jobs.state IN('ready','failed') THEN 'queued' ELSE storage_retrieval_jobs.state END,last_access_at=strftime('%s','now'),target_uuid=excluded.target_uuid,object_key=excluded.object_key,checksum=excluded.checksum,size_bytes=excluded.size_bytes,file_path=excluded.file_path WHERE storage_retrieval_jobs.state<>'fetching';\0"
                .as_ptr() as *const ::core::ffi::c_char;
            if sqlite3_prepare_v2(
                db,
                insert,
                -1 as ::core::ffi::c_int,
                &raw mut statement,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            ) == SQLITE_OK
            {
                sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
                sqlite3_bind_text(
                    statement,
                    2 as ::core::ffi::c_int,
                    &raw mut (*source).target as *mut ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                sqlite3_bind_text(
                    statement,
                    3 as ::core::ffi::c_int,
                    &raw mut (*source).key as *mut ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                sqlite3_bind_text(
                    statement,
                    4 as ::core::ffi::c_int,
                    &raw mut (*source).checksum as *mut ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                sqlite3_bind_int64(
                    statement,
                    5 as ::core::ffi::c_int,
                    (*source).bytes as sqlite3_int64,
                );
                sqlite3_bind_text(
                    statement,
                    6 as ::core::ffi::c_int,
                    path,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                if sqlite3_step(statement) != SQLITE_DONE {
                    state = C2Rust_Unnamed_0::STORAGE_SOURCE_ERROR.0;
                }
            } else {
                state = C2Rust_Unnamed_0::STORAGE_SOURCE_ERROR.0;
            }
            if !statement.is_null() {
                sqlite3_finalize(statement);
            }
        }
    }
    if state == C2Rust_Unnamed_0::STORAGE_SOURCE_READY.0 {
        statement = ::core::ptr::null_mut::<sqlite3_stmt>();
        if sqlite3_prepare_v2(
            db,
            b"UPDATE storage_retrieval_jobs SET last_access_at=strftime('%s','now') WHERE recording_id=?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
        {
            sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
            sqlite3_step(statement);
        }
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
    }
    pthread_mutex_unlock(mutex);
    if state == C2Rust_Unnamed_0::STORAGE_SOURCE_PREPARING.0 {
        pthread_mutex_lock(&raw mut source_mutex);
        pthread_cond_signal(&raw mut source_condition);
        pthread_mutex_unlock(&raw mut source_mutex);
    }
    return state;
}
unsafe extern "C" fn fetch_cancelled(mut context: *mut ::core::ffi::c_void) -> bool {
    if ::core::sync::atomic::AtomicBool::from_ptr(&raw mut source_stop).load(::core::sync::atomic::Ordering::SeqCst) {
        return r#true != 0;
    }
    if context.is_null() {
        return r#false != 0;
    }
    let mut fetch: *mut fetch_context_t = context as *mut fetch_context_t;
    let mut id: uint64_t = (*fetch).recording_id;
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if now != (*fetch).space_checked_at {
        let mut fs: statvfs = statvfs {
            f_bsize: 0,
            f_frsize: 0,
            f_blocks: 0,
            f_bfree: 0,
            f_bavail: 0,
            f_files: 0,
            f_ffree: 0,
            f_favail: 0,
            f_fsid: 0,
            f_flag: 0,
            f_namemax: 0,
            f_type: 0,
            __f_spare: [0; 5],
        };
        (*fetch).space_checked_at = now;
        if statvfs(
            &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
            &raw mut fs,
        ) != 0
            || (fs.f_bavail as uint64_t).wrapping_mul(fs.f_frsize as uint64_t)
                <= (*fetch).reserve_bytes
        {
            return r#true != 0;
        }
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return r#true != 0;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut cancel: bool = r#true != 0;
    if sqlite3_prepare_v2(
        db,
        b"SELECT deletion_pending FROM recordings WHERE id=?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        if sqlite3_step(statement) == SQLITE_ROW {
            cancel =
                sqlite3_column_int(statement, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return cancel;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_source_process_one() -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut source: recording_source_t = recording_source_t {
        target: [
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
        ],
        key: [0; 512],
        path: [0; 512],
        checksum: [0; 65],
        bytes: 0,
    };
    let mut id: uint64_t = 0 as uint64_t;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        b"SELECT j.recording_id,j.target_uuid,j.object_key,j.checksum,j.size_bytes,j.file_path FROM storage_retrieval_jobs j CROSS JOIN recordings r ON r.id=j.recording_id WHERE r.deletion_pending=0 AND j.state IN('queued','fetching') ORDER BY j.last_access_at LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW
    {
        id = sqlite3_column_int64(statement, 0 as ::core::ffi::c_int) as uint64_t;
        safe_strcpy(
            &raw mut source.target as *mut ::core::ffi::c_char,
            sqlite3_column_text(statement, 1 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut source.key as *mut ::core::ffi::c_char,
            sqlite3_column_text(statement, 2 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut source.checksum as *mut ::core::ffi::c_char,
            sqlite3_column_text(statement, 3 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
            0 as size_t,
        );
        source.bytes = sqlite3_column_int64(statement, 4 as ::core::ffi::c_int)
            as uint64_t;
        safe_strcpy(
            &raw mut source.path as *mut ::core::ffi::c_char,
            sqlite3_column_text(statement, 5 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    if id != 0
        && sqlite3_prepare_v2(
            db,
            b"UPDATE storage_retrieval_jobs SET state='fetching' WHERE recording_id=?;\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
    {
        sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    if id == 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut target: storage_target_t = storage_target_t {
        uuid: [0; 37],
        name: [0; 128],
        target_type: [0; 24],
        root_path: [0; 512],
        enabled: false,
        is_default: false,
        storage_class: [0; 16],
        reserve_bytes: 0,
        high_watermark_pct: 0.,
        low_watermark_pct: 0.,
        health_status: [0; 16],
        capacity_bytes: 0,
        available_bytes: 0,
        filesystem_device: 0,
        last_probe_at: 0,
        last_success_at: 0,
        last_error: [0; 256],
        revision: 0,
        created_at: 0,
        updated_at: 0,
        recording_count: 0,
        recording_bytes: 0,
        mount_required: false,
        mount_guard_path: [0; 512],
        migration_bandwidth_bps: 0,
        archival_window_start_minute: 0,
        archival_window_end_minute: 0,
        replica_count: 0,
        replica_bytes: 0,
        endpoint: [0; 512],
        region: [0; 64],
        bucket: [0; 128],
        credential_ref: [0; 128],
        archive_budget_bytes: 0,
    };
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
    let mut temporary: [::core::ffi::c_char; 512] = [0; 512];
    let mut n: ::core::ffi::c_int = snprintf(
        &raw mut temporary as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s.part\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut source.path as *mut ::core::ffi::c_char,
    );
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut available: uint64_t = 0 as uint64_t;
    let mut reserve: uint64_t = 0 as uint64_t;
    let mut headroom: bool = capture_space(&raw mut available, &raw mut reserve);
    let mut fetch: fetch_context_t = fetch_context_t {
        recording_id: id,
        reserve_bytes: reserve,
        space_checked_at: 0,
    };
    headroom = headroom as ::core::ffi::c_int != 0
        && available > reserve
        && source.bytes <= available.wrapping_sub(reserve);
    if !headroom {
        safe_strcpy(
            &raw mut error as *mut ::core::ffi::c_char,
            b"Archive retrieval waits for capture headroom\0".as_ptr()
                as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
    }
    if headroom as ::core::ffi::c_int != 0
        && n > 0 as ::core::ffi::c_int
        && n < ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int
        && db_storage_target_get(
            &raw mut source.target as *mut ::core::ffi::c_char,
            &raw mut target,
        )
        .0 == db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        unlink(&raw mut temporary as *mut ::core::ffi::c_char);
        let mut control: storage_transfer_control_t = storage_transfer_control_t {
            cancelled: Some(
                fetch_cancelled as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> bool,
            ),
            context: &raw mut fetch as *mut ::core::ffi::c_void,
            bandwidth_bps: target.migration_bandwidth_bps,
            job_uuid: ::core::ptr::null::<::core::ffi::c_char>(),
        };
        result = storage_s3_download(
            &raw mut target,
            &raw mut source.key as *mut ::core::ffi::c_char,
            &raw mut temporary as *mut ::core::ffi::c_char,
            source.bytes,
            &raw mut source.checksum as *mut ::core::ffi::c_char,
            &raw mut control,
            &raw mut error as *mut ::core::ffi::c_char,
        );
        if result == 0 as ::core::ffi::c_int
            && rename(
                &raw mut temporary as *mut ::core::ffi::c_char,
                &raw mut source.path as *mut ::core::ffi::c_char,
            ) != 0
        {
            result = -1 as ::core::ffi::c_int;
        }
        if result != 0 as ::core::ffi::c_int {
            unlink(&raw mut temporary as *mut ::core::ffi::c_char);
        }
    }
    if fetch_cancelled(&raw mut fetch as *mut ::core::ffi::c_void) {
        unlink(&raw mut source.path as *mut ::core::ffi::c_char);
        result = -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        b"UPDATE storage_retrieval_jobs SET state=?,last_error=?,next_attempt_at=strftime('%s','now')+30 WHERE recording_id=?3;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            if result == 0 as ::core::ffi::c_int {
                b"ready\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failed\0".as_ptr() as *const ::core::ffi::c_char
            },
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            if result == 0 as ::core::ffi::c_int {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            } else if *(&raw mut error as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                != 0
            {
                &raw mut error as *mut ::core::ffi::c_char
            } else {
                b"Archive retrieval failed\0".as_ptr() as *const ::core::ffi::c_char
            },
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(statement, 3 as ::core::ffi::c_int, id as sqlite3_int64);
        if sqlite3_step(statement) != SQLITE_DONE
            || sqlite3_changes(db) != 1 as ::core::ffi::c_int
        {
            unlink(&raw mut source.path as *mut ::core::ffi::c_char);
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn source_main(mut unused: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    while !::core::sync::atomic::AtomicBool::from_ptr(&raw mut source_stop).load(::core::sync::atomic::Ordering::SeqCst) {
        storage_source_trim(0 as uint64_t);
        if storage_source_process_one() > 0 as ::core::ffi::c_int {
            continue;
        }
        pthread_mutex_lock(&raw mut source_mutex);
        if !::core::sync::atomic::AtomicBool::from_ptr(&raw mut source_stop).load(::core::sync::atomic::Ordering::SeqCst) {
            let mut deadline: timespec = timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            clock_gettime(CLOCK_REALTIME, &raw mut deadline);
            deadline.tv_sec += 1;
            pthread_cond_timedwait(
                &raw mut source_condition,
                &raw mut source_mutex,
                &raw mut deadline,
            );
        }
        pthread_mutex_unlock(&raw mut source_mutex);
    }
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_source_worker_start() -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut source_mutex);
    if source_running {
        pthread_mutex_unlock(&raw mut source_mutex);
        return 0 as ::core::ffi::c_int;
    }
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut source_stop).store(false, ::core::sync::atomic::Ordering::SeqCst);
    let mut result: ::core::ffi::c_int = pthread_create(
        &raw mut source_thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            source_main
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    );
    source_running = result == 0 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut source_mutex);
    return if result == 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_source_worker_shutdown() {
    pthread_mutex_lock(&raw mut source_mutex);
    let mut running: bool = source_running;
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut source_stop).store(true, ::core::sync::atomic::Ordering::SeqCst);
    pthread_cond_signal(&raw mut source_condition);
    pthread_mutex_unlock(&raw mut source_mutex);
    if running {
        pthread_join(
            source_thread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
    }
    pthread_mutex_lock(&raw mut source_mutex);
    source_running = r#false != 0;
    pthread_mutex_unlock(&raw mut source_mutex);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
