use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
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
    unsafe fn sqlite3_get_autocommit(_: *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
    unsafe fn copy_trimmed_value(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
        input: *const ::core::ffi::c_char,
        input_size: size_t,
    ) -> size_t;
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strtok_r(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
        __save_ptr: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camera_tag_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub label: [::core::ffi::c_char; 256],
    pub color: [::core::ffi::c_char; 16],
    pub description: [::core::ffi::c_char; 512],
    pub camera_count: ::core::ffi::c_int,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_camera_tag_result_t(pub ::core::ffi::c_int);
impl db_camera_tag_result_t {
    pub const DB_CAMERA_TAG_OK: Self = Self(0);
    pub const DB_CAMERA_TAG_NOT_FOUND: Self = Self(-1);
    pub const DB_CAMERA_TAG_CONFLICT: Self = Self(-2);
    pub const DB_CAMERA_TAG_INVALID: Self = Self(-3);
    pub const DB_CAMERA_TAG_ERROR: Self = Self(-4);
    pub const DB_CAMERA_TAG_LIMIT: Self = Self(-5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camera_uuid_list_t {
    pub items: *mut [::core::ffi::c_char; 37],
    pub count: ::core::ffi::c_int,
    pub capacity: ::core::ffi::c_int,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const CAMERA_TAG_MAX_ASSIGNMENTS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
unsafe extern "C" fn valid_uuid_string(mut uuid: *const ::core::ffi::c_char) -> bool {
    return !uuid.is_null()
        && strlen(uuid) == (CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int) as size_t;
}
unsafe extern "C" fn normalize_label(
    mut input: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) -> bool {
    if input.is_null()
        || copy_trimmed_value(output, output_size, input, 0 as size_t) == 0 as size_t
        || !strchr(output, ',' as ::core::ffi::c_int).is_null()
    {
        return r#false != 0;
    }
    let mut p: *const ::core::ffi::c_uchar = output as *const ::core::ffi::c_uchar;
    while *p != 0 {
        if *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_IScntrl.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            return r#false != 0;
        }
        p = p.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn transaction_begin(
    mut db: *mut sqlite3,
    mut owns_transaction: *mut bool,
) -> bool {
    *owns_transaction = sqlite3_get_autocommit(db) != 0 as ::core::ffi::c_int;
    if !*owns_transaction {
        return r#true != 0;
    }
    return sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK;
}
unsafe extern "C" fn transaction_finish(
    mut db: *mut sqlite3,
    mut owns_transaction: bool,
    mut success: bool,
) -> bool {
    if !owns_transaction {
        return success;
    }
    let mut sql: *const ::core::ffi::c_char = if success as ::core::ffi::c_int != 0 {
        b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char
    };
    return sqlite3_exec(
        db,
        sql,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK
        && success as ::core::ffi::c_int != 0;
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
unsafe extern "C" fn populate_tag(mut stmt: *mut sqlite3_stmt, mut tag: *mut camera_tag_t) {
    memset(
        tag as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<camera_tag_t>(),
    );
    copy_column(
        &raw mut (*tag).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        stmt,
        0 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*tag).label as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        stmt,
        1 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*tag).color as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        stmt,
        2 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*tag).description as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        stmt,
        3 as ::core::ffi::c_int,
    );
    (*tag).created_at = sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as int64_t;
    (*tag).updated_at = sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as int64_t;
    (*tag).camera_count = sqlite3_column_int(stmt, 6 as ::core::ffi::c_int);
}
unsafe extern "C" fn get_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut tag: *mut camera_tag_t,
) -> db_camera_tag_result_t {
    let mut sql: *const ::core::ffi::c_char = b"SELECT t.uuid, t.label, t.color, t.description, t.created_at, t.updated_at, (SELECT count(*) FROM camera_tag_assignments a WHERE a.tag_uuid = t.uuid) FROM camera_tags t WHERE t.uuid = ? LIMIT 1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return db_camera_tag_result_t::DB_CAMERA_TAG_ERROR;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        uuid,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    let mut result: db_camera_tag_result_t = db_camera_tag_result_t::DB_CAMERA_TAG_NOT_FOUND;
    rc = sqlite3_step(stmt);
    if rc == SQLITE_ROW {
        populate_tag(stmt, tag);
        result = db_camera_tag_result_t::DB_CAMERA_TAG_OK;
    } else if rc != SQLITE_DONE {
        result = db_camera_tag_result_t::DB_CAMERA_TAG_ERROR;
    }
    sqlite3_finalize(stmt);
    return result;
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
unsafe extern "C" fn ensure_tag_for_label_locked(
    mut db: *mut sqlite3,
    mut label: *const ::core::ffi::c_char,
    mut tag_uuid: *mut ::core::ffi::c_char,
    mut tag_uuid_size: size_t,
) -> ::core::ffi::c_int {
    let mut insert_sql: *const ::core::ffi::c_char = b"INSERT OR IGNORE INTO camera_tags (uuid, label) VALUES (lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)), 2) || '-' || substr('89ab', (abs(random()) % 4) + 1, 1) || substr(hex(randomblob(2)), 2) || '-' || hex(randomblob(6))), ?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        insert_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        label,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    if rc != SQLITE_DONE {
        return -1 as ::core::ffi::c_int;
    }
    rc = sqlite3_prepare_v2(
        db,
        b"SELECT uuid FROM camera_tags WHERE label = ? COLLATE NOCASE LIMIT 1;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        label,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    rc = sqlite3_step(stmt);
    if rc == SQLITE_ROW {
        let mut uuid: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(tag_uuid, uuid, tag_uuid_size, 0 as size_t);
    }
    sqlite3_finalize(stmt);
    return if rc == SQLITE_ROW {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn rebuild_legacy_for_camera_locked(
    mut db: *mut sqlite3,
    mut camera_uuid: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut select_sql: *const ::core::ffi::c_char = b"SELECT t.label FROM camera_tag_assignments a JOIN camera_tags t ON t.uuid = a.tag_uuid WHERE a.camera_uuid = ? ORDER BY t.label COLLATE NOCASE;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        select_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        camera_uuid,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    let mut legacy_tags: [::core::ffi::c_char; 256] = [
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
    let mut used: size_t = 0 as size_t;
    loop {
        rc = sqlite3_step(stmt);
        if rc != SQLITE_ROW {
            break;
        }
        let mut label: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut label_length: size_t = if !label.is_null() {
            strlen(label)
        } else {
            0 as size_t
        };
        let mut required: size_t = label_length.wrapping_add(
            (if used > 0 as size_t {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as size_t,
        );
        if label.is_null()
            || label_length == 0 as size_t
            || used.wrapping_add(required) >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
        {
            sqlite3_finalize(stmt);
            return -2 as ::core::ffi::c_int;
        }
        if used > 0 as size_t {
            let c2rust_fresh1 = used;
            used = used.wrapping_add(1);
            legacy_tags[c2rust_fresh1] = ',' as ::core::ffi::c_char;
        }
        memcpy(
            (&raw mut legacy_tags as *mut ::core::ffi::c_char).offset(used as isize)
                as *mut ::core::ffi::c_void,
            label as *const ::core::ffi::c_void,
            label_length,
        );
        used = used.wrapping_add(label_length);
        legacy_tags[used] = '\0' as ::core::ffi::c_char;
    }
    sqlite3_finalize(stmt);
    if rc != SQLITE_DONE {
        return -1 as ::core::ffi::c_int;
    }
    rc = sqlite3_prepare_v2(
        db,
        b"UPDATE streams SET tags = ? WHERE camera_uuid = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        &raw mut legacy_tags as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        camera_uuid,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn camera_uuid_list_free(mut cameras: *mut camera_uuid_list_t) {
    if cameras.is_null() {
        return;
    }
    free((*cameras).items as *mut ::core::ffi::c_void);
    memset(
        cameras as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<camera_uuid_list_t>(),
    );
}
unsafe extern "C" fn collect_cameras_for_tag_locked(
    mut db: *mut sqlite3,
    mut tag_uuid: *const ::core::ffi::c_char,
    mut cameras: *mut camera_uuid_list_t,
) -> ::core::ffi::c_int {
    memset(
        cameras as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<camera_uuid_list_t>(),
    );
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT camera_uuid FROM camera_tag_assignments WHERE tag_uuid = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        tag_uuid,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    loop {
        rc = sqlite3_step(stmt);
        if rc != SQLITE_ROW {
            break;
        }
        if (*cameras).count == (*cameras).capacity {
            let mut new_capacity: ::core::ffi::c_int =
                if (*cameras).capacity == 0 as ::core::ffi::c_int {
                    16 as ::core::ffi::c_int
                } else {
                    (*cameras).capacity * 2 as ::core::ffi::c_int
                };
            let mut resized: *mut ::core::ffi::c_void = realloc(
                (*cameras).items as *mut ::core::ffi::c_void,
                (new_capacity as size_t)
                    .wrapping_mul(::core::mem::size_of::<[::core::ffi::c_char; 37]>()),
            );
            if resized.is_null() {
                sqlite3_finalize(stmt);
                camera_uuid_list_free(cameras);
                return -1 as ::core::ffi::c_int;
            }
            (*cameras).items = resized as *mut [::core::ffi::c_char; 37];
            (*cameras).capacity = new_capacity;
        }
        safe_strcpy(
            &raw mut *(*cameras).items.offset((*cameras).count as isize)
                as *mut ::core::ffi::c_char,
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
            CAMERA_UUID_STRING_SIZE as size_t,
            0 as size_t,
        );
        (*cameras).count += 1;
    }
    sqlite3_finalize(stmt);
    if rc != SQLITE_DONE {
        camera_uuid_list_free(cameras);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn rebuild_legacy_for_cameras_locked(
    mut db: *mut sqlite3,
    mut cameras: *const camera_uuid_list_t,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*cameras).count {
        let mut rebuild_rc: ::core::ffi::c_int = rebuild_legacy_for_camera_locked(
            db,
            &raw mut *(*cameras).items.offset(i as isize) as *mut ::core::ffi::c_char,
        );
        if rebuild_rc != 0 as ::core::ffi::c_int {
            return rebuild_rc;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn sync_legacy_locked(
    mut db: *mut sqlite3,
    mut camera_uuid: *const ::core::ffi::c_char,
    mut legacy_tags: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM camera_tag_assignments WHERE camera_uuid = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        camera_uuid,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    if rc != SQLITE_DONE {
        return -1 as ::core::ffi::c_int;
    }
    if legacy_tags.is_null()
        || *legacy_tags.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut tags_copy: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut tags_copy as *mut ::core::ffi::c_char,
        legacy_tags,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut assignment_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut token: *mut ::core::ffi::c_char = strtok_r(
        &raw mut tags_copy as *mut ::core::ffi::c_char,
        b",\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut saveptr,
    );
    while !token.is_null() {
        let mut label: [::core::ffi::c_char; 256] = [0; 256];
        if normalize_label(
            token,
            &raw mut label as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        ) {
            let mut tag_uuid: [::core::ffi::c_char; 37] = [0; 37];
            if ensure_tag_for_label_locked(
                db,
                &raw mut label as *mut ::core::ffi::c_char,
                &raw mut tag_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            ) != 0 as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
            rc = sqlite3_prepare_v2(
                db,
                b"INSERT OR IGNORE INTO camera_tag_assignments (camera_uuid, tag_uuid) VALUES (?, ?);\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                &raw mut stmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            if rc != SQLITE_OK {
                return -1 as ::core::ffi::c_int;
            }
            sqlite3_bind_text(
                stmt,
                1 as ::core::ffi::c_int,
                camera_uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                stmt,
                2 as ::core::ffi::c_int,
                &raw mut tag_uuid as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            rc = sqlite3_step(stmt);
            sqlite3_finalize(stmt);
            if rc != SQLITE_DONE {
                return -1 as ::core::ffi::c_int;
            }
            assignment_count += sqlite3_changes(db);
            if assignment_count > CAMERA_TAG_MAX_ASSIGNMENTS {
                return -2 as ::core::ffi::c_int;
            }
        }
        token = strtok_r(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b",\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_tags_sync_legacy_by_name_locked(
    mut db: *mut sqlite3,
    mut stream_name: *const ::core::ffi::c_char,
    mut legacy_tags: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if db.is_null() || stream_name.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT camera_uuid FROM streams WHERE name = ? LIMIT 1;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        stream_name,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    rc = sqlite3_step(stmt);
    let mut camera_uuid: [::core::ffi::c_char; 37] = [
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
    if rc == SQLITE_ROW {
        let mut value: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut camera_uuid as *mut ::core::ffi::c_char,
            value,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
    }
    sqlite3_finalize(stmt);
    if rc != SQLITE_ROW || !valid_uuid_string(&raw mut camera_uuid as *mut ::core::ffi::c_char) {
        return -1 as ::core::ffi::c_int;
    }
    let mut owns_transaction: bool = r#false != 0;
    if !transaction_begin(db, &raw mut owns_transaction) {
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = sync_legacy_locked(
        db,
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        legacy_tags,
    );
    return if transaction_finish(db, owns_transaction, result == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int
        != 0
    {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_tags_backfill_legacy() -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut owns_transaction: bool = r#false != 0;
    if !transaction_begin(db, &raw mut owns_transaction) {
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT camera_uuid, tags FROM streams;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut result: ::core::ffi::c_int = if rc == SQLITE_OK {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
    while result == 0 as ::core::ffi::c_int && {
        rc = sqlite3_step(stmt);
        rc == SQLITE_ROW
    } {
        let mut camera_uuid: [::core::ffi::c_char; 37] = [
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
        let mut legacy_tags: [::core::ffi::c_char; 256] = [0; 256];
        let mut uuid_value: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut camera_uuid as *mut ::core::ffi::c_char,
            if !uuid_value.is_null() {
                uuid_value
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        let mut legacy_value: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        safe_strcpy(
            &raw mut legacy_tags as *mut ::core::ffi::c_char,
            if !legacy_value.is_null() {
                legacy_value
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        if !valid_uuid_string(&raw mut camera_uuid as *mut ::core::ffi::c_char) {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Skipping tag backfill for a stream without a camera UUID\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            result = -1 as ::core::ffi::c_int;
            break;
        } else if sync_legacy_locked(
            db,
            &raw mut camera_uuid as *mut ::core::ffi::c_char,
            &raw mut legacy_tags as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            result = -1 as ::core::ffi::c_int;
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if rc != SQLITE_DONE && result == 0 as ::core::ffi::c_int {
        result = -1 as ::core::ffi::c_int;
    }
    let mut success: bool =
        transaction_finish(db, owns_transaction, result == 0 as ::core::ffi::c_int);
    pthread_mutex_unlock(mutex);
    if !success {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to backfill normalized camera tags: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn backfill_marker_locked(
    mut db: *mut sqlite3,
    mut completed: *mut bool,
) -> ::core::ffi::c_int {
    *completed = r#false != 0;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT value FROM system_settings WHERE key='camera_tags_backfill_completed' LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
        }
        return -1 as ::core::ffi::c_int;
    }
    if sqlite3_step(stmt) == SQLITE_ROW {
        let mut value: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        *completed = !value.is_null()
            && strcmp(value, b"1\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn mark_backfill_complete_locked(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    return if sqlite3_exec(
        db,
        b"INSERT INTO system_settings(key,value,updated_at) VALUES('camera_tags_backfill_completed','1',strftime('%s','now')) ON CONFLICT(key) DO UPDATE SET value=excluded.value,updated_at=excluded.updated_at;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_tags_backfill_legacy_once() -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut completed: bool = r#false != 0;
    let mut rc: ::core::ffi::c_int = backfill_marker_locked(db, &raw mut completed);
    pthread_mutex_unlock(mutex);
    if rc != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if completed {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"Legacy camera tag backfill already completed; skipping\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if db_camera_tags_backfill_legacy() != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    rc = mark_backfill_complete_locked(db);
    pthread_mutex_unlock(mutex);
    if rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to record camera tag backfill completion\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_tag_count() -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        b"SELECT count(*) FROM camera_tags;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
        && sqlite3_step(stmt) == SQLITE_ROW
    {
        count = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_tag_list(
    mut tags: *mut camera_tag_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || tags.is_null() || max_count <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT t.uuid, t.label, t.color, t.description, t.created_at, t.updated_at, (SELECT count(*) FROM camera_tag_assignments a WHERE a.tag_uuid = t.uuid) FROM camera_tags t ORDER BY t.label COLLATE NOCASE;\0"
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
        populate_tag(stmt, tags.offset(c2rust_fresh0 as isize));
    }
    if rc != SQLITE_ROW && rc != SQLITE_DONE {
        count = -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_tag_get(
    mut uuid: *const ::core::ffi::c_char,
    mut tag: *mut camera_tag_t,
) -> db_camera_tag_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || tag.is_null() || !valid_uuid_string(uuid) {
        return db_camera_tag_result_t::DB_CAMERA_TAG_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut result: db_camera_tag_result_t = get_locked(db, uuid, tag);
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_tag_create(
    mut tag: *mut camera_tag_t,
) -> db_camera_tag_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || tag.is_null() {
        return db_camera_tag_result_t::DB_CAMERA_TAG_INVALID;
    }
    let mut label: [::core::ffi::c_char; 256] = [0; 256];
    if !normalize_label(
        &raw mut (*tag).label as *mut ::core::ffi::c_char,
        &raw mut label as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) {
        return db_camera_tag_result_t::DB_CAMERA_TAG_INVALID;
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO camera_tags (uuid, label, color, description) VALUES (lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)), 2) || '-' || substr('89ab', (abs(random()) % 4) + 1, 1) || substr(hex(randomblob(2)), 2) || '-' || hex(randomblob(6))), ?, ?, ?);\0"
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
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw mut label as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw mut (*tag).color as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            &raw mut (*tag).description as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if rc != SQLITE_DONE {
        let mut result: db_camera_tag_result_t =
            db_camera_tag_result_t(if rc == SQLITE_CONSTRAINT {
                db_camera_tag_result_t::DB_CAMERA_TAG_CONFLICT.0
            } else {
                db_camera_tag_result_t::DB_CAMERA_TAG_ERROR.0
            });
        pthread_mutex_unlock(mutex);
        return result;
    }
    let mut row_id: [::core::ffi::c_char; 32] = [0; 32];
    snprintf(
        &raw mut row_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
        sqlite3_last_insert_rowid(db),
    );
    rc = sqlite3_prepare_v2(
        db,
        b"SELECT uuid FROM camera_tags WHERE id = ? LIMIT 1;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw mut row_id as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
        if rc == SQLITE_ROW {
            let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
            safe_strcpy(
                &raw mut uuid as *mut ::core::ffi::c_char,
                sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
            sqlite3_finalize(stmt);
            stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            let mut result_0: db_camera_tag_result_t =
                get_locked(db, &raw mut uuid as *mut ::core::ffi::c_char, tag);
            pthread_mutex_unlock(mutex);
            return result_0;
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return db_camera_tag_result_t::DB_CAMERA_TAG_ERROR;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_tag_update(
    mut tag: *mut camera_tag_t,
) -> db_camera_tag_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || tag.is_null()
        || !valid_uuid_string(&raw mut (*tag).uuid as *mut ::core::ffi::c_char)
    {
        return db_camera_tag_result_t::DB_CAMERA_TAG_INVALID;
    }
    let mut label: [::core::ffi::c_char; 256] = [0; 256];
    if !normalize_label(
        &raw mut (*tag).label as *mut ::core::ffi::c_char,
        &raw mut label as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) {
        return db_camera_tag_result_t::DB_CAMERA_TAG_INVALID;
    }
    pthread_mutex_lock(mutex);
    if !row_exists_locked(
        db,
        b"SELECT 1 FROM camera_tags WHERE uuid = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*tag).uuid as *mut ::core::ffi::c_char,
    ) {
        pthread_mutex_unlock(mutex);
        return db_camera_tag_result_t::DB_CAMERA_TAG_NOT_FOUND;
    }
    let mut affected_cameras: camera_uuid_list_t = camera_uuid_list_t {
        items: ::core::ptr::null_mut::<[::core::ffi::c_char; 37]>(),
        count: 0,
        capacity: 0,
    };
    if collect_cameras_for_tag_locked(
        db,
        &raw mut (*tag).uuid as *mut ::core::ffi::c_char,
        &raw mut affected_cameras,
    ) != 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(mutex);
        return db_camera_tag_result_t::DB_CAMERA_TAG_ERROR;
    }
    let mut owns_transaction: bool = r#false != 0;
    if !transaction_begin(db, &raw mut owns_transaction) {
        camera_uuid_list_free(&raw mut affected_cameras);
        pthread_mutex_unlock(mutex);
        return db_camera_tag_result_t::DB_CAMERA_TAG_ERROR;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"UPDATE camera_tags SET label = ?, color = ?, description = ?, updated_at = strftime('%s', 'now') WHERE uuid = ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw mut label as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw mut (*tag).color as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            &raw mut (*tag).description as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            4 as ::core::ffi::c_int,
            &raw mut (*tag).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    let mut rebuild_rc: ::core::ffi::c_int = if rc == SQLITE_DONE {
        rebuild_legacy_for_cameras_locked(db, &raw mut affected_cameras)
    } else {
        -1 as ::core::ffi::c_int
    };
    let mut success: bool = rc == SQLITE_DONE && rebuild_rc == 0 as ::core::ffi::c_int;
    success = transaction_finish(db, owns_transaction, success);
    camera_uuid_list_free(&raw mut affected_cameras);
    if !success {
        let mut result: db_camera_tag_result_t =
            db_camera_tag_result_t(if rc == SQLITE_CONSTRAINT {
                db_camera_tag_result_t::DB_CAMERA_TAG_CONFLICT.0
            } else if rebuild_rc == -2 as ::core::ffi::c_int {
                db_camera_tag_result_t::DB_CAMERA_TAG_LIMIT.0
            } else {
                db_camera_tag_result_t::DB_CAMERA_TAG_ERROR.0
            });
        pthread_mutex_unlock(mutex);
        return result;
    }
    let mut result_0: db_camera_tag_result_t =
        get_locked(db, &raw mut (*tag).uuid as *mut ::core::ffi::c_char, tag);
    pthread_mutex_unlock(mutex);
    return result_0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_tag_delete(
    mut uuid: *const ::core::ffi::c_char,
) -> db_camera_tag_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || !valid_uuid_string(uuid) {
        return db_camera_tag_result_t::DB_CAMERA_TAG_INVALID;
    }
    pthread_mutex_lock(mutex);
    if !row_exists_locked(
        db,
        b"SELECT 1 FROM camera_tags WHERE uuid = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        uuid,
    ) {
        pthread_mutex_unlock(mutex);
        return db_camera_tag_result_t::DB_CAMERA_TAG_NOT_FOUND;
    }
    let mut affected_cameras: camera_uuid_list_t = camera_uuid_list_t {
        items: ::core::ptr::null_mut::<[::core::ffi::c_char; 37]>(),
        count: 0,
        capacity: 0,
    };
    if collect_cameras_for_tag_locked(db, uuid, &raw mut affected_cameras)
        != 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(mutex);
        return db_camera_tag_result_t::DB_CAMERA_TAG_ERROR;
    }
    let mut owns_transaction: bool = r#false != 0;
    if !transaction_begin(db, &raw mut owns_transaction) {
        camera_uuid_list_free(&raw mut affected_cameras);
        pthread_mutex_unlock(mutex);
        return db_camera_tag_result_t::DB_CAMERA_TAG_ERROR;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM camera_tags WHERE uuid = ?;\0".as_ptr() as *const ::core::ffi::c_char,
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
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    let mut success: bool = rc == SQLITE_DONE
        && rebuild_legacy_for_cameras_locked(db, &raw mut affected_cameras)
            == 0 as ::core::ffi::c_int;
    success = transaction_finish(db, owns_transaction, success);
    camera_uuid_list_free(&raw mut affected_cameras);
    pthread_mutex_unlock(mutex);
    return db_camera_tag_result_t(if success as ::core::ffi::c_int != 0 {
        db_camera_tag_result_t::DB_CAMERA_TAG_OK.0
    } else {
        db_camera_tag_result_t::DB_CAMERA_TAG_ERROR.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_tag_merge(
    mut source_uuid: *const ::core::ffi::c_char,
    mut target_uuid: *const ::core::ffi::c_char,
) -> db_camera_tag_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || !valid_uuid_string(source_uuid)
        || !valid_uuid_string(target_uuid)
        || strcmp(source_uuid, target_uuid) == 0 as ::core::ffi::c_int
    {
        return db_camera_tag_result_t::DB_CAMERA_TAG_INVALID;
    }
    pthread_mutex_lock(mutex);
    if !row_exists_locked(
        db,
        b"SELECT 1 FROM camera_tags WHERE uuid = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        source_uuid,
    ) || !row_exists_locked(
        db,
        b"SELECT 1 FROM camera_tags WHERE uuid = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        target_uuid,
    ) {
        pthread_mutex_unlock(mutex);
        return db_camera_tag_result_t::DB_CAMERA_TAG_NOT_FOUND;
    }
    let mut affected_cameras: camera_uuid_list_t = camera_uuid_list_t {
        items: ::core::ptr::null_mut::<[::core::ffi::c_char; 37]>(),
        count: 0,
        capacity: 0,
    };
    if collect_cameras_for_tag_locked(db, source_uuid, &raw mut affected_cameras)
        != 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(mutex);
        return db_camera_tag_result_t::DB_CAMERA_TAG_ERROR;
    }
    let mut owns_transaction: bool = r#false != 0;
    if !transaction_begin(db, &raw mut owns_transaction) {
        camera_uuid_list_free(&raw mut affected_cameras);
        pthread_mutex_unlock(mutex);
        return db_camera_tag_result_t::DB_CAMERA_TAG_ERROR;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"INSERT OR IGNORE INTO camera_tag_assignments (camera_uuid, tag_uuid) SELECT camera_uuid, ? FROM camera_tag_assignments WHERE tag_uuid = ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            target_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            source_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    if rc == SQLITE_DONE {
        rc = sqlite3_prepare_v2(
            db,
            b"DELETE FROM camera_tags WHERE uuid = ?;\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_text(
                stmt,
                1 as ::core::ffi::c_int,
                source_uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            rc = sqlite3_step(stmt);
        }
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
        }
    }
    let mut success: bool = rc == SQLITE_DONE
        && rebuild_legacy_for_cameras_locked(db, &raw mut affected_cameras)
            == 0 as ::core::ffi::c_int;
    success = transaction_finish(db, owns_transaction, success);
    camera_uuid_list_free(&raw mut affected_cameras);
    pthread_mutex_unlock(mutex);
    return db_camera_tag_result_t(if success as ::core::ffi::c_int != 0 {
        db_camera_tag_result_t::DB_CAMERA_TAG_OK.0
    } else {
        db_camera_tag_result_t::DB_CAMERA_TAG_ERROR.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_tag_list_for_camera(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut tags: *mut camera_tag_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || tags.is_null()
        || max_count <= 0 as ::core::ffi::c_int
        || !valid_uuid_string(camera_uuid)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT t.uuid, t.label, t.color, t.description, t.created_at, t.updated_at, (SELECT count(*) FROM camera_tag_assignments a WHERE a.tag_uuid = t.uuid) FROM camera_tags t JOIN camera_tag_assignments assigned ON assigned.tag_uuid = t.uuid WHERE assigned.camera_uuid = ? ORDER BY t.label COLLATE NOCASE;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    if !row_exists_locked(
        db,
        b"SELECT 1 FROM streams WHERE camera_uuid = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        camera_uuid,
    ) {
        pthread_mutex_unlock(mutex);
        return -2 as ::core::ffi::c_int;
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
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        camera_uuid,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while count < max_count && {
        rc = sqlite3_step(stmt);
        rc == SQLITE_ROW
    } {
        let c2rust_fresh2 = count;
        count += 1;
        populate_tag(stmt, tags.offset(c2rust_fresh2 as isize));
    }
    if rc != SQLITE_ROW && rc != SQLITE_DONE {
        count = -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_tag_set_for_camera(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut tag_uuids: *const *const ::core::ffi::c_char,
    mut tag_count: ::core::ffi::c_int,
) -> db_camera_tag_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || !valid_uuid_string(camera_uuid)
        || tag_count < 0 as ::core::ffi::c_int
        || tag_count > CAMERA_TAG_MAX_ASSIGNMENTS
        || tag_count > 0 as ::core::ffi::c_int && tag_uuids.is_null()
    {
        return db_camera_tag_result_t::DB_CAMERA_TAG_INVALID;
    }
    pthread_mutex_lock(mutex);
    if !row_exists_locked(
        db,
        b"SELECT 1 FROM streams WHERE camera_uuid = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        camera_uuid,
    ) {
        pthread_mutex_unlock(mutex);
        return db_camera_tag_result_t::DB_CAMERA_TAG_NOT_FOUND;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < tag_count {
        if !valid_uuid_string(*tag_uuids.offset(i as isize))
            || !row_exists_locked(
                db,
                b"SELECT 1 FROM camera_tags WHERE uuid = ?;\0".as_ptr()
                    as *const ::core::ffi::c_char,
                *tag_uuids.offset(i as isize),
            )
        {
            pthread_mutex_unlock(mutex);
            return db_camera_tag_result_t::DB_CAMERA_TAG_NOT_FOUND;
        }
        i += 1;
    }
    let mut owns_transaction: bool = r#false != 0;
    if !transaction_begin(db, &raw mut owns_transaction) {
        pthread_mutex_unlock(mutex);
        return db_camera_tag_result_t::DB_CAMERA_TAG_ERROR;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM camera_tag_assignments WHERE camera_uuid = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            camera_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while rc == SQLITE_DONE && i_0 < tag_count {
        rc = sqlite3_prepare_v2(
            db,
            b"INSERT OR IGNORE INTO camera_tag_assignments (camera_uuid, tag_uuid) VALUES (?, ?);\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc != SQLITE_OK {
            break;
        }
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            camera_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            *tag_uuids.offset(i_0 as isize),
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        i_0 += 1;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    let mut rebuild_rc: ::core::ffi::c_int = if rc == SQLITE_DONE {
        rebuild_legacy_for_camera_locked(db, camera_uuid)
    } else {
        -1 as ::core::ffi::c_int
    };
    let mut success: bool = rc == SQLITE_DONE && rebuild_rc == 0 as ::core::ffi::c_int;
    success = transaction_finish(db, owns_transaction, success);
    pthread_mutex_unlock(mutex);
    if success {
        return db_camera_tag_result_t::DB_CAMERA_TAG_OK;
    }
    return db_camera_tag_result_t(if rebuild_rc == -2 as ::core::ffi::c_int {
        db_camera_tag_result_t::DB_CAMERA_TAG_LIMIT.0
    } else {
        db_camera_tag_result_t::DB_CAMERA_TAG_ERROR.0
    });
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
