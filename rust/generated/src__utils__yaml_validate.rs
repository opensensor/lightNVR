unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_validation_result_t {
    pub valid: ::core::ffi::c_int,
    pub err_message: [::core::ffi::c_char; 256],
    pub err_line: ::core::ffi::c_int,
    pub err_column: ::core::ffi::c_int,
    pub duplicate_keys: ::core::ffi::c_int,
    pub non_mapping_root: ::core::ffi::c_int,
    pub parse_error: ::core::ffi::c_int,
    pub warnings: [[::core::ffi::c_char; 192]; 16],
    pub warning_count: ::core::ffi::c_int,
}
unsafe extern "C" fn set_err(
    mut err_buf: *mut ::core::ffi::c_char,
    mut err_size: size_t,
    mut msg: *const ::core::ffi::c_char,
) {
    if err_buf.is_null() || err_size == 0 as size_t || msg.is_null() {
        return;
    }
    let mut n: size_t = strlen(msg);
    if n >= err_size {
        n = err_size.wrapping_sub(1 as size_t);
    }
    memcpy(
        err_buf as *mut ::core::ffi::c_void,
        msg as *const ::core::ffi::c_void,
        n,
    );
    *err_buf.offset(n as isize) = '\0' as ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_validate_is_available() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_validate_str(
    mut src: *const ::core::ffi::c_char,
    mut len: size_t,
    mut err_buf: *mut ::core::ffi::c_char,
    mut err_size: size_t,
) -> ::core::ffi::c_int {
    set_err(
        err_buf,
        err_size,
        b"libyaml not available; validation disabled\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_is_mapping_root(
    mut src: *const ::core::ffi::c_char,
    mut len: size_t,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_validate_go2rtc_override(
    mut src: *const ::core::ffi::c_char,
    mut len: size_t,
    mut out: *mut yaml_validation_result_t,
) {
    if out.is_null() {
        return;
    }
    memset(
        out as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_validation_result_t>(),
    );
    (*out).valid = -1 as ::core::ffi::c_int;
    snprintf(
        &raw mut (*out).err_message as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"libyaml not available at build time; pre-save validation skipped\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
