use ::libc;
unsafe extern "C" {
    pub type sqlite3_stmt;
    pub type sqlite3;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
    unsafe fn sqlite3_bind_double(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_double,
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
    unsafe fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_clear_bindings(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_double(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
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
    unsafe fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
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
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_detection_engine_t {
    pub id: uint64_t,
    pub engine_key: [::core::ffi::c_char; 64],
    pub engine_type: [::core::ffi::c_char; 16],
    pub model_path: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub threshold: ::core::ffi::c_float,
    pub interval_seconds: ::core::ffi::c_int,
    pub sort_order: ::core::ffi::c_int,
    pub config_json: [::core::ffi::c_char; 1024],
}
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
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
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const DETECTION_ENGINE_KEY_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MAX_DETECTION_ENGINES_PER_STREAM: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOTFOUND: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
unsafe extern "C" fn copy_column(
    mut dst: *mut ::core::ffi::c_char,
    mut dst_size: size_t,
    mut stmt: *mut sqlite3_stmt,
    mut column: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut value: *const ::core::ffi::c_uchar = sqlite3_column_text(stmt, column);
    if dst.is_null() || dst_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    snprintf(
        dst,
        dst_size,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        if !value.is_null() {
            value as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn valid_key(mut key: *const ::core::ffi::c_char) -> bool {
    if key.is_null()
        || *key.offset(0isize) == 0
        || strlen(key) >= DETECTION_ENGINE_KEY_MAX as size_t
    {
        return r#false != 0;
    }
    let mut p: *const ::core::ffi::c_uchar = key as *const ::core::ffi::c_uchar;
    while *p != 0 {
        if !(*(*__ctype_b_loc()).offset(*p as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISlower.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
            || *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
            || *p as ::core::ffi::c_int == '-' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '_' as ::core::ffi::c_int)
        {
            return r#false != 0;
        }
        p = p.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_type(mut r#type: *const ::core::ffi::c_char) -> bool {
    static mut types: [*const ::core::ffi::c_char; 5] = [
        b"motion\0".as_ptr() as *const ::core::ffi::c_char,
        b"object\0".as_ptr() as *const ::core::ffi::c_char,
        b"onvif\0".as_ptr() as *const ::core::ffi::c_char,
        b"api\0".as_ptr() as *const ::core::ffi::c_char,
        b"external\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 5]>()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        if !r#type.is_null() && strcmp(r#type, types[i]) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn validation_error(
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
    mut message: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !error.is_null() && error_size > 0 as size_t {
        snprintf(
            error,
            error_size,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            message,
        );
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_detection_engine_validate(
    mut engine: *const stream_detection_engine_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> ::core::ffi::c_int {
    if engine.is_null() {
        return validation_error(
            error,
            error_size,
            b"engine is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !valid_key(&raw const (*engine).engine_key as *const ::core::ffi::c_char) {
        return validation_error(
            error,
            error_size,
            b"invalid engine_key\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        &raw const (*engine).engine_key as *const ::core::ffi::c_char,
        b"legacy-primary\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return validation_error(
            error,
            error_size,
            b"legacy-primary is managed by stream settings\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !valid_type(&raw const (*engine).engine_type as *const ::core::ffi::c_char) {
        return validation_error(
            error,
            error_size,
            b"invalid engine_type\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*engine).threshold < 0.0f32 || (*engine).threshold > 1.0f32 {
        return validation_error(
            error,
            error_size,
            b"threshold must be between 0 and 1\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*engine).interval_seconds < 1 as ::core::ffi::c_int
        || (*engine).interval_seconds > 86400 as ::core::ffi::c_int
    {
        return validation_error(
            error,
            error_size,
            b"interval_seconds must be between 1 and 86400\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if strlen(&raw const (*engine).model_path as *const ::core::ffi::c_char)
        >= MAX_PATH_LENGTH as size_t
    {
        return validation_error(
            error,
            error_size,
            b"model_path is too long\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        &raw const (*engine).engine_type as *const ::core::ffi::c_char,
        b"object\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && (*engine).model_path[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return validation_error(
            error,
            error_size,
            b"object engines require model_path\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        &raw const (*engine).engine_type as *const ::core::ffi::c_char,
        b"api\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && (*engine).model_path[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return validation_error(
            error,
            error_size,
            b"api engines require model_path\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        &raw const (*engine).engine_type as *const ::core::ffi::c_char,
        b"motion\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && (*engine).model_path[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && strcmp(
            &raw const (*engine).model_path as *const ::core::ffi::c_char,
            b"motion\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        return validation_error(
            error,
            error_size,
            b"motion model_path must be empty or motion\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        &raw const (*engine).engine_type as *const ::core::ffi::c_char,
        b"onvif\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && (*engine).model_path[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && strcmp(
            &raw const (*engine).model_path as *const ::core::ffi::c_char,
            b"onvif\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        return validation_error(
            error,
            error_size,
            b"onvif model_path must be empty or onvif\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut json: *const ::core::ffi::c_char =
        if (*engine).config_json[0usize] as ::core::ffi::c_int != 0 {
            &raw const (*engine).config_json as *const ::core::ffi::c_char
        } else {
            b"{}\0".as_ptr() as *const ::core::ffi::c_char
        };
    let mut parsed: *mut cJSON = cJSON_Parse(json);
    if parsed.is_null() || cJSON_IsObject(parsed) == 0 {
        cJSON_Delete(parsed);
        return validation_error(
            error,
            error_size,
            b"config_json must be a JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_Delete(parsed);
    if !error.is_null() && error_size > 0 as size_t {
        *error.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_detection_engines_list(
    mut stream_name: *const ::core::ffi::c_char,
    mut engines: *mut stream_detection_engine_t,
    mut capacity: size_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null()
        || *stream_name.offset(0isize) == 0
        || engines.is_null() && capacity > 0 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT e.id, e.engine_key, e.engine_type, e.model_path, e.enabled, e.threshold, e.interval_seconds, e.sort_order, e.config_json FROM stream_detection_engines e JOIN streams s ON s.id = e.stream_id WHERE s.name = ? ORDER BY e.sort_order, e.id LIMIT ?;\0"
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
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare detection engine list: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        pthread_mutex_unlock(mutex);
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
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, capacity as sqlite3_int64);
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        rc = sqlite3_step(stmt);
        if !(rc == SQLITE_ROW && (count as size_t) < capacity) {
            break;
        }
        let c2rust_fresh0 = count;
        count += 1;
        let mut engine: *mut stream_detection_engine_t = engines.offset(c2rust_fresh0 as isize);
        memset(
            engine as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<stream_detection_engine_t>(),
        );
        (*engine).id = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as uint64_t;
        copy_column(
            &raw mut (*engine).engine_key as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            stmt,
            1 as ::core::ffi::c_int,
        );
        copy_column(
            &raw mut (*engine).engine_type as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            stmt,
            2 as ::core::ffi::c_int,
        );
        copy_column(
            &raw mut (*engine).model_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            stmt,
            3 as ::core::ffi::c_int,
        );
        (*engine).enabled =
            sqlite3_column_int(stmt, 4 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        (*engine).threshold =
            sqlite3_column_double(stmt, 5 as ::core::ffi::c_int) as ::core::ffi::c_float;
        (*engine).interval_seconds = sqlite3_column_int(stmt, 6 as ::core::ffi::c_int);
        (*engine).sort_order = sqlite3_column_int(stmt, 7 as ::core::ffi::c_int);
        copy_column(
            &raw mut (*engine).config_json as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            stmt,
            8 as ::core::ffi::c_int,
        );
    }
    if rc != SQLITE_DONE {
        count = -1 as ::core::ffi::c_int;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_detection_engine_upsert(
    mut stream_name: *const ::core::ffi::c_char,
    mut engine: *const stream_detection_engine_t,
) -> ::core::ffi::c_int {
    let mut error: [::core::ffi::c_char; 128] = [0; 128];
    if stream_name.is_null()
        || *stream_name.offset(0isize) == 0
        || db_detection_engine_validate(
            engine,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        ) != 0 as ::core::ffi::c_int
    {
        if !engine.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Rejected detection engine configuration: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut error as *mut ::core::ffi::c_char,
            );
        }
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO stream_detection_engines (stream_id, engine_key, engine_type, model_path, enabled, threshold, interval_seconds, sort_order, config_json) SELECT id, ?, ?, ?, ?, ?, ?, ?, ? FROM streams WHERE name = ? ON CONFLICT(stream_id, engine_key) DO UPDATE SET engine_type=excluded.engine_type, model_path=excluded.model_path, enabled=excluded.enabled, threshold=excluded.threshold, interval_seconds=excluded.interval_seconds, sort_order=excluded.sort_order, config_json=excluded.config_json, updated_at=strftime('%s','now');\0"
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
            &raw const (*engine).engine_key as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw const (*engine).engine_type as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            &raw const (*engine).model_path as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            stmt,
            4 as ::core::ffi::c_int,
            if (*engine).enabled as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_double(
            stmt,
            5 as ::core::ffi::c_int,
            (*engine).threshold as ::core::ffi::c_double,
        );
        sqlite3_bind_int(stmt, 6 as ::core::ffi::c_int, (*engine).interval_seconds);
        sqlite3_bind_int(stmt, 7 as ::core::ffi::c_int, (*engine).sort_order);
        sqlite3_bind_text(
            stmt,
            8 as ::core::ffi::c_int,
            if (*engine).config_json[0usize] as ::core::ffi::c_int != 0 {
                &raw const (*engine).config_json as *const ::core::ffi::c_char
            } else {
                b"{}\0".as_ptr() as *const ::core::ffi::c_char
            },
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            9 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
    }
    let mut changed: ::core::ffi::c_int = sqlite3_changes(db);
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return if rc == SQLITE_DONE && changed == 1 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_detection_engine_delete(
    mut stream_name: *const ::core::ffi::c_char,
    mut engine_key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if stream_name.is_null()
        || *stream_name.offset(0isize) == 0
        || !valid_key(engine_key)
        || strcmp(
            engine_key,
            b"legacy-primary\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"DELETE FROM stream_detection_engines WHERE engine_key = ? AND stream_id = (SELECT id FROM streams WHERE name = ?);\0"
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
            engine_key,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
    }
    let mut changed: ::core::ffi::c_int = sqlite3_changes(db);
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return if rc == SQLITE_DONE && changed == 1 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_detection_engines_replace_custom(
    mut stream_name: *const ::core::ffi::c_char,
    mut engines: *const stream_detection_engine_t,
    mut count: size_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null()
        || *stream_name.offset(0isize) == 0
        || engines.is_null() && count > 0 as size_t
        || count >= MAX_DETECTION_ENGINES_PER_STREAM as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut i: size_t = 0 as size_t;
    while i < count {
        if db_detection_engine_validate(
            engines.offset(i as isize),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as size_t,
        ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        let mut previous: size_t = 0 as size_t;
        while previous < i {
            if strcmp(
                &raw const (*engines.offset(i as isize)).engine_key as *const ::core::ffi::c_char,
                &raw const (*engines.offset(previous as isize)).engine_key
                    as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
            previous = previous.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut rc: ::core::ffi::c_int = sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    let mut stream_stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut delete_stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut insert_stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut stream_id: sqlite3_int64 = 0 as sqlite3_int64;
    if rc == SQLITE_OK {
        rc = sqlite3_prepare_v2(
            db,
            b"SELECT id FROM streams WHERE name=?;\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stream_stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
    }
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stream_stmt,
            1 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stream_stmt);
        if rc == SQLITE_ROW {
            stream_id = sqlite3_column_int64(stream_stmt, 0 as ::core::ffi::c_int);
            rc = SQLITE_OK;
        } else {
            rc = SQLITE_NOTFOUND;
        }
    }
    if !stream_stmt.is_null() {
        sqlite3_finalize(stream_stmt);
    }
    if rc == SQLITE_OK {
        rc = sqlite3_prepare_v2(
            db,
            b"DELETE FROM stream_detection_engines WHERE stream_id=? AND engine_key<>'legacy-primary';\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut delete_stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
    }
    if rc == SQLITE_OK {
        sqlite3_bind_int64(delete_stmt, 1 as ::core::ffi::c_int, stream_id);
        rc = if sqlite3_step(delete_stmt) == SQLITE_DONE {
            SQLITE_OK
        } else {
            SQLITE_ERROR
        };
    }
    if !delete_stmt.is_null() {
        sqlite3_finalize(delete_stmt);
    }
    let mut insert_sql: *const ::core::ffi::c_char = b"INSERT INTO stream_detection_engines(stream_id,engine_key,engine_type,model_path,enabled,threshold,interval_seconds,sort_order,config_json) VALUES(?,?,?,?,?,?,?,?,?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    if rc == SQLITE_OK && count > 0 as size_t {
        rc = sqlite3_prepare_v2(
            db,
            insert_sql,
            -1 as ::core::ffi::c_int,
            &raw mut insert_stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
    }
    let mut i_0: size_t = 0 as size_t;
    while rc == SQLITE_OK && i_0 < count {
        sqlite3_reset(insert_stmt);
        sqlite3_clear_bindings(insert_stmt);
        sqlite3_bind_int64(insert_stmt, 1 as ::core::ffi::c_int, stream_id);
        sqlite3_bind_text(
            insert_stmt,
            2 as ::core::ffi::c_int,
            &raw const (*engines.offset(i_0 as isize)).engine_key as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            insert_stmt,
            3 as ::core::ffi::c_int,
            &raw const (*engines.offset(i_0 as isize)).engine_type as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            insert_stmt,
            4 as ::core::ffi::c_int,
            &raw const (*engines.offset(i_0 as isize)).model_path as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(
            insert_stmt,
            5 as ::core::ffi::c_int,
            if (*engines.offset(i_0 as isize)).enabled as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        sqlite3_bind_double(
            insert_stmt,
            6 as ::core::ffi::c_int,
            (*engines.offset(i_0 as isize)).threshold as ::core::ffi::c_double,
        );
        sqlite3_bind_int(
            insert_stmt,
            7 as ::core::ffi::c_int,
            (*engines.offset(i_0 as isize)).interval_seconds,
        );
        sqlite3_bind_int(
            insert_stmt,
            8 as ::core::ffi::c_int,
            (*engines.offset(i_0 as isize)).sort_order,
        );
        sqlite3_bind_text(
            insert_stmt,
            9 as ::core::ffi::c_int,
            if (*engines.offset(i_0 as isize)).config_json[0usize] as ::core::ffi::c_int != 0 {
                &raw const (*engines.offset(i_0 as isize)).config_json as *const ::core::ffi::c_char
            } else {
                b"{}\0".as_ptr() as *const ::core::ffi::c_char
            },
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if sqlite3_step(insert_stmt) != SQLITE_DONE {
            rc = SQLITE_ERROR;
        }
        i_0 = i_0.wrapping_add(1);
    }
    if !insert_stmt.is_null() {
        sqlite3_finalize(insert_stmt);
    }
    if rc == SQLITE_OK {
        if sqlite3_exec(
            db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) != SQLITE_OK
        {
            rc = SQLITE_ERROR;
        }
    } else {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    }
    pthread_mutex_unlock(mutex);
    return if rc == SQLITE_OK {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
