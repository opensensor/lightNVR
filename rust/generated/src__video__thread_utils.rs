unsafe extern "C" {
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_setcancelstate(
        __state: ::core::ffi::c_int,
        __oldstate: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_setcanceltype(
        __type: ::core::ffi::c_int,
        __oldtype: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_cancel(__th: pthread_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
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
pub union __atomic_wide_counter {
    pub __value64: ::core::ffi::c_ulonglong,
    pub __value32: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
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
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_size: [::core::ffi::c_uint; 2],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2],
    pub __unused_initialized_1: ::core::ffi::c_uint,
    pub __unused_initialized_2: ::core::ffi::c_uint,
}
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
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
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::core::ffi::c_char; 48],
    pub __align: ::core::ffi::c_longlong,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const PTHREAD_CREATE_JOINABLE: Self = Self(0);
    pub const PTHREAD_CREATE_DETACHED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const PTHREAD_CANCEL_ENABLE: Self = Self(0);
    pub const PTHREAD_CANCEL_DISABLE: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const PTHREAD_CANCEL_DEFERRED: Self = Self(0);
    pub const PTHREAD_CANCEL_ASYNCHRONOUS: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct join_helper_data_t {
    pub thread: pthread_t,
    pub retval: *mut *mut ::core::ffi::c_void,
    pub result: ::core::ffi::c_int,
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub done: ::core::ffi::c_int,
    pub cancelled: ::core::ffi::c_int,
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
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
unsafe extern "C" fn join_helper(mut arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let mut data: *mut join_helper_data_t = arg as *mut join_helper_data_t;
    pthread_setcancelstate(
        C2Rust_Unnamed_1::PTHREAD_CANCEL_ENABLE.0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    pthread_setcanceltype(
        C2Rust_Unnamed_2::PTHREAD_CANCEL_DEFERRED.0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    pthread_mutex_lock(&raw mut (*data).mutex);
    if (*data).cancelled != 0 {
        pthread_mutex_unlock(&raw mut (*data).mutex);
        return NULL;
    }
    pthread_mutex_unlock(&raw mut (*data).mutex);
    (*data).result = pthread_join((*data).thread, (*data).retval);
    pthread_mutex_lock(&raw mut (*data).mutex);
    (*data).done = 1 as ::core::ffi::c_int;
    pthread_cond_signal(&raw mut (*data).cond);
    pthread_mutex_unlock(&raw mut (*data).mutex);
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pthread_join_with_timeout(
    mut thread: pthread_t,
    mut retval: *mut *mut ::core::ffi::c_void,
    mut timeout_sec: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut timeout_thread: pthread_t = 0;
    let mut data: *mut join_helper_data_t =
        calloc(1 as size_t, ::core::mem::size_of::<join_helper_data_t>())
            as *mut join_helper_data_t;
    if data.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for join helper data\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ENOMEM;
    }
    (*data).thread = thread;
    (*data).retval = retval;
    (*data).result = -1 as ::core::ffi::c_int;
    (*data).done = 0 as ::core::ffi::c_int;
    (*data).cancelled = 0 as ::core::ffi::c_int;
    if pthread_mutex_init(
        &raw mut (*data).mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize mutex for join helper\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(data as *mut ::core::ffi::c_void);
        return EAGAIN;
    }
    if pthread_cond_init(
        &raw mut (*data).cond,
        ::core::ptr::null::<pthread_condattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize condition variable for join helper\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        pthread_mutex_destroy(&raw mut (*data).mutex);
        free(data as *mut ::core::ffi::c_void);
        return EAGAIN;
    }
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    pthread_attr_init(&raw mut attr);
    pthread_attr_setdetachstate(
        &raw mut attr,
        C2Rust_Unnamed_0::PTHREAD_CREATE_JOINABLE.0 as ::core::ffi::c_int,
    );
    if pthread_create(
        &raw mut timeout_thread,
        &raw mut attr,
        Some(
            join_helper
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        data as *mut ::core::ffi::c_void,
    ) != 0 as ::core::ffi::c_int
    {
        pthread_attr_destroy(&raw mut attr);
        pthread_mutex_destroy(&raw mut (*data).mutex);
        pthread_cond_destroy(&raw mut (*data).cond);
        free(data as *mut ::core::ffi::c_void);
        return EAGAIN;
    }
    pthread_attr_destroy(&raw mut attr);
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_REALTIME, &raw mut ts);
    ts.tv_sec += timeout_sec as __time_t;
    pthread_mutex_lock(&raw mut (*data).mutex);
    while (*data).done == 0 {
        ret = pthread_cond_timedwait(&raw mut (*data).cond, &raw mut (*data).mutex, &raw mut ts);
        if ret == ETIMEDOUT {
            (*data).cancelled = 1 as ::core::ffi::c_int;
            pthread_mutex_unlock(&raw mut (*data).mutex);
            let mut short_wait: timespec = timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            clock_gettime(CLOCK_REALTIME, &raw mut short_wait);
            short_wait.tv_sec += 1 as __time_t;
            pthread_mutex_lock(&raw mut (*data).mutex);
            if (*data).done == 0 {
                pthread_cond_timedwait(
                    &raw mut (*data).cond,
                    &raw mut (*data).mutex,
                    &raw mut short_wait,
                );
            }
            pthread_mutex_unlock(&raw mut (*data).mutex);
            if (*data).done == 0 {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Thread join helper did not complete in time, cancelling\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                let mut old_type: ::core::ffi::c_int = 0;
                pthread_setcanceltype(
                    C2Rust_Unnamed_2::PTHREAD_CANCEL_DEFERRED.0 as ::core::ffi::c_int,
                    &raw mut old_type,
                );
                pthread_cancel(timeout_thread);
                let mut thread_result: *mut ::core::ffi::c_void =
                    ::core::ptr::null_mut::<::core::ffi::c_void>();
                pthread_join(timeout_thread, &raw mut thread_result);
            } else {
                pthread_join(
                    timeout_thread,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
                );
            }
            pthread_mutex_destroy(&raw mut (*data).mutex);
            pthread_cond_destroy(&raw mut (*data).cond);
            free(data as *mut ::core::ffi::c_void);
            return ETIMEDOUT;
        }
    }
    ret = (*data).result;
    pthread_mutex_unlock(&raw mut (*data).mutex);
    pthread_join(
        timeout_thread,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    );
    pthread_mutex_destroy(&raw mut (*data).mutex);
    pthread_cond_destroy(&raw mut (*data).cond);
    free(data as *mut ::core::ffi::c_void);
    return ret;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
