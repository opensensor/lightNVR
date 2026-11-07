unsafe extern "C" {
    pub type sqlite3;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
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
    unsafe fn sqlite3_free(_: *mut ::core::ffi::c_void);
    unsafe fn pthread_mutex_timedlock(
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
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
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
}
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn begin_transaction() -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut err_msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
    let mut timeout: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_REALTIME, &raw mut timeout);
    timeout.tv_sec += 5 as __time_t;
    let mut lock_result: ::core::ffi::c_int = pthread_mutex_timedlock(db_mutex, &raw mut timeout);
    if lock_result != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to acquire database mutex for transaction: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            strerror(lock_result),
        );
        return -1 as ::core::ffi::c_int;
    }
    rc = sqlite3_exec(
        db,
        b"BEGIN TRANSACTION;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        &raw mut err_msg,
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to begin transaction: %s\0".as_ptr() as *const ::core::ffi::c_char,
            err_msg,
        );
        sqlite3_free(err_msg as *mut ::core::ffi::c_void);
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn commit_transaction() -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut err_msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    rc = sqlite3_exec(
        db,
        b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        &raw mut err_msg,
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to commit transaction: %s\0".as_ptr() as *const ::core::ffi::c_char,
            err_msg,
        );
        sqlite3_free(err_msg as *mut ::core::ffi::c_void);
        let mut rollback_err: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            &raw mut rollback_err,
        );
        if !rollback_err.is_null() {
            sqlite3_free(rollback_err as *mut ::core::ffi::c_void);
        }
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(db_mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rollback_transaction() -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut err_msg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut db: *mut sqlite3 = get_db_handle();
    let mut db_mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Database not initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        pthread_mutex_unlock(db_mutex);
        return -1 as ::core::ffi::c_int;
    }
    rc = sqlite3_exec(
        db,
        b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        &raw mut err_msg,
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to rollback transaction: %s\0".as_ptr() as *const ::core::ffi::c_char,
            err_msg,
        );
        sqlite3_free(err_msg as *mut ::core::ffi::c_void);
    }
    pthread_mutex_unlock(db_mutex);
    return if rc == SQLITE_OK {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
