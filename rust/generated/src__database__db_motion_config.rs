unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn sqlite3_changes(_: *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
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
    unsafe fn sqlite3_column_int64(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int)
    -> sqlite3_int64;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
}
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int64_t = i64;
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
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_motion_config(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    if db.is_null() || stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for delete_motion_config\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char =
        b"DELETE FROM motion_recording_config WHERE stream_name = ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare delete_motion_config statement: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        stream_name,
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
            b"Failed to delete motion config for %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(db_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Deleted motion recording config for stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cleanup_old_motion_recordings(
    mut stream_name: *const ::core::ffi::c_char,
    mut retention_days: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    let mut deleted_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if db.is_null() || retention_days < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for cleanup_old_motion_recordings\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut cutoff_time: time_t = time(::core::ptr::null_mut::<time_t>())
        - retention_days as time_t * 24 as time_t * 60 as time_t * 60 as time_t;
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !stream_name.is_null() {
        sql = b"DELETE FROM motion_recordings WHERE stream_name = ? AND start_time < ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
    } else {
        sql = b"DELETE FROM motion_recordings WHERE start_time < ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
    }
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare cleanup_old_motion_recordings statement: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    if !stream_name.is_null() {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, cutoff_time as sqlite3_int64);
    } else {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, cutoff_time as sqlite3_int64);
    }
    rc = sqlite3_step(stmt);
    if rc == SQLITE_DONE {
        deleted_count = sqlite3_changes(db);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to cleanup old motion recordings: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        deleted_count = -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    if deleted_count > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Cleaned up %d old motion recordings (retention: %d days)\0".as_ptr()
                as *const ::core::ffi::c_char,
            deleted_count,
            retention_days,
        );
    }
    return deleted_count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_motion_recordings_disk_usage(
    mut stream_name: *const ::core::ffi::c_char,
) -> int64_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = 0;
    let mut total_size: int64_t = 0 as int64_t;
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as int64_t;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !stream_name.is_null() {
        sql = b"SELECT COALESCE(SUM(size_bytes), 0) FROM motion_recordings WHERE stream_name = ?;\0"
            .as_ptr() as *const ::core::ffi::c_char;
    } else {
        sql = b"SELECT COALESCE(SUM(size_bytes), 0) FROM motion_recordings;\0".as_ptr()
            as *const ::core::ffi::c_char;
    }
    rc = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare get_motion_recordings_disk_usage statement: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as int64_t;
    }
    if !stream_name.is_null() {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    }
    rc = sqlite3_step(stmt);
    if rc == SQLITE_ROW {
        total_size = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return total_size;
}
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
