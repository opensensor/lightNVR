unsafe extern "C" {
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn start_hls_unified_stream(
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn stop_hls_unified_stream(
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn restart_hls_unified_stream(
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_hls_stream(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Starting HLS stream for %s using unified thread architecture\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
    );
    return start_hls_unified_stream(stream_name);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn restart_hls_stream(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Restarting HLS stream for %s using unified thread architecture\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
    );
    return restart_hls_unified_stream(stream_name);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stop_hls_stream(
    mut stream_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT,
        _log_stream_name,
        b"Stopping HLS stream for %s using unified thread architecture\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
    );
    return stop_hls_unified_stream(stream_name);
}
