unsafe extern "C" {
    unsafe fn pthread_self() -> pthread_t;
    unsafe fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
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
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct go2rtc_lifecycle_operation_t(pub ::core::ffi::c_uint);
impl go2rtc_lifecycle_operation_t {
    pub const GO2RTC_LIFECYCLE_CHECK: Self = Self(0);
    pub const GO2RTC_LIFECYCLE_PROCESS_START: Self = Self(1);
    pub const GO2RTC_LIFECYCLE_PROCESS_STOP: Self = Self(2);
    pub const GO2RTC_LIFECYCLE_FULL_START: Self = Self(3);
    pub const GO2RTC_LIFECYCLE_RESTART: Self = Self(4);
    pub const GO2RTC_LIFECYCLE_RECONFIGURE: Self = Self(5);
    pub const GO2RTC_LIFECYCLE_CLEANUP: Self = Self(6);
    pub const GO2RTC_LIFECYCLE_OPERATION_COUNT: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct go2rtc_lifecycle_guard_t {
    pub acquired: bool,
    pub outermost: bool,
    pub coalesced: bool,
    pub result: bool,
    pub generation: uint64_t,
    pub restart_generation: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct go2rtc_lifecycle_state_t {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub active: bool,
    pub owner: pthread_t,
    pub depth: ::core::ffi::c_uint,
    pub operation: go2rtc_lifecycle_operation_t,
    pub intentional: bool,
    pub restart_occurred: bool,
    pub generation: uint64_t,
    pub restart_generation: uint64_t,
    pub operation_result: [bool; 7],
    pub operation_generation: [uint64_t; 7],
}
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __data: __pthread_cond_s {
        __wseq: __atomic_wide_counter {
            __value64: 0 as ::core::ffi::c_ulonglong,
        },
        __g1_start: __atomic_wide_counter {
            __value64: 0 as ::core::ffi::c_ulonglong,
        },
        __g_size: [0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint],
        __g1_orig_size: 0 as ::core::ffi::c_uint,
        __wrefs: 0 as ::core::ffi::c_uint,
        __g_signals: [0 as ::core::ffi::c_uint, 0 as ::core::ffi::c_uint],
        __unused_initialized_1: 0 as ::core::ffi::c_uint,
        __unused_initialized_2: 0 as ::core::ffi::c_uint,
    },
};
#[inline]
unsafe extern "C" fn pthread_equal(
    mut __thread1: pthread_t,
    mut __thread2: pthread_t,
) -> ::core::ffi::c_int {
    return (__thread1 == __thread2) as ::core::ffi::c_int;
}
static mut g_lifecycle: go2rtc_lifecycle_state_t = go2rtc_lifecycle_state_t {
    mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0 as ::core::ffi::c_int,
            __count: 0 as ::core::ffi::c_uint,
            __owner: 0 as ::core::ffi::c_int,
            __nusers: 0 as ::core::ffi::c_uint,
            __kind: C2Rust_Unnamed_0::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
            __spins: 0 as ::core::ffi::c_short,
            __glibc_reserved: 0 as ::core::ffi::c_short,
            __list: __pthread_internal_list {
                __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
            },
        },
    },
    cond: PTHREAD_COND_INITIALIZER,
    active: false,
    owner: 0,
    depth: 0,
    operation: go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_CHECK,
    intentional: false,
    restart_occurred: false,
    generation: 0,
    restart_generation: 0,
    operation_result: [false; 7],
    operation_generation: [0; 7],
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_lifecycle_begin(
    mut operation: go2rtc_lifecycle_operation_t,
    mut coalesce: bool,
    mut intentional: bool,
    mut guard: *mut go2rtc_lifecycle_guard_t,
) -> bool {
    if guard.is_null()
        || operation.0 < go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_CHECK.0
        || operation.0 >= go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_OPERATION_COUNT.0
    {
        return r#false != 0;
    }
    memset(
        guard as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<go2rtc_lifecycle_guard_t>(),
    );
    let mut self_0: pthread_t = pthread_self();
    if pthread_mutex_lock(&raw mut g_lifecycle.mutex) != 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    if g_lifecycle.active as ::core::ffi::c_int != 0
        && pthread_equal(g_lifecycle.owner, self_0) != 0
    {
        g_lifecycle.depth = g_lifecycle.depth.wrapping_add(1);
        if operation.0 == go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_RESTART.0
            || operation.0 == go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_FULL_START.0
        {
            g_lifecycle.restart_occurred = r#true != 0;
        }
        if intentional {
            g_lifecycle.intentional = r#true != 0;
        }
        (*guard).acquired = r#true != 0;
        (*guard).generation = g_lifecycle.generation;
        (*guard).restart_generation = g_lifecycle.restart_generation;
        pthread_mutex_unlock(&raw mut g_lifecycle.mutex);
        return r#true != 0;
    }
    if coalesce as ::core::ffi::c_int != 0
        && g_lifecycle.active as ::core::ffi::c_int != 0
        && g_lifecycle.operation.0 == operation.0
    {
        let mut active_generation: uint64_t =
            g_lifecycle.operation_generation[operation.0 as usize];
        while g_lifecycle.operation_generation[operation.0 as usize] == active_generation {
            pthread_cond_wait(&raw mut g_lifecycle.cond, &raw mut g_lifecycle.mutex);
        }
        (*guard).coalesced = r#true != 0;
        (*guard).result = g_lifecycle.operation_result[operation.0 as usize];
        (*guard).generation = g_lifecycle.generation;
        (*guard).restart_generation = g_lifecycle.restart_generation;
        pthread_mutex_unlock(&raw mut g_lifecycle.mutex);
        return r#true != 0;
    }
    while g_lifecycle.active {
        pthread_cond_wait(&raw mut g_lifecycle.cond, &raw mut g_lifecycle.mutex);
    }
    g_lifecycle.active = r#true != 0;
    g_lifecycle.owner = self_0;
    g_lifecycle.depth = 1 as ::core::ffi::c_uint;
    g_lifecycle.operation = operation;
    g_lifecycle.intentional = intentional;
    g_lifecycle.restart_occurred = operation.0
        == go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_RESTART.0
        || operation.0 == go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_FULL_START.0;
    (*guard).acquired = r#true != 0;
    (*guard).outermost = r#true != 0;
    (*guard).generation = g_lifecycle.generation;
    (*guard).restart_generation = g_lifecycle.restart_generation;
    pthread_mutex_unlock(&raw mut g_lifecycle.mutex);
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_lifecycle_try_begin(
    mut operation: go2rtc_lifecycle_operation_t,
    mut intentional: bool,
    mut guard: *mut go2rtc_lifecycle_guard_t,
) -> bool {
    if guard.is_null()
        || operation.0 < go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_CHECK.0
        || operation.0 >= go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_OPERATION_COUNT.0
    {
        return r#false != 0;
    }
    memset(
        guard as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<go2rtc_lifecycle_guard_t>(),
    );
    let mut self_0: pthread_t = pthread_self();
    if pthread_mutex_trylock(&raw mut g_lifecycle.mutex) != 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    if g_lifecycle.active {
        if pthread_equal(g_lifecycle.owner, self_0) == 0 {
            pthread_mutex_unlock(&raw mut g_lifecycle.mutex);
            return r#false != 0;
        }
        g_lifecycle.depth = g_lifecycle.depth.wrapping_add(1);
        if operation.0 == go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_RESTART.0
            || operation.0 == go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_FULL_START.0
        {
            g_lifecycle.restart_occurred = r#true != 0;
        }
        if intentional {
            g_lifecycle.intentional = r#true != 0;
        }
        (*guard).acquired = r#true != 0;
        (*guard).generation = g_lifecycle.generation;
        (*guard).restart_generation = g_lifecycle.restart_generation;
        pthread_mutex_unlock(&raw mut g_lifecycle.mutex);
        return r#true != 0;
    }
    g_lifecycle.active = r#true != 0;
    g_lifecycle.owner = self_0;
    g_lifecycle.depth = 1 as ::core::ffi::c_uint;
    g_lifecycle.operation = operation;
    g_lifecycle.intentional = intentional;
    g_lifecycle.restart_occurred = operation.0
        == go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_RESTART.0
        || operation.0 == go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_FULL_START.0;
    (*guard).acquired = r#true != 0;
    (*guard).outermost = r#true != 0;
    (*guard).generation = g_lifecycle.generation;
    (*guard).restart_generation = g_lifecycle.restart_generation;
    pthread_mutex_unlock(&raw mut g_lifecycle.mutex);
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_lifecycle_end(
    mut guard: *mut go2rtc_lifecycle_guard_t,
    mut result: bool,
) {
    if guard.is_null() || !(*guard).acquired {
        return;
    }
    pthread_mutex_lock(&raw mut g_lifecycle.mutex);
    if !g_lifecycle.active
        || pthread_equal(g_lifecycle.owner, pthread_self()) == 0
        || g_lifecycle.depth == 0 as ::core::ffi::c_uint
    {
        pthread_mutex_unlock(&raw mut g_lifecycle.mutex);
        return;
    }
    g_lifecycle.depth = g_lifecycle.depth.wrapping_sub(1);
    if g_lifecycle.depth == 0 as ::core::ffi::c_uint {
        let mut completed_operation: go2rtc_lifecycle_operation_t = g_lifecycle.operation;
        g_lifecycle.active = r#false != 0;
        g_lifecycle.intentional = r#false != 0;
        g_lifecycle.generation = g_lifecycle.generation.wrapping_add(1);
        g_lifecycle.operation_result[completed_operation.0 as usize] = result;
        g_lifecycle.operation_generation[completed_operation.0 as usize] =
            g_lifecycle.operation_generation[completed_operation.0 as usize].wrapping_add(1);
        if g_lifecycle.restart_occurred as ::core::ffi::c_int != 0
            && result as ::core::ffi::c_int != 0
        {
            g_lifecycle.restart_generation = g_lifecycle.restart_generation.wrapping_add(1);
        }
        g_lifecycle.restart_occurred = r#false != 0;
        pthread_cond_broadcast(&raw mut g_lifecycle.cond);
    }
    pthread_mutex_unlock(&raw mut g_lifecycle.mutex);
    (*guard).acquired = r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_lifecycle_intentional_restart_active() -> bool {
    pthread_mutex_lock(&raw mut g_lifecycle.mutex);
    let mut active: bool = g_lifecycle.active as ::core::ffi::c_int != 0
        && g_lifecycle.intentional as ::core::ffi::c_int != 0
        && g_lifecycle.operation.0 != go2rtc_lifecycle_operation_t::GO2RTC_LIFECYCLE_CHECK.0;
    pthread_mutex_unlock(&raw mut g_lifecycle.mutex);
    return active;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_lifecycle_generation() -> uint64_t {
    pthread_mutex_lock(&raw mut g_lifecycle.mutex);
    let mut generation: uint64_t = g_lifecycle.generation;
    pthread_mutex_unlock(&raw mut g_lifecycle.mutex);
    return generation;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_lifecycle_restart_generation() -> uint64_t {
    pthread_mutex_lock(&raw mut g_lifecycle.mutex);
    let mut generation: uint64_t = g_lifecycle.restart_generation;
    pthread_mutex_unlock(&raw mut g_lifecycle.mutex);
    return generation;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
