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
    unsafe fn sqlite3_sql(pStmt: *mut sqlite3_stmt) -> *const ::core::ffi::c_char;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_next_stmt(pDb: *mut sqlite3, pStmt: *mut sqlite3_stmt) -> *mut sqlite3_stmt;
    unsafe fn column_exists(
        table_name: *const ::core::ffi::c_char,
        column_name: *const ::core::ffi::c_char,
    ) -> bool;
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const PTHREAD_MUTEX_TIMED_NP: Self = Self(0);
    pub const PTHREAD_MUTEX_RECURSIVE_NP: Self = Self(1);
    pub const PTHREAD_MUTEX_ERRORCHECK_NP: Self = Self(2);
    pub const PTHREAD_MUTEX_ADAPTIVE_NP: Self = Self(3);
    pub const PTHREAD_MUTEX_NORMAL: Self = Self(0);
    pub const PTHREAD_MUTEX_RECURSIVE: Self = Self(1);
    pub const PTHREAD_MUTEX_ERRORCHECK: Self = Self(2);
    pub const PTHREAD_MUTEX_DEFAULT: Self = Self(0);
    pub const PTHREAD_MUTEX_FAST_NP: Self = Self(0);
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
pub struct column_cache_entry_t {
    pub table_name: [::core::ffi::c_char; 64],
    pub column_name: [::core::ffi::c_char; 64],
    pub exists: bool,
}
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
static mut column_cache: *mut column_cache_entry_t =
    ::core::ptr::null_mut::<column_cache_entry_t>();
