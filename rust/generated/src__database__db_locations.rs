use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn sqlite3_last_insert_rowid(_: *mut sqlite3) -> sqlite3_int64;
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
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camera_location_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub parent_uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub r#type: [::core::ffi::c_char; 64],
    pub metadata_json: [::core::ffi::c_char; 512],
    pub sort_order: ::core::ffi::c_int,
    pub is_system: ::core::ffi::c_int,
    pub direct_child_count: ::core::ffi::c_int,
    pub direct_camera_count: ::core::ffi::c_int,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_location_result_t(pub ::core::ffi::c_int);
impl db_location_result_t {
    pub const DB_LOCATION_OK: Self = Self(0);
    pub const DB_LOCATION_NOT_FOUND: Self = Self(-1);
    pub const DB_LOCATION_CONFLICT: Self = Self(-2);
    pub const DB_LOCATION_INVALID: Self = Self(-3);
    pub const DB_LOCATION_ERROR: Self = Self(-4);
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
unsafe extern "C" fn has_non_whitespace(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null() {
        return r#false != 0;
    }
    while *value as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || *value as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        || *value as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
        || *value as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
    {
        value = value.offset(1);
    }
    return *value as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
}
unsafe extern "C" fn valid_uuid_string(mut uuid: *const ::core::ffi::c_char) -> bool {
    return !uuid.is_null()
        && strlen(uuid) == (CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int) as size_t;
}
unsafe extern "C" fn copy_column(
    mut destination: *mut ::core::ffi::c_char,
    mut destination_size: size_t,
    mut stmt: *mut sqlite3_stmt,
    mut column: ::core::ffi::c_int,
) {
    let mut value: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, column) as *const ::core::ffi::c_char;
    if !value.is_null() {
        safe_strcpy(destination, value, destination_size, 0 as size_t);
    } else if destination_size > 0 as size_t {
        *destination.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn populate_location(
    mut stmt: *mut sqlite3_stmt,
    mut location: *mut camera_location_t,
) {
    memset(
        location as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<camera_location_t>(),
    );
    copy_column(
        &raw mut (*location).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        stmt,
        0 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*location).parent_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        stmt,
        1 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*location).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        stmt,
        2 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*location).r#type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        stmt,
        3 as ::core::ffi::c_int,
    );
    (*location).sort_order = sqlite3_column_int(stmt, 4 as ::core::ffi::c_int);
    copy_column(
        &raw mut (*location).metadata_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        stmt,
        5 as ::core::ffi::c_int,
    );
    (*location).is_system = sqlite3_column_int(stmt, 6 as ::core::ffi::c_int);
    (*location).created_at = sqlite3_column_int64(stmt, 7 as ::core::ffi::c_int) as int64_t;
    (*location).updated_at = sqlite3_column_int64(stmt, 8 as ::core::ffi::c_int) as int64_t;
    (*location).direct_child_count = sqlite3_column_int(stmt, 9 as ::core::ffi::c_int);
    (*location).direct_camera_count = sqlite3_column_int(stmt, 10 as ::core::ffi::c_int);
}
unsafe extern "C" fn prepare_error(
    mut db: *mut sqlite3,
    mut operation: *const ::core::ffi::c_char,
) -> db_location_result_t {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT,
        _log_stream_name,
        b"Failed to %s camera location: %s\0".as_ptr() as *const ::core::ffi::c_char,
        operation,
        sqlite3_errmsg(db),
    );
    return db_location_result_t::DB_LOCATION_ERROR;
}
unsafe extern "C" fn get_locked(
    mut db: *mut sqlite3,
    mut where_clause: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
    mut location: *mut camera_location_t,
) -> db_location_result_t {
    let mut sql: [::core::ffi::c_char; 768] = [0; 768];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 768]>(),
        b"SELECT l.uuid, l.parent_uuid, l.name, l.type, l.sort_order, l.metadata_json, l.is_system, l.created_at, l.updated_at, (SELECT count(*) FROM camera_locations c WHERE c.parent_uuid = l.uuid), (SELECT count(*) FROM streams s WHERE s.location_uuid = l.uuid) FROM camera_locations l WHERE %s LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        where_clause,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 768]>()
    {
        return db_location_result_t::DB_LOCATION_ERROR;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return prepare_error(db, b"read\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if !value.is_null() {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            value,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
    let mut result: db_location_result_t = db_location_result_t::DB_LOCATION_NOT_FOUND;
    rc = sqlite3_step(stmt);
    if rc == SQLITE_ROW {
        populate_location(stmt, location);
        result = db_location_result_t::DB_LOCATION_OK;
    } else if rc != SQLITE_DONE {
        result = prepare_error(db, b"read\0".as_ptr() as *const ::core::ffi::c_char);
    }
    sqlite3_finalize(stmt);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_location_get_unassigned(
    mut location: *mut camera_location_t,
) -> db_location_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || location.is_null() {
        return db_location_result_t::DB_LOCATION_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut result: db_location_result_t = get_locked(
        db,
        b"l.is_system = 1\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        location,
    );
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_location_get(
    mut uuid: *const ::core::ffi::c_char,
    mut location: *mut camera_location_t,
) -> db_location_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || location.is_null() || !valid_uuid_string(uuid) {
        return db_location_result_t::DB_LOCATION_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut result: db_location_result_t = get_locked(
        db,
        b"l.uuid = ?\0".as_ptr() as *const ::core::ffi::c_char,
        uuid,
        location,
    );
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_location_count() -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT count(*) FROM camera_locations;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW {
        count = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
    } else {
        prepare_error(db, b"count\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_location_list(
    mut locations: *mut camera_location_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || locations.is_null() || max_count <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT l.uuid, l.parent_uuid, l.name, l.type, l.sort_order, l.metadata_json, l.is_system, l.created_at, l.updated_at, (SELECT count(*) FROM camera_locations c WHERE c.parent_uuid = l.uuid), (SELECT count(*) FROM streams s WHERE s.location_uuid = l.uuid) FROM camera_locations l ORDER BY CASE WHEN l.parent_uuid IS NULL THEN 0 ELSE 1 END, l.parent_uuid, l.sort_order, l.name COLLATE NOCASE;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        prepare_error(db, b"list\0".as_ptr() as *const ::core::ffi::c_char);
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while count < max_count && {
        rc = sqlite3_step(stmt);
        rc == SQLITE_ROW
    } {
        let c2rust_fresh0 = count;
        count += 1;
        populate_location(stmt, locations.offset(c2rust_fresh0 as isize));
    }
    if rc != SQLITE_ROW && rc != SQLITE_DONE {
        prepare_error(db, b"list\0".as_ptr() as *const ::core::ffi::c_char);
        count = -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return count;
}
unsafe extern "C" fn row_exists_locked(
    mut db: *mut sqlite3,
    mut sql: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) -> bool {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        return r#false != 0;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        value,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    let mut exists: bool = sqlite3_step(stmt) == SQLITE_ROW;
    sqlite3_finalize(stmt);
    return exists;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_location_create(
    mut location: *mut camera_location_t,
) -> db_location_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || location.is_null()
        || !has_non_whitespace(&raw mut (*location).name as *mut ::core::ffi::c_char)
        || (*location).parent_uuid[0usize] as ::core::ffi::c_int != 0
            && !valid_uuid_string(&raw mut (*location).parent_uuid as *mut ::core::ffi::c_char)
    {
        return db_location_result_t::DB_LOCATION_INVALID;
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO camera_locations (uuid, parent_uuid, name, type, sort_order, metadata_json) VALUES (lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)), 2) || '-' || substr('89ab', (abs(random()) % 4) + 1, 1) || substr(hex(randomblob(2)), 2) || '-' || hex(randomblob(6))), ?, ?, ?, ?, ?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    if (*location).parent_uuid[0usize] as ::core::ffi::c_int != 0
        && !row_exists_locked(
            db,
            b"SELECT 1 FROM camera_locations WHERE uuid = ?;\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*location).parent_uuid as *mut ::core::ffi::c_char,
        )
    {
        pthread_mutex_unlock(mutex);
        return db_location_result_t::DB_LOCATION_NOT_FOUND;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        let mut result: db_location_result_t =
            prepare_error(db, b"create\0".as_ptr() as *const ::core::ffi::c_char);
        pthread_mutex_unlock(mutex);
        return result;
    }
    if (*location).parent_uuid[0usize] != 0 {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw mut (*location).parent_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    } else {
        sqlite3_bind_null(stmt, 1 as ::core::ffi::c_int);
    }
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        &raw mut (*location).name as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        3 as ::core::ffi::c_int,
        if (*location).r#type[0usize] as ::core::ffi::c_int != 0 {
            &raw mut (*location).r#type as *mut ::core::ffi::c_char
        } else {
            b"area\0".as_ptr() as *const ::core::ffi::c_char
        },
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(stmt, 4 as ::core::ffi::c_int, (*location).sort_order);
    sqlite3_bind_text(
        stmt,
        5 as ::core::ffi::c_int,
        if (*location).metadata_json[0usize] as ::core::ffi::c_int != 0 {
            &raw mut (*location).metadata_json as *mut ::core::ffi::c_char
        } else {
            b"{}\0".as_ptr() as *const ::core::ffi::c_char
        },
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    if rc != SQLITE_DONE {
        let mut result_0: db_location_result_t = db_location_result_t(if rc == SQLITE_CONSTRAINT {
            db_location_result_t::DB_LOCATION_CONFLICT.0
        } else {
            prepare_error(db, b"create\0".as_ptr() as *const ::core::ffi::c_char).0
        });
        pthread_mutex_unlock(mutex);
        return result_0;
    }
    let mut row_id: [::core::ffi::c_char; 32] = [0; 32];
    snprintf(
        &raw mut row_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
        sqlite3_last_insert_rowid(db),
    );
    let mut result_1: db_location_result_t = get_locked(
        db,
        b"l.id = ?\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut row_id as *mut ::core::ffi::c_char,
        location,
    );
    pthread_mutex_unlock(mutex);
    return result_1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_location_update(
    mut location: *mut camera_location_t,
) -> db_location_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || location.is_null()
        || !valid_uuid_string(&raw mut (*location).uuid as *mut ::core::ffi::c_char)
        || !has_non_whitespace(&raw mut (*location).name as *mut ::core::ffi::c_char)
        || (*location).parent_uuid[0usize] as ::core::ffi::c_int != 0
            && !valid_uuid_string(&raw mut (*location).parent_uuid as *mut ::core::ffi::c_char)
    {
        return db_location_result_t::DB_LOCATION_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut existing: camera_location_t = camera_location_t {
        uuid: [0; 37],
        parent_uuid: [0; 37],
        name: [0; 128],
        r#type: [0; 64],
        metadata_json: [0; 512],
        sort_order: 0,
        is_system: 0,
        direct_child_count: 0,
        direct_camera_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_location_result_t = get_locked(
        db,
        b"l.uuid = ?\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*location).uuid as *mut ::core::ffi::c_char,
        &raw mut existing,
    );
    if result.0 != db_location_result_t::DB_LOCATION_OK.0 {
        pthread_mutex_unlock(mutex);
        return result;
    }
    if existing.is_system != 0 {
        pthread_mutex_unlock(mutex);
        return db_location_result_t::DB_LOCATION_INVALID;
    }
    if (*location).parent_uuid[0usize] != 0 {
        if !row_exists_locked(
            db,
            b"SELECT 1 FROM camera_locations WHERE uuid = ?;\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*location).parent_uuid as *mut ::core::ffi::c_char,
        ) {
            pthread_mutex_unlock(mutex);
            return db_location_result_t::DB_LOCATION_NOT_FOUND;
        }
        let mut cycle_sql: *const ::core::ffi::c_char = b"WITH RECURSIVE ancestors(uuid, parent_uuid) AS (SELECT uuid, parent_uuid FROM camera_locations WHERE uuid = ? UNION ALL SELECT l.uuid, l.parent_uuid FROM camera_locations l JOIN ancestors a ON l.uuid = a.parent_uuid) SELECT 1 FROM ancestors WHERE uuid = ? LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut cycle_stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
            db,
            cycle_sql,
            -1 as ::core::ffi::c_int,
            &raw mut cycle_stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc != SQLITE_OK {
            result = prepare_error(db, b"validate\0".as_ptr() as *const ::core::ffi::c_char);
            pthread_mutex_unlock(mutex);
            return result;
        }
        sqlite3_bind_text(
            cycle_stmt,
            1 as ::core::ffi::c_int,
            &raw mut (*location).parent_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            cycle_stmt,
            2 as ::core::ffi::c_int,
            &raw mut (*location).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        let mut creates_cycle: bool = sqlite3_step(cycle_stmt) == SQLITE_ROW;
        sqlite3_finalize(cycle_stmt);
        if creates_cycle {
            pthread_mutex_unlock(mutex);
            return db_location_result_t::DB_LOCATION_CONFLICT;
        }
    }
    let mut update_sql: *const ::core::ffi::c_char = b"UPDATE camera_locations SET parent_uuid = ?, name = ?, type = ?, sort_order = ?, metadata_json = ?, updated_at = strftime('%s', 'now') WHERE uuid = ? AND is_system = 0;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc_0: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        update_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc_0 != SQLITE_OK {
        result = prepare_error(db, b"update\0".as_ptr() as *const ::core::ffi::c_char);
        pthread_mutex_unlock(mutex);
        return result;
    }
    if (*location).parent_uuid[0usize] != 0 {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw mut (*location).parent_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
    } else {
        sqlite3_bind_null(stmt, 1 as ::core::ffi::c_int);
    }
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        &raw mut (*location).name as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        3 as ::core::ffi::c_int,
        if (*location).r#type[0usize] as ::core::ffi::c_int != 0 {
            &raw mut (*location).r#type as *mut ::core::ffi::c_char
        } else {
            b"area\0".as_ptr() as *const ::core::ffi::c_char
        },
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_int(stmt, 4 as ::core::ffi::c_int, (*location).sort_order);
    sqlite3_bind_text(
        stmt,
        5 as ::core::ffi::c_int,
        if (*location).metadata_json[0usize] as ::core::ffi::c_int != 0 {
            &raw mut (*location).metadata_json as *mut ::core::ffi::c_char
        } else {
            b"{}\0".as_ptr() as *const ::core::ffi::c_char
        },
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_bind_text(
        stmt,
        6 as ::core::ffi::c_int,
        &raw mut (*location).uuid as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    rc_0 = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    if rc_0 != SQLITE_DONE {
        result = db_location_result_t(if rc_0 == SQLITE_CONSTRAINT {
            db_location_result_t::DB_LOCATION_CONFLICT.0
        } else {
            prepare_error(db, b"update\0".as_ptr() as *const ::core::ffi::c_char).0
        });
        pthread_mutex_unlock(mutex);
        return result;
    }
    result = get_locked(
        db,
        b"l.uuid = ?\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*location).uuid as *mut ::core::ffi::c_char,
        location,
    );
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_location_delete(
    mut uuid: *const ::core::ffi::c_char,
) -> db_location_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || !valid_uuid_string(uuid) {
        return db_location_result_t::DB_LOCATION_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut existing: camera_location_t = camera_location_t {
        uuid: [0; 37],
        parent_uuid: [0; 37],
        name: [0; 128],
        r#type: [0; 64],
        metadata_json: [0; 512],
        sort_order: 0,
        is_system: 0,
        direct_child_count: 0,
        direct_camera_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_location_result_t = get_locked(
        db,
        b"l.uuid = ?\0".as_ptr() as *const ::core::ffi::c_char,
        uuid,
        &raw mut existing,
    );
    if result.0 != db_location_result_t::DB_LOCATION_OK.0 {
        pthread_mutex_unlock(mutex);
        return result;
    }
    if existing.is_system != 0 {
        pthread_mutex_unlock(mutex);
        return db_location_result_t::DB_LOCATION_INVALID;
    }
    if existing.direct_child_count > 0 as ::core::ffi::c_int
        || existing.direct_camera_count > 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(mutex);
        return db_location_result_t::DB_LOCATION_CONFLICT;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM camera_locations WHERE uuid = ? AND is_system = 0;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            uuid,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        rc = sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if rc != SQLITE_DONE {
        result = db_location_result_t(if rc == SQLITE_CONSTRAINT {
            db_location_result_t::DB_LOCATION_CONFLICT.0
        } else {
            prepare_error(db, b"delete\0".as_ptr() as *const ::core::ffi::c_char).0
        });
    } else {
        result = db_location_result_t::DB_LOCATION_OK;
    }
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_location_assign_camera(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut location_uuid: *const ::core::ffi::c_char,
) -> db_location_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || !valid_uuid_string(camera_uuid) || !valid_uuid_string(location_uuid) {
        return db_location_result_t::DB_LOCATION_INVALID;
    }
    pthread_mutex_lock(mutex);
    if !row_exists_locked(
        db,
        b"SELECT 1 FROM camera_locations WHERE uuid = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        location_uuid,
    ) || !row_exists_locked(
        db,
        b"SELECT 1 FROM streams WHERE camera_uuid = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        camera_uuid,
    ) {
        pthread_mutex_unlock(mutex);
        return db_location_result_t::DB_LOCATION_NOT_FOUND;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"UPDATE streams SET location_uuid = ? WHERE camera_uuid = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            location_uuid,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            camera_uuid,
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        rc = sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    let mut result: db_location_result_t = db_location_result_t(if rc == SQLITE_DONE {
        db_location_result_t::DB_LOCATION_OK.0
    } else {
        prepare_error(db, b"assign\0".as_ptr() as *const ::core::ffi::c_char).0
    });
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_location_get_for_camera(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut location: *mut camera_location_t,
) -> db_location_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || location.is_null() || !valid_uuid_string(camera_uuid) {
        return db_location_result_t::DB_LOCATION_INVALID;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT l.uuid, l.parent_uuid, l.name, l.type, l.sort_order, l.metadata_json, l.is_system, l.created_at, l.updated_at, (SELECT count(*) FROM camera_locations c WHERE c.parent_uuid = l.uuid), (SELECT count(*) FROM streams s WHERE s.location_uuid = l.uuid) FROM camera_locations l JOIN streams s ON s.location_uuid = l.uuid WHERE s.camera_uuid = ? LIMIT 1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        let mut result: db_location_result_t =
            prepare_error(db, b"read camera\0".as_ptr() as *const ::core::ffi::c_char);
        pthread_mutex_unlock(mutex);
        return result;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        camera_uuid,
        -1 as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    let mut result_0: db_location_result_t = db_location_result_t::DB_LOCATION_NOT_FOUND;
    rc = sqlite3_step(stmt);
    if rc == SQLITE_ROW {
        populate_location(stmt, location);
        result_0 = db_location_result_t::DB_LOCATION_OK;
    } else if rc != SQLITE_DONE {
        result_0 = prepare_error(db, b"read camera\0".as_ptr() as *const ::core::ffi::c_char);
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return result_0;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
