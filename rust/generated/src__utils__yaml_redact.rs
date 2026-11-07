unsafe extern "C" {
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_redact_alloc(
    mut src: *const ::core::ffi::c_char,
    mut len: size_t,
    mut out_len: *mut size_t,
) -> *mut ::core::ffi::c_char {
    let mut copy: *mut ::core::ffi::c_char =
        malloc(len.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    if copy.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !src.is_null() && len != 0 {
        memcpy(
            copy as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            len,
        );
    }
    *copy.offset(len as isize) = '\0' as ::core::ffi::c_char;
    if !out_len.is_null() {
        *out_len = len;
    }
    return copy;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn yaml_redact_is_available() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
