use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn adjtimex(__ntx: *mut timex) -> ::core::ffi::c_int;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct system_health_capability_t(pub ::core::ffi::c_uint);
impl system_health_capability_t {
    pub const SYSTEM_HEALTH_CAPABILITY_AVAILABLE: Self = Self(0);
    pub const SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED: Self = Self(1);
    pub const SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED: Self = Self(2);
    pub const SYSTEM_HEALTH_CAPABILITY_STALE: Self = Self(3);
    pub const SYSTEM_HEALTH_CAPABILITY_ERROR: Self = Self(4);
    pub const SYSTEM_HEALTH_CAPABILITY_COUNT: Self = Self(5);
}
pub type linux_clock_read_time_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut int64_t) -> ::core::ffi::c_int>;
pub type linux_clock_read_sync_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut bool) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_clock_source_t {
    pub context: *mut ::core::ffi::c_void,
    pub realtime: linux_clock_read_time_fn,
    pub monotonic: linux_clock_read_time_fn,
    pub synchronization: linux_clock_read_sync_fn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_clock_sample_t {
    pub capability: system_health_capability_t,
    pub synchronization_known: bool,
    pub synchronized: bool,
    pub realtime_ms: int64_t,
    pub monotonic_ms: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_clock_state_t {
    pub initialized: bool,
    pub startup_monotonic_ms: uint64_t,
    pub previous_realtime_ms: int64_t,
    pub previous_monotonic_ms: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_clock_result_t {
    pub capability: system_health_capability_t,
    pub synchronization_known: bool,
    pub synchronized: bool,
    pub startup_grace_active: bool,
    pub jump_detected: bool,
    pub jump_ms: int64_t,
}
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct timex {
    pub modes: ::core::ffi::c_uint,
    pub offset: __syscall_slong_t,
    pub freq: __syscall_slong_t,
    pub maxerror: __syscall_slong_t,
    pub esterror: __syscall_slong_t,
    pub status: ::core::ffi::c_int,
    pub constant: __syscall_slong_t,
    pub precision: __syscall_slong_t,
    pub tolerance: __syscall_slong_t,
    pub time: timeval,
    pub tick: __syscall_slong_t,
    pub ppsfreq: __syscall_slong_t,
    pub jitter: __syscall_slong_t,
    pub shift: ::core::ffi::c_int,
    pub stabil: __syscall_slong_t,
    pub jitcnt: __syscall_slong_t,
    pub calcnt: __syscall_slong_t,
    pub errcnt: __syscall_slong_t,
    pub stbcnt: __syscall_slong_t,
    pub tai: ::core::ffi::c_int,
    #[bitfield(name = "c2rust_unnamed", ty = "::core::ffi::c_int", bits = "0..=31")]
    #[bitfield(name = "c2rust_unnamed_0", ty = "::core::ffi::c_int", bits = "32..=63")]
    #[bitfield(name = "c2rust_unnamed_1", ty = "::core::ffi::c_int", bits = "64..=95")]
    #[bitfield(
        name = "c2rust_unnamed_2",
        ty = "::core::ffi::c_int",
        bits = "96..=127"
    )]
    #[bitfield(
        name = "c2rust_unnamed_3",
        ty = "::core::ffi::c_int",
        bits = "128..=159"
    )]
    #[bitfield(
        name = "c2rust_unnamed_4",
        ty = "::core::ffi::c_int",
        bits = "160..=191"
    )]
    #[bitfield(
        name = "c2rust_unnamed_5",
        ty = "::core::ffi::c_int",
        bits = "192..=223"
    )]
    #[bitfield(
        name = "c2rust_unnamed_6",
        ty = "::core::ffi::c_int",
        bits = "224..=255"
    )]
    #[bitfield(
        name = "c2rust_unnamed_7",
        ty = "::core::ffi::c_int",
        bits = "256..=287"
    )]
    #[bitfield(
        name = "c2rust_unnamed_8",
        ty = "::core::ffi::c_int",
        bits = "288..=319"
    )]
    #[bitfield(
        name = "c2rust_unnamed_9",
        ty = "::core::ffi::c_int",
        bits = "320..=351"
    )]
    pub c2rust_unnamed_c2rust_unnamed_0_c2rust_unnamed_1_c2rust_unnamed_2_c2rust_unnamed_3_c2rust_unnamed_4_c2rust_unnamed_5_c2rust_unnamed_6_c2rust_unnamed_7_c2rust_unnamed_8_c2rust_unnamed_9:
        [u8; 44],
}
pub const ENOSYS: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const INT64_MIN: ::core::ffi::c_long =
    -9223372036854775807 as ::core::ffi::c_long - 1 as ::core::ffi::c_long;
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const STA_UNSYNC: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const TIME_ERROR: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn saturating_subtract(mut left: int64_t, mut right: int64_t) -> int64_t {
    if right > 0 as int64_t && left < INT64_MIN as int64_t + right {
        return INT64_MIN as int64_t;
    }
    if right < 0 as int64_t && left > INT64_MAX as int64_t + right {
        return INT64_MAX as int64_t;
    }
    return left - right;
}
unsafe extern "C" fn default_realtime(
    mut context: *mut ::core::ffi::c_void,
    mut milliseconds: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut value: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if milliseconds.is_null()
        || clock_gettime(CLOCK_REALTIME, &raw mut value) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    *milliseconds = (value.tv_sec as int64_t as ::core::ffi::c_longlong
        * 1000 as ::core::ffi::c_longlong
        + value.tv_nsec as ::core::ffi::c_longlong / 1000000 as ::core::ffi::c_longlong)
        as int64_t;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn default_monotonic(
    mut context: *mut ::core::ffi::c_void,
    mut milliseconds: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut value: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if milliseconds.is_null()
        || clock_gettime(CLOCK_MONOTONIC, &raw mut value) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    *milliseconds = (value.tv_sec as int64_t as ::core::ffi::c_longlong
        * 1000 as ::core::ffi::c_longlong
        + value.tv_nsec as ::core::ffi::c_longlong / 1000000 as ::core::ffi::c_longlong)
        as int64_t;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn default_synchronization(
    mut context: *mut ::core::ffi::c_void,
    mut synchronized: *mut bool,
) -> ::core::ffi::c_int {
    let mut value: timex = timex {
        modes: 0,
        offset: 0,
        freq: 0,
        maxerror: 0,
        esterror: 0,
        status: 0,
        constant: 0,
        precision: 0,
        tolerance: 0,
        time: timeval { tv_sec: 0, tv_usec: 0 },
        tick: 0,
        ppsfreq: 0,
        jitter: 0,
        shift: 0,
        stabil: 0,
        jitcnt: 0,
        calcnt: 0,
        errcnt: 0,
        stbcnt: 0,
        tai: 0,
        c2rust_unnamed_c2rust_unnamed_0_c2rust_unnamed_1_c2rust_unnamed_2_c2rust_unnamed_3_c2rust_unnamed_4_c2rust_unnamed_5_c2rust_unnamed_6_c2rust_unnamed_7_c2rust_unnamed_8_c2rust_unnamed_9: [0; 44],
    };
    memset(
        &raw mut value as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<timex>(),
    );
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut result: ::core::ffi::c_int = adjtimex(&raw mut value);
    if result < 0 as ::core::ffi::c_int || synchronized.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *synchronized = result != TIME_ERROR && value.status & STA_UNSYNC == 0 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_clock_default_source() -> linux_clock_source_t {
    let mut source: linux_clock_source_t = linux_clock_source_t {
        context: NULL,
        realtime: Some(
            default_realtime
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut int64_t,
                ) -> ::core::ffi::c_int,
        ),
        monotonic: Some(
            default_monotonic
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut int64_t,
                ) -> ::core::ffi::c_int,
        ),
        synchronization: Some(
            default_synchronization
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut bool) -> ::core::ffi::c_int,
        ),
    };
    return source;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_clock_sample(
    mut source: *const linux_clock_source_t,
    mut sample: *mut linux_clock_sample_t,
) -> ::core::ffi::c_int {
    if source.is_null()
        || sample.is_null()
        || (*source).realtime.is_none()
        || (*source).monotonic.is_none()
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        sample as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_clock_sample_t>(),
    );
    let mut monotonic_ms: int64_t = 0 as int64_t;
    if (*source).realtime.expect("non-null function pointer")(
        (*source).context,
        &raw mut (*sample).realtime_ms,
    ) != 0 as ::core::ffi::c_int
        || (*source).monotonic.expect("non-null function pointer")(
            (*source).context,
            &raw mut monotonic_ms,
        ) != 0 as ::core::ffi::c_int
        || monotonic_ms < 0 as int64_t
    {
        (*sample).capability = system_health_capability_t(
            (if *__errno_location() == EACCES || *__errno_location() == EPERM {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED.0
                    as ::core::ffi::c_int
            } else {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
        return -1 as ::core::ffi::c_int;
    }
    (*sample).monotonic_ms = monotonic_ms as uint64_t;
    (*sample).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    if (*source).synchronization.is_none() {
        (*sample).capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
        return 0 as ::core::ffi::c_int;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    if (*source)
        .synchronization
        .expect("non-null function pointer")(
        (*source).context, &raw mut (*sample).synchronized
    ) != 0 as ::core::ffi::c_int
    {
        (*sample).capability = system_health_capability_t(
            (if *__errno_location() == ENOSYS {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED.0
                    as ::core::ffi::c_int
            } else if *__errno_location() == EACCES || *__errno_location() == EPERM {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED.0
                    as ::core::ffi::c_int
            } else {
                system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
        return 0 as ::core::ffi::c_int;
    }
    (*sample).synchronization_known = r#true != 0;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linux_clock_evaluate(
    mut state: *mut linux_clock_state_t,
    mut sample: *const linux_clock_sample_t,
    mut startup_grace_ms: uint64_t,
    mut jump_threshold_ms: int64_t,
) -> linux_clock_result_t {
    let mut result: linux_clock_result_t = linux_clock_result_t {
        capability: system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        synchronization_known: false,
        synchronized: false,
        startup_grace_active: false,
        jump_detected: false,
        jump_ms: 0,
    };
    memset(
        &raw mut result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<linux_clock_result_t>(),
    );
    if state.is_null() || sample.is_null() {
        result.capability = system_health_capability_t::SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    result.capability = (*sample).capability;
    result.synchronization_known = (*sample).synchronization_known;
    result.synchronized = (*sample).synchronized;
    if jump_threshold_ms == INT64_MIN as int64_t {
        jump_threshold_ms = INT64_MAX as int64_t;
    } else if jump_threshold_ms < 0 as int64_t {
        jump_threshold_ms = -jump_threshold_ms;
    }
    if !(*state).initialized {
        (*state).initialized = r#true != 0;
        (*state).startup_monotonic_ms = (*sample).monotonic_ms;
    } else if (*sample).monotonic_ms >= (*state).previous_monotonic_ms {
        let mut monotonic_delta: uint64_t = (*sample)
            .monotonic_ms
            .wrapping_sub((*state).previous_monotonic_ms);
        if monotonic_delta <= INT64_MAX as uint64_t {
            let mut realtime_delta: int64_t =
                saturating_subtract((*sample).realtime_ms, (*state).previous_realtime_ms);
            result.jump_ms = saturating_subtract(realtime_delta, monotonic_delta as int64_t);
            result.jump_detected =
                result.jump_ms > jump_threshold_ms || result.jump_ms < -jump_threshold_ms;
        }
    }
    result.startup_grace_active = (*sample).monotonic_ms >= (*state).startup_monotonic_ms
        && (*sample)
            .monotonic_ms
            .wrapping_sub((*state).startup_monotonic_ms)
            < startup_grace_ms;
    (*state).previous_realtime_ms = (*sample).realtime_ms;
    (*state).previous_monotonic_ms = (*sample).monotonic_ms;
    return result;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