static mut column_cache_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut column_cache_capacity: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut column_cache_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut schema_initialized: bool = r#false != 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_schema_cache() {
    pthread_mutex_lock(&raw mut column_cache_mutex);
    if !schema_initialized {
        column_cache_capacity = 32 as ::core::ffi::c_int;
        column_cache = malloc(
            (column_cache_capacity as size_t)
                .wrapping_mul(::core::mem::size_of::<column_cache_entry_t>()),
        ) as *mut column_cache_entry_t;
        if column_cache.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to allocate memory for column cache\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            pthread_mutex_unlock(&raw mut column_cache_mutex);
            return;
        }
        column_cache_size = 0 as ::core::ffi::c_int;
        let mut detection_exists: bool = column_exists(
            b"streams\0".as_ptr() as *const ::core::ffi::c_char,
            b"detection_based_recording\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut protocol_exists: bool = column_exists(
            b"streams\0".as_ptr() as *const ::core::ffi::c_char,
            b"protocol\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut onvif_exists: bool = column_exists(
            b"streams\0".as_ptr() as *const ::core::ffi::c_char,
            b"is_onvif\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut record_audio_exists: bool = column_exists(
            b"streams\0".as_ptr() as *const ::core::ffi::c_char,
            b"record_audio\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if column_cache_size < column_cache_capacity {
            safe_strcpy(
                &raw mut (*column_cache.offset(column_cache_size as isize)).table_name
                    as *mut ::core::ffi::c_char,
                b"streams\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*column_cache.offset(column_cache_size as isize)).column_name
                    as *mut ::core::ffi::c_char,
                b"detection_based_recording\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
            (*column_cache.offset(column_cache_size as isize)).exists = detection_exists;
            column_cache_size += 1;
        }
        if column_cache_size < column_cache_capacity {
            safe_strcpy(
                &raw mut (*column_cache.offset(column_cache_size as isize)).table_name
                    as *mut ::core::ffi::c_char,
                b"streams\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*column_cache.offset(column_cache_size as isize)).column_name
                    as *mut ::core::ffi::c_char,
                b"protocol\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
            (*column_cache.offset(column_cache_size as isize)).exists = protocol_exists;
            column_cache_size += 1;
        }
        if column_cache_size < column_cache_capacity {
            safe_strcpy(
                &raw mut (*column_cache.offset(column_cache_size as isize)).table_name
                    as *mut ::core::ffi::c_char,
                b"streams\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*column_cache.offset(column_cache_size as isize)).column_name
                    as *mut ::core::ffi::c_char,
                b"is_onvif\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
            (*column_cache.offset(column_cache_size as isize)).exists = onvif_exists;
            column_cache_size += 1;
        }
        if column_cache_size < column_cache_capacity {
            safe_strcpy(
                &raw mut (*column_cache.offset(column_cache_size as isize)).table_name
                    as *mut ::core::ffi::c_char,
                b"streams\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*column_cache.offset(column_cache_size as isize)).column_name
                    as *mut ::core::ffi::c_char,
                b"record_audio\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                0 as size_t,
            );
            (*column_cache.offset(column_cache_size as isize)).exists = record_audio_exists;
            column_cache_size += 1;
        }
        schema_initialized = r#true != 0;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Schema cache initialized with %d entries\0".as_ptr() as *const ::core::ffi::c_char,
            column_cache_size,
        );
    }
    pthread_mutex_unlock(&raw mut column_cache_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cached_column_exists(
    mut table_name: *const ::core::ffi::c_char,
    mut column_name: *const ::core::ffi::c_char,
) -> bool {
    if table_name.is_null() || column_name.is_null() {
        return r#false != 0;
    }
    if !schema_initialized {
        return column_exists(table_name, column_name);
    }
    pthread_mutex_lock(&raw mut column_cache_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < column_cache_size {
        if strcmp(
            &raw mut (*column_cache.offset(i as isize)).table_name as *mut ::core::ffi::c_char,
            table_name,
        ) == 0 as ::core::ffi::c_int
            && strcmp(
                &raw mut (*column_cache.offset(i as isize)).column_name as *mut ::core::ffi::c_char,
                column_name,
            ) == 0 as ::core::ffi::c_int
        {
            let mut exists: bool = (*column_cache.offset(i as isize)).exists;
            pthread_mutex_unlock(&raw mut column_cache_mutex);
            return exists;
        }
        i += 1;
    }
    let mut exists_0: bool = column_exists(table_name, column_name);
    if column_cache_size >= column_cache_capacity {
        let mut new_capacity: ::core::ffi::c_int = column_cache_capacity * 2 as ::core::ffi::c_int;
        let mut new_cache: *mut column_cache_entry_t = realloc(
            column_cache as *mut ::core::ffi::c_void,
            (new_capacity as size_t).wrapping_mul(::core::mem::size_of::<column_cache_entry_t>()),
        ) as *mut column_cache_entry_t;
        if new_cache.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to resize column cache\0".as_ptr() as *const ::core::ffi::c_char,
            );
            pthread_mutex_unlock(&raw mut column_cache_mutex);
            return exists_0;
        }
        column_cache = new_cache;
        column_cache_capacity = new_capacity;
    }
    safe_strcpy(
        &raw mut (*column_cache.offset(column_cache_size as isize)).table_name
            as *mut ::core::ffi::c_char,
        table_name,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*column_cache.offset(column_cache_size as isize)).column_name
            as *mut ::core::ffi::c_char,
        column_name,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    (*column_cache.offset(column_cache_size as isize)).exists = exists_0;
    column_cache_size += 1;
    pthread_mutex_unlock(&raw mut column_cache_mutex);
    return exists_0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_schema_cache() {
    pthread_mutex_lock(&raw mut column_cache_mutex);
    if !column_cache.is_null() {
        free(column_cache as *mut ::core::ffi::c_void);
        column_cache = ::core::ptr::null_mut::<column_cache_entry_t>();
        column_cache_size = 0 as ::core::ffi::c_int;
        column_cache_capacity = 0 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if !db_mutex.is_null() {
        pthread_mutex_lock(db_mutex);
        db = get_db_handle();
        if !db.is_null() {
            sqlite3_exec(
                db,
                b"PRAGMA schema_version;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            loop {
                stmt = sqlite3_next_stmt(db, ::core::ptr::null_mut::<sqlite3_stmt>());
                if stmt.is_null() {
                    break;
                }
                let mut sql: *const ::core::ffi::c_char = sqlite3_sql(stmt);
                if !sql.is_null()
                    && (!strstr(sql, b"PRAGMA\0".as_ptr() as *const ::core::ffi::c_char).is_null()
                        || !strstr(
                            sql,
                            b"sqlite_master\0".as_ptr() as *const ::core::ffi::c_char,
                        )
                        .is_null())
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_INFO,
                        LOG_COMPONENT,
                        _log_stream_name,
                        b"Finalizing schema-related statement: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        sql,
                    );
                    sqlite3_finalize(stmt);
                }
            }
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Database handle not available during schema cache cleanup\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        pthread_mutex_unlock(db_mutex);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database mutex not available during schema cache cleanup\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    schema_initialized = r#false != 0;
    pthread_mutex_unlock(&raw mut column_cache_mutex);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
