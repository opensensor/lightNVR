unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    unsafe fn sqlite3_free(_: *mut ::core::ffi::c_void);
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
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
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_schema_management() -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut err_msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut create_version_table: *const ::core::ffi::c_char = b"CREATE TABLE IF NOT EXISTS schema_version (id INTEGER PRIMARY KEY CHECK (id = 1),version INTEGER NOT NULL,updated_at INTEGER NOT NULL);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    rc = sqlite3_exec(db, create_version_table, None, NULL, &raw mut err_msg);
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to create schema_version table: %s\0".as_ptr() as *const ::core::ffi::c_char,
            err_msg,
        );
        if !err_msg.is_null() {
            sqlite3_free(err_msg as *mut ::core::ffi::c_void);
            err_msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = sqlite3_prepare_v2(
        db,
        b"SELECT COUNT(*) FROM schema_version;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return rc;
    }
    if sqlite3_step(stmt) == SQLITE_ROW {
        let mut count: ::core::ffi::c_int = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
        sqlite3_finalize(stmt);
        if count == 0 as ::core::ffi::c_int {
            let mut init_version: *const ::core::ffi::c_char = b"INSERT INTO schema_version (id, version, updated_at) VALUES (1, 1, strftime('%s','now'));\0"
                .as_ptr() as *const ::core::ffi::c_char;
            rc = sqlite3_exec(db, init_version, None, NULL, &raw mut err_msg);
            if rc != SQLITE_OK {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to initialize schema version: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    err_msg,
                );
                if !err_msg.is_null() {
                    sqlite3_free(err_msg as *mut ::core::ffi::c_void);
                    err_msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                return -1 as ::core::ffi::c_int;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Initialized database schema to version 1\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    } else {
        sqlite3_finalize(stmt);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to check schema_version table\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_schema_version() -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut version: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT version FROM schema_version WHERE id = 1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
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
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return rc;
    }
    if sqlite3_step(stmt) == SQLITE_ROW {
        version = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    return version;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn run_schema_migrations() -> ::core::ffi::c_int {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT,
        _log_stream_name,
        b"run_schema_migrations() is deprecated - migrations are now handled by run_database_migrations()\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn column_exists(
    mut table_name: *const ::core::ffi::c_char,
    mut column_name: *const ::core::ffi::c_char,
) -> bool {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut check_table_stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut exists: bool = r#false != 0;
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() || table_name.is_null() || column_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for column_exists: table=%s, column=%s\0".as_ptr()
                as *const ::core::ffi::c_char,
            if !table_name.is_null() {
                table_name
            } else {
                b"NULL\0".as_ptr() as *const ::core::ffi::c_char
            },
            if !column_name.is_null() {
                column_name
            } else {
                b"NULL\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return r#false != 0;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Checking if column %s exists in table %s\0".as_ptr() as *const ::core::ffi::c_char,
        column_name,
        table_name,
    );
    let mut check_table_sql: [::core::ffi::c_char; 256] = [0; 256];
    snprintf(
        &raw mut check_table_sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"SELECT name FROM sqlite_master WHERE type='table' AND name='%s';\0".as_ptr()
            as *const ::core::ffi::c_char,
        table_name,
    );
    rc = sqlite3_prepare_v2(
        db,
        &raw mut check_table_sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut check_table_stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement to check table existence\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut table_exists: bool = r#false != 0;
    if sqlite3_step(check_table_stmt) == SQLITE_ROW {
        table_exists = r#true != 0;
    }
    if !check_table_stmt.is_null() {
        sqlite3_finalize(check_table_stmt);
        check_table_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    if !table_exists {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Table %s does not exist\0".as_ptr() as *const ::core::ffi::c_char,
            table_name,
        );
        return r#false != 0;
    }
    let mut sql: [::core::ffi::c_char; 256] = [0; 256];
    snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"PRAGMA table_info(%s);\0".as_ptr() as *const ::core::ffi::c_char,
        table_name,
    );
    rc = sqlite3_prepare_v2(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement to check column existence\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    while sqlite3_step(stmt) == SQLITE_ROW {
        let mut name: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !(!name.is_null() && strcmp(name, column_name) == 0 as ::core::ffi::c_int) {
            continue;
        }
        exists = r#true != 0;
        break;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT,
        _log_stream_name,
        b"Column %s %s in table %s\0".as_ptr() as *const ::core::ffi::c_char,
        column_name,
        if exists as ::core::ffi::c_int != 0 {
            b"exists\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"does not exist\0".as_ptr() as *const ::core::ffi::c_char
        },
        table_name,
    );
    return exists;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_column_if_not_exists(
    mut table_name: *const ::core::ffi::c_char,
    mut column_name: *const ::core::ffi::c_char,
    mut column_def: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut err_msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut check_table_stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut check_column_stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut db: *mut sqlite3 = get_db_handle();
    if db.is_null() || table_name.is_null() || column_name.is_null() || column_def.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for add_column_if_not_exists: table=%s, column=%s, def=%s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            if !table_name.is_null() {
                table_name
            } else {
                b"NULL\0".as_ptr() as *const ::core::ffi::c_char
            },
            if !column_name.is_null() {
                column_name
            } else {
                b"NULL\0".as_ptr() as *const ::core::ffi::c_char
            },
            if !column_def.is_null() {
                column_def
            } else {
                b"NULL\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Checking if column %s exists in table %s\0".as_ptr() as *const ::core::ffi::c_char,
        column_name,
        table_name,
    );
    let mut check_table_sql: [::core::ffi::c_char; 256] = [0; 256];
    snprintf(
        &raw mut check_table_sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"SELECT name FROM sqlite_master WHERE type='table' AND name='%s';\0".as_ptr()
            as *const ::core::ffi::c_char,
        table_name,
    );
    rc = sqlite3_prepare_v2(
        db,
        &raw mut check_table_sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut check_table_stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement to check table existence\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut table_exists: bool = r#false != 0;
    if sqlite3_step(check_table_stmt) == SQLITE_ROW {
        table_exists = r#true != 0;
    }
    if !check_table_stmt.is_null() {
        sqlite3_finalize(check_table_stmt);
        check_table_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    if !table_exists {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Table %s does not exist\0".as_ptr() as *const ::core::ffi::c_char,
            table_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut check_column_sql: [::core::ffi::c_char; 256] = [0; 256];
    snprintf(
        &raw mut check_column_sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"PRAGMA table_info(%s);\0".as_ptr() as *const ::core::ffi::c_char,
        table_name,
    );
    rc = sqlite3_prepare_v2(
        db,
        &raw mut check_column_sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut check_column_stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement to check column existence\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut col_found: bool = r#false != 0;
    while sqlite3_step(check_column_stmt) == SQLITE_ROW {
        let mut name: *const ::core::ffi::c_char =
            sqlite3_column_text(check_column_stmt, 1 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char;
        if !(!name.is_null() && strcmp(name, column_name) == 0 as ::core::ffi::c_int) {
            continue;
        }
        col_found = r#true != 0;
        break;
    }
    if !check_column_stmt.is_null() {
        sqlite3_finalize(check_column_stmt);
        check_column_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    if col_found {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Column %s already exists in table %s\0".as_ptr() as *const ::core::ffi::c_char,
            column_name,
            table_name,
        );
        return 0 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Adding column %s to table %s with definition: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        column_name,
        table_name,
        column_def,
    );
    let mut sql: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"ALTER TABLE %s ADD COLUMN %s %s;\0".as_ptr() as *const ::core::ffi::c_char,
        table_name,
        column_name,
        column_def,
    );
    rc = sqlite3_exec(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
        None,
        NULL,
        &raw mut err_msg,
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to add column %s to table %s: %s (SQL: %s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            column_name,
            table_name,
            err_msg,
            &raw mut sql as *mut ::core::ffi::c_char,
        );
        if !err_msg.is_null() {
            sqlite3_free(err_msg as *mut ::core::ffi::c_void);
            err_msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return -1 as ::core::ffi::c_int;
    }
    rc = sqlite3_prepare_v2(
        db,
        &raw mut check_column_sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut check_column_stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare statement to verify column was added\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut column_added: bool = r#false != 0;
    while sqlite3_step(check_column_stmt) == SQLITE_ROW {
        let mut name_0: *const ::core::ffi::c_char =
            sqlite3_column_text(check_column_stmt, 1 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char;
        if !(!name_0.is_null() && strcmp(name_0, column_name) == 0 as ::core::ffi::c_int) {
            continue;
        }
        column_added = r#true != 0;
        break;
    }
    if !check_column_stmt.is_null() {
        sqlite3_finalize(check_column_stmt);
        check_column_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    if !column_added {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Column %s was not added to table %s despite no error\0".as_ptr()
                as *const ::core::ffi::c_char,
            column_name,
            table_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Successfully added column %s to table %s\0".as_ptr() as *const ::core::ffi::c_char,
        column_name,
        table_name,
    );
    return 0 as ::core::ffi::c_int;
}
