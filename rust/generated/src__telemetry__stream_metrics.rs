unsafe extern "C" {
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn difftime(__time1: time_t, __time0: time_t) -> ::core::ffi::c_double;
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
    unsafe fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_rwlock_destroy(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    unsafe fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    unsafe fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    unsafe fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> ::core::ffi::c_int;
    unsafe fn event_producer_publish_camera_offline_for_stream(
        stream_name: *const ::core::ffi::c_char,
        reason: *const ::core::ffi::c_char,
        consecutive_failures: ::core::ffi::c_int,
        occurred_at: time_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn event_producer_publish_camera_recovered_for_stream(
        stream_name: *const ::core::ffi::c_char,
        downtime_ms: int64_t,
        occurred_at: time_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn event_producer_publish_stream_degraded_for_stream(
        stream_name: *const ::core::ffi::c_char,
        reason: *const ::core::ffi::c_char,
        observed_fps: ::core::ffi::c_double,
        expected_fps: ::core::ffi::c_double,
        occurred_at: time_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn event_producer_publish_stream_recovered_for_stream(
        stream_name: *const ::core::ffi::c_char,
        observed_fps: ::core::ffi::c_double,
        expected_fps: ::core::ffi::c_double,
        occurred_at: time_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn event_producer_publish_recording_gap_for_stream(
        stream_name: *const ::core::ffi::c_char,
        started_at: time_t,
        duration_ms: int64_t,
        occurred_at: time_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn is_shutdown_initiated() -> bool;
    unsafe fn db_camera_observation_record(
        stream_name: *const ::core::ffi::c_char,
        video_at: int64_t,
        recording_at: int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn log_set_thread_context(
        component: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
    );
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn interruptible_sleep_init(s: *mut interruptible_sleep_t) -> ::core::ffi::c_int;
    unsafe fn interruptible_sleep_destroy(s: *mut interruptible_sleep_t);
    unsafe fn interruptible_sleep_wait(s: *mut interruptible_sleep_t, seconds: ::core::ffi::c_int);
    unsafe fn interruptible_sleep_wake(s: *mut interruptible_sleep_t);
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type int64_t = i64;
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
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::core::ffi::c_uint,
    pub __writers: ::core::ffi::c_uint,
    pub __wrphase_futex: ::core::ffi::c_uint,
    pub __writers_futex: ::core::ffi::c_uint,
    pub __pad3: ::core::ffi::c_uint,
    pub __pad4: ::core::ffi::c_uint,
    pub __cur_writer: ::core::ffi::c_int,
    pub __shared: ::core::ffi::c_int,
    pub __pad1: ::core::ffi::c_ulong,
    pub __pad2: ::core::ffi::c_ulong,
    pub __flags: ::core::ffi::c_uint,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlockattr_t {
    pub __size: [::core::ffi::c_char; 8],
    pub __align: ::core::ffi::c_long,
}
pub type uint64_t = u64;
pub type int_fast64_t = ::core::ffi::c_long;
pub type uint_fast64_t = ::core::ffi::c_ulong;
pub type atomic_int = ::core::ffi::c_int;
pub type atomic_int_fast64_t = int_fast64_t;
pub type atomic_uint_fast64_t = uint_fast64_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct stream_health_status_t(pub ::core::ffi::c_uint);
impl stream_health_status_t {
    pub const STREAM_HEALTH_UP: Self = Self(0);
    pub const STREAM_HEALTH_DEGRADED: Self = Self(1);
    pub const STREAM_HEALTH_DOWN: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct metrics_frame_source_t(pub ::core::ffi::c_uint);
impl metrics_frame_source_t {
    pub const METRICS_SOURCE_GENERIC: Self = Self(0);
    pub const METRICS_SOURCE_RECORDING: Self = Self(1);
    pub const METRICS_SOURCE_HLS: Self = Self(2);
    pub const METRICS_SOURCE_DETECTION: Self = Self(3);
    pub const METRICS_SOURCE_COUNT: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct metrics_counter_delta_t {
    pub frames: uint64_t,
    pub bytes: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct metrics_ring_sample_t {
    pub fps: ::core::ffi::c_float,
    pub bitrate_kbps: ::core::ffi::c_float,
    pub timestamp: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_metrics_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub active: bool,
    pub health_status: atomic_int,
    pub stream_up: atomic_int,
    pub health_observed: bool,
    pub health_changed_at: time_t,
    pub last_packet_ts: atomic_int_fast64_t,
    pub last_frame_ts: atomic_int_fast64_t,
    pub current_fps: ::core::ffi::c_double,
    pub current_bitrate_bps: ::core::ffi::c_double,
    pub connection_latency_ms: ::core::ffi::c_double,
    pub configured_fps: ::core::ffi::c_double,
    pub frames_total: atomic_uint_fast64_t,
    pub frames_dropped: atomic_uint_fast64_t,
    pub bytes_received: atomic_uint_fast64_t,
    pub reconnects_total: atomic_uint_fast64_t,
    pub uptime_seconds: atomic_uint_fast64_t,
    pub error_decode: atomic_uint_fast64_t,
    pub error_timeout: atomic_uint_fast64_t,
    pub error_protocol: atomic_uint_fast64_t,
    pub error_io: atomic_uint_fast64_t,
    pub recording_active: atomic_int,
    pub recording_bytes_written: atomic_uint_fast64_t,
    pub recording_segments_total: atomic_uint_fast64_t,
    pub recording_gaps_total: atomic_uint_fast64_t,
    pub last_completed_segment_ts: atomic_int_fast64_t,
    pub ring: [metrics_ring_sample_t; 720],
    pub ring_head: ::core::ffi::c_int,
    pub ring_count: ::core::ffi::c_int,
    pub last_segment_end_time: time_t,
    pub expected_segment_duration: ::core::ffi::c_int,
    pub observation_persisted_at: time_t,
    pub stream_start_time: time_t,
    pub source_frames_total: [atomic_uint_fast64_t; 4],
    pub source_bytes_received: [atomic_uint_fast64_t; 4],
    pub prev_source_frames: [uint64_t; 4],
    pub prev_source_bytes: [uint64_t; 4],
    pub prev_sample_time: time_t,
    pub lock: pthread_rwlock_t,
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
pub struct interruptible_sleep_t {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub woken: bool,
    pub clock_id: clockid_t,
}
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const METRICS_RING_SIZE: ::core::ffi::c_int = 720 as ::core::ffi::c_int;
pub const STREAM_HEALTH_FRAME_TIMEOUT_UP: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const STREAM_HEALTH_FRAME_TIMEOUT_DOWN: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const STREAM_HEALTH_FPS_RATIO: ::core::ffi::c_double = 0.5f64;
pub const RECORDING_GAP_THRESHOLD_SEC: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const LOG_COMPONENT: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"Metrics\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
static mut g_metrics: *mut stream_metrics_t = ::core::ptr::null_mut::<stream_metrics_t>();
static mut g_max_streams: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut g_initialized: bool = r#false != 0;
static mut g_sampler_thread: pthread_t = 0;
static mut g_sampler_running: bool = r#false != 0;
static mut g_sampler_wake: interruptible_sleep_t = interruptible_sleep_t {
    mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __glibc_reserved: 0,
            __list: __pthread_list_t {
                __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
            },
        },
    },
    cond: pthread_cond_t {
        __data: __pthread_cond_s {
            __wseq: __atomic_wide_counter { __value64: 0 },
            __g1_start: __atomic_wide_counter { __value64: 0 },
            __g_size: [0; 2],
            __g1_orig_size: 0,
            __wrefs: 0,
            __g_signals: [0; 2],
            __unused_initialized_1: 0,
            __unused_initialized_2: 0,
        },
    },
    woken: false,
    clock_id: 0,
};
pub const SAMPLER_INTERVAL_SEC: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const OBSERVATION_PERSIST_INTERVAL_SEC: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
unsafe extern "C" fn find_slot(mut stream_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut first_free: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_max_streams {
        if (*g_metrics.offset(i as isize)).active as ::core::ffi::c_int != 0
            && strncmp(
                &raw mut (*g_metrics.offset(i as isize)).stream_name as *mut ::core::ffi::c_char,
                stream_name,
                MAX_STREAM_NAME as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            return i;
        }
        if !(*g_metrics.offset(i as isize)).active && first_free < 0 as ::core::ffi::c_int {
            first_free = i;
        }
        i += 1;
    }
    return first_free;
}
unsafe extern "C" fn find_active_slot(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_max_streams {
        if (*g_metrics.offset(i as isize)).active as ::core::ffi::c_int != 0
            && strncmp(
                &raw mut (*g_metrics.offset(i as isize)).stream_name as *mut ::core::ffi::c_char,
                stream_name,
                MAX_STREAM_NAME as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            return i;
        }
        i += 1;
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_merge_source_deltas(
    mut current_frames: *const uint64_t,
    mut current_bytes: *const uint64_t,
    mut previous_frames: *mut uint64_t,
    mut previous_bytes: *mut uint64_t,
) -> metrics_counter_delta_t {
    let mut merged: metrics_counter_delta_t = metrics_counter_delta_t {
        frames: 0 as uint64_t,
        bytes: 0 as uint64_t,
    };
    let mut source: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while source < metrics_frame_source_t::METRICS_SOURCE_COUNT.0 as ::core::ffi::c_int {
        let mut frame_delta: uint64_t = if *current_frames.offset(source as isize)
            >= *previous_frames.offset(source as isize)
        {
            (*current_frames.offset(source as isize))
                .wrapping_sub(*previous_frames.offset(source as isize))
        } else {
            *current_frames.offset(source as isize)
        };
        let mut byte_delta: uint64_t =
            if *current_bytes.offset(source as isize) >= *previous_bytes.offset(source as isize) {
                (*current_bytes.offset(source as isize))
                    .wrapping_sub(*previous_bytes.offset(source as isize))
            } else {
                *current_bytes.offset(source as isize)
            };
        if frame_delta > merged.frames {
            merged.frames = frame_delta;
        }
        if byte_delta > merged.bytes {
            merged.bytes = byte_delta;
        }
        *previous_frames.offset(source as isize) = *current_frames.offset(source as isize);
        *previous_bytes.offset(source as isize) = *current_bytes.offset(source as isize);
        source += 1;
    }
    return merged;
}
unsafe extern "C" fn log_publish_failure(
    mut r#type: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut error: *const ::core::ffi::c_char,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Could not enqueue %s event for stream '%s': %s\0".as_ptr() as *const ::core::ffi::c_char,
        r#type,
        stream_name,
        if !error.is_null() && *error.offset(0isize) as ::core::ffi::c_int != 0 {
            error
        } else {
            b"unknown error\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
}
unsafe extern "C" fn publish_health_transition(
    mut stream_name: *const ::core::ffi::c_char,
    mut old_status: stream_health_status_t,
    mut new_status: stream_health_status_t,
    mut initial_observation: bool,
    mut previous_changed_at: time_t,
    mut now: time_t,
    mut frame_age: ::core::ffi::c_double,
    mut observed_fps: ::core::ffi::c_double,
    mut expected_fps: ::core::ffi::c_double,
) {
    if !initial_observation && old_status.0 == new_status.0
        || stream_name.is_null()
        || *stream_name.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return;
    }
    let mut error: [::core::ffi::c_char; 256] = [
        0 as ::core::ffi::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    if new_status.0 == stream_health_status_t::STREAM_HEALTH_DOWN.0 {
        let mut failure_samples: ::core::ffi::c_double =
            frame_age / SAMPLER_INTERVAL_SEC as ::core::ffi::c_double;
        let mut failures: ::core::ffi::c_int = if failure_samples > INT_MAX as ::core::ffi::c_double
        {
            INT_MAX
        } else {
            failure_samples as ::core::ffi::c_int
        };
        if failures < 1 as ::core::ffi::c_int {
            failures = 1 as ::core::ffi::c_int;
        }
        if event_producer_publish_camera_offline_for_stream(
            stream_name,
            b"frame_timeout\0".as_ptr() as *const ::core::ffi::c_char,
            failures,
            now,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        ) != 0 as ::core::ffi::c_int
        {
            log_publish_failure(
                b"camera offline\0".as_ptr() as *const ::core::ffi::c_char,
                stream_name,
                &raw mut error as *mut ::core::ffi::c_char,
            );
        }
        return;
    }
    if !initial_observation && old_status.0 == stream_health_status_t::STREAM_HEALTH_DOWN.0 {
        let mut downtime_ms: int64_t =
            if previous_changed_at > 0 as time_t && now > previous_changed_at {
                (now - previous_changed_at) as int64_t * 1000 as int64_t
            } else {
                0 as int64_t
            };
        if event_producer_publish_camera_recovered_for_stream(
            stream_name,
            downtime_ms,
            now,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        ) != 0 as ::core::ffi::c_int
        {
            log_publish_failure(
                b"camera recovered\0".as_ptr() as *const ::core::ffi::c_char,
                stream_name,
                &raw mut error as *mut ::core::ffi::c_char,
            );
        }
        error[0usize] = '\0' as ::core::ffi::c_char;
    }
    if new_status.0 == stream_health_status_t::STREAM_HEALTH_DEGRADED.0 {
        let mut reason: *const ::core::ffi::c_char =
            if frame_age > STREAM_HEALTH_FRAME_TIMEOUT_UP as ::core::ffi::c_double {
                b"stale_frames\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"low_fps\0".as_ptr() as *const ::core::ffi::c_char
            };
        if event_producer_publish_stream_degraded_for_stream(
            stream_name,
            reason,
            observed_fps,
            expected_fps,
            now,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        ) != 0 as ::core::ffi::c_int
        {
            log_publish_failure(
                b"stream degraded\0".as_ptr() as *const ::core::ffi::c_char,
                stream_name,
                &raw mut error as *mut ::core::ffi::c_char,
            );
        }
    } else if !initial_observation
        && old_status.0 == stream_health_status_t::STREAM_HEALTH_DEGRADED.0
    {
        if event_producer_publish_stream_recovered_for_stream(
            stream_name,
            observed_fps,
            expected_fps,
            now,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        ) != 0 as ::core::ffi::c_int
        {
            log_publish_failure(
                b"stream recovered\0".as_ptr() as *const ::core::ffi::c_char,
                stream_name,
                &raw mut error as *mut ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn sampler_thread_func(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    log_set_thread_context(
        b"MetricsSampler\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Metrics sampler thread started (interval: %ds)\0".as_ptr() as *const ::core::ffi::c_char,
        5 as ::core::ffi::c_int,
    );
    while ::core::ptr::read_volatile::<bool>(&raw const g_sampler_running) {
        interruptible_sleep_wait(&raw mut g_sampler_wake, SAMPLER_INTERVAL_SEC);
        if is_shutdown_initiated() {
            ::core::ptr::write_volatile(&raw mut g_sampler_running, r#false != 0);
        }
        if !::core::ptr::read_volatile::<bool>(&raw const g_sampler_running) {
            break;
        }
        let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < g_max_streams {
            let mut m: *mut stream_metrics_t = g_metrics.offset(i as isize);
            if (*m).active {
                pthread_rwlock_wrlock(&raw mut (*m).lock);
                let mut cur_source_frames: [uint64_t; 4] = [0; 4];
                let mut cur_source_bytes: [uint64_t; 4] = [0; 4];
                let mut source: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while source < metrics_frame_source_t::METRICS_SOURCE_COUNT.0 as ::core::ffi::c_int
                {
                    cur_source_frames[source as usize] = ::core::intrinsics::atomic_load::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(
                        (&raw mut (*m).source_frames_total as *mut atomic_uint_fast64_t)
                            .offset(source as isize),
                    ) as uint64_t;
                    cur_source_bytes[source as usize] = ::core::intrinsics::atomic_load::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(
                        (&raw mut (*m).source_bytes_received as *mut atomic_uint_fast64_t)
                            .offset(source as isize),
                    ) as uint64_t;
                    source += 1;
                }
                if (*m).prev_sample_time > 0 as time_t {
                    let mut dt: ::core::ffi::c_double = difftime(now, (*m).prev_sample_time);
                    if dt > 0.0f64 {
                        let mut observed: metrics_counter_delta_t = metrics_merge_source_deltas(
                            &raw mut cur_source_frames as *mut uint64_t,
                            &raw mut cur_source_bytes as *mut uint64_t,
                            &raw mut (*m).prev_source_frames as *mut uint64_t,
                            &raw mut (*m).prev_source_bytes as *mut uint64_t,
                        );
                        let mut observed_fps: ::core::ffi::c_double =
                            observed.frames as ::core::ffi::c_double / dt;
                        let mut observed_bitrate: ::core::ffi::c_double =
                            observed.bytes as ::core::ffi::c_double * 8.0f64 / dt;
                        ::core::intrinsics::atomic_xadd::<
                            _,
                            _,
                            { ::core::intrinsics::AtomicOrdering::SeqCst },
                        >(&raw mut (*m).frames_total, observed.frames);
                        ::core::intrinsics::atomic_xadd::<
                            _,
                            _,
                            { ::core::intrinsics::AtomicOrdering::SeqCst },
                        >(&raw mut (*m).bytes_received, observed.bytes);
                        (*m).current_fps = if (*m).current_fps > 0.0f64 {
                            0.5f64 * (*m).current_fps + 0.5f64 * observed_fps
                        } else {
                            observed_fps
                        };
                        (*m).current_bitrate_bps = if (*m).current_bitrate_bps > 0.0f64 {
                            0.5f64 * (*m).current_bitrate_bps + 0.5f64 * observed_bitrate
                        } else {
                            observed_bitrate
                        };
                    }
                }
                (*m).prev_sample_time = now;
                if (*m).stream_start_time > 0 as time_t {
                    ::core::intrinsics::atomic_store::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(
                        &raw mut (*m).uptime_seconds,
                        difftime(now, (*m).stream_start_time) as uint64_t,
                    );
                }
                let mut last_frame: time_t = ::core::intrinsics::atomic_load::<
                    _,
                    { ::core::intrinsics::AtomicOrdering::SeqCst },
                >(&raw mut (*m).last_frame_ts);
                let mut frame_age: ::core::ffi::c_double = if last_frame > 0 as time_t {
                    difftime(now, last_frame)
                } else {
                    999.0f64
                };
                let mut cfg_fps: ::core::ffi::c_double = (*m).configured_fps;
                let mut status: stream_health_status_t = stream_health_status_t::STREAM_HEALTH_UP;
                if frame_age > STREAM_HEALTH_FRAME_TIMEOUT_DOWN as ::core::ffi::c_double {
                    status = stream_health_status_t::STREAM_HEALTH_DOWN;
                } else if frame_age > STREAM_HEALTH_FRAME_TIMEOUT_UP as ::core::ffi::c_double
                    || (*m).current_fps < cfg_fps * STREAM_HEALTH_FPS_RATIO
                {
                    status = stream_health_status_t::STREAM_HEALTH_DEGRADED;
                } else {
                    status = stream_health_status_t::STREAM_HEALTH_UP;
                }
                let mut previous_status: stream_health_status_t =
                    stream_health_status_t(::core::intrinsics::atomic_load::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut (*m).health_status)
                        as ::core::ffi::c_uint);
                let mut initial_observation: bool = !(*m).health_observed;
                let mut previous_changed_at: time_t = (*m).health_changed_at;
                let mut health_changed: bool =
                    initial_observation as ::core::ffi::c_int != 0 || status.0 != previous_status.0;
                if health_changed {
                    (*m).health_observed = r#true != 0;
                    (*m).health_changed_at = now;
                }
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).health_status,
                    status.0 as ::core::ffi::c_int,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).stream_up,
                    if status.0 == stream_health_status_t::STREAM_HEALTH_UP.0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    },
                );
                let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
                safe_strcpy(
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                    &raw mut (*m).stream_name as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    0 as size_t,
                );
                let mut observed_fps_0: ::core::ffi::c_double = (*m).current_fps;
                let mut expected_fps: ::core::ffi::c_double = cfg_fps;
                let mut last_recording: int64_t =
                    ::core::intrinsics::atomic_load::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut (*m).last_completed_segment_ts);
                let mut persist_observation: bool = (last_frame > 0 as time_t
                    || last_recording > 0 as int64_t)
                    && ((*m).observation_persisted_at == 0 as time_t
                        || difftime(now, (*m).observation_persisted_at)
                            >= OBSERVATION_PERSIST_INTERVAL_SEC as ::core::ffi::c_double);
                if persist_observation {
                    (*m).observation_persisted_at = now;
                }
                let mut idx: ::core::ffi::c_int = (*m).ring_head;
                (*m).ring[idx as usize].fps = (*m).current_fps as ::core::ffi::c_float;
                (*m).ring[idx as usize].bitrate_kbps =
                    ((*m).current_bitrate_bps / 1000.0f64) as ::core::ffi::c_float;
                (*m).ring[idx as usize].timestamp = now;
                (*m).ring_head = (idx + 1 as ::core::ffi::c_int) % METRICS_RING_SIZE;
                if (*m).ring_count < METRICS_RING_SIZE {
                    (*m).ring_count += 1;
                }
                pthread_rwlock_unlock(&raw mut (*m).lock);
                if persist_observation as ::core::ffi::c_int != 0
                    && db_camera_observation_record(
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                        last_frame as int64_t,
                        last_recording,
                    ) != 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Could not persist observation for stream '%s'\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                    );
                }
                if health_changed {
                    publish_health_transition(
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                        previous_status,
                        status,
                        initial_observation,
                        previous_changed_at,
                        now,
                        frame_age,
                        observed_fps_0,
                        expected_fps,
                    );
                }
            }
            i += 1;
        }
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Metrics sampler thread exiting\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_init(mut max_streams: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if g_initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"metrics_init called but already initialized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if max_streams <= 0 as ::core::ffi::c_int || max_streams > MAX_STREAMS {
        max_streams = MAX_STREAMS;
    }
    g_max_streams = max_streams;
    g_metrics = calloc(
        max_streams as size_t,
        ::core::mem::size_of::<stream_metrics_t>(),
    ) as *mut stream_metrics_t;
    if g_metrics.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to allocate metrics array for %d streams\0".as_ptr()
                as *const ::core::ffi::c_char,
            max_streams,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < max_streams {
        pthread_rwlock_init(
            &raw mut (*g_metrics.offset(i as isize)).lock,
            ::core::ptr::null::<pthread_rwlockattr_t>(),
        );
        i += 1;
    }
    g_initialized = r#true != 0;
    if interruptible_sleep_init(&raw mut g_sampler_wake) == 0 as ::core::ffi::c_int {
        ::core::ptr::write_volatile(&raw mut g_sampler_running, r#true != 0);
        if pthread_create(
            &raw mut g_sampler_thread,
            ::core::ptr::null::<pthread_attr_t>(),
            Some(
                sampler_thread_func
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            ),
            NULL,
        ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to create metrics sampler thread: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            ::core::ptr::write_volatile(&raw mut g_sampler_running, r#false != 0);
            interruptible_sleep_destroy(&raw mut g_sampler_wake);
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to initialize metrics sampler wake handle\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Metrics subsystem initialized for %d streams\0".as_ptr() as *const ::core::ffi::c_char,
        max_streams,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_shutdown() {
    if !g_initialized {
        return;
    }
    if ::core::ptr::read_volatile::<bool>(&raw const g_sampler_running) {
        ::core::ptr::write_volatile(&raw mut g_sampler_running, r#false != 0);
        interruptible_sleep_wake(&raw mut g_sampler_wake);
        pthread_join(
            g_sampler_thread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        interruptible_sleep_destroy(&raw mut g_sampler_wake);
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Metrics sampler thread stopped\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_max_streams {
        pthread_rwlock_destroy(&raw mut (*g_metrics.offset(i as isize)).lock);
        i += 1;
    }
    free(g_metrics as *mut ::core::ffi::c_void);
    g_metrics = ::core::ptr::null_mut::<stream_metrics_t>();
    g_max_streams = 0 as ::core::ffi::c_int;
    g_initialized = r#false != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Metrics subsystem shut down\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn metrics_get_slot(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !g_initialized || stream_name.is_null() || *stream_name.offset(0isize) == 0 {
        return -1 as ::core::ffi::c_int;
    }
    loop {
        let mut idx: ::core::ffi::c_int = find_slot(stream_name);
        if idx < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        let mut m: *mut stream_metrics_t = g_metrics.offset(idx as isize);
        if !(*m).active {
            pthread_rwlock_wrlock(&raw mut (*m).lock);
            if !(*m).active {
                memset(
                    &raw mut (*m).stream_name as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                );
                safe_strcpy(
                    &raw mut (*m).stream_name as *mut ::core::ffi::c_char,
                    stream_name,
                    MAX_STREAM_NAME as size_t,
                    0 as size_t,
                );
                (*m).active = r#true != 0;
                (*m).stream_start_time = time(::core::ptr::null_mut::<time_t>());
                (*m).prev_sample_time = (*m).stream_start_time;
                (*m).current_fps = 0.0f64;
                (*m).current_bitrate_bps = 0.0f64;
                (*m).connection_latency_ms = 0.0f64;
                (*m).ring_head = 0 as ::core::ffi::c_int;
                (*m).ring_count = 0 as ::core::ffi::c_int;
                (*m).last_segment_end_time = 0 as time_t;
                (*m).observation_persisted_at = 0 as time_t;
                (*m).configured_fps = 30.0f64;
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).health_status,
                    stream_health_status_t::STREAM_HEALTH_DOWN.0 as ::core::ffi::c_int,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).stream_up,
                    0 as ::core::ffi::c_int,
                );
                (*m).health_observed = r#false != 0;
                (*m).health_changed_at = 0 as time_t;
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).last_packet_ts,
                    0 as int_fast64_t,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).last_frame_ts,
                    0 as int_fast64_t,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).frames_total,
                    0 as uint_fast64_t,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).frames_dropped,
                    0 as uint_fast64_t,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).bytes_received,
                    0 as uint_fast64_t,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).reconnects_total,
                    0 as uint_fast64_t,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).uptime_seconds,
                    0 as uint_fast64_t,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).error_decode,
                    0 as uint_fast64_t,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).error_timeout,
                    0 as uint_fast64_t,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).error_protocol,
                    0 as uint_fast64_t,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).error_io,
                    0 as uint_fast64_t,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).recording_active,
                    0 as ::core::ffi::c_int,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).recording_bytes_written,
                    0 as uint_fast64_t,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).recording_segments_total,
                    0 as uint_fast64_t,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).recording_gaps_total,
                    0 as uint_fast64_t,
                );
                ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).last_completed_segment_ts,
                    0 as int_fast64_t,
                );
                let mut source: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while source < metrics_frame_source_t::METRICS_SOURCE_COUNT.0 as ::core::ffi::c_int
                {
                    ::core::intrinsics::atomic_store::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(
                        (&raw mut (*m).source_frames_total as *mut atomic_uint_fast64_t)
                            .offset(source as isize),
                        0 as uint_fast64_t,
                    );
                    ::core::intrinsics::atomic_store::<
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(
                        (&raw mut (*m).source_bytes_received as *mut atomic_uint_fast64_t)
                            .offset(source as isize),
                        0 as uint_fast64_t,
                    );
                    (*m).prev_source_frames[source as usize] = 0 as uint64_t;
                    (*m).prev_source_bytes[source as usize] = 0 as uint64_t;
                    source += 1;
                }
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Metrics slot %d allocated for stream '%s'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    idx,
                    stream_name,
                );
                pthread_rwlock_unlock(&raw mut (*m).lock);
                return idx;
            }
            pthread_rwlock_unlock(&raw mut (*m).lock);
            if strncmp(
                &raw mut (*m).stream_name as *mut ::core::ffi::c_char,
                stream_name,
                MAX_STREAM_NAME as size_t,
            ) != 0 as ::core::ffi::c_int
            {
                continue;
            }
        }
        return idx;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_release_slot(mut stream_name: *const ::core::ffi::c_char) {
    if !g_initialized || stream_name.is_null() {
        return;
    }
    let mut idx: ::core::ffi::c_int = find_active_slot(stream_name);
    if idx < 0 as ::core::ffi::c_int {
        return;
    }
    let mut m: *mut stream_metrics_t = g_metrics.offset(idx as isize);
    pthread_rwlock_wrlock(&raw mut (*m).lock);
    (*m).active = r#false != 0;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Metrics slot %d released for stream '%s'\0".as_ptr() as *const ::core::ffi::c_char,
        idx,
        stream_name,
    );
    pthread_rwlock_unlock(&raw mut (*m).lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_record_frame(
    mut stream_name: *const ::core::ffi::c_char,
    mut bytes: ::core::ffi::c_int,
    mut is_video: bool,
) {
    metrics_record_frame_from_source(
        stream_name,
        bytes,
        is_video,
        metrics_frame_source_t::METRICS_SOURCE_GENERIC,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_record_frame_from_source(
    mut stream_name: *const ::core::ffi::c_char,
    mut bytes: ::core::ffi::c_int,
    mut is_video: bool,
    mut source: metrics_frame_source_t,
) {
    if !g_initialized || stream_name.is_null() {
        return;
    }
    if source.0 < 0 as ::core::ffi::c_uint
        || source.0 >= metrics_frame_source_t::METRICS_SOURCE_COUNT.0
    {
        source = metrics_frame_source_t::METRICS_SOURCE_GENERIC;
    }
    let mut idx: ::core::ffi::c_int = metrics_get_slot(stream_name);
    if idx < 0 as ::core::ffi::c_int {
        return;
    }
    let mut m: *mut stream_metrics_t = g_metrics.offset(idx as isize);
    if is_video {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            (&raw mut (*m).source_frames_total as *mut atomic_uint_fast64_t)
                .offset(source.0 as isize),
            1 as uint_fast64_t,
        );
    }
    if bytes > 0 as ::core::ffi::c_int {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            (&raw mut (*m).source_bytes_received as *mut atomic_uint_fast64_t)
                .offset(source.0 as isize),
            bytes as uint64_t,
        );
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*m).last_packet_ts,
        now,
    );
    if is_video {
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*m).last_frame_ts,
            now,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_record_drop(mut stream_name: *const ::core::ffi::c_char) {
    if !g_initialized || stream_name.is_null() {
        return;
    }
    let mut idx: ::core::ffi::c_int = metrics_get_slot(stream_name);
    if idx < 0 as ::core::ffi::c_int {
        return;
    }
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*g_metrics.offset(idx as isize)).frames_dropped,
        1 as uint_fast64_t,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_record_error(
    mut stream_name: *const ::core::ffi::c_char,
    mut error_type: *const ::core::ffi::c_char,
) {
    if !g_initialized || stream_name.is_null() || error_type.is_null() {
        return;
    }
    let mut idx: ::core::ffi::c_int = metrics_get_slot(stream_name);
    if idx < 0 as ::core::ffi::c_int {
        return;
    }
    let mut m: *mut stream_metrics_t = g_metrics.offset(idx as isize);
    if strcmp(
        error_type,
        b"decode\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*m).error_decode,
            1 as uint_fast64_t,
        );
    } else if strcmp(
        error_type,
        b"timeout\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*m).error_timeout,
            1 as uint_fast64_t,
        );
    } else if strcmp(
        error_type,
        b"protocol\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*m).error_protocol,
            1 as uint_fast64_t,
        );
    } else if strcmp(error_type, b"io\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*m).error_io,
            1 as uint_fast64_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_record_reconnect(mut stream_name: *const ::core::ffi::c_char) {
    if !g_initialized || stream_name.is_null() {
        return;
    }
    let mut idx: ::core::ffi::c_int = metrics_get_slot(stream_name);
    if idx < 0 as ::core::ffi::c_int {
        return;
    }
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*g_metrics.offset(idx as isize)).reconnects_total,
        1 as uint_fast64_t,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_record_segment_complete(
    mut stream_name: *const ::core::ffi::c_char,
    mut start_time: time_t,
    mut end_time: time_t,
    mut bytes: uint64_t,
) {
    if !g_initialized || stream_name.is_null() {
        return;
    }
    let mut idx: ::core::ffi::c_int = metrics_get_slot(stream_name);
    if idx < 0 as ::core::ffi::c_int {
        return;
    }
    let mut m: *mut stream_metrics_t = g_metrics.offset(idx as isize);
    pthread_rwlock_wrlock(&raw mut (*m).lock);
    let mut gap_started_at: time_t = 0 as time_t;
    let mut gap_duration_ms: int64_t = 0 as int64_t;
    if (*m).last_segment_end_time > 0 as time_t && start_time > 0 as time_t {
        let mut gap: ::core::ffi::c_double = difftime(start_time, (*m).last_segment_end_time);
        if gap > RECORDING_GAP_THRESHOLD_SEC as ::core::ffi::c_double {
            ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut (*m).recording_gaps_total,
                1 as uint_fast64_t,
            );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Recording gap detected for stream '%s': %.0fs gap\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
                gap,
            );
            gap_started_at = (*m).last_segment_end_time;
            gap_duration_ms = (gap * 1000.0f64) as int64_t;
        }
    }
    if end_time > 0 as time_t {
        (*m).last_segment_end_time = end_time;
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*m).last_completed_segment_ts,
            end_time,
        );
    } else {
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut (*m).last_completed_segment_ts,
            time(::core::ptr::null_mut::<time_t>()),
        );
    }
    if start_time > 0 as time_t && end_time > start_time {
        (*m).expected_segment_duration = (end_time - start_time) as ::core::ffi::c_int;
    }
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*m).recording_segments_total,
        1 as uint_fast64_t,
    );
    ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*m).recording_bytes_written,
        bytes,
    );
    pthread_rwlock_unlock(&raw mut (*m).lock);
    if gap_started_at > 0 as time_t {
        let mut error: [::core::ffi::c_char; 256] = [
            0 as ::core::ffi::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let mut occurred_at: time_t = if start_time > 0 as time_t {
            start_time
        } else {
            time(::core::ptr::null_mut::<time_t>())
        };
        if event_producer_publish_recording_gap_for_stream(
            stream_name,
            gap_started_at,
            gap_duration_ms,
            occurred_at,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        ) != 0 as ::core::ffi::c_int
        {
            log_publish_failure(
                b"recording gap\0".as_ptr() as *const ::core::ffi::c_char,
                stream_name,
                &raw mut error as *mut ::core::ffi::c_char,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_set_recording_active(
    mut stream_name: *const ::core::ffi::c_char,
    mut active: bool,
) {
    if !g_initialized || stream_name.is_null() {
        return;
    }
    let mut idx: ::core::ffi::c_int = metrics_get_slot(stream_name);
    if idx < 0 as ::core::ffi::c_int {
        return;
    }
    let mut m: *mut stream_metrics_t = g_metrics.offset(idx as isize);
    pthread_rwlock_wrlock(&raw mut (*m).lock);
    let mut next: ::core::ffi::c_int = if active as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let mut previous: ::core::ffi::c_int = ::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(&raw mut (*m).recording_active);
    if previous != next {
        (*m).last_segment_end_time = 0 as time_t;
    }
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
        &raw mut (*m).recording_active,
        next,
    );
    pthread_rwlock_unlock(&raw mut (*m).lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_set_connection_latency(
    mut stream_name: *const ::core::ffi::c_char,
    mut latency_ms: ::core::ffi::c_double,
) {
    if !g_initialized || stream_name.is_null() {
        return;
    }
    let mut idx: ::core::ffi::c_int = metrics_get_slot(stream_name);
    if idx < 0 as ::core::ffi::c_int {
        return;
    }
    pthread_rwlock_wrlock(&raw mut (*g_metrics.offset(idx as isize)).lock);
    (*g_metrics.offset(idx as isize)).connection_latency_ms = latency_ms;
    pthread_rwlock_unlock(&raw mut (*g_metrics.offset(idx as isize)).lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_set_configured_fps(
    mut stream_name: *const ::core::ffi::c_char,
    mut fps: ::core::ffi::c_double,
) {
    if !g_initialized || stream_name.is_null() {
        return;
    }
    let mut idx: ::core::ffi::c_int = metrics_get_slot(stream_name);
    if idx < 0 as ::core::ffi::c_int {
        return;
    }
    pthread_rwlock_wrlock(&raw mut (*g_metrics.offset(idx as isize)).lock);
    (*g_metrics.offset(idx as isize)).configured_fps = fps;
    pthread_rwlock_unlock(&raw mut (*g_metrics.offset(idx as isize)).lock);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_snapshot_all(
    mut out_array: *mut stream_metrics_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !g_initialized || out_array.is_null() || max_count <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_max_streams && count < max_count {
        let mut m: *mut stream_metrics_t = g_metrics.offset(i as isize);
        if (*m).active {
            pthread_rwlock_rdlock(&raw mut (*m).lock);
            memcpy(
                out_array.offset(count as isize) as *mut ::core::ffi::c_void,
                m as *const ::core::ffi::c_void,
                ::core::mem::size_of::<stream_metrics_t>(),
            );
            pthread_rwlock_unlock(&raw mut (*m).lock);
            (*out_array.offset(count as isize)).health_status = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut (*m).health_status
            );
            (*out_array.offset(count as isize)).stream_up = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(&raw mut (*m).stream_up);
            (*out_array.offset(count as isize)).last_packet_ts = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut (*m).last_packet_ts
            );
            (*out_array.offset(count as isize)).last_frame_ts = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut (*m).last_frame_ts
            );
            (*out_array.offset(count as isize)).frames_total = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut (*m).frames_total
            );
            (*out_array.offset(count as isize)).frames_dropped = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut (*m).frames_dropped
            );
            (*out_array.offset(count as isize)).bytes_received = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut (*m).bytes_received
            );
            (*out_array.offset(count as isize)).reconnects_total = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut (*m).reconnects_total
            );
            (*out_array.offset(count as isize)).uptime_seconds = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut (*m).uptime_seconds
            );
            (*out_array.offset(count as isize)).error_decode = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut (*m).error_decode
            );
            (*out_array.offset(count as isize)).error_timeout = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut (*m).error_timeout
            );
            (*out_array.offset(count as isize)).error_protocol = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut (*m).error_protocol
            );
            (*out_array.offset(count as isize)).error_io = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(&raw mut (*m).error_io);
            (*out_array.offset(count as isize)).recording_active = ::core::intrinsics::atomic_load::<
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(
                &raw mut (*m).recording_active
            );
            (*out_array.offset(count as isize)).recording_bytes_written =
                ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).recording_bytes_written,
                );
            (*out_array.offset(count as isize)).recording_segments_total =
                ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).recording_segments_total,
                );
            (*out_array.offset(count as isize)).recording_gaps_total =
                ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).recording_gaps_total,
                );
            (*out_array.offset(count as isize)).last_completed_segment_ts =
                ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut (*m).last_completed_segment_ts,
                );
            count += 1;
        }
        i += 1;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_get_ring_data(
    mut stream_name: *const ::core::ffi::c_char,
    mut out: *mut metrics_ring_sample_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !g_initialized
        || stream_name.is_null()
        || out.is_null()
        || max_count <= 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut idx: ::core::ffi::c_int = find_active_slot(stream_name);
    if idx < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut m: *mut stream_metrics_t = g_metrics.offset(idx as isize);
    pthread_rwlock_rdlock(&raw mut (*m).lock);
    let mut avail: ::core::ffi::c_int = (*m).ring_count;
    let mut to_copy: ::core::ffi::c_int = if avail < max_count { avail } else { max_count };
    let mut start: ::core::ffi::c_int = 0;
    if avail < METRICS_RING_SIZE {
        start = 0 as ::core::ffi::c_int;
    } else {
        start = (*m).ring_head;
    }
    let mut skip: ::core::ffi::c_int = avail - to_copy;
    let mut read_pos: ::core::ffi::c_int = (start + skip) % METRICS_RING_SIZE;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < to_copy {
        *out.offset(i as isize) = (*m).ring[read_pos as usize];
        read_pos = (read_pos + 1 as ::core::ffi::c_int) % METRICS_RING_SIZE;
        i += 1;
    }
    pthread_rwlock_unlock(&raw mut (*m).lock);
    return to_copy;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn metrics_get_max_streams() -> ::core::ffi::c_int {
    return g_max_streams;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
