use ::c2rust_bitfields;
use ::libc;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sqlite3;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
    pub type __dirstream;
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
    unsafe fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    unsafe fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    unsafe fn sqlite3_db_mutex(_: *mut sqlite3) -> *mut sqlite3_mutex;
    static mut g_config: config_t;
    unsafe fn observation_asset_state_name(
        state: observation_asset_state_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn db_observation_asset_transition(
        asset_uuid: *const ::core::ffi::c_char,
        from: observation_asset_state_t,
        to: observation_asset_state_t,
        reason: *const ::core::ffi::c_char,
        media: *const observation_asset_media_t,
        result: *mut observation_transition_result_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_observation_meta_get(out: *mut observation_meta_t) -> ::core::ffi::c_int;
    unsafe fn db_observation_meta_regenerate_epoch() -> ::core::ffi::c_int;
    unsafe fn db_observations_prune(
        before_ms: int64_t,
        batch: ::core::ffi::c_int,
        result: *mut observation_prune_result_t,
    ) -> ::core::ffi::c_int;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn fdopendir(__fd: ::core::ffi::c_int) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
    unsafe fn dirfd(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
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
    unsafe fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn renameat(
        __oldfd: ::core::ffi::c_int,
        __old: *const ::core::ffi::c_char,
        __newfd: ::core::ffi::c_int,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn fstatat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __buf: *mut stat,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    unsafe fn unlinkat(
        __fd: ::core::ffi::c_int,
        __name: *const ::core::ffi::c_char,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn mkdir_recursive(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn db_open_readonly_connection(connection: *mut *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn db_close_readonly_connection(connection: *mut sqlite3);
    unsafe fn db_get_system_setting(
        key: *const ::core::ffi::c_char,
        out: *mut ::core::ffi::c_char,
        out_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn observation_evidence_set_budget_exhausted(
        camera_uuid: *const ::core::ffi::c_char,
        exhausted: bool,
    );
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
    unsafe fn strtok_r(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
        __save_ptr: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
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
pub type __clockid_t = ::core::ffi::c_int;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ssize_t = isize;
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
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
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
pub struct observation_asset_media_t {
    pub path: *const ::core::ffi::c_char,
    pub bytes: int64_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub expires_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_transition_result_t {
    pub observations_bumped: ::core::ffi::c_int,
    pub last_seq: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_meta_t {
    pub epoch_uuid: [::core::ffi::c_char; 37],
    pub oldest_seq: int64_t,
    pub head_seq: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_prune_result_t {
    pub changes_deleted: ::core::ffi::c_int,
    pub observations_deleted: ::core::ffi::c_int,
    pub assets_deleted: ::core::ffi::c_int,
    pub oldest_seq: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_reconcile_result_t {
    pub tmp_files_deleted: ::core::ffi::c_int,
    pub stale_files_deleted: ::core::ffi::c_int,
    pub pending_interrupted: ::core::ffi::c_int,
    pub ready_missing: ::core::ffi::c_int,
    pub epoch_regenerated: bool,
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
pub type atomic_int = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct asset_row_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub path: [::core::ffi::c_char; 512],
    pub bytes: int64_t,
    pub ref_ms: int64_t,
    pub state: [::core::ffi::c_char; 16],
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
pub struct sweep_t {
    pub db: *mut sqlite3,
    pub lookup: *mut sqlite3_stmt,
    pub root: *const ::core::ffi::c_char,
    pub result: *mut observation_reconcile_result_t,
    pub errors: ::core::ffi::c_int,
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
pub struct observation_retention_result_t {
    pub assets_expired: ::core::ffi::c_int,
    pub unreferenced_deleted: ::core::ffi::c_int,
    pub files_deleted: ::core::ffi::c_int,
    pub changes_pruned: ::core::ffi::c_int,
    pub observations_pruned: ::core::ffi::c_int,
    pub cameras_budget_exhausted: ::core::ffi::c_int,
    pub more_work: bool,
    pub oldest_seq: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_retention_camera_status_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub retained_bytes: int64_t,
    pub max_bytes: int64_t,
    pub max_age_days: ::core::ffi::c_int,
    pub budget_exhausted: bool,
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
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const OBS_UUID_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const OBS_RETENTION_DIR: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"observations\0") };
pub const OBS_CURSOR_HEAD_FILE: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b".cursor_head.json\0")
};
pub const OBS_RETENTION_INTERVAL_SEC: ::core::ffi::c_int = 300 as ::core::ffi::c_int;
pub const OBS_RETENTION_MAX_EVICTIONS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const OBS_RETENTION_MAX_PRUNED_CHANGES: ::core::ffi::c_int = 5000 as ::core::ffi::c_int;
pub const OBS_RETENTION_PRUNE_BATCH: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
pub const OBS_RETENTION_DEFAULT_HISTORY_DAYS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const OBS_RETENTION_DEFAULT_MAX_AGE_DAYS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const OBS_RETENTION_DEFAULT_MAX_BYTES: ::core::ffi::c_longlong =
    2147483648 as ::core::ffi::c_longlong;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __O_DIRECTORY: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const O_DIRECTORY: ::core::ffi::c_int = __O_DIRECTORY;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const AT_SYMLINK_NOFOLLOW: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const AT_REMOVEDIR: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const DAY_MS: ::core::ffi::c_longlong = 86400000 as ::core::ffi::c_longlong;
pub const OBS_PATH_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const OBS_WALK_MAX_DEPTH: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const OBS_PAGE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
static mut g_run_mutex: pthread_mutex_t = pthread_mutex_t {
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
static mut g_startup_done: atomic_int = 0 as ::core::ffi::c_int;
static mut g_status_mutex: pthread_mutex_t = pthread_mutex_t {
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
static mut g_status: *mut observation_retention_camera_status_t =
    ::core::ptr::null_mut::<observation_retention_camera_status_t>();
static mut g_status_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut g_exhausted_count: atomic_int = 0 as ::core::ffi::c_int;
static mut g_last_tick: time_t = 0 as time_t;
static mut g_more_work: bool = r#false != 0;
unsafe extern "C" fn root_dir(
    mut storage_path: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_int {
    if storage_path.is_null() || *storage_path == 0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut len: size_t = strlen(storage_path);
    while len > 1 as size_t
        && *storage_path.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            == '/' as ::core::ffi::c_int
    {
        len = len.wrapping_sub(1);
    }
    let mut n: ::core::ffi::c_int = snprintf(
        out,
        size,
        b"%.*s/observations\0".as_ptr() as *const ::core::ffi::c_char,
        len as ::core::ffi::c_int,
        storage_path,
    );
    return if n > 0 as ::core::ffi::c_int && (n as size_t) < size {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn has_suffix(
    mut value: *const ::core::ffi::c_char,
    mut suffix: *const ::core::ffi::c_char,
) -> bool {
    let mut a: size_t = strlen(value);
    let mut b: size_t = strlen(suffix);
    return a >= b
        && strcmp(value.offset(a as isize).offset(-(b as isize)), suffix)
            == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_relative(mut rel: *const ::core::ffi::c_char) -> bool {
    if *rel == 0 || *rel as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut segment: *const ::core::ffi::c_char = rel;
    loop {
        let mut end: *const ::core::ffi::c_char = strchr(segment, '/' as ::core::ffi::c_int);
        let mut len: size_t = if !end.is_null() {
            end.offset_from(segment) as size_t
        } else {
            strlen(segment)
        };
        if len == 0 as size_t
            || len == 1 as size_t
                && *segment.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            || len == 2 as size_t
                && *segment.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && *segment.offset(1isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        if end.is_null() {
            return r#true != 0;
        }
        segment = end.offset(1 as ::core::ffi::c_int as isize);
    }
}
unsafe extern "C" fn resolve_asset_path(
    mut root: *const ::core::ffi::c_char,
    mut stored: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_int {
    if stored.is_null()
        || *stored == 0
        || !has_suffix(stored, b".jpg\0".as_ptr() as *const ::core::ffi::c_char)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut rel: *const ::core::ffi::c_char = stored;
    let mut root_len: size_t = strlen(root);
    if *stored.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
        if strncmp(stored, root, root_len) != 0 as ::core::ffi::c_int
            || *stored.offset(root_len as isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        rel = stored
            .offset(root_len as isize)
            .offset(1 as ::core::ffi::c_int as isize);
    } else if strncmp(
        stored,
        b"observations/\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 13]>(),
    ) == 0 as ::core::ffi::c_int
    {
        rel = stored.offset(::core::mem::size_of::<[::core::ffi::c_char; 13]>() as isize);
    }
    if !safe_relative(rel) {
        return -1 as ::core::ffi::c_int;
    }
    let mut n: ::core::ffi::c_int = snprintf(
        out,
        size,
        b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
        root,
        rel,
    );
    return if n > 0 as ::core::ffi::c_int && (n as size_t) < size {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn remove_file_under_root(
    mut root: *const ::core::ffi::c_char,
    mut rel: *const ::core::ffi::c_char,
    mut deleted: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut work: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut n: ::core::ffi::c_int = snprintf(
        &raw mut work as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        rel,
    );
    if n <= 0 as ::core::ffi::c_int
        || n as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut dir_fd: ::core::ffi::c_int = open(root, O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    if dir_fd < 0 as ::core::ffi::c_int {
        if *__errno_location() == ENOENT || *__errno_location() == ENOTDIR {
            return 0 as ::core::ffi::c_int;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open observation root %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            root,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut rc: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut part: *mut ::core::ffi::c_char = &raw mut work as *mut ::core::ffi::c_char;
    '_done: {
        loop {
            let mut slash: *mut ::core::ffi::c_char = strchr(part, '/' as ::core::ffi::c_int);
            if slash.is_null() {
                break;
            }
            *slash = '\0' as ::core::ffi::c_char;
            let mut next_fd: ::core::ffi::c_int = openat(
                dir_fd,
                part,
                O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC,
            );
            if next_fd < 0 as ::core::ffi::c_int {
                if *__errno_location() == ENOENT || *__errno_location() == ENOTDIR {
                    rc = 0 as ::core::ffi::c_int;
                } else {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to traverse observation evidence path %s/%s: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        root,
                        rel,
                        strerror(*__errno_location()),
                    );
                }
                break '_done;
            } else {
                close(dir_fd);
                dir_fd = next_fd;
                part = slash.offset(1 as ::core::ffi::c_int as isize);
            }
        }
        if unlinkat(dir_fd, part, 0 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int {
            if !deleted.is_null() {
                *deleted += 1;
            }
            rc = 0 as ::core::ffi::c_int;
        } else if *__errno_location() == ENOENT || *__errno_location() == ENOTDIR {
            rc = 0 as ::core::ffi::c_int;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to delete observation evidence file %s/%s: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                root,
                rel,
                strerror(*__errno_location()),
            );
        }
    }
    close(dir_fd);
    return rc;
}
unsafe extern "C" fn remove_asset_file(
    mut root: *const ::core::ffi::c_char,
    mut stored: *const ::core::ffi::c_char,
    mut deleted: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if stored.is_null() || *stored == 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut path: [::core::ffi::c_char; 1024] = [0; 1024];
    if resolve_asset_path(
        root,
        stored,
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Observation asset path is outside the evidence tree; not deleting\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    return remove_file_under_root(
        root,
        (&raw mut path as *mut ::core::ffi::c_char)
            .offset(strlen(root) as isize)
            .offset(1 as ::core::ffi::c_int as isize),
        deleted,
    );
}
unsafe extern "C" fn wall_ms() -> int64_t {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_REALTIME, &raw mut ts);
    return ts.tv_sec as int64_t * 1000 as int64_t + ts.tv_nsec as int64_t / 1000000 as int64_t;
}
unsafe extern "C" fn copy_text(
    mut stmt: *mut sqlite3_stmt,
    mut column: ::core::ffi::c_int,
    mut out: *mut ::core::ffi::c_char,
    mut size: size_t,
) {
    let mut text: *const ::core::ffi::c_uchar = sqlite3_column_text(stmt, column);
    safe_strcpy(
        out,
        if !text.is_null() {
            text as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        size,
        0 as size_t,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_cursor_head_write(
    mut storage_path: *const ::core::ffi::c_char,
    mut epoch_uuid: *const ::core::ffi::c_char,
    mut head_seq: int64_t,
) -> ::core::ffi::c_int {
    let mut root: [::core::ffi::c_char; 1024] = [0; 1024];
    if !lightnvr_uuid_is_valid(epoch_uuid)
        || head_seq < 0 as int64_t
        || root_dir(
            storage_path,
            &raw mut root as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if strlen(&raw mut root as *mut ::core::ffi::c_char)
        .wrapping_add(::core::mem::size_of::<[::core::ffi::c_char; 19]>())
        > OBS_PATH_SIZE as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    if mkdir_recursive(&raw mut root as *mut ::core::ffi::c_char) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut body: [::core::ffi::c_char; 160] = [0; 160];
    let mut len: ::core::ffi::c_int = snprintf(
        &raw mut body as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
        b"{\"epoch\":\"%s\",\"head_seq\":%lld}\n\0".as_ptr() as *const ::core::ffi::c_char,
        epoch_uuid,
        head_seq as ::core::ffi::c_longlong,
    );
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    let mut tmp: [::core::ffi::c_char; 60] = [0; 60];
    if lightnvr_uuid_generate_v4(&raw mut uuid as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    snprintf(
        &raw mut tmp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 60]>(),
        b"%s.%s.tmp\0".as_ptr() as *const ::core::ffi::c_char,
        OBS_CURSOR_HEAD_FILE.as_ptr(),
        &raw mut uuid as *mut ::core::ffi::c_char,
    );
    let mut dfd: ::core::ffi::c_int = open(
        &raw mut root as *mut ::core::ffi::c_char,
        O_RDONLY | O_DIRECTORY | O_CLOEXEC,
    );
    if dfd < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut fd: ::core::ffi::c_int = openat(
        dfd,
        &raw mut tmp as *mut ::core::ffi::c_char,
        O_WRONLY | O_CREAT | O_EXCL | O_NOFOLLOW | O_CLOEXEC,
        0o640 as ::core::ffi::c_int,
    );
    if fd < 0 as ::core::ffi::c_int {
        close(dfd);
        return -1 as ::core::ffi::c_int;
    }
    let mut rc: ::core::ffi::c_int = if write(
        fd,
        &raw mut body as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as size_t,
    ) == len as ssize_t
        && fsync(fd) == 0 as ::core::ffi::c_int
    {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
    if close(fd) != 0 as ::core::ffi::c_int {
        rc = -1 as ::core::ffi::c_int;
    }
    if rc == 0 as ::core::ffi::c_int
        && renameat(
            dfd,
            &raw mut tmp as *mut ::core::ffi::c_char,
            dfd,
            OBS_CURSOR_HEAD_FILE.as_ptr(),
        ) != 0 as ::core::ffi::c_int
    {
        rc = -1 as ::core::ffi::c_int;
    }
    if rc != 0 as ::core::ffi::c_int {
        unlinkat(
            dfd,
            &raw mut tmp as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to write observation cursor head sidecar\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        close(dfd);
        return -1 as ::core::ffi::c_int;
    }
    rc = fsync(dfd);
    if close(dfd) != 0 as ::core::ffi::c_int {
        rc = -1 as ::core::ffi::c_int;
    }
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_cursor_head_read(
    mut storage_path: *const ::core::ffi::c_char,
    mut epoch_uuid: *mut ::core::ffi::c_char,
    mut head_seq: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut root: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut path: [::core::ffi::c_char; 1024] = [0; 1024];
    if epoch_uuid.is_null()
        || head_seq.is_null()
        || root_dir(
            storage_path,
            &raw mut root as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if snprintf(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"%s/.cursor_head.json\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut root as *mut ::core::ffi::c_char,
    ) >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut f: *mut FILE = fopen(
        &raw mut path as *mut ::core::ffi::c_char,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if f.is_null() {
        return if *__errno_location() == ENOENT {
            1 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    let mut body: [::core::ffi::c_char; 512] = [0; 512];
    let mut n: size_t = fread(
        &raw mut body as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        1 as size_t,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>().wrapping_sub(1 as size_t),
        f,
    ) as size_t;
    fclose(f);
    body[n] = '\0' as ::core::ffi::c_char;
    let mut json: *mut cJSON = cJSON_Parse(&raw mut body as *mut ::core::ffi::c_char);
    let mut rc: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if !json.is_null() {
        let mut epoch: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            json,
            b"epoch\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut head: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            json,
            b"head_seq\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if cJSON_IsString(epoch) != 0
            && lightnvr_uuid_is_valid((*epoch).valuestring) as ::core::ffi::c_int != 0
            && cJSON_IsNumber(head) != 0
            && (*head).valuedouble >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && (*head).valuedouble <= 9007199254740991.0f64
            && (*head).valuedouble == (*head).valuedouble as int64_t as ::core::ffi::c_double
        {
            safe_strcpy(
                epoch_uuid,
                (*epoch).valuestring,
                OBS_UUID_SIZE as size_t,
                0 as size_t,
            );
            *head_seq = (*head).valuedouble as int64_t;
            rc = 0 as ::core::ffi::c_int;
        }
        cJSON_Delete(json);
    }
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_cursor_head_reconcile(
    mut storage_path: *const ::core::ffi::c_char,
    mut regenerated: *mut bool,
) -> ::core::ffi::c_int {
    if !regenerated.is_null() {
        *regenerated = r#false != 0;
    }
    let mut meta: observation_meta_t = observation_meta_t {
        epoch_uuid: [0; 37],
        oldest_seq: 0,
        head_seq: 0,
    };
    if db_observation_meta_get(&raw mut meta) != C2Rust_Unnamed::DB_OBS_OK.0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut epoch: [::core::ffi::c_char; 37] = [0; 37];
    let mut head: int64_t = 0 as int64_t;
    let mut rc: ::core::ffi::c_int = observation_cursor_head_read(
        storage_path,
        &raw mut epoch as *mut ::core::ffi::c_char,
        &raw mut head,
    );
    if rc < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Observation cursor head sidecar is malformed; recreating it\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if rc == 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut epoch as *mut ::core::ffi::c_char,
            &raw mut meta.epoch_uuid as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && meta.head_seq < head
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Observation history rewound (DB head %lld < recorded head %lld): database was restored; regenerating cursor epoch\0"
                .as_ptr() as *const ::core::ffi::c_char,
            meta.head_seq as ::core::ffi::c_longlong,
            head as ::core::ffi::c_longlong,
        );
        if db_observation_meta_regenerate_epoch() != C2Rust_Unnamed::DB_OBS_OK.0
            || db_observation_meta_get(&raw mut meta) != C2Rust_Unnamed::DB_OBS_OK.0
        {
            return -1 as ::core::ffi::c_int;
        }
        if !regenerated.is_null() {
            *regenerated = r#true != 0;
        }
    }
    return observation_cursor_head_write(
        storage_path,
        &raw mut meta.epoch_uuid as *mut ::core::ffi::c_char,
        meta.head_seq,
    );
}
unsafe extern "C" fn remove_at(
    mut dir_fd: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
    mut path: *const ::core::ffi::c_char,
    mut deleted: *mut ::core::ffi::c_int,
) {
    if unlinkat(dir_fd, name, 0 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int {
        if !deleted.is_null() {
            *deleted += 1;
        }
        return;
    }
    if *__errno_location() == ENOENT || *__errno_location() == ENOTDIR {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Failed to delete observation evidence file %s: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        path,
        strerror(*__errno_location()),
    );
}
unsafe extern "C" fn sweep_final(
    mut s: *mut sweep_t,
    mut dir_fd: ::core::ffi::c_int,
    mut path: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
) {
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    let mut len: size_t = strlen(name).wrapping_sub(4 as size_t);
    if len != (OBS_UUID_SIZE - 1 as ::core::ffi::c_int) as size_t {
        return;
    }
    memcpy(
        &raw mut uuid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        name as *const ::core::ffi::c_void,
        len,
    );
    uuid[len] = '\0' as ::core::ffi::c_char;
    if !lightnvr_uuid_is_valid(&raw mut uuid as *mut ::core::ffi::c_char) {
        return;
    }
    sqlite3_reset((*s).lookup);
    sqlite3_bind_text(
        (*s).lookup,
        1 as ::core::ffi::c_int,
        &raw mut uuid as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    let mut step: ::core::ffi::c_int = sqlite3_step((*s).lookup);
    if step == SQLITE_ROW {
        let mut state: [::core::ffi::c_char; 16] = [0; 16];
        let mut stored: [::core::ffi::c_char; 512] = [0; 512];
        let mut expected: [::core::ffi::c_char; 1024] = [0; 1024];
        copy_text(
            (*s).lookup,
            0 as ::core::ffi::c_int,
            &raw mut state as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        );
        copy_text(
            (*s).lookup,
            1 as ::core::ffi::c_int,
            &raw mut stored as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        );
        if strcmp(
            &raw mut state as *mut ::core::ffi::c_char,
            b"ready\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && resolve_asset_path(
                (*s).root,
                &raw mut stored as *mut ::core::ffi::c_char,
                &raw mut expected as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            ) == 0 as ::core::ffi::c_int
            && strcmp(&raw mut expected as *mut ::core::ffi::c_char, path)
                == 0 as ::core::ffi::c_int
        {
            return;
        }
    } else if step != SQLITE_DONE {
        (*s).errors += 1;
        return;
    }
    remove_at(
        dir_fd,
        name,
        path,
        &raw mut (*(*s).result).stale_files_deleted,
    );
}
unsafe extern "C" fn sweep_dir_fd(
    mut s: *mut sweep_t,
    mut dir_fd: ::core::ffi::c_int,
    mut dir: *const ::core::ffi::c_char,
    mut depth: ::core::ffi::c_int,
) {
    let mut d: *mut DIR = fdopendir(dir_fd);
    if d.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot scan evidence directory %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            dir,
            strerror(*__errno_location()),
        );
        close(dir_fd);
        return;
    }
    let mut fd: ::core::ffi::c_int = dirfd(d);
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    loop {
        entry = readdir(d);
        if entry.is_null() {
            break;
        }
        let mut name: *const ::core::ffi::c_char =
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char;
        if strcmp(name, b".\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
            || strcmp(name, b"..\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            continue;
        }
        let mut path: [::core::ffi::c_char; 1024] = [0; 1024];
        if snprintf(
            &raw mut path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            dir,
            name,
        ) >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int
        {
            continue;
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
        if fstatat(fd, name, &raw mut st, AT_SYMLINK_NOFOLLOW) != 0 as ::core::ffi::c_int {
            continue;
        }
        if st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t {
            if depth >= OBS_WALK_MAX_DEPTH {
                continue;
            }
            let mut child: ::core::ffi::c_int =
                openat(fd, name, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
            if child < 0 as ::core::ffi::c_int {
                continue;
            }
            sweep_dir_fd(
                s,
                child,
                &raw mut path as *mut ::core::ffi::c_char,
                depth + 1 as ::core::ffi::c_int,
            );
            if depth > 1 as ::core::ffi::c_int {
                unlinkat(fd, name, AT_REMOVEDIR);
            }
        } else {
            if !(st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t) {
                continue;
            }
            if has_suffix(name, b".jpg.tmp\0".as_ptr() as *const ::core::ffi::c_char) {
                remove_at(
                    fd,
                    name,
                    &raw mut path as *mut ::core::ffi::c_char,
                    &raw mut (*(*s).result).tmp_files_deleted,
                );
            } else if has_suffix(name, b".jpg\0".as_ptr() as *const ::core::ffi::c_char) {
                sweep_final(s, fd, &raw mut path as *mut ::core::ffi::c_char, name);
            }
        }
    }
    closedir(d);
}
unsafe extern "C" fn sweep_dir(
    mut s: *mut sweep_t,
    mut dir: *const ::core::ffi::c_char,
    mut depth: ::core::ffi::c_int,
) {
    let mut fd: ::core::ffi::c_int = open(dir, O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    if fd < 0 as ::core::ffi::c_int {
        if *__errno_location() != ENOENT {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Cannot scan evidence directory %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                dir,
                strerror(*__errno_location()),
            );
        }
        return;
    }
    sweep_dir_fd(s, fd, dir, depth);
}
unsafe extern "C" fn sweep_files(
    mut root: *const ::core::ffi::c_char,
    mut result: *mut observation_reconcile_result_t,
) -> ::core::ffi::c_int {
    let mut s: sweep_t = sweep_t {
        db: ::core::ptr::null_mut::<sqlite3>(),
        lookup: ::core::ptr::null_mut::<sqlite3_stmt>(),
        root: root,
        result: result,
        errors: 0,
    };
    if db_open_readonly_connection(&raw mut s.db) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if sqlite3_prepare_v2(
        s.db,
        b"SELECT state, path FROM observation_assets WHERE uuid = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut s.lookup,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        db_close_readonly_connection(s.db);
        return -1 as ::core::ffi::c_int;
    }
    sweep_dir(&raw mut s, root, 1 as ::core::ffi::c_int);
    sqlite3_finalize(s.lookup);
    db_close_readonly_connection(s.db);
    return if s.errors != 0 {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn page_by_state(
    mut state: *const ::core::ffi::c_char,
    mut after: *const ::core::ffi::c_char,
    mut rows: *mut asset_row_t,
    mut max: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        b"SELECT uuid, COALESCE(path, '') FROM observation_assets WHERE state = ?1 AND uuid > ?2 ORDER BY uuid LIMIT ?3;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            state,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            after,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(stmt, 3 as ::core::ffi::c_int, max);
        count = 0 as ::core::ffi::c_int;
        let mut step: ::core::ffi::c_int = 0;
        while count < max
            && {
                step = sqlite3_step(stmt);
                step == SQLITE_ROW
            }
        {
            memset(
                rows.offset(count as isize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<asset_row_t>(),
            );
            copy_text(
                stmt,
                0 as ::core::ffi::c_int,
                &raw mut (*rows.offset(count as isize)).uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            );
            copy_text(
                stmt,
                1 as ::core::ffi::c_int,
                &raw mut (*rows.offset(count as isize)).path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            );
            count += 1;
        }
        if count < max && step != SQLITE_DONE {
            count = -1 as ::core::ffi::c_int;
        }
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    return count;
}
unsafe extern "C" fn asset_is_regular_nofollow(
    mut root: *const ::core::ffi::c_char,
    mut stored: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut path: [::core::ffi::c_char; 1024] = [0; 1024];
    if resolve_asset_path(
        root,
        stored,
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut dfd: ::core::ffi::c_int = open(root, O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    if dfd < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut save: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut part: *mut ::core::ffi::c_char = strtok_r(
        (&raw mut path as *mut ::core::ffi::c_char)
            .offset(strlen(root) as isize)
            .offset(1 as ::core::ffi::c_int as isize),
        b"/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut save,
    );
    if part.is_null() {
        close(dfd);
        return 0 as ::core::ffi::c_int;
    }
    let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let mut next: *mut ::core::ffi::c_char = strtok_r(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b"/\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut save,
        );
        if !next.is_null() {
            let mut nfd: ::core::ffi::c_int =
                openat(dfd, part, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
            if nfd < 0 as ::core::ffi::c_int {
                break;
            }
            close(dfd);
            dfd = nfd;
            part = next;
        } else {
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
            ok = (fstatat(dfd, part, &raw mut st, AT_SYMLINK_NOFOLLOW) == 0 as ::core::ffi::c_int
                && st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
                as ::core::ffi::c_int;
            break;
        }
    }
    close(dfd);
    return ok;
}
unsafe extern "C" fn reconcile_state(
    mut root: *const ::core::ffi::c_char,
    mut from: observation_asset_state_t,
    mut result: *mut observation_reconcile_result_t,
) -> ::core::ffi::c_int {
    let mut rows: *mut asset_row_t =
        calloc(OBS_PAGE as size_t, ::core::mem::size_of::<asset_row_t>()) as *mut asset_row_t;
    if rows.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut after: [::core::ffi::c_char; 37] =
        ::core::mem::transmute::<[u8; 37], [::core::ffi::c_char; 37]>(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
    let mut errors: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let mut n: ::core::ffi::c_int = page_by_state(
            observation_asset_state_name(from),
            &raw mut after as *mut ::core::ffi::c_char,
            rows,
            OBS_PAGE,
        );
        if n < 0 as ::core::ffi::c_int {
            errors += 1;
            break;
        } else {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < n {
                let mut reason: *const ::core::ffi::c_char =
                    ::core::ptr::null::<::core::ffi::c_char>();
                if from.0 == observation_asset_state_t::OBS_ASSET_PENDING.0 {
                    reason = b"interrupted\0".as_ptr() as *const ::core::ffi::c_char;
                } else if asset_is_regular_nofollow(
                    root,
                    &raw mut (*rows.offset(i as isize)).path as *mut ::core::ffi::c_char,
                ) == 0
                {
                    reason = b"write_failed\0".as_ptr() as *const ::core::ffi::c_char;
                }
                if !reason.is_null() {
                    let mut rc: ::core::ffi::c_int = db_observation_asset_transition(
                        &raw mut (*rows.offset(i as isize)).uuid as *mut ::core::ffi::c_char,
                        from,
                        observation_asset_state_t::OBS_ASSET_UNAVAILABLE,
                        reason,
                        ::core::ptr::null::<observation_asset_media_t>(),
                        ::core::ptr::null_mut::<observation_transition_result_t>(),
                    );
                    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
                        if from.0 == observation_asset_state_t::OBS_ASSET_PENDING.0 {
                            (*result).pending_interrupted += 1;
                        } else {
                            (*result).ready_missing += 1;
                        }
                    } else if rc != C2Rust_Unnamed::DB_OBS_CAS_LOST.0
                        && rc != C2Rust_Unnamed::DB_OBS_NOT_FOUND.0
                    {
                        errors += 1;
                    }
                }
                i += 1;
            }
            if n < OBS_PAGE {
                break;
            }
            safe_strcpy(
                &raw mut after as *mut ::core::ffi::c_char,
                &raw mut (*rows.offset((n - 1 as ::core::ffi::c_int) as isize)).uuid
                    as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
        }
    }
    free(rows as *mut ::core::ffi::c_void);
    return if errors != 0 {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_retention_startup(
    mut storage_path: *const ::core::ffi::c_char,
    mut result: *mut observation_reconcile_result_t,
) -> ::core::ffi::c_int {
    let mut local: observation_reconcile_result_t = observation_reconcile_result_t {
        tmp_files_deleted: 0,
        stale_files_deleted: 0,
        pending_interrupted: 0,
        ready_missing: 0,
        epoch_regenerated: false,
    };
    if result.is_null() {
        result = &raw mut local;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_reconcile_result_t>(),
    );
    pthread_mutex_lock(&raw mut g_run_mutex);
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g_startup_done,
        0 as ::core::ffi::c_int,
    );
    let mut root: [::core::ffi::c_char; 1024] = [0; 1024];
    if root_dir(
        storage_path,
        &raw mut root as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    ) != 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(&raw mut g_run_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if observation_cursor_head_reconcile(storage_path, &raw mut (*result).epoch_regenerated)
        != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Observation cursor head reconciliation failed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        rc = -1 as ::core::ffi::c_int;
    }
    if sweep_files(&raw mut root as *mut ::core::ffi::c_char, result) != 0 as ::core::ffi::c_int {
        rc = -1 as ::core::ffi::c_int;
    }
    if reconcile_state(
        &raw mut root as *mut ::core::ffi::c_char,
        observation_asset_state_t::OBS_ASSET_PENDING,
        result,
    ) != 0 as ::core::ffi::c_int
    {
        rc = -1 as ::core::ffi::c_int;
    }
    if reconcile_state(
        &raw mut root as *mut ::core::ffi::c_char,
        observation_asset_state_t::OBS_ASSET_READY,
        result,
    ) != 0 as ::core::ffi::c_int
    {
        rc = -1 as ::core::ffi::c_int;
    }
    if rc == 0 as ::core::ffi::c_int {
        let mut meta: observation_meta_t = observation_meta_t {
            epoch_uuid: [0; 37],
            oldest_seq: 0,
            head_seq: 0,
        };
        if db_observation_meta_get(&raw mut meta) == C2Rust_Unnamed::DB_OBS_OK.0 {
            observation_cursor_head_write(
                storage_path,
                &raw mut meta.epoch_uuid as *mut ::core::ffi::c_char,
                meta.head_seq,
            );
        }
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g_startup_done,
            1 as ::core::ffi::c_int,
        );
    }
    pthread_mutex_unlock(&raw mut g_run_mutex);
    if (*result).tmp_files_deleted != 0
        || (*result).stale_files_deleted != 0
        || (*result).pending_interrupted != 0
        || (*result).ready_missing != 0
        || (*result).epoch_regenerated as ::core::ffi::c_int != 0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Observation reconciliation: %d temp and %d stale files deleted, %d pending interrupted, %d ready missing%s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*result).tmp_files_deleted,
            (*result).stale_files_deleted,
            (*result).pending_interrupted,
            (*result).ready_missing,
            if (*result).epoch_regenerated as ::core::ffi::c_int != 0 {
                b", epoch regenerated\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    if rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Observation startup reconciliation incomplete; eviction disabled\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_retention_startup_done() -> bool {
    return ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g_startup_done,
    ) != 0;
}
unsafe extern "C" fn history_days() -> ::core::ffi::c_int {
    let mut value: [::core::ffi::c_char; 32] = [0; 32];
    if db_get_system_setting(
        b"observation_history_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as ::core::ffi::c_int,
    ) != 0 as ::core::ffi::c_int
    {
        return OBS_RETENTION_DEFAULT_HISTORY_DAYS;
    }
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut days: ::core::ffi::c_long = strtol(
        &raw mut value as *mut ::core::ffi::c_char,
        &raw mut end,
        10 as ::core::ffi::c_int,
    );
    if end == &raw mut value as *mut ::core::ffi::c_char
        || days < 1 as ::core::ffi::c_long
        || days > 365 as ::core::ffi::c_long
    {
        return OBS_RETENTION_DEFAULT_HISTORY_DAYS;
    }
    return days as ::core::ffi::c_int;
}
unsafe extern "C" fn prune_history(
    mut now_ms: int64_t,
    mut result: *mut observation_retention_result_t,
) -> ::core::ffi::c_int {
    let mut before_ms: int64_t = (now_ms as ::core::ffi::c_longlong
        - history_days() as int64_t as ::core::ffi::c_longlong * DAY_MS)
        as int64_t;
    while (*result).changes_pruned < OBS_RETENTION_MAX_PRUNED_CHANGES {
        let mut prune: observation_prune_result_t = observation_prune_result_t {
            changes_deleted: 0,
            observations_deleted: 0,
            assets_deleted: 0,
            oldest_seq: 0,
        };
        if db_observations_prune(before_ms, OBS_RETENTION_PRUNE_BATCH, &raw mut prune)
            != C2Rust_Unnamed::DB_OBS_OK.0
        {
            return -1 as ::core::ffi::c_int;
        }
        (*result).changes_pruned += prune.changes_deleted;
        (*result).observations_pruned += prune.observations_deleted;
        (*result).unreferenced_deleted += prune.assets_deleted;
        if prune.changes_deleted < OBS_RETENTION_PRUNE_BATCH {
            return 0 as ::core::ffi::c_int;
        }
    }
    (*result).more_work = r#true != 0;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn delete_asset_row(mut uuid: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut db: *mut sqlite3 = get_db_handle();
    let mut rc: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if !db.is_null() {
        let mut connection: *mut sqlite3_mutex = sqlite3_db_mutex(db);
        sqlite3_mutex_enter(connection);
        let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if sqlite3_prepare_v2(
            db,
            b"DELETE FROM observation_assets WHERE uuid = ?1 AND protected = 0 AND (path IS NULL OR path = '') AND state IN ('unavailable', 'expired') AND NOT EXISTS (SELECT 1 FROM object_observations WHERE asset_uuid = ?1);\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
        {
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
            if sqlite3_step(stmt) == SQLITE_DONE {
                rc = sqlite3_changes(db);
            }
        }
        sqlite3_finalize(stmt);
        sqlite3_mutex_leave(connection);
    }
    pthread_mutex_unlock(mutex);
    return rc;
}
unsafe extern "C" fn cleanup_asset_file(
    mut root: *const ::core::ffi::c_char,
    mut row: *const asset_row_t,
    mut deleted: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*row).path[0usize] == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if remove_asset_file(
        root,
        &raw const (*row).path as *const ::core::ffi::c_char,
        deleted,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut db: *mut sqlite3 = get_db_handle();
    let mut rc: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if !db.is_null() {
        let mut connection: *mut sqlite3_mutex = sqlite3_db_mutex(db);
        sqlite3_mutex_enter(connection);
        let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if sqlite3_prepare_v2(
            db,
            b"UPDATE observation_assets SET path = NULL WHERE uuid = ?1 AND path = ?2 AND state IN ('unavailable', 'expired');\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
        {
            sqlite3_bind_text(
                stmt,
                1 as ::core::ffi::c_int,
                &raw const (*row).uuid as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                stmt,
                2 as ::core::ffi::c_int,
                &raw const (*row).path as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            if sqlite3_step(stmt) == SQLITE_DONE {
                rc = 0 as ::core::ffi::c_int;
            }
        }
        sqlite3_finalize(stmt);
        sqlite3_mutex_leave(connection);
    }
    pthread_mutex_unlock(mutex);
    return rc;
}
unsafe extern "C" fn expire_asset(
    mut root: *const ::core::ffi::c_char,
    mut row: *const asset_row_t,
    mut result: *mut observation_retention_result_t,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = db_observation_asset_transition(
        &raw const (*row).uuid as *const ::core::ffi::c_char,
        observation_asset_state_t::OBS_ASSET_READY,
        observation_asset_state_t::OBS_ASSET_EXPIRED,
        b"retention_expired\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<observation_asset_media_t>(),
        ::core::ptr::null_mut::<observation_transition_result_t>(),
    );
    if rc == C2Rust_Unnamed::DB_OBS_CAS_LOST.0 || rc == C2Rust_Unnamed::DB_OBS_NOT_FOUND.0 {
        return 0 as ::core::ffi::c_int;
    }
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        return -1 as ::core::ffi::c_int;
    }
    (*result).assets_expired += 1;
    if cleanup_asset_file(root, row, &raw mut (*result).files_deleted) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn query_rows(
    mut sql: *const ::core::ffi::c_char,
    mut camera: *const ::core::ffi::c_char,
    mut bound: int64_t,
    mut max: ::core::ffi::c_int,
    mut rows: *mut asset_row_t,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        let mut index: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if !camera.is_null() {
            let c2rust_fresh0 = index;
            index += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh0,
                camera,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        let c2rust_fresh1 = index;
        index += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh1, bound as sqlite3_int64);
        sqlite3_bind_int(stmt, index, max);
        count = 0 as ::core::ffi::c_int;
        let mut step: ::core::ffi::c_int = SQLITE_DONE;
        while count < max && {
            step = sqlite3_step(stmt);
            step == SQLITE_ROW
        } {
            let c2rust_fresh2 = count;
            count += 1;
            let mut row: *mut asset_row_t = rows.offset(c2rust_fresh2 as isize);
            memset(
                row as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<asset_row_t>(),
            );
            copy_text(
                stmt,
                0 as ::core::ffi::c_int,
                &raw mut (*row).uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            );
            copy_text(
                stmt,
                1 as ::core::ffi::c_int,
                &raw mut (*row).path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            );
            (*row).bytes = sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int) as int64_t;
            (*row).ref_ms = sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as int64_t;
            copy_text(
                stmt,
                4 as ::core::ffi::c_int,
                &raw mut (*row).state as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            );
        }
        if count < max && step != SQLITE_DONE {
            count = -1 as ::core::ffi::c_int;
        }
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    return count;
}
unsafe extern "C" fn retry_terminal_files(
    mut root: *const ::core::ffi::c_char,
    mut budget: *mut ::core::ffi::c_int,
    mut result: *mut observation_retention_result_t,
    mut rows: *mut asset_row_t,
) -> ::core::ffi::c_int {
    let mut want: ::core::ffi::c_int = if *budget < OBS_PAGE {
        *budget
    } else {
        OBS_PAGE
    };
    if want <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut n: ::core::ffi::c_int = query_rows(
        b"SELECT uuid, path, COALESCE(bytes, 0), 0, state FROM observation_assets WHERE state IN ('expired', 'unavailable') AND path IS NOT NULL AND path <> '' AND ?1 > 0 ORDER BY uuid LIMIT ?2;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        1 as int64_t,
        want,
        rows,
    );
    if n < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < n {
        *budget -= 1;
        if cleanup_asset_file(
            root,
            rows.offset(i as isize),
            &raw mut (*result).files_deleted,
        ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if n == want {
        (*result).more_work = r#true != 0;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn reclaim_unreferenced(
    mut root: *const ::core::ffi::c_char,
    mut budget: *mut ::core::ffi::c_int,
    mut result: *mut observation_retention_result_t,
    mut rows: *mut asset_row_t,
) -> ::core::ffi::c_int {
    if *budget <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut n: ::core::ffi::c_int = query_rows(
        b"SELECT a.uuid, COALESCE(a.path, ''), COALESCE(a.bytes, 0), 0, a.state FROM observation_assets a WHERE a.state <> 'pending' AND a.protected = 0 AND ?1 > 0 AND NOT EXISTS (SELECT 1 FROM object_observations o WHERE o.asset_uuid = a.uuid) ORDER BY a.uuid LIMIT ?2;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        1 as int64_t,
        *budget,
        rows,
    );
    if n < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < n {
        *budget -= 1;
        if strcmp(
            &raw mut (*rows.offset(i as isize)).state as *mut ::core::ffi::c_char,
            b"ready\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if expire_asset(root, rows.offset(i as isize), result) < 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
        } else if cleanup_asset_file(
            root,
            rows.offset(i as isize),
            &raw mut (*result).files_deleted,
        ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut deleted: ::core::ffi::c_int =
            delete_asset_row(&raw mut (*rows.offset(i as isize)).uuid as *mut ::core::ffi::c_char);
        if deleted < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        (*result).unreferenced_deleted += deleted;
        i += 1;
    }
    if *budget <= 0 as ::core::ffi::c_int {
        (*result).more_work = r#true != 0;
    }
    return 0 as ::core::ffi::c_int;
}
pub const EVICTION_CANDIDATES: [::core::ffi::c_char; 417] = unsafe {
    ::core::mem::transmute::<
        [u8; 417],
        [::core::ffi::c_char; 417],
    >(
        *b"SELECT uuid, path, bytes, ref_ms, 'ready' FROM (SELECT a.uuid AS uuid, COALESCE(a.path, '') AS path, COALESCE(a.bytes, 0) AS bytes, COALESCE((SELECT MAX(o.observed_at_ms) FROM object_observations o WHERE o.asset_uuid = a.uuid), a.captured_at_ms, a.created_at_ms) AS ref_ms FROM observation_assets a WHERE a.camera_uuid = ?1 AND a.state = 'ready' AND a.protected = 0) WHERE ref_ms < ?2 ORDER BY ref_ms, uuid LIMIT ?3;\0",
    )
};
unsafe extern "C" fn count_unprotected_ready(
    mut camera: *const ::core::ffi::c_char,
    mut count: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        b"SELECT count(*) FROM observation_assets WHERE camera_uuid = ? AND state = 'ready' AND protected = 0;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            camera,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if sqlite3_step(stmt) == SQLITE_ROW {
            *count = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
            rc = 0 as ::core::ffi::c_int;
        }
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    return rc;
}
unsafe extern "C" fn enforce_camera(
    mut root: *const ::core::ffi::c_char,
    mut now_ms: int64_t,
    mut camera: *mut observation_retention_camera_status_t,
    mut budget: *mut ::core::ffi::c_int,
    mut result: *mut observation_retention_result_t,
    mut rows: *mut asset_row_t,
) -> ::core::ffi::c_int {
    let mut cutoff: int64_t = (now_ms as ::core::ffi::c_longlong
        - (*camera).max_age_days as int64_t as ::core::ffi::c_longlong * DAY_MS)
        as int64_t;
    if *budget > 0 as ::core::ffi::c_int {
        let mut want: ::core::ffi::c_int = if *budget < OBS_PAGE {
            *budget
        } else {
            OBS_PAGE
        };
        let mut n: ::core::ffi::c_int = query_rows(
            EVICTION_CANDIDATES.as_ptr(),
            &raw mut (*camera).camera_uuid as *mut ::core::ffi::c_char,
            cutoff,
            want,
            rows,
        );
        if n < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < n {
            *budget -= 1;
            let mut rc: ::core::ffi::c_int = expire_asset(root, rows.offset(i as isize), result);
            if rc < 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            if rc > 0 as ::core::ffi::c_int {
                (*camera).retained_bytes -= (*rows.offset(i as isize)).bytes;
            }
            i += 1;
        }
        if n == want {
            (*result).more_work = r#true != 0;
        }
    }
    while (*camera).retained_bytes > (*camera).max_bytes && *budget > 0 as ::core::ffi::c_int {
        let mut want_0: ::core::ffi::c_int = if *budget < OBS_PAGE {
            *budget
        } else {
            OBS_PAGE
        };
        let mut n_0: ::core::ffi::c_int = query_rows(
            EVICTION_CANDIDATES.as_ptr(),
            &raw mut (*camera).camera_uuid as *mut ::core::ffi::c_char,
            INT64_MAX as int64_t,
            want_0,
            rows,
        );
        if n_0 < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if n_0 == 0 as ::core::ffi::c_int {
            break;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < n_0 && (*camera).retained_bytes > (*camera).max_bytes {
            *budget -= 1;
            let mut rc_0: ::core::ffi::c_int =
                expire_asset(root, rows.offset(i_0 as isize), result);
            if rc_0 < 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            if rc_0 > 0 as ::core::ffi::c_int {
                (*camera).retained_bytes -= (*rows.offset(i_0 as isize)).bytes;
            }
            i_0 += 1;
        }
        if n_0 < want_0 {
            break;
        }
    }
    (*camera).budget_exhausted = r#false != 0;
    if (*camera).retained_bytes > (*camera).max_bytes {
        let mut evictable: int64_t = 0 as int64_t;
        if count_unprotected_ready(
            &raw mut (*camera).camera_uuid as *mut ::core::ffi::c_char,
            &raw mut evictable,
        ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if evictable == 0 as int64_t {
            (*camera).budget_exhausted = r#true != 0;
            (*result).cameras_budget_exhausted += 1;
        } else {
            (*result).more_work = r#true != 0;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn load_cameras(
    mut out: *mut *mut observation_retention_camera_status_t,
    mut count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *out = ::core::ptr::null_mut::<observation_retention_camera_status_t>();
    *count = 0 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut capacity: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        b"SELECT c.camera_uuid, s.observation_max_age_days, s.observation_max_bytes, (SELECT COALESCE(SUM(x.bytes), 0) FROM observation_assets x  WHERE x.camera_uuid = c.camera_uuid AND x.state = 'ready') FROM (SELECT DISTINCT camera_uuid FROM observation_assets WHERE state = 'ready') c LEFT JOIN streams s ON s.camera_uuid = c.camera_uuid ORDER BY c.camera_uuid;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        rc = 0 as ::core::ffi::c_int;
        let mut step: ::core::ffi::c_int = 0;
        loop {
            step = sqlite3_step(stmt);
            if step != SQLITE_ROW {
                break;
            }
            if *count == capacity {
                let mut next: ::core::ffi::c_int = if capacity != 0 {
                    capacity * 2 as ::core::ffi::c_int
                } else {
                    16 as ::core::ffi::c_int
                };
                let mut grown: *mut ::core::ffi::c_void = realloc(
                    *out as *mut ::core::ffi::c_void,
                    (next as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<
                                observation_retention_camera_status_t,
                            >(),
                        ),
                );
                if grown.is_null() {
                    rc = -1 as ::core::ffi::c_int;
                    break;
                } else {
                    *out = grown as *mut observation_retention_camera_status_t;
                    capacity = next;
                }
            }
            let c2rust_fresh3 = *count;
            *count += 1;
            let mut camera: *mut observation_retention_camera_status_t = (*out)
                .offset(c2rust_fresh3 as isize);
            memset(
                camera as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<observation_retention_camera_status_t>(),
            );
            copy_text(
                stmt,
                0 as ::core::ffi::c_int,
                &raw mut (*camera).camera_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            );
            (*camera).max_age_days = sqlite3_column_int(stmt, 1 as ::core::ffi::c_int);
            (*camera).max_bytes = sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int)
                as int64_t;
            (*camera).retained_bytes = sqlite3_column_int64(
                stmt,
                3 as ::core::ffi::c_int,
            ) as int64_t;
            if (*camera).max_age_days <= 0 as ::core::ffi::c_int {
                (*camera).max_age_days = OBS_RETENTION_DEFAULT_MAX_AGE_DAYS;
            }
            if (*camera).max_bytes <= 0 as int64_t {
                (*camera).max_bytes = OBS_RETENTION_DEFAULT_MAX_BYTES as int64_t;
            }
        }
        if rc == 0 as ::core::ffi::c_int && step != SQLITE_DONE {
            rc = -1 as ::core::ffi::c_int;
        }
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    if rc != 0 as ::core::ffi::c_int {
        free(*out as *mut ::core::ffi::c_void);
        *out = ::core::ptr::null_mut::<observation_retention_camera_status_t>();
        *count = 0 as ::core::ffi::c_int;
    }
    return rc;
}
unsafe extern "C" fn find_status(
    mut cameras: *const observation_retention_camera_status_t,
    mut count: ::core::ffi::c_int,
    mut uuid: *const ::core::ffi::c_char,
) -> *const observation_retention_camera_status_t {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if strcmp(
            &raw const (*cameras.offset(i as isize)).camera_uuid as *const ::core::ffi::c_char,
            uuid,
        ) == 0 as ::core::ffi::c_int
        {
            return cameras.offset(i as isize);
        }
        i += 1;
    }
    return ::core::ptr::null::<observation_retention_camera_status_t>();
}
unsafe extern "C" fn publish_status(
    mut cameras: *mut observation_retention_camera_status_t,
    mut count: ::core::ffi::c_int,
) {
    let mut exhausted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if (*cameras.offset(i as isize)).budget_exhausted {
            exhausted += 1;
        }
        i += 1;
    }
    pthread_mutex_lock(&raw mut g_status_mutex);
    let mut old: *mut observation_retention_camera_status_t = g_status;
    let mut old_count: ::core::ffi::c_int = g_status_count;
    g_status = cameras;
    g_status_count = count;
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g_exhausted_count,
        exhausted,
    );
    pthread_mutex_unlock(&raw mut g_status_mutex);
    if Some(
        observation_evidence_set_budget_exhausted
            as unsafe extern "C" fn(*const ::core::ffi::c_char, bool) -> (),
    )
    .is_some()
    {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < count {
            let mut before: *const observation_retention_camera_status_t = find_status(
                old,
                old_count,
                &raw mut (*cameras.offset(i_0 as isize)).camera_uuid as *mut ::core::ffi::c_char,
            );
            if (*cameras.offset(i_0 as isize)).budget_exhausted as ::core::ffi::c_int != 0
                || !before.is_null() && (*before).budget_exhausted as ::core::ffi::c_int != 0
            {
                observation_evidence_set_budget_exhausted(
                    &raw mut (*cameras.offset(i_0 as isize)).camera_uuid
                        as *mut ::core::ffi::c_char,
                    (*cameras.offset(i_0 as isize)).budget_exhausted,
                );
            }
            i_0 += 1;
        }
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < old_count {
            if (*old.offset(i_1 as isize)).budget_exhausted as ::core::ffi::c_int != 0
                && find_status(
                    cameras,
                    count,
                    &raw mut (*old.offset(i_1 as isize)).camera_uuid as *mut ::core::ffi::c_char,
                )
                .is_null()
            {
                observation_evidence_set_budget_exhausted(
                    &raw mut (*old.offset(i_1 as isize)).camera_uuid as *mut ::core::ffi::c_char,
                    r#false != 0,
                );
            }
            i_1 += 1;
        }
    }
    free(old as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn retention_run_locked(
    mut storage_path: *const ::core::ffi::c_char,
    mut now_ms: int64_t,
    mut result: *mut observation_retention_result_t,
) -> ::core::ffi::c_int {
    let mut local: observation_retention_result_t = observation_retention_result_t {
        assets_expired: 0,
        unreferenced_deleted: 0,
        files_deleted: 0,
        changes_pruned: 0,
        observations_pruned: 0,
        cameras_budget_exhausted: 0,
        more_work: false,
        oldest_seq: 0,
    };
    if result.is_null() {
        result = &raw mut local;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_retention_result_t>(),
    );
    let mut root: [::core::ffi::c_char; 1024] = [0; 1024];
    if root_dir(
        storage_path,
        &raw mut root as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g_startup_done,
    ) == 0
    {
        return 1 as ::core::ffi::c_int;
    }
    let mut rows: *mut asset_row_t =
        calloc(OBS_PAGE as size_t, ::core::mem::size_of::<asset_row_t>()) as *mut asset_row_t;
    if rows.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut budget: ::core::ffi::c_int = OBS_RETENTION_MAX_EVICTIONS;
    let mut rc: ::core::ffi::c_int = prune_history(now_ms, result);
    if rc == 0 as ::core::ffi::c_int {
        rc = retry_terminal_files(
            &raw mut root as *mut ::core::ffi::c_char,
            &raw mut budget,
            result,
            rows,
        );
    }
    if rc == 0 as ::core::ffi::c_int {
        rc = reclaim_unreferenced(
            &raw mut root as *mut ::core::ffi::c_char,
            &raw mut budget,
            result,
            rows,
        );
    }
    let mut cameras: *mut observation_retention_camera_status_t =
        ::core::ptr::null_mut::<observation_retention_camera_status_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if rc == 0 as ::core::ffi::c_int {
        rc = load_cameras(&raw mut cameras, &raw mut camera_count);
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while rc == 0 as ::core::ffi::c_int && i < camera_count {
        rc = enforce_camera(
            &raw mut root as *mut ::core::ffi::c_char,
            now_ms,
            cameras.offset(i as isize),
            &raw mut budget,
            result,
            rows,
        );
        i += 1;
    }
    if rc == 0 as ::core::ffi::c_int {
        publish_status(cameras, camera_count);
        cameras = ::core::ptr::null_mut::<observation_retention_camera_status_t>();
    }
    free(cameras as *mut ::core::ffi::c_void);
    free(rows as *mut ::core::ffi::c_void);
    let mut meta: observation_meta_t = observation_meta_t {
        epoch_uuid: [0; 37],
        oldest_seq: 0,
        head_seq: 0,
    };
    if db_observation_meta_get(&raw mut meta) == C2Rust_Unnamed::DB_OBS_OK.0 {
        (*result).oldest_seq = meta.oldest_seq;
    }
    if (*result).assets_expired != 0
        || (*result).unreferenced_deleted != 0
        || (*result).changes_pruned != 0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Observation retention: %d media expired, %d files deleted, %d unreferenced assets reclaimed, %d changes and %d observations pruned (oldest seq %lld)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*result).assets_expired,
            (*result).files_deleted,
            (*result).unreferenced_deleted,
            (*result).changes_pruned,
            (*result).observations_pruned,
            (*result).oldest_seq as ::core::ffi::c_longlong,
        );
    }
    if (*result).cameras_budget_exhausted != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Observation evidence budget exhausted on %d camera(s): only protected media remains\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*result).cameras_budget_exhausted,
        );
    }
    if rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Observation retention pass failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return if rc == 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_retention_run(
    mut storage_path: *const ::core::ffi::c_char,
    mut now_ms: int64_t,
    mut result: *mut observation_retention_result_t,
) -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut g_run_mutex);
    let mut rc: ::core::ffi::c_int = retention_run_locked(storage_path, now_ms, result);
    pthread_mutex_unlock(&raw mut g_run_mutex);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_retention_maintenance_tick() {
    if ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g_startup_done,
    ) == 0
    {
        return;
    }
    pthread_mutex_lock(&raw mut g_run_mutex);
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if !g_more_work
        && g_last_tick != 0 as time_t
        && now - g_last_tick < OBS_RETENTION_INTERVAL_SEC as time_t
    {
        pthread_mutex_unlock(&raw mut g_run_mutex);
        return;
    }
    g_last_tick = now;
    let mut result: observation_retention_result_t = observation_retention_result_t {
        assets_expired: 0,
        unreferenced_deleted: 0,
        files_deleted: 0,
        changes_pruned: 0,
        observations_pruned: 0,
        cameras_budget_exhausted: 0,
        more_work: false,
        oldest_seq: 0,
    };
    let mut rc: ::core::ffi::c_int = retention_run_locked(
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
        wall_ms(),
        &raw mut result,
    );
    g_more_work = rc == 0 as ::core::ffi::c_int && result.more_work as ::core::ffi::c_int != 0;
    pthread_mutex_unlock(&raw mut g_run_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_retention_budget_exhausted(
    mut camera_uuid: *const ::core::ffi::c_char,
) -> bool {
    if camera_uuid.is_null()
        || ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut g_exhausted_count,
        ) == 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    if pthread_mutex_trylock(&raw mut g_status_mutex) != 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut exhausted: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_status_count {
        if strcmp(
            &raw mut (*g_status.offset(i as isize)).camera_uuid as *mut ::core::ffi::c_char,
            camera_uuid,
        ) == 0 as ::core::ffi::c_int
        {
            exhausted = (*g_status.offset(i as isize)).budget_exhausted;
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut g_status_mutex);
    return exhausted;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_retention_budget_exhausted_count() -> ::core::ffi::c_int {
    return ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g_exhausted_count,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_retention_camera_status(
    mut out: *mut observation_retention_camera_status_t,
    mut max: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if out.is_null() || max <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut g_status_mutex);
    let mut n: ::core::ffi::c_int = if g_status_count < max {
        g_status_count
    } else {
        max
    };
    if n > 0 as ::core::ffi::c_int {
        memcpy(
            out as *mut ::core::ffi::c_void,
            g_status as *const ::core::ffi::c_void,
            (n as size_t)
                .wrapping_mul(::core::mem::size_of::<observation_retention_camera_status_t>()),
        );
    }
    pthread_mutex_unlock(&raw mut g_status_mutex);
    return n;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_retention_reset_for_testing() {
    pthread_mutex_lock(&raw mut g_run_mutex);
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut g_startup_done,
        0 as ::core::ffi::c_int,
    );
    publish_status(
        ::core::ptr::null_mut::<observation_retention_camera_status_t>(),
        0 as ::core::ffi::c_int,
    );
    g_last_tick = 0 as time_t;
    g_more_work = r#false != 0;
    pthread_mutex_unlock(&raw mut g_run_mutex);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
