use ::libc;
unsafe extern "C" {
    pub type sqlite3_stmt;
    pub type sqlite3;
    pub type __dirstream;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
    static mut g_config: config_t;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    unsafe fn build_recording_transcode_cache_path(
        storage_path: *const ::core::ffi::c_char,
        recording_id: uint64_t,
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn sqlite3_exec(
        _: *mut sqlite3,
        sql: *const ::core::ffi::c_char,
        callback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
        errmsg: *mut *mut ::core::ffi::c_char,
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
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn storage_s3_probe(
        target: *mut storage_target_t,
        write_test: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn storage_s3_abort_upload(
        target: *const storage_target_t,
        key: *const ::core::ffi::c_char,
        upload_id: *const ::core::ffi::c_char,
        error: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn storage_s3_delete(
        target: *const storage_target_t,
        key: *const ::core::ffi::c_char,
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
    unsafe fn strspn(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
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
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deletion_item_t {
    pub id: int64_t,
    pub deletion: [::core::ffi::c_char; 37],
    pub target: [::core::ffi::c_char; 37],
    pub key: [::core::ffi::c_char; 512],
    pub upload_id: [::core::ffi::c_char; 1024],
    pub path: [::core::ffi::c_char; 512],
    pub size: uint64_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct derivative_paths_t {
    pub count: ::core::ffi::c_int,
    pub paths: [[::core::ffi::c_char; 512]; 32],
}
pub type DIR = __dirstream;
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
pub struct detection_engine_config_t {
    pub enabled: bool,
    pub num_threads: ::core::ffi::c_int,
    pub delegate: [::core::ffi::c_char; 16],
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_protocol_t(pub ::core::ffi::c_uint);
impl stream_protocol_t {
    pub const STREAM_PROTOCOL_TCP: Self = Self(0);
    pub const STREAM_PROTOCOL_UDP: Self = Self(1);
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
pub const STORAGE_DELETION_PRUNE_DEFAULT_DAYS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const STORAGE_DELETION_PRUNE_DEFAULT_ROWS: ::core::ffi::c_int = 50000 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const F_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STORAGE_DELETION_SWEEP_INTERVAL_SEC: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const STORAGE_DELETION_PRUNE_BATCH: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
pub const STORAGE_DELETION_DUE_WINDOW: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
unsafe extern "C" fn execute_id(
    mut db: *mut sqlite3,
    mut sql: *const ::core::ffi::c_char,
    mut id: uint64_t,
    mut uuid: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        if !uuid.is_null() {
            sqlite3_bind_text(
                statement,
                2 as ::core::ffi::c_int,
                uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return if result == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn process_item(
    mut item: *const deletion_item_t,
    mut local_only: bool,
    mut removed: *mut uint64_t,
) -> ::core::ffi::c_int {
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
    let mut success: bool = r#false != 0;
    let mut file_removed: bool = r#false != 0;
    if (*item).target[0usize] != 0 {
        if db_storage_target_get(
            &raw const (*item).target as *const ::core::ffi::c_char,
            &raw mut target,
        )
        .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
        {
            safe_strcpy(
                &raw mut error as *mut ::core::ffi::c_char,
                b"Deletion target is unavailable\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        } else if strcmp(
            &raw mut target.target_type as *mut ::core::ffi::c_char,
            b"s3\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if local_only {
                return 0 as ::core::ffi::c_int;
            }
            if storage_s3_probe(&raw mut target, r#false != 0) == 0 as ::core::ffi::c_int {
                success = storage_s3_abort_upload(
                    &raw mut target,
                    &raw const (*item).key as *const ::core::ffi::c_char,
                    &raw const (*item).upload_id as *const ::core::ffi::c_char,
                    &raw mut error as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    && storage_s3_delete(
                        &raw mut target,
                        &raw const (*item).key as *const ::core::ffi::c_char,
                        &raw mut error as *mut ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int;
            } else {
                safe_strcpy(
                    &raw mut error as *mut ::core::ffi::c_char,
                    &raw mut target.last_error as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    0 as size_t,
                );
            }
        } else if !db_storage_target_mount_guard_active(&raw mut target) {
            safe_strcpy(
                &raw mut error as *mut ::core::ffi::c_char,
                b"Deletion target mount is absent\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        } else if db_storage_target_resolve_path(
            &raw const (*item).target as *const ::core::ffi::c_char,
            &raw const (*item).key as *const ::core::ffi::c_char,
            &raw mut path as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut error as *mut ::core::ffi::c_char,
                b"Deletion object identity is invalid\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        }
    } else {
        safe_strcpy(
            &raw mut path as *mut ::core::ffi::c_char,
            &raw const (*item).path as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    if path[0usize] != 0 {
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
        if lstat(&raw mut path as *mut ::core::ffi::c_char, &raw mut info)
            == 0 as ::core::ffi::c_int
            && !(info.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
        {
            safe_strcpy(
                &raw mut error as *mut ::core::ffi::c_char,
                b"Deletion object is not a regular file\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        } else if unlink(&raw mut path as *mut ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
            file_removed = r#true != 0;
            success = file_removed;
        } else if *__errno_location() == ENOENT {
            success = r#true != 0;
        } else {
            safe_strcpy(
                &raw mut error as *mut ::core::ffi::c_char,
                b"Cannot remove recording file\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        }
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char = b"UPDATE storage_deletion_objects SET state=?,last_error=?,attempt_count=attempt_count+1,next_attempt_at=CASE WHEN ? THEN 0 ELSE strftime('%s','now')+MIN(3600,30*(1<<MIN(attempt_count,7))) END,target_uuid=CASE WHEN ? THEN NULL ELSE target_uuid END,updated_at=strftime('%s','now') WHERE id=?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            if success as ::core::ffi::c_int != 0 {
                b"completed\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"retry_wait\0".as_ptr() as *const ::core::ffi::c_char
            },
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            if success as ::core::ffi::c_int != 0 {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            } else if *(&raw mut error as *mut ::core::ffi::c_char) as ::core::ffi::c_int != 0 {
                &raw mut error as *mut ::core::ffi::c_char
            } else {
                b"Deletion failed\0".as_ptr() as *const ::core::ffi::c_char
            },
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            3 as ::core::ffi::c_int,
            success as ::core::ffi::c_int,
        );
        sqlite3_bind_int(
            statement,
            4 as ::core::ffi::c_int,
            success as ::core::ffi::c_int,
        );
        sqlite3_bind_int64(
            statement,
            5 as ::core::ffi::c_int,
            (*item).id as sqlite3_int64,
        );
        sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    if file_removed as ::core::ffi::c_int != 0 && !removed.is_null() {
        *removed = (*removed).wrapping_add((*item).size);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn execute_uuid(
    mut db: *mut sqlite3,
    mut sql: *const ::core::ffi::c_char,
    mut uuid: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        if !uuid.is_null() {
            sqlite3_bind_text(
                statement,
                1 as ::core::ffi::c_int,
                uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return if result == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn finalize_deletions_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut scoped: [*const ::core::ffi::c_char; 3] = [
        b"UPDATE storage_deletions SET completed_at=strftime('%s','now') WHERE uuid=?1 AND completed_at IS NULL AND NOT EXISTS(SELECT 1 FROM storage_deletion_objects o WHERE o.deletion_uuid=?1 AND o.state<>'completed');\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"UPDATE detections SET recording_id=NULL WHERE recording_id IN (SELECT recording_id FROM storage_deletions WHERE uuid=?1 AND completed_at IS NOT NULL);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"DELETE FROM recordings WHERE deletion_pending=1 AND id IN (SELECT recording_id FROM storage_deletions WHERE uuid=?1 AND completed_at IS NOT NULL);\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut sweep: [*const ::core::ffi::c_char; 3] = [
        b"UPDATE storage_deletions SET completed_at=strftime('%s','now') WHERE completed_at IS NULL AND NOT EXISTS(SELECT 1 FROM storage_deletion_objects o WHERE o.deletion_uuid=storage_deletions.uuid AND o.state<>'completed');\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"UPDATE detections SET recording_id=NULL WHERE recording_id IN (SELECT r.id FROM recordings r JOIN storage_deletions d ON d.recording_id=r.id WHERE r.deletion_pending=1 AND d.completed_at IS NOT NULL);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"DELETE FROM recordings WHERE deletion_pending=1 AND EXISTS(SELECT 1 FROM storage_deletions d WHERE d.recording_id=recordings.id AND d.completed_at IS NOT NULL);\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut steps: *const *const ::core::ffi::c_char = if !uuid.is_null() {
        &raw const scoped as *const *const ::core::ffi::c_char
    } else {
        &raw const sweep as *const *const ::core::ffi::c_char
    };
    if sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    while result == 0 as ::core::ffi::c_int && i < 3 as size_t {
        result = execute_uuid(db, *steps.offset(i as isize), uuid);
        i = i.wrapping_add(1);
    }
    if result == 0 as ::core::ffi::c_int
        && sqlite3_exec(
            db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) == SQLITE_OK
    {
        return 0 as ::core::ffi::c_int;
    }
    sqlite3_exec(
        db,
        b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn process_due(
    mut deletion_uuid: *const ::core::ffi::c_char,
    mut local_only: bool,
    mut removed: *mut uint64_t,
) -> ::core::ffi::c_int {
    static mut scoped: *const ::core::ffi::c_char = b"SELECT o.id,o.deletion_uuid,COALESCE(o.target_uuid,''),o.object_key,o.file_path,o.size_bytes,o.upload_id FROM storage_deletion_objects o LEFT JOIN storage_targets t ON t.uuid=o.target_uuid WHERE o.deletion_uuid=?1 AND o.state<>'completed' AND o.next_attempt_at<=strftime('%s','now') AND (?2=0 OR COALESCE(t.target_type,'filesystem')='filesystem') AND NOT EXISTS(SELECT 1 FROM storage_deletions d JOIN storage_read_leases l ON l.recording_id=d.recording_id WHERE d.uuid=o.deletion_uuid AND l.expires_at>strftime('%s','now')) AND NOT EXISTS(SELECT 1 FROM storage_deletions d JOIN storage_retrieval_jobs j ON j.recording_id=d.recording_id WHERE d.uuid=o.deletion_uuid AND j.state='fetching') ORDER BY o.id LIMIT ?3;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    static mut sweep: *const ::core::ffi::c_char = b"SELECT o.id,o.deletion_uuid,COALESCE(o.target_uuid,''),o.object_key,o.file_path,o.size_bytes,o.upload_id FROM storage_deletion_objects o LEFT JOIN storage_targets t ON t.uuid=o.target_uuid WHERE o.id IN (SELECT id FROM storage_deletion_objects WHERE state<>'completed' AND next_attempt_at<=strftime('%s','now') ORDER BY next_attempt_at,id LIMIT ?1) AND (?2=0 OR COALESCE(t.target_type,'filesystem')='filesystem') AND NOT EXISTS(SELECT 1 FROM storage_deletions d JOIN storage_read_leases l ON l.recording_id=d.recording_id WHERE d.uuid=o.deletion_uuid AND l.expires_at>strftime('%s','now')) AND NOT EXISTS(SELECT 1 FROM storage_deletions d JOIN storage_retrieval_jobs j ON j.recording_id=d.recording_id WHERE d.uuid=o.deletion_uuid AND j.state='fetching') ORDER BY o.id LIMIT ?3;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    static mut last_sweep: time_t = 0;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut items: [deletion_item_t; 32] = [deletion_item_t {
        id: 0,
        deletion: [0; 37],
        target: [0; 37],
        key: [0; 512],
        upload_id: [0; 1024],
        path: [0; 512],
        size: 0,
    }; 32];
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        if !deletion_uuid.is_null() {
            scoped
        } else {
            sweep
        },
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        if !deletion_uuid.is_null() {
            sqlite3_bind_text(
                statement,
                1 as ::core::ffi::c_int,
                deletion_uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_int(
                statement,
                1 as ::core::ffi::c_int,
                STORAGE_DELETION_DUE_WINDOW,
            );
        }
        sqlite3_bind_int(
            statement,
            2 as ::core::ffi::c_int,
            local_only as ::core::ffi::c_int,
        );
        sqlite3_bind_int(
            statement,
            3 as ::core::ffi::c_int,
            if local_only as ::core::ffi::c_int != 0 {
                32 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            },
        );
        while count < 32 as ::core::ffi::c_int && sqlite3_step(statement) == SQLITE_ROW {
            let c2rust_fresh0 = count;
            count += 1;
            let mut item: *mut deletion_item_t =
                (&raw mut items as *mut deletion_item_t).offset(c2rust_fresh0 as isize);
            memset(
                item as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<deletion_item_t>(),
            );
            (*item).id = sqlite3_column_int64(statement, 0 as ::core::ffi::c_int) as int64_t;
            safe_strcpy(
                &raw mut (*item).deletion as *mut ::core::ffi::c_char,
                sqlite3_column_text(statement, 1 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*item).target as *mut ::core::ffi::c_char,
                sqlite3_column_text(statement, 2 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*item).key as *mut ::core::ffi::c_char,
                sqlite3_column_text(statement, 3 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*item).path as *mut ::core::ffi::c_char,
                sqlite3_column_text(statement, 4 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
            (*item).size = sqlite3_column_int64(statement, 5 as ::core::ffi::c_int) as uint64_t;
            safe_strcpy(
                &raw mut (*item).upload_id as *mut ::core::ffi::c_char,
                sqlite3_column_text(statement, 6 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                0 as size_t,
            );
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        process_item(
            (&raw mut items as *mut deletion_item_t).offset(i as isize),
            local_only,
            removed,
        );
        i += 1;
    }
    pthread_mutex_lock(mutex);
    if !deletion_uuid.is_null() {
        finalize_deletions_locked(db, deletion_uuid);
    } else {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < count {
            let mut seen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < i_0 && seen == 0 {
                seen = (strcmp(
                    &raw mut (*(&raw mut items as *mut deletion_item_t).offset(j as isize)).deletion
                        as *mut ::core::ffi::c_char,
                    &raw mut (*(&raw mut items as *mut deletion_item_t).offset(i_0 as isize))
                        .deletion as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                j += 1;
            }
            if seen == 0 {
                finalize_deletions_locked(
                    db,
                    &raw mut (*(&raw mut items as *mut deletion_item_t).offset(i_0 as isize))
                        .deletion as *mut ::core::ffi::c_char,
                );
            }
            i_0 += 1;
        }
        let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
        if now - last_sweep >= STORAGE_DELETION_SWEEP_INTERVAL_SEC as time_t {
            last_sweep = now;
            finalize_deletions_locked(db, ::core::ptr::null::<::core::ffi::c_char>());
        }
    }
    pthread_mutex_unlock(mutex);
    return count;
}
unsafe extern "C" fn recovery_read_only() -> bool {
    let mut value: *const ::core::ffi::c_char =
        getenv(b"LIGHTNVR_STORAGE_READ_ONLY\0".as_ptr() as *const ::core::ffi::c_char);
    return !value.is_null() && strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char) == 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_deletion_process_one() -> ::core::ffi::c_int {
    return if recovery_read_only() as ::core::ffi::c_int != 0 {
        0 as ::core::ffi::c_int
    } else {
        process_due(
            ::core::ptr::null::<::core::ffi::c_char>(),
            r#false != 0,
            ::core::ptr::null_mut::<uint64_t>(),
        )
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_deletion_finalize_all() -> ::core::ffi::c_int {
    if recovery_read_only() {
        return 0 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut result: ::core::ffi::c_int =
        finalize_deletions_locked(db, ::core::ptr::null::<::core::ffi::c_char>());
    pthread_mutex_unlock(mutex);
    return result;
}
unsafe extern "C" fn prune_batch_locked(
    mut db: *mut sqlite3,
    mut cutoff: int64_t,
    mut limit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    static mut steps: [*const ::core::ffi::c_char; 2] = [
        b"DELETE FROM storage_deletion_objects WHERE deletion_uuid IN (SELECT uuid FROM storage_deletions WHERE completed_at IS NOT NULL AND completed_at<?1 ORDER BY completed_at,rowid LIMIT ?2);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"DELETE FROM storage_deletions WHERE rowid IN (SELECT rowid FROM storage_deletions WHERE completed_at IS NOT NULL AND completed_at<?1 ORDER BY completed_at,rowid LIMIT ?2);\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ];
    if sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut pruned: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    while pruned >= 0 as ::core::ffi::c_int && i < 2 as size_t {
        let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
            db,
            steps[i],
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, cutoff as sqlite3_int64);
            sqlite3_bind_int(statement, 2 as ::core::ffi::c_int, limit);
            result = sqlite3_step(statement);
        }
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
        pruned = if result == SQLITE_DONE {
            sqlite3_changes(db)
        } else {
            -1 as ::core::ffi::c_int
        };
        i = i.wrapping_add(1);
    }
    if pruned >= 0 as ::core::ffi::c_int
        && sqlite3_exec(
            db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) == SQLITE_OK
    {
        return pruned;
    }
    sqlite3_exec(
        db,
        b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_deletion_prune_completed(
    mut older_than_days: ::core::ffi::c_int,
    mut max_rows: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if recovery_read_only() {
        return 0 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if older_than_days <= 0 as ::core::ffi::c_int {
        older_than_days = STORAGE_DELETION_PRUNE_DEFAULT_DAYS;
    }
    if max_rows <= 0 as ::core::ffi::c_int {
        max_rows = STORAGE_DELETION_PRUNE_DEFAULT_ROWS;
    }
    let mut cutoff: int64_t = time(::core::ptr::null_mut::<time_t>()) as int64_t
        - older_than_days as int64_t * 86400 as int64_t;
    let mut pruned: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while pruned < max_rows {
        let mut limit: ::core::ffi::c_int = max_rows - pruned;
        if limit > STORAGE_DELETION_PRUNE_BATCH {
            limit = STORAGE_DELETION_PRUNE_BATCH;
        }
        pthread_mutex_lock(mutex);
        let mut batch: ::core::ffi::c_int = prune_batch_locked(db, cutoff, limit);
        pthread_mutex_unlock(mutex);
        if batch < 0 as ::core::ffi::c_int {
            return if pruned != 0 {
                pruned
            } else {
                -1 as ::core::ffi::c_int
            };
        }
        pruned += batch;
        if batch < limit {
            break;
        }
    }
    return pruned;
}
unsafe extern "C" fn inventory_path(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"INSERT INTO storage_deletion_objects(deletion_uuid,file_path) VALUES(?,?);\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            path,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
pub const DERIVATIVE_PATHS_MAX: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
unsafe extern "C" fn derivative_add(
    mut out: *mut derivative_paths_t,
    mut path: *const ::core::ffi::c_char,
) {
    if (*out).count >= DERIVATIVE_PATHS_MAX {
        return;
    }
    let c2rust_fresh1 = (*out).count;
    (*out).count += 1;
    safe_strcpy(
        &raw mut *(&raw mut (*out).paths as *mut [::core::ffi::c_char; 512])
            .offset(c2rust_fresh1 as isize) as *mut ::core::ffi::c_char,
        path,
        MAX_PATH_LENGTH as size_t,
        0 as size_t,
    );
}
unsafe extern "C" fn collect_derivatives(
    mut id: uint64_t,
    mut out: *mut derivative_paths_t,
) -> ::core::ffi::c_int {
    let mut path: [::core::ffi::c_char; 512] = [0; 512];
    let mut directory: [::core::ffi::c_char; 512] = [0; 512];
    (*out).count = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int {
        let mut n: ::core::ffi::c_int = snprintf(
            &raw mut path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/thumbnails/%llu_%d.jpg\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
            id as ::core::ffi::c_ulonglong,
            i,
        );
        if n < 0 as ::core::ffi::c_int
            || n >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if access(&raw mut path as *mut ::core::ffi::c_char, F_OK) == 0 as ::core::ffi::c_int {
            derivative_add(out, &raw mut path as *mut ::core::ffi::c_char);
        }
        i += 1;
    }
    if build_recording_transcode_cache_path(
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
        id,
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) == 0
        && access(&raw mut path as *mut ::core::ffi::c_char, F_OK) == 0 as ::core::ffi::c_int
    {
        derivative_add(out, &raw mut path as *mut ::core::ffi::c_char);
    }
    let mut n_0: ::core::ffi::c_int = snprintf(
        &raw mut directory as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/thumbnails/investigation/%llu\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
    );
    if n_0 < 0 as ::core::ffi::c_int
        || n_0 >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut dir: *mut DIR = opendir(&raw mut directory as *mut ::core::ffi::c_char);
    if dir.is_null() {
        return if *__errno_location() == ENOENT {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        let mut digits: size_t = strspn(
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
            b"0123456789\0".as_ptr() as *const ::core::ffi::c_char,
        ) as size_t;
        if digits == 0
            || strcmp(
                (&raw mut (*entry).d_name as *mut ::core::ffi::c_char).offset(digits as isize),
                b".jpg\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0
        {
            continue;
        }
        n_0 = snprintf(
            &raw mut path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut directory as *mut ::core::ffi::c_char,
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
        );
        if n_0 < 0 as ::core::ffi::c_int
            || n_0 >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int
        {
            result = -1 as ::core::ffi::c_int;
            break;
        } else {
            derivative_add(out, &raw mut path as *mut ::core::ffi::c_char);
        }
    }
    closedir(dir);
    return result;
}
unsafe extern "C" fn inventory_derivatives(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut paths: *const derivative_paths_t,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*paths).count {
        if inventory_path(
            db,
            uuid,
            &raw const *(&raw const (*paths).paths as *const [::core::ffi::c_char; 512])
                .offset(i as isize) as *const ::core::ffi::c_char,
        ) != 0
        {
            return -1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn recording_delete(
    mut id: uint64_t,
    mut reason: *const ::core::ffi::c_char,
    mut removed: *mut uint64_t,
    mut require_policy_expiry: bool,
    mut age_cutoff: int64_t,
    mut stale_incomplete: bool,
) -> ::core::ffi::c_int {
    if recovery_read_only() {
        return -1 as ::core::ffi::c_int;
    }
    if !removed.is_null() {
        *removed = 0 as uint64_t;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() || id == 0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if lightnvr_uuid_generate_v4(&raw mut uuid as *mut ::core::ffi::c_char) != 0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut derivatives: derivative_paths_t = derivative_paths_t {
        count: 0,
        paths: [[0; 512]; 32],
    };
    if collect_derivatives(id, &raw mut derivatives) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    if sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT protected,EXISTS(SELECT 1 FROM storage_migration_jobs WHERE recording_id=?1 AND state IN('copying','verifying','committing')),deletion_pending,is_complete,EXISTS(SELECT 1 FROM storage_recording_copies WHERE recording_id=?1) FROM recordings WHERE id=?1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        result = sqlite3_step(statement);
    }
    let mut blocked: bool = result == SQLITE_ROW
        && (sqlite3_column_int(statement, 0 as ::core::ffi::c_int) != 0
            || sqlite3_column_int(statement, 1 as ::core::ffi::c_int) != 0
            || if stale_incomplete as ::core::ffi::c_int != 0 {
                (sqlite3_column_int(statement, 3 as ::core::ffi::c_int) != 0
                    || sqlite3_column_int(statement, 4 as ::core::ffi::c_int) != 0)
                    as ::core::ffi::c_int
            } else {
                (sqlite3_column_int(statement, 3 as ::core::ffi::c_int) == 0) as ::core::ffi::c_int
            } != 0);
    let mut pending: bool =
        result == SQLITE_ROW && sqlite3_column_int(statement, 2 as ::core::ffi::c_int) != 0;
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if blocked as ::core::ffi::c_int != 0
        || pending as ::core::ffi::c_int != 0
        || result != SQLITE_ROW
    {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return if blocked as ::core::ffi::c_int != 0 {
            -2 as ::core::ffi::c_int
        } else if pending as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else if result == SQLITE_DONE {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    if age_cutoff != 0 {
        statement = ::core::ptr::null_mut::<sqlite3_stmt>();
        result = sqlite3_prepare_v2(
            db,
            b"SELECT 1 FROM recordings r LEFT JOIN storage_recording_policies p ON p.recording_id=r.id WHERE r.end_time<?1 AND r.is_complete=1 AND COALESCE(r.retention_override_days,-1)<>0 AND (COALESCE(p.minimum_retention_days,0)=0 OR r.start_time<strftime('%s','now')-p.minimum_retention_days*86400) AND ((r.retention_override_days>0 AND r.start_time<strftime('%s','now')-r.retention_override_days*86400) OR (COALESCE(r.retention_override_days,-1)<0 AND (p.recording_id IS NULL OR (p.maximum_retention_days>0 AND r.start_time<strftime('%s','now')-p.maximum_retention_days*86400)))) AND r.id=?2;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_int64(
                statement,
                1 as ::core::ffi::c_int,
                age_cutoff as sqlite3_int64,
            );
            sqlite3_bind_int64(statement, 2 as ::core::ffi::c_int, id as sqlite3_int64);
            result = sqlite3_step(statement);
        }
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
        if result != SQLITE_ROW {
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            pthread_mutex_unlock(mutex);
            return if result == SQLITE_DONE {
                -2 as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        }
    }
    if require_policy_expiry {
        statement = ::core::ptr::null_mut::<sqlite3_stmt>();
        result = sqlite3_prepare_v2(
            db,
            b"SELECT 1 FROM recordings r JOIN storage_recording_policies p ON p.recording_id=r.id WHERE r.id=? AND r.is_complete=1 AND COALESCE(r.retention_override_days,-1)<>0 AND (p.minimum_retention_days=0 OR r.start_time<strftime('%s','now')-p.minimum_retention_days*86400) AND ((r.retention_override_days>0 AND r.start_time<strftime('%s','now')-r.retention_override_days*86400) OR (COALESCE(r.retention_override_days,-1)<0 AND p.maximum_retention_days>0 AND r.start_time<strftime('%s','now')-p.maximum_retention_days*86400));\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
            result = sqlite3_step(statement);
        }
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
        if result != SQLITE_ROW {
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            pthread_mutex_unlock(mutex);
            return if result == SQLITE_DONE {
                -2 as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        }
    }
    result = SQLITE_OK;
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    if result == SQLITE_OK {
        result = sqlite3_prepare_v2(
            db,
            b"INSERT INTO storage_deletions(uuid,recording_id,camera_uuid,stream_name,reason) SELECT ?2,id,COALESCE(camera_uuid,''),stream_name,?3 FROM recordings WHERE id=?1;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
    }
    if result == SQLITE_OK {
        sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            &raw mut uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            if !reason.is_null() {
                reason
            } else {
                b"recording deletion\0".as_ptr() as *const ::core::ffi::c_char
            },
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    let mut inventory: [*const ::core::ffi::c_char; 7] = [
        b"INSERT INTO storage_deletion_objects(deletion_uuid,target_uuid,object_key,file_path,size_bytes) SELECT ?2,storage_target_uuid,COALESCE(object_key,''),file_path,MAX(size_bytes,0) FROM recordings WHERE id=?1 AND (file_path<>'' OR storage_target_uuid IS NOT NULL);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"INSERT INTO storage_deletion_objects(deletion_uuid,target_uuid,object_key,size_bytes) SELECT ?2,target_uuid,object_key,size_bytes FROM storage_recording_copies WHERE recording_id=?1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"INSERT INTO storage_deletion_objects(deletion_uuid,target_uuid,object_key,size_bytes) SELECT ?2,source_target_uuid,source_object_key,bytes_total FROM storage_migration_jobs WHERE recording_id=?1 AND state='cleanup_pending';\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"INSERT INTO storage_deletion_objects(deletion_uuid,target_uuid,object_key,size_bytes,upload_id) SELECT ?2,destination_target_uuid,destination_object_key,bytes_total,upload_id FROM storage_migration_jobs WHERE recording_id=?1 AND state NOT IN('completed','cleanup_pending');\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"INSERT INTO storage_deletion_objects(deletion_uuid,file_path,size_bytes) SELECT ?2,file_path,size_bytes FROM storage_retrieval_jobs WHERE recording_id=?1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"UPDATE storage_migration_jobs SET cancel_requested=1,state='cancelled' WHERE recording_id=?1 AND state NOT IN('completed','cleanup_pending');\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"UPDATE recordings SET deletion_pending=1 WHERE id=?1;\0".as_ptr()
            as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0 as size_t;
    while result == SQLITE_DONE
        && i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 7]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        if execute_id(
            db,
            inventory[i],
            id,
            if i < 5 as size_t {
                &raw mut uuid as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
        ) != 0
        {
            result = SQLITE_ERROR;
        }
        i = i.wrapping_add(1);
    }
    if result == SQLITE_DONE
        && inventory_derivatives(
            db,
            &raw mut uuid as *mut ::core::ffi::c_char,
            &raw mut derivatives,
        ) != 0
    {
        result = SQLITE_ERROR;
    }
    let mut committed: bool = result == SQLITE_DONE
        && sqlite3_exec(
            db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) == SQLITE_OK;
    if !committed {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    }
    pthread_mutex_unlock(mutex);
    if !committed {
        return -1 as ::core::ffi::c_int;
    }
    process_due(
        &raw mut uuid as *mut ::core::ffi::c_char,
        r#true != 0,
        removed,
    );
    pthread_mutex_lock(mutex);
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut still_pending: bool = r#true != 0;
    if sqlite3_prepare_v2(
        db,
        b"SELECT 1 FROM recordings WHERE id=?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, id as sqlite3_int64);
        still_pending = sqlite3_step(statement) == SQLITE_ROW;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return if still_pending as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_recording_delete(
    mut id: uint64_t,
    mut reason: *const ::core::ffi::c_char,
    mut removed: *mut uint64_t,
) -> ::core::ffi::c_int {
    return recording_delete(
        id,
        reason,
        removed,
        r#false != 0,
        0 as int64_t,
        r#false != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_recording_expire_policy(mut id: uint64_t) -> ::core::ffi::c_int {
    return recording_delete(
        id,
        b"policy expiry\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<uint64_t>(),
        r#true != 0,
        0 as int64_t,
        r#false != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_recording_expire_age(
    mut id: uint64_t,
    mut cutoff: int64_t,
) -> ::core::ffi::c_int {
    if cutoff == 0 {
        return -2 as ::core::ffi::c_int;
    }
    return recording_delete(
        id,
        b"legacy age retention\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<uint64_t>(),
        r#false != 0,
        cutoff,
        r#false != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_recording_delete_stale_incomplete(
    mut id: uint64_t,
) -> ::core::ffi::c_int {
    return recording_delete(
        id,
        b"stale incomplete recording\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<uint64_t>(),
        r#false != 0,
        0 as int64_t,
        r#true != 0,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
