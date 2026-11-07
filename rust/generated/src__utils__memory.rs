unsafe extern "C" {
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn posix_fadvise(
        __fd: ::core::ffi::c_int,
        __offset: off_t,
        __len: off_t,
        __advise: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fdatasync(__fildes: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type uint8_t = u8;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const __POSIX_FADV_DONTNEED: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const POSIX_FADV_DONTNEED: ::core::ffi::c_int = __POSIX_FADV_DONTNEED;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
static mut total_memory_allocated: size_t = 0 as size_t;
static mut peak_memory_allocated: size_t = 0 as size_t;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_malloc(mut size: size_t) -> *mut ::core::ffi::c_void {
    if size == 0 as size_t {
        return NULL;
    }
    let mut ptr: *mut ::core::ffi::c_void = malloc(size);
    if ptr.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory allocation failed for size %zu\0".as_ptr() as *const ::core::ffi::c_char,
            size,
        );
    }
    return ptr;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_realloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    if size == 0 as size_t {
        free(ptr);
        return NULL;
    }
    let mut new_ptr: *mut ::core::ffi::c_void = realloc(ptr, size);
    if new_ptr.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory reallocation failed for size %zu\0".as_ptr() as *const ::core::ffi::c_char,
            size,
        );
    }
    return new_ptr;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    if nmemb == 0 as size_t || size == 0 as size_t {
        return NULL;
    }
    let mut ptr: *mut ::core::ffi::c_void = calloc(nmemb, size);
    if ptr.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory calloc failed for %zu x %zu bytes\0".as_ptr() as *const ::core::ffi::c_char,
            nmemb,
            size,
        );
    }
    return ptr;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_free(mut ptr: *mut ::core::ffi::c_void) {
    if !ptr.is_null() {
        free(ptr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn secure_zero_memory(mut ptr: *mut ::core::ffi::c_void, mut size: size_t) {
    if ptr.is_null() || size == 0 as size_t {
        return;
    }
    let mut p: *mut uint8_t = ptr as *mut uint8_t;
    loop {
        let c2rust_fresh0 = size;
        size = size.wrapping_sub(1);
        if c2rust_fresh0 == 0 {
            break;
        }
        let c2rust_fresh1 = p;
        p = p.offset(1);
        ::core::ptr::write_volatile(c2rust_fresh1, 0 as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_memory_allocation(mut size: size_t, mut is_allocation: bool) {
    if is_allocation {
        total_memory_allocated = total_memory_allocated.wrapping_add(size);
        if total_memory_allocated > peak_memory_allocated {
            peak_memory_allocated = total_memory_allocated;
        }
    } else if size <= total_memory_allocated {
        total_memory_allocated = total_memory_allocated.wrapping_sub(size);
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT,
            _log_stream_name,
            b"Memory tracking inconsistency: trying to free %zu bytes when only %zu are tracked\0"
                .as_ptr() as *const ::core::ffi::c_char,
            size,
            total_memory_allocated,
        );
        total_memory_allocated = 0 as size_t;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_total_memory_allocated() -> size_t {
    return total_memory_allocated;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_peak_memory_allocated() -> size_t {
    return peak_memory_allocated;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_cache_release_due(
    mut released_through: off_t,
    mut current_position: off_t,
    mut interval_bytes: off_t,
) -> bool {
    if released_through < 0 as off_t
        || current_position < released_through
        || interval_bytes <= 0 as off_t
    {
        return r#false != 0;
    }
    return current_position - released_through >= interval_bytes;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_cache_flush_and_release(
    mut fd: ::core::ffi::c_int,
    mut offset: off_t,
    mut length: off_t,
) -> ::core::ffi::c_int {
    if fd < 0 as ::core::ffi::c_int || offset < 0 as off_t || length < 0 as off_t {
        return EINVAL;
    }
    if fdatasync(fd) != 0 as ::core::ffi::c_int {
        return *__errno_location();
    }
    return posix_fadvise(fd, offset, length, POSIX_FADV_DONTNEED);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
