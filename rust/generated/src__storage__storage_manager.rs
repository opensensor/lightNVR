unsafe extern "C" {
    pub type __dirstream;
    pub type sqlite3;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn fstatat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __buf: *mut stat,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn statvfs(
        __file: *const ::core::ffi::c_char,
        __buf: *mut statvfs,
    ) -> ::core::ffi::c_int;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
    unsafe fn dirfd(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn unlinkat(
        __fd: ::core::ffi::c_int,
        __name: *const ::core::ffi::c_char,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
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
    unsafe fn pthread_detach(__th: pthread_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
    static mut g_config: config_t;
    unsafe fn storage_recording_delete(
        recording_id: uint64_t,
        reason: *const ::core::ffi::c_char,
        local_bytes_removed: *mut uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn storage_deletion_prune_completed(
        older_than_days: ::core::ffi::c_int,
        max_rows: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_storage_target_count() -> ::core::ffi::c_int;
    unsafe fn db_storage_target_list(
        targets: *mut storage_target_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_storage_target_get(
        uuid: *const ::core::ffi::c_char,
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
    unsafe fn db_storage_target_get_default(
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
    unsafe fn db_storage_target_resolve_path(
        target_uuid: *const ::core::ffi::c_char,
        object_key: *const ::core::ffi::c_char,
        absolute_path: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_storage_target_mount_guard_active(target: *const storage_target_t) -> bool;
    unsafe fn storage_source_worker_start() -> ::core::ffi::c_int;
    unsafe fn storage_source_worker_shutdown();
    unsafe fn storage_source_trim(bytes_needed: uint64_t) -> uint64_t;
    unsafe fn storage_migration_worker_start() -> ::core::ffi::c_int;
    unsafe fn storage_migration_worker_shutdown();
    unsafe fn storage_migration_worker_wake();
    unsafe fn force_refresh_cache() -> ::core::ffi::c_int;
    unsafe fn storage_target_health_register(
        root_path: *const ::core::ffi::c_char,
        recording_path: *const ::core::ffi::c_char,
        write_probe_enabled: bool,
    ) -> bool;
    unsafe fn storage_target_probe_and_publish(
        uuid: *const ::core::ffi::c_char,
        write_test: bool,
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
    unsafe fn storage_target_refresh_health_and_publish() -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn db_auth_cleanup_sessions() -> ::core::ffi::c_int;
    unsafe fn get_stream_config_by_name(
        name: *const ::core::ffi::c_char,
        stream: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_retention_config(
        stream_name: *const ::core::ffi::c_char,
        config: *mut stream_retention_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_all_stream_names_including_disabled(
        names: *mut [::core::ffi::c_char; 256],
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn get_orphaned_recording_stream_names(
        names: *mut [::core::ffi::c_char; 256],
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn finalize_recording_reconciliation(
        id: uint64_t,
        end_time: time_t,
        size_bytes: uint64_t,
        playback_issue: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn get_recording_metadata_by_path(
        file_path: *const ::core::ffi::c_char,
        metadata: *mut recording_metadata_t,
    ) -> ::core::ffi::c_int;
    unsafe fn delete_recording_metadata(id: uint64_t) -> ::core::ffi::c_int;
    unsafe fn delete_stale_incomplete_recording_metadata(id: uint64_t) -> ::core::ffi::c_int;
    unsafe fn get_recordings_for_retention(
        stream_name: *const ::core::ffi::c_char,
        retention_days: ::core::ffi::c_int,
        detection_retention_days: ::core::ffi::c_int,
        recordings: *mut recording_metadata_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn get_recordings_for_quota_enforcement(
        stream_name: *const ::core::ffi::c_char,
        recordings: *mut recording_metadata_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn get_orphaned_db_entries(
        recordings: *mut recording_metadata_t,
        max_count: ::core::ffi::c_int,
        total_checked: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn get_recordings_for_tiered_retention(
        stream_name: *const ::core::ffi::c_char,
        base_retention_days: ::core::ffi::c_int,
        tier_multipliers: *const ::core::ffi::c_double,
        recordings: *mut recording_metadata_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn get_recordings_for_pressure_cleanup_target(
        storage_target_uuid: *const ::core::ffi::c_char,
        recordings: *mut recording_metadata_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn get_recordings_for_pressure_cleanup_default_target(
        storage_target_uuid: *const ::core::ffi::c_char,
        recordings: *mut recording_metadata_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stale_incomplete_recordings_page(
        recordings: *mut recording_metadata_t,
        max_count: ::core::ffi::c_int,
        cutoff: time_t,
        after_id: uint64_t,
        maximum_id: *mut uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_recordings_capture_startup_boundary(
        maximum_id: *mut uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_startup_incomplete_recordings_page(
        recordings: *mut recording_metadata_t,
        max_count: ::core::ffi::c_int,
        after_id: uint64_t,
        maximum_id: uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_storage_bytes(stream_name: *const ::core::ffi::c_char) -> int64_t;
    unsafe fn delete_old_detections_for_stream(
        stream_name: *const ::core::ffi::c_char,
        max_age: uint64_t,
        batch_limit: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_lpr_reads_prune(
        received_before_ms: int64_t,
        batch_limit: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_get_system_setting(
        key: *const ::core::ffi::c_char,
        out: *mut ::core::ffi::c_char,
        out_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn delete_recording_thumbnails(recording_id: uint64_t);
    unsafe fn recording_media_probe_issue(
        path: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    unsafe fn build_recording_transcode_cache_path(
        storage_path: *const ::core::ffi::c_char,
        recording_id: uint64_t,
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn cleanup_stale_hls_segments();
    unsafe fn event_producer_publish_storage_pressure(
        level: *const ::core::ffi::c_char,
        previous_level: *const ::core::ffi::c_char,
        used_percent: ::core::ffi::c_double,
        free_bytes: uint64_t,
        occurred_at: time_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn event_producer_publish_storage_recovered(
        previous_level: *const ::core::ffi::c_char,
        used_percent: ::core::ffi::c_double,
        free_bytes: uint64_t,
        occurred_at: time_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
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
    unsafe fn mqtt_publish_raw(
        topic: *const ::core::ffi::c_char,
        payload: *const ::core::ffi::c_char,
        retain: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn observation_retention_maintenance_tick();
    unsafe fn sanitize_stream_name(
        input: *const ::core::ffi::c_char,
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    );
    unsafe fn ensure_dir(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn mkdir_recursive(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
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
pub type __useconds_t = ::core::ffi::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_info_t {
    pub path: [::core::ffi::c_char; 512],
    pub stream_name: [::core::ffi::c_char; 64],
    pub start_time: time_t,
    pub end_time: time_t,
    pub size_bytes: uint64_t,
    pub codec: [::core::ffi::c_char; 16],
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub fps: ::core::ffi::c_int,
    pub is_complete: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_stats_t {
    pub total_space: uint64_t,
    pub used_space: uint64_t,
    pub free_space: uint64_t,
    pub reserved_space: uint64_t,
    pub total_recordings: uint64_t,
    pub total_recording_bytes: uint64_t,
    pub oldest_recording_time: uint64_t,
    pub newest_recording_time: uint64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct disk_pressure_level_t(pub ::core::ffi::c_uint);
impl disk_pressure_level_t {
    pub const DISK_PRESSURE_NORMAL: Self = Self(0);
    pub const DISK_PRESSURE_WARNING: Self = Self(1);
    pub const DISK_PRESSURE_CRITICAL: Self = Self(2);
    pub const DISK_PRESSURE_EMERGENCY: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_health_t {
    pub pressure_level: disk_pressure_level_t,
    pub free_space_pct: ::core::ffi::c_double,
    pub free_space_bytes: uint64_t,
    pub total_space_bytes: uint64_t,
    pub used_space_bytes: uint64_t,
    pub last_check_time: time_t,
    pub last_cleanup_time: time_t,
    pub last_deep_time: time_t,
    pub last_cleanup_deleted: ::core::ffi::c_int,
    pub last_cleanup_freed: uint64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct storage_target_pressure_t(pub ::core::ffi::c_uint);
impl storage_target_pressure_t {
    pub const STORAGE_TARGET_PRESSURE_UNAVAILABLE: Self = Self(0);
    pub const STORAGE_TARGET_PRESSURE_NORMAL: Self = Self(1);
    pub const STORAGE_TARGET_PRESSURE_HIGH: Self = Self(2);
    pub const STORAGE_TARGET_PRESSURE_RESERVE: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_target_cleanup_result_t {
    pub initial_pressure: storage_target_pressure_t,
    pub target_free_bytes: uint64_t,
    pub available_bytes_before: uint64_t,
    pub available_bytes_after: uint64_t,
    pub deleted_recordings: ::core::ffi::c_int,
    pub freed_bytes: uint64_t,
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
pub struct C2Rust_Unnamed_1 {
    pub thread: pthread_t,
    pub running: bool,
    pub exited: bool,
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub last_heartbeat: time_t,
    pub last_standard: time_t,
    pub last_deep: time_t,
    pub health: storage_health_t,
    pub force_cleanup: bool,
    pub force_aggressive: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub storage_path: [::core::ffi::c_char; 512],
    pub max_size: uint64_t,
    pub retention_days: ::core::ffi::c_int,
    pub auto_delete_oldest: bool,
    pub total_space: uint64_t,
    pub used_space: uint64_t,
    pub reserved_space: uint64_t,
    pub startup_recording_id: uint64_t,
    pub startup_recording_boundary_valid: bool,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_retention_config_t {
    pub retention_days: ::core::ffi::c_int,
    pub detection_retention_days: ::core::ffi::c_int,
    pub max_storage_mb: uint64_t,
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
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AT_SYMLINK_NOFOLLOW: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
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
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const DISK_PRESSURE_WARNING_PCT: ::core::ffi::c_double = 20.0f64;
pub const DISK_PRESSURE_CRITICAL_PCT: ::core::ffi::c_double = 10.0f64;
pub const DISK_PRESSURE_EMERGENCY_PCT: ::core::ffi::c_double = 5.0f64;
#[inline]
unsafe extern "C" fn storage_target_pressure_evaluate(
    mut capacity_bytes: uint64_t,
    mut available_bytes: uint64_t,
    mut reserve_bytes: uint64_t,
    mut high_watermark_pct: ::core::ffi::c_double,
) -> storage_target_pressure_t {
    if capacity_bytes == 0 as uint64_t || available_bytes > capacity_bytes {
        return storage_target_pressure_t::STORAGE_TARGET_PRESSURE_UNAVAILABLE;
    }
    if available_bytes <= reserve_bytes {
        return storage_target_pressure_t::STORAGE_TARGET_PRESSURE_RESERVE;
    }
    let mut used_pct: ::core::ffi::c_double = 100.0f64
        * capacity_bytes.wrapping_sub(available_bytes) as ::core::ffi::c_double
        / capacity_bytes as ::core::ffi::c_double;
    return storage_target_pressure_t(
        (if used_pct >= high_watermark_pct {
            storage_target_pressure_t::STORAGE_TARGET_PRESSURE_HIGH.0 as ::core::ffi::c_int
        } else {
            storage_target_pressure_t::STORAGE_TARGET_PRESSURE_NORMAL.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
}
#[inline]
unsafe extern "C" fn storage_target_cleanup_goal_bytes(
    mut capacity_bytes: uint64_t,
    mut reserve_bytes: uint64_t,
    mut low_watermark_pct: ::core::ffi::c_double,
) -> uint64_t {
    if capacity_bytes == 0 as uint64_t {
        return reserve_bytes;
    }
    let mut free_pct: ::core::ffi::c_double = 100.0f64 - low_watermark_pct;
    if free_pct < 0.0f64 {
        free_pct = 0.0f64;
    }
    if free_pct > 100.0f64 {
        free_pct = 100.0f64;
    }
    let mut watermark_goal: uint64_t =
        (capacity_bytes as ::core::ffi::c_double * free_pct / 100.0f64) as uint64_t;
    return if reserve_bytes > watermark_goal {
        reserve_bytes
    } else {
        watermark_goal
    };
}
#[inline]
unsafe extern "C" fn storage_target_pressure_name(
    mut pressure: storage_target_pressure_t,
) -> *const ::core::ffi::c_char {
    match pressure {
        storage_target_pressure_t::STORAGE_TARGET_PRESSURE_NORMAL => {
            return b"normal\0".as_ptr() as *const ::core::ffi::c_char;
        }
        storage_target_pressure_t::STORAGE_TARGET_PRESSURE_HIGH => {
            return b"high\0".as_ptr() as *const ::core::ffi::c_char;
        }
        storage_target_pressure_t::STORAGE_TARGET_PRESSURE_RESERVE => {
            return b"reserve\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
#[inline]
unsafe extern "C" fn disk_pressure_level_str(
    mut level: disk_pressure_level_t,
) -> *const ::core::ffi::c_char {
    match level {
        disk_pressure_level_t::DISK_PRESSURE_NORMAL => {
            return b"Normal\0".as_ptr() as *const ::core::ffi::c_char;
        }
        disk_pressure_level_t::DISK_PRESSURE_WARNING => {
            return b"Warning\0".as_ptr() as *const ::core::ffi::c_char;
        }
        disk_pressure_level_t::DISK_PRESSURE_CRITICAL => {
            return b"Critical\0".as_ptr() as *const ::core::ffi::c_char;
        }
        disk_pressure_level_t::DISK_PRESSURE_EMERGENCY => {
            return b"Emergency\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return b"Unknown\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
#[inline]
unsafe extern "C" fn evaluate_disk_pressure_level(
    mut free_pct: ::core::ffi::c_double,
) -> disk_pressure_level_t {
    if free_pct < DISK_PRESSURE_EMERGENCY_PCT {
        return disk_pressure_level_t::DISK_PRESSURE_EMERGENCY;
    }
    if free_pct < DISK_PRESSURE_CRITICAL_PCT {
        return disk_pressure_level_t::DISK_PRESSURE_CRITICAL;
    }
    if free_pct < DISK_PRESSURE_WARNING_PCT {
        return disk_pressure_level_t::DISK_PRESSURE_WARNING;
    }
    return disk_pressure_level_t::DISK_PRESSURE_NORMAL;
}
#[inline]
unsafe extern "C" fn should_continue_emergency_cleanup(
    mut initial_level: disk_pressure_level_t,
    mut current_level: disk_pressure_level_t,
    mut aggressive: bool,
) -> bool {
    if initial_level.0 < disk_pressure_level_t::DISK_PRESSURE_CRITICAL.0 {
        return r#true != 0;
    }
    return current_level.0 >= disk_pressure_level_t::DISK_PRESSURE_CRITICAL.0;
}
pub const STORAGE_DELETION_PRUNE_DEFAULT_DAYS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const STORAGE_DELETION_PRUNE_DEFAULT_ROWS: ::core::ffi::c_int = 50000 as ::core::ffi::c_int;
pub const STORAGE_TARGET_MAX_COUNT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_STREAMS_BATCH: ::core::ffi::c_int = MAX_STREAMS;
pub const MAX_RECORDINGS_PER_STREAM: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const MAX_DETECTIONS_PER_BATCH: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
pub const MAX_ORPHANED_BATCH: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
pub const RETENTION_TIME_BUDGET_SEC: ::core::ffi::c_int = 300 as ::core::ffi::c_int;
pub const DEFAULT_RETENTION_DAYS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const ORPHAN_SAFETY_THRESHOLD: ::core::ffi::c_double = 0.5f64;
pub const MIN_RECORDINGS_FOR_THRESHOLD: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const MP4_SUBDIR: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"mp4\0") };
static mut storage_manager: C2Rust_Unnamed_2 = unsafe {
    C2Rust_Unnamed_2 {
        storage_path: ::core::mem::transmute::<
            [u8; 512],
            [::core::ffi::c_char; 512],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        max_size: 0 as uint64_t,
        retention_days: 30 as ::core::ffi::c_int,
        auto_delete_oldest: r#true != 0,
        total_space: 0 as uint64_t,
        used_space: 0 as uint64_t,
        reserved_space: 0 as uint64_t,
        startup_recording_id: 0,
        startup_recording_boundary_valid: false,
    }
};
pub const CLEANUP_PACE_MIN_US: ::core::ffi::c_long = 5000 as ::core::ffi::c_long;
pub const CLEANUP_PACE_MAX_US: ::core::ffi::c_long = 250000 as ::core::ffi::c_long;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_cleanup_pace_delay_us(
    mut elapsed_us: ::core::ffi::c_long,
) -> ::core::ffi::c_uint {
    if elapsed_us < CLEANUP_PACE_MIN_US {
        return CLEANUP_PACE_MIN_US as ::core::ffi::c_uint;
    }
    if elapsed_us > CLEANUP_PACE_MAX_US {
        return CLEANUP_PACE_MAX_US as ::core::ffi::c_uint;
    }
    return elapsed_us as ::core::ffi::c_uint;
}
unsafe extern "C" fn cleanup_pace_begin(mut started: *mut timespec) {
    clock_gettime(CLOCK_MONOTONIC, started);
}
unsafe extern "C" fn cleanup_pace_end(mut started: *const timespec) {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut now);
    let mut elapsed_us: ::core::ffi::c_long = (now.tv_sec - (*started).tv_sec)
        * 1000000 as ::core::ffi::c_long
        + (now.tv_nsec - (*started).tv_nsec) / 1000 as ::core::ffi::c_long;
    usleep(storage_cleanup_pace_delay_us(elapsed_us));
}
unsafe extern "C" fn delete_recording_file_and_metadata_now(
    mut recording: *const recording_metadata_t,
    mut context: *const ::core::ffi::c_char,
    mut freed_bytes: *mut uint64_t,
) -> bool {
    let mut removed: uint64_t = 0 as uint64_t;
    if !freed_bytes.is_null() {
        *freed_bytes = 0 as uint64_t;
    }
    if recording.is_null()
        || storage_recording_delete((*recording).id, context, &raw mut removed)
            != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut file_deleted: bool = removed > 0 as uint64_t;
    delete_recording_thumbnails((*recording).id);
    let mut transcode_cache_path: [::core::ffi::c_char; 512] = [0; 512];
    if build_recording_transcode_cache_path(
        &raw mut g_config.storage_path as *mut ::core::ffi::c_char,
        (*recording).id,
        &raw mut transcode_cache_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) == 0 as ::core::ffi::c_int
    {
        if unlink(&raw mut transcode_cache_path as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
            && *__errno_location() != ENOENT
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"%s: failed to delete transcode cache %s: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                context,
                &raw mut transcode_cache_path as *mut ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
    }
    if file_deleted as ::core::ffi::c_int != 0 && !freed_bytes.is_null() {
        *freed_bytes = (*recording).size_bytes;
    }
    return r#true != 0;
}
unsafe extern "C" fn delete_recording_file_and_metadata(
    mut recording: *const recording_metadata_t,
    mut context: *const ::core::ffi::c_char,
    mut freed_bytes: *mut uint64_t,
) -> bool {
    let mut started: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    cleanup_pace_begin(&raw mut started);
    let mut deleted: bool = delete_recording_file_and_metadata_now(recording, context, freed_bytes);
    cleanup_pace_end(&raw mut started);
    return deleted;
}
unsafe extern "C" fn delete_pressure_recording_file_and_metadata(
    mut recording: *const recording_metadata_t,
    mut context: *const ::core::ffi::c_char,
    mut freed_bytes: *mut uint64_t,
) -> bool {
    if recording.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: refusing NULL recording\0".as_ptr() as *const ::core::ffi::c_char,
            if !context.is_null() {
                context
            } else {
                b"Pressure cleanup\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        if !freed_bytes.is_null() {
            *freed_bytes = 0 as uint64_t;
        }
        return r#false != 0;
    }
    let mut has_identity: bool = (*recording).storage_target_uuid[0usize] as ::core::ffi::c_int
        != '\0' as ::core::ffi::c_int
        || (*recording).object_key[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
    if has_identity {
        let mut resolved: [::core::ffi::c_char; 512] = [0; 512];
        if (*recording).storage_target_uuid[0usize] as ::core::ffi::c_int
            == '\0' as ::core::ffi::c_int
            || (*recording).object_key[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            || db_storage_target_resolve_path(
                &raw const (*recording).storage_target_uuid as *const ::core::ffi::c_char,
                &raw const (*recording).object_key as *const ::core::ffi::c_char,
                &raw mut resolved as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut resolved as *mut ::core::ffi::c_char,
                &raw const (*recording).file_path as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"%s: refusing recording with inconsistent target identity (id=%llu, target=%s, object=%s, path=%s)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                if !context.is_null() {
                    context
                } else {
                    b"Pressure cleanup\0".as_ptr() as *const ::core::ffi::c_char
                },
                (*recording).id as ::core::ffi::c_ulonglong,
                &raw const (*recording).storage_target_uuid
                    as *const ::core::ffi::c_char,
                &raw const (*recording).object_key as *const ::core::ffi::c_char,
                &raw const (*recording).file_path as *const ::core::ffi::c_char,
            );
            if !freed_bytes.is_null() {
                *freed_bytes = 0 as uint64_t;
            }
            return r#false != 0;
        }
    } else {
        let mut file_info: stat = stat {
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
        let mut measured_info: stat = stat {
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
        if stat(
            &raw const (*recording).file_path as *const ::core::ffi::c_char,
            &raw mut file_info,
        ) == 0 as ::core::ffi::c_int
            && (stat(
                &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
                &raw mut measured_info,
            ) != 0 as ::core::ffi::c_int
                || file_info.st_dev != measured_info.st_dev)
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"%s: skipping unattributed recording %llu; %s is not on the filesystem being relieved (%s)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                if !context.is_null() {
                    context
                } else {
                    b"Pressure cleanup\0".as_ptr() as *const ::core::ffi::c_char
                },
                (*recording).id as ::core::ffi::c_ulonglong,
                &raw const (*recording).file_path as *const ::core::ffi::c_char,
                &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
            );
            if !freed_bytes.is_null() {
                *freed_bytes = 0 as uint64_t;
            }
            return r#false != 0;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"%s: recording %llu predates storage target attribution; deleting by path %s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            if !context.is_null() {
                context
            } else {
                b"Pressure cleanup\0".as_ptr() as *const ::core::ffi::c_char
            },
            (*recording).id as ::core::ffi::c_ulonglong,
            &raw const (*recording).file_path as *const ::core::ffi::c_char,
        );
    }
    return delete_recording_file_and_metadata(recording, context, freed_bytes);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_storage_manager(
    mut storage_path: *const ::core::ffi::c_char,
    mut max_size: uint64_t,
) -> ::core::ffi::c_int {
    if storage_path.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Storage path is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    safe_strcpy(
        &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
        storage_path,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    storage_manager.max_size = max_size;
    if mkdir_recursive(storage_path) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create storage directory: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Storage manager initialized with path: %s\0".as_ptr() as *const ::core::ffi::c_char,
        storage_path,
    );
    storage_manager.startup_recording_boundary_valid =
        db_recordings_capture_startup_boundary(&raw mut storage_manager.startup_recording_id)
            == 0 as ::core::ffi::c_int;
    if !storage_manager.startup_recording_boundary_valid {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to capture startup recording boundary; startup reconciliation will be skipped\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    storage_target_health_register(
        b"/\0".as_ptr() as *const ::core::ffi::c_char,
        storage_path,
        g_config.health.write_probe_enabled,
    );
    if start_storage_manager_thread(3600 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to start storage manager thread, automatic tasks will not be performed\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if storage_migration_worker_start() != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to start durable storage migration worker\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if storage_source_worker_start() != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Could not start archive retrieval worker\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdown_storage_manager() {
    storage_migration_worker_shutdown();
    storage_source_worker_shutdown();
    if stop_storage_manager_thread() != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to stop storage manager thread\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Storage manager shutdown\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn open_recording_file(
    mut stream_name: *const ::core::ffi::c_char,
    mut codec: *const ::core::ffi::c_char,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut fps: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Opening recording file for stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_frame_to_recording(
    mut handle: *mut ::core::ffi::c_void,
    mut data: *const uint8_t,
    mut size: size_t,
    mut timestamp: uint64_t,
    mut is_key_frame: bool,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn close_recording_file(
    mut handle: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_storage_stats(mut stats: *mut storage_stats_t) -> ::core::ffi::c_int {
    if stats.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        stats as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_stats_t>(),
    );
    let mut fs_stats: statvfs = statvfs {
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
    if statvfs(
        &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
        &raw mut fs_stats,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get filesystem statistics: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut block_size: uint64_t = fs_stats.f_frsize as uint64_t;
    (*stats).total_space = (fs_stats.f_blocks as uint64_t).wrapping_mul(block_size);
    (*stats).free_space = (fs_stats.f_bavail as uint64_t).wrapping_mul(block_size);
    (*stats).used_space = (*stats).total_space.wrapping_sub((*stats).free_space);
    (*stats).reserved_space = storage_manager.reserved_space;
    let mut dir: *mut DIR =
        opendir(&raw mut storage_manager.storage_path as *mut ::core::ffi::c_char);
    if !dir.is_null() {
        let mut entry: *const dirent = ::core::ptr::null::<dirent>();
        (*stats).total_recordings = 0 as uint64_t;
        (*stats).total_recording_bytes = 0 as uint64_t;
        (*stats).oldest_recording_time = UINT64_MAX as uint64_t;
        (*stats).newest_recording_time = 0 as uint64_t;
        loop {
            entry = readdir(dir);
            if entry.is_null() {
                break;
            }
            if strcmp(
                &raw const (*entry).d_name as *const ::core::ffi::c_char,
                b".\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw const (*entry).d_name as *const ::core::ffi::c_char,
                    b"..\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                continue;
            }
            let mut path: [::core::ffi::c_char; 768] = [0; 768];
            snprintf(
                &raw mut path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 768]>(),
                b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
                &raw const (*entry).d_name as *const ::core::ffi::c_char,
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
            if stat(&raw mut path as *mut ::core::ffi::c_char, &raw mut st)
                == 0 as ::core::ffi::c_int
                && st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
            {
                let mut stream_dir: *mut DIR = opendir(&raw mut path as *mut ::core::ffi::c_char);
                if !stream_dir.is_null() {
                    let mut rec_entry: *const dirent = ::core::ptr::null::<dirent>();
                    loop {
                        rec_entry = readdir(stream_dir);
                        if rec_entry.is_null() {
                            break;
                        }
                        if strcmp(
                            &raw const (*rec_entry).d_name as *const ::core::ffi::c_char,
                            b".\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                            || strcmp(
                                &raw const (*rec_entry).d_name as *const ::core::ffi::c_char,
                                b"..\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                        {
                            continue;
                        }
                        let mut rec_path: [::core::ffi::c_char; 768] = [0; 768];
                        snprintf(
                            &raw mut rec_path as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 768]>(),
                            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut path as *mut ::core::ffi::c_char,
                            &raw const (*rec_entry).d_name as *const ::core::ffi::c_char,
                        );
                        let mut rec_st: stat = stat {
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
                        if stat(
                            &raw mut rec_path as *mut ::core::ffi::c_char,
                            &raw mut rec_st,
                        ) == 0 as ::core::ffi::c_int
                            && rec_st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
                        {
                            (*stats).total_recordings = (*stats).total_recordings.wrapping_add(1);
                            (*stats).total_recording_bytes = (*stats)
                                .total_recording_bytes
                                .wrapping_add(rec_st.st_size as uint64_t);
                            if (rec_st.st_mtim.tv_sec as uint64_t) < (*stats).oldest_recording_time
                            {
                                (*stats).oldest_recording_time = rec_st.st_mtim.tv_sec as uint64_t;
                            }
                            if rec_st.st_mtim.tv_sec as uint64_t > (*stats).newest_recording_time {
                                (*stats).newest_recording_time = rec_st.st_mtim.tv_sec as uint64_t;
                            }
                        }
                    }
                    closedir(stream_dir);
                }
            }
        }
        closedir(dir);
        if (*stats).oldest_recording_time == UINT64_MAX as uint64_t {
            (*stats).oldest_recording_time = 0 as uint64_t;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn list_recordings(
    mut stream_name: *const ::core::ffi::c_char,
    mut start_time: time_t,
    mut end_time: time_t,
    mut recordings: *mut recording_info_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_recording(
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if path.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid path for delete_recording\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Deleting recording file: %s\0".as_ptr() as *const ::core::ffi::c_char,
        path,
    );
    if unlink(path) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to delete file: %s (error: %s)\0".as_ptr() as *const ::core::ffi::c_char,
            path,
            strerror(*__errno_location()),
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Successfully deleted recording file: %s\0".as_ptr() as *const ::core::ffi::c_char,
        path,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn collect_retention_stream_names(
    mut names: *mut [::core::ffi::c_char; 256],
    mut max_count: ::core::ffi::c_int,
    mut configured_out: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut configured: ::core::ffi::c_int =
        get_all_stream_names_including_disabled(names, max_count);
    if configured < 0 as ::core::ffi::c_int {
        *configured_out = 0 as ::core::ffi::c_int;
        return -1 as ::core::ffi::c_int;
    }
    *configured_out = configured;
    if configured >= max_count {
        return configured;
    }
    let mut orphans: ::core::ffi::c_int = get_orphaned_recording_stream_names(
        names.offset(configured as isize),
        max_count - configured,
    );
    if orphans < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to list recordings of deleted streams - skipping them this cycle\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return configured;
    }
    return configured + orphans;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn apply_retention_policy() -> ::core::ffi::c_int {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Applying per-stream retention policy\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut total_deleted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut total_freed: uint64_t = 0 as uint64_t;
    let mut budget_start: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut stream_names: *mut [::core::ffi::c_char; 256] = calloc(
        MAX_STREAMS_BATCH as size_t,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    )
        as *mut [::core::ffi::c_char; 256];
    if stream_names.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate stream-name buffer for retention policy\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut configured_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream_count: ::core::ffi::c_int =
        collect_retention_stream_names(stream_names, MAX_STREAMS_BATCH, &raw mut configured_count);
    if stream_count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get stream names for retention policy\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(stream_names as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    if stream_count == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"No streams found for retention policy\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if stream_count == MAX_STREAMS_BATCH {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Stream count reached batch limit (%d) - some streams may be skipped for retention cleanup\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1024 as ::core::ffi::c_int,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Processing retention policy for %d configured streams and %d deleted streams with recordings\0"
            .as_ptr() as *const ::core::ffi::c_char,
        configured_count,
        stream_count - configured_count,
    );
    let mut batch: *mut recording_metadata_t = calloc(
        MAX_RECORDINGS_PER_STREAM as size_t,
        ::core::mem::size_of::<recording_metadata_t>(),
    ) as *mut recording_metadata_t;
    if batch.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate recording batch buffer for retention policy\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        free(stream_names as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut s: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while s < stream_count {
        if time(::core::ptr::null_mut::<time_t>()) - budget_start
            >= RETENTION_TIME_BUDGET_SEC as time_t
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Retention time budget (%d s) exceeded after processing %d/%d streams, deleted %d recordings so far - remaining streams deferred to next cycle\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                300 as ::core::ffi::c_int,
                s,
                stream_count,
                total_deleted,
            );
            break;
        } else {
            let mut stream_name: *const ::core::ffi::c_char =
                &raw mut *stream_names.offset(s as isize) as *mut ::core::ffi::c_char;
            let mut config: stream_retention_config_t = stream_retention_config_t {
                retention_days: 0,
                detection_retention_days: 0,
                max_storage_mb: 0,
            };
            if s >= configured_count {
                config.retention_days = -1 as ::core::ffi::c_int;
                config.detection_retention_days = -1 as ::core::ffi::c_int;
                config.max_storage_mb = 0 as uint64_t;
            } else if get_stream_retention_config(stream_name, &raw mut config)
                != 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to get retention config for stream %s, using defaults\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_name,
                );
                config.retention_days = storage_manager.retention_days;
                config.detection_retention_days = config.retention_days;
                config.max_storage_mb = 0 as uint64_t;
            }
            if config.retention_days < 0 as ::core::ffi::c_int {
                config.retention_days = storage_manager.retention_days;
            }
            if config.detection_retention_days < 0 as ::core::ffi::c_int {
                config.detection_retention_days = storage_manager.retention_days;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Stream %s: effective retention=%d days, detection_retention=%d days, max_storage=%lu MB\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                stream_name,
                config.retention_days,
                config.detection_retention_days,
                config.max_storage_mb as ::core::ffi::c_ulong,
            );
            if config.retention_days <= 0 as ::core::ffi::c_int
                && config.detection_retention_days <= 0 as ::core::ffi::c_int
                && config.max_storage_mb == 0 as uint64_t
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_DEBUG,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"No retention policy for stream %s, skipping\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_name,
                );
            } else {
                if config.retention_days > 0 as ::core::ffi::c_int
                    || config.detection_retention_days > 0 as ::core::ffi::c_int
                {
                    let mut stream_deleted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut count: ::core::ffi::c_int = 0;
                    while time(::core::ptr::null_mut::<time_t>()) - budget_start
                        < RETENTION_TIME_BUDGET_SEC as time_t
                    {
                        count = get_recordings_for_retention(
                            stream_name,
                            config.retention_days,
                            config.detection_retention_days,
                            batch,
                            MAX_RECORDINGS_PER_STREAM,
                        );
                        if count > 0 as ::core::ffi::c_int {
                            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i < count {
                                let mut freed_bytes: uint64_t = 0 as uint64_t;
                                if delete_recording_file_and_metadata(
                                    batch.offset(i as isize),
                                    b"Retention cleanup\0".as_ptr() as *const ::core::ffi::c_char,
                                    &raw mut freed_bytes,
                                ) {
                                    total_freed = total_freed.wrapping_add(freed_bytes);
                                    total_deleted += 1;
                                    stream_deleted += 1;
                                }
                                i += 1;
                            }
                        }
                        if count != MAX_RECORDINGS_PER_STREAM {
                            break;
                        }
                    }
                    if stream_deleted > 0 as ::core::ffi::c_int {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Stream %s: deleted %d recordings past retention\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            stream_name,
                            stream_deleted,
                        );
                    }
                }
                if config.detection_retention_days > 0 as ::core::ffi::c_int {
                    let mut detection_max_age: uint64_t =
                        (config.detection_retention_days as uint64_t as ::core::ffi::c_ulonglong)
                            .wrapping_mul(86400 as ::core::ffi::c_ulonglong)
                            as uint64_t;
                    let mut detections_deleted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut pruned: ::core::ffi::c_int = 0;
                    while time(::core::ptr::null_mut::<time_t>()) - budget_start
                        < RETENTION_TIME_BUDGET_SEC as time_t
                    {
                        let mut prune_started: timespec = timespec {
                            tv_sec: 0,
                            tv_nsec: 0,
                        };
                        cleanup_pace_begin(&raw mut prune_started);
                        pruned = delete_old_detections_for_stream(
                            stream_name,
                            detection_max_age,
                            MAX_DETECTIONS_PER_BATCH,
                        );
                        cleanup_pace_end(&raw mut prune_started);
                        if pruned > 0 as ::core::ffi::c_int {
                            detections_deleted += pruned;
                        }
                        if pruned != MAX_DETECTIONS_PER_BATCH {
                            break;
                        }
                    }
                    if detections_deleted > 0 as ::core::ffi::c_int {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Stream %s: deleted %d detections past %d-day retention\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            stream_name,
                            detections_deleted,
                            config.detection_retention_days,
                        );
                    }
                }
                if config.max_storage_mb > 0 as uint64_t {
                    let mut current_usage: uint64_t =
                        get_stream_storage_bytes(stream_name) as uint64_t;
                    let mut max_bytes: uint64_t = config
                        .max_storage_mb
                        .wrapping_mul(1024 as uint64_t)
                        .wrapping_mul(1024 as uint64_t);
                    if current_usage > max_bytes {
                        let mut to_free: uint64_t = current_usage.wrapping_sub(max_bytes);
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Stream %s: over quota by %lu bytes, need to free space\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            stream_name,
                            to_free as ::core::ffi::c_ulong,
                        );
                        let mut freed: uint64_t = 0 as uint64_t;
                        let mut count_0: ::core::ffi::c_int = 0;
                        while time(::core::ptr::null_mut::<time_t>()) - budget_start
                            < RETENTION_TIME_BUDGET_SEC as time_t
                        {
                            count_0 = get_recordings_for_quota_enforcement(
                                stream_name,
                                batch,
                                MAX_RECORDINGS_PER_STREAM,
                            );
                            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_0 < count_0 && freed < to_free {
                                let mut freed_bytes_0: uint64_t = 0 as uint64_t;
                                if delete_recording_file_and_metadata(
                                    batch.offset(i_0 as isize),
                                    b"Quota cleanup\0".as_ptr() as *const ::core::ffi::c_char,
                                    &raw mut freed_bytes_0,
                                ) {
                                    freed = freed.wrapping_add(freed_bytes_0);
                                    total_freed = total_freed.wrapping_add(freed_bytes_0);
                                    total_deleted += 1;
                                }
                                i_0 += 1;
                            }
                            if !(count_0 == MAX_RECORDINGS_PER_STREAM && freed < to_free) {
                                break;
                            }
                        }
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Stream %s: freed %lu bytes for quota enforcement\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            stream_name,
                            freed as ::core::ffi::c_ulong,
                        );
                    }
                }
            }
            s += 1;
        }
    }
    free(batch as *mut ::core::ffi::c_void);
    free(stream_names as *mut ::core::ffi::c_void);
    let mut lpr_days_text: [::core::ffi::c_char; 32] = [
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
    if time(::core::ptr::null_mut::<time_t>()) - budget_start < RETENTION_TIME_BUDGET_SEC as time_t
        && db_get_system_setting(
            b"lpr_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut lpr_days_text as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as ::core::ffi::c_int,
        ) == 0 as ::core::ffi::c_int
    {
        let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lpr_days: ::core::ffi::c_long = strtol(
            &raw mut lpr_days_text as *mut ::core::ffi::c_char,
            &raw mut end,
            10 as ::core::ffi::c_int,
        );
        if end != &raw mut lpr_days_text as *mut ::core::ffi::c_char
            && *end as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            && lpr_days > 0 as ::core::ffi::c_long
            && lpr_days <= 36500 as ::core::ffi::c_long
        {
            let mut cutoff_ms: int64_t = (time(::core::ptr::null_mut::<time_t>()) as int64_t
                - lpr_days as int64_t * 86400 as int64_t)
                * 1000 as int64_t;
            let mut lpr_deleted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pruned_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            loop {
                let mut lpr_started: timespec = timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                cleanup_pace_begin(&raw mut lpr_started);
                pruned_0 = db_lpr_reads_prune(cutoff_ms, MAX_DETECTIONS_PER_BATCH);
                cleanup_pace_end(&raw mut lpr_started);
                if pruned_0 > 0 as ::core::ffi::c_int {
                    lpr_deleted += pruned_0;
                }
                if !(pruned_0 == MAX_DETECTIONS_PER_BATCH
                    && time(::core::ptr::null_mut::<time_t>()) - budget_start
                        < RETENTION_TIME_BUDGET_SEC as time_t)
                {
                    break;
                }
            }
            if lpr_deleted > 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Deleted %d protected LPR reads past %ld-day retention\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    lpr_deleted,
                    lpr_days,
                );
            }
        }
    }
    if time(::core::ptr::null_mut::<time_t>()) - budget_start < RETENTION_TIME_BUDGET_SEC as time_t
    {
        let mut storage_accessible: bool = r#false != 0;
        let mut mp4_path: [::core::ffi::c_char; 512] = [0; 512];
        snprintf(
            &raw mut mp4_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
            MP4_SUBDIR.as_ptr(),
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
        if stat(
            &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
            &raw mut st,
        ) == 0 as ::core::ffi::c_int
            && st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
            && stat(&raw mut mp4_path as *mut ::core::ffi::c_char, &raw mut st)
                == 0 as ::core::ffi::c_int
            && st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
        {
            storage_accessible = r#true != 0;
        }
        if !storage_accessible {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Storage path %s or mp4 subdirectory is not accessible - skipping orphan cleanup to prevent incorrect mass deletion\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
            );
        } else {
            let mut total_checked: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut orphaned: *mut recording_metadata_t = calloc(
                MAX_ORPHANED_BATCH as size_t,
                ::core::mem::size_of::<recording_metadata_t>(),
            )
                as *mut recording_metadata_t;
            if !orphaned.is_null() {
                let mut orphan_count: ::core::ffi::c_int =
                    get_orphaned_db_entries(orphaned, MAX_ORPHANED_BATCH, &raw mut total_checked);
                if orphan_count > 0 as ::core::ffi::c_int && total_checked > 0 as ::core::ffi::c_int
                {
                    let mut orphan_ratio: ::core::ffi::c_double = orphan_count
                        as ::core::ffi::c_double
                        / total_checked as ::core::ffi::c_double;
                    if orphan_ratio > ORPHAN_SAFETY_THRESHOLD
                        && total_checked >= MIN_RECORDINGS_FOR_THRESHOLD
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Orphan safety threshold exceeded: %d of %d checked recordings (%.0f%%) appear orphaned - this likely indicates a storage availability issue, skipping orphan cleanup to protect database integrity\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            orphan_count,
                            total_checked,
                            orphan_ratio * 100.0f64,
                        );
                    } else {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Found %d orphaned database entries (checked %d, ratio %.0f%%), cleaning up\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            orphan_count,
                            total_checked,
                            orphan_ratio * 100.0f64,
                        );
                        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_1 < orphan_count {
                            if delete_recording_metadata((*orphaned.offset(i_1 as isize)).id)
                                == 0 as ::core::ffi::c_int
                            {
                                _log_message_ctx(
                                    log_level_t::LOG_LEVEL_DEBUG,
                                    LOG_COMPONENT,
                                    _log_stream_name,
                                    b"Deleted orphaned DB entry: ID %llu, path %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*orphaned.offset(i_1 as isize)).id as ::core::ffi::c_ulonglong,
                                    &raw mut (*orphaned.offset(i_1 as isize)).file_path
                                        as *mut ::core::ffi::c_char,
                                );
                            }
                            i_1 += 1;
                        }
                    }
                }
                free(orphaned as *mut ::core::ffi::c_void);
            }
        }
    }
    if storage_manager.retention_days > 0 as ::core::ffi::c_int
        || storage_manager.max_size > 0 as uint64_t
    {
        let mut legacy_deleted: ::core::ffi::c_int = apply_legacy_retention_policy();
        if legacy_deleted > 0 as ::core::ffi::c_int {
            total_deleted += legacy_deleted;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Legacy retention policy deleted %d additional files\0".as_ptr()
                    as *const ::core::ffi::c_char,
                legacy_deleted,
            );
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Retention policy complete: deleted %d recordings, freed %lu bytes\0".as_ptr()
            as *const ::core::ffi::c_char,
        total_deleted,
        total_freed as ::core::ffi::c_ulong,
    );
    return total_deleted;
}
unsafe extern "C" fn apply_legacy_retention_policy() -> ::core::ffi::c_int {
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut cutoff_time: time_t = now - storage_manager.retention_days as time_t * 86400 as time_t;
    let mut deleted_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut dir: *mut DIR =
        opendir(&raw mut storage_manager.storage_path as *mut ::core::ffi::c_char);
    if dir.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open storage directory: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut entry: *const dirent = ::core::ptr::null::<dirent>();
    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        if strcmp(
            &raw const (*entry).d_name as *const ::core::ffi::c_char,
            b".\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw const (*entry).d_name as *const ::core::ffi::c_char,
                b"..\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            continue;
        }
        let mut stream_path: [::core::ffi::c_char; 768] = [0; 768];
        snprintf(
            &raw mut stream_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 768]>(),
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
            &raw const (*entry).d_name as *const ::core::ffi::c_char,
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
        if stat(
            &raw mut stream_path as *mut ::core::ffi::c_char,
            &raw mut st,
        ) == 0 as ::core::ffi::c_int
            && st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
        {
            let mut stream_dir: *mut DIR =
                opendir(&raw mut stream_path as *mut ::core::ffi::c_char);
            if !stream_dir.is_null() {
                let mut rec_entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
                loop {
                    rec_entry = readdir(stream_dir);
                    if rec_entry.is_null() {
                        break;
                    }
                    if strcmp(
                        &raw mut (*rec_entry).d_name as *mut ::core::ffi::c_char,
                        b".\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                        || strcmp(
                            &raw mut (*rec_entry).d_name as *mut ::core::ffi::c_char,
                            b"..\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        continue;
                    }
                    let mut dir_fd: ::core::ffi::c_int = dirfd(stream_dir);
                    if dir_fd == -1 as ::core::ffi::c_int {
                        continue;
                    }
                    let mut rec_st: stat = stat {
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
                        dir_fd,
                        &raw mut (*rec_entry).d_name as *mut ::core::ffi::c_char,
                        &raw mut rec_st,
                        AT_SYMLINK_NOFOLLOW,
                    ) == 0 as ::core::ffi::c_int
                        && rec_st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
                    {
                        let mut rec_path: [::core::ffi::c_char; 768] = [0; 768];
                        snprintf(
                            &raw mut rec_path as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 768]>(),
                            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut stream_path as *mut ::core::ffi::c_char,
                            &raw mut (*rec_entry).d_name as *mut ::core::ffi::c_char,
                        );
                        if storage_manager.retention_days > 0 as ::core::ffi::c_int
                            && rec_st.st_mtim.tv_sec < cutoff_time
                        {
                            let mut meta: recording_metadata_t = recording_metadata_t {
                                id: 0,
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
                            if get_recording_metadata_by_path(
                                &raw mut rec_path as *mut ::core::ffi::c_char,
                                &raw mut meta,
                            ) != 0 as ::core::ffi::c_int
                            {
                                if unlinkat(
                                    dir_fd,
                                    &raw mut (*rec_entry).d_name as *mut ::core::ffi::c_char,
                                    0 as ::core::ffi::c_int,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    _log_message_ctx(
                                        log_level_t::LOG_LEVEL_DEBUG,
                                        LOG_COMPONENT,
                                        _log_stream_name,
                                        b"Deleted untracked old recording: %s (age: %ld days)\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        &raw mut rec_path as *mut ::core::ffi::c_char,
                                        (now - rec_st.st_mtim.tv_sec) / 86400 as __time_t,
                                    );
                                    deleted_count += 1;
                                }
                            }
                        }
                    }
                }
                closedir(stream_dir);
            }
        }
    }
    closedir(dir);
    return deleted_count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_max_storage_size(mut max_size: uint64_t) -> ::core::ffi::c_int {
    storage_manager.max_size = max_size;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_retention_days(mut days: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if days < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    storage_manager.retention_days = days;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_storage_available() -> bool {
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
    return stat(
        &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
        &raw mut st,
    ) == 0 as ::core::ffi::c_int
        && st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_recording_path(
    mut stream_name: *const ::core::ffi::c_char,
    mut timestamp: time_t,
    mut path: *mut ::core::ffi::c_char,
    mut path_size: size_t,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_stream_directory(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if stream_name.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut encoded_name: [::core::ffi::c_char; 256] = [0; 256];
    sanitize_stream_name(
        stream_name,
        &raw mut encoded_name as *mut ::core::ffi::c_char,
        MAX_STREAM_NAME as size_t,
    );
    let mut dir_path: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut dir_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
        &raw mut encoded_name as *mut ::core::ffi::c_char,
    );
    if ensure_dir(&raw mut dir_path as *mut ::core::ffi::c_char) != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create stream directory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
pub const HEARTBEAT_INTERVAL_SEC: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const STANDARD_INTERVAL_SEC: ::core::ffi::c_int = 900 as ::core::ffi::c_int;
pub const DEEP_INTERVAL_SEC: ::core::ffi::c_int = 21600 as ::core::ffi::c_int;
pub const MAX_EMERGENCY_RECORDINGS: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
static mut unified_ctrl: C2Rust_Unnamed_1 = C2Rust_Unnamed_1 {
    thread: 0,
    running: r#false != 0,
    exited: r#true != 0,
    mutex: pthread_mutex_t {
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
    },
    cond: PTHREAD_COND_INITIALIZER,
    last_heartbeat: 0 as time_t,
    last_standard: 0 as time_t,
    last_deep: 0 as time_t,
    health: storage_health_t {
        pressure_level: disk_pressure_level_t::DISK_PRESSURE_NORMAL,
        free_space_pct: 100.0f64,
        free_space_bytes: 0 as uint64_t,
        total_space_bytes: 0 as uint64_t,
        used_space_bytes: 0 as uint64_t,
        last_check_time: 0 as time_t,
        last_cleanup_time: 0 as time_t,
        last_deep_time: 0 as time_t,
        last_cleanup_deleted: 0 as ::core::ffi::c_int,
        last_cleanup_freed: 0 as uint64_t,
    },
    force_cleanup: r#false != 0,
    force_aggressive: r#false != 0,
};
unsafe extern "C" fn evaluate_disk_pressure_level_cfg(
    mut free_pct: ::core::ffi::c_double,
) -> disk_pressure_level_t {
    let mut emerg: ::core::ffi::c_double = g_config.storage_pressure_emergency_pct;
    let mut crit: ::core::ffi::c_double = g_config.storage_pressure_critical_pct;
    let mut warn: ::core::ffi::c_double = g_config.storage_pressure_warning_pct;
    if !(emerg > 0.0f64 && crit > emerg && warn > crit) {
        return evaluate_disk_pressure_level(free_pct);
    }
    if free_pct < emerg {
        return disk_pressure_level_t::DISK_PRESSURE_EMERGENCY;
    }
    if free_pct < crit {
        return disk_pressure_level_t::DISK_PRESSURE_CRITICAL;
    }
    if free_pct < warn {
        return disk_pressure_level_t::DISK_PRESSURE_WARNING;
    }
    return disk_pressure_level_t::DISK_PRESSURE_NORMAL;
}
unsafe extern "C" fn disk_pressure_event_level(
    mut level: disk_pressure_level_t,
) -> *const ::core::ffi::c_char {
    match level {
        disk_pressure_level_t::DISK_PRESSURE_WARNING => {
            return b"warning\0".as_ptr() as *const ::core::ffi::c_char;
        }
        disk_pressure_level_t::DISK_PRESSURE_CRITICAL => {
            return b"critical\0".as_ptr() as *const ::core::ffi::c_char;
        }
        disk_pressure_level_t::DISK_PRESSURE_EMERGENCY => {
            return b"emergency\0".as_ptr() as *const ::core::ffi::c_char;
        }
        disk_pressure_level_t::DISK_PRESSURE_NORMAL => {
            return b"normal\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"normal\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn current_free_pct_at(
    mut root_path: *const ::core::ffi::c_char,
    mut free_bytes_out: *mut uint64_t,
    mut total_bytes_out: *mut uint64_t,
) -> ::core::ffi::c_double {
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
    if root_path.is_null() || statvfs(root_path, &raw mut fs) != 0 as ::core::ffi::c_int {
        return -1.0f64;
    }
    let mut total: uint64_t = (fs.f_blocks as uint64_t).wrapping_mul(fs.f_frsize as uint64_t);
    let mut avail: uint64_t = (fs.f_bavail as uint64_t).wrapping_mul(fs.f_frsize as uint64_t);
    if !free_bytes_out.is_null() {
        *free_bytes_out = avail;
    }
    if !total_bytes_out.is_null() {
        *total_bytes_out = total;
    }
    return if total > 0 as uint64_t {
        avail as ::core::ffi::c_double / total as ::core::ffi::c_double * 100.0f64
    } else {
        0.0f64
    };
}
unsafe extern "C" fn current_free_pct(
    mut free_bytes_out: *mut uint64_t,
    mut total_bytes_out: *mut uint64_t,
) -> ::core::ffi::c_double {
    return current_free_pct_at(
        &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
        free_bytes_out,
        total_bytes_out,
    );
}
unsafe extern "C" fn emergency_target_free_bytes(
    mut total_bytes: uint64_t,
    mut free_bytes: uint64_t,
) -> uint64_t {
    let mut target_pct: ::core::ffi::c_double = g_config.storage_pressure_critical_pct + 2.0f64;
    if g_config.storage_min_free_pct as ::core::ffi::c_double > target_pct {
        target_pct = g_config.storage_min_free_pct as ::core::ffi::c_double;
    }
    if target_pct <= 0.0f64 {
        target_pct = 7.0f64;
    }
    if target_pct > 50.0f64 {
        target_pct = 50.0f64;
    }
    let mut target: uint64_t =
        (total_bytes as ::core::ffi::c_double * target_pct / 100.0f64) as uint64_t;
    return if free_bytes >= target {
        0 as uint64_t
    } else {
        target
    };
}
unsafe extern "C" fn emergency_filesystem_fallback(
    mut reason: *const ::core::ffi::c_char,
) -> uint64_t {
    let mut free_bytes: uint64_t = 0 as uint64_t;
    let mut total_bytes: uint64_t = 0 as uint64_t;
    if current_free_pct(&raw mut free_bytes, &raw mut total_bytes) < 0.0f64 {
        return 0 as uint64_t;
    }
    let mut target: uint64_t = emergency_target_free_bytes(total_bytes, free_bytes);
    if target == 0 as uint64_t {
        return 0 as uint64_t;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Emergency filesystem fallback (%s): free=%llu MB, target=%llu MB\0".as_ptr()
            as *const ::core::ffi::c_char,
        if !reason.is_null() {
            reason
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        (free_bytes as ::core::ffi::c_ulonglong).wrapping_div(
            (1024 as ::core::ffi::c_ulonglong).wrapping_mul(1024 as ::core::ffi::c_ulonglong),
        ),
        (target as ::core::ffi::c_ulonglong).wrapping_div(
            (1024 as ::core::ffi::c_ulonglong).wrapping_mul(1024 as ::core::ffi::c_ulonglong),
        ),
    );
    storage_migration_worker_wake();
    return storage_source_trim(if target > free_bytes {
        target.wrapping_sub(free_bytes)
    } else {
        0 as uint64_t
    });
}
unsafe extern "C" fn heartbeat_check_disk_pressure() {
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
    if statvfs(
        &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
        &raw mut fs,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Heartbeat: failed to statvfs(%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return;
    }
    let mut total: uint64_t = (fs.f_blocks as uint64_t).wrapping_mul(fs.f_frsize as uint64_t);
    let mut avail: uint64_t = (fs.f_bavail as uint64_t).wrapping_mul(fs.f_frsize as uint64_t);
    let mut used: uint64_t = if total > avail {
        total.wrapping_sub(avail)
    } else {
        0 as uint64_t
    };
    let mut free_pct: ::core::ffi::c_double = if total > 0 as uint64_t {
        avail as ::core::ffi::c_double / total as ::core::ffi::c_double * 100.0f64
    } else {
        0.0f64
    };
    let mut new_level: disk_pressure_level_t = evaluate_disk_pressure_level_cfg(free_pct);
    pthread_mutex_lock(&raw mut unified_ctrl.mutex);
    let mut old_level: disk_pressure_level_t = unified_ctrl.health.pressure_level;
    unified_ctrl.health.pressure_level = new_level;
    unified_ctrl.health.free_space_pct = free_pct;
    unified_ctrl.health.free_space_bytes = avail;
    unified_ctrl.health.total_space_bytes = total;
    unified_ctrl.health.used_space_bytes = used;
    unified_ctrl.health.last_check_time = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
    if new_level.0 != old_level.0 {
        if new_level.0 > old_level.0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Disk pressure INCREASED: %s -> %s (%.1f%% free, %llu MB available)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                disk_pressure_level_str(old_level),
                disk_pressure_level_str(new_level),
                free_pct,
                (avail as ::core::ffi::c_ulonglong).wrapping_div(
                    (1024 as ::core::ffi::c_ulonglong)
                        .wrapping_mul(1024 as ::core::ffi::c_ulonglong),
                ),
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Disk pressure decreased: %s -> %s (%.1f%% free)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                disk_pressure_level_str(old_level),
                disk_pressure_level_str(new_level),
                free_pct,
            );
        }
        let mut event_error: [::core::ffi::c_char; 256] = [
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
        let mut used_pct: ::core::ffi::c_double = 100.0f64 - free_pct;
        let mut event_result: ::core::ffi::c_int = 0;
        if new_level.0 == disk_pressure_level_t::DISK_PRESSURE_NORMAL.0 {
            event_result = event_producer_publish_storage_recovered(
                disk_pressure_event_level(old_level),
                used_pct,
                avail,
                time(::core::ptr::null_mut::<time_t>()),
                &raw mut event_error as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            );
        } else {
            event_result = event_producer_publish_storage_pressure(
                disk_pressure_event_level(new_level),
                disk_pressure_event_level(old_level),
                used_pct,
                avail,
                time(::core::ptr::null_mut::<time_t>()),
                &raw mut event_error as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            );
        }
        if event_result != 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Could not enqueue normalized storage pressure event: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if event_error[0usize] as ::core::ffi::c_int != 0 {
                    &raw mut event_error as *mut ::core::ffi::c_char
                } else {
                    b"unknown error\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        let mut mqtt_topic: [::core::ffi::c_char; 256] = [0; 256];
        let mut mqtt_payload: [::core::ffi::c_char; 512] = [0; 512];
        snprintf(
            &raw mut mqtt_topic as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"%s/storage/pressure\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.mqtt_topic_prefix as *mut ::core::ffi::c_char,
        );
        snprintf(
            &raw mut mqtt_payload as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"{\"previous\":\"%s\",\"current\":\"%s\",\"free_pct\":%.1f,\"free_mb\":%llu,\"total_mb\":%llu}\0"
                .as_ptr() as *const ::core::ffi::c_char,
            disk_pressure_level_str(old_level),
            disk_pressure_level_str(new_level),
            free_pct,
            (avail as ::core::ffi::c_ulonglong)
                .wrapping_div(
                    (1024 as ::core::ffi::c_ulonglong)
                        .wrapping_mul(1024 as ::core::ffi::c_ulonglong),
                ),
            (total as ::core::ffi::c_ulonglong)
                .wrapping_div(
                    (1024 as ::core::ffi::c_ulonglong)
                        .wrapping_mul(1024 as ::core::ffi::c_ulonglong),
                ),
        );
        mqtt_publish_raw(
            &raw mut mqtt_topic as *mut ::core::ffi::c_char,
            &raw mut mqtt_payload as *mut ::core::ffi::c_char,
            r#true != 0,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Heartbeat: %.1f%% free (%llu MB), pressure=%s\0".as_ptr() as *const ::core::ffi::c_char,
        free_pct,
        (avail as ::core::ffi::c_ulonglong).wrapping_div(
            (1024 as ::core::ffi::c_ulonglong).wrapping_mul(1024 as ::core::ffi::c_ulonglong),
        ),
        disk_pressure_level_str(new_level),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_paths_share_filesystem(
    mut first: *const ::core::ffi::c_char,
    mut second: *const ::core::ffi::c_char,
) -> bool {
    let mut first_info: stat = stat {
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
    let mut second_info: stat = stat {
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
    if first.is_null()
        || second.is_null()
        || *first.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || *second.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    if stat(first, &raw mut first_info) != 0 as ::core::ffi::c_int
        || stat(second, &raw mut second_info) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    return first_info.st_dev == second_info.st_dev;
}
unsafe extern "C" fn default_target_holds_measured_filesystem(
    mut target: *const storage_target_t,
) -> bool {
    static mut last_warning: time_t = 0 as time_t;
    if storage_paths_share_filesystem(
        &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
        &raw const (*target).root_path as *const ::core::ffi::c_char,
    ) {
        return r#true != 0;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    if now - last_warning >= 300 as time_t {
        last_warning = now;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Pressure cleanup: default target \"%s\" (%s) is not on the filesystem being relieved (%s); evicting from it could not free space there, so its recordings are left to per-target cleanup\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw const (*target).name as *const ::core::ffi::c_char,
            &raw const (*target).root_path as *const ::core::ffi::c_char,
            &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
        );
    }
    return r#false != 0;
}
unsafe extern "C" fn get_default_pressure_candidates(
    mut recordings: *mut recording_metadata_t,
    mut max_count: ::core::ffi::c_int,
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
    if db_storage_target_get_default(&raw mut target).0
        != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Pressure cleanup: default storage target is unavailable\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !default_target_holds_measured_filesystem(&raw mut target) {
        return 0 as ::core::ffi::c_int;
    }
    return get_recordings_for_pressure_cleanup_default_target(
        &raw mut target.uuid as *mut ::core::ffi::c_char,
        recordings,
        max_count,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_cleanup_target_pressure(
    mut storage_target_uuid: *const ::core::ffi::c_char,
    mut result: *mut storage_target_cleanup_result_t,
) -> ::core::ffi::c_int {
    let mut local_result: storage_target_cleanup_result_t = storage_target_cleanup_result_t {
        initial_pressure: storage_target_pressure_t::STORAGE_TARGET_PRESSURE_UNAVAILABLE,
        target_free_bytes: 0,
        available_bytes_before: 0,
        available_bytes_after: 0,
        deleted_recordings: 0,
        freed_bytes: 0,
    };
    memset(
        &raw mut local_result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_target_cleanup_result_t>(),
    );
    local_result.initial_pressure = storage_target_pressure_t::STORAGE_TARGET_PRESSURE_UNAVAILABLE;
    if storage_target_uuid.is_null()
        || *storage_target_uuid.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
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
    let mut probe: db_storage_target_result_t =
        storage_target_probe_and_publish(storage_target_uuid, r#false != 0, &raw mut target);
    if probe.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
        || !target.enabled
        || target.mount_required as ::core::ffi::c_int != 0
            && !db_storage_target_mount_guard_active(&raw mut target)
    {
        if !result.is_null() {
            *result = local_result;
        }
        if probe.0 == db_storage_target_result_t::DB_STORAGE_TARGET_UNAVAILABLE.0
            || probe.0 == db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
        {
            return 1 as ::core::ffi::c_int;
        }
        return -1 as ::core::ffi::c_int;
    }
    let mut available: uint64_t = 0 as uint64_t;
    let mut capacity: uint64_t = 0 as uint64_t;
    if current_free_pct_at(
        &raw mut target.root_path as *mut ::core::ffi::c_char,
        &raw mut available,
        &raw mut capacity,
    ) < 0.0f64
    {
        if !result.is_null() {
            *result = local_result;
        }
        return -1 as ::core::ffi::c_int;
    }
    local_result.available_bytes_before = available;
    local_result.available_bytes_after = available;
    local_result.initial_pressure = storage_target_pressure_evaluate(
        capacity,
        available,
        target.reserve_bytes,
        target.high_watermark_pct,
    );
    if local_result.initial_pressure.0
        == storage_target_pressure_t::STORAGE_TARGET_PRESSURE_NORMAL.0
        || local_result.initial_pressure.0
            == storage_target_pressure_t::STORAGE_TARGET_PRESSURE_UNAVAILABLE.0
    {
        if !result.is_null() {
            *result = local_result;
        }
        return 1 as ::core::ffi::c_int;
    }
    local_result.target_free_bytes =
        storage_target_cleanup_goal_bytes(capacity, target.reserve_bytes, target.low_watermark_pct);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Target pressure cleanup: target=%s (%s), pressure=%s, available=%llu MB, goal=%llu MB\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw mut target.name as *mut ::core::ffi::c_char,
        &raw mut target.uuid as *mut ::core::ffi::c_char,
        storage_target_pressure_name(local_result.initial_pressure),
        (available as ::core::ffi::c_ulonglong).wrapping_div(
            (1024 as ::core::ffi::c_ulonglong).wrapping_mul(1024 as ::core::ffi::c_ulonglong),
        ),
        (local_result.target_free_bytes as ::core::ffi::c_ulonglong).wrapping_div(
            (1024 as ::core::ffi::c_ulonglong).wrapping_mul(1024 as ::core::ffi::c_ulonglong),
        ),
    );
    let mut batch: *mut recording_metadata_t = calloc(
        MAX_RECORDINGS_PER_STREAM as size_t,
        ::core::mem::size_of::<recording_metadata_t>(),
    ) as *mut recording_metadata_t;
    if batch.is_null() {
        if !result.is_null() {
            *result = local_result;
        }
        return -1 as ::core::ffi::c_int;
    }
    while local_result.deleted_recordings < MAX_EMERGENCY_RECORDINGS
        && available < local_result.target_free_bytes
    {
        let mut remaining: ::core::ffi::c_int =
            MAX_EMERGENCY_RECORDINGS - local_result.deleted_recordings;
        let mut limit: ::core::ffi::c_int = if remaining < MAX_RECORDINGS_PER_STREAM {
            remaining
        } else {
            MAX_RECORDINGS_PER_STREAM
        };
        let mut count: ::core::ffi::c_int = get_recordings_for_pressure_cleanup_target(
            &raw mut target.uuid as *mut ::core::ffi::c_char,
            batch,
            limit,
        );
        if count <= 0 as ::core::ffi::c_int {
            break;
        }
        let mut deleted_this_batch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while index < count {
            let mut freed: uint64_t = 0 as uint64_t;
            if delete_pressure_recording_file_and_metadata(
                batch.offset(index as isize),
                b"Target pressure cleanup\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut freed,
            ) {
                local_result.deleted_recordings += 1;
                local_result.freed_bytes = local_result.freed_bytes.wrapping_add(freed);
                deleted_this_batch += 1;
            }
            if local_result.deleted_recordings % 16 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && current_free_pct_at(
                    &raw mut target.root_path as *mut ::core::ffi::c_char,
                    &raw mut available,
                    ::core::ptr::null_mut::<uint64_t>(),
                ) >= 0.0f64
                && available >= local_result.target_free_bytes
            {
                break;
            }
            index += 1;
        }
        if deleted_this_batch == 0 as ::core::ffi::c_int || count < limit {
            break;
        }
        memset(
            batch as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (MAX_RECORDINGS_PER_STREAM as size_t)
                .wrapping_mul(::core::mem::size_of::<recording_metadata_t>()),
        );
        if current_free_pct_at(
            &raw mut target.root_path as *mut ::core::ffi::c_char,
            &raw mut available,
            ::core::ptr::null_mut::<uint64_t>(),
        ) < 0.0f64
        {
            break;
        }
    }
    free(batch as *mut ::core::ffi::c_void);
    current_free_pct_at(
        &raw mut target.root_path as *mut ::core::ffi::c_char,
        &raw mut available,
        ::core::ptr::null_mut::<uint64_t>(),
    );
    local_result.available_bytes_after = available;
    storage_target_probe_and_publish(
        &raw mut target.uuid as *mut ::core::ffi::c_char,
        r#false != 0,
        ::core::ptr::null_mut::<storage_target_t>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Target pressure cleanup complete: target=%s, deleted=%d, freed=%llu MB, remaining=%llu MB\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw mut target.name as *mut ::core::ffi::c_char,
        local_result.deleted_recordings,
        (local_result.freed_bytes as ::core::ffi::c_ulonglong)
            .wrapping_div(
                (1024 as ::core::ffi::c_ulonglong)
                    .wrapping_mul(1024 as ::core::ffi::c_ulonglong),
            ),
        (available as ::core::ffi::c_ulonglong)
            .wrapping_div(
                (1024 as ::core::ffi::c_ulonglong)
                    .wrapping_mul(1024 as ::core::ffi::c_ulonglong),
            ),
    );
    if !result.is_null() {
        *result = local_result;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn cleanup_pressured_targets() {
    if get_disk_pressure_level().0 >= disk_pressure_level_t::DISK_PRESSURE_WARNING.0 {
        storage_source_trim(
            (256 as ::core::ffi::c_ulonglong)
                .wrapping_mul(1024 as ::core::ffi::c_ulonglong)
                .wrapping_mul(1024 as ::core::ffi::c_ulonglong) as uint64_t,
        );
        storage_migration_worker_wake();
    }
    let mut total: ::core::ffi::c_int = db_storage_target_count();
    if total <= 0 as ::core::ffi::c_int || total > STORAGE_TARGET_MAX_COUNT {
        return;
    }
    let mut targets: *mut storage_target_t =
        calloc(total as size_t, ::core::mem::size_of::<storage_target_t>())
            as *mut storage_target_t;
    if targets.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate storage target list for pressure cleanup\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int = db_storage_target_list(targets, total);
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        if (*targets.offset(index as isize)).enabled {
            let mut result: storage_target_cleanup_result_t = storage_target_cleanup_result_t {
                initial_pressure: storage_target_pressure_t::STORAGE_TARGET_PRESSURE_UNAVAILABLE,
                target_free_bytes: 0,
                available_bytes_before: 0,
                available_bytes_after: 0,
                deleted_recordings: 0,
                freed_bytes: 0,
            };
            storage_cleanup_target_pressure(
                &raw mut (*targets.offset(index as isize)).uuid as *mut ::core::ffi::c_char,
                &raw mut result,
            );
        }
        index += 1;
    }
    free(targets as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn emergency_cleanup(mut aggressive: bool) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Emergency cleanup triggered (aggressive=%s)\0".as_ptr() as *const ::core::ffi::c_char,
        if aggressive as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut max_to_delete: ::core::ffi::c_int = if aggressive as ::core::ffi::c_int != 0 {
        MAX_EMERGENCY_RECORDINGS
    } else {
        MAX_EMERGENCY_RECORDINGS / 2 as ::core::ffi::c_int
    };
    let mut initial_pressure: disk_pressure_level_t = get_disk_pressure_level();
    let mut recordings: *mut recording_metadata_t = calloc(
        max_to_delete as size_t,
        ::core::mem::size_of::<recording_metadata_t>(),
    ) as *mut recording_metadata_t;
    if recordings.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Emergency cleanup: failed to allocate recording buffer\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int = get_default_pressure_candidates(recordings, max_to_delete);
    if count <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Emergency cleanup: no eligible DB recordings (count=%d), using filesystem fallback\0"
                .as_ptr() as *const ::core::ffi::c_char,
            count,
        );
        free(recordings as *mut ::core::ffi::c_void);
        let mut fb_freed: uint64_t = emergency_filesystem_fallback(
            b"no eligible DB rows\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_lock(&raw mut unified_ctrl.mutex);
        unified_ctrl.health.last_cleanup_freed = fb_freed;
        unified_ctrl.health.last_cleanup_time = time(::core::ptr::null_mut::<time_t>());
        pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
        heartbeat_check_disk_pressure();
        return;
    }
    let mut deleted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut freed: uint64_t = 0 as uint64_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if !::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.running) {
            break;
        }
        let mut freed_bytes: uint64_t = 0 as uint64_t;
        if delete_pressure_recording_file_and_metadata(
            recordings.offset(i as isize),
            b"Emergency cleanup\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut freed_bytes,
        ) {
            freed = freed.wrapping_add(freed_bytes);
            deleted += 1;
            if freed_bytes > 0 as uint64_t {
                heartbeat_check_disk_pressure();
                let mut current_pressure: disk_pressure_level_t = get_disk_pressure_level();
                if !should_continue_emergency_cleanup(
                    initial_pressure,
                    current_pressure,
                    aggressive,
                ) {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Emergency cleanup stopping after pressure recovered to %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        disk_pressure_level_str(current_pressure),
                    );
                    break;
                }
            }
        }
        i += 1;
    }
    free(recordings as *mut ::core::ffi::c_void);
    pthread_mutex_lock(&raw mut unified_ctrl.mutex);
    unified_ctrl.health.last_cleanup_deleted = deleted;
    unified_ctrl.health.last_cleanup_freed = freed;
    unified_ctrl.health.last_cleanup_time = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Emergency cleanup complete: deleted %d recordings, freed %llu MB\0".as_ptr()
            as *const ::core::ffi::c_char,
        deleted,
        (freed as ::core::ffi::c_ulonglong).wrapping_div(
            (1024 as ::core::ffi::c_ulonglong).wrapping_mul(1024 as ::core::ffi::c_ulonglong),
        ),
    );
    heartbeat_check_disk_pressure();
    if get_disk_pressure_level().0 >= disk_pressure_level_t::DISK_PRESSURE_EMERGENCY.0 {
        let mut fb_freed_0: uint64_t = emergency_filesystem_fallback(
            b"pressure persisted after DB cleanup\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if fb_freed_0 > 0 as uint64_t {
            pthread_mutex_lock(&raw mut unified_ctrl.mutex);
            unified_ctrl.health.last_cleanup_freed = freed.wrapping_add(fb_freed_0);
            pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
            heartbeat_check_disk_pressure();
        }
    }
}
unsafe extern "C" fn capacity_enforce_cycle() {
    let mut min_free: ::core::ffi::c_int = g_config.storage_min_free_pct;
    if min_free <= 0 as ::core::ffi::c_int {
        return;
    }
    let mut free_bytes: uint64_t = 0 as uint64_t;
    let mut total_bytes: uint64_t = 0 as uint64_t;
    let mut free_pct: ::core::ffi::c_double =
        current_free_pct(&raw mut free_bytes, &raw mut total_bytes);
    if free_pct < 0.0f64 || total_bytes == 0 as uint64_t {
        return;
    }
    if free_pct >= min_free as ::core::ffi::c_double {
        return;
    }
    let mut target_free: uint64_t = (total_bytes as ::core::ffi::c_double
        * min_free as ::core::ffi::c_double
        / 100.0f64) as uint64_t;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Capacity enforcement: %.1f%% free is below %d%% target, evicting oldest eligible recordings\0"
            .as_ptr() as *const ::core::ffi::c_char,
        free_pct,
        min_free,
    );
    let mut total_deleted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut total_freed: uint64_t = 0 as uint64_t;
    let mut batch: *mut recording_metadata_t = calloc(
        MAX_RECORDINGS_PER_STREAM as size_t,
        ::core::mem::size_of::<recording_metadata_t>(),
    ) as *mut recording_metadata_t;
    if !batch.is_null() {
        let mut count: ::core::ffi::c_int = 0;
        while ::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.running) {
            let mut fb: uint64_t = 0 as uint64_t;
            if current_free_pct(&raw mut fb, ::core::ptr::null_mut::<uint64_t>()) >= 0.0f64
                && fb >= target_free
            {
                break;
            }
            count = get_default_pressure_candidates(batch, MAX_RECORDINGS_PER_STREAM);
            let mut deleted_this_batch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < count
                && ::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.running)
                    as ::core::ffi::c_int
                    != 0
            {
                let mut freed_bytes: uint64_t = 0 as uint64_t;
                if delete_pressure_recording_file_and_metadata(
                    batch.offset(i as isize),
                    b"Capacity enforcement\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut freed_bytes,
                ) {
                    total_freed = total_freed.wrapping_add(freed_bytes);
                    total_deleted += 1;
                    deleted_this_batch += 1;
                }
                if total_deleted % 32 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    let mut fb2: uint64_t = 0 as uint64_t;
                    if current_free_pct(&raw mut fb2, ::core::ptr::null_mut::<uint64_t>()) >= 0.0f64
                        && fb2 >= target_free
                    {
                        count = 0 as ::core::ffi::c_int;
                        break;
                    }
                }
                i += 1;
            }
            if deleted_this_batch == 0 as ::core::ffi::c_int {
                count = 0 as ::core::ffi::c_int;
            }
            if count != MAX_RECORDINGS_PER_STREAM {
                break;
            }
        }
        free(batch as *mut ::core::ffi::c_void);
    }
    let mut fb_0: uint64_t = 0 as uint64_t;
    if current_free_pct(&raw mut fb_0, ::core::ptr::null_mut::<uint64_t>()) >= 0.0f64
        && fb_0 < target_free
    {
        total_freed = total_freed.wrapping_add(storage_source_trim(target_free.wrapping_sub(fb_0)));
    }
    heartbeat_check_disk_pressure();
    if total_deleted > 0 as ::core::ffi::c_int || total_freed > 0 as uint64_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Capacity enforcement complete: deleted %d recordings, freed %llu MB\0".as_ptr()
                as *const ::core::ffi::c_char,
            total_deleted,
            (total_freed as ::core::ffi::c_ulonglong).wrapping_div(
                (1024 as ::core::ffi::c_ulonglong).wrapping_mul(1024 as ::core::ffi::c_ulonglong),
            ),
        );
    }
}
unsafe extern "C" fn standard_cleanup_cycle() {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Standard cleanup cycle starting\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut cycle_start: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut deleted: ::core::ffi::c_int = apply_retention_policy();
    if deleted > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Standard cleanup: retention policy deleted %d recordings\0".as_ptr()
                as *const ::core::ffi::c_char,
            deleted,
        );
    } else if deleted < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Standard cleanup: retention policy error\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut tier_deleted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tier_freed: uint64_t = 0 as uint64_t;
    let mut tier_recs: *mut recording_metadata_t = calloc(
        MAX_RECORDINGS_PER_STREAM as size_t,
        ::core::mem::size_of::<recording_metadata_t>(),
    ) as *mut recording_metadata_t;
    if !tier_recs.is_null() {
        let mut stream_names: *mut [::core::ffi::c_char; 256] = calloc(
            MAX_STREAMS_BATCH as size_t,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        )
            as *mut [::core::ffi::c_char; 256];
        if stream_names.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Tiered cleanup: failed to allocate stream-name buffer\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            free(tier_recs as *mut ::core::ffi::c_void);
            tier_recs = ::core::ptr::null_mut::<recording_metadata_t>();
        }
        let mut configured_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut stream_count: ::core::ffi::c_int = if !stream_names.is_null() {
            collect_retention_stream_names(
                stream_names,
                MAX_STREAMS_BATCH,
                &raw mut configured_count,
            )
        } else {
            -1 as ::core::ffi::c_int
        };
        if stream_count == MAX_STREAMS_BATCH {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Tiered cleanup: stream count reached batch limit (%d) - some streams may be skipped\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1024 as ::core::ffi::c_int,
            );
        }
        let mut s: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while s < stream_count
            && ::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.running)
                as ::core::ffi::c_int
                != 0
        {
            let mut sconfig: stream_config_t = stream_config_t {
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
            's_81: {
                if s >= configured_count {
                    memset(
                        &raw mut sconfig as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<stream_config_t>(),
                    );
                    sconfig.retention_days = -1 as ::core::ffi::c_int;
                    sconfig.tier_critical_multiplier = 3.0f64;
                    sconfig.tier_important_multiplier = 2.0f64;
                    sconfig.tier_ephemeral_multiplier = 0.25f64;
                } else if get_stream_config_by_name(
                    &raw mut *stream_names.offset(s as isize) as *mut ::core::ffi::c_char,
                    &raw mut sconfig,
                ) != 0 as ::core::ffi::c_int
                {
                    break 's_81;
                }
                let mut base_retention: ::core::ffi::c_int =
                    if sconfig.retention_days < 0 as ::core::ffi::c_int {
                        storage_manager.retention_days
                    } else {
                        sconfig.retention_days
                    };
                if base_retention != 0 as ::core::ffi::c_int {
                    if base_retention < 0 as ::core::ffi::c_int {
                        base_retention = DEFAULT_RETENTION_DAYS;
                    }
                    let tier_mults: [::core::ffi::c_double; 4] = [
                        if sconfig.tier_critical_multiplier > 0.0f64 {
                            sconfig.tier_critical_multiplier
                        } else {
                            3.0f64
                        },
                        if sconfig.tier_important_multiplier > 0.0f64 {
                            sconfig.tier_important_multiplier
                        } else {
                            2.0f64
                        },
                        1.0f64,
                        if sconfig.tier_ephemeral_multiplier > 0.0f64 {
                            sconfig.tier_ephemeral_multiplier
                        } else {
                            0.25f64
                        },
                    ];
                    if sconfig.tier_critical_multiplier <= 0.0f64
                        || sconfig.tier_important_multiplier <= 0.0f64
                        || sconfig.tier_ephemeral_multiplier <= 0.0f64
                    {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_WARN,
                            LOG_COMPONENT,
                            _log_stream_name,
                            b"Stream '%s' has zero tier multipliers in database \xE2\x80\x93 using defaults (3.0/2.0/0.25). Update the stream via the API to persist correct values.\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            &raw mut *stream_names.offset(s as isize)
                                as *mut ::core::ffi::c_char,
                        );
                    }
                    let mut count: ::core::ffi::c_int = 0;
                    while ::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.running) {
                        count = get_recordings_for_tiered_retention(
                            &raw mut *stream_names.offset(s as isize) as *mut ::core::ffi::c_char,
                            base_retention,
                            &raw const tier_mults as *const ::core::ffi::c_double,
                            tier_recs,
                            MAX_RECORDINGS_PER_STREAM,
                        );
                        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i < count
                            && ::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.running)
                                as ::core::ffi::c_int
                                != 0
                        {
                            let mut freed_bytes: uint64_t = 0 as uint64_t;
                            if delete_recording_file_and_metadata(
                                tier_recs.offset(i as isize),
                                b"Tiered cleanup\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut freed_bytes,
                            ) {
                                tier_freed = tier_freed.wrapping_add(freed_bytes);
                                tier_deleted += 1;
                            }
                            i += 1;
                        }
                        if count != MAX_RECORDINGS_PER_STREAM {
                            break;
                        }
                    }
                }
            }
            s += 1;
        }
        free(stream_names as *mut ::core::ffi::c_void);
        free(tier_recs as *mut ::core::ffi::c_void);
    }
    if tier_deleted > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Standard cleanup: tiered retention deleted %d recordings, freed %llu MB\0".as_ptr()
                as *const ::core::ffi::c_char,
            tier_deleted,
            (tier_freed as ::core::ffi::c_ulonglong).wrapping_div(
                (1024 as ::core::ffi::c_ulonglong).wrapping_mul(1024 as ::core::ffi::c_ulonglong),
            ),
        );
    }
    capacity_enforce_cycle();
    if force_refresh_cache() == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Standard cleanup: cache refresh successful\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Standard cleanup: cache refresh failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    pthread_mutex_lock(&raw mut unified_ctrl.mutex);
    let mut pressure: disk_pressure_level_t = unified_ctrl.health.pressure_level;
    unified_ctrl.health.last_cleanup_deleted = deleted + tier_deleted;
    unified_ctrl.health.last_cleanup_freed = tier_freed;
    unified_ctrl.health.last_cleanup_time = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
    if pressure.0 >= disk_pressure_level_t::DISK_PRESSURE_CRITICAL.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Standard cleanup: pressure is %s, triggering emergency cleanup\0".as_ptr()
                as *const ::core::ffi::c_char,
            disk_pressure_level_str(pressure),
        );
        emergency_cleanup(pressure.0 == disk_pressure_level_t::DISK_PRESSURE_EMERGENCY.0);
    }
    let mut elapsed: time_t = time(::core::ptr::null_mut::<time_t>()) - cycle_start;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Standard cleanup cycle complete in %ld seconds (deleted=%d, tier_deleted=%d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        elapsed,
        deleted,
        tier_deleted,
    );
    if deleted + tier_deleted > 0 as ::core::ffi::c_int {
        let mut mqtt_topic: [::core::ffi::c_char; 256] = [0; 256];
        let mut mqtt_payload: [::core::ffi::c_char; 512] = [0; 512];
        snprintf(
            &raw mut mqtt_topic as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"%s/storage/cleanup\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut g_config.mqtt_topic_prefix as *mut ::core::ffi::c_char,
        );
        snprintf(
            &raw mut mqtt_payload as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"{\"deleted\":%d,\"tier_deleted\":%d,\"freed_bytes\":%llu,\"elapsed_sec\":%ld,\"pressure\":\"%s\"}\0"
                .as_ptr() as *const ::core::ffi::c_char,
            deleted,
            tier_deleted,
            tier_freed as ::core::ffi::c_ulonglong,
            elapsed,
            disk_pressure_level_str(pressure),
        );
        mqtt_publish_raw(
            &raw mut mqtt_topic as *mut ::core::ffi::c_char,
            &raw mut mqtt_payload as *mut ::core::ffi::c_char,
            r#false != 0,
        );
    }
}
unsafe extern "C" fn deep_maintenance_cycle() {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Deep maintenance cycle starting\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut sessions_deleted: ::core::ffi::c_int = db_auth_cleanup_sessions();
    if sessions_deleted > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Deep maintenance: cleaned up %d expired sessions\0".as_ptr()
                as *const ::core::ffi::c_char,
            sessions_deleted,
        );
    } else if sessions_deleted < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Deep maintenance: session cleanup error\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut ledger_pruned: ::core::ffi::c_int = storage_deletion_prune_completed(
        STORAGE_DELETION_PRUNE_DEFAULT_DAYS,
        STORAGE_DELETION_PRUNE_DEFAULT_ROWS,
    );
    if ledger_pruned > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Deep maintenance: pruned %d completed deletion ledger entries\0".as_ptr()
                as *const ::core::ffi::c_char,
            ledger_pruned,
        );
    } else if ledger_pruned < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Deep maintenance: deletion ledger prune error\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    standard_cleanup_cycle();
    pthread_mutex_lock(&raw mut unified_ctrl.mutex);
    unified_ctrl.health.last_deep_time = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Deep maintenance cycle complete\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn unified_storage_controller_func(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    log_set_thread_context(
        b"StorageManager\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Unified storage controller started\0".as_ptr() as *const ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"  Heartbeat interval: %d seconds\0".as_ptr() as *const ::core::ffi::c_char,
        60 as ::core::ffi::c_int,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"  Standard cleanup interval: %d seconds\0".as_ptr() as *const ::core::ffi::c_char,
        900 as ::core::ffi::c_int,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"  Deep maintenance interval: %d seconds\0".as_ptr() as *const ::core::ffi::c_char,
        21600 as ::core::ffi::c_int,
    );
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    unified_ctrl.last_heartbeat = now;
    unified_ctrl.last_standard = now;
    unified_ctrl.last_deep = now;
    if force_refresh_cache() == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Initial cache refresh successful\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Initial cache refresh failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cleanup_stale_hls_segments();
    heartbeat_check_disk_pressure();
    if storage_target_refresh_health_and_publish() < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Initial storage target health refresh failed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    cleanup_pressured_targets();
    if storage_manager.startup_recording_boundary_valid {
        if storage_reconcile_startup_recordings(storage_manager.startup_recording_id)
            < 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Startup recording reconciliation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    if ::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.running) {
        capacity_enforce_cycle();
    }
    while ::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.running) {
        pthread_mutex_lock(&raw mut unified_ctrl.mutex);
        if !::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.force_cleanup) {
            let mut ts: timespec = timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            clock_gettime(CLOCK_REALTIME, &raw mut ts);
            ts.tv_sec += HEARTBEAT_INTERVAL_SEC as __time_t;
            pthread_cond_timedwait(
                &raw mut unified_ctrl.cond,
                &raw mut unified_ctrl.mutex,
                &raw mut ts,
            );
        }
        let mut do_force: bool =
            ::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.force_cleanup);
        let mut do_aggressive: bool =
            ::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.force_aggressive);
        ::core::ptr::write_volatile(&raw mut unified_ctrl.force_cleanup, r#false != 0);
        ::core::ptr::write_volatile(&raw mut unified_ctrl.force_aggressive, r#false != 0);
        pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
        if !::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.running) {
            break;
        }
        now = time(::core::ptr::null_mut::<time_t>());
        cleanup_stale_hls_segments();
        heartbeat_check_disk_pressure();
        if storage_target_refresh_health_and_publish() < 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Storage target health refresh failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        cleanup_pressured_targets();
        observation_retention_maintenance_tick();
        unified_ctrl.last_heartbeat = now;
        if do_force {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Forced cleanup requested (aggressive=%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if do_aggressive as ::core::ffi::c_int != 0 {
                    b"true\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"false\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            if do_aggressive {
                emergency_cleanup(r#true != 0);
            } else {
                standard_cleanup_cycle();
            }
        } else {
            pthread_mutex_lock(&raw mut unified_ctrl.mutex);
            let mut pressure: disk_pressure_level_t = unified_ctrl.health.pressure_level;
            pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
            if pressure.0 >= disk_pressure_level_t::DISK_PRESSURE_EMERGENCY.0 {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Emergency pressure detected in heartbeat, triggering emergency cleanup\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                emergency_cleanup(r#true != 0);
            } else if pressure.0 >= disk_pressure_level_t::DISK_PRESSURE_CRITICAL.0 {
                if now - unified_ctrl.last_standard
                    >= (STANDARD_INTERVAL_SEC / 3 as ::core::ffi::c_int) as time_t
                {
                    standard_cleanup_cycle();
                    unified_ctrl.last_standard = now;
                }
            }
            if now - unified_ctrl.last_standard >= STANDARD_INTERVAL_SEC as time_t {
                standard_cleanup_cycle();
                unified_ctrl.last_standard = now;
            }
            if now - unified_ctrl.last_deep >= DEEP_INTERVAL_SEC as time_t {
                deep_maintenance_cycle();
                unified_ctrl.last_deep = now;
            }
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Unified storage controller exiting\0".as_ptr() as *const ::core::ffi::c_char,
    );
    ::core::ptr::write_volatile(&raw mut unified_ctrl.exited, r#true != 0);
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ensure_disk_space(mut min_free_bytes: uint64_t) -> bool {
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
    if statvfs(
        &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
        &raw mut fs,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"ensure_disk_space: statvfs failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return r#false != 0;
    }
    let mut avail: uint64_t = (fs.f_bavail as uint64_t).wrapping_mul(fs.f_frsize as uint64_t);
    return avail >= min_free_bytes;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_storage_manager_thread(
    mut interval_seconds: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut unified_ctrl.mutex);
    if ::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.running) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Storage controller thread is already running\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
        return 0 as ::core::ffi::c_int;
    }
    ::core::ptr::write_volatile(&raw mut unified_ctrl.running, r#true != 0);
    ::core::ptr::write_volatile(&raw mut unified_ctrl.exited, r#false != 0);
    if pthread_create(
        &raw mut unified_ctrl.thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            unified_storage_controller_func
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create storage controller thread: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        ::core::ptr::write_volatile(&raw mut unified_ctrl.running, r#false != 0);
        pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Unified storage controller thread started (heartbeat=%ds, standard=%ds, deep=%ds)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        60 as ::core::ffi::c_int,
        900 as ::core::ffi::c_int,
        21600 as ::core::ffi::c_int,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_storage_manager_thread() -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut unified_ctrl.mutex);
    if !::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.running)
        && ::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.exited) as ::core::ffi::c_int
            != 0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Storage controller thread is not running\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
        return 0 as ::core::ffi::c_int;
    }
    ::core::ptr::write_volatile(&raw mut unified_ctrl.running, r#false != 0);
    pthread_cond_broadcast(&raw mut unified_ctrl.cond);
    pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Waiting for storage controller thread to exit...\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut timeout_ms: ::core::ffi::c_int = 5000 as ::core::ffi::c_int;
    let mut elapsed_ms: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let poll_interval_ms: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
    while elapsed_ms < timeout_ms {
        if ::core::ptr::read_volatile::<bool>(&raw const unified_ctrl.exited) {
            pthread_join(
                unified_ctrl.thread,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Storage controller thread stopped successfully\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
        usleep((poll_interval_ms * 1000 as ::core::ffi::c_int) as __useconds_t);
        elapsed_ms += poll_interval_ms;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Storage controller thread did not exit in time (%d ms), detaching\0".as_ptr()
            as *const ::core::ffi::c_char,
        timeout_ms,
    );
    pthread_detach(unified_ctrl.thread);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_storage_health(
    mut health: *mut storage_health_t,
) -> ::core::ffi::c_int {
    if health.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut unified_ctrl.mutex);
    *health = unified_ctrl.health;
    pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_disk_pressure_level() -> disk_pressure_level_t {
    pthread_mutex_lock(&raw mut unified_ctrl.mutex);
    let mut level: disk_pressure_level_t = unified_ctrl.health.pressure_level;
    pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
    return level;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trigger_storage_cleanup(mut force_aggressive: bool) {
    pthread_mutex_lock(&raw mut unified_ctrl.mutex);
    ::core::ptr::write_volatile(&raw mut unified_ctrl.force_cleanup, r#true != 0);
    ::core::ptr::write_volatile(&raw mut unified_ctrl.force_aggressive, force_aggressive);
    pthread_cond_broadcast(&raw mut unified_ctrl.cond);
    pthread_mutex_unlock(&raw mut unified_ctrl.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Triggered immediate storage cleanup (aggressive=%s)\0".as_ptr()
            as *const ::core::ffi::c_char,
        if force_aggressive as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_should_pause_recording() -> bool {
    return get_disk_pressure_level().0 >= disk_pressure_level_t::DISK_PRESSURE_EMERGENCY.0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_startup_reclaim_if_full(
    mut recordings_root: *const ::core::ffi::c_char,
    mut cfg: *const config_t,
) -> uint64_t {
    if recordings_root.is_null()
        || *recordings_root.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return 0 as uint64_t;
    }
    safe_strcpy(
        &raw mut storage_manager.storage_path as *mut ::core::ffi::c_char,
        recordings_root,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    let mut free_bytes: uint64_t = 0 as uint64_t;
    let mut total_bytes: uint64_t = 0 as uint64_t;
    let mut free_pct: ::core::ffi::c_double =
        current_free_pct(&raw mut free_bytes, &raw mut total_bytes);
    if free_pct < 0.0f64 || total_bytes == 0 as uint64_t {
        return 0 as uint64_t;
    }
    let mut emerg: ::core::ffi::c_double =
        if !cfg.is_null() && (*cfg).storage_pressure_emergency_pct > 0.0f64 {
            (*cfg).storage_pressure_emergency_pct
        } else {
            5.0f64
        };
    if free_pct >= emerg {
        return 0 as uint64_t;
    }
    let mut min_free: ::core::ffi::c_int =
        if !cfg.is_null() && (*cfg).storage_min_free_pct > 0 as ::core::ffi::c_int {
            (*cfg).storage_min_free_pct
        } else {
            10 as ::core::ffi::c_int
        };
    let mut target_pct: ::core::ffi::c_double = min_free as ::core::ffi::c_double;
    if target_pct < emerg + 2.0f64 {
        target_pct = emerg + 2.0f64;
    }
    if target_pct > 50.0f64 {
        target_pct = 50.0f64;
    }
    let mut target: uint64_t =
        (total_bytes as ::core::ffi::c_double * target_pct / 100.0f64) as uint64_t;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"Startup: recordings volume only %.1f%% free (%llu MB) \xE2\x80\x94 reclaiming to %.0f%% before database init to avoid a full-disk crash loop\0"
            .as_ptr() as *const ::core::ffi::c_char,
        free_pct,
        (free_bytes as ::core::ffi::c_ulonglong)
            .wrapping_div(
                (1024 as ::core::ffi::c_ulonglong)
                    .wrapping_mul(1024 as ::core::ffi::c_ulonglong),
            ),
        target_pct,
    );
    storage_migration_worker_wake();
    return storage_source_trim(if target > free_bytes {
        target.wrapping_sub(free_bytes)
    } else {
        0 as uint64_t
    });
}
unsafe extern "C" fn stale_media_target_is_available(
    mut recording: *const recording_metadata_t,
) -> bool {
    if (*recording).storage_target_uuid[0usize] as ::core::ffi::c_int != 0
        || (*recording).object_key[0usize] as ::core::ffi::c_int != 0
    {
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
        let mut resolved: [::core::ffi::c_char; 512] = [0; 512];
        if (*recording).storage_target_uuid[0usize] == 0
            || (*recording).object_key[0usize] == 0
            || db_storage_target_get(
                &raw const (*recording).storage_target_uuid as *const ::core::ffi::c_char,
                &raw mut target,
            )
            .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
            || strcmp(
                &raw mut target.target_type as *mut ::core::ffi::c_char,
                b"s3\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || !db_storage_target_mount_guard_active(&raw mut target)
            || db_storage_target_resolve_path(
                &raw const (*recording).storage_target_uuid as *const ::core::ffi::c_char,
                &raw const (*recording).object_key as *const ::core::ffi::c_char,
                &raw mut resolved as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut resolved as *mut ::core::ffi::c_char,
                &raw const (*recording).file_path as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            return r#false != 0;
        }
    }
    return r#true != 0;
}
unsafe extern "C" fn stale_media_absence_is_authoritative(
    mut recording: *const recording_metadata_t,
) -> bool {
    if (*recording).file_path[0usize] == 0 {
        return r#true != 0;
    }
    let mut directory: [::core::ffi::c_char; 512] = [0; 512];
    safe_strcpy(
        &raw mut directory as *mut ::core::ffi::c_char,
        &raw const (*recording).file_path as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    let mut slash: *mut ::core::ffi::c_char = strrchr(
        &raw mut directory as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if slash.is_null() || slash == &raw mut directory as *mut ::core::ffi::c_char {
        return r#false != 0;
    }
    *slash = '\0' as ::core::ffi::c_char;
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
    return stat(&raw mut directory as *mut ::core::ffi::c_char, &raw mut st)
        == 0 as ::core::ffi::c_int
        && st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t;
}
unsafe extern "C" fn reconcile_incomplete_recordings(
    mut startup: bool,
    mut startup_maximum_id: uint64_t,
) -> ::core::ffi::c_int {
    if startup as ::core::ffi::c_int != 0 && startup_maximum_id > INT64_MAX as uint64_t {
        return -1 as ::core::ffi::c_int;
    }
    if startup as ::core::ffi::c_int != 0 && startup_maximum_id == 0 as uint64_t {
        return 0 as ::core::ffi::c_int;
    }
    let STALE_WINDOW_SEC: ::core::ffi::c_int = 3600 as ::core::ffi::c_int;
    let RECONCILE_BATCH: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
    if get_db_handle().is_null() {
        return if startup as ::core::ffi::c_int != 0 {
            -1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
    let mut batch: *mut recording_metadata_t = calloc(
        RECONCILE_BATCH as size_t,
        ::core::mem::size_of::<recording_metadata_t>(),
    ) as *mut recording_metadata_t;
    if batch.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Reconcile: failed to allocate batch buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return if startup as ::core::ffi::c_int != 0 {
            -1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
    let mut reconciled: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut finalized: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pruned: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut retained: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut unplayable: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut count: ::core::ffi::c_int = 0;
    let mut after_id: uint64_t = 0 as uint64_t;
    let mut maximum_id: uint64_t = startup_maximum_id;
    let cutoff: time_t = time(::core::ptr::null_mut::<time_t>()) - STALE_WINDOW_SEC as time_t;
    loop {
        count = if startup as ::core::ffi::c_int != 0 {
            get_startup_incomplete_recordings_page(batch, RECONCILE_BATCH, after_id, maximum_id)
        } else {
            get_stale_incomplete_recordings_page(
                batch,
                RECONCILE_BATCH,
                cutoff,
                after_id,
                &raw mut maximum_id,
            )
        };
        if count <= 0 as ::core::ffi::c_int {
            break;
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < count {
            after_id = (*batch.offset(i as isize)).id;
            if !stale_media_target_is_available(batch.offset(i as isize)) {
                retained += 1;
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
                let mut stat_result: ::core::ffi::c_int =
                    if (*batch.offset(i as isize)).file_path[0usize] as ::core::ffi::c_int != 0 {
                        lstat(
                            &raw mut (*batch.offset(i as isize)).file_path
                                as *mut ::core::ffi::c_char,
                            &raw mut st,
                        )
                    } else {
                        -1 as ::core::ffi::c_int
                    };
                let mut stat_error: ::core::ffi::c_int =
                    if (*batch.offset(i as isize)).file_path[0usize] as ::core::ffi::c_int != 0 {
                        *__errno_location()
                    } else {
                        ENOENT
                    };
                if stat_result != 0 as ::core::ffi::c_int && stat_error != ENOENT
                    || stat_result == 0 as ::core::ffi::c_int
                        && !(st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
                {
                    retained += 1;
                } else {
                    let mut have_file: bool = stat_result == 0 as ::core::ffi::c_int;
                    if have_file as ::core::ffi::c_int != 0 && st.st_size > 0 as __off_t {
                        let mut issue: *const ::core::ffi::c_char = recording_media_probe_issue(
                            &raw mut (*batch.offset(i as isize)).file_path
                                as *mut ::core::ffi::c_char,
                        );
                        let mut end_time: time_t =
                            if (*batch.offset(i as isize)).end_time > 0 as time_t {
                                (*batch.offset(i as isize)).end_time
                            } else {
                                st.st_mtim.tv_sec
                            };
                        if end_time < (*batch.offset(i as isize)).start_time {
                            end_time = (*batch.offset(i as isize)).start_time;
                        }
                        if finalize_recording_reconciliation(
                            (*batch.offset(i as isize)).id,
                            end_time,
                            st.st_size as uint64_t,
                            issue,
                        ) == 0 as ::core::ffi::c_int
                        {
                            if *issue.offset(0isize) != 0 {
                                unplayable += 1;
                            }
                            finalized += 1;
                            reconciled += 1;
                        } else {
                            retained += 1;
                        }
                    } else if have_file as ::core::ffi::c_int != 0
                        || stale_media_absence_is_authoritative(batch.offset(i as isize))
                            as ::core::ffi::c_int
                            != 0
                    {
                        let mut deleted: ::core::ffi::c_int =
                            if (*batch.offset(i as isize)).is_complete as ::core::ffi::c_int != 0 {
                                delete_recording_metadata((*batch.offset(i as isize)).id)
                            } else {
                                delete_stale_incomplete_recording_metadata(
                                    (*batch.offset(i as isize)).id,
                                )
                            };
                        if deleted == 0 as ::core::ffi::c_int {
                            delete_recording_thumbnails((*batch.offset(i as isize)).id);
                            pruned += 1;
                            reconciled += 1;
                        } else {
                            retained += 1;
                        }
                    } else {
                        retained += 1;
                    }
                }
            }
            i += 1;
        }
        if count != RECONCILE_BATCH {
            break;
        }
    }
    free(batch as *mut ::core::ffi::c_void);
    if reconciled > 0 as ::core::ffi::c_int
        || retained > 0 as ::core::ffi::c_int
        || unplayable > 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Reconciled %d incomplete recordings on startup (%d finalized, %d pruned, %d retained)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            reconciled,
            finalized,
            pruned,
            retained,
        );
    }
    if unplayable > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Listed %d interrupted recordings with playback-check warnings; normal retention and protection rules still apply\0"
                .as_ptr() as *const ::core::ffi::c_char,
            unplayable,
        );
    }
    return if startup as ::core::ffi::c_int != 0 && count < 0 as ::core::ffi::c_int {
        -1 as ::core::ffi::c_int
    } else {
        reconciled
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_reconcile_incomplete_recordings() -> ::core::ffi::c_int {
    return reconcile_incomplete_recordings(r#false != 0, 0 as uint64_t);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_reconcile_startup_recordings(
    mut maximum_id: uint64_t,
) -> ::core::ffi::c_int {
    return reconcile_incomplete_recordings(r#true != 0, maximum_id);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
