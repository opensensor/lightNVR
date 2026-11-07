unsafe extern "C" {
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn sqrtf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn build_motion_zone_mask(
        stream_name: *const ::core::ffi::c_char,
        grid_size: ::core::ffi::c_int,
        zone_mask: *mut bool,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
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
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
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
pub union pthread_mutexattr_t {
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
pub struct detection_t {
    pub label: [::core::ffi::c_char; 32],
    pub confidence: ::core::ffi::c_float,
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub width: ::core::ffi::c_float,
    pub height: ::core::ffi::c_float,
    pub track_id: ::core::ffi::c_int,
    pub zone_id: [::core::ffi::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_result_t {
    pub count: ::core::ffi::c_int,
    pub detections: [detection_t; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct motion_stream_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub prev_frame: *mut ::core::ffi::c_uchar,
    pub blur_buffer: *mut ::core::ffi::c_uchar,
    pub background: *mut ::core::ffi::c_uchar,
    pub frame_history: *mut frame_history_t,
    pub history_size: ::core::ffi::c_int,
    pub history_index: ::core::ffi::c_int,
    pub grid_scores: *mut ::core::ffi::c_float,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub channels: ::core::ffi::c_int,
    pub sensitivity: ::core::ffi::c_float,
    pub min_motion_area: ::core::ffi::c_float,
    pub max_motion_area: ::core::ffi::c_float,
    pub cooldown_time: ::core::ffi::c_int,
    pub blur_radius: ::core::ffi::c_int,
    pub noise_threshold: ::core::ffi::c_int,
    pub use_grid_detection: bool,
    pub grid_size: ::core::ffi::c_int,
    pub last_detection_time: time_t,
    pub last_background_update_time: time_t,
    pub enabled: bool,
    pub downscale_enabled: bool,
    pub downscale_factor: ::core::ffi::c_int,
    pub downscaled_width: ::core::ffi::c_int,
    pub downscaled_height: ::core::ffi::c_int,
    pub allocated_memory: size_t,
    pub peak_memory: size_t,
    pub last_frame_start: timespec,
    pub last_processing_time: ::core::ffi::c_float,
    pub avg_processing_time: ::core::ffi::c_float,
    pub peak_processing_time: ::core::ffi::c_float,
    pub frames_processed: ::core::ffi::c_int,
    pub mutex: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_history_t {
    pub frame: *mut ::core::ffi::c_uchar,
    pub timestamp: time_t,
}
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_DETECTIONS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const MAX_LABEL_LENGTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MAX_MOTION_STREAMS: ::core::ffi::c_int = MAX_STREAMS;
pub const DEFAULT_SENSITIVITY: ::core::ffi::c_float = 0.15f32;
pub const DEFAULT_MIN_MOTION_AREA: ::core::ffi::c_float = 0.005f32;
pub const DEFAULT_MAX_MOTION_AREA: ::core::ffi::c_float = 0.90f32;
pub const DEFAULT_COOLDOWN_TIME: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const DEFAULT_MOTION_HISTORY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const DEFAULT_BLUR_RADIUS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DEFAULT_NOISE_THRESHOLD: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const DEFAULT_USE_GRID_DETECTION: ::core::ffi::c_int = r#true;
pub const DEFAULT_GRID_SIZE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const DEFAULT_DOWNSCALE_ENABLED: ::core::ffi::c_int = r#true;
pub const DEFAULT_DOWNSCALE_FACTOR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MOTION_LABEL: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"motion\0") };
static mut motion_streams: [*mut motion_stream_t; 1024] =
    [::core::ptr::null_mut::<motion_stream_t>(); 1024];
static mut motion_streams_mutex: pthread_mutex_t = pthread_mutex_t {
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
static mut initialized: bool = r#false != 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn allocate_motion_stream() -> *mut motion_stream_t {
    let mut stream: *mut motion_stream_t =
        calloc(1 as size_t, ::core::mem::size_of::<motion_stream_t>()) as *mut motion_stream_t;
    if !stream.is_null() {
        pthread_mutex_init(
            &raw mut (*stream).mutex,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        );
    }
    return stream;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_motion_stream(mut stream: *mut motion_stream_t) {
    if stream.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut (*stream).mutex);
    if !(*stream).prev_frame.is_null() {
        free((*stream).prev_frame as *mut ::core::ffi::c_void);
        (*stream).prev_frame = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if !(*stream).blur_buffer.is_null() {
        free((*stream).blur_buffer as *mut ::core::ffi::c_void);
        (*stream).blur_buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if !(*stream).background.is_null() {
        free((*stream).background as *mut ::core::ffi::c_void);
        (*stream).background = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    if !(*stream).grid_scores.is_null() {
        free((*stream).grid_scores as *mut ::core::ffi::c_void);
        (*stream).grid_scores = ::core::ptr::null_mut::<::core::ffi::c_float>();
    }
    if !(*stream).frame_history.is_null() {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < (*stream).history_size {
            if !(*(*stream).frame_history.offset(j as isize))
                .frame
                .is_null()
            {
                free(
                    (*(*stream).frame_history.offset(j as isize)).frame as *mut ::core::ffi::c_void,
                );
                (*(*stream).frame_history.offset(j as isize)).frame =
                    ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            }
            j += 1;
        }
        free((*stream).frame_history as *mut ::core::ffi::c_void);
        (*stream).frame_history = ::core::ptr::null_mut::<frame_history_t>();
    }
    pthread_mutex_unlock(&raw mut (*stream).mutex);
    pthread_mutex_destroy(&raw mut (*stream).mutex);
    free(stream as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_motion_detection_system() -> ::core::ffi::c_int {
    if initialized {
        return 0 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut motion_streams_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_MOTION_STREAMS {
        motion_streams[i as usize] = ::core::ptr::null_mut::<motion_stream_t>();
        i += 1;
    }
    initialized = r#true != 0;
    pthread_mutex_unlock(&raw mut motion_streams_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Motion detection system initialized with embedded device optimizations\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shutdown_motion_detection_system() {
    if !initialized {
        return;
    }
    pthread_mutex_lock(&raw mut motion_streams_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_MOTION_STREAMS {
        if !motion_streams[i as usize].is_null() {
            free_motion_stream(motion_streams[i as usize]);
            motion_streams[i as usize] = ::core::ptr::null_mut::<motion_stream_t>();
        }
        i += 1;
    }
    initialized = r#false != 0;
    pthread_mutex_unlock(&raw mut motion_streams_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Motion detection system shutdown\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn get_motion_stream(
    mut stream_name: *const ::core::ffi::c_char,
) -> *mut motion_stream_t {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid stream name (NULL) for get_motion_stream\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<motion_stream_t>();
    }
    if !initialized {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Motion detection system not initialized for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
        );
        init_motion_detection_system();
    }
    pthread_mutex_lock(&raw mut motion_streams_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_MOTION_STREAMS {
        if !motion_streams[i as usize].is_null()
            && (*motion_streams[i as usize]).stream_name[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            && strcmp(
                &raw mut (**(&raw mut motion_streams as *mut *mut motion_stream_t)
                    .offset(i as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
            ) == 0 as ::core::ffi::c_int
        {
            pthread_mutex_unlock(&raw mut motion_streams_mutex);
            return motion_streams[i as usize];
        }
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < MAX_MOTION_STREAMS {
        if motion_streams[i_0 as usize].is_null() {
            motion_streams[i_0 as usize] = allocate_motion_stream();
            if motion_streams[i_0 as usize].is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to allocate memory for motion stream\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                pthread_mutex_unlock(&raw mut motion_streams_mutex);
                return ::core::ptr::null_mut::<motion_stream_t>();
            }
            safe_strcpy(
                &raw mut (**(&raw mut motion_streams as *mut *mut motion_stream_t)
                    .offset(i_0 as isize))
                .stream_name as *mut ::core::ffi::c_char,
                stream_name,
                MAX_STREAM_NAME as size_t,
                0 as size_t,
            );
            (*motion_streams[i_0 as usize]).sensitivity = DEFAULT_SENSITIVITY;
            (*motion_streams[i_0 as usize]).min_motion_area = DEFAULT_MIN_MOTION_AREA;
            (*motion_streams[i_0 as usize]).max_motion_area = DEFAULT_MAX_MOTION_AREA;
            (*motion_streams[i_0 as usize]).cooldown_time = DEFAULT_COOLDOWN_TIME;
            (*motion_streams[i_0 as usize]).history_size = DEFAULT_MOTION_HISTORY;
            (*motion_streams[i_0 as usize]).blur_radius = DEFAULT_BLUR_RADIUS;
            (*motion_streams[i_0 as usize]).noise_threshold = DEFAULT_NOISE_THRESHOLD;
            (*motion_streams[i_0 as usize]).use_grid_detection = DEFAULT_USE_GRID_DETECTION != 0;
            (*motion_streams[i_0 as usize]).grid_size = DEFAULT_GRID_SIZE;
            (*motion_streams[i_0 as usize]).enabled = r#false != 0;
            (*motion_streams[i_0 as usize]).downscale_enabled = DEFAULT_DOWNSCALE_ENABLED != 0;
            (*motion_streams[i_0 as usize]).downscale_factor = DEFAULT_DOWNSCALE_FACTOR;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Created new motion stream entry for %s\0".as_ptr() as *const ::core::ffi::c_char,
                stream_name,
            );
            pthread_mutex_unlock(&raw mut motion_streams_mutex);
            return motion_streams[i_0 as usize];
        }
        i_0 += 1;
    }
    pthread_mutex_unlock(&raw mut motion_streams_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_ERROR,
        LOG_COMPONENT,
        _log_stream_name,
        b"No available slots for motion detection stream: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
    );
    return ::core::ptr::null_mut::<motion_stream_t>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn configure_motion_detection(
    mut stream_name: *const ::core::ffi::c_char,
    mut sensitivity: ::core::ffi::c_float,
    mut min_motion_area: ::core::ffi::c_float,
    mut cooldown_time: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid stream name for configure_motion_detection\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: *mut motion_stream_t = get_motion_stream(stream_name);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get motion stream for %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*stream).mutex);
    (*stream).sensitivity = if sensitivity > 0.0f32 && sensitivity <= 1.0f32 {
        sensitivity
    } else {
        DEFAULT_SENSITIVITY
    };
    (*stream).min_motion_area = if min_motion_area > 0.0f32 && min_motion_area <= 1.0f32 {
        min_motion_area
    } else {
        DEFAULT_MIN_MOTION_AREA
    };
    (*stream).cooldown_time = if cooldown_time > 0 as ::core::ffi::c_int {
        cooldown_time
    } else {
        DEFAULT_COOLDOWN_TIME
    };
    pthread_mutex_unlock(&raw mut (*stream).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Configured motion detection for stream %s: sensitivity=%.2f, min_area=%.2f, cooldown=%d\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        (*stream).sensitivity as ::core::ffi::c_double,
        (*stream).min_motion_area as ::core::ffi::c_double,
        (*stream).cooldown_time,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn configure_advanced_motion_detection(
    mut stream_name: *const ::core::ffi::c_char,
    mut blur_radius: ::core::ffi::c_int,
    mut noise_threshold: ::core::ffi::c_int,
    mut use_grid_detection: bool,
    mut grid_size: ::core::ffi::c_int,
    mut history_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid stream name for configure_advanced_motion_detection\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: *mut motion_stream_t = get_motion_stream(stream_name);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get motion stream for %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*stream).mutex);
    let mut old_grid_size: ::core::ffi::c_int = (*stream).grid_size;
    let mut old_history_size: ::core::ffi::c_int = (*stream).history_size;
    (*stream).blur_radius =
        if blur_radius >= 0 as ::core::ffi::c_int && blur_radius <= 5 as ::core::ffi::c_int {
            blur_radius
        } else {
            DEFAULT_BLUR_RADIUS
        };
    (*stream).noise_threshold = if noise_threshold >= 0 as ::core::ffi::c_int
        && noise_threshold <= 50 as ::core::ffi::c_int
    {
        noise_threshold
    } else {
        DEFAULT_NOISE_THRESHOLD
    };
    (*stream).use_grid_detection = use_grid_detection;
    (*stream).grid_size =
        if grid_size >= 2 as ::core::ffi::c_int && grid_size <= 32 as ::core::ffi::c_int {
            grid_size
        } else {
            DEFAULT_GRID_SIZE
        };
    let mut new_history_size: ::core::ffi::c_int =
        if history_size > 0 as ::core::ffi::c_int && history_size <= 10 as ::core::ffi::c_int {
            history_size
        } else {
            DEFAULT_MOTION_HISTORY
        };
    if (*stream).grid_size != old_grid_size && !(*stream).grid_scores.is_null() {
        free((*stream).grid_scores as *mut ::core::ffi::c_void);
        (*stream).grid_scores = ::core::ptr::null_mut::<::core::ffi::c_float>();
    }
    if new_history_size != old_history_size && !(*stream).frame_history.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < old_history_size {
            if !(*(*stream).frame_history.offset(i as isize))
                .frame
                .is_null()
            {
                free(
                    (*(*stream).frame_history.offset(i as isize)).frame as *mut ::core::ffi::c_void,
                );
                (*(*stream).frame_history.offset(i as isize)).frame =
                    ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            }
            i += 1;
        }
        free((*stream).frame_history as *mut ::core::ffi::c_void);
        (*stream).frame_history = ::core::ptr::null_mut::<frame_history_t>();
    }
    (*stream).history_size = new_history_size;
    pthread_mutex_unlock(&raw mut (*stream).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Configured advanced motion detection for stream %s: blur=%d, noise=%d, grid=%s, grid_size=%d, history=%d\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        (*stream).blur_radius,
        (*stream).noise_threshold,
        if (*stream).use_grid_detection as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
        (*stream).grid_size,
        (*stream).history_size,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn configure_motion_detection_optimizations(
    mut stream_name: *const ::core::ffi::c_char,
    mut downscale_enabled: bool,
    mut downscale_factor: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid stream name for configure_motion_detection\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: *mut motion_stream_t = get_motion_stream(stream_name);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get motion stream for %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*stream).mutex);
    (*stream).downscale_enabled = downscale_enabled;
    (*stream).downscale_factor = if downscale_factor >= 1 as ::core::ffi::c_int
        && downscale_factor <= 4 as ::core::ffi::c_int
    {
        downscale_factor
    } else {
        DEFAULT_DOWNSCALE_FACTOR
    };
    if (*stream).width > 0 as ::core::ffi::c_int && (*stream).height > 0 as ::core::ffi::c_int {
        (*stream).downscaled_width = (*stream).width / (*stream).downscale_factor;
        (*stream).downscaled_height = (*stream).height / (*stream).downscale_factor;
        if (*stream).downscaled_width < 32 as ::core::ffi::c_int {
            (*stream).downscaled_width = 32 as ::core::ffi::c_int;
        }
        if (*stream).downscaled_height < 32 as ::core::ffi::c_int {
            (*stream).downscaled_height = 32 as ::core::ffi::c_int;
        }
    }
    pthread_mutex_unlock(&raw mut (*stream).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Configured motion detection optimizations for stream %s: downscale=%s, factor=%d\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        if (*stream).downscale_enabled as ::core::ffi::c_int != 0 {
            b"enabled\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char
        },
        (*stream).downscale_factor,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_motion_detection_enabled(
    mut stream_name: *const ::core::ffi::c_char,
    mut enabled: bool,
) -> ::core::ffi::c_int {
    if stream_name.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid stream name for set_motion_detection_enabled\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: *mut motion_stream_t = get_motion_stream(stream_name);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get motion stream for %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*stream).mutex);
    if !enabled && (*stream).enabled as ::core::ffi::c_int != 0 {
        if !(*stream).prev_frame.is_null() {
            free((*stream).prev_frame as *mut ::core::ffi::c_void);
            (*stream).prev_frame = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        if !(*stream).blur_buffer.is_null() {
            free((*stream).blur_buffer as *mut ::core::ffi::c_void);
            (*stream).blur_buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        if !(*stream).background.is_null() {
            free((*stream).background as *mut ::core::ffi::c_void);
            (*stream).background = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        if !(*stream).grid_scores.is_null() {
            free((*stream).grid_scores as *mut ::core::ffi::c_void);
            (*stream).grid_scores = ::core::ptr::null_mut::<::core::ffi::c_float>();
        }
        if !(*stream).frame_history.is_null() {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*stream).history_size {
                if !(*(*stream).frame_history.offset(i as isize))
                    .frame
                    .is_null()
                {
                    free(
                        (*(*stream).frame_history.offset(i as isize)).frame
                            as *mut ::core::ffi::c_void,
                    );
                    (*(*stream).frame_history.offset(i as isize)).frame =
                        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                }
                i += 1;
            }
            free((*stream).frame_history as *mut ::core::ffi::c_void);
            (*stream).frame_history = ::core::ptr::null_mut::<frame_history_t>();
        }
        (*stream).width = 0 as ::core::ffi::c_int;
        (*stream).height = 0 as ::core::ffi::c_int;
        (*stream).channels = 0 as ::core::ffi::c_int;
        (*stream).history_index = 0 as ::core::ffi::c_int;
        (*stream).downscaled_width = 0 as ::core::ffi::c_int;
        (*stream).downscaled_height = 0 as ::core::ffi::c_int;
    }
    (*stream).enabled = enabled;
    pthread_mutex_unlock(&raw mut (*stream).mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Motion detection %s for stream %s\0".as_ptr() as *const ::core::ffi::c_char,
        if enabled as ::core::ffi::c_int != 0 {
            b"enabled\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"disabled\0".as_ptr() as *const ::core::ffi::c_char
        },
        stream_name,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_motion_detection_enabled(
    mut stream_name: *const ::core::ffi::c_char,
) -> bool {
    if stream_name.is_null() {
        return r#false != 0;
    }
    let mut stream: *mut motion_stream_t = get_motion_stream(stream_name);
    if stream.is_null() {
        return r#false != 0;
    }
    pthread_mutex_lock(&raw mut (*stream).mutex);
    let mut enabled: bool = (*stream).enabled;
    pthread_mutex_unlock(&raw mut (*stream).mutex);
    return enabled;
}
unsafe extern "C" fn rgb_to_grayscale(
    mut rgb_data: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    let mut gray_data: *mut ::core::ffi::c_uchar =
        malloc((width as size_t).wrapping_mul(height as size_t)) as *mut ::core::ffi::c_uchar;
    if gray_data.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for grayscale conversion\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    let r_coeff: ::core::ffi::c_int =
        (0.299f32 * 256 as ::core::ffi::c_int as ::core::ffi::c_float) as ::core::ffi::c_int;
    let g_coeff: ::core::ffi::c_int =
        (0.587f32 * 256 as ::core::ffi::c_int as ::core::ffi::c_float) as ::core::ffi::c_int;
    let b_coeff: ::core::ffi::c_int =
        (0.114f32 * 256 as ::core::ffi::c_int as ::core::ffi::c_float) as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < height {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut rgb_idx: ::core::ffi::c_int = (y * width + x) * 3 as ::core::ffi::c_int;
            let mut gray_idx: ::core::ffi::c_int = y * width + x;
            let mut gray_value: ::core::ffi::c_int = r_coeff
                * *rgb_data.offset(rgb_idx as isize) as ::core::ffi::c_int
                + g_coeff
                    * *rgb_data.offset((rgb_idx + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                + b_coeff
                    * *rgb_data.offset((rgb_idx + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int;
            if gray_value > 255 as ::core::ffi::c_int {
                gray_value = 255 as ::core::ffi::c_int;
            }
            *gray_data.offset(gray_idx as isize) = gray_value as ::core::ffi::c_uchar;
            x += 1;
        }
        y += 1;
    }
    return gray_data;
}
unsafe extern "C" fn downscale_grayscale(
    mut src: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut factor: ::core::ffi::c_int,
    mut out_width: *mut ::core::ffi::c_int,
    mut out_height: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    if factor <= 1 as ::core::ffi::c_int {
        let mut copy: *mut ::core::ffi::c_uchar =
            malloc((width as size_t).wrapping_mul(height as size_t)) as *mut ::core::ffi::c_uchar;
        if copy.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to allocate memory for image copy\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        memcpy(
            copy as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (width as size_t).wrapping_mul(height as size_t),
        );
        *out_width = width;
        *out_height = height;
        return copy;
    }
    let mut new_width: ::core::ffi::c_int = width / factor;
    let mut new_height: ::core::ffi::c_int = height / factor;
    if new_width < 32 as ::core::ffi::c_int {
        new_width = 32 as ::core::ffi::c_int;
    }
    if new_height < 32 as ::core::ffi::c_int {
        new_height = 32 as ::core::ffi::c_int;
    }
    let mut dst: *mut ::core::ffi::c_uchar =
        malloc((new_width as size_t).wrapping_mul(new_height as size_t))
            as *mut ::core::ffi::c_uchar;
    if dst.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for downscaled image\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < new_height {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < new_width {
            let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut dy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while dy < factor && y * factor + dy < height {
                let mut dx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while dx < factor && x * factor + dx < width {
                    sum += *src.offset(((y * factor + dy) * width + (x * factor + dx)) as isize)
                        as ::core::ffi::c_int;
                    count += 1;
                    dx += 1;
                }
                dy += 1;
            }
            *dst.offset((y * new_width + x) as isize) = (if count > 0 as ::core::ffi::c_int {
                (sum / count) as ::core::ffi::c_uchar as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uchar;
            x += 1;
        }
        y += 1;
    }
    *out_width = new_width;
    *out_height = new_height;
    return dst;
}
unsafe extern "C" fn apply_box_blur(
    mut src: *const ::core::ffi::c_uchar,
    mut dst: *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut radius: ::core::ffi::c_int,
) {
    if radius <= 0 as ::core::ffi::c_int {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (width as size_t).wrapping_mul(height as size_t),
        );
        return;
    }
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < height {
        let mut row_offset: ::core::ffi::c_int = y * width;
        let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i <= radius && i < width {
            sum += *src.offset((row_offset + i) as isize) as ::core::ffi::c_int;
            count += 1;
            i += 1;
        }
        *dst.offset(row_offset as isize) = (sum / count) as ::core::ffi::c_uchar;
        let mut x: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while x < width {
            if x + radius < width {
                sum += *src.offset((row_offset + x + radius) as isize) as ::core::ffi::c_int;
                count += 1;
            }
            if x - radius - 1 as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                sum -= *src.offset((row_offset + x - radius - 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
                count -= 1;
            }
            *dst.offset((row_offset + x) as isize) = (sum / count) as ::core::ffi::c_uchar;
            x += 1;
        }
        y += 1;
    }
    let mut temp: *mut ::core::ffi::c_uchar =
        malloc((width as size_t).wrapping_mul(height as size_t)) as *mut ::core::ffi::c_uchar;
    if temp.is_null() {
        return;
    }
    memcpy(
        temp as *mut ::core::ffi::c_void,
        dst as *const ::core::ffi::c_void,
        (width as size_t).wrapping_mul(height as size_t),
    );
    let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while x_0 < width {
        let mut sum_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut count_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 <= radius && i_0 < height {
            sum_0 += *temp.offset((i_0 * width + x_0) as isize) as ::core::ffi::c_int;
            count_0 += 1;
            i_0 += 1;
        }
        *dst.offset(x_0 as isize) = (sum_0 / count_0) as ::core::ffi::c_uchar;
        let mut y_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while y_0 < height {
            if y_0 + radius < height {
                sum_0 +=
                    *temp.offset(((y_0 + radius) * width + x_0) as isize) as ::core::ffi::c_int;
                count_0 += 1;
            }
            if y_0 - radius - 1 as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
                sum_0 -= *temp
                    .offset(((y_0 - radius - 1 as ::core::ffi::c_int) * width + x_0) as isize)
                    as ::core::ffi::c_int;
                count_0 -= 1;
            }
            *dst.offset((y_0 * width + x_0) as isize) = (sum_0 / count_0) as ::core::ffi::c_uchar;
            y_0 += 1;
        }
        x_0 += 1;
    }
    free(temp as *mut ::core::ffi::c_void);
    temp = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
}
unsafe extern "C" fn update_background_model(
    mut background: *mut ::core::ffi::c_uchar,
    mut current: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut learning_rate: ::core::ffi::c_float,
) {
    if background.is_null() || current.is_null() {
        return;
    }
    let mut alpha: ::core::ffi::c_int =
        (learning_rate * 256 as ::core::ffi::c_int as ::core::ffi::c_float) as ::core::ffi::c_int;
    let mut inv_alpha: ::core::ffi::c_int = 256 as ::core::ffi::c_int - alpha;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < width * height {
        *background.offset(i as isize) =
            (inv_alpha * *background.offset(i as isize) as ::core::ffi::c_int
                + alpha * *current.offset(i as isize) as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        i += 1;
    }
}
unsafe extern "C" fn calculate_grid_motion(
    mut curr_frame: *const ::core::ffi::c_uchar,
    mut prev_frame: *const ::core::ffi::c_uchar,
    mut background: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut sensitivity: ::core::ffi::c_float,
    mut noise_threshold: ::core::ffi::c_int,
    mut grid_size: ::core::ffi::c_int,
    mut grid_scores: *mut ::core::ffi::c_float,
    mut motion_area: *mut ::core::ffi::c_float,
    mut zone_mask: *const bool,
) -> ::core::ffi::c_float {
    if curr_frame.is_null()
        || prev_frame.is_null()
        || background.is_null()
        || grid_scores.is_null()
        || motion_area.is_null()
    {
        return 0.0f32;
    }
    let mut cell_width: ::core::ffi::c_int = width / grid_size;
    let mut cell_height: ::core::ffi::c_int = height / grid_size;
    let mut total_cells: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cells_with_motion: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_cell_score: ::core::ffi::c_float = 0.0f32;
    let mut sensitivity_threshold: ::core::ffi::c_int =
        (sensitivity * 255.0f32) as ::core::ffi::c_int;
    let mut gy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while gy < grid_size {
        let mut gx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while gx < grid_size {
            let mut cell_idx: ::core::ffi::c_int = gy * grid_size + gx;
            if !zone_mask.is_null() && !*zone_mask.offset(cell_idx as isize) {
                *grid_scores.offset(cell_idx as isize) = 0.0f32;
            } else {
                total_cells += 1;
                let mut cell_start_x: ::core::ffi::c_int = gx * cell_width;
                let mut cell_start_y: ::core::ffi::c_int = gy * cell_height;
                let mut cell_end_x: ::core::ffi::c_int =
                    (gx + 1 as ::core::ffi::c_int) * cell_width;
                let mut cell_end_y: ::core::ffi::c_int =
                    (gy + 1 as ::core::ffi::c_int) * cell_height;
                if cell_end_x > width {
                    cell_end_x = width;
                }
                if cell_end_y > height {
                    cell_end_y = height;
                }
                let mut cell_pixels: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut changed_pixels: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut y: ::core::ffi::c_int = cell_start_y;
                while y < cell_end_y {
                    let mut x: ::core::ffi::c_int = cell_start_x;
                    while x < cell_end_x {
                        let mut idx: ::core::ffi::c_int = y * width + x;
                        let mut frame_diff: ::core::ffi::c_int =
                            abs(*curr_frame.offset(idx as isize) as ::core::ffi::c_int
                                - *prev_frame.offset(idx as isize) as ::core::ffi::c_int);
                        let mut bg_diff: ::core::ffi::c_int = abs(*curr_frame.offset(idx as isize)
                            as ::core::ffi::c_int
                            - *background.offset(idx as isize) as ::core::ffi::c_int);
                        let mut diff: ::core::ffi::c_int = if frame_diff > bg_diff {
                            frame_diff
                        } else {
                            bg_diff
                        };
                        if diff > noise_threshold {
                            if diff > sensitivity_threshold {
                                changed_pixels += 1;
                            }
                        }
                        cell_pixels += 1;
                        x += 2 as ::core::ffi::c_int;
                    }
                    y += 2 as ::core::ffi::c_int;
                }
                let mut cell_score: ::core::ffi::c_float = if cell_pixels > 0 as ::core::ffi::c_int
                {
                    changed_pixels as ::core::ffi::c_float / cell_pixels as ::core::ffi::c_float
                } else {
                    0.0f32
                };
                *grid_scores.offset(cell_idx as isize) = cell_score;
                if cell_score > 0.01f32 {
                    cells_with_motion += 1;
                    if cell_score > max_cell_score {
                        max_cell_score = cell_score;
                    }
                }
            }
            gx += 1;
        }
        gy += 1;
    }
    if total_cells > 0 as ::core::ffi::c_int {
        *motion_area =
            cells_with_motion as ::core::ffi::c_float / total_cells as ::core::ffi::c_float;
    } else {
        *motion_area = 0.0f32;
    }
    return max_cell_score;
}
unsafe extern "C" fn add_frame_to_history(
    mut stream: *mut motion_stream_t,
    mut frame: *const ::core::ffi::c_uchar,
    mut timestamp: time_t,
) {
    if stream.is_null() || frame.is_null() || (*stream).frame_history.is_null() {
        return;
    }
    if !(*(*stream)
        .frame_history
        .offset((*stream).history_index as isize))
    .frame
    .is_null()
    {
        free(
            (*(*stream)
                .frame_history
                .offset((*stream).history_index as isize))
            .frame as *mut ::core::ffi::c_void,
        );
    }
    (*(*stream)
        .frame_history
        .offset((*stream).history_index as isize))
    .frame = malloc(((*stream).width as size_t).wrapping_mul((*stream).height as size_t))
        as *mut ::core::ffi::c_uchar;
    if (*(*stream)
        .frame_history
        .offset((*stream).history_index as isize))
    .frame
    .is_null()
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate memory for frame history\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    memcpy(
        (*(*stream)
            .frame_history
            .offset((*stream).history_index as isize))
        .frame as *mut ::core::ffi::c_void,
        frame as *const ::core::ffi::c_void,
        ((*stream).width as size_t).wrapping_mul((*stream).height as size_t),
    );
    (*(*stream)
        .frame_history
        .offset((*stream).history_index as isize))
    .timestamp = timestamp;
    (*stream).history_index =
        ((*stream).history_index + 1 as ::core::ffi::c_int) % (*stream).history_size;
}
unsafe extern "C" fn update_memory_usage(mut stream: *mut motion_stream_t, mut allocated: size_t) {
    if stream.is_null() {
        return;
    }
    (*stream).allocated_memory = allocated;
    if allocated > (*stream).peak_memory {
        (*stream).peak_memory = allocated;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_motion(
    mut stream_name: *const ::core::ffi::c_char,
    mut frame_data: *const ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut channels: ::core::ffi::c_int,
    mut frame_time: time_t,
    mut result: *mut detection_result_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null()
        || frame_data.is_null()
        || result.is_null()
        || width <= 0 as ::core::ffi::c_int
        || height <= 0 as ::core::ffi::c_int
        || channels <= 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Invalid parameters for detect_motion\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<detection_result_t>(),
    );
    let mut stream: *mut motion_stream_t = get_motion_stream(stream_name);
    if stream.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get motion stream for %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*stream).mutex);
    let mut start_time: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut start_time);
    (*stream).last_frame_start = start_time;
    let mut current_memory: size_t = 0 as size_t;
    if !(*stream).enabled {
        pthread_mutex_unlock(&raw mut (*stream).mutex);
        return 0 as ::core::ffi::c_int;
    }
    if (*stream).last_detection_time > 0 as time_t
        && frame_time - (*stream).last_detection_time < (*stream).cooldown_time as time_t
    {
        pthread_mutex_unlock(&raw mut (*stream).mutex);
        return 0 as ::core::ffi::c_int;
    }
    let mut gray_frame: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if channels == 3 as ::core::ffi::c_int {
        gray_frame = rgb_to_grayscale(frame_data, width, height);
        if gray_frame.is_null() {
            pthread_mutex_unlock(&raw mut (*stream).mutex);
            return -1 as ::core::ffi::c_int;
        }
        current_memory =
            current_memory.wrapping_add((width as size_t).wrapping_mul(height as size_t));
    } else if channels == 1 as ::core::ffi::c_int {
        gray_frame =
            malloc((width as size_t).wrapping_mul(height as size_t)) as *mut ::core::ffi::c_uchar;
        if gray_frame.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to allocate memory for gray frame\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            pthread_mutex_unlock(&raw mut (*stream).mutex);
            return -1 as ::core::ffi::c_int;
        }
        memcpy(
            gray_frame as *mut ::core::ffi::c_void,
            frame_data as *const ::core::ffi::c_void,
            (width as size_t).wrapping_mul(height as size_t),
        );
        current_memory =
            current_memory.wrapping_add((width as size_t).wrapping_mul(height as size_t));
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Unsupported number of channels: %d\0".as_ptr() as *const ::core::ffi::c_char,
            channels,
        );
        pthread_mutex_unlock(&raw mut (*stream).mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut processing_frame: *mut ::core::ffi::c_uchar = gray_frame;
    let mut processing_width: ::core::ffi::c_int = width;
    let mut processing_height: ::core::ffi::c_int = height;
    if (*stream).downscale_enabled as ::core::ffi::c_int != 0
        && (*stream).downscale_factor > 1 as ::core::ffi::c_int
    {
        let mut downscaled: *mut ::core::ffi::c_uchar = downscale_grayscale(
            gray_frame,
            width,
            height,
            (*stream).downscale_factor,
            &raw mut processing_width,
            &raw mut processing_height,
        );
        if !downscaled.is_null() {
            free(gray_frame as *mut ::core::ffi::c_void);
            gray_frame = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            processing_frame = downscaled;
            current_memory = current_memory
                .wrapping_sub((width as size_t).wrapping_mul(height as size_t))
                .wrapping_add(
                    (processing_width as size_t).wrapping_mul(processing_height as size_t),
                );
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"Downscaled frame from %dx%d to %dx%d for motion detection\0".as_ptr()
                    as *const ::core::ffi::c_char,
                width,
                height,
                processing_width,
                processing_height,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to downscale frame, using original resolution\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if (*stream).prev_frame.is_null()
        || (*stream).width != processing_width
        || (*stream).height != processing_height
    {
        if !(*stream).prev_frame.is_null() {
            free((*stream).prev_frame as *mut ::core::ffi::c_void);
            (*stream).prev_frame = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        if !(*stream).blur_buffer.is_null() {
            free((*stream).blur_buffer as *mut ::core::ffi::c_void);
            (*stream).blur_buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        if !(*stream).background.is_null() {
            free((*stream).background as *mut ::core::ffi::c_void);
            (*stream).background = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        }
        if !(*stream).grid_scores.is_null() {
            free((*stream).grid_scores as *mut ::core::ffi::c_void);
            (*stream).grid_scores = ::core::ptr::null_mut::<::core::ffi::c_float>();
        }
        if !(*stream).frame_history.is_null() {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*stream).history_size {
                if !(*(*stream).frame_history.offset(i as isize))
                    .frame
                    .is_null()
                {
                    free(
                        (*(*stream).frame_history.offset(i as isize)).frame
                            as *mut ::core::ffi::c_void,
                    );
                }
                i += 1;
            }
            free((*stream).frame_history as *mut ::core::ffi::c_void);
            (*stream).frame_history = ::core::ptr::null_mut::<frame_history_t>();
        }
        (*stream).prev_frame =
            malloc((processing_width as size_t).wrapping_mul(processing_height as size_t))
                as *mut ::core::ffi::c_uchar;
        (*stream).blur_buffer =
            malloc((processing_width as size_t).wrapping_mul(processing_height as size_t))
                as *mut ::core::ffi::c_uchar;
        (*stream).background =
            malloc((processing_width as size_t).wrapping_mul(processing_height as size_t))
                as *mut ::core::ffi::c_uchar;
        if (*stream).prev_frame.is_null()
            || (*stream).blur_buffer.is_null()
            || (*stream).background.is_null()
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to allocate memory for motion detection buffers\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            if !(*stream).prev_frame.is_null() {
                free((*stream).prev_frame as *mut ::core::ffi::c_void);
                (*stream).prev_frame = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            }
            if !(*stream).blur_buffer.is_null() {
                free((*stream).blur_buffer as *mut ::core::ffi::c_void);
                (*stream).blur_buffer = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            }
            if !(*stream).background.is_null() {
                free((*stream).background as *mut ::core::ffi::c_void);
                (*stream).background = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            }
            free(processing_frame as *mut ::core::ffi::c_void);
            pthread_mutex_unlock(&raw mut (*stream).mutex);
            return -1 as ::core::ffi::c_int;
        }
        memcpy(
            (*stream).background as *mut ::core::ffi::c_void,
            processing_frame as *const ::core::ffi::c_void,
            (processing_width as size_t).wrapping_mul(processing_height as size_t),
        );
        memcpy(
            (*stream).prev_frame as *mut ::core::ffi::c_void,
            processing_frame as *const ::core::ffi::c_void,
            (processing_width as size_t).wrapping_mul(processing_height as size_t),
        );
        if (*stream).use_grid_detection {
            (*stream).grid_scores = malloc(
                ((*stream).grid_size as size_t)
                    .wrapping_mul((*stream).grid_size as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>()),
            ) as *mut ::core::ffi::c_float;
            if (*stream).grid_scores.is_null() {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Failed to allocate memory for grid scores\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                free(processing_frame as *mut ::core::ffi::c_void);
                pthread_mutex_unlock(&raw mut (*stream).mutex);
                return -1 as ::core::ffi::c_int;
            }
            memset(
                (*stream).grid_scores as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ((*stream).grid_size as size_t)
                    .wrapping_mul((*stream).grid_size as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>()),
            );
        }
        (*stream).frame_history = malloc(
            ((*stream).history_size as size_t)
                .wrapping_mul(::core::mem::size_of::<frame_history_t>()),
        ) as *mut frame_history_t;
        if (*stream).frame_history.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to allocate memory for frame history\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            if !(*stream).grid_scores.is_null() {
                free((*stream).grid_scores as *mut ::core::ffi::c_void);
                (*stream).grid_scores = ::core::ptr::null_mut::<::core::ffi::c_float>();
            }
            free(processing_frame as *mut ::core::ffi::c_void);
            pthread_mutex_unlock(&raw mut (*stream).mutex);
            return -1 as ::core::ffi::c_int;
        }
        memset(
            (*stream).frame_history as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*stream).history_size as size_t)
                .wrapping_mul(::core::mem::size_of::<frame_history_t>()),
        );
        (*stream).history_index = 0 as ::core::ffi::c_int;
        (*stream).width = processing_width;
        (*stream).height = processing_height;
        (*stream).channels = 1 as ::core::ffi::c_int;
        (*stream).downscaled_width = processing_width;
        (*stream).downscaled_height = processing_height;
        free(processing_frame as *mut ::core::ffi::c_void);
        pthread_mutex_unlock(&raw mut (*stream).mutex);
        return 0 as ::core::ffi::c_int;
    }
    apply_box_blur(
        processing_frame,
        (*stream).blur_buffer,
        processing_width,
        processing_height,
        (*stream).blur_radius,
    );
    let mut motion_detected: bool = r#false != 0;
    let mut motion_score: ::core::ffi::c_float = 0.0f32;
    let mut motion_area: ::core::ffi::c_float = 0.0f32;
    let mut zone_mask_buf: [bool; 1024] = [false; 1024];
    let mut zone_mask: *mut bool = ::core::ptr::null_mut::<bool>();
    if (*stream).use_grid_detection {
        let mut mask_size: ::core::ffi::c_int = (*stream).grid_size * (*stream).grid_size;
        if mask_size
            <= ::core::mem::size_of::<[bool; 1024]>().wrapping_div(::core::mem::size_of::<bool>())
                as ::core::ffi::c_int
        {
            zone_mask = &raw mut zone_mask_buf as *mut bool;
        } else {
            zone_mask = malloc((mask_size as size_t).wrapping_mul(::core::mem::size_of::<bool>()))
                as *mut bool;
        }
        if !zone_mask.is_null() {
            build_motion_zone_mask(stream_name, (*stream).grid_size, zone_mask);
        }
    }
    if (*stream).use_grid_detection {
        motion_score = calculate_grid_motion(
            (*stream).blur_buffer,
            (*stream).prev_frame,
            (*stream).background,
            processing_width,
            processing_height,
            (*stream).sensitivity,
            (*stream).noise_threshold,
            (*stream).grid_size,
            (*stream).grid_scores,
            &raw mut motion_area,
            zone_mask,
        );
        motion_detected = motion_area >= (*stream).min_motion_area
            && motion_area <= (*stream).max_motion_area
            && motion_score > 0.01f32;
    } else {
        let mut changed_pixels: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pixel_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < processing_height {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < processing_width {
                let mut idx: ::core::ffi::c_int = y * processing_width + x;
                let mut frame_diff: ::core::ffi::c_int = abs(*(*stream)
                    .blur_buffer
                    .offset(idx as isize)
                    as ::core::ffi::c_int
                    - *(*stream).prev_frame.offset(idx as isize) as ::core::ffi::c_int);
                let mut bg_diff: ::core::ffi::c_int = abs(*(*stream)
                    .blur_buffer
                    .offset(idx as isize)
                    as ::core::ffi::c_int
                    - *(*stream).background.offset(idx as isize) as ::core::ffi::c_int);
                let mut diff: ::core::ffi::c_int = if frame_diff > bg_diff {
                    frame_diff
                } else {
                    bg_diff
                };
                if diff > (*stream).noise_threshold {
                    if diff as ::core::ffi::c_float > (*stream).sensitivity * 255.0f32 {
                        changed_pixels += 1;
                    }
                }
                x += 2 as ::core::ffi::c_int;
            }
            y += 2 as ::core::ffi::c_int;
        }
        pixel_count = processing_width * processing_height / 4 as ::core::ffi::c_int;
        motion_area = changed_pixels as ::core::ffi::c_float / pixel_count as ::core::ffi::c_float;
        motion_score = motion_area;
        motion_detected =
            motion_area >= (*stream).min_motion_area && motion_area <= (*stream).max_motion_area;
    }
    add_frame_to_history(stream, (*stream).blur_buffer, frame_time);
    let mut base_rate: ::core::ffi::c_float = if motion_detected as ::core::ffi::c_int != 0 {
        0.01f32
    } else {
        0.05f32
    };
    let mut dt: ::core::ffi::c_float = if (*stream).last_background_update_time > 0 as time_t {
        (frame_time - (*stream).last_background_update_time) as ::core::ffi::c_float
    } else {
        1.0f32
    };
    if dt < 0.0f32 {
        dt = 0.0f32;
    }
    if dt > 5.0f32 {
        dt = 5.0f32;
    }
    let mut learning_rate: ::core::ffi::c_float = base_rate * dt;
    if learning_rate > 0.25f32 {
        learning_rate = 0.25f32;
    }
    update_background_model(
        (*stream).background,
        (*stream).blur_buffer,
        processing_width,
        processing_height,
        learning_rate,
    );
    (*stream).last_background_update_time = frame_time;
    memcpy(
        (*stream).prev_frame as *mut ::core::ffi::c_void,
        (*stream).blur_buffer as *const ::core::ffi::c_void,
        (processing_width as size_t).wrapping_mul(processing_height as size_t),
    );
    if motion_detected {
        (*stream).last_detection_time = frame_time;
        if (*stream).use_grid_detection as ::core::ffi::c_int != 0
            && (*stream).grid_size > 0 as ::core::ffi::c_int
        {
            let mut gs: ::core::ffi::c_int = (*stream).grid_size;
            let mut total: ::core::ffi::c_int = gs * gs;
            let mut labels: [::core::ffi::c_int; 1024] = [0; 1024];
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < total {
                labels[i_0 as usize] = -1 as ::core::ffi::c_int;
                i_0 += 1;
            }
            let mut num_components: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut gy: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while gy < gs {
                let mut gx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while gx < gs {
                    let mut idx_0: ::core::ffi::c_int = gy * gs + gx;
                    if !(*(*stream).grid_scores.offset(idx_0 as isize) <= 0.01f32
                        || labels[idx_0 as usize] >= 0 as ::core::ffi::c_int)
                    {
                        let c2rust_fresh0 = num_components;
                        num_components += 1;
                        let mut comp: ::core::ffi::c_int = c2rust_fresh0;
                        let mut queue: [::core::ffi::c_int; 1024] = [0; 1024];
                        let mut head: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let mut tail: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let c2rust_fresh1 = tail;
                        tail += 1;
                        queue[c2rust_fresh1 as usize] = idx_0;
                        labels[idx_0 as usize] = comp;
                        while head < tail {
                            let c2rust_fresh2 = head;
                            head += 1;
                            let mut ci: ::core::ffi::c_int = queue[c2rust_fresh2 as usize];
                            let mut cy: ::core::ffi::c_int = ci / gs;
                            let mut cx: ::core::ffi::c_int = ci % gs;
                            let dx: [::core::ffi::c_int; 4] = [
                                -1 as ::core::ffi::c_int,
                                1 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                            ];
                            let dy: [::core::ffi::c_int; 4] = [
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                -1 as ::core::ffi::c_int,
                                1 as ::core::ffi::c_int,
                            ];
                            let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while d < 4 as ::core::ffi::c_int {
                                let mut nx: ::core::ffi::c_int = cx + dx[d as usize];
                                let mut ny: ::core::ffi::c_int = cy + dy[d as usize];
                                if !(nx < 0 as ::core::ffi::c_int
                                    || nx >= gs
                                    || ny < 0 as ::core::ffi::c_int
                                    || ny >= gs)
                                {
                                    let mut ni: ::core::ffi::c_int = ny * gs + nx;
                                    if labels[ni as usize] < 0 as ::core::ffi::c_int
                                        && *(*stream).grid_scores.offset(ni as isize) > 0.01f32
                                    {
                                        labels[ni as usize] = comp;
                                        let c2rust_fresh3 = tail;
                                        tail += 1;
                                        queue[c2rust_fresh3 as usize] = ni;
                                    }
                                }
                                d += 1;
                            }
                        }
                        if num_components >= MAX_DETECTIONS {
                            break;
                        }
                    }
                    gx += 1;
                }
                if num_components >= MAX_DETECTIONS {
                    break;
                }
                gy += 1;
            }
            let mut clamp: ::core::ffi::c_int = if num_components > MAX_DETECTIONS {
                MAX_DETECTIONS
            } else {
                num_components
            };
            (*result).count = 0 as ::core::ffi::c_int;
            let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while c < clamp {
                let mut min_x: ::core::ffi::c_int = gs;
                let mut min_y: ::core::ffi::c_int = gs;
                let mut max_x: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
                let mut max_y: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
                let mut peak_score: ::core::ffi::c_float = 0.0f32;
                let mut cell_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_1 < total {
                    if labels[i_1 as usize] == c {
                        let mut cy_0: ::core::ffi::c_int = i_1 / gs;
                        let mut cx_0: ::core::ffi::c_int = i_1 % gs;
                        if cx_0 < min_x {
                            min_x = cx_0;
                        }
                        if cx_0 > max_x {
                            max_x = cx_0;
                        }
                        if cy_0 < min_y {
                            min_y = cy_0;
                        }
                        if cy_0 > max_y {
                            max_y = cy_0;
                        }
                        if *(*stream).grid_scores.offset(i_1 as isize) > peak_score {
                            peak_score = *(*stream).grid_scores.offset(i_1 as isize);
                        }
                        cell_count += 1;
                    }
                    i_1 += 1;
                }
                if !(max_x < 0 as ::core::ffi::c_int || cell_count == 0 as ::core::ffi::c_int) {
                    let mut conf: ::core::ffi::c_float = sqrtf(peak_score);
                    if conf > 1.0f32 {
                        conf = 1.0f32;
                    }
                    let mut bx: ::core::ffi::c_float =
                        min_x as ::core::ffi::c_float / gs as ::core::ffi::c_float;
                    let mut by: ::core::ffi::c_float =
                        min_y as ::core::ffi::c_float / gs as ::core::ffi::c_float;
                    let mut bw: ::core::ffi::c_float = (max_x - min_x + 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_float
                        / gs as ::core::ffi::c_float;
                    let mut bh: ::core::ffi::c_float = (max_y - min_y + 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_float
                        / gs as ::core::ffi::c_float;
                    let mut ri: ::core::ffi::c_int = (*result).count;
                    safe_strcpy(
                        &raw mut (*(&raw mut (*result).detections as *mut detection_t)
                            .offset(ri as isize))
                        .label as *mut ::core::ffi::c_char,
                        MOTION_LABEL.as_ptr(),
                        MAX_LABEL_LENGTH as size_t,
                        0 as size_t,
                    );
                    (*result).detections[ri as usize].confidence = conf;
                    (*result).detections[ri as usize].x = bx;
                    (*result).detections[ri as usize].y = by;
                    (*result).detections[ri as usize].width = bw;
                    (*result).detections[ri as usize].height = bh;
                    (*result).count += 1;
                }
                c += 1;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Motion detected in stream %s: score=%.3f, area=%.2f%%, clusters=%d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
                motion_score as ::core::ffi::c_double,
                (motion_area * 100.0f32) as ::core::ffi::c_double,
                (*result).count,
            );
        } else {
            let mut conf_0: ::core::ffi::c_float = sqrtf(motion_score);
            if conf_0 > 1.0f32 {
                conf_0 = 1.0f32;
            }
            (*result).count = 1 as ::core::ffi::c_int;
            safe_strcpy(
                &raw mut (*(&raw mut (*result).detections as *mut detection_t).offset(0isize)).label
                    as *mut ::core::ffi::c_char,
                MOTION_LABEL.as_ptr(),
                MAX_LABEL_LENGTH as size_t,
                0 as size_t,
            );
            (*result).detections[0usize].confidence = conf_0;
            (*result).detections[0usize].x = 0.0f32;
            (*result).detections[0usize].y = 0.0f32;
            (*result).detections[0usize].width = 1.0f32;
            (*result).detections[0usize].height = 1.0f32;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT,
                _log_stream_name,
                b"Motion detected in stream %s: score=%.3f, area=%.2f%%\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_name,
                motion_score as ::core::ffi::c_double,
                (motion_area * 100.0f32) as ::core::ffi::c_double,
            );
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT,
            _log_stream_name,
            b"No motion in stream %s: score=%.3f, area=%.2f%%, threshold=%.2f\0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_name,
            motion_score as ::core::ffi::c_double,
            (motion_area * 100.0f32) as ::core::ffi::c_double,
            (*stream).min_motion_area as ::core::ffi::c_double,
        );
    }
    if !zone_mask.is_null() && zone_mask != &raw mut zone_mask_buf as *mut bool {
        free(zone_mask as *mut ::core::ffi::c_void);
    }
    free(processing_frame as *mut ::core::ffi::c_void);
    let mut end_time: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut end_time);
    let mut processing_time: ::core::ffi::c_float =
        (end_time.tv_sec - start_time.tv_sec) as ::core::ffi::c_float * 1000.0f32
            + (end_time.tv_nsec - start_time.tv_nsec) as ::core::ffi::c_float / 1000000.0f32;
    (*stream).last_processing_time = processing_time;
    (*stream).frames_processed += 1;
    (*stream).avg_processing_time = ((*stream).avg_processing_time
        * ((*stream).frames_processed - 1 as ::core::ffi::c_int) as ::core::ffi::c_float
        + processing_time)
        / (*stream).frames_processed as ::core::ffi::c_float;
    if processing_time > (*stream).peak_processing_time {
        (*stream).peak_processing_time = processing_time;
    }
    update_memory_usage(stream, current_memory);
    pthread_mutex_unlock(&raw mut (*stream).mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_motion_detection_memory_usage(
    mut stream_name: *const ::core::ffi::c_char,
    mut allocated_memory: *mut size_t,
    mut peak_memory: *mut size_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null() || allocated_memory.is_null() || peak_memory.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: *mut motion_stream_t = get_motion_stream(stream_name);
    if stream.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*stream).mutex);
    *allocated_memory = (*stream).allocated_memory;
    *peak_memory = (*stream).peak_memory;
    pthread_mutex_unlock(&raw mut (*stream).mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_motion_detection_cpu_usage(
    mut stream_name: *const ::core::ffi::c_char,
    mut avg_processing_time: *mut ::core::ffi::c_float,
    mut peak_processing_time: *mut ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    if stream_name.is_null() || avg_processing_time.is_null() || peak_processing_time.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: *mut motion_stream_t = get_motion_stream(stream_name);
    if stream.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*stream).mutex);
    *avg_processing_time = (*stream).avg_processing_time;
    *peak_processing_time = (*stream).peak_processing_time;
    pthread_mutex_unlock(&raw mut (*stream).mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_motion_detection_statistics(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if stream_name.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut stream: *mut motion_stream_t = get_motion_stream(stream_name);
    if stream.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut (*stream).mutex);
    (*stream).avg_processing_time = 0.0f32;
    (*stream).peak_processing_time = 0.0f32;
    (*stream).peak_memory = (*stream).allocated_memory;
    (*stream).frames_processed = 0 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut (*stream).mutex);
    return 0 as ::core::ffi::c_int;
}
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
