use ::libc;
unsafe extern "C" {
    pub type sqlite3_stmt;
    pub type sqlite3;
    pub type fleet_selector;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
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
pub struct fleet_saved_view_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub owner_user_id: int64_t,
    pub name: [::core::ffi::c_char; 128],
    pub is_shared: bool,
    pub selector_json: [::core::ffi::c_char; 4096],
    pub search: [::core::ffi::c_char; 256],
    pub collection_uuid: [::core::ffi::c_char; 37],
    pub columns_json: [::core::ffi::c_char; 1024],
    pub sort_by: [::core::ffi::c_char; 32],
    pub sort_order: [::core::ffi::c_char; 8],
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_fleet_saved_view_result_t(pub ::core::ffi::c_int);
impl db_fleet_saved_view_result_t {
    pub const DB_FLEET_SAVED_VIEW_OK: Self = Self(0);
    pub const DB_FLEET_SAVED_VIEW_NOT_FOUND: Self = Self(-1);
    pub const DB_FLEET_SAVED_VIEW_INVALID: Self = Self(-2);
    pub const DB_FLEET_SAVED_VIEW_CONFLICT: Self = Self(-3);
    pub const DB_FLEET_SAVED_VIEW_STALE: Self = Self(-4);
    pub const DB_FLEET_SAVED_VIEW_LIMIT: Self = Self(-5);
    pub const DB_FLEET_SAVED_VIEW_ERROR: Self = Self(-6);
}
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
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
pub type fleet_selector_t = fleet_selector;
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
pub const FLEET_SAVED_VIEW_MAX_VISIBLE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SAVED_VIEW_SELECT_FIELDS: [::core::ffi::c_char; 191] = unsafe {
    ::core::mem::transmute::<
        [u8; 191],
        [::core::ffi::c_char; 191],
    >(
        *b"v.uuid,COALESCE(v.owner_user_id,0),v.name,v.is_shared,v.selector_json,v.search_text,COALESCE(v.collection_uuid,''),v.columns_json,v.sort_by,v.sort_order,v.revision,v.created_at,v.updated_at \0",
    )
};
unsafe extern "C" fn bind_owner(
    mut statement: *mut sqlite3_stmt,
    mut parameter: ::core::ffi::c_int,
    mut owner_user_id: int64_t,
) {
    sqlite3_bind_int64(statement, parameter, owner_user_id as sqlite3_int64);
}
unsafe extern "C" fn copy_column(
    mut destination: *mut ::core::ffi::c_char,
    mut size: size_t,
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
        size,
        0 as size_t,
    );
}
unsafe extern "C" fn populate(mut statement: *mut sqlite3_stmt, mut view: *mut fleet_saved_view_t) {
    memset(
        view as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<fleet_saved_view_t>(),
    );
    copy_column(
        &raw mut (*view).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        0 as ::core::ffi::c_int,
    );
    (*view).owner_user_id = sqlite3_column_int64(statement, 1 as ::core::ffi::c_int) as int64_t;
    copy_column(
        &raw mut (*view).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        statement,
        2 as ::core::ffi::c_int,
    );
    (*view).is_shared =
        sqlite3_column_int(statement, 3 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    copy_column(
        &raw mut (*view).selector_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        statement,
        4 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*view).search as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        statement,
        5 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*view).collection_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        statement,
        6 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*view).columns_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        statement,
        7 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*view).sort_by as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        statement,
        8 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*view).sort_order as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
        statement,
        9 as ::core::ffi::c_int,
    );
    (*view).revision = sqlite3_column_int64(statement, 10 as ::core::ffi::c_int) as int64_t;
    (*view).created_at = sqlite3_column_int64(statement, 11 as ::core::ffi::c_int) as int64_t;
    (*view).updated_at = sqlite3_column_int64(statement, 12 as ::core::ffi::c_int) as int64_t;
}
unsafe extern "C" fn valid_text(
    mut value: *const ::core::ffi::c_char,
    mut maximum: size_t,
    mut allow_empty: bool,
) -> bool {
    if value.is_null() {
        return r#false != 0;
    }
    let mut length: size_t = strnlen(value, maximum);
    if length == maximum || !allow_empty && length == 0 as size_t {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = value as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_IScntrl.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_sort(mut view: *const fleet_saved_view_t) -> bool {
    let mut field: bool = strcmp(
        &raw const (*view).sort_by as *const ::core::ffi::c_char,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            &raw const (*view).sort_by as *const ::core::ffi::c_char,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            &raw const (*view).sort_by as *const ::core::ffi::c_char,
            b"location\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            &raw const (*view).sort_by as *const ::core::ffi::c_char,
            b"health\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            &raw const (*view).sort_by as *const ::core::ffi::c_char,
            b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            &raw const (*view).sort_by as *const ::core::ffi::c_char,
            b"recording_mode\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        || strcmp(
            &raw const (*view).sort_by as *const ::core::ffi::c_char,
            b"address\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
    return field as ::core::ffi::c_int != 0
        && (strcmp(
            &raw const (*view).sort_order as *const ::core::ffi::c_char,
            b"asc\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw const (*view).sort_order as *const ::core::ffi::c_char,
                b"desc\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn valid_columns(mut encoded: *const ::core::ffi::c_char) -> bool {
    static mut allowed: [*const ::core::ffi::c_char; 6] = [
        b"camera\0".as_ptr() as *const ::core::ffi::c_char,
        b"health\0".as_ptr() as *const ::core::ffi::c_char,
        b"location\0".as_ptr() as *const ::core::ffi::c_char,
        b"tags\0".as_ptr() as *const ::core::ffi::c_char,
        b"recording\0".as_ptr() as *const ::core::ffi::c_char,
        b"actions\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut columns: *mut cJSON = cJSON_Parse(encoded);
    let mut count: ::core::ffi::c_int = if cJSON_IsArray(columns) != 0 {
        cJSON_GetArraySize(columns)
    } else {
        0 as ::core::ffi::c_int
    };
    let mut valid: bool = count > 0 as ::core::ffi::c_int && count <= 16 as ::core::ffi::c_int;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while valid as ::core::ffi::c_int != 0 && index < count {
        let mut column: *const cJSON = cJSON_GetArrayItem(columns, index);
        valid = cJSON_IsString(column) != 0 && !(*column).valuestring.is_null();
        let mut found: bool = r#false != 0;
        let mut option: size_t = 0 as size_t;
        while valid as ::core::ffi::c_int != 0
            && option
                < ::core::mem::size_of::<[*const ::core::ffi::c_char; 6]>()
                    .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
        {
            if strcmp((*column).valuestring, allowed[option]) == 0 as ::core::ffi::c_int {
                found = r#true != 0;
                break;
            } else {
                option = option.wrapping_add(1);
            }
        }
        if !found {
            valid = r#false != 0;
        }
        let mut previous: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while valid as ::core::ffi::c_int != 0 && previous < index {
            let mut other: *const cJSON = cJSON_GetArrayItem(columns, previous);
            if strcmp((*column).valuestring, (*other).valuestring) == 0 as ::core::ffi::c_int {
                valid = r#false != 0;
            }
            previous += 1;
        }
        index += 1;
    }
    cJSON_Delete(columns);
    return valid;
}
unsafe extern "C" fn valid_selector(mut encoded: *const ::core::ffi::c_char) -> bool {
    let mut json: *mut cJSON = cJSON_Parse(encoded);
    let mut selector: *mut fleet_selector_t = if cJSON_IsObject(json) != 0 {
        fleet_selector_parse(
            json,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as size_t,
        )
    } else {
        ::core::ptr::null_mut::<fleet_selector_t>()
    };
    let mut valid: bool = !selector.is_null();
    fleet_selector_free(selector);
    cJSON_Delete(json);
    return valid;
}
unsafe extern "C" fn valid_view(
    mut view: *const fleet_saved_view_t,
    mut require_uuid: bool,
) -> bool {
    return !view.is_null()
        && (*view).owner_user_id >= 0 as int64_t
        && (!require_uuid
            || lightnvr_uuid_is_valid(&raw const (*view).uuid as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0)
        && valid_text(
            &raw const (*view).name as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            r#false != 0,
        ) as ::core::ffi::c_int
            != 0
        && valid_text(
            &raw const (*view).search as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            r#true != 0,
        ) as ::core::ffi::c_int
            != 0
        && valid_text(
            &raw const (*view).selector_json as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
            r#false != 0,
        ) as ::core::ffi::c_int
            != 0
        && valid_text(
            &raw const (*view).columns_json as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            r#false != 0,
        ) as ::core::ffi::c_int
            != 0
        && ((*view).collection_uuid[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            || lightnvr_uuid_is_valid(
                &raw const (*view).collection_uuid as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0)
        && valid_selector(&raw const (*view).selector_json as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        && valid_columns(&raw const (*view).columns_json as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        && valid_sort(view) as ::core::ffi::c_int != 0;
}
unsafe extern "C" fn get_locked(
    mut db: *mut sqlite3,
    mut owner_user_id: int64_t,
    mut uuid: *const ::core::ffi::c_char,
    mut visible: bool,
    mut view: *mut fleet_saved_view_t,
) -> db_fleet_saved_view_result_t {
    let mut visibility: *const ::core::ffi::c_char = if visible as ::core::ffi::c_int != 0 {
        b"(v.is_shared=1 OR ((?=0 AND v.owner_user_id IS NULL) OR v.owner_user_id=?))\0".as_ptr()
            as *const ::core::ffi::c_char
    } else {
        b"((?=0 AND v.owner_user_id IS NULL) OR v.owner_user_id=?)\0".as_ptr()
            as *const ::core::ffi::c_char
    };
    let mut sql: [::core::ffi::c_char; 768] = [0; 768];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 768]>(),
        b"SELECT %sFROM fleet_saved_views v WHERE v.uuid=? AND %s LIMIT 1;\0".as_ptr()
            as *const ::core::ffi::c_char,
        SAVED_VIEW_SELECT_FIELDS.as_ptr(),
        visibility,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 768]>()
    {
        return db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_ERROR;
    }
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
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
        bind_owner(statement, 2 as ::core::ffi::c_int, owner_user_id);
        bind_owner(statement, 3 as ::core::ffi::c_int, owner_user_id);
        result = sqlite3_step(statement);
    }
    let mut outcome: db_fleet_saved_view_result_t =
        db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_NOT_FOUND;
    if result == SQLITE_ROW {
        populate(statement, view);
        outcome = db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_OK;
    } else if result != SQLITE_DONE {
        outcome = db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_ERROR;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_fleet_saved_view_list_visible(
    mut owner_user_id: int64_t,
    mut views: *mut fleet_saved_view_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || mutex.is_null()
        || views.is_null()
        || owner_user_id < 0 as int64_t
        || max_count < 1 as ::core::ffi::c_int
        || max_count > FLEET_SAVED_VIEW_MAX_VISIBLE
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT v.uuid,COALESCE(v.owner_user_id,0),v.name,v.is_shared,v.selector_json,v.search_text,COALESCE(v.collection_uuid,''),v.columns_json,v.sort_by,v.sort_order,v.revision,v.created_at,v.updated_at FROM fleet_saved_views v WHERE v.is_shared=1 OR ((?=0 AND v.owner_user_id IS NULL) OR v.owner_user_id=?) ORDER BY v.is_shared DESC,v.name COLLATE NOCASE,v.uuid LIMIT ?;\0"
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
        sqlite3_bind_int(statement, 3 as ::core::ffi::c_int, max_count);
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if result == SQLITE_OK {
        while count < max_count && {
            result = sqlite3_step(statement);
            result == SQLITE_ROW
        } {
            let c2rust_fresh0 = count;
            count += 1;
            populate(statement, views.offset(c2rust_fresh0 as isize));
        }
    }
    if result != SQLITE_DONE && result != SQLITE_ROW {
        count = -1 as ::core::ffi::c_int;
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_fleet_saved_view_get_visible(
    mut owner_user_id: int64_t,
    mut uuid: *const ::core::ffi::c_char,
    mut view: *mut fleet_saved_view_t,
) -> db_fleet_saved_view_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || mutex.is_null()
        || view.is_null()
        || owner_user_id < 0 as int64_t
        || !lightnvr_uuid_is_valid(uuid)
    {
        return db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut result: db_fleet_saved_view_result_t =
        get_locked(db, owner_user_id, uuid, r#true != 0, view);
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_fleet_saved_view_create(
    mut view: *mut fleet_saved_view_t,
) -> db_fleet_saved_view_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    let mut normalized_name: [::core::ffi::c_char; 128] = [0; 128];
    if db.is_null()
        || mutex.is_null()
        || !valid_view(view, r#false != 0)
        || copy_trimmed_value(
            &raw mut normalized_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            &raw mut (*view).name as *mut ::core::ffi::c_char,
            0 as size_t,
        ) == 0 as size_t
        || lightnvr_uuid_generate_v4(&raw mut (*view).uuid as *mut ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
    {
        return db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut count_sql: *const ::core::ffi::c_char = b"SELECT count(*) FROM fleet_saved_views WHERE ((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        count_sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if result == SQLITE_OK {
        bind_owner(statement, 1 as ::core::ffi::c_int, (*view).owner_user_id);
        bind_owner(statement, 2 as ::core::ffi::c_int, (*view).owner_user_id);
        if sqlite3_step(statement) == SQLITE_ROW {
            count = sqlite3_column_int(statement, 0 as ::core::ffi::c_int);
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if count < 0 as ::core::ffi::c_int || count >= FLEET_SAVED_VIEW_MAX_VISIBLE {
        pthread_mutex_unlock(mutex);
        return db_fleet_saved_view_result_t(if count < 0 as ::core::ffi::c_int {
            db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_ERROR.0
        } else {
            db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_LIMIT.0
        });
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO fleet_saved_views(uuid,owner_user_id,name,is_shared,selector_json,search_text,collection_uuid,columns_json,sort_by,sort_order) VALUES(?,?,?,?,?,?,?,?,?,?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    result = sqlite3_prepare_v2(
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
            &raw mut (*view).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if (*view).owner_user_id > 0 as int64_t {
            sqlite3_bind_int64(
                statement,
                2 as ::core::ffi::c_int,
                (*view).owner_user_id as sqlite3_int64,
            );
        } else {
            sqlite3_bind_null(statement, 2 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            &raw mut normalized_name as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            4 as ::core::ffi::c_int,
            if (*view).is_shared as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            statement,
            5 as ::core::ffi::c_int,
            &raw mut (*view).selector_json as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            6 as ::core::ffi::c_int,
            &raw mut (*view).search as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if (*view).collection_uuid[0usize] != 0 {
            sqlite3_bind_text(
                statement,
                7 as ::core::ffi::c_int,
                &raw mut (*view).collection_uuid as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(statement, 7 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            statement,
            8 as ::core::ffi::c_int,
            &raw mut (*view).columns_json as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            9 as ::core::ffi::c_int,
            &raw mut (*view).sort_by as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            10 as ::core::ffi::c_int,
            &raw mut (*view).sort_order as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    if result != SQLITE_DONE {
        pthread_mutex_unlock(mutex);
        return db_fleet_saved_view_result_t(if result == SQLITE_CONSTRAINT {
            db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_CONFLICT.0
        } else {
            db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_ERROR.0
        });
    }
    safe_strcpy(
        &raw mut (*view).name as *mut ::core::ffi::c_char,
        &raw mut normalized_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    let mut outcome: db_fleet_saved_view_result_t = get_locked(
        db,
        (*view).owner_user_id,
        &raw mut (*view).uuid as *mut ::core::ffi::c_char,
        r#false != 0,
        view,
    );
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_fleet_saved_view_update(
    mut view: *mut fleet_saved_view_t,
    mut expected_revision: int64_t,
) -> db_fleet_saved_view_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    let mut normalized_name: [::core::ffi::c_char; 128] = [0; 128];
    if db.is_null()
        || mutex.is_null()
        || expected_revision < 1 as int64_t
        || !valid_view(view, r#true != 0)
        || copy_trimmed_value(
            &raw mut normalized_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            &raw mut (*view).name as *mut ::core::ffi::c_char,
            0 as size_t,
        ) == 0 as size_t
    {
        return db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut existing: fleet_saved_view_t = fleet_saved_view_t {
        uuid: [0; 37],
        owner_user_id: 0,
        name: [0; 128],
        is_shared: false,
        selector_json: [0; 4096],
        search: [0; 256],
        collection_uuid: [0; 37],
        columns_json: [0; 1024],
        sort_by: [0; 32],
        sort_order: [0; 8],
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut outcome: db_fleet_saved_view_result_t = get_locked(
        db,
        (*view).owner_user_id,
        &raw mut (*view).uuid as *mut ::core::ffi::c_char,
        r#false != 0,
        &raw mut existing,
    );
    if outcome.0 != db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_OK.0 {
        pthread_mutex_unlock(mutex);
        return outcome;
    }
    if existing.revision != expected_revision {
        pthread_mutex_unlock(mutex);
        return db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_STALE;
    }
    let mut sql: *const ::core::ffi::c_char = b"UPDATE fleet_saved_views SET name=?,is_shared=?,selector_json=?,search_text=?,collection_uuid=?,columns_json=?,sort_by=?,sort_order=?,revision=revision+1,updated_at=strftime('%s','now') WHERE uuid=? AND revision=? AND ((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);\0"
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
            &raw mut normalized_name as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            statement,
            2 as ::core::ffi::c_int,
            if (*view).is_shared as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_text(
            statement,
            3 as ::core::ffi::c_int,
            &raw mut (*view).selector_json as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            4 as ::core::ffi::c_int,
            &raw mut (*view).search as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if (*view).collection_uuid[0usize] != 0 {
            sqlite3_bind_text(
                statement,
                5 as ::core::ffi::c_int,
                &raw mut (*view).collection_uuid as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(statement, 5 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            statement,
            6 as ::core::ffi::c_int,
            &raw mut (*view).columns_json as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            7 as ::core::ffi::c_int,
            &raw mut (*view).sort_by as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            8 as ::core::ffi::c_int,
            &raw mut (*view).sort_order as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            9 as ::core::ffi::c_int,
            &raw mut (*view).uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            statement,
            10 as ::core::ffi::c_int,
            expected_revision as sqlite3_int64,
        );
        bind_owner(statement, 11 as ::core::ffi::c_int, (*view).owner_user_id);
        bind_owner(statement, 12 as ::core::ffi::c_int, (*view).owner_user_id);
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
        if result == SQLITE_CONSTRAINT {
            return db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_CONFLICT;
        }
        return db_fleet_saved_view_result_t(if result == SQLITE_DONE {
            db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_STALE.0
        } else {
            db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_ERROR.0
        });
    }
    outcome = get_locked(
        db,
        (*view).owner_user_id,
        &raw mut (*view).uuid as *mut ::core::ffi::c_char,
        r#false != 0,
        view,
    );
    pthread_mutex_unlock(mutex);
    return outcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_fleet_saved_view_delete(
    mut owner_user_id: int64_t,
    mut uuid: *const ::core::ffi::c_char,
    mut expected_revision: int64_t,
) -> db_fleet_saved_view_result_t {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null()
        || mutex.is_null()
        || owner_user_id < 0 as int64_t
        || expected_revision < 1 as int64_t
        || !lightnvr_uuid_is_valid(uuid)
    {
        return db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_INVALID;
    }
    pthread_mutex_lock(mutex);
    let mut existing: fleet_saved_view_t = fleet_saved_view_t {
        uuid: [0; 37],
        owner_user_id: 0,
        name: [0; 128],
        is_shared: false,
        selector_json: [0; 4096],
        search: [0; 256],
        collection_uuid: [0; 37],
        columns_json: [0; 1024],
        sort_by: [0; 32],
        sort_order: [0; 8],
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut outcome: db_fleet_saved_view_result_t =
        get_locked(db, owner_user_id, uuid, r#false != 0, &raw mut existing);
    if outcome.0 != db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_OK.0
        || existing.revision != expected_revision
    {
        pthread_mutex_unlock(mutex);
        return db_fleet_saved_view_result_t(
            if outcome.0 == db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_OK.0 {
                db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_STALE.0
            } else {
                outcome.0
            },
        );
    }
    let mut sql: *const ::core::ffi::c_char = b"DELETE FROM fleet_saved_views WHERE uuid=? AND revision=? AND ((?=0 AND owner_user_id IS NULL) OR owner_user_id=?);\0"
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
        bind_owner(statement, 3 as ::core::ffi::c_int, owner_user_id);
        bind_owner(statement, 4 as ::core::ffi::c_int, owner_user_id);
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
        return db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_ERROR;
    }
    return db_fleet_saved_view_result_t(if changed == 1 as ::core::ffi::c_int {
        db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_OK.0
    } else {
        db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_STALE.0
    });
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
