use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
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
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
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
pub type cJSON_bool = ::core::ffi::c_int;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct investigation_bookmark_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub owner_user_id: int64_t,
    pub title: [::core::ffi::c_char; 128],
    pub note: [::core::ffi::c_char; 2048],
    pub start_time: int64_t,
    pub end_time: int64_t,
    pub cursor_time: int64_t,
    pub primary_camera_uuid: [::core::ffi::c_char; 37],
    pub filters_json: [::core::ffi::c_char; 4096],
    pub representative_result_json: [::core::ffi::c_char; 2048],
    pub camera_count: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_investigation_bookmark_result_t(pub ::core::ffi::c_int);
impl db_investigation_bookmark_result_t {
    pub const DB_INVESTIGATION_BOOKMARK_OK: Self = Self(0);
    pub const DB_INVESTIGATION_BOOKMARK_NOT_FOUND: Self = Self(-1);
    pub const DB_INVESTIGATION_BOOKMARK_INVALID: Self = Self(-2);
    pub const DB_INVESTIGATION_BOOKMARK_STALE: Self = Self(-3);
    pub const DB_INVESTIGATION_BOOKMARK_LIMIT: Self = Self(-4);
    pub const DB_INVESTIGATION_BOOKMARK_ERROR: Self = Self(-5);
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const INVESTIGATION_BOOKMARK_MAX_CAMERAS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const INVESTIGATION_BOOKMARK_MAX_PER_OWNER: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
unsafe extern "C" fn owner_predicate_bind(
    mut statement: *mut sqlite3_stmt,
    mut parameter: ::core::ffi::c_int,
    mut owner_user_id: int64_t,
) -> bool {
    if statement.is_null() || parameter < 1 as ::core::ffi::c_int || owner_user_id < 0 as int64_t {
        return r#false != 0;
    }
    sqlite3_bind_int64(statement, parameter, owner_user_id as sqlite3_int64);
    return r#true != 0;
}
unsafe extern "C" fn valid_json_object(
    mut value: *const ::core::ffi::c_char,
    mut allow_empty: bool,
) -> bool {
    if value.is_null() || *value.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return allow_empty;
    }
    let mut root: *mut cJSON = cJSON_Parse(value);
    let mut valid: bool = cJSON_IsObject(root) != 0;
    cJSON_Delete(root);
    return valid;
}
unsafe extern "C" fn valid_cameras(
    mut camera_uuids: *const [::core::ffi::c_char; 37],
    mut camera_count: ::core::ffi::c_int,
    mut primary_camera_uuid: *const ::core::ffi::c_char,
) -> bool {
    if camera_uuids.is_null()
        || camera_count < 1 as ::core::ffi::c_int
        || camera_count > INVESTIGATION_BOOKMARK_MAX_CAMERAS
        || !lightnvr_uuid_is_valid(primary_camera_uuid)
    {
        return r#false != 0;
    }
    let mut primary_found: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < camera_count {
        if !lightnvr_uuid_is_valid(
            &raw const *camera_uuids.offset(i as isize) as *const ::core::ffi::c_char
        ) {
            return r#false != 0;
        }
        if strcmp(
            &raw const *camera_uuids.offset(i as isize) as *const ::core::ffi::c_char,
            primary_camera_uuid,
        ) == 0 as ::core::ffi::c_int
        {
            primary_found = r#true != 0;
        }
        let mut previous: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while previous < i {
            if strcmp(
                &raw const *camera_uuids.offset(previous as isize) as *const ::core::ffi::c_char,
                &raw const *camera_uuids.offset(i as isize) as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                return r#false != 0;
            }
            previous += 1;
        }
        i += 1;
    }
    return primary_found;
}
unsafe extern "C" fn valid_bookmark(
    mut bookmark: *const investigation_bookmark_t,
    mut require_uuid: bool,
) -> bool {
    if bookmark.is_null()
        || (*bookmark).owner_user_id < 0 as int64_t
        || require_uuid as ::core::ffi::c_int != 0
            && !lightnvr_uuid_is_valid(&raw const (*bookmark).uuid as *const ::core::ffi::c_char)
        || (*bookmark).title[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strnlen(
            &raw const (*bookmark).title as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        ) >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
        || strnlen(
            &raw const (*bookmark).note as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
        ) >= ::core::mem::size_of::<[::core::ffi::c_char; 2048]>()
        || (*bookmark).start_time < 1 as int64_t
        || (*bookmark).end_time <= (*bookmark).start_time
        || ((*bookmark).end_time - (*bookmark).start_time) as ::core::ffi::c_longlong
            > 31 as ::core::ffi::c_longlong
                * 24 as ::core::ffi::c_longlong
                * 60 as ::core::ffi::c_longlong
                * 60 as ::core::ffi::c_longlong
        || (*bookmark).cursor_time < (*bookmark).start_time
        || (*bookmark).cursor_time > (*bookmark).end_time
        || !lightnvr_uuid_is_valid(
            &raw const (*bookmark).primary_camera_uuid as *const ::core::ffi::c_char,
        )
        || strnlen(
            &raw const (*bookmark).filters_json as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        ) >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
        || strnlen(
            &raw const (*bookmark).representative_result_json as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
        ) >= ::core::mem::size_of::<[::core::ffi::c_char; 2048]>()
        || !valid_json_object(
            &raw const (*bookmark).filters_json as *const ::core::ffi::c_char,
            r#false != 0,
        )
        || !valid_json_object(
            &raw const (*bookmark).representative_result_json as *const ::core::ffi::c_char,
            r#true != 0,
        )
    {
        return r#false != 0;
    }
    return r#true != 0;
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
unsafe extern "C" fn populate_bookmark(
    mut statement: *mut sqlite3_stmt,
    mut bookmark: *mut investigation_bookmark_t,
) {
    memset(
        bookmark as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<investigation_bookmark_t>(),
    );
    copy_column(
        &raw mut (*bookmark).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        0 as ::core::ffi::c_int,
    );
    (*bookmark).owner_user_id = sqlite3_column_int64(statement, 1 as ::core::ffi::c_int) as int64_t;
    copy_column(
        &raw mut (*bookmark).title as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        statement,
        2 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*bookmark).note as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
        statement,
        3 as ::core::ffi::c_int,
    );
    (*bookmark).start_time = sqlite3_column_int64(statement, 4 as ::core::ffi::c_int) as int64_t;
    (*bookmark).end_time = sqlite3_column_int64(statement, 5 as ::core::ffi::c_int) as int64_t;
    (*bookmark).cursor_time = sqlite3_column_int64(statement, 6 as ::core::ffi::c_int) as int64_t;
    copy_column(
        &raw mut (*bookmark).primary_camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        7 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*bookmark).filters_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        statement,
        8 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*bookmark).representative_result_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
        statement,
        9 as ::core::ffi::c_int,
    );
    (*bookmark).revision = sqlite3_column_int64(statement, 10 as ::core::ffi::c_int) as int64_t;
    (*bookmark).created_at = sqlite3_column_int64(statement, 11 as ::core::ffi::c_int) as int64_t;
    (*bookmark).updated_at = sqlite3_column_int64(statement, 12 as ::core::ffi::c_int) as int64_t;
    (*bookmark).camera_count = sqlite3_column_int(statement, 13 as ::core::ffi::c_int);
}
unsafe extern "C" fn get_locked(
    mut db: *mut sqlite3,
    mut owner_user_id: int64_t,
    mut uuid: *const ::core::ffi::c_char,
    mut bookmark: *mut investigation_bookmark_t,
) -> db_investigation_bookmark_result_t {
    let mut sql: *const ::core::ffi::c_char = b"SELECT b.uuid, COALESCE(b.owner_user_id, 0), b.title, b.note, b.start_time, b.end_time, b.cursor_time, b.primary_camera_uuid, b.filters_json, COALESCE(b.representative_result_json, ''), b.revision, b.created_at, b.updated_at, (SELECT count(*) FROM investigation_bookmark_cameras c  WHERE c.bookmark_uuid = b.uuid) FROM investigation_bookmarks b WHERE b.uuid=? AND ((?=0 AND b.owner_user_id IS NULL) OR b.owner_user_id=?) LIMIT 1;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result != SQLITE_OK {
        return db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_ERROR;
    }
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
    owner_predicate_bind(statement, 2 as ::core::ffi::c_int, owner_user_id);
    owner_predicate_bind(statement, 3 as ::core::ffi::c_int, owner_user_id);
    result = sqlite3_step(statement);
    let mut outcome: db_investigation_bookmark_result_t =
        db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_NOT_FOUND;
    if result == SQLITE_ROW {
        populate_bookmark(statement, bookmark);
        outcome = db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_OK;
    } else if result != SQLITE_DONE {
        outcome = db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_ERROR;
    }
    sqlite3_finalize(statement);
    return outcome;
}
unsafe extern "C" fn count_locked(
    mut db: *mut sqlite3,
    mut owner_user_id: int64_t,
) -> ::core::ffi::c_int {
    let mut sql: *const ::core::ffi::c_char = b"SELECT count(*) FROM investigation_bookmarks WHERE ((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        return -1 as ::core::ffi::c_int;
    }
    owner_predicate_bind(statement, 1 as ::core::ffi::c_int, owner_user_id);
    owner_predicate_bind(statement, 2 as ::core::ffi::c_int, owner_user_id);
    let mut count: ::core::ffi::c_int = if sqlite3_step(statement) == SQLITE_ROW {
        sqlite3_column_int(statement, 0 as ::core::ffi::c_int)
    } else {
        -1 as ::core::ffi::c_int
    };
    sqlite3_finalize(statement);
    return count;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_investigation_bookmark_count(
    mut owner_user_id: int64_t,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() || owner_user_id < 0 as int64_t {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut count: ::core::ffi::c_int = count_locked(db, owner_user_id);
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_investigation_bookmark_list(
    mut owner_user_id: int64_t,
    mut offset: ::core::ffi::c_int,
    mut bookmarks: *mut investigation_bookmark_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || mutex.is_null()
        || bookmarks.is_null()
        || max_count <= 0 as ::core::ffi::c_int
        || offset < 0 as ::core::ffi::c_int
        || owner_user_id < 0 as int64_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT b.uuid, COALESCE(b.owner_user_id, 0), b.title, b.note, b.start_time, b.end_time, b.cursor_time, b.primary_camera_uuid, b.filters_json, COALESCE(b.representative_result_json, ''), b.revision, b.created_at, b.updated_at, (SELECT count(*) FROM investigation_bookmark_cameras c  WHERE c.bookmark_uuid = b.uuid) FROM investigation_bookmarks b WHERE ((?=0 AND b.owner_user_id IS NULL) OR b.owner_user_id=?) ORDER BY b.updated_at DESC, b.uuid LIMIT ? OFFSET ?;\0"
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
    if result != SQLITE_OK {
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    owner_predicate_bind(statement, 1 as ::core::ffi::c_int, owner_user_id);
    owner_predicate_bind(statement, 2 as ::core::ffi::c_int, owner_user_id);
    sqlite3_bind_int(statement, 3 as ::core::ffi::c_int, max_count);
    sqlite3_bind_int(statement, 4 as ::core::ffi::c_int, offset);
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while count < max_count && {
        result = sqlite3_step(statement);
        result == SQLITE_ROW
    } {
        let c2rust_fresh0 = count;
        count += 1;
        populate_bookmark(statement, bookmarks.offset(c2rust_fresh0 as isize));
    }
    if result != SQLITE_DONE && result != SQLITE_ROW {
        count = -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_investigation_bookmark_get(
    mut owner_user_id: int64_t,
    mut uuid: *const ::core::ffi::c_char,
    mut bookmark: *mut investigation_bookmark_t,
) -> db_investigation_bookmark_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || mutex.is_null()
        || bookmark.is_null()
        || owner_user_id < 0 as int64_t
        || !lightnvr_uuid_is_valid(uuid)
    {
        return db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut result: db_investigation_bookmark_result_t =
        get_locked(db, owner_user_id, uuid, bookmark);
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_investigation_bookmark_list_cameras(
    mut bookmark_uuid: *const ::core::ffi::c_char,
    mut camera_uuids: *mut [::core::ffi::c_char; 37],
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || mutex.is_null()
        || camera_uuids.is_null()
        || max_count <= 0 as ::core::ffi::c_int
        || !lightnvr_uuid_is_valid(bookmark_uuid)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT camera_uuid FROM investigation_bookmark_cameras WHERE bookmark_uuid=? ORDER BY sort_order LIMIT ?;\0"
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
    if result != SQLITE_OK {
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_text(
        statement,
        1 as ::core::ffi::c_int,
        bookmark_uuid,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_int(statement, 2 as ::core::ffi::c_int, max_count);
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while count < max_count && {
        result = sqlite3_step(statement);
        result == SQLITE_ROW
    } {
        copy_column(
            &raw mut *camera_uuids.offset(count as isize) as *mut ::core::ffi::c_char,
            CAMERA_UUID_STRING_SIZE as size_t,
            statement,
            0 as ::core::ffi::c_int,
        );
        count += 1;
    }
    if result != SQLITE_DONE && result != SQLITE_ROW {
        count = -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_investigation_bookmark_create(
    mut bookmark: *mut investigation_bookmark_t,
    mut camera_uuids: *const [::core::ffi::c_char; 37],
    mut camera_count: ::core::ffi::c_int,
) -> db_investigation_bookmark_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || mutex.is_null()
        || !valid_bookmark(bookmark, r#false != 0)
        || !valid_cameras(
            camera_uuids,
            camera_count,
            &raw mut (*bookmark).primary_camera_uuid as *mut ::core::ffi::c_char,
        )
        || lightnvr_uuid_generate_v4(&raw mut (*bookmark).uuid as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
    {
        return db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_INVALID;
    }
    let mut normalized_title: [::core::ffi::c_char; 128] = [0; 128];
    if copy_trimmed_value(
        &raw mut normalized_title as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        &raw mut (*bookmark).title as *mut ::core::ffi::c_char,
        0 as size_t,
    ) == 0 as size_t
    {
        return db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut existing_count: ::core::ffi::c_int = count_locked(db, (*bookmark).owner_user_id);
    if existing_count < 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(mutex);
        return db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_ERROR;
    }
    if existing_count >= INVESTIGATION_BOOKMARK_MAX_PER_OWNER {
        pthread_mutex_unlock(mutex);
        return db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_LIMIT;
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
        return db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_ERROR;
    }
    let mut insert_bookmark: *const ::core::ffi::c_char = b"INSERT INTO investigation_bookmarks(uuid,owner_user_id,title,note,start_time,end_time,cursor_time,primary_camera_uuid,filters_json,representative_result_json) VALUES(?,?,?,?,?,?,?,?,?,?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        insert_bookmark,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            &raw mut (*bookmark).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if (*bookmark).owner_user_id > 0 as int64_t {
            sqlite3_bind_int64(
                statement,
                2 as ::core::ffi::c_int,
                (*bookmark).owner_user_id as sqlite3_int64,
            );
        } else {
            sqlite3_bind_null(statement, 2 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            &raw mut normalized_title as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            4 as ::core::ffi::c_int,
            &raw mut (*bookmark).note as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            5 as ::core::ffi::c_int,
            (*bookmark).start_time as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            6 as ::core::ffi::c_int,
            (*bookmark).end_time as sqlite3_int64,
        );
        sqlite3_bind_int64(
            statement,
            7 as ::core::ffi::c_int,
            (*bookmark).cursor_time as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            8 as ::core::ffi::c_int,
            &raw mut (*bookmark).primary_camera_uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            9 as ::core::ffi::c_int,
            &raw mut (*bookmark).filters_json as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if (*bookmark).representative_result_json[0usize] != 0 {
            sqlite3_bind_text(
                statement,
                10 as ::core::ffi::c_int,
                &raw mut (*bookmark).representative_result_json as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(statement, 10 as ::core::ffi::c_int);
        }
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    let mut insert_camera: *const ::core::ffi::c_char = b"INSERT INTO investigation_bookmark_cameras(bookmark_uuid,camera_uuid,sort_order) VALUES(?,?,?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while result == SQLITE_DONE && i < camera_count {
        statement = ::core::ptr::null_mut::<sqlite3_stmt>();
        result = sqlite3_prepare_v2(
            db,
            insert_camera,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if result == SQLITE_OK {
            sqlite3_bind_text(
                statement,
                1 as ::core::ffi::c_int,
                &raw mut (*bookmark).uuid as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                statement,
                2 as ::core::ffi::c_int,
                &raw const *camera_uuids.offset(i as isize) as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_int(statement, 3 as ::core::ffi::c_int, i);
            result = sqlite3_step(statement);
        }
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
        i += 1;
    }
    if !transaction_finish(db, result == SQLITE_DONE) {
        pthread_mutex_unlock(mutex);
        return db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_ERROR;
    }
    let mut outcome: db_investigation_bookmark_result_t = get_locked(
        db,
        (*bookmark).owner_user_id,
        &raw mut (*bookmark).uuid as *mut ::core::ffi::c_char,
        bookmark,
    );
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_investigation_bookmark_update_metadata(
    mut bookmark: *mut investigation_bookmark_t,
    mut expected_revision: int64_t,
) -> db_investigation_bookmark_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || mutex.is_null()
        || !valid_bookmark(bookmark, r#true != 0)
        || expected_revision < 1 as int64_t
    {
        return db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_INVALID;
    }
    let mut normalized_title: [::core::ffi::c_char; 128] = [0; 128];
    if copy_trimmed_value(
        &raw mut normalized_title as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        &raw mut (*bookmark).title as *mut ::core::ffi::c_char,
        0 as size_t,
    ) == 0 as size_t
    {
        return db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut existing: investigation_bookmark_t = investigation_bookmark_t {
        uuid: [0; 37],
        owner_user_id: 0,
        title: [0; 128],
        note: [0; 2048],
        start_time: 0,
        end_time: 0,
        cursor_time: 0,
        primary_camera_uuid: [0; 37],
        filters_json: [0; 4096],
        representative_result_json: [0; 2048],
        camera_count: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut outcome: db_investigation_bookmark_result_t = get_locked(
        db,
        (*bookmark).owner_user_id,
        &raw mut (*bookmark).uuid as *mut ::core::ffi::c_char,
        &raw mut existing,
    );
    if outcome.0 != db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_OK.0 {
        pthread_mutex_unlock(mutex);
        return outcome;
    }
    if existing.revision != expected_revision {
        pthread_mutex_unlock(mutex);
        return db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_STALE;
    }
    let mut sql: *const ::core::ffi::c_char = b"UPDATE investigation_bookmarks SET title=?,note=?,revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=? AND revision=? AND ((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);\0"
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
            &raw mut normalized_title as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            &raw mut (*bookmark).note as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            &raw mut (*bookmark).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            4 as ::core::ffi::c_int,
            expected_revision as sqlite3_int64,
        );
        owner_predicate_bind(
            statement,
            5 as ::core::ffi::c_int,
            (*bookmark).owner_user_id,
        );
        owner_predicate_bind(
            statement,
            6 as ::core::ffi::c_int,
            (*bookmark).owner_user_id,
        );
        result = sqlite3_step(statement);
    }
    let mut changed: ::core::ffi::c_int = if result == SQLITE_DONE {
        sqlite3_changes(db)
    } else {
        0 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if result != SQLITE_DONE || changed != 1 as ::core::ffi::c_int {
        pthread_mutex_unlock(mutex);
        return db_investigation_bookmark_result_t(if result == SQLITE_DONE {
            db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_STALE.0
        } else {
            db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_ERROR.0
        });
    }
    outcome = get_locked(
        db,
        (*bookmark).owner_user_id,
        &raw mut (*bookmark).uuid as *mut ::core::ffi::c_char,
        bookmark,
    );
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_investigation_bookmark_delete(
    mut owner_user_id: int64_t,
    mut uuid: *const ::core::ffi::c_char,
    mut expected_revision: int64_t,
) -> db_investigation_bookmark_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || mutex.is_null()
        || owner_user_id < 0 as int64_t
        || expected_revision < 1 as int64_t
        || !lightnvr_uuid_is_valid(uuid)
    {
        return db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut existing: investigation_bookmark_t = investigation_bookmark_t {
        uuid: [0; 37],
        owner_user_id: 0,
        title: [0; 128],
        note: [0; 2048],
        start_time: 0,
        end_time: 0,
        cursor_time: 0,
        primary_camera_uuid: [0; 37],
        filters_json: [0; 4096],
        representative_result_json: [0; 2048],
        camera_count: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut outcome: db_investigation_bookmark_result_t =
        get_locked(db, owner_user_id, uuid, &raw mut existing);
    if outcome.0 != db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_OK.0 {
        pthread_mutex_unlock(mutex);
        return outcome;
    }
    if existing.revision != expected_revision {
        pthread_mutex_unlock(mutex);
        return db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_STALE;
    }
    let mut sql: *const ::core::ffi::c_char = b"DELETE FROM investigation_bookmarks WHERE uuid=? AND revision=? AND ((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);\0"
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
        sqlite3_bind_int64(
            statement,
            2 as ::core::ffi::c_int,
            expected_revision as sqlite3_int64,
        );
        owner_predicate_bind(statement, 3 as ::core::ffi::c_int, owner_user_id);
        owner_predicate_bind(statement, 4 as ::core::ffi::c_int, owner_user_id);
        result = sqlite3_step(statement);
    }
    let mut changed: ::core::ffi::c_int = if result == SQLITE_DONE {
        sqlite3_changes(db)
    } else {
        0 as ::core::ffi::c_int
    };
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    if result != SQLITE_DONE {
        return db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_ERROR;
    }
    return db_investigation_bookmark_result_t(if changed == 1 as ::core::ffi::c_int {
        db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_OK.0
    } else {
        db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_STALE.0
    });
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
