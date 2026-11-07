unsafe extern "C" {
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffmpeg_allocation_t {
    pub ptr: *mut ::core::ffi::c_void,
    pub r#type: *const ::core::ffi::c_char,
    pub location: *const ::core::ffi::c_char,
    pub line: ::core::ffi::c_int,
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
static mut tracked_allocations: *mut ffmpeg_allocation_t =
    ::core::ptr::null_mut::<ffmpeg_allocation_t>();
static mut allocation_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut allocation_capacity: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut allocation_mutex: pthread_mutex_t = pthread_mutex_t {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffmpeg_leak_detector_init() {
    pthread_mutex_lock(&raw mut allocation_mutex);
    allocation_capacity = 1000 as ::core::ffi::c_int;
    tracked_allocations = calloc(
        allocation_capacity as size_t,
        ::core::mem::size_of::<ffmpeg_allocation_t>(),
    ) as *mut ffmpeg_allocation_t;
    if tracked_allocations.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize FFmpeg leak detector\0".as_ptr() as *const ::core::ffi::c_char,
        );
        allocation_capacity = 0 as ::core::ffi::c_int;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"FFmpeg leak detector initialized with capacity for %d allocations\0".as_ptr()
                as *const ::core::ffi::c_char,
            allocation_capacity,
        );
    }
    pthread_mutex_unlock(&raw mut allocation_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffmpeg_leak_detector_cleanup() {
    pthread_mutex_lock(&raw mut allocation_mutex);
    if allocation_count > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"FFmpeg leak detector found %d potential leaks:\0".as_ptr()
                as *const ::core::ffi::c_char,
            allocation_count,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < allocation_count {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT,
                _log_stream_name,
                b"  Leak %d: %s at %p (from %s:%d)\0".as_ptr() as *const ::core::ffi::c_char,
                i + 1 as ::core::ffi::c_int,
                (*tracked_allocations.offset(i as isize)).r#type,
                (*tracked_allocations.offset(i as isize)).ptr,
                (*tracked_allocations.offset(i as isize)).location,
                (*tracked_allocations.offset(i as isize)).line,
            );
            i += 1;
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"FFmpeg leak detector found no leaks\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    free(tracked_allocations as *mut ::core::ffi::c_void);
    tracked_allocations = ::core::ptr::null_mut::<ffmpeg_allocation_t>();
    allocation_count = 0 as ::core::ffi::c_int;
    allocation_capacity = 0 as ::core::ffi::c_int;
    pthread_mutex_unlock(&raw mut allocation_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffmpeg_track_allocation(
    mut ptr: *mut ::core::ffi::c_void,
    mut r#type: *const ::core::ffi::c_char,
    mut location: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) {
    if ptr.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut allocation_mutex);
    if allocation_count >= allocation_capacity && !tracked_allocations.is_null() {
        let mut new_capacity: ::core::ffi::c_int = allocation_capacity * 2 as ::core::ffi::c_int;
        let mut new_array: *mut ffmpeg_allocation_t = realloc(
            tracked_allocations as *mut ::core::ffi::c_void,
            (new_capacity as size_t).wrapping_mul(::core::mem::size_of::<ffmpeg_allocation_t>()),
        ) as *mut ffmpeg_allocation_t;
        if !new_array.is_null() {
            tracked_allocations = new_array;
            allocation_capacity = new_capacity;
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT,
                _log_stream_name,
                b"FFmpeg leak detector expanded to capacity %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                allocation_capacity,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Failed to expand FFmpeg leak detector capacity\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            pthread_mutex_unlock(&raw mut allocation_mutex);
            return;
        }
    }
    if !tracked_allocations.is_null() && allocation_count < allocation_capacity {
        (*tracked_allocations.offset(allocation_count as isize)).ptr = ptr;
        (*tracked_allocations.offset(allocation_count as isize)).r#type = r#type;
        (*tracked_allocations.offset(allocation_count as isize)).location = location;
        (*tracked_allocations.offset(allocation_count as isize)).line = line;
        allocation_count += 1;
    }
    pthread_mutex_unlock(&raw mut allocation_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffmpeg_untrack_allocation(mut ptr: *const ::core::ffi::c_void) {
    if ptr.is_null() {
        return;
    }
    pthread_mutex_lock(&raw mut allocation_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < allocation_count {
        if (*tracked_allocations.offset(i as isize)).ptr == ptr as *mut ::core::ffi::c_void {
            if i < allocation_count - 1 as ::core::ffi::c_int {
                memmove(
                    tracked_allocations.offset(i as isize) as *mut ::core::ffi::c_void,
                    tracked_allocations.offset((i + 1 as ::core::ffi::c_int) as isize)
                        as *const ::core::ffi::c_void,
                    ((allocation_count - i - 1 as ::core::ffi::c_int) as size_t)
                        .wrapping_mul(::core::mem::size_of::<ffmpeg_allocation_t>()),
                );
            }
            allocation_count -= 1;
            break;
        } else {
            i += 1;
        }
    }
    pthread_mutex_unlock(&raw mut allocation_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffmpeg_get_allocation_count() -> ::core::ffi::c_int {
    pthread_mutex_lock(&raw mut allocation_mutex);
    let mut count: ::core::ffi::c_int = allocation_count;
    pthread_mutex_unlock(&raw mut allocation_mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffmpeg_dump_allocations() {
    pthread_mutex_lock(&raw mut allocation_mutex);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"FFmpeg allocation dump (%d allocations):\0".as_ptr() as *const ::core::ffi::c_char,
        allocation_count,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < allocation_count {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"  Allocation %d: %s at %p (from %s:%d)\0".as_ptr() as *const ::core::ffi::c_char,
            i + 1 as ::core::ffi::c_int,
            (*tracked_allocations.offset(i as isize)).r#type,
            (*tracked_allocations.offset(i as isize)).ptr,
            (*tracked_allocations.offset(i as isize)).location,
            (*tracked_allocations.offset(i as isize)).line,
        );
        i += 1;
    }
    pthread_mutex_unlock(&raw mut allocation_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ffmpeg_force_cleanup_all() {
    pthread_mutex_lock(&raw mut allocation_mutex);
    if allocation_count > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Skipping cleanup of %d FFmpeg allocations to avoid potential crashes\0".as_ptr()
                as *const ::core::ffi::c_char,
            allocation_count,
        );
        allocation_count = 0 as ::core::ffi::c_int;
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"No FFmpeg allocations to clean up\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    pthread_mutex_unlock(&raw mut allocation_mutex);
}
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
