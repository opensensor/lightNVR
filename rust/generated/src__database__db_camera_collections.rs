use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type fleet_selector;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fleet_selector_parse(
        json: *const cJSON,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> *mut fleet_selector_t;
    unsafe fn fleet_selector_free(selector: *mut fleet_selector_t);
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
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cJSON {
    pub next: *mut cJSON,
    pub prev: *mut cJSON,
    pub child: *mut cJSON,
    pub r#type: ::core::ffi::c_int,
    pub valuestring: *mut ::core::ffi::c_char,
    pub valueint: ::core::ffi::c_int,
    pub valuedouble: ::core::ffi::c_double,
    pub string: *mut ::core::ffi::c_char,
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
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type fleet_selector_t = fleet_selector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camera_collection_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub description: [::core::ffi::c_char; 512],
    pub collection_type: [::core::ffi::c_char; 16],
    pub selector_json: [::core::ffi::c_char; 8192],
    pub is_shared: bool,
    pub owner_user_id: int64_t,
    pub member_count: ::core::ffi::c_int,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_camera_collection_result_t(pub ::core::ffi::c_int);
impl db_camera_collection_result_t {
    pub const DB_CAMERA_COLLECTION_OK: Self = Self(0);
    pub const DB_CAMERA_COLLECTION_NOT_FOUND: Self = Self(-1);
    pub const DB_CAMERA_COLLECTION_CONFLICT: Self = Self(-2);
    pub const DB_CAMERA_COLLECTION_INVALID: Self = Self(-3);
    pub const DB_CAMERA_COLLECTION_ERROR: Self = Self(-4);
    pub const DB_CAMERA_COLLECTION_WRONG_TYPE: Self = Self(-5);
    pub const DB_CAMERA_COLLECTION_LIMIT: Self = Self(-6);
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const CAMERA_COLLECTION_MAX_MEMBERS: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
unsafe extern "C" fn valid_uuid(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null()
        || strlen(value) != (CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int) as size_t
    {
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int {
        let mut c: ::core::ffi::c_uchar = *value.offset(i as isize) as ::core::ffi::c_uchar;
        if i == 8 as ::core::ffi::c_int
            || i == 13 as ::core::ffi::c_int
            || i == 18 as ::core::ffi::c_int
            || i == 23 as ::core::ffi::c_int
        {
            if c as ::core::ffi::c_int != '-' as ::core::ffi::c_int {
                return r#false != 0;
            }
        } else if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISxdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        {
            return r#false != 0;
        }
        i += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_name(
    mut input: *const ::core::ffi::c_char,
    mut normalized: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> bool {
    if input.is_null() || copy_trimmed_value(normalized, size, input, 0 as size_t) == 0 as size_t {
        return r#false != 0;
    }
    let mut p: *const ::core::ffi::c_uchar = normalized as *const ::core::ffi::c_uchar;
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
unsafe extern "C" fn valid_selector_json(mut json: *const ::core::ffi::c_char) -> bool {
    if json.is_null() || *json.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut root: *mut cJSON = cJSON_Parse(json);
    if root.is_null() {
        return r#false != 0;
    }
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
    let mut selector: *mut fleet_selector_t = fleet_selector_parse(
        root,
        &raw mut error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    let mut valid: bool = !selector.is_null();
    fleet_selector_free(selector);
    cJSON_Delete(root);
    return valid;
}
unsafe extern "C" fn valid_collection(
    mut collection: *mut camera_collection_t,
    mut normalized_name: *mut ::core::ffi::c_char,
    mut name_size: size_t,
) -> bool {
    if collection.is_null()
        || !valid_name(
            &raw mut (*collection).name as *mut ::core::ffi::c_char,
            normalized_name,
            name_size,
        )
    {
        return r#false != 0;
    }
    if strcmp(
        &raw mut (*collection).collection_type as *mut ::core::ffi::c_char,
        b"static\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    if strcmp(
        &raw mut (*collection).collection_type as *mut ::core::ffi::c_char,
        b"smart\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return valid_selector_json(
            &raw mut (*collection).selector_json as *mut ::core::ffi::c_char,
        );
    }
    return r#false != 0;
}
unsafe extern "C" fn transaction_begin(mut db: *mut sqlite3) -> bool {
    return sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK;
}
unsafe extern "C" fn transaction_finish(mut db: *mut sqlite3, mut success: bool) -> bool {
    if !success {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        return r#false != 0;
    }
    if sqlite3_exec(
        db,
        b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        return r#true != 0;
    }
    sqlite3_exec(
        db,
        b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    return r#false != 0;
}
unsafe extern "C" fn copy_column(
    mut destination: *mut ::core::ffi::c_char,
    mut destination_size: size_t,
    mut stmt: *mut sqlite3_stmt,
    mut column: ::core::ffi::c_int,
) {
    let mut value: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, column) as *const ::core::ffi::c_char;
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
unsafe extern "C" fn populate_collection(
    mut stmt: *mut sqlite3_stmt,
    mut collection: *mut camera_collection_t,
) {
    memset(
        collection as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<camera_collection_t>(),
    );
    copy_column(
        &raw mut (*collection).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        stmt,
        0 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*collection).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        stmt,
        1 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*collection).description as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        stmt,
        2 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*collection).collection_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        stmt,
        3 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*collection).selector_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        stmt,
        4 as ::core::ffi::c_int,
    );
    (*collection).is_shared =
        sqlite3_column_int(stmt, 5 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    (*collection).owner_user_id = sqlite3_column_int64(stmt, 6 as ::core::ffi::c_int) as int64_t;
    (*collection).created_at = sqlite3_column_int64(stmt, 7 as ::core::ffi::c_int) as int64_t;
    (*collection).updated_at = sqlite3_column_int64(stmt, 8 as ::core::ffi::c_int) as int64_t;
    (*collection).member_count = sqlite3_column_int(stmt, 9 as ::core::ffi::c_int);
}
unsafe extern "C" fn get_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut collection: *mut camera_collection_t,
) -> db_camera_collection_result_t {
    let mut sql: *const ::core::ffi::c_char = b"SELECT c.uuid, c.name, c.description, c.collection_type, c.selector_json, c.is_shared, COALESCE(c.owner_user_id, 0), c.created_at, c.updated_at, (SELECT count(*) FROM camera_collection_members m  WHERE m.collection_uuid = c.uuid) FROM camera_collections c WHERE c.uuid = ? LIMIT 1;\0"
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
        return db_camera_collection_result_t::DB_CAMERA_COLLECTION_ERROR;
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
    let mut result: db_camera_collection_result_t =
        db_camera_collection_result_t::DB_CAMERA_COLLECTION_NOT_FOUND;
    rc = sqlite3_step(stmt);
    if rc == SQLITE_ROW {
        populate_collection(stmt, collection);
        result = db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK;
    } else if rc != SQLITE_DONE {
        result = db_camera_collection_result_t::DB_CAMERA_COLLECTION_ERROR;
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
unsafe extern "C" fn authorization_scope_in_use_locked(
    mut db: *mut sqlite3,
    mut collection_uuid: *const ::core::ffi::c_char,
) -> bool {
    return row_exists_locked(
        db,
        b"SELECT collection_uuid FROM authz_grants WHERE collection_uuid = ?1 UNION ALL SELECT collection_uuid FROM authz_api_tokens WHERE collection_uuid = ?1 LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        collection_uuid,
    );
}
unsafe extern "C" fn bump_authorization_policy_locked(mut db: *mut sqlite3) -> bool {
    return sqlite3_exec(
        db,
        b"UPDATE authz_policy_state SET version=version+1,updated_at=strftime('%s','now') WHERE id=1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_collection_count() -> ::core::ffi::c_int {
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
        b"SELECT count(*) FROM camera_collections;\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn db_camera_collection_list(
    mut collections: *mut camera_collection_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || collections.is_null() || max_count <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT c.uuid, c.name, c.description, c.collection_type, c.selector_json, c.is_shared, COALESCE(c.owner_user_id, 0), c.created_at, c.updated_at, (SELECT count(*) FROM camera_collection_members m  WHERE m.collection_uuid = c.uuid) FROM camera_collections c ORDER BY c.name COLLATE NOCASE;\0"
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
        populate_collection(stmt, collections.offset(c2rust_fresh0 as isize));
    }
    if rc != SQLITE_ROW && rc != SQLITE_DONE {
        count = -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_collection_get_on_connection(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut collection: *mut camera_collection_t,
) -> db_camera_collection_result_t {
    if db.is_null() || collection.is_null() || !valid_uuid(uuid) {
        return db_camera_collection_result_t::DB_CAMERA_COLLECTION_INVALID;
    }
    return get_locked(db, uuid, collection);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_collection_get(
    mut uuid: *const ::core::ffi::c_char,
    mut collection: *mut camera_collection_t,
) -> db_camera_collection_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || collection.is_null() || !valid_uuid(uuid) {
        return db_camera_collection_result_t::DB_CAMERA_COLLECTION_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut result: db_camera_collection_result_t = get_locked(db, uuid, collection);
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_collection_create(
    mut collection: *mut camera_collection_t,
) -> db_camera_collection_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    let mut normalized_name: [::core::ffi::c_char; 128] = [0; 128];
    if db.is_null()
        || !valid_collection(
            collection,
            &raw mut normalized_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        )
    {
        return db_camera_collection_result_t::DB_CAMERA_COLLECTION_INVALID;
    }
    if strcmp(
        &raw mut (*collection).collection_type as *mut ::core::ffi::c_char,
        b"static\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*collection).selector_json[0usize] = '\0' as ::core::ffi::c_char;
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO camera_collections (uuid, name, description, collection_type, selector_json, is_shared,  owner_user_id) VALUES (lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)), 2) || '-' || substr('89ab', (abs(random()) % 4) + 1, 1) || substr(hex(randomblob(2)), 2) || '-' || hex(randomblob(6))), ?, ?, ?, ?, ?, ?);\0"
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
            &raw mut normalized_name as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw mut (*collection).description as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            &raw mut (*collection).collection_type as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            4 as ::core::ffi::c_int,
            &raw mut (*collection).selector_json as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            stmt,
            5 as ::core::ffi::c_int,
            if (*collection).is_shared as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        if (*collection).owner_user_id > 0 as int64_t {
            sqlite3_bind_int64(
                stmt,
                6 as ::core::ffi::c_int,
                (*collection).owner_user_id as sqlite3_int64,
            );
        } else {
            sqlite3_bind_null(stmt, 6 as ::core::ffi::c_int);
        }
        rc = sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    if rc != SQLITE_DONE {
        let mut result: db_camera_collection_result_t =
            db_camera_collection_result_t(if rc == SQLITE_CONSTRAINT {
                db_camera_collection_result_t::DB_CAMERA_COLLECTION_CONFLICT.0
            } else {
                db_camera_collection_result_t::DB_CAMERA_COLLECTION_ERROR.0
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
        b"SELECT uuid FROM camera_collections WHERE id = ? LIMIT 1;\0".as_ptr()
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
            copy_column(
                &raw mut uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                stmt,
                0 as ::core::ffi::c_int,
            );
            sqlite3_finalize(stmt);
            stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            let mut result_0: db_camera_collection_result_t =
                get_locked(db, &raw mut uuid as *mut ::core::ffi::c_char, collection);
            pthread_mutex_unlock(mutex);
            return result_0;
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return db_camera_collection_result_t::DB_CAMERA_COLLECTION_ERROR;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_collection_update(
    mut collection: *mut camera_collection_t,
) -> db_camera_collection_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    let mut normalized_name: [::core::ffi::c_char; 128] = [0; 128];
    if db.is_null()
        || collection.is_null()
        || !valid_uuid(&raw mut (*collection).uuid as *mut ::core::ffi::c_char)
        || !valid_collection(
            collection,
            &raw mut normalized_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        )
    {
        return db_camera_collection_result_t::DB_CAMERA_COLLECTION_INVALID;
    }
    if strcmp(
        &raw mut (*collection).collection_type as *mut ::core::ffi::c_char,
        b"static\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*collection).selector_json[0usize] = '\0' as ::core::ffi::c_char;
    }
    pthread_mutex_lock(mutex);
    let mut existing: camera_collection_t = camera_collection_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        collection_type: [0; 16],
        selector_json: [0; 8192],
        is_shared: false,
        owner_user_id: 0,
        member_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_camera_collection_result_t = get_locked(
        db,
        &raw mut (*collection).uuid as *mut ::core::ffi::c_char,
        &raw mut existing,
    );
    if result.0 != db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK.0 {
        pthread_mutex_unlock(mutex);
        return result;
    }
    let mut authorization_scope: bool = authorization_scope_in_use_locked(
        db,
        &raw mut (*collection).uuid as *mut ::core::ffi::c_char,
    );
    if authorization_scope as ::core::ffi::c_int != 0 && !(*collection).is_shared {
        pthread_mutex_unlock(mutex);
        return db_camera_collection_result_t::DB_CAMERA_COLLECTION_CONFLICT;
    }
    let mut scope_membership_changed: bool = authorization_scope as ::core::ffi::c_int != 0
        && (strcmp(
            &raw mut existing.collection_type as *mut ::core::ffi::c_char,
            &raw mut (*collection).collection_type as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut existing.selector_json as *mut ::core::ffi::c_char,
                &raw mut (*collection).selector_json as *mut ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int);
    if !transaction_begin(db) {
        pthread_mutex_unlock(mutex);
        return db_camera_collection_result_t::DB_CAMERA_COLLECTION_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = b"UPDATE camera_collections SET name = ?, description = ?, collection_type = ?, selector_json = ?, is_shared = ?, updated_at = strftime('%s', 'now') WHERE uuid = ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
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
            &raw mut normalized_name as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw mut (*collection).description as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            &raw mut (*collection).collection_type as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            4 as ::core::ffi::c_int,
            &raw mut (*collection).selector_json as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            stmt,
            5 as ::core::ffi::c_int,
            if (*collection).is_shared as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            stmt,
            6 as ::core::ffi::c_int,
            &raw mut (*collection).uuid as *mut ::core::ffi::c_char,
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
    if rc == SQLITE_DONE
        && strcmp(
            &raw mut (*collection).collection_type as *mut ::core::ffi::c_char,
            b"smart\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        rc = sqlite3_prepare_v2(
            db,
            b"DELETE FROM camera_collection_members WHERE collection_uuid = ?;\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_text(
                stmt,
                1 as ::core::ffi::c_int,
                &raw mut (*collection).uuid as *mut ::core::ffi::c_char,
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
    }
    if rc == SQLITE_DONE
        && scope_membership_changed as ::core::ffi::c_int != 0
        && !bump_authorization_policy_locked(db)
    {
        rc = SQLITE_ERROR;
    }
    let mut success: bool = transaction_finish(db, rc == SQLITE_DONE);
    if !success {
        result = db_camera_collection_result_t(if rc == SQLITE_CONSTRAINT {
            db_camera_collection_result_t::DB_CAMERA_COLLECTION_CONFLICT.0
        } else {
            db_camera_collection_result_t::DB_CAMERA_COLLECTION_ERROR.0
        });
        pthread_mutex_unlock(mutex);
        return result;
    }
    result = get_locked(
        db,
        &raw mut (*collection).uuid as *mut ::core::ffi::c_char,
        collection,
    );
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_collection_delete(
    mut uuid: *const ::core::ffi::c_char,
) -> db_camera_collection_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || !valid_uuid(uuid) {
        return db_camera_collection_result_t::DB_CAMERA_COLLECTION_INVALID;
    }
    pthread_mutex_lock(mutex);
    if !row_exists_locked(
        db,
        b"SELECT 1 FROM camera_collections WHERE uuid = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        uuid,
    ) {
        pthread_mutex_unlock(mutex);
        return db_camera_collection_result_t::DB_CAMERA_COLLECTION_NOT_FOUND;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM camera_collections WHERE uuid = ?;\0".as_ptr() as *const ::core::ffi::c_char,
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
    pthread_mutex_unlock(mutex);
    if rc == SQLITE_DONE {
        return db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK;
    }
    return db_camera_collection_result_t(
        if rc & 0xff as ::core::ffi::c_int == SQLITE_CONSTRAINT {
            db_camera_collection_result_t::DB_CAMERA_COLLECTION_CONFLICT.0
        } else {
            db_camera_collection_result_t::DB_CAMERA_COLLECTION_ERROR.0
        },
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_collection_list_members(
    mut collection_uuid: *const ::core::ffi::c_char,
    mut camera_uuids: *mut [::core::ffi::c_char; 37],
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut result: ::core::ffi::c_int = db_camera_collection_list_members_on_connection(
        db,
        collection_uuid,
        camera_uuids,
        max_count,
    );
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_collection_list_members_on_connection(
    mut db: *mut sqlite3,
    mut collection_uuid: *const ::core::ffi::c_char,
    mut camera_uuids: *mut [::core::ffi::c_char; 37],
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if db.is_null()
        || !valid_uuid(collection_uuid)
        || camera_uuids.is_null()
        || max_count <= 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut collection: camera_collection_t = camera_collection_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        collection_type: [0; 16],
        selector_json: [0; 8192],
        is_shared: false,
        owner_user_id: 0,
        member_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_camera_collection_result_t =
        get_locked(db, collection_uuid, &raw mut collection);
    if result.0 == db_camera_collection_result_t::DB_CAMERA_COLLECTION_NOT_FOUND.0 {
        return -2 as ::core::ffi::c_int;
    }
    if result.0 != db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK.0 {
        return -1 as ::core::ffi::c_int;
    }
    if strcmp(
        &raw mut collection.collection_type as *mut ::core::ffi::c_char,
        b"static\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        return -3 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT camera_uuid FROM camera_collection_members WHERE collection_uuid = ? ORDER BY camera_uuid;\0"
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
        collection_uuid,
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
        copy_column(
            &raw mut *camera_uuids.offset(count as isize) as *mut ::core::ffi::c_char,
            CAMERA_UUID_STRING_SIZE as size_t,
            stmt,
            0 as ::core::ffi::c_int,
        );
        count += 1;
    }
    if rc != SQLITE_ROW && rc != SQLITE_DONE {
        count = -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_collection_set_members(
    mut collection_uuid: *const ::core::ffi::c_char,
    mut camera_uuids: *const *const ::core::ffi::c_char,
    mut camera_count: ::core::ffi::c_int,
) -> db_camera_collection_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || !valid_uuid(collection_uuid)
        || camera_count < 0 as ::core::ffi::c_int
        || camera_count > CAMERA_COLLECTION_MAX_MEMBERS
        || camera_count > 0 as ::core::ffi::c_int && camera_uuids.is_null()
    {
        return db_camera_collection_result_t(if camera_count > CAMERA_COLLECTION_MAX_MEMBERS {
            db_camera_collection_result_t::DB_CAMERA_COLLECTION_LIMIT.0
        } else {
            db_camera_collection_result_t::DB_CAMERA_COLLECTION_INVALID.0
        });
    }
    pthread_mutex_lock(mutex);
    let mut collection: camera_collection_t = camera_collection_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        collection_type: [0; 16],
        selector_json: [0; 8192],
        is_shared: false,
        owner_user_id: 0,
        member_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_camera_collection_result_t =
        get_locked(db, collection_uuid, &raw mut collection);
    if result.0 != db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK.0 {
        pthread_mutex_unlock(mutex);
        return result;
    }
    if strcmp(
        &raw mut collection.collection_type as *mut ::core::ffi::c_char,
        b"static\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        pthread_mutex_unlock(mutex);
        return db_camera_collection_result_t::DB_CAMERA_COLLECTION_WRONG_TYPE;
    }
    let mut authorization_scope: bool = authorization_scope_in_use_locked(db, collection_uuid);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < camera_count {
        if !valid_uuid(*camera_uuids.offset(i as isize))
            || !row_exists_locked(
                db,
                b"SELECT 1 FROM streams WHERE camera_uuid = ?;\0".as_ptr()
                    as *const ::core::ffi::c_char,
                *camera_uuids.offset(i as isize),
            )
        {
            pthread_mutex_unlock(mutex);
            return db_camera_collection_result_t::DB_CAMERA_COLLECTION_NOT_FOUND;
        }
        i += 1;
    }
    if !transaction_begin(db) {
        pthread_mutex_unlock(mutex);
        return db_camera_collection_result_t::DB_CAMERA_COLLECTION_ERROR;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM camera_collection_members WHERE collection_uuid = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            collection_uuid,
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
    while rc == SQLITE_DONE && i_0 < camera_count {
        rc = sqlite3_prepare_v2(
            db,
            b"INSERT OR IGNORE INTO camera_collection_members (collection_uuid, camera_uuid) VALUES (?, ?);\0"
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
            collection_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            *camera_uuids.offset(i_0 as isize),
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
    if rc == SQLITE_DONE
        && authorization_scope as ::core::ffi::c_int != 0
        && !bump_authorization_policy_locked(db)
    {
        rc = SQLITE_ERROR;
    }
    let mut success: bool = transaction_finish(db, rc == SQLITE_DONE);
    pthread_mutex_unlock(mutex);
    return db_camera_collection_result_t(if success as ::core::ffi::c_int != 0 {
        db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK.0
    } else {
        db_camera_collection_result_t::DB_CAMERA_COLLECTION_ERROR.0
    });
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
