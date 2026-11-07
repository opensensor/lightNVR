use ::f128;
use ::libc;
unsafe extern "C" {
    pub type sqlite3_stmt;
    pub type sqlite3;
    pub type __dirstream;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
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
    unsafe fn dirname(__path: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn mbedtls_sha256_init(ctx: *mut mbedtls_sha256_context);
    unsafe fn mbedtls_sha256_free(ctx: *mut mbedtls_sha256_context);
    unsafe fn mbedtls_sha256_starts(
        ctx: *mut mbedtls_sha256_context,
        is224: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_sha256_update(
        ctx: *mut mbedtls_sha256_context,
        input: *const ::core::ffi::c_uchar,
        ilen: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_sha256_finish(
        ctx: *mut mbedtls_sha256_context,
        output: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
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
    unsafe fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn fstatat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __buf: *mut stat,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn mkdirat(
        __fd: ::core::ffi::c_int,
        __path: *const ::core::ffi::c_char,
        __mode: __mode_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lseek(
        __fd: ::core::ffi::c_int,
        __offset: __off_t,
        __whence: ::core::ffi::c_int,
    ) -> __off_t;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    unsafe fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    unsafe fn geteuid() -> __uid_t;
    unsafe fn link(
        __from: *const ::core::ffi::c_char,
        __to: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn unlinkat(
        __fd: ::core::ffi::c_int,
        __name: *const ::core::ffi::c_char,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn syscall(__sysno: ::core::ffi::c_long, ...) -> ::core::ffi::c_long;
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
    unsafe fn ensure_path(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn db_storage_lifecycle_schedule(max_jobs: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn db_storage_lifecycle_expire(max_recordings: ::core::ffi::c_int)
    -> ::core::ffi::c_int;
    unsafe fn db_storage_lifecycle_reconcile() -> ::core::ffi::c_int;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
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
    unsafe fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn db_storage_target_count() -> ::core::ffi::c_int;
    unsafe fn db_storage_target_list(
        targets: *mut storage_target_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    unsafe fn db_storage_migration_get(
        uuid: *const ::core::ffi::c_char,
        job: *mut storage_migration_job_t,
    ) -> db_storage_migration_result_t;
    unsafe fn db_storage_migration_claim_due(
        job: *mut storage_migration_job_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_storage_migration_update_progress(
        uuid: *const ::core::ffi::c_char,
        state: *const ::core::ffi::c_char,
        bytes_copied: uint64_t,
        bytes_total: uint64_t,
    ) -> db_storage_migration_result_t;
    unsafe fn db_storage_migration_record_failure(
        job: *const storage_migration_job_t,
        error: *const ::core::ffi::c_char,
        retryable: bool,
    ) -> db_storage_migration_result_t;
    unsafe fn db_storage_migration_defer_source(
        uuid: *const ::core::ffi::c_char,
    ) -> db_storage_migration_result_t;
    unsafe fn db_storage_migration_commit_location(
        job: *const storage_migration_job_t,
        destination_path: *const ::core::ffi::c_char,
        checksum: *const ::core::ffi::c_char,
    ) -> db_storage_migration_result_t;
    unsafe fn db_storage_migration_complete_cleanup(
        uuid: *const ::core::ffi::c_char,
    ) -> db_storage_migration_result_t;
    unsafe fn db_storage_migration_defer_cleanup(
        job: *const storage_migration_job_t,
        error: *const ::core::ffi::c_char,
    ) -> db_storage_migration_result_t;
    unsafe fn db_storage_migration_commit_copy(
        job: *const storage_migration_job_t,
        checksum: *const ::core::ffi::c_char,
    ) -> db_storage_migration_result_t;
    unsafe fn db_storage_migration_mark_cancelled(
        uuid: *const ::core::ffi::c_char,
        job: *mut storage_migration_job_t,
    ) -> db_storage_migration_result_t;
    unsafe fn db_storage_migration_cancel_requested(uuid: *const ::core::ffi::c_char) -> bool;
    unsafe fn storage_target_probe_and_publish(
        uuid: *const ::core::ffi::c_char,
        write_test: bool,
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
    unsafe fn storage_s3_probe(
        target: *mut storage_target_t,
        write_test: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn storage_s3_stat(
        target: *const storage_target_t,
        key: *const ::core::ffi::c_char,
        size: *mut uint64_t,
        error: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn storage_s3_upload(
        target: *const storage_target_t,
        key: *const ::core::ffi::c_char,
        path: *const ::core::ffi::c_char,
        checksum: *const ::core::ffi::c_char,
        control: *const storage_transfer_control_t,
        error: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn storage_s3_download(
        target: *const storage_target_t,
        key: *const ::core::ffi::c_char,
        path: *const ::core::ffi::c_char,
        expected_size: uint64_t,
        expected_checksum: *const ::core::ffi::c_char,
        control: *const storage_transfer_control_t,
        error: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn storage_s3_verify(
        target: *const storage_target_t,
        key: *const ::core::ffi::c_char,
        size: uint64_t,
        checksum: *const ::core::ffi::c_char,
        control: *const storage_transfer_control_t,
        error: *mut ::core::ffi::c_char,
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
    unsafe fn storage_deletion_process_one() -> ::core::ffi::c_int;
    unsafe fn storage_source_resolve(
        recording_id: uint64_t,
        path: *mut ::core::ffi::c_char,
        error: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn storage_source_has_lease(recording_id: uint64_t) -> bool;
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
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
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
pub struct migration_worker_control_t {
    pub mutex: pthread_mutex_t,
    pub condition: pthread_cond_t,
    pub thread: pthread_t,
    pub running: bool,
    pub exited: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct migration_publication_t {
    pub fd: ::core::ffi::c_int,
    pub parent: ::core::ffi::c_int,
    pub identity: stat,
    pub parent_identity: stat,
    pub parent_path: [::core::ffi::c_char; 512],
    pub leaf: [::core::ffi::c_char; 512],
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
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_migration_job_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub recording_id: uint64_t,
    pub owner_user_id: int64_t,
    pub operation: [::core::ffi::c_char; 8],
    pub source_target_uuid: [::core::ffi::c_char; 37],
    pub source_object_key: [::core::ffi::c_char; 512],
    pub destination_target_uuid: [::core::ffi::c_char; 37],
    pub destination_object_key: [::core::ffi::c_char; 512],
    pub state: [::core::ffi::c_char; 24],
    pub checksum: [::core::ffi::c_char; 65],
    pub bytes_total: uint64_t,
    pub bytes_copied: uint64_t,
    pub attempt_count: ::core::ffi::c_int,
    pub max_attempts: ::core::ffi::c_int,
    pub next_attempt_at: int64_t,
    pub last_error: [::core::ffi::c_char; 256],
    pub cancel_requested: bool,
    pub bandwidth_limit_bps: uint64_t,
    pub window_start_minute: ::core::ffi::c_int,
    pub window_end_minute: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub started_at: int64_t,
    pub completed_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_storage_migration_result_t(pub ::core::ffi::c_int);
impl db_storage_migration_result_t {
    pub const DB_STORAGE_MIGRATION_OK: Self = Self(0);
    pub const DB_STORAGE_MIGRATION_NOT_FOUND: Self = Self(-1);
    pub const DB_STORAGE_MIGRATION_CONFLICT: Self = Self(-2);
    pub const DB_STORAGE_MIGRATION_INVALID: Self = Self(-3);
    pub const DB_STORAGE_MIGRATION_SOURCE_INCOMPLETE: Self = Self(-4);
    pub const DB_STORAGE_MIGRATION_TARGET_UNAVAILABLE: Self = Self(-5);
    pub const DB_STORAGE_MIGRATION_SOURCE_CHANGED: Self = Self(-6);
    pub const DB_STORAGE_MIGRATION_ERROR: Self = Self(-7);
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
pub struct mbedtls_sha256_context {
    pub private_buffer: [::core::ffi::c_uchar; 64],
    pub private_total: [uint32_t; 2],
    pub private_state: [uint32_t; 8],
    pub private_is224: ::core::ffi::c_int,
}
pub type uint32_t = u32;
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
pub type atomic_bool = bool;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_1 {
    pub const STORAGE_S3_OK: Self = Self(0);
    pub const STORAGE_S3_ERROR: Self = Self(-1);
    pub const STORAGE_S3_MISSING: Self = Self(-2);
    pub const STORAGE_S3_CANCELLED: Self = Self(-3);
    pub const STORAGE_S3_CONFLICT: Self = Self(-4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_2 {
    pub const STORAGE_SOURCE_READY: Self = Self(0);
    pub const STORAGE_SOURCE_PREPARING: Self = Self(1);
    pub const STORAGE_SOURCE_ERROR: Self = Self(-1);
    pub const STORAGE_SOURCE_MISSING: Self = Self(-2);
    pub const STORAGE_SOURCE_DELETING: Self = Self(-3);
}
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __O_DIRECTORY: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_DIRECTORY: ::core::ffi::c_int = __O_DIRECTORY;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const __NR_renameat2: ::core::ffi::c_int = 316 as ::core::ffi::c_int;
pub const AT_SYMLINK_NOFOLLOW: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const AT_REMOVEDIR: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
pub const SYS_renameat2: ::core::ffi::c_int = __NR_renameat2;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const STORAGE_TARGET_MAX_COUNT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const STORAGE_MIGRATION_CHECKSUM_MAX: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const STORAGE_MIGRATION_ERROR_MAX: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MIGRATION_COPY_BUFFER: ::core::ffi::c_uint =
    (256 as ::core::ffi::c_uint).wrapping_mul(1024 as ::core::ffi::c_uint);
pub const MIGRATION_PROGRESS_INTERVAL: ::core::ffi::c_ulonglong = (4 as ::core::ffi::c_ulonglong)
    .wrapping_mul(1024 as ::core::ffi::c_ulonglong)
    .wrapping_mul(1024 as ::core::ffi::c_ulonglong);
static mut worker: migration_worker_control_t = migration_worker_control_t {
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
    condition: PTHREAD_COND_INITIALIZER,
    thread: 0,
    running: r#false != 0,
    exited: r#true != 0,
};
static mut stop_requested: atomic_bool = r#false != 0;
unsafe extern "C" fn migration_cancel_requested(mut job: *const storage_migration_job_t) -> bool {
    return !job.is_null()
        && db_storage_migration_cancel_requested(
            &raw const (*job).uuid as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn transfer_cancelled(mut context: *mut ::core::ffi::c_void) -> bool {
    return ::core::sync::atomic::AtomicBool::from_ptr(&raw mut stop_requested).load(::core::sync::atomic::Ordering::SeqCst) as ::core::ffi::c_int
        != 0
        || migration_cancel_requested(context as *const storage_migration_job_t)
            as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn throttle_copy(
    mut job: *const storage_migration_job_t,
    mut started: *const timespec,
    mut copied: uint64_t,
) -> ::core::ffi::c_int {
    if job.is_null() || (*job).bandwidth_limit_bps == 0 as uint64_t || started.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_MONOTONIC, &raw mut now) != 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut expected_ns: ::f128::f128 = ::f128::f128::new(copied) * ::f128::f128::new(1000000000.0)
        / ::f128::f128::new((*job).bandwidth_limit_bps);
    let mut elapsed_ns: int64_t = ((now.tv_sec - (*started).tv_sec) as int64_t
        as ::core::ffi::c_longlong
        * 1000000000 as ::core::ffi::c_longlong
        + (now.tv_nsec - (*started).tv_nsec) as int64_t as ::core::ffi::c_longlong)
        as int64_t;
    if expected_ns <= ::f128::f128::new(elapsed_ns) {
        return 0 as ::core::ffi::c_int;
    }
    let mut wait_ns: int64_t = Into::<int64_t>::into(expected_ns - ::f128::f128::new(elapsed_ns));
    let mut delay: timespec = timespec {
        tv_sec: (wait_ns as ::core::ffi::c_longlong / 1000000000 as ::core::ffi::c_longlong)
            as __time_t,
        tv_nsec: (wait_ns as ::core::ffi::c_longlong % 1000000000 as ::core::ffi::c_longlong)
            as __syscall_slong_t,
    };
    while nanosleep(&raw mut delay, &raw mut delay) != 0 as ::core::ffi::c_int
        && *__errno_location() == EINTR
    {
        if ::core::sync::atomic::AtomicBool::from_ptr(&raw mut stop_requested).load(::core::sync::atomic::Ordering::SeqCst) {
            return -1 as ::core::ffi::c_int;
        }
        if migration_cancel_requested(job) {
            return 1 as ::core::ffi::c_int;
        }
    }
    return if migration_cancel_requested(job) as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn set_error(
    mut error: *mut ::core::ffi::c_char,
    mut operation: *const ::core::ffi::c_char,
    mut path: *const ::core::ffi::c_char,
) {
    let mut saved_errno: ::core::ffi::c_int = *__errno_location();
    snprintf(
        error,
        STORAGE_MIGRATION_ERROR_MAX as size_t,
        b"%s%s%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
        operation,
        if !path.is_null() {
            b" \0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !path.is_null() {
            path
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        strerror(saved_errno),
    );
}
unsafe extern "C" fn digest_hex(
    mut digest: *const ::core::ffi::c_uchar,
    mut output: *mut ::core::ffi::c_char,
) {
    static mut symbols: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"0123456789abcdef\0")
    };
    let mut index: size_t = 0 as size_t;
    while index < 32 as size_t {
        *output.offset(index.wrapping_mul(2 as size_t) as isize) =
            symbols[(*digest.offset(index as isize) as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as usize];
        *output.offset(index.wrapping_mul(2 as size_t).wrapping_add(1 as size_t) as isize) =
            symbols[(*digest.offset(index as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as usize];
        index = index.wrapping_add(1);
    }
    *output.offset(64isize) = '\0' as ::core::ffi::c_char;
}
unsafe extern "C" fn finish_digest(
    mut context: *mut mbedtls_sha256_context,
    mut output: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut digest: [::core::ffi::c_uchar; 32] = [0; 32];
    let mut result: ::core::ffi::c_int =
        mbedtls_sha256_finish(context, &raw mut digest as *mut ::core::ffi::c_uchar);
    mbedtls_sha256_free(context);
    if result != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    digest_hex(&raw mut digest as *mut ::core::ffi::c_uchar, output);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn sha256_descriptor(
    mut descriptor: ::core::ffi::c_int,
    mut path: *const ::core::ffi::c_char,
    mut size: *mut uint64_t,
    mut checksum: *mut ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if lseek(descriptor, 0 as __off_t, SEEK_SET) < 0 as __off_t {
        set_error(
            error,
            b"Cannot seek\0".as_ptr() as *const ::core::ffi::c_char,
            path,
        );
        return -1 as ::core::ffi::c_int;
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
    if fstat(descriptor, &raw mut status) != 0 as ::core::ffi::c_int
        || !(status.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
        || status.st_size < 0 as __off_t
    {
        if *__errno_location() == 0 as ::core::ffi::c_int {
            *__errno_location() = EINVAL;
        }
        set_error(
            error,
            b"Not a readable regular file\0".as_ptr() as *const ::core::ffi::c_char,
            path,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut context: mbedtls_sha256_context = mbedtls_sha256_context {
        private_buffer: [0; 64],
        private_total: [0; 2],
        private_state: [0; 8],
        private_is224: 0,
    };
    mbedtls_sha256_init(&raw mut context);
    if mbedtls_sha256_starts(&raw mut context, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int {
        mbedtls_sha256_free(&raw mut context);
        safe_strcpy(
            error,
            b"Could not initialize SHA-256\0".as_ptr() as *const ::core::ffi::c_char,
            STORAGE_MIGRATION_ERROR_MAX as size_t,
            0 as size_t,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut buffer: *mut ::core::ffi::c_uchar =
        malloc(MIGRATION_COPY_BUFFER as size_t) as *mut ::core::ffi::c_uchar;
    if buffer.is_null() {
        mbedtls_sha256_free(&raw mut context);
        safe_strcpy(
            error,
            b"Out of memory while verifying recording\0".as_ptr() as *const ::core::ffi::c_char,
            STORAGE_MIGRATION_ERROR_MAX as size_t,
            0 as size_t,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut total: uint64_t = 0 as uint64_t;
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let mut count: ssize_t = read(
            descriptor,
            buffer as *mut ::core::ffi::c_void,
            MIGRATION_COPY_BUFFER as size_t,
        );
        if count == 0 as ssize_t {
            break;
        }
        if count < 0 as ssize_t {
            if *__errno_location() == EINTR {
                continue;
            }
            set_error(
                error,
                b"Cannot read\0".as_ptr() as *const ::core::ffi::c_char,
                path,
            );
            result = -1 as ::core::ffi::c_int;
            break;
        } else if mbedtls_sha256_update(&raw mut context, buffer, count as size_t)
            != 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                error,
                b"SHA-256 update failed\0".as_ptr() as *const ::core::ffi::c_char,
                STORAGE_MIGRATION_ERROR_MAX as size_t,
                0 as size_t,
            );
            result = -1 as ::core::ffi::c_int;
            break;
        } else {
            total = total.wrapping_add(count as uint64_t);
        }
    }
    free(buffer as *mut ::core::ffi::c_void);
    if result == 0 as ::core::ffi::c_int
        && finish_digest(&raw mut context, checksum) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            error,
            b"Could not finish SHA-256\0".as_ptr() as *const ::core::ffi::c_char,
            STORAGE_MIGRATION_ERROR_MAX as size_t,
            0 as size_t,
        );
        result = -1 as ::core::ffi::c_int;
    } else if result != 0 as ::core::ffi::c_int {
        mbedtls_sha256_free(&raw mut context);
    }
    if result == 0 as ::core::ffi::c_int && !size.is_null() {
        *size = total;
    }
    return result;
}
unsafe extern "C" fn sha256_file(
    mut path: *const ::core::ffi::c_char,
    mut size: *mut uint64_t,
    mut checksum: *mut ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut descriptor: ::core::ffi::c_int = open(path, O_RDONLY | O_CLOEXEC | O_NOFOLLOW);
    if descriptor < 0 as ::core::ffi::c_int {
        set_error(
            error,
            b"Cannot open\0".as_ptr() as *const ::core::ffi::c_char,
            path,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = sha256_descriptor(descriptor, path, size, checksum, error);
    close(descriptor);
    return result;
}
unsafe extern "C" fn write_all(
    mut descriptor: ::core::ffi::c_int,
    mut buffer: *const ::core::ffi::c_uchar,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut offset: size_t = 0 as size_t;
    while offset < length {
        let mut written: ssize_t = write(
            descriptor,
            buffer.offset(offset as isize) as *const ::core::ffi::c_void,
            length.wrapping_sub(offset),
        );
        if written < 0 as ssize_t {
            if *__errno_location() == EINTR {
                continue;
            }
            return -1 as ::core::ffi::c_int;
        } else {
            if written == 0 as ssize_t {
                *__errno_location() = EIO;
                return -1 as ::core::ffi::c_int;
            }
            offset = offset.wrapping_add(written as size_t);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn fsync_parent(mut path: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut buffer: [::core::ffi::c_char; 512] = [0; 512];
    safe_strcpy(
        &raw mut buffer as *mut ::core::ffi::c_char,
        path,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    let mut parent: *mut ::core::ffi::c_char = dirname(&raw mut buffer as *mut ::core::ffi::c_char);
    let mut descriptor: ::core::ffi::c_int =
        open(parent, O_RDONLY | O_DIRECTORY | O_CLOEXEC | O_NOFOLLOW);
    if descriptor < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = fsync(descriptor);
    close(descriptor);
    return result;
}
unsafe extern "C" fn verified_existing_destination(
    mut source_path: *const ::core::ffi::c_char,
    mut destination_path: *const ::core::ffi::c_char,
    mut bytes_total: *mut uint64_t,
    mut checksum: *mut ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut destination_status: stat = stat {
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
    if lstat(destination_path, &raw mut destination_status) != 0 as ::core::ffi::c_int {
        if *__errno_location() == ENOENT {
            return 0 as ::core::ffi::c_int;
        }
        set_error(
            error,
            b"Cannot inspect destination\0".as_ptr() as *const ::core::ffi::c_char,
            destination_path,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !(destination_status.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t) {
        safe_strcpy(
            error,
            b"Destination exists and is not a regular file\0".as_ptr()
                as *const ::core::ffi::c_char,
            STORAGE_MIGRATION_ERROR_MAX as size_t,
            0 as size_t,
        );
        return -2 as ::core::ffi::c_int;
    }
    let mut source_checksum: [::core::ffi::c_char; 65] = [0; 65];
    let mut destination_checksum: [::core::ffi::c_char; 65] = [0; 65];
    let mut source_size: uint64_t = 0 as uint64_t;
    let mut destination_size: uint64_t = 0 as uint64_t;
    if sha256_file(
        source_path,
        &raw mut source_size,
        &raw mut source_checksum as *mut ::core::ffi::c_char,
        error,
    ) != 0 as ::core::ffi::c_int
        || sha256_file(
            destination_path,
            &raw mut destination_size,
            &raw mut destination_checksum as *mut ::core::ffi::c_char,
            error,
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if source_size != destination_size
        || strcmp(
            &raw mut source_checksum as *mut ::core::ffi::c_char,
            &raw mut destination_checksum as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            error,
            b"Destination path already contains different data\0".as_ptr()
                as *const ::core::ffi::c_char,
            STORAGE_MIGRATION_ERROR_MAX as size_t,
            0 as size_t,
        );
        return -2 as ::core::ffi::c_int;
    }
    *bytes_total = source_size;
    safe_strcpy(
        checksum,
        &raw mut source_checksum as *mut ::core::ffi::c_char,
        STORAGE_MIGRATION_CHECKSUM_MAX as size_t,
        0 as size_t,
    );
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn same_inode(mut a: *const stat, mut b: *const stat) -> bool {
    return (*a).st_dev == (*b).st_dev && (*a).st_ino == (*b).st_ino;
}
unsafe extern "C" fn same_verified_file(mut a: *const stat, mut b: *const stat) -> bool {
    return same_inode(a, b) as ::core::ffi::c_int != 0
        && (*b).st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
        && (*a).st_size == (*b).st_size
        && (*a).st_mtim.tv_sec == (*b).st_mtim.tv_sec
        && (*a).st_mtim.tv_nsec == (*b).st_mtim.tv_nsec;
}
unsafe extern "C" fn release_publication(mut publication: *mut migration_publication_t) {
    if (*publication).fd >= 0 as ::core::ffi::c_int {
        close((*publication).fd);
    }
    if (*publication).parent >= 0 as ::core::ffi::c_int {
        close((*publication).parent);
    }
    (*publication).parent = -1 as ::core::ffi::c_int;
    (*publication).fd = (*publication).parent;
}
unsafe extern "C" fn publication_still_verified(
    mut publication: *const migration_publication_t,
) -> bool {
    if (*publication).fd < 0 as ::core::ffi::c_int {
        return r#true != 0;
    }
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
    let mut parent: stat = stat {
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
    return fstat((*publication).fd, &raw mut held) == 0 as ::core::ffi::c_int
        && same_verified_file(&raw const (*publication).identity, &raw mut held)
            as ::core::ffi::c_int
            != 0
        && fstatat(
            (*publication).parent,
            &raw const (*publication).leaf as *const ::core::ffi::c_char,
            &raw mut named,
            AT_SYMLINK_NOFOLLOW,
        ) == 0 as ::core::ffi::c_int
        && same_verified_file(&raw const (*publication).identity, &raw mut named)
            as ::core::ffi::c_int
            != 0
        && lstat(
            &raw const (*publication).parent_path as *const ::core::ffi::c_char,
            &raw mut parent,
        ) == 0 as ::core::ffi::c_int
        && same_inode(&raw const (*publication).parent_identity, &raw mut parent)
            as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn move_migration_noreplace(
    mut from: ::core::ffi::c_int,
    mut old_name: *const ::core::ffi::c_char,
    mut to: ::core::ffi::c_int,
    mut new_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return syscall(
        SYS_renameat2 as ::core::ffi::c_long,
        from,
        old_name,
        to,
        new_name,
        1 as ::core::ffi::c_uint,
    ) as ::core::ffi::c_int;
}
unsafe extern "C" fn publish_verified_temporary(
    mut descriptor: ::core::ffi::c_int,
    mut verified: *const stat,
    mut temporary: *const ::core::ffi::c_char,
    mut destination: *const ::core::ffi::c_char,
    mut job_uuid: *const ::core::ffi::c_char,
    mut publication: *mut migration_publication_t,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut parent_path: [::core::ffi::c_char; 512] = [0; 512];
    let mut private_name: [::core::ffi::c_char; 512] = [
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
    safe_strcpy(
        &raw mut parent_path as *mut ::core::ffi::c_char,
        destination,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    let mut parent_name: *mut ::core::ffi::c_char =
        dirname(&raw mut parent_path as *mut ::core::ffi::c_char);
    let mut destination_leaf: *const ::core::ffi::c_char =
        strrchr(destination, '/' as ::core::ffi::c_int);
    destination_leaf = if !destination_leaf.is_null() {
        destination_leaf.offset(1 as ::core::ffi::c_int as isize)
    } else {
        destination
    };
    let mut temporary_leaf: *const ::core::ffi::c_char =
        strrchr(temporary, '/' as ::core::ffi::c_int);
    temporary_leaf = if !temporary_leaf.is_null() {
        temporary_leaf.offset(1 as ::core::ffi::c_int as isize)
    } else {
        temporary
    };
    let mut parent: ::core::ffi::c_int =
        open(parent_name, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
    let mut private_fd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut private_created: bool = r#false != 0;
    let mut identity_known: bool = r#false != 0;
    let mut retained: bool = r#false != 0;
    let mut directory_identity: stat = stat {
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
    '_finish: {
        if !(parent < 0 as ::core::ffi::c_int
            || fstat(parent, &raw mut parent_identity) != 0 as ::core::ffi::c_int)
        {
            let mut attempt: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while attempt < 4 as ::core::ffi::c_uint {
                let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
                if lightnvr_uuid_generate_v4(&raw mut uuid as *mut ::core::ffi::c_char)
                    != 0 as ::core::ffi::c_int
                {
                    break '_finish;
                }
                let mut length: ::core::ffi::c_int = snprintf(
                    &raw mut private_name as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    b"%s.migration-%s.publish-%s\0".as_ptr() as *const ::core::ffi::c_char,
                    destination_leaf,
                    job_uuid,
                    &raw mut uuid as *mut ::core::ffi::c_char,
                );
                if length < 0 as ::core::ffi::c_int
                    || length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
                {
                    *__errno_location() = ENAMETOOLONG;
                    break '_finish;
                } else if mkdirat(
                    parent,
                    &raw mut private_name as *mut ::core::ffi::c_char,
                    0o700 as __mode_t,
                ) == 0 as ::core::ffi::c_int
                {
                    private_created = r#true != 0;
                    break;
                } else {
                    if *__errno_location() != EEXIST {
                        break '_finish;
                    }
                    attempt = attempt.wrapping_add(1);
                }
            }
            if !(!private_created
                || fstatat(
                    parent,
                    &raw mut private_name as *mut ::core::ffi::c_char,
                    &raw mut directory_identity,
                    AT_SYMLINK_NOFOLLOW,
                ) != 0 as ::core::ffi::c_int)
            {
                identity_known = r#true != 0;
                if !(directory_identity.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
                    || directory_identity.st_uid != geteuid()
                    || directory_identity.st_mode & 0o777 as __mode_t != 0o700 as __mode_t
                {
                    *__errno_location() = EPERM;
                } else {
                    private_fd = openat(
                        parent,
                        &raw mut private_name as *mut ::core::ffi::c_char,
                        O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC,
                    );
                    if !(private_fd < 0 as ::core::ffi::c_int
                        || fstat(private_fd, &raw mut current) != 0 as ::core::ffi::c_int
                        || !same_inode(&raw mut directory_identity, &raw mut current))
                    {
                        if move_migration_noreplace(
                            parent,
                            temporary_leaf,
                            private_fd,
                            b"verified\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            retained = r#true != 0;
                            if fstatat(
                                private_fd,
                                b"verified\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut current,
                                AT_SYMLINK_NOFOLLOW,
                            ) != 0 as ::core::ffi::c_int
                                || !same_verified_file(verified, &raw mut current)
                            {
                                safe_strcpy(
                                    error,
                                    b"Temporary identity changed after verification; retained\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    STORAGE_MIGRATION_ERROR_MAX as size_t,
                                    0 as size_t,
                                );
                                result = -2 as ::core::ffi::c_int;
                            } else if move_migration_noreplace(
                                private_fd,
                                b"verified\0".as_ptr() as *const ::core::ffi::c_char,
                                parent,
                                destination_leaf,
                            ) != 0 as ::core::ffi::c_int
                            {
                                if *__errno_location() == EEXIST {
                                    safe_strcpy(
                                        error,
                                        b"Destination appeared while migration was copying\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        STORAGE_MIGRATION_ERROR_MAX as size_t,
                                        0 as size_t,
                                    );
                                    result = -2 as ::core::ffi::c_int;
                                } else {
                                    set_error(
                                        error,
                                        b"Cannot publish verified destination\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        destination,
                                    );
                                }
                            } else {
                                retained = r#false != 0;
                                if fsync(parent) != 0 as ::core::ffi::c_int {
                                    set_error(
                                        error,
                                        b"Cannot sync published destination directory\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        destination,
                                    );
                                    break '_finish;
                                } else {
                                    (*publication).fd = descriptor;
                                    (*publication).parent = parent;
                                    (*publication).identity = *verified;
                                    (*publication).parent_identity = parent_identity;
                                    safe_strcpy(
                                        &raw mut (*publication).parent_path
                                            as *mut ::core::ffi::c_char,
                                        parent_name,
                                        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                                        0 as size_t,
                                    );
                                    safe_strcpy(
                                        &raw mut (*publication).leaf as *mut ::core::ffi::c_char,
                                        destination_leaf,
                                        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                                        0 as size_t,
                                    );
                                    result = 0 as ::core::ffi::c_int;
                                    break '_finish;
                                }
                            }
                            if move_migration_noreplace(
                                private_fd,
                                b"verified\0".as_ptr() as *const ::core::ffi::c_char,
                                parent,
                                temporary_leaf,
                            ) == 0 as ::core::ffi::c_int
                            {
                                retained = r#false != 0;
                            }
                        }
                    }
                }
            }
        }
    }
    if result != 0 as ::core::ffi::c_int && *error.offset(0isize) == 0 {
        set_error(
            error,
            b"Cannot bind verified temporary\0".as_ptr() as *const ::core::ffi::c_char,
            temporary,
        );
    }
    if private_created {
        if retained as ::core::ffi::c_int != 0
            || !identity_known
            || fstatat(
                parent,
                &raw mut private_name as *mut ::core::ffi::c_char,
                &raw mut current,
                AT_SYMLINK_NOFOLLOW,
            ) != 0 as ::core::ffi::c_int
            || !same_inode(&raw mut directory_identity, &raw mut current)
            || unlinkat(
                parent,
                &raw mut private_name as *mut ::core::ffi::c_char,
                AT_REMOVEDIR,
            ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Migration private artifact retained for inspection: %s/%s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                parent_name,
                &raw mut private_name as *mut ::core::ffi::c_char,
            );
        }
    }
    if private_fd >= 0 as ::core::ffi::c_int {
        close(private_fd);
    }
    if result != 0 as ::core::ffi::c_int && parent >= 0 as ::core::ffi::c_int {
        close(parent);
    }
    return result;
}
unsafe extern "C" fn copy_and_verify(
    mut job: *mut storage_migration_job_t,
    mut source_path: *const ::core::ffi::c_char,
    mut destination_path: *const ::core::ffi::c_char,
    mut publication: *mut migration_publication_t,
    mut checksum: *mut ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut existing: ::core::ffi::c_int = verified_existing_destination(
        source_path,
        destination_path,
        &raw mut (*job).bytes_total,
        checksum,
        error,
    );
    if existing != 0 as ::core::ffi::c_int {
        return if existing > 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            existing
        };
    }
    let mut temporary_path: [::core::ffi::c_char; 512] = [0; 512];
    let mut length: ::core::ffi::c_int = snprintf(
        &raw mut temporary_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s.migration-%s.part\0".as_ptr() as *const ::core::ffi::c_char,
        destination_path,
        &raw mut (*job).uuid as *mut ::core::ffi::c_char,
    );
    if length < 0 as ::core::ffi::c_int
        || length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
    {
        safe_strcpy(
            error,
            b"Destination path is too long for migration\0".as_ptr() as *const ::core::ffi::c_char,
            STORAGE_MIGRATION_ERROR_MAX as size_t,
            0 as size_t,
        );
        return -2 as ::core::ffi::c_int;
    }
    if ensure_path(&raw mut temporary_path as *mut ::core::ffi::c_char) != 0 as ::core::ffi::c_int {
        set_error(
            error,
            b"Cannot create destination directory for\0".as_ptr() as *const ::core::ffi::c_char,
            destination_path,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut source: ::core::ffi::c_int = open(source_path, O_RDONLY | O_CLOEXEC | O_NOFOLLOW);
    if source < 0 as ::core::ffi::c_int {
        set_error(
            error,
            b"Cannot open source\0".as_ptr() as *const ::core::ffi::c_char,
            source_path,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut source_status: stat = stat {
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
    if fstat(source, &raw mut source_status) != 0 as ::core::ffi::c_int
        || !(source_status.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
        || source_status.st_size < 0 as __off_t
    {
        if *__errno_location() == 0 as ::core::ffi::c_int {
            *__errno_location() = EINVAL;
        }
        set_error(
            error,
            b"Source is not a complete regular file\0".as_ptr() as *const ::core::ffi::c_char,
            source_path,
        );
        close(source);
        return -2 as ::core::ffi::c_int;
    }
    let mut destination: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut attempt: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while attempt < 8 as ::core::ffi::c_uint {
        destination = open(
            &raw mut temporary_path as *mut ::core::ffi::c_char,
            O_RDWR | O_CREAT | O_EXCL | O_CLOEXEC | O_NOFOLLOW,
            source_status.st_mode & 0o777 as __mode_t,
        );
        if destination >= 0 as ::core::ffi::c_int || *__errno_location() != EEXIST {
            break;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unowned migration temporary retained for inspection: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut temporary_path as *mut ::core::ffi::c_char,
        );
        let mut attempt_uuid: [::core::ffi::c_char; 37] = [0; 37];
        if lightnvr_uuid_generate_v4(&raw mut attempt_uuid as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            break;
        }
        length = snprintf(
            &raw mut temporary_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s.migration-%s.%s.part\0".as_ptr() as *const ::core::ffi::c_char,
            destination_path,
            &raw mut (*job).uuid as *mut ::core::ffi::c_char,
            &raw mut attempt_uuid as *mut ::core::ffi::c_char,
        );
        if length < 0 as ::core::ffi::c_int
            || length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        {
            *__errno_location() = ENAMETOOLONG;
            break;
        } else {
            attempt = attempt.wrapping_add(1);
        }
    }
    if destination < 0 as ::core::ffi::c_int {
        set_error(
            error,
            b"Cannot create temporary destination\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut temporary_path as *mut ::core::ffi::c_char,
        );
        close(source);
        return -1 as ::core::ffi::c_int;
    }
    let mut buffer: *mut ::core::ffi::c_uchar =
        malloc(MIGRATION_COPY_BUFFER as size_t) as *mut ::core::ffi::c_uchar;
    if buffer.is_null() {
        safe_strcpy(
            error,
            b"Out of memory while copying recording\0".as_ptr() as *const ::core::ffi::c_char,
            STORAGE_MIGRATION_ERROR_MAX as size_t,
            0 as size_t,
        );
        close(destination);
        close(source);
        return -1 as ::core::ffi::c_int;
    }
    let mut context: mbedtls_sha256_context = mbedtls_sha256_context {
        private_buffer: [0; 64],
        private_total: [0; 2],
        private_state: [0; 8],
        private_is224: 0,
    };
    mbedtls_sha256_init(&raw mut context);
    if mbedtls_sha256_starts(&raw mut context, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int {
        safe_strcpy(
            error,
            b"Could not initialize SHA-256\0".as_ptr() as *const ::core::ffi::c_char,
            STORAGE_MIGRATION_ERROR_MAX as size_t,
            0 as size_t,
        );
        mbedtls_sha256_free(&raw mut context);
        free(buffer as *mut ::core::ffi::c_void);
        close(destination);
        close(source);
        return -1 as ::core::ffi::c_int;
    }
    let mut copied: uint64_t = 0 as uint64_t;
    let mut reported: uint64_t = 0 as uint64_t;
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut copy_started: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut copy_started);
    loop {
        if ::core::sync::atomic::AtomicBool::from_ptr(&raw mut stop_requested).load(::core::sync::atomic::Ordering::SeqCst) {
            safe_strcpy(
                error,
                b"Migration worker is shutting down\0".as_ptr() as *const ::core::ffi::c_char,
                STORAGE_MIGRATION_ERROR_MAX as size_t,
                0 as size_t,
            );
            result = -1 as ::core::ffi::c_int;
            break;
        } else if migration_cancel_requested(job) {
            safe_strcpy(
                error,
                b"Cancelled by operator\0".as_ptr() as *const ::core::ffi::c_char,
                STORAGE_MIGRATION_ERROR_MAX as size_t,
                0 as size_t,
            );
            result = -3 as ::core::ffi::c_int;
            break;
        } else {
            let mut count: ssize_t = read(
                source,
                buffer as *mut ::core::ffi::c_void,
                MIGRATION_COPY_BUFFER as size_t,
            );
            if count == 0 as ssize_t {
                break;
            }
            if count < 0 as ssize_t {
                if *__errno_location() == EINTR {
                    continue;
                }
                set_error(
                    error,
                    b"Cannot read source\0".as_ptr() as *const ::core::ffi::c_char,
                    source_path,
                );
                result = -1 as ::core::ffi::c_int;
                break;
            } else if write_all(destination, buffer, count as size_t) != 0 as ::core::ffi::c_int {
                set_error(
                    error,
                    b"Cannot write temporary destination\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut temporary_path as *mut ::core::ffi::c_char,
                );
                result = -1 as ::core::ffi::c_int;
                break;
            } else if mbedtls_sha256_update(&raw mut context, buffer, count as size_t)
                != 0 as ::core::ffi::c_int
            {
                safe_strcpy(
                    error,
                    b"SHA-256 update failed\0".as_ptr() as *const ::core::ffi::c_char,
                    STORAGE_MIGRATION_ERROR_MAX as size_t,
                    0 as size_t,
                );
                result = -1 as ::core::ffi::c_int;
                break;
            } else {
                copied = copied.wrapping_add(count as uint64_t);
                let mut throttled: ::core::ffi::c_int =
                    throttle_copy(job, &raw mut copy_started, copied);
                if throttled != 0 as ::core::ffi::c_int {
                    safe_strcpy(
                        error,
                        if throttled > 0 as ::core::ffi::c_int {
                            b"Cancelled by operator\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"Migration worker is shutting down\0".as_ptr()
                                as *const ::core::ffi::c_char
                        },
                        STORAGE_MIGRATION_ERROR_MAX as size_t,
                        0 as size_t,
                    );
                    result = if throttled > 0 as ::core::ffi::c_int {
                        -3 as ::core::ffi::c_int
                    } else {
                        -1 as ::core::ffi::c_int
                    };
                    break;
                } else {
                    if (copied.wrapping_sub(reported) as ::core::ffi::c_ulonglong)
                        < MIGRATION_PROGRESS_INTERVAL
                    {
                        continue;
                    }
                    if db_storage_migration_update_progress(
                        &raw mut (*job).uuid as *mut ::core::ffi::c_char,
                        b"copying\0".as_ptr() as *const ::core::ffi::c_char,
                        copied,
                        source_status.st_size as uint64_t,
                    )
                    .0 != db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0
                    {
                        safe_strcpy(
                            error,
                            b"Could not persist migration progress\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            STORAGE_MIGRATION_ERROR_MAX as size_t,
                            0 as size_t,
                        );
                        result = -1 as ::core::ffi::c_int;
                        break;
                    } else {
                        reported = copied;
                    }
                }
            }
        }
    }
    if result == 0 as ::core::ffi::c_int && fsync(destination) != 0 as ::core::ffi::c_int {
        set_error(
            error,
            b"Cannot sync temporary destination\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut temporary_path as *mut ::core::ffi::c_char,
        );
        result = -1 as ::core::ffi::c_int;
    }
    free(buffer as *mut ::core::ffi::c_void);
    close(source);
    let mut source_checksum: [::core::ffi::c_char; 65] = [0; 65];
    if result == 0 as ::core::ffi::c_int
        && finish_digest(
            &raw mut context,
            &raw mut source_checksum as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            error,
            b"Could not finish source SHA-256\0".as_ptr() as *const ::core::ffi::c_char,
            STORAGE_MIGRATION_ERROR_MAX as size_t,
            0 as size_t,
        );
        result = -1 as ::core::ffi::c_int;
    } else if result != 0 as ::core::ffi::c_int {
        mbedtls_sha256_free(&raw mut context);
    }
    if result != 0 as ::core::ffi::c_int {
        close(destination);
        return result;
    }
    (*job).bytes_total = copied;
    if db_storage_migration_update_progress(
        &raw mut (*job).uuid as *mut ::core::ffi::c_char,
        b"verifying\0".as_ptr() as *const ::core::ffi::c_char,
        copied,
        copied,
    )
    .0 != db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0
    {
        safe_strcpy(
            error,
            b"Could not persist verification state\0".as_ptr() as *const ::core::ffi::c_char,
            STORAGE_MIGRATION_ERROR_MAX as size_t,
            0 as size_t,
        );
        close(destination);
        return -1 as ::core::ffi::c_int;
    }
    let mut destination_size: uint64_t = 0 as uint64_t;
    let mut destination_checksum: [::core::ffi::c_char; 65] = [0; 65];
    let mut verified: stat = stat {
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
    let mut after_verification: stat = stat {
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
    if fstat(destination, &raw mut verified) != 0 as ::core::ffi::c_int
        || sha256_descriptor(
            destination,
            &raw mut temporary_path as *mut ::core::ffi::c_char,
            &raw mut destination_size,
            &raw mut destination_checksum as *mut ::core::ffi::c_char,
            error,
        ) != 0 as ::core::ffi::c_int
        || fstat(destination, &raw mut after_verification) != 0 as ::core::ffi::c_int
        || !same_verified_file(&raw mut verified, &raw mut after_verification)
    {
        close(destination);
        return -1 as ::core::ffi::c_int;
    }
    if destination_size != copied
        || strcmp(
            &raw mut source_checksum as *mut ::core::ffi::c_char,
            &raw mut destination_checksum as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            error,
            b"Destination checksum verification failed\0".as_ptr() as *const ::core::ffi::c_char,
            STORAGE_MIGRATION_ERROR_MAX as size_t,
            0 as size_t,
        );
        close(destination);
        return -1 as ::core::ffi::c_int;
    }
    if migration_cancel_requested(job) {
        safe_strcpy(
            error,
            b"Cancelled by operator\0".as_ptr() as *const ::core::ffi::c_char,
            STORAGE_MIGRATION_ERROR_MAX as size_t,
            0 as size_t,
        );
        close(destination);
        return -3 as ::core::ffi::c_int;
    }
    result = publish_verified_temporary(
        destination,
        &raw mut verified,
        &raw mut temporary_path as *mut ::core::ffi::c_char,
        destination_path,
        &raw mut (*job).uuid as *mut ::core::ffi::c_char,
        publication,
        error,
    );
    if result != 0 as ::core::ffi::c_int {
        close(destination);
        return result;
    }
    safe_strcpy(
        checksum,
        &raw mut source_checksum as *mut ::core::ffi::c_char,
        STORAGE_MIGRATION_CHECKSUM_MAX as size_t,
        0 as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn destination_retained(
    mut job: *const storage_migration_job_t,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut retained: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        b"SELECT 1 FROM recordings WHERE storage_target_uuid=?1 AND object_key=?2 UNION ALL SELECT 1 FROM storage_recording_copies WHERE target_uuid=?1 AND object_key=?2 LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw const (*job).destination_target_uuid as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw const (*job).destination_object_key as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        let mut rc: ::core::ffi::c_int = sqlite3_step(stmt);
        retained = if rc == SQLITE_ROW {
            1 as ::core::ffi::c_int
        } else if rc == SQLITE_DONE {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return retained;
}
unsafe extern "C" fn finish_cleanup(
    mut job: *mut storage_migration_job_t,
    mut source_path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if storage_source_has_lease((*job).recording_id) {
        db_storage_migration_defer_cleanup(
            job,
            b"Source cleanup waits for active recording readers\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut destination: storage_target_t = storage_target_t {
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
    let mut source: storage_target_t = storage_target_t {
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
    let mut path: [::core::ffi::c_char; 512] = [0; 512];
    let mut hash: [::core::ffi::c_char; 65] = [0; 65];
    let mut bytes: uint64_t = 0 as uint64_t;
    let mut verified: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if destination_retained(job) == 1 as ::core::ffi::c_int
        && storage_target_probe_and_publish(
            &raw mut (*job).destination_target_uuid as *mut ::core::ffi::c_char,
            r#false != 0,
            &raw mut destination,
        )
        .0 == db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        if strcmp(
            &raw mut destination.target_type as *mut ::core::ffi::c_char,
            b"s3\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            let mut control: storage_transfer_control_t = storage_transfer_control_t {
                cancelled: Some(
                    transfer_cancelled as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> bool,
                ),
                context: job as *mut ::core::ffi::c_void,
                bandwidth_bps: (*job).bandwidth_limit_bps,
                job_uuid: ::core::ptr::null::<::core::ffi::c_char>(),
            };
            verified = storage_s3_verify(
                &raw mut destination,
                &raw mut (*job).destination_object_key as *mut ::core::ffi::c_char,
                (*job).bytes_total,
                &raw mut (*job).checksum as *mut ::core::ffi::c_char,
                &raw mut control,
                &raw mut error as *mut ::core::ffi::c_char,
            );
        } else if db_storage_target_mount_guard_active(&raw mut destination) as ::core::ffi::c_int
            != 0
            && db_storage_target_resolve_path(
                &raw mut destination.uuid as *mut ::core::ffi::c_char,
                &raw mut (*job).destination_object_key as *mut ::core::ffi::c_char,
                &raw mut path as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            && sha256_file(
                &raw mut path as *mut ::core::ffi::c_char,
                &raw mut bytes,
                &raw mut hash as *mut ::core::ffi::c_char,
                &raw mut error as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            && bytes == (*job).bytes_total
            && strcmp(
                &raw mut hash as *mut ::core::ffi::c_char,
                &raw mut (*job).checksum as *mut ::core::ffi::c_char,
            ) == 0
        {
            verified = 0 as ::core::ffi::c_int;
        }
    }
    if verified != 0 as ::core::ffi::c_int
        || db_storage_target_get(
            &raw mut (*job).source_target_uuid as *mut ::core::ffi::c_char,
            &raw mut source,
        )
        .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        db_storage_migration_defer_cleanup(
            job,
            b"Source retained: committed destination cannot be verified\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut removed: ::core::ffi::c_int = 0;
    if strcmp(
        &raw mut source.target_type as *mut ::core::ffi::c_char,
        b"s3\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        removed = if storage_s3_probe(&raw mut source, r#false != 0) == 0 as ::core::ffi::c_int {
            storage_s3_delete(
                &raw mut source,
                &raw mut (*job).source_object_key as *mut ::core::ffi::c_char,
                &raw mut error as *mut ::core::ffi::c_char,
            )
        } else {
            -1 as ::core::ffi::c_int
        };
    } else {
        removed =
            if db_storage_target_mount_guard_active(&raw mut source) as ::core::ffi::c_int != 0 {
                unlink(source_path)
            } else {
                -1 as ::core::ffi::c_int
            };
        if removed != 0 as ::core::ffi::c_int
            && db_storage_target_mount_guard_active(&raw mut source) as ::core::ffi::c_int != 0
            && *__errno_location() == ENOENT
        {
            removed = 0 as ::core::ffi::c_int;
        }
        if removed == 0 as ::core::ffi::c_int {
            fsync_parent(source_path);
        }
    }
    if removed != 0 as ::core::ffi::c_int {
        db_storage_migration_defer_cleanup(
            job,
            b"Verified destination committed; source cleanup failed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return if db_storage_migration_complete_cleanup(
        &raw mut (*job).uuid as *mut ::core::ffi::c_char,
    )
    .0 == db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0
    {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn cleanup_abandoned() {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [
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
    let mut upload: [::core::ffi::c_char; 1024] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char = b"SELECT uuid,upload_id FROM storage_migration_jobs WHERE next_attempt_at<=strftime('%s','now') AND ((artifacts_cleaned=0 AND (state IN('failed','cancelled') OR (cancel_requested=1 AND state IN('copying','verifying','committing')))) OR (state='completed' AND upload_id<>'')) ORDER BY updated_at LIMIT 1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
        && sqlite3_step(stmt) == SQLITE_ROW
    {
        safe_strcpy(
            &raw mut uuid as *mut ::core::ffi::c_char,
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut upload as *mut ::core::ffi::c_char,
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            0 as size_t,
        );
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    if *(&raw mut uuid as *mut ::core::ffi::c_char) == 0 {
        return;
    }
    let mut job: storage_migration_job_t = storage_migration_job_t {
        uuid: [0; 37],
        recording_id: 0,
        owner_user_id: 0,
        operation: [0; 8],
        source_target_uuid: [0; 37],
        source_object_key: [0; 512],
        destination_target_uuid: [0; 37],
        destination_object_key: [0; 512],
        state: [0; 24],
        checksum: [0; 65],
        bytes_total: 0,
        bytes_copied: 0,
        attempt_count: 0,
        max_attempts: 0,
        next_attempt_at: 0,
        last_error: [0; 256],
        cancel_requested: false,
        bandwidth_limit_bps: 0,
        window_start_minute: 0,
        window_end_minute: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
        started_at: 0,
        completed_at: 0,
    };
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
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
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
    let mut path: [::core::ffi::c_char; 512] = [0; 512];
    let mut temporary: [::core::ffi::c_char; 512] = [0; 512];
    if db_storage_migration_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut job).0
        == db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0
        && db_storage_target_get(
            &raw mut job.destination_target_uuid as *mut ::core::ffi::c_char,
            &raw mut target,
        )
        .0 == db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        let mut retained: ::core::ffi::c_int = destination_retained(&raw mut job);
        if strcmp(
            &raw mut target.target_type as *mut ::core::ffi::c_char,
            b"s3\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            result = storage_s3_probe(&raw mut target, r#false != 0);
            if result == 0 {
                result = storage_s3_abort_upload(
                    &raw mut target,
                    &raw mut job.destination_object_key as *mut ::core::ffi::c_char,
                    &raw mut upload as *mut ::core::ffi::c_char,
                    &raw mut error as *mut ::core::ffi::c_char,
                );
            }
            if result == 0 && retained == 0 {
                result = storage_s3_delete(
                    &raw mut target,
                    &raw mut job.destination_object_key as *mut ::core::ffi::c_char,
                    &raw mut error as *mut ::core::ffi::c_char,
                );
            }
        } else if db_storage_target_mount_guard_active(&raw mut target) as ::core::ffi::c_int != 0
            && db_storage_target_resolve_path(
                &raw mut target.uuid as *mut ::core::ffi::c_char,
                &raw mut job.destination_object_key as *mut ::core::ffi::c_char,
                &raw mut path as *mut ::core::ffi::c_char,
            ) == 0
        {
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
            if retained < 0 as ::core::ffi::c_int {
                safe_strcpy(
                    &raw mut error as *mut ::core::ffi::c_char,
                    b"Cannot establish destination ownership; retained\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    0 as size_t,
                );
                result = -1 as ::core::ffi::c_int;
            } else if retained > 0 as ::core::ffi::c_int {
                result = 0 as ::core::ffi::c_int;
            } else if lstat(&raw mut path as *mut ::core::ffi::c_char, &raw mut status)
                != 0 as ::core::ffi::c_int
            {
                result = if *__errno_location() == ENOENT {
                    0 as ::core::ffi::c_int
                } else {
                    -1 as ::core::ffi::c_int
                };
                if result != 0 as ::core::ffi::c_int {
                    safe_strcpy(
                        &raw mut error as *mut ::core::ffi::c_char,
                        b"Cannot inspect unowned destination; retained\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                        0 as size_t,
                    );
                }
            } else {
                safe_strcpy(
                    &raw mut error as *mut ::core::ffi::c_char,
                    b"Final destination retained: migration ownership is not recorded\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    0 as size_t,
                );
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Unowned migration destination retained for inspection: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut path as *mut ::core::ffi::c_char,
                );
                result = -1 as ::core::ffi::c_int;
            }
            let mut n: ::core::ffi::c_int = snprintf(
                &raw mut temporary as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                b"%s.migration-%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut path as *mut ::core::ffi::c_char,
                &raw mut uuid as *mut ::core::ffi::c_char,
            );
            if n <= 0 as ::core::ffi::c_int
                || n >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int
            {
                result = -1 as ::core::ffi::c_int;
            } else {
                let mut parent_path: [::core::ffi::c_char; 512] = [0; 512];
                safe_strcpy(
                    &raw mut parent_path as *mut ::core::ffi::c_char,
                    &raw mut temporary as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    0 as size_t,
                );
                let mut leaf: *mut ::core::ffi::c_char = strrchr(
                    &raw mut temporary as *mut ::core::ffi::c_char,
                    '/' as ::core::ffi::c_int,
                );
                let mut prefix: *const ::core::ffi::c_char = if !leaf.is_null() {
                    leaf.offset(1 as ::core::ffi::c_int as isize)
                } else {
                    &raw mut temporary as *mut ::core::ffi::c_char
                };
                let mut directory: *mut DIR =
                    opendir(dirname(&raw mut parent_path as *mut ::core::ffi::c_char));
                if directory.is_null() {
                    if *__errno_location() != ENOENT {
                        result = -1 as ::core::ffi::c_int;
                    }
                } else {
                    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
                    *__errno_location() = 0 as ::core::ffi::c_int;
                    loop {
                        entry = readdir(directory);
                        if entry.is_null() {
                            break;
                        }
                        if strncmp(
                            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
                            prefix,
                            strlen(prefix),
                        ) == 0
                            && (*entry).d_name[strlen(prefix)] as ::core::ffi::c_int
                                == '.' as ::core::ffi::c_int
                        {
                            safe_strcpy(
                                &raw mut error as *mut ::core::ffi::c_char,
                                b"Temporary destination retained: migration ownership is not recorded\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                                0 as size_t,
                            );
                            _log_message_ctx(
                                log_level_t::LOG_LEVEL_WARN,
                                LOG_COMPONENT,
                                _log_stream_name,
                                b"Unowned migration attempt retained for inspection: %s/%s\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut parent_path as *mut ::core::ffi::c_char,
                                &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
                            );
                            result = -1 as ::core::ffi::c_int;
                        }
                        *__errno_location() = 0 as ::core::ffi::c_int;
                    }
                    if *__errno_location() != 0 as ::core::ffi::c_int {
                        result = -1 as ::core::ffi::c_int;
                    }
                    closedir(directory);
                }
            }
        }
    }
    pthread_mutex_lock(mutex);
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    sql = if result == 0 as ::core::ffi::c_int {
        b"UPDATE storage_migration_jobs SET artifacts_cleaned=1,upload_id='',upload_parts='',state=CASE WHEN cancel_requested=1 THEN 'cancelled' ELSE state END WHERE uuid=?;\0"
            .as_ptr() as *const ::core::ffi::c_char
    } else {
        b"UPDATE storage_migration_jobs SET next_attempt_at=strftime('%s','now')+60,last_error=? WHERE uuid=?;\0"
            .as_ptr() as *const ::core::ffi::c_char
    };
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        let mut parameter: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if result != 0 as ::core::ffi::c_int {
            let c2rust_fresh0 = parameter;
            parameter += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh0,
                if error[0usize] as ::core::ffi::c_int != 0 {
                    &raw mut error as *mut ::core::ffi::c_char
                } else {
                    b"Unpublished transfer artifacts await cleanup\0".as_ptr()
                        as *const ::core::ffi::c_char
                },
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        sqlite3_bind_text(
            stmt,
            parameter,
            &raw mut uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
}
unsafe extern "C" fn restore_to_filesystem(
    mut job: *mut storage_migration_job_t,
    mut source: *const storage_target_t,
    mut destination: *const storage_target_t,
    mut path: *const ::core::ffi::c_char,
    mut checksum: *mut ::core::ffi::c_char,
    mut error: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    *checksum.offset(0isize) = 0 as ::core::ffi::c_char;
    if sqlite3_prepare_v2(
        db,
        b"SELECT archive_checksum FROM recordings WHERE id=?1 AND storage_target_uuid=?2 AND object_key=?3 AND deletion_pending=0;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int64(
            stmt,
            1 as ::core::ffi::c_int,
            (*job).recording_id as sqlite3_int64,
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw mut (*job).source_target_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            &raw mut (*job).source_object_key as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if sqlite3_step(stmt) == SQLITE_ROW {
            safe_strcpy(
                checksum,
                sqlite3_column_text(stmt, 0 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
                65 as size_t,
                0 as size_t,
            );
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    if strlen(checksum) != 64 as size_t {
        safe_strcpy(
            error,
            b"Archive source identity changed or has no checksum\0".as_ptr()
                as *const ::core::ffi::c_char,
            256 as size_t,
            0 as size_t,
        );
        return -2 as ::core::ffi::c_int;
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
    if lstat(path, &raw mut info) == 0 as ::core::ffi::c_int {
        let mut bytes: uint64_t = 0;
        let mut hash: [::core::ffi::c_char; 65] = [0; 65];
        if sha256_file(
            path,
            &raw mut bytes,
            &raw mut hash as *mut ::core::ffi::c_char,
            error,
        ) != 0
            || bytes != (*job).bytes_total
            || strcmp(&raw mut hash as *mut ::core::ffi::c_char, checksum) != 0
        {
            safe_strcpy(
                error,
                b"Restore destination contains different data\0".as_ptr()
                    as *const ::core::ffi::c_char,
                256 as size_t,
                0 as size_t,
            );
            return -2 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
    if *__errno_location() != ENOENT {
        return -1 as ::core::ffi::c_int;
    }
    if (*destination).available_bytes <= (*destination).reserve_bytes
        || (*job).bytes_total
            > (*destination)
                .available_bytes
                .wrapping_sub((*destination).reserve_bytes)
    {
        safe_strcpy(
            error,
            b"Restore would exceed destination free-space reserve\0".as_ptr()
                as *const ::core::ffi::c_char,
            256 as size_t,
            0 as size_t,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut temporary: [::core::ffi::c_char; 512] = [0; 512];
    let mut n: ::core::ffi::c_int = snprintf(
        &raw mut temporary as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"%s.migration-%s.part\0".as_ptr() as *const ::core::ffi::c_char,
        path,
        &raw mut (*job).uuid as *mut ::core::ffi::c_char,
    );
    if n < 0 as ::core::ffi::c_int
        || n >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int
        || ensure_path(&raw mut temporary as *mut ::core::ffi::c_char) != 0
    {
        return -1 as ::core::ffi::c_int;
    }
    if unlink(&raw mut temporary as *mut ::core::ffi::c_char) != 0 && *__errno_location() != ENOENT
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut control: storage_transfer_control_t = storage_transfer_control_t {
        cancelled: Some(
            transfer_cancelled as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> bool,
        ),
        context: job as *mut ::core::ffi::c_void,
        bandwidth_bps: (*job).bandwidth_limit_bps,
        job_uuid: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut result: ::core::ffi::c_int = storage_s3_download(
        source,
        &raw mut (*job).source_object_key as *mut ::core::ffi::c_char,
        &raw mut temporary as *mut ::core::ffi::c_char,
        (*job).bytes_total,
        checksum,
        &raw mut control,
        error,
    );
    if result == 0 && transfer_cancelled(job as *mut ::core::ffi::c_void) as ::core::ffi::c_int != 0
    {
        result = -3 as ::core::ffi::c_int;
    }
    if result == 0 {
        if link(&raw mut temporary as *mut ::core::ffi::c_char, path) != 0 {
            result = if *__errno_location() == EEXIST {
                -2 as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        } else if fsync_parent(path) != 0 {
            result = -1 as ::core::ffi::c_int;
        }
    }
    unlink(&raw mut temporary as *mut ::core::ffi::c_char);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_migration_process_one() -> ::core::ffi::c_int {
    let mut read_only: *const ::core::ffi::c_char =
        getenv(b"LIGHTNVR_STORAGE_READ_ONLY\0".as_ptr() as *const ::core::ffi::c_char);
    if !read_only.is_null() && strcmp(read_only, b"1\0".as_ptr() as *const ::core::ffi::c_char) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    cleanup_abandoned();
    let mut job: storage_migration_job_t = storage_migration_job_t {
        uuid: [0; 37],
        recording_id: 0,
        owner_user_id: 0,
        operation: [0; 8],
        source_target_uuid: [0; 37],
        source_object_key: [0; 512],
        destination_target_uuid: [0; 37],
        destination_object_key: [0; 512],
        state: [0; 24],
        checksum: [0; 65],
        bytes_total: 0,
        bytes_copied: 0,
        attempt_count: 0,
        max_attempts: 0,
        next_attempt_at: 0,
        last_error: [0; 256],
        cancel_requested: false,
        bandwidth_limit_bps: 0,
        window_start_minute: 0,
        window_end_minute: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
        started_at: 0,
        completed_at: 0,
    };
    let mut claimed: ::core::ffi::c_int = db_storage_migration_claim_due(&raw mut job);
    if claimed <= 0 as ::core::ffi::c_int {
        return claimed;
    }
    let mut destination_target: storage_target_t = storage_target_t {
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
        &raw mut job.destination_target_uuid as *mut ::core::ffi::c_char,
        &raw mut destination_target,
    )
    .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        db_storage_migration_record_failure(
            &raw mut job,
            b"Destination target no longer exists\0".as_ptr() as *const ::core::ffi::c_char,
            r#false != 0,
        );
        return 1 as ::core::ffi::c_int;
    }
    let mut source_target: storage_target_t = storage_target_t {
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
        &raw mut job.source_target_uuid as *mut ::core::ffi::c_char,
        &raw mut source_target,
    )
    .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        db_storage_migration_record_failure(
            &raw mut job,
            b"Source target no longer exists\0".as_ptr() as *const ::core::ffi::c_char,
            r#false != 0,
        );
        return 1 as ::core::ffi::c_int;
    }
    let mut object_source: bool = strcmp(
        &raw mut source_target.target_type as *mut ::core::ffi::c_char,
        b"s3\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0;
    let mut object_destination: bool = strcmp(
        &raw mut destination_target.target_type as *mut ::core::ffi::c_char,
        b"s3\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    let mut source_path: [::core::ffi::c_char; 512] = [
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
    let mut destination_path: [::core::ffi::c_char; 512] = [
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
    if !object_source && !db_storage_target_mount_guard_active(&raw mut source_target) {
        if strcmp(
            &raw mut job.state as *mut ::core::ffi::c_char,
            b"cleanup_pending\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            db_storage_migration_defer_cleanup(
                &raw mut job,
                b"Source mount is unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            db_storage_migration_record_failure(
                &raw mut job,
                b"Source mount is unavailable\0".as_ptr() as *const ::core::ffi::c_char,
                r#true != 0,
            );
        }
        return 1 as ::core::ffi::c_int;
    }
    if !object_source
        && db_storage_target_resolve_path(
            &raw mut job.source_target_uuid as *mut ::core::ffi::c_char,
            &raw mut job.source_object_key as *mut ::core::ffi::c_char,
            &raw mut source_path as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        || !object_destination
            && db_storage_target_resolve_path(
                &raw mut job.destination_target_uuid as *mut ::core::ffi::c_char,
                &raw mut job.destination_object_key as *mut ::core::ffi::c_char,
                &raw mut destination_path as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
    {
        db_storage_migration_record_failure(
            &raw mut job,
            b"Storage target identity no longer resolves\0".as_ptr() as *const ::core::ffi::c_char,
            r#false != 0,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw mut job.state as *mut ::core::ffi::c_char,
        b"cleanup_pending\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        finish_cleanup(
            &raw mut job,
            &raw mut source_path as *mut ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    if storage_target_probe_and_publish(
        &raw mut job.destination_target_uuid as *mut ::core::ffi::c_char,
        r#false != 0,
        &raw mut destination_target,
    )
    .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
        || !destination_target.enabled
        || !db_storage_target_mount_guard_active(&raw mut destination_target)
    {
        db_storage_migration_record_failure(
            &raw mut job,
            b"Destination target is unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            r#true != 0,
        );
        return 1 as ::core::ffi::c_int;
    }
    let mut checksum: [::core::ffi::c_char; 65] = [0; 65];
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
    let mut copied: ::core::ffi::c_int = 0;
    let mut publication: migration_publication_t = migration_publication_t {
        fd: -1 as ::core::ffi::c_int,
        parent: -1 as ::core::ffi::c_int,
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
        parent_path: [0; 512],
        leaf: [0; 512],
    };
    if object_source as ::core::ffi::c_int != 0 && object_destination as ::core::ffi::c_int != 0 {
        let mut resolved: ::core::ffi::c_int = storage_source_resolve(
            job.recording_id,
            &raw mut source_path as *mut ::core::ffi::c_char,
            &raw mut error as *mut ::core::ffi::c_char,
        );
        if resolved != C2Rust_Unnamed_2::STORAGE_SOURCE_READY.0 {
            if resolved == C2Rust_Unnamed_2::STORAGE_SOURCE_PREPARING.0 {
                db_storage_migration_defer_source(&raw mut job.uuid as *mut ::core::ffi::c_char);
            } else {
                db_storage_migration_record_failure(
                    &raw mut job,
                    &raw mut error as *mut ::core::ffi::c_char,
                    r#true != 0,
                );
            }
            return 1 as ::core::ffi::c_int;
        }
    }
    let mut retained_destination: bool = destination_retained(&raw mut job) != 0;
    if object_destination {
        let mut control: storage_transfer_control_t = storage_transfer_control_t {
            cancelled: Some(
                transfer_cancelled as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> bool,
            ),
            context: &raw mut job as *mut ::core::ffi::c_void,
            bandwidth_bps: job.bandwidth_limit_bps,
            job_uuid: &raw mut job.uuid as *mut ::core::ffi::c_char,
        };
        copied = sha256_file(
            &raw mut source_path as *mut ::core::ffi::c_char,
            &raw mut job.bytes_total,
            &raw mut checksum as *mut ::core::ffi::c_char,
            &raw mut error as *mut ::core::ffi::c_char,
        );
        let mut remote_size: uint64_t = 0 as uint64_t;
        if copied == 0 as ::core::ffi::c_int {
            let mut found: ::core::ffi::c_int = storage_s3_stat(
                &raw mut destination_target,
                &raw mut job.destination_object_key as *mut ::core::ffi::c_char,
                &raw mut remote_size,
                &raw mut error as *mut ::core::ffi::c_char,
            );
            if found == C2Rust_Unnamed_1::STORAGE_S3_OK.0 {
                copied = if remote_size == job.bytes_total {
                    storage_s3_verify(
                        &raw mut destination_target,
                        &raw mut job.destination_object_key as *mut ::core::ffi::c_char,
                        job.bytes_total,
                        &raw mut checksum as *mut ::core::ffi::c_char,
                        &raw mut control,
                        &raw mut error as *mut ::core::ffi::c_char,
                    )
                } else {
                    C2Rust_Unnamed_1::STORAGE_S3_CONFLICT.0
                };
                if copied == C2Rust_Unnamed_1::STORAGE_S3_CONFLICT.0
                    && !transfer_cancelled(&raw mut job as *mut ::core::ffi::c_void)
                    && destination_retained(&raw mut job) == 0 as ::core::ffi::c_int
                {
                    copied = storage_s3_delete(
                        &raw mut destination_target,
                        &raw mut job.destination_object_key as *mut ::core::ffi::c_char,
                        &raw mut error as *mut ::core::ffi::c_char,
                    );
                    if copied == 0 {
                        found = C2Rust_Unnamed_1::STORAGE_S3_MISSING.0;
                    }
                }
            } else if found != C2Rust_Unnamed_1::STORAGE_S3_MISSING.0 {
                copied = -1 as ::core::ffi::c_int;
            }
            if copied == 0 && found == C2Rust_Unnamed_1::STORAGE_S3_MISSING.0 {
                copied = storage_s3_upload(
                    &raw mut destination_target,
                    &raw mut job.destination_object_key as *mut ::core::ffi::c_char,
                    &raw mut source_path as *mut ::core::ffi::c_char,
                    &raw mut checksum as *mut ::core::ffi::c_char,
                    &raw mut control,
                    &raw mut error as *mut ::core::ffi::c_char,
                );
            }
        }
        if copied == 0 as ::core::ffi::c_int {
            db_storage_migration_update_progress(
                &raw mut job.uuid as *mut ::core::ffi::c_char,
                b"verifying\0".as_ptr() as *const ::core::ffi::c_char,
                job.bytes_total,
                job.bytes_total,
            );
            copied = storage_s3_verify(
                &raw mut destination_target,
                &raw mut job.destination_object_key as *mut ::core::ffi::c_char,
                job.bytes_total,
                &raw mut checksum as *mut ::core::ffi::c_char,
                &raw mut control,
                &raw mut error as *mut ::core::ffi::c_char,
            );
        }
    } else if object_source {
        copied = restore_to_filesystem(
            &raw mut job,
            &raw mut source_target,
            &raw mut destination_target,
            &raw mut destination_path as *mut ::core::ffi::c_char,
            &raw mut checksum as *mut ::core::ffi::c_char,
            &raw mut error as *mut ::core::ffi::c_char,
        );
    } else {
        copied = copy_and_verify(
            &raw mut job,
            &raw mut source_path as *mut ::core::ffi::c_char,
            &raw mut destination_path as *mut ::core::ffi::c_char,
            &raw mut publication,
            &raw mut checksum as *mut ::core::ffi::c_char,
            &raw mut error as *mut ::core::ffi::c_char,
        );
    }
    if copied != 0 as ::core::ffi::c_int {
        if copied == -3 as ::core::ffi::c_int {
            db_storage_migration_mark_cancelled(
                &raw mut job.uuid as *mut ::core::ffi::c_char,
                &raw mut job,
            );
            release_publication(&raw mut publication);
            return 1 as ::core::ffi::c_int;
        }
        db_storage_migration_record_failure(
            &raw mut job,
            if error[0usize] as ::core::ffi::c_int != 0 {
                &raw mut error as *mut ::core::ffi::c_char
            } else {
                b"Recording copy failed\0".as_ptr() as *const ::core::ffi::c_char
            },
            copied != -2 as ::core::ffi::c_int,
        );
        release_publication(&raw mut publication);
        return 1 as ::core::ffi::c_int;
    }
    if migration_cancel_requested(&raw mut job) {
        if retained_destination {
            db_storage_migration_mark_cancelled(
                &raw mut job.uuid as *mut ::core::ffi::c_char,
                &raw mut job,
            );
            release_publication(&raw mut publication);
            return 1 as ::core::ffi::c_int;
        }
        if object_destination {
            if storage_s3_delete(
                &raw mut destination_target,
                &raw mut job.destination_object_key as *mut ::core::ffi::c_char,
                &raw mut error as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                db_storage_migration_record_failure(
                    &raw mut job,
                    b"Cancelled upload awaits destination cleanup\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    r#true != 0,
                );
                release_publication(&raw mut publication);
                return 1 as ::core::ffi::c_int;
            }
        } else if publication.fd < 0 as ::core::ffi::c_int {
            unlink(&raw mut destination_path as *mut ::core::ffi::c_char);
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Cancelled filesystem destination retained for inspection: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut destination_path as *mut ::core::ffi::c_char,
            );
        }
        db_storage_migration_mark_cancelled(
            &raw mut job.uuid as *mut ::core::ffi::c_char,
            &raw mut job,
        );
        release_publication(&raw mut publication);
        return 1 as ::core::ffi::c_int;
    }
    if db_storage_migration_update_progress(
        &raw mut job.uuid as *mut ::core::ffi::c_char,
        b"committing\0".as_ptr() as *const ::core::ffi::c_char,
        job.bytes_total,
        job.bytes_total,
    )
    .0 != db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0
    {
        db_storage_migration_record_failure(
            &raw mut job,
            b"Could not persist commit state\0".as_ptr() as *const ::core::ffi::c_char,
            r#true != 0,
        );
        release_publication(&raw mut publication);
        return 1 as ::core::ffi::c_int;
    }
    if storage_target_probe_and_publish(
        &raw mut job.destination_target_uuid as *mut ::core::ffi::c_char,
        r#false != 0,
        &raw mut destination_target,
    )
    .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
        || !destination_target.enabled
        || !db_storage_target_mount_guard_active(&raw mut destination_target)
    {
        db_storage_migration_record_failure(
            &raw mut job,
            b"Destination target became unavailable before commit\0".as_ptr()
                as *const ::core::ffi::c_char,
            r#true != 0,
        );
        release_publication(&raw mut publication);
        return 1 as ::core::ffi::c_int;
    }
    if !object_source && !db_storage_target_mount_guard_active(&raw mut source_target) {
        db_storage_migration_record_failure(
            &raw mut job,
            b"Source mount became unavailable before commit\0".as_ptr()
                as *const ::core::ffi::c_char,
            r#true != 0,
        );
        release_publication(&raw mut publication);
        return 1 as ::core::ffi::c_int;
    }
    if !publication_still_verified(&raw mut publication) {
        db_storage_migration_record_failure(
            &raw mut job,
            b"Published destination identity changed before commit; retained\0".as_ptr()
                as *const ::core::ffi::c_char,
            r#false != 0,
        );
        release_publication(&raw mut publication);
        return 1 as ::core::ffi::c_int;
    }
    let mut committed: db_storage_migration_result_t = db_storage_migration_result_t(
        if strcmp(
            &raw mut job.operation as *mut ::core::ffi::c_char,
            b"copy\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            db_storage_migration_commit_copy(
                &raw mut job,
                &raw mut checksum as *mut ::core::ffi::c_char,
            )
            .0
        } else {
            db_storage_migration_commit_location(
                &raw mut job,
                &raw mut destination_path as *mut ::core::ffi::c_char,
                &raw mut checksum as *mut ::core::ffi::c_char,
            )
            .0
        },
    );
    if committed.0 != db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0 {
        if !retained_destination && object_destination as ::core::ffi::c_int != 0 {
            if storage_s3_delete(
                &raw mut destination_target,
                &raw mut job.destination_object_key as *mut ::core::ffi::c_char,
                &raw mut error as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                db_storage_migration_record_failure(
                    &raw mut job,
                    b"Uncommitted archive copy awaits cleanup\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    r#true != 0,
                );
                release_publication(&raw mut publication);
                return 1 as ::core::ffi::c_int;
            }
        } else if !retained_destination && publication.fd < 0 as ::core::ffi::c_int {
            unlink(&raw mut destination_path as *mut ::core::ffi::c_char);
        } else if !retained_destination {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Uncommitted filesystem destination retained for inspection: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut destination_path as *mut ::core::ffi::c_char,
            );
        }
        if migration_cancel_requested(&raw mut job) {
            db_storage_migration_mark_cancelled(
                &raw mut job.uuid as *mut ::core::ffi::c_char,
                &raw mut job,
            );
        } else if committed.0
            == db_storage_migration_result_t::DB_STORAGE_MIGRATION_SOURCE_CHANGED.0
        {
            db_storage_migration_record_failure(
                &raw mut job,
                b"Recording location changed before migration commit\0".as_ptr()
                    as *const ::core::ffi::c_char,
                r#false != 0,
            );
        } else {
            db_storage_migration_record_failure(
                &raw mut job,
                b"Could not atomically commit recording location\0".as_ptr()
                    as *const ::core::ffi::c_char,
                r#true != 0,
            );
        }
        release_publication(&raw mut publication);
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw mut job.operation as *mut ::core::ffi::c_char,
        b"move\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut job.checksum as *mut ::core::ffi::c_char,
            &raw mut checksum as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
            0 as size_t,
        );
        finish_cleanup(
            &raw mut job,
            &raw mut source_path as *mut ::core::ffi::c_char,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Storage replication %s completed for recording %llu\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut job.uuid as *mut ::core::ffi::c_char,
            job.recording_id as ::core::ffi::c_ulonglong,
        );
    }
    release_publication(&raw mut publication);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn migration_worker_main(
    mut unused: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    log_set_thread_context(
        b"StorageMigration\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let mut last_lifecycle_check: time_t = 0 as time_t;
    loop {
        pthread_mutex_lock(&raw mut worker.mutex);
        let mut running: bool = worker.running;
        pthread_mutex_unlock(&raw mut worker.mutex);
        if !running {
            break;
        }
        let mut result: ::core::ffi::c_int = storage_migration_process_one();
        storage_deletion_process_one();
        let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
        if now - last_lifecycle_check >= 60 as time_t {
            last_lifecycle_check = now;
            let mut total: ::core::ffi::c_int = db_storage_target_count();
            let mut targets: *mut storage_target_t =
                (if total > 0 as ::core::ffi::c_int && total <= STORAGE_TARGET_MAX_COUNT {
                    calloc(total as size_t, ::core::mem::size_of::<storage_target_t>())
                } else {
                    NULL
                }) as *mut storage_target_t;
            if !targets.is_null() {
                let mut count: ::core::ffi::c_int = db_storage_target_list(targets, total);
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < count
                    && !::core::sync::atomic::AtomicBool::from_ptr(&raw mut stop_requested).load(::core::sync::atomic::Ordering::SeqCst)
                {
                    if (*targets.offset(i as isize)).enabled as ::core::ffi::c_int != 0
                        && strcmp(
                            &raw mut (*targets.offset(i as isize)).target_type
                                as *mut ::core::ffi::c_char,
                            b"s3\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0
                    {
                        storage_target_probe_and_publish(
                            &raw mut (*targets.offset(i as isize)).uuid as *mut ::core::ffi::c_char,
                            r#false != 0,
                            targets.offset(i as isize),
                        );
                    }
                    i += 1;
                }
                free(targets as *mut ::core::ffi::c_void);
            }
            db_storage_lifecycle_expire(32 as ::core::ffi::c_int);
            let mut scheduled: ::core::ffi::c_int =
                db_storage_lifecycle_schedule(16 as ::core::ffi::c_int);
            let mut violations: ::core::ffi::c_int = db_storage_lifecycle_reconcile();
            if scheduled > 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Scheduled %d policy-driven storage lifecycle job(s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    scheduled,
                );
                continue;
            } else if scheduled < 0 as ::core::ffi::c_int || violations < 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Could not refresh storage lifecycle policy state\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        }
        if result > 0 as ::core::ffi::c_int {
            continue;
        }
        pthread_mutex_lock(&raw mut worker.mutex);
        if worker.running {
            let mut deadline: timespec = timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            clock_gettime(CLOCK_REALTIME, &raw mut deadline);
            deadline.tv_sec += (if result < 0 as ::core::ffi::c_int {
                5 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as __time_t;
            pthread_cond_timedwait(
                &raw mut worker.condition,
                &raw mut worker.mutex,
                &raw mut deadline,
            );
        }
        pthread_mutex_unlock(&raw mut worker.mutex);
    }
    pthread_mutex_lock(&raw mut worker.mutex);
    worker.exited = r#true != 0;
    pthread_mutex_unlock(&raw mut worker.mutex);
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_migration_worker_start() -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut worker.mutex);
    if worker.running {
        pthread_mutex_unlock(&raw mut worker.mutex);
        return 0 as ::core::ffi::c_int;
    }
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut stop_requested).store(false, ::core::sync::atomic::Ordering::SeqCst);
    worker.running = r#true != 0;
    worker.exited = r#false != 0;
    if pthread_create(
        &raw mut worker.thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            migration_worker_main
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    ) != 0 as ::core::ffi::c_int
    {
        worker.running = r#false != 0;
        worker.exited = r#true != 0;
        pthread_mutex_unlock(&raw mut worker.mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut worker.mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Durable storage migration worker started\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_migration_worker_wake() {
    pthread_mutex_lock(&raw mut worker.mutex);
    pthread_cond_broadcast(&raw mut worker.condition);
    pthread_mutex_unlock(&raw mut worker.mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn storage_migration_worker_shutdown() {
    pthread_mutex_lock(&raw mut worker.mutex);
    if !worker.running {
        pthread_mutex_unlock(&raw mut worker.mutex);
        return;
    }
    ::core::sync::atomic::AtomicBool::from_ptr(&raw mut stop_requested).store(true, ::core::sync::atomic::Ordering::SeqCst);
    worker.running = r#false != 0;
    pthread_cond_broadcast(&raw mut worker.condition);
    pthread_mutex_unlock(&raw mut worker.mutex);
    pthread_join(
        worker.thread,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Durable storage migration worker stopped\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENAMETOOLONG: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
