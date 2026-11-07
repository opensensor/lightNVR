unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn sqlite3_busy_timeout(_: *mut sqlite3, ms: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    unsafe fn sqlite3_column_bytes(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn db_open_readonly_connection(connection: *mut *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn db_close_readonly_connection(connection: *mut sqlite3);
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
    unsafe fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
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
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_get_system_setting(
    mut key: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_char,
    mut out_len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if key.is_null() || out.is_null() || out_len <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"db_get_system_setting: no db handle\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char =
        b"SELECT value FROM system_settings WHERE key = ? LIMIT 1;\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"db_get_system_setting: prepare failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        key,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    let mut rc: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_step(stmt) == SQLITE_ROW {
        let mut val: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !val.is_null() {
            safe_strcpy(out, val, out_len as size_t, 0 as size_t);
            rc = 0 as ::core::ffi::c_int;
        }
    }
    sqlite3_finalize(stmt);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_get_system_setting_alloc(
    mut key: *const ::core::ffi::c_char,
    mut out: *mut *mut ::core::ffi::c_char,
    mut out_len: *mut size_t,
) -> ::core::ffi::c_int {
    if key.is_null() || out.is_null() || out_len.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *out = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *out_len = 0 as size_t;
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"db_get_system_setting_alloc: no db handle\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char =
        b"SELECT value FROM system_settings WHERE key = ? LIMIT 1;\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"db_get_system_setting_alloc: prepare failed: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        key,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    let mut rc: ::core::ffi::c_int = 0;
    let mut step: ::core::ffi::c_int = sqlite3_step(stmt);
    if step == SQLITE_ROW {
        let mut val: *const ::core::ffi::c_uchar =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int);
        let mut nbytes: ::core::ffi::c_int = sqlite3_column_bytes(stmt, 0 as ::core::ffi::c_int);
        if val.is_null() || nbytes < 0 as ::core::ffi::c_int {
            rc = 1 as ::core::ffi::c_int;
        } else {
            let mut buf: *mut ::core::ffi::c_char =
                malloc((nbytes as size_t).wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
            if buf.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"db_get_system_setting_alloc: malloc(%d) failed\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    nbytes + 1 as ::core::ffi::c_int,
                );
                rc = -1 as ::core::ffi::c_int;
            } else {
                if nbytes > 0 as ::core::ffi::c_int {
                    memcpy(
                        buf as *mut ::core::ffi::c_void,
                        val as *const ::core::ffi::c_void,
                        nbytes as size_t,
                    );
                }
                *buf.offset(nbytes as isize) = '\0' as ::core::ffi::c_char;
                *out = buf;
                *out_len = nbytes as size_t;
                rc = 0 as ::core::ffi::c_int;
            }
        }
    } else if step == SQLITE_DONE {
        rc = 1 as ::core::ffi::c_int;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"db_get_system_setting_alloc: step failed (rc=%d): %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            step,
            sqlite3_errmsg(db),
        );
        rc = -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_set_system_setting(
    mut key: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if key.is_null() || value.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"db_set_system_setting: no db handle\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO system_settings (key, value, updated_at) VALUES (?, ?, strftime('%s','now')) ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"db_set_system_setting: prepare failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        key,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        value,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    let mut rc: ::core::ffi::c_int = if sqlite3_step(stmt) == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
    if rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"db_set_system_setting: step failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    sqlite3_finalize(stmt);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_is_setup_complete() -> bool {
    let mut val: [::core::ffi::c_char; 8] = [0 as ::core::ffi::c_char, 0, 0, 0, 0, 0, 0, 0];
    if db_get_system_setting(
        b"setup_complete\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut val as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>() as ::core::ffi::c_int,
    ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    return val[0usize] as ::core::ffi::c_int == '1' as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_get_setup_status(
    mut complete: *mut bool,
    mut completed_at: *mut ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    if complete.is_null() || completed_at.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *complete = r#false != 0;
    *completed_at = 0 as ::core::ffi::c_longlong;
    let mut connection: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut connection) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_busy_timeout(connection, 1000 as ::core::ffi::c_int);
    let mut sql: *const ::core::ffi::c_char = b"SELECT (SELECT value FROM system_settings WHERE key = 'setup_complete'), (SELECT value FROM system_settings WHERE key = 'setup_completed_at');\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        connection,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        rc = sqlite3_step(statement);
    }
    if rc == SQLITE_ROW {
        let mut value: *const ::core::ffi::c_char =
            sqlite3_column_text(statement, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut timestamp: *const ::core::ffi::c_char =
            sqlite3_column_text(statement, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        *complete = !value.is_null()
            && *value.offset(0isize) as ::core::ffi::c_int == '1' as ::core::ffi::c_int;
        *completed_at = if !timestamp.is_null() {
            strtoll(
                timestamp,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                10 as ::core::ffi::c_int,
            )
        } else {
            0 as ::core::ffi::c_longlong
        };
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to read setup status: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(connection),
        );
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    db_close_readonly_connection(connection);
    return if rc == SQLITE_ROW {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_mark_setup_complete() -> ::core::ffi::c_int {
    if db_set_system_setting(
        b"setup_complete\0".as_ptr() as *const ::core::ffi::c_char,
        b"1\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut ts: [::core::ffi::c_char; 32] = [0; 32];
    snprintf(
        &raw mut ts as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
        time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_longlong,
    );
    db_set_system_setting(
        b"setup_completed_at\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut ts as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Setup marked as complete\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
