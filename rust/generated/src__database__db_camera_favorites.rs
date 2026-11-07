use ::libc;
unsafe extern "C" {
    pub type sqlite3_stmt;
    pub type sqlite3;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
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
    unsafe fn sqlite3_column_int64(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int)
    -> sqlite3_int64;
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
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
pub struct camera_favorite_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub created_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_camera_favorite_result_t(pub ::core::ffi::c_int);
impl db_camera_favorite_result_t {
    pub const DB_CAMERA_FAVORITE_OK: Self = Self(0);
    pub const DB_CAMERA_FAVORITE_NOT_FOUND: Self = Self(-1);
    pub const DB_CAMERA_FAVORITE_INVALID: Self = Self(-2);
    pub const DB_CAMERA_FAVORITE_ERROR: Self = Self(-3);
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
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
unsafe extern "C" fn scope_id(mut user_id: int64_t) -> int64_t {
    return if user_id > 0 as int64_t {
        user_id
    } else {
        0 as int64_t
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_favorites_list(
    mut user_id: int64_t,
    mut favorites: *mut camera_favorite_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if favorites.is_null() || max_count < 1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    static mut sql: *const ::core::ffi::c_char = b"SELECT camera_uuid,created_at FROM camera_favorites WHERE ((?1=0 AND user_id IS NULL) OR user_id=?1) ORDER BY created_at,camera_uuid LIMIT ?2;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(
            statement,
            1 as ::core::ffi::c_int,
            scope_id(user_id) as sqlite3_int64,
        );
        sqlite3_bind_int(statement, 2 as ::core::ffi::c_int, max_count);
        loop {
            rc = sqlite3_step(statement);
            if !(rc == SQLITE_ROW && count < max_count) {
                break;
            }
            let mut uuid: *const ::core::ffi::c_char =
                sqlite3_column_text(statement, 0 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char;
            safe_strcpy(
                &raw mut (*favorites.offset(count as isize)).camera_uuid
                    as *mut ::core::ffi::c_char,
                if !uuid.is_null() {
                    uuid
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
            (*favorites.offset(count as isize)).created_at =
                sqlite3_column_int64(statement, 1 as ::core::ffi::c_int) as int64_t;
            count += 1;
        }
        if rc == SQLITE_ROW {
            rc = SQLITE_DONE;
        }
    }
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to list camera favorites: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return if rc == SQLITE_DONE {
        count
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_favorite_add(
    mut user_id: int64_t,
    mut camera_uuid: *const ::core::ffi::c_char,
    mut favorite: *mut camera_favorite_t,
) -> db_camera_favorite_result_t {
    if !lightnvr_uuid_is_valid(camera_uuid) {
        return db_camera_favorite_result_t::DB_CAMERA_FAVORITE_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_camera_favorite_result_t::DB_CAMERA_FAVORITE_ERROR;
    }
    static mut insert_sql: *const ::core::ffi::c_char =
        b"INSERT OR IGNORE INTO camera_favorites(user_id,camera_uuid) VALUES(?1,?2);\0".as_ptr()
            as *const ::core::ffi::c_char;
    static mut select_sql: *const ::core::ffi::c_char = b"SELECT camera_uuid,created_at FROM camera_favorites WHERE ((?1=0 AND user_id IS NULL) OR user_id=?1) AND camera_uuid=?2;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut result: db_camera_favorite_result_t =
        db_camera_favorite_result_t::DB_CAMERA_FAVORITE_ERROR;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        insert_sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        if user_id > 0 as int64_t {
            sqlite3_bind_int64(statement, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
        } else {
            sqlite3_bind_null(statement, 1 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            camera_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(statement);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    statement = ::core::ptr::null_mut::<sqlite3_stmt>();
    if rc == SQLITE_CONSTRAINT {
        result = db_camera_favorite_result_t::DB_CAMERA_FAVORITE_NOT_FOUND;
    } else if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to add camera favorite: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    } else {
        rc = sqlite3_prepare_v2(
            db,
            select_sql,
            -1 as ::core::ffi::c_int,
            &raw mut statement,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_int64(
                statement,
                1 as ::core::ffi::c_int,
                scope_id(user_id) as sqlite3_int64,
            );
            sqlite3_bind_text(
                statement,
                2 as ::core::ffi::c_int,
                camera_uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            rc = sqlite3_step(statement);
        }
        if rc == SQLITE_ROW {
            if !favorite.is_null() {
                memset(
                    favorite as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<camera_favorite_t>(),
                );
                let mut uuid: *const ::core::ffi::c_char =
                    sqlite3_column_text(statement, 0 as ::core::ffi::c_int)
                        as *const ::core::ffi::c_char;
                safe_strcpy(
                    &raw mut (*favorite).camera_uuid as *mut ::core::ffi::c_char,
                    if !uuid.is_null() {
                        uuid
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                    0 as size_t,
                );
                (*favorite).created_at =
                    sqlite3_column_int64(statement, 1 as ::core::ffi::c_int) as int64_t;
            }
            result = db_camera_favorite_result_t::DB_CAMERA_FAVORITE_OK;
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to read back camera favorite: %s\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
            );
        }
        if !statement.is_null() {
            sqlite3_finalize(statement);
        }
    }
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_camera_favorite_remove(
    mut user_id: int64_t,
    mut camera_uuid: *const ::core::ffi::c_char,
    mut removed: *mut bool,
) -> db_camera_favorite_result_t {
    if !removed.is_null() {
        *removed = r#false != 0;
    }
    if !lightnvr_uuid_is_valid(camera_uuid) {
        return db_camera_favorite_result_t::DB_CAMERA_FAVORITE_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_camera_favorite_result_t::DB_CAMERA_FAVORITE_ERROR;
    }
    static mut sql: *const ::core::ffi::c_char = b"DELETE FROM camera_favorites WHERE ((?1=0 AND user_id IS NULL) OR user_id=?1) AND camera_uuid=?2;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(
            statement,
            1 as ::core::ffi::c_int,
            scope_id(user_id) as sqlite3_int64,
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            camera_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(statement);
    }
    if rc == SQLITE_DONE {
        if !removed.is_null() {
            *removed = sqlite3_changes(db) > 0 as ::core::ffi::c_int;
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to remove camera favorite: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return db_camera_favorite_result_t(if rc == SQLITE_DONE {
        db_camera_favorite_result_t::DB_CAMERA_FAVORITE_OK.0
    } else {
        db_camera_favorite_result_t::DB_CAMERA_FAVORITE_ERROR.0
    });
}
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
