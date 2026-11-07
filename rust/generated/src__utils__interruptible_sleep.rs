unsafe extern "C" {
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
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
    unsafe fn pthread_condattr_init(__attr: *mut pthread_condattr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_condattr_destroy(__attr: *mut pthread_condattr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_condattr_setclock(
        __attr: *mut pthread_condattr_t,
        __clock_id: __clockid_t,
    ) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct interruptible_sleep_t {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub woken: bool,
    pub clock_id: clockid_t,
}
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn interruptible_sleep_init(
    mut s: *mut interruptible_sleep_t,
) -> ::core::ffi::c_int {
    if s.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    (*s).woken = r#false != 0;
    if pthread_mutex_init(
        &raw mut (*s).mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut attr: pthread_condattr_t = pthread_condattr_t { __size: [0; 4] };
    pthread_condattr_init(&raw mut attr);
    (*s).clock_id = CLOCK_REALTIME as clockid_t;
    if pthread_condattr_setclock(&raw mut attr, CLOCK_MONOTONIC) == 0 as ::core::ffi::c_int {
        (*s).clock_id = CLOCK_MONOTONIC as clockid_t;
    }
    let mut rc: ::core::ffi::c_int = pthread_cond_init(&raw mut (*s).cond, &raw mut attr);
    pthread_condattr_destroy(&raw mut attr);
    if rc != 0 as ::core::ffi::c_int {
        pthread_mutex_destroy(&raw mut (*s).mutex);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn interruptible_sleep_destroy(mut s: *mut interruptible_sleep_t) {
    if s.is_null() {
        return;
    }
    pthread_cond_destroy(&raw mut (*s).cond);
    pthread_mutex_destroy(&raw mut (*s).mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn interruptible_sleep_reset(mut s: *mut interruptible_sleep_t) {
    if s.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut (*s).mutex);
    (*s).woken = r#false != 0;
    pthread_mutex_unlock(&raw mut (*s).mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn interruptible_sleep_wait(
    mut s: *mut interruptible_sleep_t,
    mut seconds: ::core::ffi::c_int,
) {
    if s.is_null() {
        return;
    }
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime((*s).clock_id, &raw mut ts);
    if seconds > 0 as ::core::ffi::c_int {
        ts.tv_sec += seconds as __time_t;
    }
    pthread_mutex_lock(&raw mut (*s).mutex);
    while !(*s).woken {
        let mut rc: ::core::ffi::c_int =
            pthread_cond_timedwait(&raw mut (*s).cond, &raw mut (*s).mutex, &raw mut ts);
        if rc == ETIMEDOUT || rc != 0 as ::core::ffi::c_int {
            break;
        }
    }
    (*s).woken = r#false != 0;
    pthread_mutex_unlock(&raw mut (*s).mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn interruptible_sleep_wake(mut s: *mut interruptible_sleep_t) {
    if s.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut (*s).mutex);
    (*s).woken = r#true != 0;
    pthread_cond_signal(&raw mut (*s).cond);
    pthread_mutex_unlock(&raw mut (*s).mutex);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
