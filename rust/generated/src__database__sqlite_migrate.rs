use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type sqlite3;
    pub type sqlite3_stmt;
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
    unsafe fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
    unsafe fn rewinddir(__dirp: *mut DIR);
    unsafe fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
    unsafe fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    unsafe fn sqlite3_free(_: *mut ::core::ffi::c_void);
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
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
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
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
    unsafe fn strndup(
        __string: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
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
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn realpath(
        __name: *const ::core::ffi::c_char,
        __resolved: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const DT_UNKNOWN: Self = Self(0);
    pub const DT_FIFO: Self = Self(1);
    pub const DT_CHR: Self = Self(2);
    pub const DT_DIR: Self = Self(4);
    pub const DT_BLK: Self = Self(6);
    pub const DT_REG: Self = Self(8);
    pub const DT_LNK: Self = Self(10);
    pub const DT_SOCK: Self = Self(12);
    pub const DT_WHT: Self = Self(14);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct migrate_status_t(pub ::core::ffi::c_uint);
impl migrate_status_t {
    pub const MIGRATE_STATUS_PENDING: Self = Self(0);
    pub const MIGRATE_STATUS_APPLIED: Self = Self(1);
    pub const MIGRATE_STATUS_FAILED: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct migration_t {
    pub version: [::core::ffi::c_char; 32],
    pub description: [::core::ffi::c_char; 256],
    pub filepath: [::core::ffi::c_char; 512],
    pub sql_up: *const ::core::ffi::c_char,
    pub sql_down: *const ::core::ffi::c_char,
    pub status: migrate_status_t,
    pub is_embedded: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite_migrate {
    pub db: *mut sqlite3,
    pub config: migrate_config_t,
    pub migrations: *mut migration_t,
    pub migration_count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct migrate_config_t {
    pub migrations_dir: *const ::core::ffi::c_char,
    pub migrations_table: *const ::core::ffi::c_char,
    pub embedded_migrations: *const migration_t,
    pub embedded_count: size_t,
    pub callback: migrate_callback_t,
    pub callback_data: *mut ::core::ffi::c_void,
    pub dry_run: bool,
    pub verbose: bool,
}
pub type migrate_callback_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        migrate_status_t,
        *mut ::core::ffi::c_void,
    ) -> (),
>;
pub type sqlite_migrate_t = sqlite_migrate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct migrate_stats_t {
    pub total: ::core::ffi::c_int,
    pub applied: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub failed: ::core::ffi::c_int,
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
    pub kw: *const ::core::ffi::c_char,
    pub len: size_t,
}
static mut allowed: [C2Rust_Unnamed_1; 18] = [C2Rust_Unnamed_1 {
    kw: ::core::ptr::null::<::core::ffi::c_char>(),
    len: 0,
}; 18];
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_IWRITE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const S_IWUSR: ::core::ffi::c_int = __S_IWRITE;
pub const S_IWGRP: ::core::ffi::c_int = S_IWUSR >> 3 as ::core::ffi::c_int;
pub const S_IWOTH: ::core::ffi::c_int = S_IWGRP >> 3 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const MIGRATE_VERSION_LEN: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MIGRATE_DESC_LEN: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MIGRATE_SQL_MAX_LEN: ::core::ffi::c_long =
    1024 as ::core::ffi::c_long * 1024 as ::core::ffi::c_long;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[inline]
unsafe extern "C" fn rtrim_pos(
    mut input: *const ::core::ffi::c_char,
    mut input_size: size_t,
) -> *const ::core::ffi::c_char {
    if input.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if input_size > 0 as size_t {
        end = input
            .offset(strnlen(input, input_size) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
    } else {
        end = input
            .offset(strlen(input) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
    }
    while end > input
        && *(*__ctype_b_loc()).offset(*end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed_0::_ISgraph.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
    {
        end = end.offset(-1);
    }
    return end.offset(1 as ::core::ffi::c_int as isize);
}
pub const DEFAULT_MIGRATIONS_TABLE: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"schema_migrations\0")
};
unsafe extern "C" fn create_migrations_table(mut ctx: *mut sqlite_migrate_t) -> ::core::ffi::c_int {
    let mut table_name: *const ::core::ffi::c_char = if !(*ctx).config.migrations_table.is_null() {
        (*ctx).config.migrations_table
    } else {
        DEFAULT_MIGRATIONS_TABLE.as_ptr()
    };
    let mut sql: *mut ::core::ffi::c_char = sqlite3_mprintf(
        b"CREATE TABLE IF NOT EXISTS \"%w\" (  version TEXT PRIMARY KEY,  applied_at INTEGER NOT NULL DEFAULT (strftime('%%s', 'now')));\0"
            .as_ptr() as *const ::core::ffi::c_char,
        table_name,
    );
    if sql.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut err_msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rc: ::core::ffi::c_int = sqlite3_exec((*ctx).db, sql, None, NULL, &raw mut err_msg);
    sqlite3_free(sql as *mut ::core::ffi::c_void);
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create migrations table: %s\0".as_ptr() as *const ::core::ffi::c_char,
            err_msg,
        );
        sqlite3_free(err_msg as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn is_migration_applied(
    mut ctx: *mut sqlite_migrate_t,
    mut version: *const ::core::ffi::c_char,
) -> bool {
    let mut table_name: *const ::core::ffi::c_char = if !(*ctx).config.migrations_table.is_null() {
        (*ctx).config.migrations_table
    } else {
        DEFAULT_MIGRATIONS_TABLE.as_ptr()
    };
    let mut sql: *mut ::core::ffi::c_char = sqlite3_mprintf(
        b"SELECT 1 FROM \"%w\" WHERE version = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        table_name,
    );
    if sql.is_null() {
        return r#false != 0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        (*ctx).db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    sqlite3_free(sql as *mut ::core::ffi::c_void);
    if rc != SQLITE_OK {
        return r#false != 0;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        version,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    let mut applied: bool = sqlite3_step(stmt) == SQLITE_ROW;
    sqlite3_finalize(stmt);
    return applied;
}
unsafe extern "C" fn record_migration_applied(
    mut ctx: *mut sqlite_migrate_t,
    mut version: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut table_name: *const ::core::ffi::c_char = if !(*ctx).config.migrations_table.is_null() {
        (*ctx).config.migrations_table
    } else {
        DEFAULT_MIGRATIONS_TABLE.as_ptr()
    };
    let mut sql: *mut ::core::ffi::c_char = sqlite3_mprintf(
        b"INSERT INTO \"%w\" (version) VALUES (?);\0".as_ptr() as *const ::core::ffi::c_char,
        table_name,
    );
    if sql.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        (*ctx).db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    sqlite3_free(sql as *mut ::core::ffi::c_void);
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare migration record: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg((*ctx).db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        version,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to record migration: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg((*ctx).db),
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn remove_migration_record(
    mut ctx: *mut sqlite_migrate_t,
    mut version: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut table_name: *const ::core::ffi::c_char = if !(*ctx).config.migrations_table.is_null() {
        (*ctx).config.migrations_table
    } else {
        DEFAULT_MIGRATIONS_TABLE.as_ptr()
    };
    let mut sql: *mut ::core::ffi::c_char = sqlite3_mprintf(
        b"DELETE FROM \"%w\" WHERE version = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        table_name,
    );
    if sql.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        (*ctx).db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    sqlite3_free(sql as *mut ::core::ffi::c_void);
    if rc != SQLITE_OK {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        version,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn parse_migration_filename(
    mut filename: *const ::core::ffi::c_char,
    mut version: *mut ::core::ffi::c_char,
    mut description: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut len: size_t = strlen(filename);
    if len < 5 as size_t
        || strcmp(
            filename
                .offset(len as isize)
                .offset(-(4 as ::core::ffi::c_int as isize)),
            b".sql\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut underscore: *const ::core::ffi::c_char = strchr(filename, '_' as ::core::ffi::c_int);
    if underscore.is_null() || underscore == filename {
        return -1 as ::core::ffi::c_int;
    }
    let mut version_len: size_t = underscore.offset_from(filename) as size_t;
    if version_len >= MIGRATE_VERSION_LEN as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Migration file version too long in %s\0".as_ptr() as *const ::core::ffi::c_char,
            filename,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut i: size_t = 0 as size_t;
    while i < version_len {
        if *(*__ctype_b_loc()).offset(
            *filename.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
        ) as ::core::ffi::c_int
            & C2Rust_Unnamed_0::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Unexpected version character in %s\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
            );
            return -1 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    safe_strcpy(
        version,
        filename,
        MIGRATE_VERSION_LEN as size_t,
        version_len,
    );
    let mut desc_start: *const ::core::ffi::c_char =
        underscore.offset(1 as ::core::ffi::c_int as isize);
    let mut desc_len: size_t = len
        .wrapping_sub(4 as size_t)
        .wrapping_sub(desc_start.offset_from(filename) as size_t);
    safe_strcpy(
        description,
        desc_start,
        MIGRATE_DESC_LEN as size_t,
        desc_len,
    );
    let mut p: *mut ::core::ffi::c_char = description;
    while *p != 0 {
        if *p as ::core::ffi::c_int == '_' as ::core::ffi::c_int {
            *p = ' ' as ::core::ffi::c_char;
        }
        p = p.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn compare_migrations(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ma: *const migration_t = a as *const migration_t;
    let mut mb: *const migration_t = b as *const migration_t;
    return strcmp(
        &raw const (*ma).version as *const ::core::ffi::c_char,
        &raw const (*mb).version as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn validate_migration_file_security(
    mut filepath: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
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
    if lstat(filepath, &raw mut st) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cannot stat migration file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            filepath,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !(st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t) {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Migration file is not a regular file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            filepath,
        );
        return -1 as ::core::ffi::c_int;
    }
    if st.st_mode & S_IWOTH as __mode_t != 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Migration file is world-writable, refusing to execute: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            filepath,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn is_word_char(mut c: ::core::ffi::c_char) -> bool {
    return *(*__ctype_b_loc()).offset(c as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed_0::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        != 0
        || c as ::core::ffi::c_int == '_' as ::core::ffi::c_int
        || c as ::core::ffi::c_int == '$' as ::core::ffi::c_int
        || c as ::core::ffi::c_uchar as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int;
}
unsafe extern "C" fn word_matches(
    mut start: *const ::core::ffi::c_char,
    mut cursor: *const ::core::ffi::c_char,
    mut word: *const ::core::ffi::c_char,
    mut length: size_t,
) -> bool {
    if cursor > start && is_word_char(*cursor.offset(-1isize)) as ::core::ffi::c_int != 0 {
        return r#false != 0;
    }
    if strncasecmp(cursor, word, length) != 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    return !is_word_char(*cursor.offset(length as isize));
}
unsafe extern "C" fn statement_end(
    mut start: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut cursor: *const ::core::ffi::c_char = start;
    while *cursor as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc())
            .offset(*cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        cursor = cursor.offset(1);
    }
    let mut trigger: bool = word_matches(
        start,
        cursor,
        b"CREATE\0".as_ptr() as *const ::core::ffi::c_char,
        6 as size_t,
    );
    if trigger {
        let mut after: *const ::core::ffi::c_char = cursor.offset(6 as ::core::ffi::c_int as isize);
        while *after as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc())
                .offset(*after as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            after = after.offset(1);
        }
        trigger = word_matches(
            start,
            after,
            b"TRIGGER\0".as_ptr() as *const ::core::ffi::c_char,
            7 as size_t,
        );
    }
    let mut quote: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    let mut terminator: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    cursor = start;
    while *cursor != 0 {
        if quote != 0 {
            if *cursor as ::core::ffi::c_int == quote as ::core::ffi::c_int {
                if quote as ::core::ffi::c_int != ']' as ::core::ffi::c_int
                    && *cursor.offset(1isize) as ::core::ffi::c_int == quote as ::core::ffi::c_int
                {
                    cursor = cursor.offset(1);
                } else {
                    quote = 0 as ::core::ffi::c_char;
                }
            }
        } else if *cursor as ::core::ffi::c_int == '\'' as ::core::ffi::c_int
            || *cursor as ::core::ffi::c_int == '"' as ::core::ffi::c_int
            || *cursor as ::core::ffi::c_int == '`' as ::core::ffi::c_int
            || *cursor as ::core::ffi::c_int == '[' as ::core::ffi::c_int
        {
            terminator = 0 as ::core::ffi::c_uint;
            quote = (if *cursor as ::core::ffi::c_int == '[' as ::core::ffi::c_int {
                ']' as ::core::ffi::c_int
            } else {
                *cursor as ::core::ffi::c_int
            }) as ::core::ffi::c_char;
        } else if *cursor.offset(0isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
            && *cursor.offset(1isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
        {
            while *cursor as ::core::ffi::c_int != 0
                && *cursor as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
            {
                cursor = cursor.offset(1);
            }
            if *cursor == 0 {
                return cursor;
            }
        } else if *cursor.offset(0isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
            && *cursor.offset(1isize) as ::core::ffi::c_int == '*' as ::core::ffi::c_int
        {
            cursor = cursor.offset(2 as ::core::ffi::c_int as isize);
            while *cursor as ::core::ffi::c_int != 0
                && !(*cursor.offset(0isize) as ::core::ffi::c_int == '*' as ::core::ffi::c_int
                    && *cursor.offset(1isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int)
            {
                cursor = cursor.offset(1);
            }
            if *cursor == 0 {
                return cursor;
            }
            cursor = cursor.offset(1);
        } else if !trigger {
            if *cursor as ::core::ffi::c_int == ';' as ::core::ffi::c_int {
                return cursor;
            }
        } else if *(*__ctype_b_loc())
            .offset(*cursor as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        {
            if *cursor as ::core::ffi::c_int == ';' as ::core::ffi::c_int {
                if terminator == 2 as ::core::ffi::c_uint {
                    return cursor;
                }
                terminator = 1 as ::core::ffi::c_uint;
            } else if terminator == 1 as ::core::ffi::c_uint
                && word_matches(
                    start,
                    cursor,
                    b"END\0".as_ptr() as *const ::core::ffi::c_char,
                    3 as size_t,
                ) as ::core::ffi::c_int
                    != 0
            {
                terminator = 2 as ::core::ffi::c_uint;
                cursor = cursor.offset(2 as ::core::ffi::c_int as isize);
            } else {
                terminator = 0 as ::core::ffi::c_uint;
            }
        }
        cursor = cursor.offset(1);
    }
    return cursor;
}
unsafe extern "C" fn validate_migration_sql(
    mut sql: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if sql.is_null() || *sql as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut p: *const ::core::ffi::c_char = sql;
    while *p != 0 {
        while *p as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            break;
        }
        if *p.offset(0isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
            && *p.offset(1isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
        {
            while *p as ::core::ffi::c_int != 0
                && *p as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
            {
                p = p.offset(1);
            }
        } else {
            let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while !allowed[i as usize].kw.is_null() {
                if strncasecmp(p, allowed[i as usize].kw, allowed[i as usize].len)
                    == 0 as ::core::ffi::c_int
                {
                    let mut next: ::core::ffi::c_char = *p.offset(allowed[i as usize].len as isize);
                    if next as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                        || *(*__ctype_b_loc())
                            .offset(next as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int
                                as ::core::ffi::c_ushort
                                as ::core::ffi::c_int
                            != 0
                        || next as ::core::ffi::c_int == ';' as ::core::ffi::c_int
                        || next as ::core::ffi::c_int == '(' as ::core::ffi::c_int
                    {
                        found = 1 as ::core::ffi::c_int;
                        break;
                    }
                }
                i += 1;
            }
            if found == 0 {
                let mut snippet: [::core::ffi::c_char; 61] = [
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
                ];
                let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut q: *const ::core::ffi::c_char = p;
                while *q as ::core::ffi::c_int != 0
                    && *q as ::core::ffi::c_int != ';' as ::core::ffi::c_int
                    && *q as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
                    && n < 60 as ::core::ffi::c_int
                {
                    let c2rust_fresh4 = q;
                    q = q.offset(1);
                    let c2rust_fresh5 = n;
                    n += 1;
                    snippet[c2rust_fresh5 as usize] = *c2rust_fresh4;
                }
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Migration SQL rejected disallowed statement: %.60s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut snippet as *mut ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            p = statement_end(p);
            if *p as ::core::ffi::c_int == ';' as ::core::ffi::c_int {
                p = p.offset(1);
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn read_sql_file(
    mut filepath: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut fp: *mut FILE = fopen(filepath, b"r\0".as_ptr() as *const ::core::ffi::c_char);
    if fp.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to open migration file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            filepath,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    fseek(fp, 0 as ::core::ffi::c_long, SEEK_END);
    let mut size: ::core::ffi::c_long = ftell(fp);
    fseek(fp, 0 as ::core::ffi::c_long, SEEK_SET);
    if size <= 0 as ::core::ffi::c_long || size > MIGRATE_SQL_MAX_LEN {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Migration file too large or empty: %s\0".as_ptr() as *const ::core::ffi::c_char,
            filepath,
        );
        fclose(fp);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut content: *mut ::core::ffi::c_char =
        malloc((size + 1 as ::core::ffi::c_long) as size_t) as *mut ::core::ffi::c_char;
    if content.is_null() {
        fclose(fp);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut read: size_t = fread(
        content as *mut ::core::ffi::c_void,
        1 as size_t,
        size as size_t,
        fp,
    ) as size_t;
    fclose(fp);
    if read > size as size_t {
        read = size as size_t;
    }
    *content.offset(read as isize) = '\0' as ::core::ffi::c_char;
    return content;
}
unsafe extern "C" fn extract_sql_section(
    mut content: *const ::core::ffi::c_char,
    mut marker: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut search: [::core::ffi::c_char; 64] = [0; 64];
    snprintf(
        &raw mut search as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"-- migrate:%s\0".as_ptr() as *const ::core::ffi::c_char,
        marker,
    );
    let mut start: *const ::core::ffi::c_char =
        strstr(content, &raw mut search as *mut ::core::ffi::c_char);
    if start.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    start = strchr(start, '\n' as ::core::ffi::c_int);
    if start.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    start = start.offset(1);
    let mut end: *const ::core::ffi::c_char = strstr(
        start,
        b"-- migrate:\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if end.is_null() {
        end = content.offset(strlen(content) as isize);
    }
    end = rtrim_pos(start, end.offset_from(start) as size_t);
    let mut len: size_t = end.offset_from(start) as size_t;
    let mut sql: *mut ::core::ffi::c_char = strndup(start, len);
    if sql.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return sql;
}
unsafe extern "C" fn scan_migrations_dir(mut ctx: *mut sqlite_migrate_t) -> ::core::ffi::c_int {
    if (*ctx).config.migrations_dir.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut dir: *mut DIR = opendir((*ctx).config.migrations_dir);
    if dir.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Migrations directory not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            (*ctx).config.migrations_dir,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut entry: *const dirent = ::core::ptr::null::<dirent>();
    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        if (*entry).d_type as ::core::ffi::c_int == C2Rust_Unnamed::DT_REG.0 as ::core::ffi::c_int
            || (*entry).d_type as ::core::ffi::c_int
                == C2Rust_Unnamed::DT_UNKNOWN.0 as ::core::ffi::c_int
        {
            let mut len: size_t = strlen(&raw const (*entry).d_name as *const ::core::ffi::c_char);
            if len > 4 as size_t
                && strcmp(
                    (&raw const (*entry).d_name as *const ::core::ffi::c_char)
                        .offset(len as isize)
                        .offset(-(4 as ::core::ffi::c_int as isize)),
                    b".sql\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                count += 1;
            }
        }
    }
    if count == 0 as ::core::ffi::c_int {
        closedir(dir);
        return 0 as ::core::ffi::c_int;
    }
    (*ctx).migrations =
        calloc(count as size_t, ::core::mem::size_of::<migration_t>()) as *mut migration_t;
    if (*ctx).migrations.is_null() {
        closedir(dir);
        return -1 as ::core::ffi::c_int;
    }
    rewinddir(dir);
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        entry = readdir(dir);
        if !(!entry.is_null() && idx < count) {
            break;
        }
        if (*entry).d_type as ::core::ffi::c_int != C2Rust_Unnamed::DT_REG.0 as ::core::ffi::c_int
            && (*entry).d_type as ::core::ffi::c_int
                != C2Rust_Unnamed::DT_UNKNOWN.0 as ::core::ffi::c_int
        {
            continue;
        }
        let mut m: *mut migration_t = (*ctx).migrations.offset(idx as isize);
        if parse_migration_filename(
            &raw const (*entry).d_name as *const ::core::ffi::c_char,
            &raw mut (*m).version as *mut ::core::ffi::c_char,
            &raw mut (*m).description as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            continue;
        }
        snprintf(
            &raw mut (*m).filepath as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            (*ctx).config.migrations_dir,
            &raw const (*entry).d_name as *const ::core::ffi::c_char,
        );
        (*m).is_embedded = r#false != 0;
        (*m).status = migrate_status_t(
            (if is_migration_applied(ctx, &raw mut (*m).version as *mut ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0
            {
                migrate_status_t::MIGRATE_STATUS_APPLIED.0 as ::core::ffi::c_int
            } else {
                migrate_status_t::MIGRATE_STATUS_PENDING.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
        idx += 1;
    }
    closedir(dir);
    (*ctx).migration_count = idx;
    qsort(
        (*ctx).migrations as *mut ::core::ffi::c_void,
        (*ctx).migration_count as size_t,
        ::core::mem::size_of::<migration_t>(),
        Some(
            compare_migrations
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    return (*ctx).migration_count;
}
unsafe extern "C" fn add_embedded_migrations(mut ctx: *mut sqlite_migrate_t) -> ::core::ffi::c_int {
    if (*ctx).config.embedded_migrations.is_null() || (*ctx).config.embedded_count == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    let mut to_add: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    while i < (*ctx).config.embedded_count {
        let mut src: *const migration_t = (*ctx).config.embedded_migrations.offset(i as isize);
        let mut already_exists: bool = r#false != 0;
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < (*ctx).migration_count {
            if strcmp(
                &raw mut (*(*ctx).migrations.offset(j as isize)).version
                    as *mut ::core::ffi::c_char,
                &raw const (*src).version as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                already_exists = r#true != 0;
                break;
            } else {
                j += 1;
            }
        }
        if !already_exists {
            to_add += 1;
        }
        i = i.wrapping_add(1);
    }
    if to_add == 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"All embedded migrations already covered by filesystem migrations\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut existing: ::core::ffi::c_int = (*ctx).migration_count;
    let mut new_count: ::core::ffi::c_int = existing + to_add;
    let mut new_array: *mut migration_t = realloc(
        (*ctx).migrations as *mut ::core::ffi::c_void,
        (new_count as size_t).wrapping_mul(::core::mem::size_of::<migration_t>()),
    ) as *mut migration_t;
    if new_array.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    (*ctx).migrations = new_array;
    let mut added: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i_0: size_t = 0 as size_t;
    while i_0 < (*ctx).config.embedded_count {
        let mut src_0: *const migration_t = (*ctx).config.embedded_migrations.offset(i_0 as isize);
        let mut already_exists_0: bool = r#false != 0;
        let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j_0 < existing {
            if strcmp(
                &raw mut (*(*ctx).migrations.offset(j_0 as isize)).version
                    as *mut ::core::ffi::c_char,
                &raw const (*src_0).version as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                already_exists_0 = r#true != 0;
                break;
            } else {
                j_0 += 1;
            }
        }
        if !already_exists_0 {
            let mut dst: *mut migration_t = (*ctx).migrations.offset((existing + added) as isize);
            memcpy(
                dst as *mut ::core::ffi::c_void,
                src_0 as *const ::core::ffi::c_void,
                ::core::mem::size_of::<migration_t>(),
            );
            (*dst).is_embedded = r#true != 0;
            (*dst).status = migrate_status_t(
                (if is_migration_applied(ctx, &raw mut (*dst).version as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_int
                    != 0
                {
                    migrate_status_t::MIGRATE_STATUS_APPLIED.0 as ::core::ffi::c_int
                } else {
                    migrate_status_t::MIGRATE_STATUS_PENDING.0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint,
            );
            added += 1;
        }
        i_0 = i_0.wrapping_add(1);
    }
    (*ctx).migration_count = existing + added;
    qsort(
        (*ctx).migrations as *mut ::core::ffi::c_void,
        (*ctx).migration_count as size_t,
        ::core::mem::size_of::<migration_t>(),
        Some(
            compare_migrations
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Added %d embedded migrations (%d skipped as duplicates)\0".as_ptr()
            as *const ::core::ffi::c_char,
        added,
        (*ctx).config.embedded_count as ::core::ffi::c_int - added,
    );
    return added;
}
unsafe extern "C" fn column_exists(
    mut db: *mut sqlite3,
    mut table: *const ::core::ffi::c_char,
    mut column: *const ::core::ffi::c_char,
) -> bool {
    let mut sql: [::core::ffi::c_char; 256] = [0; 256];
    snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"PRAGMA table_info(%s);\0".as_ptr() as *const ::core::ffi::c_char,
        table,
    );
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return r#false != 0;
    }
    let mut exists: bool = r#false != 0;
    while sqlite3_step(stmt) == SQLITE_ROW {
        let mut col_name: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !(!col_name.is_null() && strcasecmp(col_name, column) == 0 as ::core::ffi::c_int) {
            continue;
        }
        exists = r#true != 0;
        break;
    }
    sqlite3_finalize(stmt);
    return exists;
}
unsafe extern "C" fn execute_single_statement(
    mut db: *mut sqlite3,
    mut sql: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    while *sql as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*sql as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        sql = sql.offset(1);
    }
    if *sql as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if strncasecmp(
        sql,
        b"ALTER TABLE\0".as_ptr() as *const ::core::ffi::c_char,
        11 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        let mut table: [::core::ffi::c_char; 128] = [0; 128];
        let mut p: *const ::core::ffi::c_char = sql.offset(11 as ::core::ffi::c_int as isize);
        while *p as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            p = p.offset(1);
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while *p as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                == 0
            && i < 127 as ::core::ffi::c_int
        {
            let c2rust_fresh0 = p;
            p = p.offset(1);
            let c2rust_fresh1 = i;
            i += 1;
            table[c2rust_fresh1 as usize] = *c2rust_fresh0;
        }
        table[i as usize] = '\0' as ::core::ffi::c_char;
        while *p as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            p = p.offset(1);
        }
        if strncasecmp(
            p,
            b"ADD\0".as_ptr() as *const ::core::ffi::c_char,
            3 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            p = p.offset(3 as ::core::ffi::c_int as isize);
            while *p as ::core::ffi::c_int != 0
                && *(*__ctype_b_loc())
                    .offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int
                    != 0
            {
                p = p.offset(1);
            }
            if strncasecmp(
                p,
                b"COLUMN\0".as_ptr() as *const ::core::ffi::c_char,
                6 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                p = p.offset(6 as ::core::ffi::c_int as isize);
                while *p as ::core::ffi::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int
                            as ::core::ffi::c_ushort as ::core::ffi::c_int
                        != 0
                {
                    p = p.offset(1);
                }
                let mut column: [::core::ffi::c_char; 128] = [0; 128];
                i = 0 as ::core::ffi::c_int;
                while *p as ::core::ffi::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int
                            as ::core::ffi::c_ushort as ::core::ffi::c_int
                        == 0
                    && i < 127 as ::core::ffi::c_int
                {
                    let c2rust_fresh2 = p;
                    p = p.offset(1);
                    let c2rust_fresh3 = i;
                    i += 1;
                    column[c2rust_fresh3 as usize] = *c2rust_fresh2;
                }
                column[i as usize] = '\0' as ::core::ffi::c_char;
                if table[0usize] as ::core::ffi::c_int != 0
                    && column[0usize] as ::core::ffi::c_int != 0
                    && column_exists(
                        db,
                        &raw mut table as *mut ::core::ffi::c_char,
                        &raw mut column as *mut ::core::ffi::c_char,
                    ) as ::core::ffi::c_int
                        != 0
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Column %s.%s already exists, skipping\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut table as *mut ::core::ffi::c_char,
                        &raw mut column as *mut ::core::ffi::c_char,
                    );
                    return 1 as ::core::ffi::c_int;
                }
            }
        }
    }
    let mut err_msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rc: ::core::ffi::c_int = sqlite3_exec(db, sql, None, NULL, &raw mut err_msg);
    if rc != SQLITE_OK {
        if !err_msg.is_null() {
            if !strstr(
                err_msg,
                b"duplicate column name\0".as_ptr() as *const ::core::ffi::c_char,
            )
            .is_null()
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Column already exists (idempotent): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    err_msg,
                );
                sqlite3_free(err_msg as *mut ::core::ffi::c_void);
                return 1 as ::core::ffi::c_int;
            }
            if !strstr(err_msg, b"table\0".as_ptr() as *const ::core::ffi::c_char).is_null()
                && !strstr(
                    err_msg,
                    b"already exists\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .is_null()
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Table already exists (idempotent): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    err_msg,
                );
                sqlite3_free(err_msg as *mut ::core::ffi::c_void);
                return 1 as ::core::ffi::c_int;
            }
            if !strstr(err_msg, b"index\0".as_ptr() as *const ::core::ffi::c_char).is_null()
                && !strstr(
                    err_msg,
                    b"already exists\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .is_null()
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Index already exists (idempotent): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    err_msg,
                );
                sqlite3_free(err_msg as *mut ::core::ffi::c_void);
                return 1 as ::core::ffi::c_int;
            }
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"SQL execution failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            err_msg,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"SQL was: %.200s\0".as_ptr() as *const ::core::ffi::c_char,
            sql,
        );
        sqlite3_free(err_msg as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn execute_sql(
    mut db: *mut sqlite3,
    mut sql: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if strncasecmp(
        sql,
        b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char,
        5 as size_t,
    ) == 0 as ::core::ffi::c_int
        || strncasecmp(
            sql,
            b"COMMIT\0".as_ptr() as *const ::core::ffi::c_char,
            6 as size_t,
        ) == 0 as ::core::ffi::c_int
        || strncasecmp(
            sql,
            b"ROLLBACK\0".as_ptr() as *const ::core::ffi::c_char,
            8 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        let mut err_msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut rc: ::core::ffi::c_int = sqlite3_exec(db, sql, None, NULL, &raw mut err_msg);
        if rc != SQLITE_OK {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"SQL execution failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                err_msg,
            );
            sqlite3_free(err_msg as *mut ::core::ffi::c_void);
            return -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
    let mut start: *const ::core::ffi::c_char = sql;
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut stmt_copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut stmt_size: size_t = 0 as size_t;
    while *start != 0 {
        while *start as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc())
                .offset(*start as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed_0::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            start = start.offset(1);
        }
        if *start as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            break;
        }
        if *start.offset(0isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
            && *start.offset(1isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
        {
            while *start as ::core::ffi::c_int != 0
                && *start as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
            {
                start = start.offset(1);
            }
        } else {
            end = statement_end(start);
            if end == start {
                start = end.offset(1 as ::core::ffi::c_int as isize);
            } else {
                let mut len: size_t = end.offset_from(start) as size_t;
                if len.wrapping_add(2 as size_t) > stmt_size {
                    stmt_size = len.wrapping_add(256 as size_t);
                    let mut new_copy: *mut ::core::ffi::c_char =
                        realloc(stmt_copy as *mut ::core::ffi::c_void, stmt_size)
                            as *mut ::core::ffi::c_char;
                    if new_copy.is_null() {
                        free(stmt_copy as *mut ::core::ffi::c_void);
                        return -1 as ::core::ffi::c_int;
                    }
                    stmt_copy = new_copy;
                }
                if stmt_copy.is_null() {
                    return -1 as ::core::ffi::c_int;
                }
                memcpy(
                    stmt_copy as *mut ::core::ffi::c_void,
                    start as *const ::core::ffi::c_void,
                    len,
                );
                *stmt_copy.offset(len as isize) = ';' as ::core::ffi::c_char;
                *stmt_copy.offset(len.wrapping_add(1 as size_t) as isize) =
                    '\0' as ::core::ffi::c_char;
                let mut result: ::core::ffi::c_int = execute_single_statement(db, stmt_copy);
                if result < 0 as ::core::ffi::c_int {
                    free(stmt_copy as *mut ::core::ffi::c_void);
                    return -1 as ::core::ffi::c_int;
                }
                start = if *end as ::core::ffi::c_int != 0 {
                    end.offset(1 as ::core::ffi::c_int as isize)
                } else {
                    end
                };
            }
        }
    }
    free(stmt_copy as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn execute_validated_migration_sql(
    mut db: *mut sqlite3,
    mut sql: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if validate_migration_sql(sql) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Migration SQL failed allowlist validation\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return execute_sql(db, sql);
}
unsafe extern "C" fn apply_migration(
    mut ctx: *mut sqlite_migrate_t,
    mut m: *mut migration_t,
) -> ::core::ffi::c_int {
    let mut sql_up: *const ::core::ffi::c_char = (*m).sql_up;
    let mut allocated_sql: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*m).is_embedded {
        if !(*ctx).config.migrations_dir.is_null() {
            let mut resolved_file: [::core::ffi::c_char; 4096] = [0; 4096];
            let mut resolved_dir: [::core::ffi::c_char; 4096] = [0; 4096];
            if realpath(
                &raw mut (*m).filepath as *mut ::core::ffi::c_char,
                &raw mut resolved_file as *mut ::core::ffi::c_char,
            )
            .is_null()
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Cannot resolve migration file path: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*m).filepath as *mut ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            if realpath(
                (*ctx).config.migrations_dir,
                &raw mut resolved_dir as *mut ::core::ffi::c_char,
            )
            .is_null()
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Cannot resolve migrations directory: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*ctx).config.migrations_dir,
                );
                return -1 as ::core::ffi::c_int;
            }
            let mut dir_len: size_t = strlen(&raw mut resolved_dir as *mut ::core::ffi::c_char);
            if strncmp(
                &raw mut resolved_file as *mut ::core::ffi::c_char,
                &raw mut resolved_dir as *mut ::core::ffi::c_char,
                dir_len,
            ) != 0 as ::core::ffi::c_int
                || resolved_file[dir_len] as ::core::ffi::c_int != '/' as ::core::ffi::c_int
                    && resolved_file[dir_len] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Migration file is outside the configured directory: %s (resolved: %s)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*m).filepath as *mut ::core::ffi::c_char,
                    &raw mut resolved_file as *mut ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        if validate_migration_file_security(&raw mut (*m).filepath as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut content: *mut ::core::ffi::c_char =
            read_sql_file(&raw mut (*m).filepath as *mut ::core::ffi::c_char);
        if content.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        allocated_sql =
            extract_sql_section(content, b"up\0".as_ptr() as *const ::core::ffi::c_char);
        free(content as *mut ::core::ffi::c_void);
        if allocated_sql.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"No 'migrate:up' section found in %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*m).filepath as *mut ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        sql_up = allocated_sql;
    }
    if sql_up.is_null() || strlen(sql_up) == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Empty UP migration: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*m).version as *mut ::core::ffi::c_char,
        );
        if !allocated_sql.is_null() {
            free(allocated_sql as *mut ::core::ffi::c_void);
        }
        return -1 as ::core::ffi::c_int;
    }
    if execute_sql(
        (*ctx).db,
        b"BEGIN TRANSACTION;\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        if !allocated_sql.is_null() {
            free(allocated_sql as *mut ::core::ffi::c_void);
        }
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = execute_validated_migration_sql((*ctx).db, sql_up);
    if result != 0 as ::core::ffi::c_int {
        execute_sql(
            (*ctx).db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !allocated_sql.is_null() {
            free(allocated_sql as *mut ::core::ffi::c_void);
        }
        return -1 as ::core::ffi::c_int;
    }
    result = record_migration_applied(ctx, &raw mut (*m).version as *mut ::core::ffi::c_char);
    if result == 0 as ::core::ffi::c_int {
        execute_sql(
            (*ctx).db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*m).status = migrate_status_t::MIGRATE_STATUS_APPLIED;
    } else {
        execute_sql(
            (*ctx).db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*m).status = migrate_status_t::MIGRATE_STATUS_FAILED;
    }
    if !allocated_sql.is_null() {
        free(allocated_sql as *mut ::core::ffi::c_void);
    }
    return result;
}
unsafe extern "C" fn rollback_migration(
    mut ctx: *mut sqlite_migrate_t,
    mut m: *mut migration_t,
) -> ::core::ffi::c_int {
    let mut sql_down: *const ::core::ffi::c_char = (*m).sql_down;
    let mut allocated_sql: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*m).is_embedded {
        if !(*ctx).config.migrations_dir.is_null() {
            let mut resolved_file: [::core::ffi::c_char; 4096] = [0; 4096];
            let mut resolved_dir: [::core::ffi::c_char; 4096] = [0; 4096];
            if realpath(
                &raw mut (*m).filepath as *mut ::core::ffi::c_char,
                &raw mut resolved_file as *mut ::core::ffi::c_char,
            )
            .is_null()
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Cannot resolve migration file path: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*m).filepath as *mut ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            if realpath(
                (*ctx).config.migrations_dir,
                &raw mut resolved_dir as *mut ::core::ffi::c_char,
            )
            .is_null()
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Cannot resolve migrations directory: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*ctx).config.migrations_dir,
                );
                return -1 as ::core::ffi::c_int;
            }
            let mut dir_len: size_t = strlen(&raw mut resolved_dir as *mut ::core::ffi::c_char);
            if strncmp(
                &raw mut resolved_file as *mut ::core::ffi::c_char,
                &raw mut resolved_dir as *mut ::core::ffi::c_char,
                dir_len,
            ) != 0 as ::core::ffi::c_int
                || resolved_file[dir_len] as ::core::ffi::c_int != '/' as ::core::ffi::c_int
                    && resolved_file[dir_len] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Migration file is outside the configured directory: %s (resolved: %s)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*m).filepath as *mut ::core::ffi::c_char,
                    &raw mut resolved_file as *mut ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        if validate_migration_file_security(&raw mut (*m).filepath as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut content: *mut ::core::ffi::c_char =
            read_sql_file(&raw mut (*m).filepath as *mut ::core::ffi::c_char);
        if content.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        allocated_sql =
            extract_sql_section(content, b"down\0".as_ptr() as *const ::core::ffi::c_char);
        free(content as *mut ::core::ffi::c_void);
        if allocated_sql.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"No 'migrate:down' section found in %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*m).filepath as *mut ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        sql_down = allocated_sql;
    }
    if sql_down.is_null() || strlen(sql_down) == 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Empty DOWN migration: %s (skipping rollback)\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*m).version as *mut ::core::ffi::c_char,
        );
        if !allocated_sql.is_null() {
            free(allocated_sql as *mut ::core::ffi::c_void);
        }
        return 0 as ::core::ffi::c_int;
    }
    if execute_sql(
        (*ctx).db,
        b"BEGIN TRANSACTION;\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        if !allocated_sql.is_null() {
            free(allocated_sql as *mut ::core::ffi::c_void);
        }
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = execute_validated_migration_sql((*ctx).db, sql_down);
    if result != 0 as ::core::ffi::c_int {
        execute_sql(
            (*ctx).db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !allocated_sql.is_null() {
            free(allocated_sql as *mut ::core::ffi::c_void);
        }
        return -1 as ::core::ffi::c_int;
    }
    result = remove_migration_record(ctx, &raw mut (*m).version as *mut ::core::ffi::c_char);
    if result == 0 as ::core::ffi::c_int {
        execute_sql(
            (*ctx).db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*m).status = migrate_status_t::MIGRATE_STATUS_PENDING;
    } else {
        execute_sql(
            (*ctx).db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !allocated_sql.is_null() {
        free(allocated_sql as *mut ::core::ffi::c_void);
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn migrate_init(
    mut db: *mut sqlite3,
    mut config: *const migrate_config_t,
) -> *mut sqlite_migrate_t {
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"migrate_init: NULL database handle\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<sqlite_migrate_t>();
    }
    let mut ctx: *mut sqlite_migrate_t =
        calloc(1 as size_t, ::core::mem::size_of::<sqlite_migrate_t>()) as *mut sqlite_migrate_t;
    if ctx.is_null() {
        return ::core::ptr::null_mut::<sqlite_migrate_t>();
    }
    (*ctx).db = db;
    if !config.is_null() {
        memcpy(
            &raw mut (*ctx).config as *mut ::core::ffi::c_void,
            config as *const ::core::ffi::c_void,
            ::core::mem::size_of::<migrate_config_t>(),
        );
    }
    if create_migrations_table(ctx) != 0 as ::core::ffi::c_int {
        free(ctx as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<sqlite_migrate_t>();
    }
    if scan_migrations_dir(ctx) < 0 as ::core::ffi::c_int {
        free(ctx as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<sqlite_migrate_t>();
    }
    if add_embedded_migrations(ctx) < 0 as ::core::ffi::c_int {
        free((*ctx).migrations as *mut ::core::ffi::c_void);
        free(ctx as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<sqlite_migrate_t>();
    }
    if (*ctx).config.verbose {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Found %d migrations\0".as_ptr() as *const ::core::ffi::c_char,
            (*ctx).migration_count,
        );
    }
    return ctx;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn migrate_free(mut ctx: *mut sqlite_migrate_t) {
    if ctx.is_null() {
        return;
    }
    free((*ctx).migrations as *mut ::core::ffi::c_void);
    free(ctx as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn migrate_up(
    mut ctx: *mut sqlite_migrate_t,
    mut stats: *mut migrate_stats_t,
) -> ::core::ffi::c_int {
    if ctx.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut local_stats: migrate_stats_t = migrate_stats_t {
        total: 0 as ::core::ffi::c_int,
        applied: 0,
        pending: 0,
        failed: 0,
    };
    local_stats.total = (*ctx).migration_count;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*ctx).migration_count {
        let mut m: *mut migration_t = (*ctx).migrations.offset(i as isize);
        if (*m).status.0 == migrate_status_t::MIGRATE_STATUS_APPLIED.0 {
            local_stats.applied += 1;
        } else {
            local_stats.pending += 1;
            if (*ctx).config.verbose {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Applying: %s_%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*m).version as *mut ::core::ffi::c_char,
                    &raw mut (*m).description as *mut ::core::ffi::c_char,
                );
            }
            if (*ctx).config.dry_run {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[DRY RUN] Would apply: %s_%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*m).version as *mut ::core::ffi::c_char,
                    &raw mut (*m).description as *mut ::core::ffi::c_char,
                );
            } else {
                let mut result: ::core::ffi::c_int = apply_migration(ctx, m);
                if (*ctx).config.callback.is_some() {
                    (*ctx).config.callback.expect("non-null function pointer")(
                        &raw mut (*m).version as *mut ::core::ffi::c_char,
                        &raw mut (*m).description as *mut ::core::ffi::c_char,
                        (*m).status,
                        (*ctx).config.callback_data,
                    );
                }
                if result != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to apply migration: %s_%s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*m).version as *mut ::core::ffi::c_char,
                        &raw mut (*m).description as *mut ::core::ffi::c_char,
                    );
                    local_stats.failed += 1;
                    if !stats.is_null() {
                        *stats = local_stats;
                    }
                    return -1 as ::core::ffi::c_int;
                }
                if (*ctx).config.verbose {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Applied: %s_%s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*m).version as *mut ::core::ffi::c_char,
                        &raw mut (*m).description as *mut ::core::ffi::c_char,
                    );
                }
                local_stats.applied += 1;
                local_stats.pending -= 1;
            }
        }
        i += 1;
    }
    if !stats.is_null() {
        *stats = local_stats;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn migrate_down(mut ctx: *mut sqlite_migrate_t) -> ::core::ffi::c_int {
    return migrate_down_n(ctx, 1 as ::core::ffi::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn migrate_down_n(
    mut ctx: *mut sqlite_migrate_t,
    mut count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if ctx.is_null() || count <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut rolled_back: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = (*ctx).migration_count - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int && rolled_back < count {
        let mut m: *mut migration_t = (*ctx).migrations.offset(i as isize);
        if (*m).status.0 == migrate_status_t::MIGRATE_STATUS_APPLIED.0 {
            if (*ctx).config.verbose {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Rolling back: %s_%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*m).version as *mut ::core::ffi::c_char,
                    &raw mut (*m).description as *mut ::core::ffi::c_char,
                );
            }
            if (*ctx).config.dry_run {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"[DRY RUN] Would rollback: %s_%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*m).version as *mut ::core::ffi::c_char,
                    &raw mut (*m).description as *mut ::core::ffi::c_char,
                );
                rolled_back += 1;
            } else {
                let mut result: ::core::ffi::c_int = rollback_migration(ctx, m);
                if (*ctx).config.callback.is_some() {
                    (*ctx).config.callback.expect("non-null function pointer")(
                        &raw mut (*m).version as *mut ::core::ffi::c_char,
                        &raw mut (*m).description as *mut ::core::ffi::c_char,
                        (*m).status,
                        (*ctx).config.callback_data,
                    );
                }
                if result != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_ERROR,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Failed to rollback migration: %s_%s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*m).version as *mut ::core::ffi::c_char,
                        &raw mut (*m).description as *mut ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if (*ctx).config.verbose {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Rolled back: %s_%s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*m).version as *mut ::core::ffi::c_char,
                        &raw mut (*m).description as *mut ::core::ffi::c_char,
                    );
                }
                rolled_back += 1;
            }
        }
        i -= 1;
    }
    return rolled_back;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn migrate_status(
    mut ctx: *mut sqlite_migrate_t,
    mut migrations: *mut migration_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if ctx.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = if (*ctx).migration_count < max_count {
        (*ctx).migration_count
    } else {
        max_count
    };
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        memcpy(
            migrations.offset(i as isize) as *mut ::core::ffi::c_void,
            (*ctx).migrations.offset(i as isize) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<migration_t>(),
        );
        i += 1;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn migrate_get_version(
    mut ctx: *const sqlite_migrate_t,
    mut version: *mut ::core::ffi::c_char,
    mut version_len: size_t,
) -> ::core::ffi::c_int {
    if ctx.is_null() || version.is_null() || version_len == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    *version.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = (*ctx).migration_count - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        if (*(*ctx).migrations.offset(i as isize)).status.0
            == migrate_status_t::MIGRATE_STATUS_APPLIED.0
        {
            safe_strcpy(
                version,
                &raw mut (*(*ctx).migrations.offset(i as isize)).version
                    as *mut ::core::ffi::c_char,
                version_len,
                0 as size_t,
            );
            return 0 as ::core::ffi::c_int;
        }
        i -= 1;
    }
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn c2rust_run_static_initializers() {
    allowed = [
        C2Rust_Unnamed_1 {
            kw: b"CREATE UNIQUE INDEX\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 20]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"CREATE TRIGGER\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 15]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"CREATE TABLE\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"CREATE INDEX\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"CREATE VIEW\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 12]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"ALTER TABLE\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 12]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"DROP TRIGGER\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 13]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"DROP TABLE\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 11]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"DROP INDEX\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 11]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"DROP VIEW\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 10]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"INSERT OR REPLACE\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 18]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"INSERT OR IGNORE\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 17]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"INSERT\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 7]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"REPLACE\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 8]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"UPDATE\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 7]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 7]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: b"SELECT\0".as_ptr() as *const ::core::ffi::c_char,
            len: ::core::mem::size_of::<[::core::ffi::c_char; 7]>().wrapping_sub(1 as size_t),
        },
        C2Rust_Unnamed_1 {
            kw: ::core::ptr::null::<::core::ffi::c_char>(),
            len: 0 as size_t,
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
#[cfg_attr(target_os = "windows", unsafe(link_section = ".CRT$XIB"))]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__mod_init_func"))]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
