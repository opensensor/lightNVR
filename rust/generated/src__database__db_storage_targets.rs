use ::c2rust_bitfields;
use ::libc;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sqlite3_stmt;
    pub type sqlite3;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
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
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_double(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_double,
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
    unsafe fn sqlite3_bind_null(_: *mut sqlite3_stmt, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_double(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
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
    unsafe fn sqlite3_column_type(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
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
    unsafe fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
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
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn statvfs(
        __file: *const ::core::ffi::c_char,
        __buf: *mut statvfs,
    ) -> ::core::ffi::c_int;
    unsafe fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn copy_trimmed_value(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
        input: *const ::core::ffi::c_char,
        input_size: size_t,
    ) -> size_t;
    unsafe fn storage_s3_validate(
        target: *const storage_target_t,
        error: *mut ::core::ffi::c_char,
        length: size_t,
    ) -> bool;
    unsafe fn storage_s3_probe(
        target: *mut storage_target_t,
        write_test: bool,
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
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn mkstemp(__template: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn realpath(
        __name: *const ::core::ffi::c_char,
        __resolved: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
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
pub type __fsblkcnt_t = ::core::ffi::c_ulong;
pub type __fsfilcnt_t = ::core::ffi::c_ulong;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ssize_t = isize;
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
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
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
pub type uint64_t = u64;
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
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const LIGHTNVR_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const STORAGE_TARGET_OBJECT_KEY_MAX: ::core::ffi::c_int = MAX_PATH_LENGTH;
pub const STORAGE_TARGET_MAX_COUNT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut ::core::ffi::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as ::core::ffi::c_int, __stream);
}
pub const W_OK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const X_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
unsafe extern "C" fn set_error(
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    if error.is_null()
        || error_size == 0 as size_t
        || *error.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        return;
    }
    let mut arguments: ::core::ffi::VaList;
    arguments = c2rust_args.clone();
    vsnprintf(error, error_size, format, arguments.clone());
}
unsafe extern "C" fn copy_column(
    mut destination: *mut ::core::ffi::c_char,
    mut destination_size: size_t,
    mut statement: *mut sqlite3_stmt,
    mut column: ::core::ffi::c_int,
) {
    let mut value: *const ::core::ffi::c_char =
        sqlite3_column_text(statement, column) as *const ::core::ffi::c_char;
    safe_strcpy(
        destination,
        if !value.is_null() {
            value
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        destination_size,
        0 as size_t,
    );
}
unsafe extern "C" fn populate(mut statement: *mut sqlite3_stmt, mut target: *mut storage_target_t) {
    memset(
        target as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_target_t>(),
    );
    copy_column(
        &raw mut (*target).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        0 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*target).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        statement,
        1 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*target).target_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        statement,
        2 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*target).root_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        3 as ::core::ffi::c_int,
    );
    (*target).enabled =
        sqlite3_column_int(statement, 4 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    (*target).is_default =
        sqlite3_column_int(statement, 5 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    copy_column(
        &raw mut (*target).storage_class as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        statement,
        6 as ::core::ffi::c_int,
    );
    (*target).reserve_bytes = sqlite3_column_int64(statement, 7 as ::core::ffi::c_int) as uint64_t;
    (*target).high_watermark_pct = sqlite3_column_double(statement, 8 as ::core::ffi::c_int);
    (*target).low_watermark_pct = sqlite3_column_double(statement, 9 as ::core::ffi::c_int);
    copy_column(
        &raw mut (*target).health_status as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        statement,
        10 as ::core::ffi::c_int,
    );
    (*target).capacity_bytes =
        sqlite3_column_int64(statement, 11 as ::core::ffi::c_int) as uint64_t;
    (*target).available_bytes =
        sqlite3_column_int64(statement, 12 as ::core::ffi::c_int) as uint64_t;
    (*target).filesystem_device =
        sqlite3_column_int64(statement, 13 as ::core::ffi::c_int) as uint64_t;
    (*target).last_probe_at =
        (if sqlite3_column_type(statement, 14 as ::core::ffi::c_int) == SQLITE_NULL {
            0 as sqlite3_int64
        } else {
            sqlite3_column_int64(statement, 14 as ::core::ffi::c_int)
        }) as int64_t;
    (*target).last_success_at =
        (if sqlite3_column_type(statement, 15 as ::core::ffi::c_int) == SQLITE_NULL {
            0 as sqlite3_int64
        } else {
            sqlite3_column_int64(statement, 15 as ::core::ffi::c_int)
        }) as int64_t;
    copy_column(
        &raw mut (*target).last_error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        statement,
        16 as ::core::ffi::c_int,
    );
    (*target).revision = sqlite3_column_int64(statement, 17 as ::core::ffi::c_int) as int64_t;
    (*target).created_at = sqlite3_column_int64(statement, 18 as ::core::ffi::c_int) as int64_t;
    (*target).updated_at = sqlite3_column_int64(statement, 19 as ::core::ffi::c_int) as int64_t;
    (*target).recording_count =
        sqlite3_column_int64(statement, 20 as ::core::ffi::c_int) as uint64_t;
    (*target).recording_bytes =
        sqlite3_column_int64(statement, 21 as ::core::ffi::c_int) as uint64_t;
    (*target).mount_required =
        sqlite3_column_int(statement, 22 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    copy_column(
        &raw mut (*target).mount_guard_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        23 as ::core::ffi::c_int,
    );
    (*target).migration_bandwidth_bps =
        sqlite3_column_int64(statement, 24 as ::core::ffi::c_int) as uint64_t;
    (*target).archival_window_start_minute =
        sqlite3_column_int(statement, 25 as ::core::ffi::c_int);
    (*target).archival_window_end_minute = sqlite3_column_int(statement, 26 as ::core::ffi::c_int);
    (*target).replica_count = sqlite3_column_int64(statement, 27 as ::core::ffi::c_int) as uint64_t;
    (*target).replica_bytes = sqlite3_column_int64(statement, 28 as ::core::ffi::c_int) as uint64_t;
    copy_column(
        &raw mut (*target).endpoint as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        statement,
        29 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*target).region as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        statement,
        30 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*target).bucket as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        statement,
        31 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*target).credential_ref as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        statement,
        32 as ::core::ffi::c_int,
    );
    (*target).archive_budget_bytes =
        sqlite3_column_int64(statement, 33 as ::core::ffi::c_int) as uint64_t;
}
unsafe extern "C" fn valid_text(
    mut value: *const ::core::ffi::c_char,
    mut maximum: size_t,
    mut required: bool,
) -> bool {
    if value.is_null() {
        return r#false != 0;
    }
    let mut length: size_t = strnlen(value, maximum);
    if length == maximum || required as ::core::ffi::c_int != 0 && length == 0 as size_t {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = value as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_IScntrl.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn path_has_unsafe_segment(mut path: *const ::core::ffi::c_char) -> bool {
    if path.is_null() {
        return r#true != 0;
    }
    let mut segment: *const ::core::ffi::c_char = path;
    while *segment != 0 {
        while *segment as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
            segment = segment.offset(1);
        }
        let mut end: *const ::core::ffi::c_char = segment;
        while *end as ::core::ffi::c_int != 0
            && *end as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        {
            end = end.offset(1);
        }
        let mut length: size_t = end.offset_from(segment) as size_t;
        if length == 1 as size_t
            && *segment.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            || length == 2 as size_t
                && *segment.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && *segment.offset(1isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        segment = end;
    }
    return r#false != 0;
}
unsafe extern "C" fn normalize_root_path(mut path: *mut ::core::ffi::c_char) -> bool {
    if !valid_text(path, MAX_PATH_LENGTH as size_t, r#true != 0)
        || *path.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        || strcmp(path, b"/\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        || !strstr(path, b"//\0".as_ptr() as *const ::core::ffi::c_char).is_null()
        || path_has_unsafe_segment(path) as ::core::ffi::c_int != 0
    {
        return r#false != 0;
    }
    let mut length: size_t = strlen(path);
    while length > 1 as size_t
        && *path.offset(length.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            == '/' as ::core::ffi::c_int
    {
        length = length.wrapping_sub(1);
        *path.offset(length as isize) = '\0' as ::core::ffi::c_char;
    }
    if length <= 1 as size_t {
        return r#false != 0;
    }
    let mut resolved: [::core::ffi::c_char; 4096] = [0; 4096];
    if !realpath(path, &raw mut resolved as *mut ::core::ffi::c_char).is_null()
        && resolved[0usize] as ::core::ffi::c_int == '/' as ::core::ffi::c_int
        && strcmp(
            &raw mut resolved as *mut ::core::ffi::c_char,
            b"/\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        && strlen(&raw mut resolved as *mut ::core::ffi::c_char) < MAX_PATH_LENGTH as size_t
        && !path_has_unsafe_segment(&raw mut resolved as *mut ::core::ffi::c_char)
    {
        safe_strcpy(
            path,
            &raw mut resolved as *mut ::core::ffi::c_char,
            MAX_PATH_LENGTH as size_t,
            0 as size_t,
        );
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_object_key(mut key: *const ::core::ffi::c_char) -> bool {
    if !valid_text(key, STORAGE_TARGET_OBJECT_KEY_MAX as size_t, r#true != 0)
        || *key.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
        || !strchr(key, '\\' as ::core::ffi::c_int).is_null()
        || path_has_unsafe_segment(key) as ::core::ffi::c_int != 0
    {
        return r#false != 0;
    }
    let mut length: size_t = strlen(key);
    return *key.offset(length.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        != '/' as ::core::ffi::c_int;
}
unsafe extern "C" fn path_is_within(
    mut path: *const ::core::ffi::c_char,
    mut root: *const ::core::ffi::c_char,
) -> bool {
    if path.is_null()
        || root.is_null()
        || *path.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        || *root.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut root_length: size_t = strlen(root);
    if root_length == 1 as size_t
        && *root.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    return strncmp(path, root, root_length) == 0 as ::core::ffi::c_int
        && (*path.offset(root_length as isize) as ::core::ffi::c_int
            == '\0' as ::core::ffi::c_int
            || *path.offset(root_length as isize) as ::core::ffi::c_int
                == '/' as ::core::ffi::c_int);
}
unsafe extern "C" fn decode_mount_path(
    mut encoded: *const ::core::ffi::c_char,
    mut decoded: *mut ::core::ffi::c_char,
    mut decoded_size: size_t,
) -> bool {
    if encoded.is_null() || decoded.is_null() || decoded_size == 0 as size_t {
        return r#false != 0;
    }
    let mut encoded_length: size_t = strlen(encoded);
    let mut output: size_t = 0 as size_t;
    let mut input: size_t = 0 as size_t;
    while input < encoded_length {
        let mut value: ::core::ffi::c_uchar =
            *encoded.offset(input as isize) as ::core::ffi::c_uchar;
        if value as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
            && input.wrapping_add(3 as size_t) < encoded_length
            && *encoded.offset(input.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                >= '0' as ::core::ffi::c_int
            && *encoded.offset(input.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                <= '7' as ::core::ffi::c_int
            && *encoded.offset(input.wrapping_add(2 as size_t) as isize) as ::core::ffi::c_int
                >= '0' as ::core::ffi::c_int
            && *encoded.offset(input.wrapping_add(2 as size_t) as isize) as ::core::ffi::c_int
                <= '7' as ::core::ffi::c_int
            && *encoded.offset(input.wrapping_add(3 as size_t) as isize) as ::core::ffi::c_int
                >= '0' as ::core::ffi::c_int
            && *encoded.offset(input.wrapping_add(3 as size_t) as isize) as ::core::ffi::c_int
                <= '7' as ::core::ffi::c_int
        {
            value = ((*encoded.offset(input.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_int
                - '0' as ::core::ffi::c_int)
                * 64 as ::core::ffi::c_int
                + (*encoded.offset(input.wrapping_add(2 as size_t) as isize) as ::core::ffi::c_int
                    - '0' as ::core::ffi::c_int)
                    * 8 as ::core::ffi::c_int
                + (*encoded.offset(input.wrapping_add(3 as size_t) as isize) as ::core::ffi::c_int
                    - '0' as ::core::ffi::c_int)) as ::core::ffi::c_uchar;
            input = input.wrapping_add(3 as size_t);
        }
        if value as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            || output.wrapping_add(1 as size_t) >= decoded_size
        {
            return r#false != 0;
        }
        let c2rust_fresh1 = output;
        output = output.wrapping_add(1);
        *decoded.offset(c2rust_fresh1 as isize) = value as ::core::ffi::c_char;
        input = input.wrapping_add(1);
    }
    *decoded.offset(output as isize) = '\0' as ::core::ffi::c_char;
    return r#true != 0;
}
unsafe extern "C" fn find_mount(
    mut root_path: *const ::core::ffi::c_char,
    mut mountinfo_path: *const ::core::ffi::c_char,
    mut exact_path: *const ::core::ffi::c_char,
    mut mount_path: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if root_path.is_null()
        || *root_path.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        || mountinfo_path.is_null()
        || mount_path.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    *mount_path.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut mountinfo: *mut FILE = fopen(
        mountinfo_path,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if mountinfo.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut line_capacity: size_t = 0 as size_t;
    let mut line_length: ssize_t = 0;
    let mut best_length: size_t = 0 as size_t;
    loop {
        line_length = getline(&raw mut line, &raw mut line_capacity, mountinfo) as ssize_t;
        if line_length < 0 as ssize_t {
            break;
        }
        let mut encoded: [::core::ffi::c_char; 512] = [0; 512];
        if sscanf(
            line,
            b"%*s %*s %*s %*s %511s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut encoded as *mut ::core::ffi::c_char,
        ) != 1 as ::core::ffi::c_int
        {
            continue;
        }
        let mut decoded: [::core::ffi::c_char; 512] = [0; 512];
        if !decode_mount_path(
            &raw mut encoded as *mut ::core::ffi::c_char,
            &raw mut decoded as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        ) {
            continue;
        }
        if !exact_path.is_null() {
            if strcmp(&raw mut decoded as *mut ::core::ffi::c_char, exact_path)
                != 0 as ::core::ffi::c_int
            {
                continue;
            }
            safe_strcpy(
                mount_path,
                &raw mut decoded as *mut ::core::ffi::c_char,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
            best_length = strlen(&raw mut decoded as *mut ::core::ffi::c_char);
            break;
        } else {
            let mut length: size_t = strlen(&raw mut decoded as *mut ::core::ffi::c_char);
            if strcmp(
                &raw mut decoded as *mut ::core::ffi::c_char,
                b"/\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || length <= best_length
                || !path_is_within(root_path, &raw mut decoded as *mut ::core::ffi::c_char)
            {
                continue;
            }
            safe_strcpy(
                mount_path,
                &raw mut decoded as *mut ::core::ffi::c_char,
                MAX_PATH_LENGTH as size_t,
                0 as size_t,
            );
            best_length = length;
        }
    }
    free(line as *mut ::core::ffi::c_void);
    fclose(mountinfo);
    return if best_length > 0 as size_t {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_detect_mount(
    mut root_path: *const ::core::ffi::c_char,
    mut mountinfo_path: *const ::core::ffi::c_char,
    mut mount_path: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return find_mount(
        root_path,
        mountinfo_path,
        ::core::ptr::null::<::core::ffi::c_char>(),
        mount_path,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_mount_guard_active(
    mut target: *const storage_target_t,
) -> bool {
    if target.is_null() {
        return r#false != 0;
    }
    if !(*target).mount_required {
        return r#true != 0;
    }
    if (*target).mount_guard_path[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut active_mount: [::core::ffi::c_char; 512] = [0; 512];
    return find_mount(
        &raw const (*target).root_path as *const ::core::ffi::c_char,
        b"/proc/self/mountinfo\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).mount_guard_path as *const ::core::ffi::c_char,
        &raw mut active_mount as *mut ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_validate(
    mut target: *mut storage_target_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> db_storage_target_result_t {
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if target.is_null() {
        set_error(
            error,
            error_size,
            b"storage target is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    let mut normalized_name: [::core::ffi::c_char; 128] = [0; 128];
    if !valid_text(
        &raw mut (*target).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        r#true != 0,
    ) || copy_trimmed_value(
        &raw mut normalized_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        &raw mut (*target).name as *mut ::core::ffi::c_char,
        0 as size_t,
    ) == 0 as size_t
    {
        set_error(
            error,
            error_size,
            b"target name cannot be blank\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    safe_strcpy(
        &raw mut (*target).name as *mut ::core::ffi::c_char,
        &raw mut normalized_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    let mut object_target: bool = strcmp(
        &raw mut (*target).target_type as *mut ::core::ffi::c_char,
        b"s3\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    if object_target as ::core::ffi::c_int != 0 && !storage_s3_validate(target, error, error_size) {
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    if !object_target
        && !normalize_root_path(&raw mut (*target).root_path as *mut ::core::ffi::c_char)
    {
        set_error(
            error,
            error_size,
            b"root_path must be an absolute non-root path without traversal segments\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    if !object_target
        && strcmp(
            &raw mut (*target).target_type as *mut ::core::ffi::c_char,
            b"filesystem\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"target_type must be filesystem or s3\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    if strcmp(
        &raw mut (*target).storage_class as *mut ::core::ffi::c_char,
        b"hot\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut (*target).storage_class as *mut ::core::ffi::c_char,
            b"warm\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut (*target).storage_class as *mut ::core::ffi::c_char,
            b"cold\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"storage_class must be hot, warm, or cold\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    if (*target).high_watermark_pct <= 0.0f64
        || (*target).high_watermark_pct >= 100.0f64
        || (*target).low_watermark_pct < 0.0f64
        || (*target).low_watermark_pct >= (*target).high_watermark_pct
    {
        set_error(
            error,
            error_size,
            b"watermarks must satisfy 0 <= low < high < 100\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    if (*target).reserve_bytes > INT64_MAX as uint64_t
        || (*target).archive_budget_bytes > INT64_MAX as uint64_t
    {
        set_error(
            error,
            error_size,
            b"reserve_bytes exceeds the supported SQLite integer range\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    if (*target).migration_bandwidth_bps > INT64_MAX as uint64_t
        || (*target).migration_bandwidth_bps > 0 as uint64_t
            && (*target).migration_bandwidth_bps < 65536 as uint64_t
    {
        set_error(
            error,
            error_size,
            b"migration bandwidth must be zero or at least 65536 bytes/second\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    if (*target).archival_window_start_minute < 0 as ::core::ffi::c_int
        || (*target).archival_window_start_minute > 1439 as ::core::ffi::c_int
        || (*target).archival_window_end_minute < 0 as ::core::ffi::c_int
        || (*target).archival_window_end_minute > 1439 as ::core::ffi::c_int
    {
        set_error(
            error,
            error_size,
            b"archival window minutes must be between 0 and 1439\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    if (*target).mount_guard_path[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && ((*target).mount_guard_path[0usize] as ::core::ffi::c_int != '/' as ::core::ffi::c_int
            || strcmp(
                &raw mut (*target).mount_guard_path as *mut ::core::ffi::c_char,
                b"/\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || !path_is_within(
                &raw mut (*target).root_path as *mut ::core::ffi::c_char,
                &raw mut (*target).mount_guard_path as *mut ::core::ffi::c_char,
            ))
    {
        set_error(
            error,
            error_size,
            b"mount guard must be a non-root mount containing root_path\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    if !(*target).mount_required {
        (*target).mount_guard_path[0usize] = '\0' as ::core::ffi::c_char;
    }
    return db_storage_target_result_t::DB_STORAGE_TARGET_OK;
}
unsafe extern "C" fn get_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut target: *mut storage_target_t,
) -> db_storage_target_result_t {
    let mut sql: *const ::core::ffi::c_char = b"SELECT t.uuid,t.name,t.target_type,t.root_path,t.enabled,t.is_default,t.storage_class,t.reserve_bytes,t.high_watermark_pct,t.low_watermark_pct,t.health_status,t.capacity_bytes,t.available_bytes,t.filesystem_device,t.last_probe_at,t.last_success_at,t.last_error,t.revision,t.created_at,t.updated_at,t.recording_count,t.recording_bytes,t.mount_required,t.mount_guard_path,t.migration_bandwidth_bps,t.archival_window_start_minute,t.archival_window_end_minute,t.replica_count,t.replica_bytes,t.endpoint,t.region,t.bucket,t.credential_ref,t.archive_budget_bytes FROM storage_targets t WHERE t.uuid=? LIMIT 1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
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
        result = sqlite3_step(statement);
    }
    let mut outcome: db_storage_target_result_t =
        db_storage_target_result_t::DB_STORAGE_TARGET_NOT_FOUND;
    if result == SQLITE_ROW {
        populate(statement, target);
        outcome = db_storage_target_result_t::DB_STORAGE_TARGET_OK;
    } else if result != SQLITE_DONE {
        outcome = db_storage_target_result_t::DB_STORAGE_TARGET_ERROR;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_count() -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        b"SELECT count(*) FROM storage_targets;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
        && sqlite3_step(statement) == SQLITE_ROW
    {
        count = sqlite3_column_int(statement, 0 as ::core::ffi::c_int);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_list(
    mut targets: *mut storage_target_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if targets.is_null() || max_count <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT t.uuid,t.name,t.target_type,t.root_path,t.enabled,t.is_default,t.storage_class,t.reserve_bytes,t.high_watermark_pct,t.low_watermark_pct,t.health_status,t.capacity_bytes,t.available_bytes,t.filesystem_device,t.last_probe_at,t.last_success_at,t.last_error,t.revision,t.created_at,t.updated_at,t.recording_count,t.recording_bytes,t.mount_required,t.mount_guard_path,t.migration_bandwidth_bps,t.archival_window_start_minute,t.archival_window_end_minute,t.replica_count,t.replica_bytes,t.endpoint,t.region,t.bucket,t.credential_ref,t.archive_budget_bytes FROM storage_targets t ORDER BY t.is_default DESC,t.name COLLATE NOCASE,t.uuid;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if result == SQLITE_OK {
        while count < max_count && {
            result = sqlite3_step(statement);
            result == SQLITE_ROW
        } {
            let c2rust_fresh0 = count;
            count += 1;
            populate(statement, targets.offset(c2rust_fresh0 as isize));
        }
    }
    if result != SQLITE_DONE && result != SQLITE_ROW {
        count = -1 as ::core::ffi::c_int;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_get(
    mut uuid: *const ::core::ffi::c_char,
    mut target: *mut storage_target_t,
) -> db_storage_target_result_t {
    if !lightnvr_uuid_is_valid(uuid) || target.is_null() {
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_target_result_t::DB_STORAGE_TARGET_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut result: db_storage_target_result_t = get_locked(db, uuid, target);
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_get_default(
    mut target: *mut storage_target_t,
) -> db_storage_target_result_t {
    if target.is_null() {
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_target_result_t::DB_STORAGE_TARGET_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT t.uuid,t.name,t.target_type,t.root_path,t.enabled,t.is_default,t.storage_class,t.reserve_bytes,t.high_watermark_pct,t.low_watermark_pct,t.health_status,t.capacity_bytes,t.available_bytes,t.filesystem_device,t.last_probe_at,t.last_success_at,t.last_error,t.revision,t.created_at,t.updated_at,t.recording_count,t.recording_bytes,t.mount_required,t.mount_guard_path,t.migration_bandwidth_bps,t.archival_window_start_minute,t.archival_window_end_minute,t.replica_count,t.replica_bytes,t.endpoint,t.region,t.bucket,t.credential_ref,t.archive_budget_bytes FROM storage_targets t WHERE t.is_default=1 LIMIT 1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut outcome: db_storage_target_result_t =
        db_storage_target_result_t::DB_STORAGE_TARGET_NOT_FOUND;
    if result == SQLITE_OK {
        result = sqlite3_step(statement);
    }
    if result == SQLITE_ROW {
        populate(statement, target);
        outcome = db_storage_target_result_t::DB_STORAGE_TARGET_OK;
    } else if result != SQLITE_DONE {
        outcome = db_storage_target_result_t::DB_STORAGE_TARGET_ERROR;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return outcome;
}
unsafe extern "C" fn probe_path(
    mut target: *mut storage_target_t,
    mut write_test: bool,
) -> db_storage_target_result_t {
    if (*target).enabled as ::core::ffi::c_int != 0
        && strcmp(
            &raw mut (*target).target_type as *mut ::core::ffi::c_char,
            b"s3\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        return db_storage_target_result_t(
            if storage_s3_probe(target, write_test) == 0 as ::core::ffi::c_int {
                db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
            } else {
                db_storage_target_result_t::DB_STORAGE_TARGET_UNAVAILABLE.0
            },
        );
    }
    (*target).last_probe_at = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    (*target).capacity_bytes = 0 as uint64_t;
    (*target).available_bytes = 0 as uint64_t;
    (*target).filesystem_device = 0 as uint64_t;
    (*target).last_error[0usize] = '\0' as ::core::ffi::c_char;
    if !(*target).enabled {
        safe_strcpy(
            &raw mut (*target).health_status as *mut ::core::ffi::c_char,
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        return db_storage_target_result_t::DB_STORAGE_TARGET_OK;
    }
    if (*target).mount_required {
        let mut active_mount: [::core::ffi::c_char; 512] = [0; 512];
        let mut mount_result: ::core::ffi::c_int = 0;
        if (*target).mount_guard_path[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            mount_result = find_mount(
                &raw mut (*target).root_path as *mut ::core::ffi::c_char,
                b"/proc/self/mountinfo\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut active_mount as *mut ::core::ffi::c_char,
            );
            if mount_result == 0 as ::core::ffi::c_int {
                safe_strcpy(
                    &raw mut (*target).mount_guard_path as *mut ::core::ffi::c_char,
                    &raw mut active_mount as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                    0 as size_t,
                );
            }
        } else {
            mount_result = find_mount(
                &raw mut (*target).root_path as *mut ::core::ffi::c_char,
                b"/proc/self/mountinfo\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*target).mount_guard_path as *mut ::core::ffi::c_char,
                &raw mut active_mount as *mut ::core::ffi::c_char,
            );
        }
        if mount_result != 0 as ::core::ffi::c_int {
            if (*target).mount_guard_path[0usize] != 0 {
                snprintf(
                    &raw mut (*target).last_error as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    b"Required mount is absent: %.200s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*target).mount_guard_path as *mut ::core::ffi::c_char,
                );
            } else {
                safe_strcpy(
                    &raw mut (*target).last_error as *mut ::core::ffi::c_char,
                    b"No distinct mounted filesystem contains this target\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    0 as size_t,
                );
            }
            safe_strcpy(
                &raw mut (*target).health_status as *mut ::core::ffi::c_char,
                b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
            return db_storage_target_result_t::DB_STORAGE_TARGET_UNAVAILABLE;
        }
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
    if stat(
        &raw mut (*target).root_path as *mut ::core::ffi::c_char,
        &raw mut info,
    ) != 0 as ::core::ffi::c_int
        || !(info.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
    {
        snprintf(
            &raw mut (*target).last_error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"Target directory is unavailable: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        safe_strcpy(
            &raw mut (*target).health_status as *mut ::core::ffi::c_char,
            b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        return db_storage_target_result_t::DB_STORAGE_TARGET_UNAVAILABLE;
    }
    (*target).filesystem_device = info.st_dev as uint64_t;
    let mut filesystem: statvfs = statvfs {
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
        &raw mut (*target).root_path as *mut ::core::ffi::c_char,
        &raw mut filesystem,
    ) != 0 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut (*target).last_error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"Capacity probe failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        safe_strcpy(
            &raw mut (*target).health_status as *mut ::core::ffi::c_char,
            b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        return db_storage_target_result_t::DB_STORAGE_TARGET_UNAVAILABLE;
    }
    let mut fragment: uint64_t = filesystem.f_frsize as uint64_t;
    (*target).capacity_bytes = (filesystem.f_blocks as uint64_t).wrapping_mul(fragment);
    (*target).available_bytes = (filesystem.f_bavail as uint64_t).wrapping_mul(fragment);
    if access(
        &raw mut (*target).root_path as *mut ::core::ffi::c_char,
        W_OK | X_OK,
    ) != 0 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut (*target).last_error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"Target directory is not writable: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        safe_strcpy(
            &raw mut (*target).health_status as *mut ::core::ffi::c_char,
            b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        return db_storage_target_result_t::DB_STORAGE_TARGET_UNAVAILABLE;
    }
    if write_test {
        let mut probe_path_0: [::core::ffi::c_char; 512] = [0; 512];
        let mut written: ::core::ffi::c_int = snprintf(
            &raw mut probe_path_0 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/.lightnvr-storage-probe-XXXXXX\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*target).root_path as *mut ::core::ffi::c_char,
        );
        if written < 0 as ::core::ffi::c_int
            || written >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*target).last_error as *mut ::core::ffi::c_char,
                b"Probe path is too long\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*target).health_status as *mut ::core::ffi::c_char,
                b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
            return db_storage_target_result_t::DB_STORAGE_TARGET_UNAVAILABLE;
        }
        let mut descriptor: ::core::ffi::c_int =
            mkstemp(&raw mut probe_path_0 as *mut ::core::ffi::c_char);
        if descriptor < 0 as ::core::ffi::c_int
            || write(
                descriptor,
                b"1\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                1 as size_t,
            ) != 1 as ssize_t
            || fsync(descriptor) != 0 as ::core::ffi::c_int
        {
            let mut saved_errno: ::core::ffi::c_int = *__errno_location();
            if descriptor >= 0 as ::core::ffi::c_int {
                close(descriptor);
            }
            unlink(&raw mut probe_path_0 as *mut ::core::ffi::c_char);
            snprintf(
                &raw mut (*target).last_error as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                b"Write probe failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(saved_errno),
            );
            safe_strcpy(
                &raw mut (*target).health_status as *mut ::core::ffi::c_char,
                b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
            return db_storage_target_result_t::DB_STORAGE_TARGET_UNAVAILABLE;
        }
        close(descriptor);
        if unlink(&raw mut probe_path_0 as *mut ::core::ffi::c_char) != 0 as ::core::ffi::c_int {
            snprintf(
                &raw mut (*target).last_error as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                b"Probe cleanup failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            safe_strcpy(
                &raw mut (*target).health_status as *mut ::core::ffi::c_char,
                b"degraded\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                0 as size_t,
            );
            return db_storage_target_result_t::DB_STORAGE_TARGET_UNAVAILABLE;
        }
    }
    let mut used_pct: ::core::ffi::c_double = if (*target).capacity_bytes > 0 as uint64_t {
        100.0f64
            * (*target)
                .capacity_bytes
                .wrapping_sub((*target).available_bytes) as ::core::ffi::c_double
            / (*target).capacity_bytes as ::core::ffi::c_double
    } else {
        100.0f64
    };
    let mut health: *const ::core::ffi::c_char = if (*target).available_bytes
        <= (*target).reserve_bytes
        || used_pct >= (*target).high_watermark_pct
    {
        b"degraded\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"healthy\0".as_ptr() as *const ::core::ffi::c_char
    };
    safe_strcpy(
        &raw mut (*target).health_status as *mut ::core::ffi::c_char,
        health,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    (*target).last_success_at = (*target).last_probe_at;
    return db_storage_target_result_t::DB_STORAGE_TARGET_OK;
}
unsafe extern "C" fn persist_health(mut target: *const storage_target_t) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"UPDATE storage_targets SET health_status=?,capacity_bytes=?,available_bytes=?,filesystem_device=?,last_probe_at=?,last_success_at=CASE WHEN ?>0 THEN ? ELSE last_success_at END,last_error=?,mount_guard_path=? WHERE uuid=?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw const (*target).health_status as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            2 as ::core::ffi::c_int,
            (*target).capacity_bytes as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            3 as ::core::ffi::c_int,
            (*target).available_bytes as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            4 as ::core::ffi::c_int,
            (*target).filesystem_device as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            5 as ::core::ffi::c_int,
            (*target).last_probe_at as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            6 as ::core::ffi::c_int,
            (*target).last_success_at as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            7 as ::core::ffi::c_int,
            (*target).last_success_at as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            8 as ::core::ffi::c_int,
            &raw const (*target).last_error as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            9 as ::core::ffi::c_int,
            &raw const (*target).mount_guard_path as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            10 as ::core::ffi::c_int,
            &raw const (*target).uuid as *const ::core::ffi::c_char,
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
    pthread_mutex_unlock(mutex);
    return if result == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn valid_normalized_probe_error(mut value: *const ::core::ffi::c_char) -> bool {
    static mut allowed: [*const ::core::ffi::c_char; 11] = [
        b"none\0".as_ptr() as *const ::core::ffi::c_char,
        b"not_found\0".as_ptr() as *const ::core::ffi::c_char,
        b"permission\0".as_ptr() as *const ::core::ffi::c_char,
        b"read_only\0".as_ptr() as *const ::core::ffi::c_char,
        b"no_space\0".as_ptr() as *const ::core::ffi::c_char,
        b"io\0".as_ptr() as *const ::core::ffi::c_char,
        b"timed_out\0".as_ptr() as *const ::core::ffi::c_char,
        b"busy\0".as_ptr() as *const ::core::ffi::c_char,
        b"invalid\0".as_ptr() as *const ::core::ffi::c_char,
        b"other\0".as_ptr() as *const ::core::ffi::c_char,
        b"mount_absent\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if value.is_null() {
        return r#false != 0;
    }
    let mut index: size_t = 0 as size_t;
    while index
        < ::core::mem::size_of::<[*const ::core::ffi::c_char; 11]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        if strcmp(value, allowed[index]) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        index = index.wrapping_add(1);
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_record_health(
    mut uuid: *const ::core::ffi::c_char,
    mut update: *const storage_target_health_update_t,
    mut target: *mut storage_target_t,
) -> db_storage_target_result_t {
    if !lightnvr_uuid_is_valid(uuid)
        || update.is_null()
        || (*update).probed_at <= 0 as int64_t
        || (*update).capacity_bytes > INT64_MAX as uint64_t
        || (*update).available_bytes > (*update).capacity_bytes
        || (*update).filesystem_device > INT64_MAX as uint64_t
        || (*update).cleanup_failed as ::core::ffi::c_int != 0
            && (!(*update).write_checked || !(*update).writeable)
        || !(*update).write_checked && (*update).writeable as ::core::ffi::c_int != 0
        || !(*update).available && (*update).writeable as ::core::ffi::c_int != 0
        || !valid_normalized_probe_error(
            &raw const (*update).normalized_error as *const ::core::ffi::c_char,
        )
        || (*update).available as ::core::ffi::c_int != 0
            && strcmp(
                &raw const (*update).normalized_error as *const ::core::ffi::c_char,
                b"none\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            && !((*update).write_checked as ::core::ffi::c_int != 0
                && (!(*update).writeable || (*update).cleanup_failed as ::core::ffi::c_int != 0))
        || !(*update).available
            && strcmp(
                &raw const (*update).normalized_error as *const ::core::ffi::c_char,
                b"none\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
    {
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    let mut snapshot: storage_target_t = storage_target_t {
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
    let mut result: db_storage_target_result_t = db_storage_target_get(uuid, &raw mut snapshot);
    if result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        return result;
    }
    snapshot.last_probe_at = (*update).probed_at;
    snapshot.capacity_bytes = (*update).capacity_bytes;
    snapshot.available_bytes = (*update).available_bytes;
    snapshot.filesystem_device = (*update).filesystem_device;
    safe_strcpy(
        &raw mut snapshot.last_error as *mut ::core::ffi::c_char,
        if strcmp(
            &raw const (*update).normalized_error as *const ::core::ffi::c_char,
            b"none\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            &raw const (*update).normalized_error as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    if !snapshot.enabled {
        safe_strcpy(
            &raw mut snapshot.health_status as *mut ::core::ffi::c_char,
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
    } else if !(*update).available
        || (*update).write_checked as ::core::ffi::c_int != 0 && !(*update).writeable
    {
        safe_strcpy(
            &raw mut snapshot.health_status as *mut ::core::ffi::c_char,
            b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
    } else {
        let mut used_pct: ::core::ffi::c_double = if (*update).capacity_bytes > 0 as uint64_t {
            100.0f64
                * (*update)
                    .capacity_bytes
                    .wrapping_sub((*update).available_bytes)
                    as ::core::ffi::c_double
                / (*update).capacity_bytes as ::core::ffi::c_double
        } else {
            100.0f64
        };
        let mut degraded: bool = (*update).cleanup_failed as ::core::ffi::c_int != 0
            || (*update).available_bytes <= snapshot.reserve_bytes
            || used_pct >= snapshot.high_watermark_pct;
        safe_strcpy(
            &raw mut snapshot.health_status as *mut ::core::ffi::c_char,
            if degraded as ::core::ffi::c_int != 0 {
                b"degraded\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"healthy\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        snapshot.last_success_at = (*update).probed_at;
    }
    if persist_health(&raw mut snapshot) != 0 as ::core::ffi::c_int {
        return db_storage_target_result_t::DB_STORAGE_TARGET_ERROR;
    }
    return db_storage_target_get(
        uuid,
        if !target.is_null() {
            target
        } else {
            &raw mut snapshot
        },
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_recording_growth_bps(
    mut uuid: *const ::core::ffi::c_char,
    mut bytes_per_second: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if bytes_per_second.is_null() || !uuid.is_null() && !lightnvr_uuid_is_valid(uuid) {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT COALESCE(SUM(CASE WHEN size_bytes>0 THEN size_bytes ELSE 0 END),0),MIN(start_time),COUNT(*),strftime('%s','now') FROM recordings WHERE start_time>=strftime('%s','now')-86400 AND (? IS NULL OR storage_target_uuid=?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
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
        } else {
            sqlite3_bind_null(statement, 1 as ::core::ffi::c_int);
            sqlite3_bind_null(statement, 2 as ::core::ffi::c_int);
        }
        result = sqlite3_step(statement);
    }
    if result == SQLITE_ROW {
        let mut bytes: uint64_t =
            sqlite3_column_int64(statement, 0 as ::core::ffi::c_int) as uint64_t;
        let mut first: int64_t =
            sqlite3_column_int64(statement, 1 as ::core::ffi::c_int) as int64_t;
        let mut count: int64_t =
            sqlite3_column_int64(statement, 2 as ::core::ffi::c_int) as int64_t;
        let mut now: int64_t = sqlite3_column_int64(statement, 3 as ::core::ffi::c_int) as int64_t;
        let mut span: int64_t = if count > 0 as int64_t && now > first {
            now - first
        } else {
            0 as int64_t
        };
        *bytes_per_second = if span >= 60 as int64_t {
            bytes as ::core::ffi::c_double / span as ::core::ffi::c_double
        } else {
            0.0f64
        };
        result = SQLITE_DONE;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return if result == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_probe(
    mut uuid: *const ::core::ffi::c_char,
    mut write_test: bool,
    mut target: *mut storage_target_t,
) -> db_storage_target_result_t {
    let mut snapshot: storage_target_t = storage_target_t {
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
    let mut result: db_storage_target_result_t = db_storage_target_get(uuid, &raw mut snapshot);
    if result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        return result;
    }
    let mut probe_result: db_storage_target_result_t = probe_path(&raw mut snapshot, write_test);
    if persist_health(&raw mut snapshot) != 0 as ::core::ffi::c_int {
        return db_storage_target_result_t::DB_STORAGE_TARGET_ERROR;
    }
    result = db_storage_target_get(
        uuid,
        if !target.is_null() {
            target
        } else {
            &raw mut snapshot
        },
    );
    if result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        return result;
    }
    return probe_result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_refresh_health() -> ::core::ffi::c_int {
    let mut total: ::core::ffi::c_int = db_storage_target_count();
    if total < 0 as ::core::ffi::c_int || total > STORAGE_TARGET_MAX_COUNT {
        return -1 as ::core::ffi::c_int;
    }
    if total == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut targets: *mut storage_target_t =
        calloc(total as size_t, ::core::mem::size_of::<storage_target_t>())
            as *mut storage_target_t;
    if targets.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = db_storage_target_list(targets, total);
    if count < 0 as ::core::ffi::c_int {
        free(targets as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    let mut failures: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut result: db_storage_target_result_t = db_storage_target_probe(
            &raw mut (*targets.offset(index as isize)).uuid as *mut ::core::ffi::c_char,
            r#false != 0,
            ::core::ptr::null_mut::<storage_target_t>(),
        );
        if result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
            && result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_UNAVAILABLE.0
        {
            failures += 1;
        }
        index += 1;
    }
    free(targets as *mut ::core::ffi::c_void);
    return if failures == 0 as ::core::ffi::c_int {
        count
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_create(
    mut target: *mut storage_target_t,
) -> db_storage_target_result_t {
    let mut validation_error: [::core::ffi::c_char; 256] = [
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
    if db_storage_target_validate(
        target,
        &raw mut validation_error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    )
    .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
        || (*target).is_default as ::core::ffi::c_int != 0
    {
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    if (*target).enabled as ::core::ffi::c_int != 0
        && probe_path(target, r#true != 0).0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        return db_storage_target_result_t::DB_STORAGE_TARGET_UNAVAILABLE;
    }
    if !(*target).enabled {
        safe_strcpy(
            &raw mut (*target).health_status as *mut ::core::ffi::c_char,
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
    }
    if lightnvr_uuid_generate_v4(&raw mut (*target).uuid as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        return db_storage_target_result_t::DB_STORAGE_TARGET_ERROR;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_target_result_t::DB_STORAGE_TARGET_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO storage_targets(uuid,name,target_type,root_path,enabled,is_default,storage_class,reserve_bytes,high_watermark_pct,low_watermark_pct,health_status,capacity_bytes,available_bytes,filesystem_device,last_probe_at,last_success_at,last_error,mount_required,mount_guard_path,migration_bandwidth_bps,archival_window_start_minute,archival_window_end_minute,endpoint,region,bucket,credential_ref,archive_budget_bytes) VALUES(?,?,?,?,?,0,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut count_statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        b"SELECT count(*) FROM storage_targets;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut count_statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
        && sqlite3_step(count_statement) == SQLITE_ROW
    {
        count = sqlite3_column_int(count_statement, 0 as ::core::ffi::c_int);
    }
    if !count_statement.is_null() {
        sqlite3_finalize(count_statement);
    }
    if count < 0 as ::core::ffi::c_int || count >= STORAGE_TARGET_MAX_COUNT {
        pthread_mutex_unlock(mutex);
        return db_storage_target_result_t(if count < 0 as ::core::ffi::c_int {
            db_storage_target_result_t::DB_STORAGE_TARGET_ERROR.0
        } else {
            db_storage_target_result_t::DB_STORAGE_TARGET_LIMIT.0
        });
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw mut (*target).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            &raw mut (*target).name as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            &raw mut (*target).target_type as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            4 as ::core::ffi::c_int,
            &raw mut (*target).root_path as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            5 as ::core::ffi::c_int,
            if (*target).enabled as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            statement,
            6 as ::core::ffi::c_int,
            &raw mut (*target).storage_class as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            7 as ::core::ffi::c_int,
            (*target).reserve_bytes as sqlite3_int64,
        );
        sqlite3_bind_double(
            statement,
            8 as ::core::ffi::c_int,
            (*target).high_watermark_pct,
        );
        sqlite3_bind_double(
            statement,
            9 as ::core::ffi::c_int,
            (*target).low_watermark_pct,
        );
        sqlite3_bind_text(
            statement,
            10 as ::core::ffi::c_int,
            &raw mut (*target).health_status as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            11 as ::core::ffi::c_int,
            (*target).capacity_bytes as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            12 as ::core::ffi::c_int,
            (*target).available_bytes as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            13 as ::core::ffi::c_int,
            (*target).filesystem_device as sqlite3_int64,
        );
        if (*target).last_probe_at > 0 as int64_t {
            sqlite3_bind_int64(
                statement,
                14 as ::core::ffi::c_int,
                (*target).last_probe_at as sqlite3_int64,
            );
        } else {
            sqlite3_bind_null(statement, 14 as ::core::ffi::c_int);
        }
        if (*target).last_success_at > 0 as int64_t {
            sqlite3_bind_int64(
                statement,
                15 as ::core::ffi::c_int,
                (*target).last_success_at as sqlite3_int64,
            );
        } else {
            sqlite3_bind_null(statement, 15 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            statement,
            16 as ::core::ffi::c_int,
            &raw mut (*target).last_error as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            17 as ::core::ffi::c_int,
            if (*target).mount_required as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            statement,
            18 as ::core::ffi::c_int,
            &raw mut (*target).mount_guard_path as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            19 as ::core::ffi::c_int,
            (*target).migration_bandwidth_bps as sqlite3_int64,
        );
        sqlite3_bind_int(
            statement,
            20 as ::core::ffi::c_int,
            (*target).archival_window_start_minute,
        );
        sqlite3_bind_int(
            statement,
            21 as ::core::ffi::c_int,
            (*target).archival_window_end_minute,
        );
        sqlite3_bind_text(
            statement,
            22 as ::core::ffi::c_int,
            &raw mut (*target).endpoint as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            23 as ::core::ffi::c_int,
            &raw mut (*target).region as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            24 as ::core::ffi::c_int,
            &raw mut (*target).bucket as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            25 as ::core::ffi::c_int,
            &raw mut (*target).credential_ref as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            26 as ::core::ffi::c_int,
            (*target).archive_budget_bytes as sqlite3_int64,
        );
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    if result == SQLITE_CONSTRAINT {
        return db_storage_target_result_t::DB_STORAGE_TARGET_CONFLICT;
    }
    if result != SQLITE_DONE {
        return db_storage_target_result_t::DB_STORAGE_TARGET_ERROR;
    }
    return db_storage_target_get(&raw mut (*target).uuid as *mut ::core::ffi::c_char, target);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_update(
    mut target: *mut storage_target_t,
    mut expected_revision: int64_t,
) -> db_storage_target_result_t {
    if target.is_null()
        || !lightnvr_uuid_is_valid(&raw mut (*target).uuid as *mut ::core::ffi::c_char)
        || expected_revision < 1 as int64_t
    {
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    let mut existing: storage_target_t = storage_target_t {
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
    let mut get_result: db_storage_target_result_t = db_storage_target_get(
        &raw mut (*target).uuid as *mut ::core::ffi::c_char,
        &raw mut existing,
    );
    if get_result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        return get_result;
    }
    if existing.revision != expected_revision {
        return db_storage_target_result_t::DB_STORAGE_TARGET_STALE;
    }
    if strcmp(
        &raw mut existing.target_type as *mut ::core::ffi::c_char,
        &raw mut (*target).target_type as *mut ::core::ffi::c_char,
    ) != 0
        || strcmp(
            &raw mut existing.endpoint as *mut ::core::ffi::c_char,
            &raw mut (*target).endpoint as *mut ::core::ffi::c_char,
        ) != 0
        || strcmp(
            &raw mut existing.bucket as *mut ::core::ffi::c_char,
            &raw mut (*target).bucket as *mut ::core::ffi::c_char,
        ) != 0
        || strcmp(
            &raw mut existing.region as *mut ::core::ffi::c_char,
            &raw mut (*target).region as *mut ::core::ffi::c_char,
        ) != 0
        || strcmp(
            &raw mut existing.target_type as *mut ::core::ffi::c_char,
            b"s3\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && strcmp(
                &raw mut existing.root_path as *mut ::core::ffi::c_char,
                &raw mut (*target).root_path as *mut ::core::ffi::c_char,
            ) != 0
    {
        return db_storage_target_result_t::DB_STORAGE_TARGET_IN_USE;
    }
    if (*target).is_default as ::core::ffi::c_int != existing.is_default as ::core::ffi::c_int
        || existing.is_default as ::core::ffi::c_int != 0 && !(*target).enabled
    {
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    if strcmp(
        &raw mut existing.root_path as *mut ::core::ffi::c_char,
        &raw mut (*target).root_path as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        (*target).mount_guard_path[0usize] = '\0' as ::core::ffi::c_char;
    }
    let mut validation_error: [::core::ffi::c_char; 256] = [
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
    if db_storage_target_validate(
        target,
        &raw mut validation_error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    )
    .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    if strcmp(
        &raw mut existing.root_path as *mut ::core::ffi::c_char,
        &raw mut (*target).root_path as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
        && (existing.is_default as ::core::ffi::c_int != 0
            || existing.recording_count > 0 as uint64_t
            || existing.replica_count > 0 as uint64_t)
    {
        return db_storage_target_result_t::DB_STORAGE_TARGET_IN_USE;
    }
    if (*target).enabled as ::core::ffi::c_int != 0
        && probe_path(target, r#true != 0).0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        return db_storage_target_result_t::DB_STORAGE_TARGET_UNAVAILABLE;
    }
    if !(*target).enabled {
        safe_strcpy(
            &raw mut (*target).health_status as *mut ::core::ffi::c_char,
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        (*target).capacity_bytes = 0 as uint64_t;
        (*target).available_bytes = 0 as uint64_t;
        (*target).filesystem_device = 0 as uint64_t;
        (*target).last_error[0usize] = '\0' as ::core::ffi::c_char;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_target_result_t::DB_STORAGE_TARGET_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = b"UPDATE storage_targets SET name=?,target_type=?,root_path=?,enabled=?,storage_class=?,reserve_bytes=?,high_watermark_pct=?,low_watermark_pct=?,health_status=?,capacity_bytes=?,available_bytes=?,filesystem_device=?,last_probe_at=?,last_success_at=CASE WHEN ?>0 THEN ? ELSE last_success_at END,last_error=?,mount_required=?,mount_guard_path=?,migration_bandwidth_bps=?,archival_window_start_minute=?,archival_window_end_minute=?,credential_ref=?25,archive_budget_bytes=?26,revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=?22 AND revision=?23 AND (root_path=?24 OR (is_default=0 AND recording_count=0 AND replica_count=0 AND NOT EXISTS(SELECT 1 FROM storage_migration_jobs j WHERE (j.source_target_uuid=?22 OR j.destination_target_uuid=?22) AND j.state<>'completed') AND NOT EXISTS(SELECT 1 FROM storage_deletion_objects o WHERE o.target_uuid=?22 AND o.state<>'completed')));\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw mut (*target).name as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            &raw mut (*target).target_type as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            &raw mut (*target).root_path as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            4 as ::core::ffi::c_int,
            if (*target).enabled as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            statement,
            5 as ::core::ffi::c_int,
            &raw mut (*target).storage_class as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            6 as ::core::ffi::c_int,
            (*target).reserve_bytes as sqlite3_int64,
        );
        sqlite3_bind_double(
            statement,
            7 as ::core::ffi::c_int,
            (*target).high_watermark_pct,
        );
        sqlite3_bind_double(
            statement,
            8 as ::core::ffi::c_int,
            (*target).low_watermark_pct,
        );
        sqlite3_bind_text(
            statement,
            9 as ::core::ffi::c_int,
            &raw mut (*target).health_status as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            10 as ::core::ffi::c_int,
            (*target).capacity_bytes as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            11 as ::core::ffi::c_int,
            (*target).available_bytes as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            12 as ::core::ffi::c_int,
            (*target).filesystem_device as sqlite3_int64,
        );
        if (*target).last_probe_at > 0 as int64_t {
            sqlite3_bind_int64(
                statement,
                13 as ::core::ffi::c_int,
                (*target).last_probe_at as sqlite3_int64,
            );
        } else {
            sqlite3_bind_null(statement, 13 as ::core::ffi::c_int);
        }
        sqlite3_bind_int64(
            statement,
            14 as ::core::ffi::c_int,
            (*target).last_success_at as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            15 as ::core::ffi::c_int,
            (*target).last_success_at as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            16 as ::core::ffi::c_int,
            &raw mut (*target).last_error as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            17 as ::core::ffi::c_int,
            if (*target).mount_required as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            statement,
            18 as ::core::ffi::c_int,
            &raw mut (*target).mount_guard_path as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            19 as ::core::ffi::c_int,
            (*target).migration_bandwidth_bps as sqlite3_int64,
        );
        sqlite3_bind_int(
            statement,
            20 as ::core::ffi::c_int,
            (*target).archival_window_start_minute,
        );
        sqlite3_bind_int(
            statement,
            21 as ::core::ffi::c_int,
            (*target).archival_window_end_minute,
        );
        sqlite3_bind_text(
            statement,
            22 as ::core::ffi::c_int,
            &raw mut (*target).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            23 as ::core::ffi::c_int,
            expected_revision as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            24 as ::core::ffi::c_int,
            &raw mut (*target).root_path as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            25 as ::core::ffi::c_int,
            &raw mut (*target).credential_ref as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            26 as ::core::ffi::c_int,
            (*target).archive_budget_bytes as sqlite3_int64,
        );
        result = sqlite3_step(statement);
    }
    let mut changed: ::core::ffi::c_int = sqlite3_changes(db);
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    if result == SQLITE_CONSTRAINT {
        return db_storage_target_result_t::DB_STORAGE_TARGET_CONFLICT;
    }
    if result != SQLITE_DONE {
        return db_storage_target_result_t::DB_STORAGE_TARGET_ERROR;
    }
    if changed == 0 as ::core::ffi::c_int {
        let mut current: storage_target_t = storage_target_t {
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
        let mut current_result: db_storage_target_result_t = db_storage_target_get(
            &raw mut (*target).uuid as *mut ::core::ffi::c_char,
            &raw mut current,
        );
        if current_result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
            return current_result;
        }
        if current.revision != expected_revision {
            return db_storage_target_result_t::DB_STORAGE_TARGET_STALE;
        }
        if strcmp(
            &raw mut current.root_path as *mut ::core::ffi::c_char,
            &raw mut (*target).root_path as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            return db_storage_target_result_t::DB_STORAGE_TARGET_IN_USE;
        }
        return db_storage_target_result_t::DB_STORAGE_TARGET_ERROR;
    }
    return db_storage_target_get(&raw mut (*target).uuid as *mut ::core::ffi::c_char, target);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_delete(
    mut uuid: *const ::core::ffi::c_char,
    mut expected_revision: int64_t,
) -> db_storage_target_result_t {
    if !lightnvr_uuid_is_valid(uuid) || expected_revision < 1 as int64_t {
        return db_storage_target_result_t::DB_STORAGE_TARGET_INVALID;
    }
    let mut existing: storage_target_t = storage_target_t {
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
    let mut result: db_storage_target_result_t = db_storage_target_get(uuid, &raw mut existing);
    if result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        return result;
    }
    if existing.revision != expected_revision {
        return db_storage_target_result_t::DB_STORAGE_TARGET_STALE;
    }
    if existing.is_default as ::core::ffi::c_int != 0
        || existing.recording_count > 0 as uint64_t
        || existing.replica_count > 0 as uint64_t
    {
        return db_storage_target_result_t::DB_STORAGE_TARGET_IN_USE;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_storage_target_result_t::DB_STORAGE_TARGET_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sqlite_result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT EXISTS(SELECT 1 FROM storage_pool_members WHERE target_uuid=?) OR EXISTS(SELECT 1 FROM storage_policies WHERE primary_target_uuid=? OR fallback_target_uuid=? OR migration_target_uuid=?) OR EXISTS(SELECT 1 FROM storage_migration_jobs WHERE source_target_uuid=? OR destination_target_uuid=?) OR EXISTS(SELECT 1 FROM storage_recording_policies WHERE primary_target_uuid=?1 OR migration_target_uuid=?1 OR fallback_target_uuid=?1);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if sqlite_result == SQLITE_OK {
        let mut parameter: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while parameter <= 6 as ::core::ffi::c_int {
            sqlite3_bind_text(
                statement,
                parameter,
                uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            parameter += 1;
        }
        sqlite_result = sqlite3_step(statement);
    }
    let mut referenced: bool = sqlite_result == SQLITE_ROW
        && sqlite3_column_int(statement, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if referenced {
        pthread_mutex_unlock(mutex);
        return db_storage_target_result_t::DB_STORAGE_TARGET_IN_USE;
    }
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    sqlite_result = sqlite3_prepare_v2(
        db,
        b"DELETE FROM storage_targets WHERE uuid=? AND revision=? AND is_default=0 AND recording_count=0 AND replica_count=0;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if sqlite_result == SQLITE_OK {
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
        sqlite3_bind_int64(
            statement,
            2 as ::core::ffi::c_int,
            expected_revision as sqlite3_int64,
        );
        sqlite_result = sqlite3_step(statement);
    }
    let mut changed: ::core::ffi::c_int = sqlite3_changes(db);
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    if sqlite_result == SQLITE_CONSTRAINT {
        return db_storage_target_result_t::DB_STORAGE_TARGET_IN_USE;
    }
    if sqlite_result != SQLITE_DONE {
        return db_storage_target_result_t::DB_STORAGE_TARGET_ERROR;
    }
    if changed == 1 as ::core::ffi::c_int {
        return db_storage_target_result_t::DB_STORAGE_TARGET_OK;
    }
    let mut current: storage_target_t = storage_target_t {
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
    result = db_storage_target_get(uuid, &raw mut current);
    if result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        return result;
    }
    if current.revision != expected_revision {
        return db_storage_target_result_t::DB_STORAGE_TARGET_STALE;
    }
    return db_storage_target_result_t(
        if current.is_default as ::core::ffi::c_int != 0
            || current.recording_count > 0 as uint64_t
            || current.replica_count > 0 as uint64_t
        {
            db_storage_target_result_t::DB_STORAGE_TARGET_IN_USE.0
        } else {
            db_storage_target_result_t::DB_STORAGE_TARGET_ERROR.0
        },
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_bootstrap_default(
    mut legacy_root: *const ::core::ffi::c_char,
    mut uuid: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if legacy_root.is_null() || uuid.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut candidate: storage_target_t = storage_target_t {
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
        &raw mut candidate as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_target_t>(),
    );
    safe_strcpy(
        &raw mut candidate.name as *mut ::core::ffi::c_char,
        b"Default storage\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut candidate.target_type as *mut ::core::ffi::c_char,
        b"filesystem\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut candidate.root_path as *mut ::core::ffi::c_char,
        legacy_root,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    candidate.enabled = r#true != 0;
    candidate.is_default = r#true != 0;
    safe_strcpy(
        &raw mut candidate.storage_class as *mut ::core::ffi::c_char,
        b"hot\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    candidate.high_watermark_pct = 90.0f64;
    candidate.low_watermark_pct = 80.0f64;
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
    if db_storage_target_validate(
        &raw mut candidate,
        &raw mut error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    )
    .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot bootstrap default storage target: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut error as *mut ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT uuid,root_path FROM storage_targets WHERE is_default=1 LIMIT 1;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut exists: bool = r#false != 0;
    let mut selected_root: [::core::ffi::c_char; 512] = [
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
    if result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW {
        let mut existing_uuid: *const ::core::ffi::c_char =
            sqlite3_column_text(statement, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut existing_root: *const ::core::ffi::c_char =
            sqlite3_column_text(statement, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            uuid,
            if !existing_uuid.is_null() {
                existing_uuid
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            LIGHTNVR_UUID_STRING_SIZE as size_t,
            0 as size_t,
        );
        safe_strcpy(
            &raw mut selected_root as *mut ::core::ffi::c_char,
            if !existing_root.is_null() {
                existing_root
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        exists = r#true != 0;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if exists as ::core::ffi::c_int != 0
        && strcmp(
            &raw mut selected_root as *mut ::core::ffi::c_char,
            &raw mut candidate.root_path as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        let mut previous_uuid: [::core::ffi::c_char; 37] = [0; 37];
        safe_strcpy(
            &raw mut previous_uuid as *mut ::core::ffi::c_char,
            uuid,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        let mut next_uuid: [::core::ffi::c_char; 37] = [
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
        statement = ::core::ptr::null_mut::<sqlite3_stmt>();
        result = sqlite3_prepare_v2(
            db,
            b"SELECT uuid FROM storage_targets WHERE root_path=? LIMIT 1;\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_text(
                statement,
                1 as ::core::ffi::c_int,
                &raw mut candidate.root_path as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            if sqlite3_step(statement) == SQLITE_ROW {
                let mut matching_uuid: *const ::core::ffi::c_char =
                    sqlite3_column_text(statement, 0 as ::core::ffi::c_int)
                        as *const ::core::ffi::c_char;
                safe_strcpy(
                    &raw mut next_uuid as *mut ::core::ffi::c_char,
                    if !matching_uuid.is_null() {
                        matching_uuid
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                    0 as size_t,
                );
            }
        }
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
        if next_uuid[0usize] == 0
            && lightnvr_uuid_generate_v4(&raw mut next_uuid as *mut ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
        {
            pthread_mutex_unlock(mutex);
            return -1 as ::core::ffi::c_int;
        }
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
        let mut switched: bool = r#true != 0;
        statement = ::core::ptr::null_mut::<sqlite3_stmt>();
        result = sqlite3_prepare_v2(
            db,
            b"UPDATE storage_targets SET is_default=0,revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_text(
                statement,
                1 as ::core::ffi::c_int,
                &raw mut previous_uuid as *mut ::core::ffi::c_char,
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
        switched = result == SQLITE_DONE;
        if switched as ::core::ffi::c_int != 0 && next_uuid[0usize] as ::core::ffi::c_int != 0 {
            statement = ::core::ptr::null_mut::<sqlite3_stmt>();
            result = sqlite3_prepare_v2(
                db,
                b"SELECT count(*) FROM storage_targets WHERE uuid=?;\0".as_ptr()
                    as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                &raw mut statement,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            let mut target_exists: bool = r#false != 0;
            if result == SQLITE_OK {
                sqlite3_bind_text(
                    statement,
                    1 as ::core::ffi::c_int,
                    &raw mut next_uuid as *mut ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                target_exists = sqlite3_step(statement) == SQLITE_ROW
                    && sqlite3_column_int(statement, 0 as ::core::ffi::c_int)
                        == 1 as ::core::ffi::c_int;
            }
            if !statement.is_null() {
                sqlite3_finalize(statement);
            }
            if target_exists {
                statement = ::core::ptr::null_mut::<sqlite3_stmt>();
                result = sqlite3_prepare_v2(
                    db,
                    b"UPDATE storage_targets SET is_default=1,enabled=1,revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=?;\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                    &raw mut statement,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                if result == SQLITE_OK {
                    sqlite3_bind_text(
                        statement,
                        1 as ::core::ffi::c_int,
                        &raw mut next_uuid as *mut ::core::ffi::c_char,
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
                switched = result == SQLITE_DONE;
            } else {
                let mut generated_name: [::core::ffi::c_char; 128] = [0; 128];
                snprintf(
                    &raw mut generated_name as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                    b"Default storage %.8s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut next_uuid as *mut ::core::ffi::c_char,
                );
                let mut insert_sql: *const ::core::ffi::c_char = b"INSERT INTO storage_targets(uuid,name,target_type,root_path,enabled,is_default,storage_class,reserve_bytes,high_watermark_pct,low_watermark_pct,health_status) VALUES(?,?,'filesystem',?,1,1,'hot',0,90,80,'unknown');\0"
                    .as_ptr() as *const ::core::ffi::c_char;
                statement = ::core::ptr::null_mut::<sqlite3_stmt>();
                result = sqlite3_prepare_v2(
                    db,
                    insert_sql,
                    -1 as ::core::ffi::c_int,
                    &raw mut statement,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                if result == SQLITE_OK {
                    sqlite3_bind_text(
                        statement,
                        1 as ::core::ffi::c_int,
                        &raw mut next_uuid as *mut ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                    sqlite3_bind_text(
                        statement,
                        2 as ::core::ffi::c_int,
                        &raw mut generated_name as *mut ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                    sqlite3_bind_text(
                        statement,
                        3 as ::core::ffi::c_int,
                        &raw mut candidate.root_path as *mut ::core::ffi::c_char,
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
                switched = result == SQLITE_DONE;
            }
        }
        if switched as ::core::ffi::c_int != 0
            && sqlite3_exec(
                db,
                b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ) == SQLITE_OK
        {
            safe_strcpy(
                uuid,
                &raw mut next_uuid as *mut ::core::ffi::c_char,
                LIGHTNVR_UUID_STRING_SIZE as size_t,
                0 as size_t,
            );
            safe_strcpy(
                &raw mut selected_root as *mut ::core::ffi::c_char,
                &raw mut candidate.root_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Storage default changed from target %s to %s for root %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut previous_uuid as *mut ::core::ffi::c_char,
                uuid,
                &raw mut selected_root as *mut ::core::ffi::c_char,
            );
        } else {
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to switch the default storage target: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            pthread_mutex_unlock(mutex);
            return -1 as ::core::ffi::c_int;
        }
    }
    if !exists {
        if lightnvr_uuid_generate_v4(uuid) != 0 as ::core::ffi::c_int {
            pthread_mutex_unlock(mutex);
            return -1 as ::core::ffi::c_int;
        }
        safe_strcpy(
            &raw mut selected_root as *mut ::core::ffi::c_char,
            &raw mut candidate.root_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        let mut insert_sql_0: *const ::core::ffi::c_char = b"INSERT INTO storage_targets(uuid,name,target_type,root_path,enabled,is_default,storage_class,reserve_bytes,high_watermark_pct,low_watermark_pct,health_status) VALUES(?,?, 'filesystem',?,1,1,'hot',0,90,80,'unknown');\0"
            .as_ptr() as *const ::core::ffi::c_char;
        statement = ::core::ptr::null_mut::<sqlite3_stmt>();
        result = sqlite3_prepare_v2(
            db,
            insert_sql_0,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
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
            sqlite3_bind_text(
                statement,
                2 as ::core::ffi::c_int,
                &raw mut candidate.name as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                statement,
                3 as ::core::ffi::c_int,
                &raw mut candidate.root_path as *mut ::core::ffi::c_char,
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
        if result != SQLITE_DONE {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to create default storage target: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
            pthread_mutex_unlock(mutex);
            return -1 as ::core::ffi::c_int;
        }
    }
    let mut backfill_sql: *const ::core::ffi::c_char = b"UPDATE recordings SET storage_target_uuid=?,object_key=substr(file_path,length(?)+2),placement_reason='legacy-default' WHERE storage_target_uuid IS NULL AND substr(file_path,1,length(?)+1)=?||'/';\0"
        .as_ptr() as *const ::core::ffi::c_char;
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    result = sqlite3_prepare_v2(
        db,
        backfill_sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
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
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            &raw mut selected_root as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            &raw mut selected_root as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            4 as ::core::ffi::c_int,
            &raw mut selected_root as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    let mut attached: ::core::ffi::c_int = if result == SQLITE_DONE {
        sqlite3_changes(db)
    } else {
        -1 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    if result != SQLITE_DONE {
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Storage target bootstrap: default=%s root=%s attached=%d\0".as_ptr()
            as *const ::core::ffi::c_char,
        uuid,
        &raw mut selected_root as *mut ::core::ffi::c_char,
        attached,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_classify_path(
    mut absolute_path: *const ::core::ffi::c_char,
    mut target_uuid: *mut ::core::ffi::c_char,
    mut object_key: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if absolute_path.is_null()
        || *absolute_path.offset(0isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        || target_uuid.is_null()
        || object_key.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    *target_uuid.offset(0isize) = '\0' as ::core::ffi::c_char;
    *object_key.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT uuid,root_path FROM storage_targets WHERE target_type='filesystem' ORDER BY length(root_path) DESC,uuid;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut outcome: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    while result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW {
        let mut uuid: *const ::core::ffi::c_char =
            sqlite3_column_text(statement, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut root: *const ::core::ffi::c_char =
            sqlite3_column_text(statement, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if uuid.is_null() || root.is_null() {
            continue;
        }
        let mut root_length: size_t = strlen(root);
        if strncmp(absolute_path, root, root_length) != 0 as ::core::ffi::c_int
            || *absolute_path.offset(root_length as isize) as ::core::ffi::c_int
                != '/' as ::core::ffi::c_int
        {
            continue;
        }
        let mut relative: *const ::core::ffi::c_char = absolute_path
            .offset(root_length as isize)
            .offset(1 as ::core::ffi::c_int as isize);
        if !valid_object_key(relative) {
            continue;
        }
        safe_strcpy(
            target_uuid,
            uuid,
            LIGHTNVR_UUID_STRING_SIZE as size_t,
            0 as size_t,
        );
        safe_strcpy(
            object_key,
            relative,
            STORAGE_TARGET_OBJECT_KEY_MAX as size_t,
            0 as size_t,
        );
        outcome = 0 as ::core::ffi::c_int;
        break;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_storage_target_resolve_path(
    mut target_uuid: *const ::core::ffi::c_char,
    mut object_key: *const ::core::ffi::c_char,
    mut absolute_path: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !lightnvr_uuid_is_valid(target_uuid)
        || !valid_object_key(object_key)
        || absolute_path.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT root_path FROM storage_targets WHERE uuid=? AND target_type='filesystem' LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut outcome: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            target_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if sqlite3_step(statement) == SQLITE_ROW {
            let mut root: *const ::core::ffi::c_char =
                sqlite3_column_text(statement, 0 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char;
            let mut written: ::core::ffi::c_int = if !root.is_null() {
                snprintf(
                    absolute_path,
                    MAX_PATH_LENGTH as size_t,
                    b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
                    root,
                    object_key,
                )
            } else {
                -1 as ::core::ffi::c_int
            };
            if written > 0 as ::core::ffi::c_int && written < MAX_PATH_LENGTH {
                outcome = 0 as ::core::ffi::c_int;
            }
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return outcome;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
