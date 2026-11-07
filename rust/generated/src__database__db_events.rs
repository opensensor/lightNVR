unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn sqlite3_last_insert_rowid(_: *mut sqlite3) -> sqlite3_int64;
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
    unsafe fn sqlite3_column_int64(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int)
    -> sqlite3_int64;
    unsafe fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
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
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcat(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
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
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_type_t(pub ::core::ffi::c_uint);
impl event_type_t {
    pub const EVENT_RECORDING_START: Self = Self(0);
    pub const EVENT_RECORDING_STOP: Self = Self(1);
    pub const EVENT_STREAM_CONNECTED: Self = Self(2);
    pub const EVENT_STREAM_DISCONNECTED: Self = Self(3);
    pub const EVENT_STREAM_ERROR: Self = Self(4);
    pub const EVENT_SYSTEM_START: Self = Self(5);
    pub const EVENT_SYSTEM_STOP: Self = Self(6);
    pub const EVENT_STORAGE_LOW: Self = Self(7);
    pub const EVENT_STORAGE_FULL: Self = Self(8);
    pub const EVENT_USER_LOGIN: Self = Self(9);
    pub const EVENT_USER_LOGOUT: Self = Self(10);
    pub const EVENT_CONFIG_CHANGE: Self = Self(11);
    pub const EVENT_CUSTOM: Self = Self(12);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_info_t {
    pub id: uint64_t,
    pub r#type: event_type_t,
    pub timestamp: time_t,
    pub stream_name: [::core::ffi::c_char; 64],
    pub description: [::core::ffi::c_char; 256],
    pub details: [::core::ffi::c_char; 1024],
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
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_event(
    mut r#type: event_type_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut description: *const ::core::ffi::c_char,
    mut details: *const ::core::ffi::c_char,
) -> uint64_t {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut event_id: uint64_t = 0 as uint64_t;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as uint64_t;
    }
    if description.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Event description is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as uint64_t;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO events (type, timestamp, stream_name, description, details) VALUES (?, ?, ?, ?, ?);\0"
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
        pthread_mutex_unlock(db_mutex);
        return 0 as uint64_t;
    }
    sqlite3_bind_int(
        stmt,
        1 as ::core::ffi::c_int,
        r#type.0 as ::core::ffi::c_int,
    );
    sqlite3_bind_int64(
        stmt,
        2 as ::core::ffi::c_int,
        time(::core::ptr::null_mut::<time_t>()) as sqlite3_int64,
    );
    if !stream_name.is_null() {
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    } else {
        sqlite3_bind_null(stmt, 3 as ::core::ffi::c_int);
    }
    sqlite3_bind_text(
        stmt,
        4 as ::core::ffi::c_int,
        description,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    if !details.is_null() {
        sqlite3_bind_text(
            stmt,
            5 as ::core::ffi::c_int,
            details,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    } else {
        sqlite3_bind_null(stmt, 5 as ::core::ffi::c_int);
    }
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to add event: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    } else {
        event_id = sqlite3_last_insert_rowid(db) as uint64_t;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Added event with ID %llu\0".as_ptr() as *const ::core::ffi::c_char,
            event_id as ::core::ffi::c_ulonglong,
        );
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return event_id;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_events(
    mut start_time: time_t,
    mut end_time: time_t,
    mut r#type: ::core::ffi::c_int,
    mut stream_name: *const ::core::ffi::c_char,
    mut events: *mut event_info_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if events.is_null() || max_count <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for get_events\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: [::core::ffi::c_char; 1024] = [0; 1024];
    safe_strcpy(
        &raw mut sql as *mut ::core::ffi::c_char,
        b"SELECT id, type, timestamp, stream_name, description, details FROM events WHERE 1=1\0"
            .as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        0 as size_t,
    );
    if start_time > 0 as time_t {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND timestamp >= ?\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        );
    }
    if end_time > 0 as time_t {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND timestamp <= ?\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        );
    }
    if r#type >= 0 as ::core::ffi::c_int {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND type = ?\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        );
    }
    if !stream_name.is_null() {
        safe_strcat(
            &raw mut sql as *mut ::core::ffi::c_char,
            b" AND stream_name = ?\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        );
    }
    safe_strcat(
        &raw mut sql as *mut ::core::ffi::c_char,
        b" ORDER BY timestamp DESC LIMIT ?;\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
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
            b"Failed to prepare statement: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut param_index: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if start_time > 0 as time_t {
        let c2rust_fresh0 = param_index;
        param_index += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh0, start_time as sqlite3_int64);
    }
    if end_time > 0 as time_t {
        let c2rust_fresh1 = param_index;
        param_index += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh1, end_time as sqlite3_int64);
    }
    if r#type >= 0 as ::core::ffi::c_int {
        let c2rust_fresh2 = param_index;
        param_index += 1;
        sqlite3_bind_int(stmt, c2rust_fresh2, r#type);
    }
    if !stream_name.is_null() {
        let c2rust_fresh3 = param_index;
        param_index += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh3,
            stream_name,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    }
    sqlite3_bind_int(stmt, param_index, max_count);
    while sqlite3_step(stmt) == SQLITE_ROW && count < max_count {
        (*events.offset(count as isize)).id =
            sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
        (*events.offset(count as isize)).r#type =
            event_type_t(sqlite3_column_int(stmt, 1 as ::core::ffi::c_int) as ::core::ffi::c_uint);
        (*events.offset(count as isize)).timestamp =
            sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int) as time_t;
        let mut stream: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 3 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !stream.is_null() {
            safe_strcpy(
                &raw mut (*events.offset(count as isize)).stream_name as *mut ::core::ffi::c_char,
                stream,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
        } else {
            (*events.offset(count as isize)).stream_name[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut desc: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 4 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !desc.is_null() {
            safe_strcpy(
                &raw mut (*events.offset(count as isize)).description as *mut ::core::ffi::c_char,
                desc,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        } else {
            (*events.offset(count as isize)).description[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut details: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 5 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        if !details.is_null() {
            safe_strcpy(
                &raw mut (*events.offset(count as isize)).details as *mut ::core::ffi::c_char,
                details,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                0 as size_t,
            );
        } else {
            (*events.offset(count as isize)).details[0usize] = '\0' as ::core::ffi::c_char;
        }
        count += 1;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Found %d events in database matching criteria\0".as_ptr() as *const ::core::ffi::c_char,
        count,
    );
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_old_events(mut max_age: uint64_t) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut deleted_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(db_mutex);
    let mut sql: *const ::core::ffi::c_char =
        b"DELETE FROM events WHERE timestamp < ?;\0".as_ptr() as *const ::core::ffi::c_char;
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
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut cutoff_time: time_t = time(::core::ptr::null_mut::<time_t>()) - max_age as time_t;
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, cutoff_time as sqlite3_int64);
    rc = sqlite3_step(stmt);
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to delete old events: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    deleted_count = sqlite3_changes(db);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(db_mutex);
    return deleted_count;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
