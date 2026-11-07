unsafe extern "C" {
    pub type sqlite3_stmt;
    pub type sqlite3;
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
    unsafe fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct workspace_preferences_t {
    pub live_navigator_visible: bool,
    pub investigation_visible: bool,
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
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub const WORKSPACE_KEY_LIVE_NAVIGATOR: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"live.navigator\0") };
pub const WORKSPACE_KEY_INVESTIGATION: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"investigation\0") };
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_workspace_preferences_defaults(
    mut preferences: *mut workspace_preferences_t,
) {
    if preferences.is_null() {
        return;
    }
    (*preferences).live_navigator_visible = r#true != 0;
    (*preferences).investigation_visible = r#true != 0;
}
unsafe extern "C" fn bind_owner(
    mut statement: *mut sqlite3_stmt,
    mut parameter: ::core::ffi::c_int,
    mut owner_user_id: int64_t,
) {
    sqlite3_bind_int64(statement, parameter, owner_user_id as sqlite3_int64);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_workspace_preferences_get(
    mut owner_user_id: int64_t,
    mut preferences: *mut workspace_preferences_t,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() || preferences.is_null() || owner_user_id < 0 as int64_t {
        return -1 as ::core::ffi::c_int;
    }
    db_workspace_preferences_defaults(preferences);
    let mut sql: *const ::core::ffi::c_char = b"SELECT workspace_key,is_visible FROM user_workspace_preferences WHERE (?=0 AND owner_user_id IS NULL) OR owner_user_id=?;\0"
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
        bind_owner(statement, 1 as ::core::ffi::c_int, owner_user_id);
        bind_owner(statement, 2 as ::core::ffi::c_int, owner_user_id);
    }
    if result == SQLITE_OK {
        loop {
            result = sqlite3_step(statement);
            if result != SQLITE_ROW {
                break;
            }
            let mut key: *const ::core::ffi::c_char =
                sqlite3_column_text(statement, 0 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char;
            let mut visible: bool =
                sqlite3_column_int(statement, 1 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
            if !key.is_null()
                && strcmp(key, WORKSPACE_KEY_LIVE_NAVIGATOR.as_ptr()) == 0 as ::core::ffi::c_int
            {
                (*preferences).live_navigator_visible = visible;
            } else if !key.is_null()
                && strcmp(key, WORKSPACE_KEY_INVESTIGATION.as_ptr()) == 0 as ::core::ffi::c_int
            {
                (*preferences).investigation_visible = visible;
            }
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if result != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to load workspace preferences: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    pthread_mutex_unlock(mutex);
    return if result == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn replace_one(
    mut db: *mut sqlite3,
    mut owner_user_id: int64_t,
    mut key: *const ::core::ffi::c_char,
    mut visible: bool,
) -> ::core::ffi::c_int {
    let mut update_sql: *const ::core::ffi::c_char = b"UPDATE user_workspace_preferences SET is_visible=?,updated_at=strftime('%s','now') WHERE workspace_key=? AND ((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        update_sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_int(
            statement,
            1 as ::core::ffi::c_int,
            if visible as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            key,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        bind_owner(statement, 3 as ::core::ffi::c_int, owner_user_id);
        bind_owner(statement, 4 as ::core::ffi::c_int, owner_user_id);
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if result != SQLITE_DONE {
        return -1 as ::core::ffi::c_int;
    }
    if sqlite3_changes(db) > 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut insert_sql: *const ::core::ffi::c_char = b"INSERT INTO user_workspace_preferences(owner_user_id,workspace_key,is_visible) VALUES(?,?,?);\0"
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
        if owner_user_id > 0 as int64_t {
            sqlite3_bind_int64(
                statement,
                1 as ::core::ffi::c_int,
                owner_user_id as sqlite3_int64,
            );
        } else {
            sqlite3_bind_null(statement, 1 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            key,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_int(
            statement,
            3 as ::core::ffi::c_int,
            if visible as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_workspace_preferences_replace(
    mut owner_user_id: int64_t,
    mut preferences: *const workspace_preferences_t,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() || preferences.is_null() || owner_user_id < 0 as int64_t {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut result: ::core::ffi::c_int = sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK
        && replace_one(
            db,
            owner_user_id,
            WORKSPACE_KEY_LIVE_NAVIGATOR.as_ptr(),
            (*preferences).live_navigator_visible,
        ) == 0 as ::core::ffi::c_int
        && replace_one(
            db,
            owner_user_id,
            WORKSPACE_KEY_INVESTIGATION.as_ptr(),
            (*preferences).investigation_visible,
        ) == 0 as ::core::ffi::c_int
    {
        result = sqlite3_exec(
            db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    } else {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        result = SQLITE_ERROR;
    }
    pthread_mutex_unlock(mutex);
    return if result == SQLITE_OK {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
