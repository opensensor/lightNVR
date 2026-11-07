unsafe extern "C" {
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_telemetry_event_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub session_id: [::core::ffi::c_char; 64],
    pub transport: [::core::ffi::c_char; 16],
    pub ttff_ms: ::core::ffi::c_double,
    pub rebuffer_count: ::core::ffi::c_int,
    pub rebuffer_duration_ms: ::core::ffi::c_double,
    pub resolution_switches: ::core::ffi::c_int,
    pub webrtc_rtt_ms: ::core::ffi::c_double,
    pub timestamp: time_t,
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
pub const PLAYER_TELEMETRY_RING_SIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const LOG_COMPONENT: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"PlayerTelemetry\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
static mut g_events: [player_telemetry_event_t; 256] = [player_telemetry_event_t {
    stream_name: [0; 256],
    session_id: [0; 64],
    transport: [0; 16],
    ttff_ms: 0.,
    rebuffer_count: 0,
    rebuffer_duration_ms: 0.,
    resolution_switches: 0,
    webrtc_rtt_ms: 0.,
    timestamp: 0,
}; 256];
static mut g_head: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut g_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut g_mutex: pthread_mutex_t = pthread_mutex_t {
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
static mut g_initialized: bool = r#false != 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn player_telemetry_init() {
    pthread_mutex_lock(&raw mut g_mutex);
    memset(
        &raw mut g_events as *mut player_telemetry_event_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[player_telemetry_event_t; 256]>(),
    );
    g_head = 0 as ::core::ffi::c_int;
    g_count = 0 as ::core::ffi::c_int;
    g_initialized = r#true != 0;
    pthread_mutex_unlock(&raw mut g_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Player telemetry subsystem initialized (ring size: %d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        256 as ::core::ffi::c_int,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn player_telemetry_shutdown() {
    pthread_mutex_lock(&raw mut g_mutex);
    g_initialized = r#false != 0;
    g_head = 0 as ::core::ffi::c_int;
    g_count = 0 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut g_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Player telemetry subsystem shut down\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn player_telemetry_record(mut event: *const player_telemetry_event_t) {
    if event.is_null() || !g_initialized {
        return;
    }
    pthread_mutex_lock(&raw mut g_mutex);
    memcpy(
        (&raw mut g_events as *mut player_telemetry_event_t).offset(g_head as isize)
            as *mut ::core::ffi::c_void,
        event as *const ::core::ffi::c_void,
        ::core::mem::size_of::<player_telemetry_event_t>(),
    );
    if g_events[g_head as usize].timestamp == 0 as time_t {
        g_events[g_head as usize].timestamp = time(::core::ptr::null_mut::<time_t>());
    }
    g_head = (g_head + 1 as ::core::ffi::c_int) % PLAYER_TELEMETRY_RING_SIZE;
    if g_count < PLAYER_TELEMETRY_RING_SIZE {
        g_count += 1;
    }
    pthread_mutex_unlock(&raw mut g_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn player_telemetry_snapshot(
    mut out: *mut player_telemetry_event_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if out.is_null() || max_count <= 0 as ::core::ffi::c_int || !g_initialized {
        return 0 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut g_mutex);
    let mut to_copy: ::core::ffi::c_int = if g_count < max_count {
        g_count
    } else {
        max_count
    };
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < to_copy {
        let mut idx: ::core::ffi::c_int = (g_head - 1 as ::core::ffi::c_int - i
            + PLAYER_TELEMETRY_RING_SIZE)
            % PLAYER_TELEMETRY_RING_SIZE;
        memcpy(
            out.offset(i as isize) as *mut ::core::ffi::c_void,
            (&raw mut g_events as *mut player_telemetry_event_t).offset(idx as isize)
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<player_telemetry_event_t>(),
        );
        i += 1;
    }
    pthread_mutex_unlock(&raw mut g_mutex);
    return to_copy;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn player_telemetry_count() -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut g_mutex);
    let mut c: ::core::ffi::c_int = g_count;
    pthread_mutex_unlock(&raw mut g_mutex);
    return c;
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
