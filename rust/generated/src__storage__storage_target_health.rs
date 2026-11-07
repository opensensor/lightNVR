use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    unsafe fn system_health_observation_set_available(
        observation: *mut system_health_observation_t,
        value: ::core::ffi::c_double,
        unit: system_health_unit_t,
    );
    unsafe fn system_health_observation_set_unavailable(
        observation: *mut system_health_observation_t,
        capability: system_health_capability_t,
    );
    unsafe fn db_storage_target_count() -> ::core::ffi::c_int;
    unsafe fn db_storage_target_list(
        targets: *mut storage_target_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_storage_target_get(
        uuid: *const ::core::ffi::c_char,
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
    unsafe fn db_storage_target_probe(
        uuid: *const ::core::ffi::c_char,
        write_test: bool,
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
    unsafe fn db_storage_target_record_health(
        uuid: *const ::core::ffi::c_char,
        update: *const storage_target_health_update_t,
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
    unsafe fn db_storage_target_recording_growth_bps(
        uuid: *const ::core::ffi::c_char,
        bytes_per_second: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    unsafe fn system_health_observation_sink_append(
        sink: *mut system_health_observation_sink_t,
        observation: *const system_health_observation_t,
    ) -> bool;
    unsafe fn linux_filesystem_normalize_errno(
        error_code: ::core::ffi::c_int,
    ) -> linux_filesystem_probe_error_t;
    unsafe fn linux_filesystem_capability_from_errno(
        error_code: ::core::ffi::c_int,
    ) -> system_health_capability_t;
    unsafe fn linux_filesystem_logical_id_valid(logical_id: *const ::core::ffi::c_char) -> bool;
    unsafe fn linux_filesystem_mountinfo_contains(
        mountinfo_path: *const ::core::ffi::c_char,
        mount_path: *const ::core::ffi::c_char,
        present: *mut bool,
    ) -> ::core::ffi::c_int;
    unsafe fn health_helper_run(
        request: *const health_helper_request_t,
        result: *mut health_helper_result_t,
    ) -> ::core::ffi::c_int;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
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
    unsafe fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __getdelim(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __delimiter: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    unsafe fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn getpid() -> __pid_t;
    unsafe fn event_producer_publish_storage_target_unavailable(
        target_uuid: *const ::core::ffi::c_char,
        previous_state: *const ::core::ffi::c_char,
        reason: *const ::core::ffi::c_char,
        is_default: bool,
        occurred_at: time_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn event_producer_publish_storage_target_recovered(
        target_uuid: *const ::core::ffi::c_char,
        current_state: *const ::core::ffi::c_char,
        downtime_ms: int64_t,
        is_default: bool,
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
    unsafe fn system_health_register_collector(collector: *const system_health_collector_t)
    -> bool;
    unsafe fn system_health_request_tier(tier: system_health_sampling_tier_t) -> bool;
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
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __ssize_t = ::core::ffi::c_long;
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
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_scope_t(pub ::core::ffi::c_uint);
impl system_health_scope_t {
    pub const SYSTEM_HEALTH_SCOPE_PROCESS: Self = Self(0);
    pub const SYSTEM_HEALTH_SCOPE_CONTAINER: Self = Self(1);
    pub const SYSTEM_HEALTH_SCOPE_HOST: Self = Self(2);
    pub const SYSTEM_HEALTH_SCOPE_FILESYSTEM: Self = Self(3);
    pub const SYSTEM_HEALTH_SCOPE_DEVICE: Self = Self(4);
    pub const SYSTEM_HEALTH_SCOPE_COUNT: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_capability_t(pub ::core::ffi::c_uint);
impl system_health_capability_t {
    pub const SYSTEM_HEALTH_CAPABILITY_AVAILABLE: Self = Self(0);
    pub const SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED: Self = Self(1);
    pub const SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED: Self = Self(2);
    pub const SYSTEM_HEALTH_CAPABILITY_STALE: Self = Self(3);
    pub const SYSTEM_HEALTH_CAPABILITY_ERROR: Self = Self(4);
    pub const SYSTEM_HEALTH_CAPABILITY_COUNT: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_freshness_t(pub ::core::ffi::c_uint);
impl system_health_freshness_t {
    pub const SYSTEM_HEALTH_FRESHNESS_UNKNOWN: Self = Self(0);
    pub const SYSTEM_HEALTH_FRESHNESS_FRESH: Self = Self(1);
    pub const SYSTEM_HEALTH_FRESHNESS_STALE: Self = Self(2);
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_observation_t {
    pub metric: [::core::ffi::c_char; 64],
    pub resource_id: [::core::ffi::c_char; 64],
    pub scope: system_health_scope_t,
    pub capability: system_health_capability_t,
    pub freshness: system_health_freshness_t,
    pub unit: system_health_unit_t,
    pub value_valid: bool,
    pub value: ::core::ffi::c_double,
    pub sampled_monotonic_ms: uint64_t,
    pub observed_wall_time_ms: int64_t,
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
pub struct storage_target_health_update_t {
    pub available: bool,
    pub write_checked: bool,
    pub writeable: bool,
    pub cleanup_failed: bool,
    pub capacity_bytes: uint64_t,
    pub available_bytes: uint64_t,
    pub filesystem_device: uint64_t,
    pub probed_at: int64_t,
    pub normalized_error: [::core::ffi::c_char; 32],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_sampling_tier_t(pub ::core::ffi::c_uint);
impl system_health_sampling_tier_t {
    pub const SYSTEM_HEALTH_TIER_FAST: Self = Self(0);
    pub const SYSTEM_HEALTH_TIER_NORMAL: Self = Self(1);
    pub const SYSTEM_HEALTH_TIER_SLOW: Self = Self(2);
    pub const SYSTEM_HEALTH_TIER_DEVICE: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_collect_context_t {
    pub monotonic_ms: uint64_t,
    pub wall_time_ms: int64_t,
    pub proc_root: *const ::core::ffi::c_char,
    pub sys_root: *const ::core::ffi::c_char,
    pub cgroup_root: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_observation_sink_t {
    pub items: *mut system_health_observation_t,
    pub capacity: size_t,
    pub count: size_t,
    pub dropped: size_t,
}
pub type system_health_collect_fn = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const system_health_collect_context_t,
        *mut system_health_observation_sink_t,
    ) -> ::core::ffi::c_int,
>;
pub type system_health_collector_destroy_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_health_collector_t {
    pub name: [::core::ffi::c_char; 32],
    pub scope: system_health_scope_t,
    pub tier: system_health_sampling_tier_t,
    pub interval_seconds: uint32_t,
    pub stale_after_seconds: uint32_t,
    pub state: *mut ::core::ffi::c_void,
    pub collect: system_health_collect_fn,
    pub destroy: system_health_collector_destroy_fn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_filesystem_value_t {
    pub value: uint64_t,
    pub capability: system_health_capability_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_filesystem_flag_t {
    pub value: bool,
    pub capability: system_health_capability_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_filesystem_resource_t {
    pub logical_id: [::core::ffi::c_char; 64],
    pub path: [::core::ffi::c_char; 1024],
    pub mount_required: bool,
    pub mount_guard_path: [::core::ffi::c_char; 1024],
    pub expected_device_key: [::core::ffi::c_char; 48],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_filesystem_sample_t {
    pub logical_id: [::core::ffi::c_char; 64],
    pub device_key: [::core::ffi::c_char; 48],
    pub mount_present: linux_filesystem_flag_t,
    pub read_only: linux_filesystem_flag_t,
    pub mount_flags: linux_filesystem_value_t,
    pub capacity_bytes: linux_filesystem_value_t,
    pub available_bytes: linux_filesystem_value_t,
    pub capacity_inodes: linux_filesystem_value_t,
    pub available_inodes: linux_filesystem_value_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct linux_filesystem_probe_error_t(pub ::core::ffi::c_uint);
impl linux_filesystem_probe_error_t {
    pub const LINUX_FILESYSTEM_PROBE_ERROR_NONE: Self = Self(0);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_NOT_FOUND: Self = Self(1);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_PERMISSION: Self = Self(2);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_READ_ONLY: Self = Self(3);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_NO_SPACE: Self = Self(4);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_IO: Self = Self(5);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_TIMED_OUT: Self = Self(6);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_BUSY: Self = Self(7);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_INVALID: Self = Self(8);
    pub const LINUX_FILESYSTEM_PROBE_ERROR_OTHER: Self = Self(9);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_filesystem_probe_result_t {
    pub logical_id: [::core::ffi::c_char; 64],
    pub device_key: [::core::ffi::c_char; 48],
    pub capability: system_health_capability_t,
    pub error: linux_filesystem_probe_error_t,
    pub latency_ms: uint32_t,
    pub write_completed: bool,
    pub fsync_completed: bool,
    pub unlink_completed: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct health_helper_outcome_t(pub ::core::ffi::c_uint);
impl health_helper_outcome_t {
    pub const HEALTH_HELPER_OK: Self = Self(0);
    pub const HEALTH_HELPER_EXITED: Self = Self(1);
    pub const HEALTH_HELPER_TIMED_OUT: Self = Self(2);
    pub const HEALTH_HELPER_BUSY: Self = Self(3);
    pub const HEALTH_HELPER_EXEC_ERROR: Self = Self(4);
    pub const HEALTH_HELPER_SYSTEM_ERROR: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct health_helper_request_t {
    pub program: *const ::core::ffi::c_char,
    pub argv: *const *mut ::core::ffi::c_char,
    pub timeout_ms: uint32_t,
    pub terminate_grace_ms: uint32_t,
    pub output_limit: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct health_helper_result_t {
    pub outcome: health_helper_outcome_t,
    pub exit_code: ::core::ffi::c_int,
    pub term_signal: ::core::ffi::c_int,
    pub latency_ms: uint32_t,
    pub output_truncated: bool,
    pub abandoned: bool,
    pub output_length: size_t,
    pub output: [::core::ffi::c_char; 4097],
}
pub type storage_health_helper_run_fn = Option<
    unsafe extern "C" fn(
        *const health_helper_request_t,
        *mut health_helper_result_t,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_target_probe_ops_t {
    pub run_helper: storage_health_helper_run_fn,
    pub stat_program: *const ::core::ffi::c_char,
    pub dd_program: *const ::core::ffi::c_char,
    pub rm_program: *const ::core::ffi::c_char,
    pub mountinfo_path: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_target_health_sample_t {
    pub filesystem: linux_filesystem_sample_t,
    pub writeable: linux_filesystem_flag_t,
    pub probe: linux_filesystem_probe_result_t,
    pub filesystem_device: uint64_t,
    pub cleanup_failed: bool,
    pub recording_growth_bps: ::core::ffi::c_double,
    pub recording_growth_capability: system_health_capability_t,
    pub sampled_monotonic_ms: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_target_health_slot_t {
    pub resource: linux_filesystem_resource_t,
    pub target_uuid: [::core::ffi::c_char; 37],
    pub sample: storage_target_health_sample_t,
    pub sample_valid: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_target_health_collector_state_t {
    pub slots: [storage_target_health_slot_t; 18],
    pub slot_count: size_t,
    pub slots_dropped: size_t,
    pub next_probe: size_t,
    pub max_probes_per_cycle: size_t,
    pub timeout_ms: uint32_t,
    pub stale_after_ms: uint32_t,
    pub write_probe_enabled: bool,
    pub root_path: [::core::ffi::c_char; 1024],
    pub recording_path: [::core::ffi::c_char; 1024],
    pub mountinfo_path: [::core::ffi::c_char; 1024],
    pub ops: storage_target_probe_ops_t,
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
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const SYSTEM_HEALTH_MAX_FILESYSTEMS: ::core::ffi::c_uint = 18 as ::core::ffi::c_uint;
pub const STORAGE_TARGET_MAX_COUNT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const STORAGE_HEALTH_PROBES_PER_CYCLE: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const STORAGE_HEALTH_DEFAULT_TIMEOUT_MS: ::core::ffi::c_uint = 3000 as ::core::ffi::c_uint;
pub const STORAGE_HEALTH_DEFAULT_STALE_MS: ::core::ffi::c_uint = 900000 as ::core::ffi::c_uint;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut ::core::ffi::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as ::core::ffi::c_int, __stream);
}
pub const X_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
static mut target_probe_mutex: pthread_mutex_t = pthread_mutex_t {
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
static mut registered_state: storage_target_health_collector_state_t =
    storage_target_health_collector_state_t {
        slots: [storage_target_health_slot_t {
            resource: linux_filesystem_resource_t {
                logical_id: [0; 64],
                path: [0; 1024],
                mount_required: false,
                mount_guard_path: [0; 1024],
                expected_device_key: [0; 48],
            },
            target_uuid: [0; 37],
            sample: storage_target_health_sample_t {
                filesystem: linux_filesystem_sample_t {
                    logical_id: [0; 64],
                    device_key: [0; 48],
                    mount_present: linux_filesystem_flag_t {
                        value: false,
                        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                    },
                    read_only: linux_filesystem_flag_t {
                        value: false,
                        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                    },
                    mount_flags: linux_filesystem_value_t {
                        value: 0,
                        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                    },
                    capacity_bytes: linux_filesystem_value_t {
                        value: 0,
                        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                    },
                    available_bytes: linux_filesystem_value_t {
                        value: 0,
                        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                    },
                    capacity_inodes: linux_filesystem_value_t {
                        value: 0,
                        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                    },
                    available_inodes: linux_filesystem_value_t {
                        value: 0,
                        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                    },
                },
                writeable: linux_filesystem_flag_t {
                    value: false,
                    capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                },
                probe: linux_filesystem_probe_result_t {
                    logical_id: [0; 64],
                    device_key: [0; 48],
                    capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                    error: linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_NONE,
                    latency_ms: 0,
                    write_completed: false,
                    fsync_completed: false,
                    unlink_completed: false,
                },
                filesystem_device: 0,
                cleanup_failed: false,
                recording_growth_bps: 0.,
                recording_growth_capability:
                    system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                sampled_monotonic_ms: 0,
            },
            sample_valid: false,
        }; 18],
        slot_count: 0,
        slots_dropped: 0,
        next_probe: 0,
        max_probes_per_cycle: 0,
        timeout_ms: 0,
        stale_after_ms: 0,
        write_probe_enabled: false,
        root_path: [0; 1024],
        recording_path: [0; 1024],
        mountinfo_path: [0; 1024],
        ops: storage_target_probe_ops_t {
            run_helper: None,
            stat_program: ::core::ptr::null::<::core::ffi::c_char>(),
            dd_program: ::core::ptr::null::<::core::ffi::c_char>(),
            rm_program: ::core::ptr::null::<::core::ffi::c_char>(),
            mountinfo_path: ::core::ptr::null::<::core::ffi::c_char>(),
        },
    };
static mut registered_state_initialized: bool = false;
unsafe extern "C" fn monotonic_ms() -> uint64_t {
    let mut value: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_MONOTONIC, &raw mut value) != 0 as ::core::ffi::c_int {
        return 0 as uint64_t;
    }
    return (value.tv_sec as uint64_t)
        .wrapping_mul(1000 as uint64_t)
        .wrapping_add((value.tv_nsec as uint64_t).wrapping_div(1000000 as uint64_t));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_target_probe_error_name(
    mut error: linux_filesystem_probe_error_t,
) -> *const ::core::ffi::c_char {
    match error {
        linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_NONE => {
            return b"none\0".as_ptr() as *const ::core::ffi::c_char;
        }
        linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_NOT_FOUND => {
            return b"not_found\0".as_ptr() as *const ::core::ffi::c_char;
        }
        linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_PERMISSION => {
            return b"permission\0".as_ptr() as *const ::core::ffi::c_char;
        }
        linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_READ_ONLY => {
            return b"read_only\0".as_ptr() as *const ::core::ffi::c_char;
        }
        linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_NO_SPACE => {
            return b"no_space\0".as_ptr() as *const ::core::ffi::c_char;
        }
        linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_IO => {
            return b"io\0".as_ptr() as *const ::core::ffi::c_char;
        }
        linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_TIMED_OUT => {
            return b"timed_out\0".as_ptr() as *const ::core::ffi::c_char;
        }
        linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_BUSY => {
            return b"busy\0".as_ptr() as *const ::core::ffi::c_char;
        }
        linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_INVALID => {
            return b"invalid\0".as_ptr() as *const ::core::ffi::c_char;
        }
        linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_OTHER => {
            return b"other\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"other\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn select_program(
    mut preferred: *const ::core::ffi::c_char,
    mut fallback: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if !preferred.is_null() && access(preferred, X_OK) == 0 as ::core::ffi::c_int {
        return preferred;
    }
    return fallback;
}
unsafe extern "C" fn output_error(
    mut result: *const health_helper_result_t,
) -> linux_filesystem_probe_error_t {
    if result.is_null() {
        return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_OTHER;
    }
    if (*result).outcome.0 == health_helper_outcome_t::HEALTH_HELPER_TIMED_OUT.0 {
        return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_TIMED_OUT;
    }
    if (*result).outcome.0 == health_helper_outcome_t::HEALTH_HELPER_BUSY.0 {
        return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_BUSY;
    }
    if (*result).outcome.0 == health_helper_outcome_t::HEALTH_HELPER_EXEC_ERROR.0 {
        return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_INVALID;
    }
    if !strstr(
        &raw const (*result).output as *const ::core::ffi::c_char,
        b"Read-only file system\0".as_ptr() as *const ::core::ffi::c_char,
    )
    .is_null()
    {
        return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_READ_ONLY;
    }
    if !strstr(
        &raw const (*result).output as *const ::core::ffi::c_char,
        b"No space left on device\0".as_ptr() as *const ::core::ffi::c_char,
    )
    .is_null()
        || !strstr(
            &raw const (*result).output as *const ::core::ffi::c_char,
            b"Disk quota exceeded\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_NO_SPACE;
    }
    if !strstr(
        &raw const (*result).output as *const ::core::ffi::c_char,
        b"Input/output error\0".as_ptr() as *const ::core::ffi::c_char,
    )
    .is_null()
    {
        return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_IO;
    }
    if !strstr(
        &raw const (*result).output as *const ::core::ffi::c_char,
        b"Permission denied\0".as_ptr() as *const ::core::ffi::c_char,
    )
    .is_null()
        || !strstr(
            &raw const (*result).output as *const ::core::ffi::c_char,
            b"Operation not permitted\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_PERMISSION;
    }
    if !strstr(
        &raw const (*result).output as *const ::core::ffi::c_char,
        b"No such file\0".as_ptr() as *const ::core::ffi::c_char,
    )
    .is_null()
        || !strstr(
            &raw const (*result).output as *const ::core::ffi::c_char,
            b"not found\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_NOT_FOUND;
    }
    return linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_OTHER;
}
unsafe extern "C" fn helper_capability(
    mut result: *const health_helper_result_t,
) -> system_health_capability_t {
    if result.is_null() {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    if (*result).outcome.0 == health_helper_outcome_t::HEALTH_HELPER_BUSY.0 {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE;
    }
    if (*result).outcome.0 == health_helper_outcome_t::HEALTH_HELPER_EXEC_ERROR.0 {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    }
    if (*result).outcome.0 == health_helper_outcome_t::HEALTH_HELPER_OK.0 {
        return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    return system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
}
unsafe extern "C" fn invoke_helper(
    mut ops: *const storage_target_probe_ops_t,
    mut request: *const health_helper_request_t,
    mut result: *mut health_helper_result_t,
) -> bool {
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<health_helper_result_t>(),
    );
    if (*ops).run_helper.expect("non-null function pointer")(request, result)
        != 0 as ::core::ffi::c_int
    {
        memset(
            result as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<health_helper_result_t>(),
        );
        (*result).outcome = health_helper_outcome_t::HEALTH_HELPER_SYSTEM_ERROR;
        return r#false != 0;
    }
    return (*result).outcome.0 == health_helper_outcome_t::HEALTH_HELPER_OK.0;
}
unsafe extern "C" fn saturating_add_latency(mut latency: *mut uint32_t, mut addition: uint32_t) {
    if (UINT32_MAX as uint32_t).wrapping_sub(*latency) < addition {
        *latency = UINT32_MAX as uint32_t;
    } else {
        *latency = (*latency).wrapping_add(addition);
    };
}
unsafe extern "C" fn path_within(
    mut path: *const ::core::ffi::c_char,
    mut mount: *const ::core::ffi::c_char,
) -> bool {
    let mut length: size_t = strlen(mount);
    if strcmp(mount, b"/\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        return *path.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int;
    }
    return strncmp(path, mount, length) == 0 as ::core::ffi::c_int
        && (*path.offset(length as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            || *path.offset(length as isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int);
}
unsafe extern "C" fn decode_mount_path(
    mut encoded: *const ::core::ffi::c_char,
    mut decoded: *mut ::core::ffi::c_char,
    mut capacity: size_t,
) -> bool {
    let mut output: size_t = 0 as size_t;
    let mut input: size_t = 0 as size_t;
    while *encoded.offset(input as isize) != 0 {
        let mut value: ::core::ffi::c_uchar =
            *encoded.offset(input as isize) as ::core::ffi::c_uchar;
        if value as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
            && *encoded.offset(input.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int != 0
            && *encoded.offset(input.wrapping_add(2 as size_t) as isize) as ::core::ffi::c_int != 0
            && *encoded.offset(input.wrapping_add(3 as size_t) as isize) as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc())
                .offset(*encoded.offset(input.wrapping_add(1 as size_t) as isize)
                    as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
            && *(*__ctype_b_loc())
                .offset(*encoded.offset(input.wrapping_add(2 as size_t) as isize)
                    as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
            && *(*__ctype_b_loc())
                .offset(*encoded.offset(input.wrapping_add(3 as size_t) as isize)
                    as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            value = ((*encoded.offset(input.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_int
                - '0' as ::core::ffi::c_int)
                * 64 as ::core::ffi::c_int
                + (*encoded.offset(input.wrapping_add(2 as size_t) as isize) as ::core::ffi::c_int
                    - '0' as ::core::ffi::c_int)
                    * 8 as ::core::ffi::c_int
                + *encoded.offset(input.wrapping_add(3 as size_t) as isize) as ::core::ffi::c_int
                - '0' as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            input = input.wrapping_add(3 as size_t);
        }
        if value as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint
            || output.wrapping_add(1 as size_t) >= capacity
        {
            return r#false != 0;
        }
        let c2rust_fresh0 = output;
        output = output.wrapping_add(1);
        *decoded.offset(c2rust_fresh0 as isize) = value as ::core::ffi::c_char;
        input = input.wrapping_add(1);
    }
    *decoded.offset(output as isize) = '\0' as ::core::ffi::c_char;
    return r#true != 0;
}
unsafe extern "C" fn mount_facts(
    mut resource: *const linux_filesystem_resource_t,
    mut mountinfo_path: *const ::core::ffi::c_char,
    mut mounted: *mut bool,
    mut read_only: *mut bool,
) -> ::core::ffi::c_int {
    if resource.is_null() || mountinfo_path.is_null() || mounted.is_null() || read_only.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut input: *mut FILE = fopen(
        mountinfo_path,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if input.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut line_capacity: size_t = 0 as size_t;
    let mut best: size_t = 0 as size_t;
    let mut exact_guard: bool = r#false != 0;
    *mounted = r#false != 0;
    *read_only = r#false != 0;
    while getline(&raw mut line, &raw mut line_capacity, input) >= 0 as __ssize_t {
        let mut encoded: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut options: [::core::ffi::c_char; 256] = [0; 256];
        if sscanf(
            line,
            b"%*s %*s %*s %*s %1023s %255s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut encoded as *mut ::core::ffi::c_char,
            &raw mut options as *mut ::core::ffi::c_char,
        ) != 2 as ::core::ffi::c_int
        {
            continue;
        }
        let mut mount: [::core::ffi::c_char; 1024] = [0; 1024];
        if !decode_mount_path(
            &raw mut encoded as *mut ::core::ffi::c_char,
            &raw mut mount as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        ) {
            continue;
        }
        if (*resource).mount_required as ::core::ffi::c_int != 0
            && (*resource).mount_guard_path[0usize] as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut mount as *mut ::core::ffi::c_char,
                &raw const (*resource).mount_guard_path as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            exact_guard = r#true != 0;
        }
        let mut length: size_t = strlen(&raw mut mount as *mut ::core::ffi::c_char);
        if length < best
            || !path_within(
                &raw const (*resource).path as *const ::core::ffi::c_char,
                &raw mut mount as *mut ::core::ffi::c_char,
            )
        {
            continue;
        }
        best = length;
        *mounted = r#true != 0;
        *read_only = strcmp(
            &raw mut options as *mut ::core::ffi::c_char,
            b"ro\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                &raw mut options as *mut ::core::ffi::c_char,
                b"ro,\0".as_ptr() as *const ::core::ffi::c_char,
                3 as size_t,
            ) == 0 as ::core::ffi::c_int;
    }
    let mut saved_errno: ::core::ffi::c_int = *__errno_location();
    free(line as *mut ::core::ffi::c_void);
    let mut failed: bool = ferror(input) != 0 as ::core::ffi::c_int;
    fclose(input);
    *__errno_location() = saved_errno;
    if failed {
        return -1 as ::core::ffi::c_int;
    }
    if (*resource).mount_required {
        let mut guard: *const ::core::ffi::c_char =
            if (*resource).mount_guard_path[0usize] as ::core::ffi::c_int != 0 {
                &raw const (*resource).mount_guard_path as *const ::core::ffi::c_char
            } else {
                &raw const (*resource).path as *const ::core::ffi::c_char
            };
        if (*resource).mount_guard_path[0usize] != 0 {
            *mounted = exact_guard;
        } else {
            let mut present: bool = r#false != 0;
            if linux_filesystem_mountinfo_contains(mountinfo_path, guard, &raw mut present)
                != 0 as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
            *mounted = present;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_u64_token(
    mut cursor: *mut *const ::core::ffi::c_char,
    mut base: ::core::ffi::c_int,
    mut value: *mut uint64_t,
) -> bool {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    while *(*__ctype_b_loc())
        .offset(**cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        != 0
    {
        *cursor = (*cursor).offset(1);
    }
    if **cursor as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || **cursor as ::core::ffi::c_int == '-' as ::core::ffi::c_int
        || **cursor as ::core::ffi::c_int == '+' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut parsed: ::core::ffi::c_ulonglong = strtoull(*cursor, &raw mut end, base);
    if *__errno_location() == ERANGE || end == *cursor as *mut ::core::ffi::c_char {
        return r#false != 0;
    }
    *value = parsed as uint64_t;
    *cursor = end;
    return r#true != 0;
}
unsafe extern "C" fn multiply_u64(
    mut left: uint64_t,
    mut right: uint64_t,
    mut result: *mut uint64_t,
) -> bool {
    if left != 0 as uint64_t && right > (UINT64_MAX as uint64_t).wrapping_div(left) {
        return r#false != 0;
    }
    *result = left.wrapping_mul(right);
    return r#true != 0;
}
unsafe extern "C" fn parse_stat_output(
    mut text: *const ::core::ffi::c_char,
    mut sample: *mut storage_target_health_sample_t,
) -> ::core::ffi::c_int {
    let mut fragment: uint64_t = 0;
    let mut blocks: uint64_t = 0;
    let mut available: uint64_t = 0;
    let mut files: uint64_t = 0;
    let mut free_files: uint64_t = 0;
    let mut device: uint64_t = 0;
    let mut cursor: *const ::core::ffi::c_char = text;
    if !parse_u64_token(&raw mut cursor, 10 as ::core::ffi::c_int, &raw mut fragment)
        || !parse_u64_token(&raw mut cursor, 10 as ::core::ffi::c_int, &raw mut blocks)
        || !parse_u64_token(
            &raw mut cursor,
            10 as ::core::ffi::c_int,
            &raw mut available,
        )
        || !parse_u64_token(&raw mut cursor, 10 as ::core::ffi::c_int, &raw mut files)
        || !parse_u64_token(
            &raw mut cursor,
            10 as ::core::ffi::c_int,
            &raw mut free_files,
        )
        || !parse_u64_token(&raw mut cursor, 16 as ::core::ffi::c_int, &raw mut device)
    {
        return -1 as ::core::ffi::c_int;
    }
    while *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        != 0
    {
        cursor = cursor.offset(1);
    }
    if *cursor as ::core::ffi::c_int != 0
        || available > blocks
        || free_files > files
        || !multiply_u64(
            fragment,
            blocks,
            &raw mut (*sample).filesystem.capacity_bytes.value,
        )
        || !multiply_u64(
            fragment,
            available,
            &raw mut (*sample).filesystem.available_bytes.value,
        )
    {
        return -1 as ::core::ffi::c_int;
    }
    (*sample).filesystem.capacity_bytes.capability =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    (*sample).filesystem.available_bytes.capability =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    if files == 0 as uint64_t {
        (*sample).filesystem.capacity_inodes.capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
        (*sample).filesystem.available_inodes.capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    } else {
        (*sample).filesystem.capacity_inodes.value = files;
        (*sample).filesystem.capacity_inodes.capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        (*sample).filesystem.available_inodes.value = free_files;
        (*sample).filesystem.available_inodes.capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    (*sample).filesystem_device = device;
    snprintf(
        &raw mut (*sample).filesystem.device_key as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 48]>(),
        b"linux-fs-%016llx\0".as_ptr() as *const ::core::ffi::c_char,
        device as ::core::ffi::c_ulonglong,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn make_unavailable(
    mut sample: *mut storage_target_health_sample_t,
    mut capability: system_health_capability_t,
) {
    memset(
        sample as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_target_health_sample_t>(),
    );
    (*sample).filesystem.mount_present.capability = capability;
    (*sample).filesystem.read_only.capability = capability;
    (*sample).filesystem.capacity_bytes.capability = capability;
    (*sample).filesystem.available_bytes.capability = capability;
    (*sample).filesystem.capacity_inodes.capability = capability;
    (*sample).filesystem.available_inodes.capability = capability;
    (*sample).writeable.capability = capability;
    (*sample).probe.capability = capability;
    (*sample).recording_growth_capability =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_target_health_probe_with_ops(
    mut resource: *const linux_filesystem_resource_t,
    mut write_probe: bool,
    mut timeout_ms: uint32_t,
    mut ops: *const storage_target_probe_ops_t,
    mut sample: *mut storage_target_health_sample_t,
) -> ::core::ffi::c_int {
    if resource.is_null()
        || ops.is_null()
        || (*ops).run_helper.is_none()
        || (*ops).stat_program.is_null()
        || (*ops).dd_program.is_null()
        || (*ops).rm_program.is_null()
        || sample.is_null()
        || timeout_ms == 0 as uint32_t
        || !linux_filesystem_logical_id_valid(
            &raw const (*resource).logical_id as *const ::core::ffi::c_char,
        )
        || (*resource).path[0usize] as ::core::ffi::c_int != '/' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    make_unavailable(
        sample,
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR,
    );
    snprintf(
        &raw mut (*sample).filesystem.logical_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*resource).logical_id as *const ::core::ffi::c_char,
    );
    snprintf(
        &raw mut (*sample).probe.logical_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*resource).logical_id as *const ::core::ffi::c_char,
    );
    (*sample).sampled_monotonic_ms = monotonic_ms();
    let mut mounted: bool = r#true != 0;
    let mut read_only: bool = r#false != 0;
    let mut mountinfo: *const ::core::ffi::c_char = if !(*ops).mountinfo_path.is_null()
        && *(*ops).mountinfo_path.offset(0isize) as ::core::ffi::c_int != 0
    {
        (*ops).mountinfo_path
    } else {
        b"/proc/self/mountinfo\0".as_ptr() as *const ::core::ffi::c_char
    };
    if mount_facts(resource, mountinfo, &raw mut mounted, &raw mut read_only)
        != 0 as ::core::ffi::c_int
    {
        let mut mount_error: ::core::ffi::c_int = *__errno_location();
        let mut capability: system_health_capability_t =
            linux_filesystem_capability_from_errno(mount_error);
        make_unavailable(sample, capability);
        snprintf(
            &raw mut (*sample).filesystem.logical_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*resource).logical_id as *const ::core::ffi::c_char,
        );
        snprintf(
            &raw mut (*sample).probe.logical_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*resource).logical_id as *const ::core::ffi::c_char,
        );
        (*sample).probe.error = linux_filesystem_normalize_errno(mount_error);
        if (*sample).probe.error.0
            == linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_NONE.0
        {
            (*sample).probe.error =
                linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_OTHER;
        }
        return 0 as ::core::ffi::c_int;
    }
    (*sample).filesystem.mount_present.value = mounted;
    (*sample).filesystem.mount_present.capability =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    (*sample).filesystem.read_only.value = read_only;
    (*sample).filesystem.read_only.capability =
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    if !mounted {
        (*sample).probe.error =
            linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_NOT_FOUND;
        return 0 as ::core::ffi::c_int;
    }
    let mut stat_arguments: [*mut ::core::ffi::c_char; 7] = [
        (*ops).stat_program as *mut ::core::ffi::c_char,
        b"-f\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"-c\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"%S %b %a %c %d %i\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"--\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw const (*resource).path as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ];
    let mut request: health_helper_request_t = health_helper_request_t {
        program: (*ops).stat_program,
        argv: &raw mut stat_arguments as *mut *mut ::core::ffi::c_char,
        timeout_ms: timeout_ms,
        terminate_grace_ms: 100 as uint32_t,
        output_limit: 256 as size_t,
    };
    let mut helper: health_helper_result_t = health_helper_result_t {
        outcome: health_helper_outcome_t::HEALTH_HELPER_OK,
        exit_code: 0,
        term_signal: 0,
        latency_ms: 0,
        output_truncated: false,
        abandoned: false,
        output_length: 0,
        output: [0; 4097],
    };
    if !invoke_helper(ops, &raw mut request, &raw mut helper) {
        (*sample).probe.capability = helper_capability(&raw mut helper);
        (*sample).probe.error = output_error(&raw mut helper);
        (*sample).probe.latency_ms = helper.latency_ms;
        return 0 as ::core::ffi::c_int;
    }
    (*sample).probe.latency_ms = helper.latency_ms;
    if parse_stat_output(&raw mut helper.output as *mut ::core::ffi::c_char, sample)
        != 0 as ::core::ffi::c_int
    {
        (*sample).probe.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        (*sample).probe.error =
            linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_INVALID;
        return 0 as ::core::ffi::c_int;
    }
    snprintf(
        &raw mut (*sample).probe.device_key as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 48]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*sample).filesystem.device_key as *mut ::core::ffi::c_char,
    );
    if !write_probe {
        (*sample).writeable.capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
        (*sample).probe.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        (*sample).probe.error = linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_NONE;
        return 0 as ::core::ffi::c_int;
    }
    if read_only {
        (*sample).writeable.value = r#false != 0;
        (*sample).writeable.capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        (*sample).probe.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        (*sample).probe.error =
            linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_READ_ONLY;
        return 0 as ::core::ffi::c_int;
    }
    let mut probe_path: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut output_argument: [::core::ffi::c_char; 1028] = [0; 1028];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut probe_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"%s/.lightnvr-health-%ld-%08llx\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*resource).path as *const ::core::ffi::c_char,
        getpid() as ::core::ffi::c_long,
        ((*sample).sampled_monotonic_ms & 0xffffffff as uint64_t) as ::core::ffi::c_ulonglong,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
        || snprintf(
            &raw mut output_argument as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1028]>(),
            b"of=%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut probe_path as *mut ::core::ffi::c_char,
        ) >= ::core::mem::size_of::<[::core::ffi::c_char; 1028]>() as ::core::ffi::c_int
    {
        (*sample).probe.error =
            linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_INVALID;
        return 0 as ::core::ffi::c_int;
    }
    let mut dd_arguments: [*mut ::core::ffi::c_char; 8] = [
        (*ops).dd_program as *mut ::core::ffi::c_char,
        b"if=/dev/zero\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut output_argument as *mut ::core::ffi::c_char,
        b"bs=1\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"count=1\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"conv=fsync\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"status=none\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ];
    request.program = (*ops).dd_program;
    request.argv = &raw mut dd_arguments as *mut *mut ::core::ffi::c_char;
    if !invoke_helper(ops, &raw mut request, &raw mut helper) {
        let mut write_result: health_helper_result_t = helper;
        (*sample).writeable.value = r#false != 0;
        (*sample).writeable.capability = system_health_capability_t(
            if write_result.outcome.0 == health_helper_outcome_t::HEALTH_HELPER_EXITED.0 {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
            } else {
                helper_capability(&raw mut write_result).0
            },
        );
        (*sample).probe.capability = (*sample).writeable.capability;
        (*sample).probe.error = output_error(&raw mut write_result);
        saturating_add_latency(&raw mut (*sample).probe.latency_ms, write_result.latency_ms);
        let mut rm_arguments: [*mut ::core::ffi::c_char; 5] = [
            (*ops).rm_program as *mut ::core::ffi::c_char,
            b"-f\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            b"--\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut probe_path as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ];
        request.program = (*ops).rm_program;
        request.argv = &raw mut rm_arguments as *mut *mut ::core::ffi::c_char;
        if write_result.abandoned {
            (*sample).cleanup_failed = r#true != 0;
            return 0 as ::core::ffi::c_int;
        }
        if !invoke_helper(ops, &raw mut request, &raw mut helper) {
            (*sample).cleanup_failed = r#true != 0;
        } else {
            (*sample).probe.unlink_completed = r#true != 0;
        }
        saturating_add_latency(&raw mut (*sample).probe.latency_ms, helper.latency_ms);
        return 0 as ::core::ffi::c_int;
    }
    saturating_add_latency(&raw mut (*sample).probe.latency_ms, helper.latency_ms);
    (*sample).probe.write_completed = r#true != 0;
    (*sample).probe.fsync_completed = r#true != 0;
    let mut rm_arguments_0: [*mut ::core::ffi::c_char; 5] = [
        (*ops).rm_program as *mut ::core::ffi::c_char,
        b"-f\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        b"--\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut probe_path as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ];
    request.program = (*ops).rm_program;
    request.argv = &raw mut rm_arguments_0 as *mut *mut ::core::ffi::c_char;
    if !invoke_helper(ops, &raw mut request, &raw mut helper) {
        (*sample).writeable.value = r#true != 0;
        (*sample).writeable.capability =
            system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        (*sample).probe.capability = helper_capability(&raw mut helper);
        (*sample).probe.error = output_error(&raw mut helper);
        (*sample).cleanup_failed = r#true != 0;
        saturating_add_latency(&raw mut (*sample).probe.latency_ms, helper.latency_ms);
        return 0 as ::core::ffi::c_int;
    }
    saturating_add_latency(&raw mut (*sample).probe.latency_ms, helper.latency_ms);
    (*sample).writeable.value = r#true != 0;
    (*sample).writeable.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    (*sample).probe.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    (*sample).probe.error = linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_NONE;
    (*sample).probe.unlink_completed = r#true != 0;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn default_probe_ops(mut ops: *mut storage_target_probe_ops_t) {
    memset(
        ops as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_target_probe_ops_t>(),
    );
    (*ops).run_helper = Some(
        health_helper_run
            as unsafe extern "C" fn(
                *const health_helper_request_t,
                *mut health_helper_result_t,
            ) -> ::core::ffi::c_int,
    ) as storage_health_helper_run_fn;
    (*ops).stat_program = select_program(
        b"/usr/bin/stat\0".as_ptr() as *const ::core::ffi::c_char,
        b"/bin/stat\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*ops).dd_program = select_program(
        b"/usr/bin/dd\0".as_ptr() as *const ::core::ffi::c_char,
        b"/bin/dd\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*ops).rm_program = select_program(
        b"/usr/bin/rm\0".as_ptr() as *const ::core::ffi::c_char,
        b"/bin/rm\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_target_health_collector_state_init(
    mut state: *mut storage_target_health_collector_state_t,
    mut root_path: *const ::core::ffi::c_char,
    mut recording_path: *const ::core::ffi::c_char,
    mut write_probe_enabled: bool,
) {
    if state.is_null() {
        return;
    }
    memset(
        state as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_target_health_collector_state_t>(),
    );
    snprintf(
        &raw mut (*state).root_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        if !root_path.is_null() && *root_path.offset(0isize) as ::core::ffi::c_int != 0 {
            root_path
        } else {
            b"/\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    snprintf(
        &raw mut (*state).recording_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        if !recording_path.is_null() && *recording_path.offset(0isize) as ::core::ffi::c_int != 0 {
            recording_path
        } else {
            b"/\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    (*state).write_probe_enabled = write_probe_enabled;
    (*state).timeout_ms = STORAGE_HEALTH_DEFAULT_TIMEOUT_MS as uint32_t;
    (*state).stale_after_ms = STORAGE_HEALTH_DEFAULT_STALE_MS as uint32_t;
    (*state).max_probes_per_cycle = STORAGE_HEALTH_PROBES_PER_CYCLE as size_t;
    default_probe_ops(&raw mut (*state).ops);
}
unsafe extern "C" fn find_old_slot(
    mut old: *const storage_target_health_slot_t,
    mut count: size_t,
    mut logical_id: *const ::core::ffi::c_char,
) -> *const storage_target_health_slot_t {
    let mut index: size_t = 0 as size_t;
    while index < count {
        if strcmp(
            &raw const (*old.offset(index as isize)).resource.logical_id
                as *const ::core::ffi::c_char,
            logical_id,
        ) == 0 as ::core::ffi::c_int
        {
            return old.offset(index as isize);
        }
        index = index.wrapping_add(1);
    }
    return ::core::ptr::null::<storage_target_health_slot_t>();
}
unsafe extern "C" fn add_slot(
    mut state: *mut storage_target_health_collector_state_t,
    mut old: *const storage_target_health_slot_t,
    mut old_count: size_t,
    mut logical_id: *const ::core::ffi::c_char,
    mut path: *const ::core::ffi::c_char,
    mut target: *const storage_target_t,
) -> bool {
    if (*state).slot_count >= SYSTEM_HEALTH_MAX_FILESYSTEMS as size_t {
        (*state).slots_dropped = (*state).slots_dropped.wrapping_add(1);
        return r#false != 0;
    }
    let c2rust_fresh2 = (*state).slot_count;
    (*state).slot_count = (*state).slot_count.wrapping_add(1);
    let mut slot: *mut storage_target_health_slot_t = (&raw mut (*state).slots
        as *mut storage_target_health_slot_t)
        .offset(c2rust_fresh2 as isize);
    memset(
        slot as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_target_health_slot_t>(),
    );
    let mut previous: *const storage_target_health_slot_t =
        find_old_slot(old, old_count, logical_id);
    if !previous.is_null() {
        (*slot).sample = (*previous).sample;
        (*slot).sample_valid = (*previous).sample_valid;
    }
    snprintf(
        &raw mut (*slot).resource.logical_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        logical_id,
    );
    snprintf(
        &raw mut (*slot).resource.path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        path,
    );
    if !target.is_null() {
        snprintf(
            &raw mut (*slot).target_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*target).uuid as *const ::core::ffi::c_char,
        );
        (*slot).resource.mount_required = (*target).mount_required;
        snprintf(
            &raw mut (*slot).resource.mount_guard_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*target).mount_guard_path as *const ::core::ffi::c_char,
        );
    }
    return r#true != 0;
}
unsafe extern "C" fn refresh_slots(
    mut state: *mut storage_target_health_collector_state_t,
) -> ::core::ffi::c_int {
    let mut old: *mut storage_target_health_slot_t = calloc(
        SYSTEM_HEALTH_MAX_FILESYSTEMS as size_t,
        ::core::mem::size_of::<storage_target_health_slot_t>(),
    ) as *mut storage_target_health_slot_t;
    if old.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut old_count: size_t = (*state).slot_count;
    memcpy(
        old as *mut ::core::ffi::c_void,
        &raw mut (*state).slots as *mut storage_target_health_slot_t as *const ::core::ffi::c_void,
        old_count.wrapping_mul(::core::mem::size_of::<storage_target_health_slot_t>()),
    );
    memset(
        &raw mut (*state).slots as *mut storage_target_health_slot_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[storage_target_health_slot_t; 18]>(),
    );
    (*state).slot_count = 0 as size_t;
    (*state).slots_dropped = 0 as size_t;
    add_slot(
        state,
        old,
        old_count,
        b"root\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*state).root_path as *mut ::core::ffi::c_char,
        ::core::ptr::null::<storage_target_t>(),
    );
    add_slot(
        state,
        old,
        old_count,
        b"recording\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*state).recording_path as *mut ::core::ffi::c_char,
        ::core::ptr::null::<storage_target_t>(),
    );
    let mut total: ::core::ffi::c_int = db_storage_target_count();
    if total < 0 as ::core::ffi::c_int || total > STORAGE_TARGET_MAX_COUNT {
        free(old as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut targets: *mut storage_target_t = (if total > 0 as ::core::ffi::c_int {
        calloc(total as size_t, ::core::mem::size_of::<storage_target_t>())
    } else {
        NULL
    }) as *mut storage_target_t;
    let mut count: ::core::ffi::c_int = if total > 0 as ::core::ffi::c_int && !targets.is_null() {
        db_storage_target_list(targets, total)
    } else {
        total
    };
    if count < 0 as ::core::ffi::c_int || total > 0 as ::core::ffi::c_int && targets.is_null() {
        free(targets as *mut ::core::ffi::c_void);
        free(old as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        if !(!(*targets.offset(index as isize)).enabled
            || strcmp(
                &raw mut (*targets.offset(index as isize)).target_type as *mut ::core::ffi::c_char,
                b"filesystem\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int)
        {
            let mut logical_id: [::core::ffi::c_char; 64] = [0; 64];
            snprintf(
                &raw mut logical_id as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                b"target:%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*targets.offset(index as isize)).uuid as *mut ::core::ffi::c_char,
            );
            add_slot(
                state,
                old,
                old_count,
                &raw mut logical_id as *mut ::core::ffi::c_char,
                &raw mut (*targets.offset(index as isize)).root_path as *mut ::core::ffi::c_char,
                targets.offset(index as isize),
            );
        }
        index += 1;
    }
    free(targets as *mut ::core::ffi::c_void);
    free(old as *mut ::core::ffi::c_void);
    if (*state).slot_count > 0 as size_t {
        (*state).next_probe = (*state).next_probe.wrapping_rem((*state).slot_count);
    } else {
        (*state).next_probe = 0 as size_t;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn sample_available(mut sample: *const storage_target_health_sample_t) -> bool {
    return (*sample).filesystem.mount_present.capability.0
        == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        && (*sample).filesystem.mount_present.value as ::core::ffi::c_int != 0
        && (*sample).filesystem.capacity_bytes.capability.0
            == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        && (*sample).filesystem.available_bytes.capability.0
            == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0;
}
unsafe extern "C" fn persist_target_sample(
    mut slot: *const storage_target_health_slot_t,
    mut write_probe: bool,
    mut wall_time_ms: int64_t,
) -> ::core::ffi::c_int {
    if (*slot).target_uuid[0usize] == 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut before: storage_target_t = storage_target_t {
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
    let mut sample: *const storage_target_health_sample_t = &raw const (*slot).sample;
    let mut update: storage_target_health_update_t = storage_target_health_update_t {
        available: false,
        write_checked: false,
        writeable: false,
        cleanup_failed: false,
        capacity_bytes: 0,
        available_bytes: 0,
        filesystem_device: 0,
        probed_at: 0,
        normalized_error: [0; 32],
    };
    memset(
        &raw mut update as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_target_health_update_t>(),
    );
    update.available = sample_available(sample);
    update.write_checked = write_probe;
    update.writeable = (*sample).writeable.capability.0
        == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        && (*sample).writeable.value as ::core::ffi::c_int != 0;
    update.cleanup_failed = (*sample).cleanup_failed;
    update.capacity_bytes = (*sample).filesystem.capacity_bytes.value;
    update.available_bytes = (*sample).filesystem.available_bytes.value;
    update.filesystem_device = (*sample).filesystem_device;
    update.probed_at = if wall_time_ms > 0 as int64_t {
        wall_time_ms / 1000 as int64_t
    } else {
        time(::core::ptr::null_mut::<time_t>()) as int64_t
    };
    let mut error: *const ::core::ffi::c_char =
        storage_target_probe_error_name((*sample).probe.error);
    if !update.available
        && (*sample).filesystem.mount_present.capability.0
            == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0
        && !(*sample).filesystem.mount_present.value
    {
        error = b"mount_absent\0".as_ptr() as *const ::core::ffi::c_char;
    }
    snprintf(
        &raw mut update.normalized_error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        error,
    );
    let mut after: storage_target_t = storage_target_t {
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
    pthread_mutex_lock(&raw mut target_probe_mutex);
    if db_storage_target_get(
        &raw const (*slot).target_uuid as *const ::core::ffi::c_char,
        &raw mut before,
    )
    .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        pthread_mutex_unlock(&raw mut target_probe_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut result: db_storage_target_result_t = db_storage_target_record_health(
        &raw const (*slot).target_uuid as *const ::core::ffi::c_char,
        &raw mut update,
        &raw mut after,
    );
    if result.0 == db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        publish_transition(&raw mut before, &raw mut after);
    }
    pthread_mutex_unlock(&raw mut target_probe_mutex);
    return if result.0 == db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn effective_capability(
    mut capability: system_health_capability_t,
    mut stale: bool,
) -> system_health_capability_t {
    return system_health_capability_t(if stale as ::core::ffi::c_int != 0 {
        system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE.0
    } else {
        capability.0
    });
}
unsafe extern "C" fn prepare_observation(
    mut observation: *mut system_health_observation_t,
    mut slot: *const storage_target_health_slot_t,
    mut context: *const system_health_collect_context_t,
    mut metric: *const ::core::ffi::c_char,
) {
    memset(
        observation as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_observation_t>(),
    );
    snprintf(
        &raw mut (*observation).metric as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        metric,
    );
    snprintf(
        &raw mut (*observation).resource_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*slot).resource.logical_id as *const ::core::ffi::c_char,
    );
    (*observation).scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_FILESYSTEM;
    (*observation).sampled_monotonic_ms = (*slot).sample.sampled_monotonic_ms;
    (*observation).observed_wall_time_ms = (*context).wall_time_ms;
}
unsafe extern "C" fn emit_number(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
    mut slot: *const storage_target_health_slot_t,
    mut metric: *const ::core::ffi::c_char,
    mut value: ::core::ffi::c_double,
    mut capability: system_health_capability_t,
    mut unit: system_health_unit_t,
    mut stale: bool,
) {
    let mut observation: system_health_observation_t = system_health_observation_t {
        metric: [0; 64],
        resource_id: [0; 64],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        freshness: system_health_freshness_t::SYSTEM_HEALTH_FRESHNESS_UNKNOWN,
        unit: system_health_unit_t::SYSTEM_HEALTH_UNIT_NONE,
        value_valid: false,
        value: 0.,
        sampled_monotonic_ms: 0,
        observed_wall_time_ms: 0,
    };
    prepare_observation(&raw mut observation, slot, context, metric);
    capability = effective_capability(capability, stale);
    if capability.0 == system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE.0 {
        system_health_observation_set_available(&raw mut observation, value, unit);
    } else {
        system_health_observation_set_unavailable(&raw mut observation, capability);
    }
    system_health_observation_sink_append(sink, &raw mut observation);
}
unsafe extern "C" fn emit_flag(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
    mut slot: *const storage_target_health_slot_t,
    mut metric: *const ::core::ffi::c_char,
    mut flag: *const linux_filesystem_flag_t,
    mut stale: bool,
) {
    emit_number(
        sink,
        context,
        slot,
        metric,
        if (*flag).value as ::core::ffi::c_int != 0 {
            1.0f64
        } else {
            0.0f64
        },
        (*flag).capability,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
        stale,
    );
}
unsafe extern "C" fn emit_value(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
    mut slot: *const storage_target_health_slot_t,
    mut metric: *const ::core::ffi::c_char,
    mut value: *const linux_filesystem_value_t,
    mut unit: system_health_unit_t,
    mut stale: bool,
) {
    emit_number(
        sink,
        context,
        slot,
        metric,
        (*value).value as ::core::ffi::c_double,
        (*value).capability,
        unit,
        stale,
    );
}
unsafe extern "C" fn emit_storage_observations(
    mut sink: *mut system_health_observation_sink_t,
    mut context: *const system_health_collect_context_t,
    mut slot: *const storage_target_health_slot_t,
    mut stale: bool,
    mut emit_canonical: bool,
    mut emit_write_condition: bool,
) {
    let mut sample: *const storage_target_health_sample_t = &raw const (*slot).sample;
    emit_flag(
        sink,
        context,
        slot,
        b"storage.filesystem.mount_present\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*sample).filesystem.mount_present,
        stale,
    );
    emit_flag(
        sink,
        context,
        slot,
        b"storage.filesystem.read_only\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*sample).filesystem.read_only,
        stale,
    );
    emit_value(
        sink,
        context,
        slot,
        b"storage.filesystem.capacity_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*sample).filesystem.capacity_bytes,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BYTES,
        stale,
    );
    emit_value(
        sink,
        context,
        slot,
        b"storage.filesystem.available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*sample).filesystem.available_bytes,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BYTES,
        stale,
    );
    emit_value(
        sink,
        context,
        slot,
        b"storage.filesystem.capacity_inodes\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*sample).filesystem.capacity_inodes,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        stale,
    );
    emit_value(
        sink,
        context,
        slot,
        b"storage.filesystem.available_inodes\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*sample).filesystem.available_inodes,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        stale,
    );
    emit_flag(
        sink,
        context,
        slot,
        b"storage.filesystem.writeable\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*sample).writeable,
        stale,
    );
    emit_number(
        sink,
        context,
        slot,
        b"storage.filesystem.probe_latency_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        (*sample).probe.latency_ms as ::core::ffi::c_double / 1000.0f64,
        (*sample).probe.capability,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_SECONDS,
        stale,
    );
    emit_number(
        sink,
        context,
        slot,
        b"storage.filesystem.recording_growth_bytes_per_second\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*sample).recording_growth_bps,
        (*sample).recording_growth_capability,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        stale,
    );
    if emit_write_condition {
        emit_number(
            sink,
            context,
            slot,
            b"filesystem.write_failed\0".as_ptr() as *const ::core::ffi::c_char,
            if (*sample).probe.error.0
                == linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_NONE.0
            {
                0.0f64
            } else {
                1.0f64
            },
            (*sample).writeable.capability,
            system_health_unit_t::SYSTEM_HEALTH_UNIT_BOOLEAN,
            stale,
        );
    }
    if !emit_canonical {
        return;
    }
    emit_flag(
        sink,
        context,
        slot,
        b"filesystem.mount_present\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*sample).filesystem.mount_present,
        stale,
    );
    emit_flag(
        sink,
        context,
        slot,
        b"filesystem.read_only\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*sample).filesystem.read_only,
        stale,
    );
    emit_value(
        sink,
        context,
        slot,
        b"filesystem.capacity_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*sample).filesystem.capacity_bytes,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BYTES,
        stale,
    );
    emit_value(
        sink,
        context,
        slot,
        b"filesystem.available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*sample).filesystem.available_bytes,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_BYTES,
        stale,
    );
    emit_value(
        sink,
        context,
        slot,
        b"filesystem.capacity_inodes\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*sample).filesystem.capacity_inodes,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        stale,
    );
    emit_value(
        sink,
        context,
        slot,
        b"filesystem.available_inodes\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*sample).filesystem.available_inodes,
        system_health_unit_t::SYSTEM_HEALTH_UNIT_COUNT,
        stale,
    );
}
unsafe extern "C" fn device_seen_before(
    mut state: *const storage_target_health_collector_state_t,
    mut index: size_t,
) -> bool {
    let mut key: *const ::core::ffi::c_char = &raw const (*(&raw const (*state).slots
        as *const storage_target_health_slot_t)
        .offset(index as isize))
    .sample
    .filesystem
    .device_key as *const ::core::ffi::c_char;
    if *key.offset(0isize) == 0 {
        return r#false != 0;
    }
    let mut previous: size_t = 0 as size_t;
    while previous < index {
        if (*state).slots[previous].sample_valid as ::core::ffi::c_int != 0
            && strcmp(
                key,
                &raw const (*(&raw const (*state).slots as *const storage_target_health_slot_t)
                    .offset(previous as isize))
                .sample
                .filesystem
                .device_key as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        previous = previous.wrapping_add(1);
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_target_health_collect(
    mut opaque: *mut ::core::ffi::c_void,
    mut context: *const system_health_collect_context_t,
    mut sink: *mut system_health_observation_sink_t,
) -> ::core::ffi::c_int {
    let mut state: *mut storage_target_health_collector_state_t =
        opaque as *mut storage_target_health_collector_state_t;
    if state.is_null()
        || context.is_null()
        || sink.is_null()
        || refresh_slots(state) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut mountinfo: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut proc_root: *const ::core::ffi::c_char = if !(*context).proc_root.is_null() {
        (*context).proc_root
    } else {
        b"/proc\0".as_ptr() as *const ::core::ffi::c_char
    };
    snprintf(
        &raw mut mountinfo as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"%s/self/mountinfo\0".as_ptr() as *const ::core::ffi::c_char,
        proc_root,
    );
    let mut ops: storage_target_probe_ops_t = (*state).ops;
    if ops.mountinfo_path.is_null() {
        ops.mountinfo_path = if (*state).mountinfo_path[0usize] as ::core::ffi::c_int != 0 {
            &raw mut (*state).mountinfo_path as *mut ::core::ffi::c_char
        } else {
            &raw mut mountinfo as *mut ::core::ffi::c_char
        };
    }
    let mut probes: size_t = (*state).max_probes_per_cycle;
    if probes == 0 as size_t || probes > (*state).slot_count {
        probes = (*state).slot_count;
    }
    let mut failures: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut attempt: size_t = 0 as size_t;
    while attempt < probes && (*state).slot_count > 0 as size_t {
        let c2rust_fresh1 = (*state).next_probe;
        (*state).next_probe = (*state).next_probe.wrapping_add(1);
        let mut index: size_t = c2rust_fresh1.wrapping_rem((*state).slot_count);
        let mut slot: *mut storage_target_health_slot_t =
            (&raw mut (*state).slots as *mut storage_target_health_slot_t).offset(index as isize);
        let mut sample: storage_target_health_sample_t = storage_target_health_sample_t {
            filesystem: linux_filesystem_sample_t {
                logical_id: [0; 64],
                device_key: [0; 48],
                mount_present: linux_filesystem_flag_t {
                    value: false,
                    capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                },
                read_only: linux_filesystem_flag_t {
                    value: false,
                    capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                },
                mount_flags: linux_filesystem_value_t {
                    value: 0,
                    capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                },
                capacity_bytes: linux_filesystem_value_t {
                    value: 0,
                    capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                },
                available_bytes: linux_filesystem_value_t {
                    value: 0,
                    capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                },
                capacity_inodes: linux_filesystem_value_t {
                    value: 0,
                    capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                },
                available_inodes: linux_filesystem_value_t {
                    value: 0,
                    capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                },
            },
            writeable: linux_filesystem_flag_t {
                value: false,
                capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            },
            probe: linux_filesystem_probe_result_t {
                logical_id: [0; 64],
                device_key: [0; 48],
                capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                error: linux_filesystem_probe_error_t::LINUX_FILESYSTEM_PROBE_ERROR_NONE,
                latency_ms: 0,
                write_completed: false,
                fsync_completed: false,
                unlink_completed: false,
            },
            filesystem_device: 0,
            cleanup_failed: false,
            recording_growth_bps: 0.,
            recording_growth_capability:
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
            sampled_monotonic_ms: 0,
        };
        if storage_target_health_probe_with_ops(
            &raw mut (*slot).resource,
            (*state).write_probe_enabled,
            (*state).timeout_ms,
            &raw mut ops,
            &raw mut sample,
        ) != 0 as ::core::ffi::c_int
        {
            failures += 1;
        } else {
            sample.sampled_monotonic_ms = (*context).monotonic_ms;
            let mut growth: ::core::ffi::c_double = 0.0f64;
            if db_storage_target_recording_growth_bps(
                if (*slot).target_uuid[0usize] as ::core::ffi::c_int != 0 {
                    &raw mut (*slot).target_uuid as *mut ::core::ffi::c_char
                } else {
                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                },
                &raw mut growth,
            ) == 0 as ::core::ffi::c_int
            {
                sample.recording_growth_bps = growth;
                sample.recording_growth_capability =
                    system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
            }
            (*slot).sample = sample;
            (*slot).sample_valid = r#true != 0;
            if persist_target_sample(slot, (*state).write_probe_enabled, (*context).wall_time_ms)
                != 0 as ::core::ffi::c_int
            {
                failures += 1;
            }
        }
        attempt = attempt.wrapping_add(1);
    }
    let mut index_0: size_t = 0 as size_t;
    while index_0 < (*state).slot_count {
        let mut slot_0: *mut storage_target_health_slot_t =
            (&raw mut (*state).slots as *mut storage_target_health_slot_t).offset(index_0 as isize);
        if !(*slot_0).sample_valid {
            make_unavailable(
                &raw mut (*slot_0).sample,
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_STALE,
            );
            let mut logical_id: [::core::ffi::c_char; 64] = [0; 64];
            snprintf(
                &raw mut logical_id as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*slot_0).resource.logical_id as *mut ::core::ffi::c_char,
            );
            snprintf(
                &raw mut (*slot_0).sample.filesystem.logical_id as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut logical_id as *mut ::core::ffi::c_char,
            );
            snprintf(
                &raw mut (*slot_0).sample.probe.logical_id as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut logical_id as *mut ::core::ffi::c_char,
            );
        }
        let mut stale: bool = (*slot_0).sample.sampled_monotonic_ms == 0 as uint64_t
            || (*context).monotonic_ms < (*slot_0).sample.sampled_monotonic_ms
            || (*context)
                .monotonic_ms
                .wrapping_sub((*slot_0).sample.sampled_monotonic_ms)
                > (*state).stale_after_ms as uint64_t;
        let mut unique_device: bool = !device_seen_before(state, index_0);
        let mut canonical: bool =
            index_0 >= 2 as size_t && unique_device as ::core::ffi::c_int != 0;
        emit_storage_observations(sink, context, slot_0, stale, canonical, unique_device);
        index_0 = index_0.wrapping_add(1);
    }
    (*sink).dropped = (*sink).dropped.wrapping_add((*state).slots_dropped);
    return if failures == 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_target_health_collector_init(
    mut collector: *mut system_health_collector_t,
    mut state: *mut storage_target_health_collector_state_t,
) {
    if collector.is_null() {
        return;
    }
    memset(
        collector as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<system_health_collector_t>(),
    );
    snprintf(
        &raw mut (*collector).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage_targets\0".as_ptr() as *const ::core::ffi::c_char,
    );
    (*collector).scope = system_health_scope_t::SYSTEM_HEALTH_SCOPE_FILESYSTEM;
    (*collector).tier = system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_SLOW;
    (*collector).interval_seconds = 300 as ::core::ffi::c_uint as uint32_t;
    (*collector).stale_after_seconds = 900 as ::core::ffi::c_uint as uint32_t;
    (*collector).state = state as *mut ::core::ffi::c_void;
    (*collector).collect = Some(
        storage_target_health_collect
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const system_health_collect_context_t,
                *mut system_health_observation_sink_t,
            ) -> ::core::ffi::c_int,
    ) as system_health_collect_fn;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_target_health_register(
    mut root_path: *const ::core::ffi::c_char,
    mut recording_path: *const ::core::ffi::c_char,
    mut write_probe_enabled: bool,
) -> bool {
    if registered_state_initialized {
        return r#true != 0;
    }
    storage_target_health_collector_state_init(
        &raw mut registered_state,
        root_path,
        recording_path,
        write_probe_enabled,
    );
    let mut collector: system_health_collector_t = system_health_collector_t {
        name: [0; 32],
        scope: system_health_scope_t::SYSTEM_HEALTH_SCOPE_PROCESS,
        tier: system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_FAST,
        interval_seconds: 0,
        stale_after_seconds: 0,
        state: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        collect: None,
        destroy: None,
    };
    storage_target_health_collector_init(&raw mut collector, &raw mut registered_state);
    let mut result: bool = system_health_register_collector(&raw mut collector);
    if result {
        registered_state_initialized = r#true != 0;
    }
    return result;
}
unsafe extern "C" fn state_is(
    mut target: *const storage_target_t,
    mut state: *const ::core::ffi::c_char,
) -> bool {
    return !target.is_null()
        && strcmp(
            &raw const (*target).health_status as *const ::core::ffi::c_char,
            state,
        ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn normalized_previous_state(
    mut target: *const storage_target_t,
) -> *const ::core::ffi::c_char {
    if state_is(target, b"healthy\0".as_ptr() as *const ::core::ffi::c_char) {
        return b"healthy\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if state_is(target, b"degraded\0".as_ptr() as *const ::core::ffi::c_char) {
        return b"degraded\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn normalized_unavailable_reason(
    mut error: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if error.is_null() {
        return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcmp(
        error,
        b"mount_absent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return b"mount_unavailable\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcmp(error, b"not_found\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return b"directory_unavailable\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcmp(
        error,
        b"permission\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(error, b"read_only\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        return b"not_writable\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcmp(error, b"no_space\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcmp(error, b"io\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        || strcmp(error, b"timed_out\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(error, b"busy\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(error, b"invalid\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(error, b"other\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        return b"write_probe_failed\0".as_ptr() as *const ::core::ffi::c_char;
    }
    static mut missing_mount: [::core::ffi::c_char; 26] = unsafe {
        ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
            *b"Required mount is absent:\0",
        )
    };
    static mut no_mount: [::core::ffi::c_char; 31] = unsafe {
        ::core::mem::transmute::<[u8; 31], [::core::ffi::c_char; 31]>(
            *b"No distinct mounted filesystem\0",
        )
    };
    static mut directory: [::core::ffi::c_char; 33] = unsafe {
        ::core::mem::transmute::<[u8; 33], [::core::ffi::c_char; 33]>(
            *b"Target directory is unavailable:\0",
        )
    };
    static mut capacity: [::core::ffi::c_char; 23] = unsafe {
        ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"Capacity probe failed:\0")
    };
    static mut not_writable: [::core::ffi::c_char; 34] = unsafe {
        ::core::mem::transmute::<[u8; 34], [::core::ffi::c_char; 34]>(
            *b"Target directory is not writable:\0",
        )
    };
    static mut long_path: [::core::ffi::c_char; 23] = unsafe {
        ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"Probe path is too long\0")
    };
    static mut write_probe: [::core::ffi::c_char; 20] = unsafe {
        ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"Write probe failed:\0")
    };
    static mut cleanup: [::core::ffi::c_char; 22] = unsafe {
        ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"Probe cleanup failed:\0")
    };
    if strncmp(
        error,
        &raw const missing_mount as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 26]>().wrapping_sub(1 as size_t),
    ) == 0 as ::core::ffi::c_int
        || strncmp(
            error,
            &raw const no_mount as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 31]>().wrapping_sub(1 as size_t),
        ) == 0 as ::core::ffi::c_int
    {
        return b"mount_unavailable\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strncmp(
        error,
        &raw const directory as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 33]>().wrapping_sub(1 as size_t),
    ) == 0 as ::core::ffi::c_int
    {
        return b"directory_unavailable\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strncmp(
        error,
        &raw const capacity as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 23]>().wrapping_sub(1 as size_t),
    ) == 0 as ::core::ffi::c_int
    {
        return b"capacity_probe_failed\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strncmp(
        error,
        &raw const not_writable as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 34]>().wrapping_sub(1 as size_t),
    ) == 0 as ::core::ffi::c_int
    {
        return b"not_writable\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strncmp(
        error,
        &raw const long_path as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 23]>().wrapping_sub(1 as size_t),
    ) == 0 as ::core::ffi::c_int
        || strncmp(
            error,
            &raw const write_probe as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 20]>().wrapping_sub(1 as size_t),
        ) == 0 as ::core::ffi::c_int
    {
        return b"write_probe_failed\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strncmp(
        error,
        &raw const cleanup as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 22]>().wrapping_sub(1 as size_t),
    ) == 0 as ::core::ffi::c_int
    {
        return b"probe_cleanup_failed\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn downtime_milliseconds(
    mut before: *const storage_target_t,
    mut after: *const storage_target_t,
) -> int64_t {
    if before.is_null()
        || after.is_null()
        || (*before).last_success_at <= 0 as int64_t
        || (*after).last_probe_at <= (*before).last_success_at
    {
        return 0 as int64_t;
    }
    let mut seconds: int64_t = (*after).last_probe_at - (*before).last_success_at;
    return if seconds > INT64_MAX as int64_t / 1000 as int64_t {
        INT64_MAX as int64_t
    } else {
        seconds * 1000 as int64_t
    };
}
unsafe extern "C" fn publish_transition(
    mut before: *const storage_target_t,
    mut after: *const storage_target_t,
) {
    let mut was_unavailable: bool = state_is(
        before,
        b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut is_unavailable: bool = state_is(
        after,
        b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut is_recovered: bool =
        state_is(after, b"healthy\0".as_ptr() as *const ::core::ffi::c_char) as ::core::ffi::c_int
            != 0
            || state_is(after, b"degraded\0".as_ptr() as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0;
    let mut occurred_at: time_t = if (*after).last_probe_at > 0 as int64_t {
        (*after).last_probe_at as time_t
    } else {
        time(::core::ptr::null_mut::<time_t>())
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
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !was_unavailable && is_unavailable as ::core::ffi::c_int != 0 {
        result = event_producer_publish_storage_target_unavailable(
            &raw const (*after).uuid as *const ::core::ffi::c_char,
            normalized_previous_state(before),
            normalized_unavailable_reason(
                &raw const (*after).last_error as *const ::core::ffi::c_char,
            ),
            (*after).is_default,
            occurred_at,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        );
    } else if was_unavailable as ::core::ffi::c_int != 0 && is_recovered as ::core::ffi::c_int != 0
    {
        result = event_producer_publish_storage_target_recovered(
            &raw const (*after).uuid as *const ::core::ffi::c_char,
            &raw const (*after).health_status as *const ::core::ffi::c_char,
            downtime_milliseconds(before, after),
            (*after).is_default,
            occurred_at,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        );
    } else {
        return;
    }
    if result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to enqueue storage target transition event for %s: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*after).uuid as *const ::core::ffi::c_char,
            if error[0usize] as ::core::ffi::c_int != 0 {
                &raw mut error as *mut ::core::ffi::c_char
            } else {
                b"event pipeline unavailable\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_target_probe_and_publish(
    mut uuid: *const ::core::ffi::c_char,
    mut write_test: bool,
    mut target: *mut storage_target_t,
) -> db_storage_target_result_t {
    let mut before: storage_target_t = storage_target_t {
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
    let mut after: storage_target_t = storage_target_t {
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
    memset(
        &raw mut before as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_target_t>(),
    );
    memset(
        &raw mut after as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_target_t>(),
    );
    pthread_mutex_lock(&raw mut target_probe_mutex);
    let mut result: db_storage_target_result_t = db_storage_target_get(uuid, &raw mut before);
    if result.0 == db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        result = db_storage_target_probe(uuid, write_test, &raw mut after);
        if result.0 == db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
            || result.0 == db_storage_target_result_t::DB_STORAGE_TARGET_UNAVAILABLE.0
        {
            publish_transition(&raw mut before, &raw mut after);
            if !target.is_null() {
                *target = after;
            }
        }
    }
    pthread_mutex_unlock(&raw mut target_probe_mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_target_refresh_health_and_publish() -> ::core::ffi::c_int {
    let mut total: ::core::ffi::c_int = db_storage_target_count();
    if total < 0 as ::core::ffi::c_int || total > STORAGE_TARGET_MAX_COUNT {
        return -1 as ::core::ffi::c_int;
    }
    if total == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    system_health_request_tier(system_health_sampling_tier_t::SYSTEM_HEALTH_TIER_SLOW);
    return total;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
