unsafe extern "C" {
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn force_cleanup_model_cache();
    unsafe fn init_detection_system() -> ::core::ffi::c_int;
    unsafe fn shutdown_detection_system();
    unsafe fn shutdown_motion_detection_system();
    unsafe fn shutdown_api_detection_system();
    unsafe fn init_detection_config() -> ::core::ffi::c_int;
    unsafe fn get_detection_config() -> *mut detection_config_t;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type ptrdiff_t = isize;
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
pub struct detection_config_t {
    pub buffer_pool_size: ::core::ffi::c_int,
    pub concurrent_detections: ::core::ffi::c_int,
    pub buffer_allocation_retries: ::core::ffi::c_int,
    pub downscale_factor_default: ::core::ffi::c_int,
    pub downscale_factor_cnn: ::core::ffi::c_int,
    pub downscale_factor_realnet: ::core::ffi::c_int,
    pub threshold_cnn: ::core::ffi::c_float,
    pub threshold_cnn_embedded: ::core::ffi::c_float,
    pub threshold_realnet: ::core::ffi::c_float,
    pub save_frames_for_debug: bool,
}
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub static mut active_detection_streams: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[unsafe(no_mangle)]
pub static mut active_detections: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut max_detections: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub static mut use_stream_based_detection: bool = r#true != 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_detection_integration() -> ::core::ffi::c_int {
    if init_detection_config() != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize detection configuration\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut config: *const detection_config_t = get_detection_config();
    if config.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to get detection configuration\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    max_detections = (*config).concurrent_detections;
    active_detection_streams =
        calloc(max_detections as size_t, MAX_STREAM_NAME as size_t) as *mut ::core::ffi::c_char;
    if active_detection_streams.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to allocate active detection streams array\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if init_detection_system() != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to initialize detection system\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(active_detection_streams as *mut ::core::ffi::c_void);
        active_detection_streams = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return -1 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Detection integration initialized with %d max concurrent detections\0".as_ptr()
            as *const ::core::ffi::c_char,
        max_detections,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn force_cleanup_sod_models() {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Forcing cleanup of all SOD models in global cache...\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    force_cleanup_model_cache();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"SOD model cleanup completed\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cleanup_detection_resources() {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting detection resources cleanup...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !active_detection_streams.is_null() {
        free(active_detection_streams as *mut ::core::ffi::c_void);
        active_detection_streams = ::core::ptr::null_mut::<::core::ffi::c_char>();
        active_detections = 0 as ::core::ffi::c_int;
    }
    force_cleanup_sod_models();
    shutdown_detection_system();
    shutdown_motion_detection_system();
    shutdown_api_detection_system();
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"API detection system has shutdown\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_active_detection_count() -> ::core::ffi::c_int {
    return active_detections;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_max_detection_count() -> ::core::ffi::c_int {
    return max_detections;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_detection_in_progress(
    mut stream_name: *const ::core::ffi::c_char,
) -> bool {
    if stream_name.is_null() || active_detection_streams.is_null() {
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < max_detections {
        let mut active_stream: *const ::core::ffi::c_char = active_detection_streams
            .offset((i as ptrdiff_t * MAX_STREAM_NAME as ptrdiff_t) as isize);
        if *active_stream.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && strcmp(active_stream, stream_name) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
