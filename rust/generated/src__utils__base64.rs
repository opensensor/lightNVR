unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type uint32_t = u32;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
static mut BASE64_ALPHABET: [::core::ffi::c_char; 65] = unsafe {
    ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
        *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0",
    )
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn base64_encoded_size(mut input_len: size_t) -> size_t {
    if input_len
        > (SIZE_MAX as size_t)
            .wrapping_sub(1 as size_t)
            .wrapping_div(4 as size_t)
            .wrapping_mul(3 as size_t)
            .wrapping_sub(2 as size_t)
    {
        return 0 as size_t;
    }
    return input_len
        .wrapping_add(2 as size_t)
        .wrapping_div(3 as size_t)
        .wrapping_mul(4 as size_t)
        .wrapping_add(1 as size_t);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn base64_encode(
    mut input: *const ::core::ffi::c_uchar,
    mut input_len: size_t,
    mut dst: *mut ::core::ffi::c_char,
    mut dst_size: size_t,
    mut out_len: *mut size_t,
) -> ::core::ffi::c_int {
    if !out_len.is_null() {
        *out_len = 0 as size_t;
    }
    if dst.is_null() || dst_size == 0 as size_t || input.is_null() && input_len > 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut needed: size_t = base64_encoded_size(input_len);
    if needed == 0 as size_t || dst_size < needed {
        return -1 as ::core::ffi::c_int;
    }
    let mut written: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i.wrapping_add(2 as size_t) < input_len {
        let mut triple: uint32_t = (*input.offset(i as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
            | (*input.offset(i.wrapping_add(1 as size_t) as isize) as uint32_t)
                << 8 as ::core::ffi::c_int
            | *input.offset(i.wrapping_add(2 as size_t) as isize) as uint32_t;
        let c2rust_fresh0 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh0 as isize) =
            BASE64_ALPHABET[(triple >> 18 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        let c2rust_fresh1 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh1 as isize) =
            BASE64_ALPHABET[(triple >> 12 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        let c2rust_fresh2 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh2 as isize) =
            BASE64_ALPHABET[(triple >> 6 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        let c2rust_fresh3 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh3 as isize) = BASE64_ALPHABET[(triple & 0x3f as uint32_t) as usize];
        i = i.wrapping_add(3 as size_t);
    }
    let mut remaining: size_t = input_len.wrapping_sub(i);
    if remaining == 1 as size_t {
        let mut triple_0: uint32_t =
            (*input.offset(i as isize) as uint32_t) << 16 as ::core::ffi::c_int;
        let c2rust_fresh4 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh4 as isize) =
            BASE64_ALPHABET[(triple_0 >> 18 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        let c2rust_fresh5 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh5 as isize) =
            BASE64_ALPHABET[(triple_0 >> 12 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        let c2rust_fresh6 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh6 as isize) = '=' as ::core::ffi::c_char;
        let c2rust_fresh7 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh7 as isize) = '=' as ::core::ffi::c_char;
    } else if remaining == 2 as size_t {
        let mut triple_1: uint32_t = (*input.offset(i as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
            | (*input.offset(i.wrapping_add(1 as size_t) as isize) as uint32_t)
                << 8 as ::core::ffi::c_int;
        let c2rust_fresh8 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh8 as isize) =
            BASE64_ALPHABET[(triple_1 >> 18 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        let c2rust_fresh9 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh9 as isize) =
            BASE64_ALPHABET[(triple_1 >> 12 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        let c2rust_fresh10 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh10 as isize) =
            BASE64_ALPHABET[(triple_1 >> 6 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        let c2rust_fresh11 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh11 as isize) = '=' as ::core::ffi::c_char;
    }
    *dst.offset(written as isize) = '\0' as ::core::ffi::c_char;
    if !out_len.is_null() {
        *out_len = written;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn base64_encode_alloc(
    mut input: *const ::core::ffi::c_uchar,
    mut input_len: size_t,
) -> *mut ::core::ffi::c_char {
    let mut needed: size_t = base64_encoded_size(input_len);
    if needed == 0 as size_t {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut encoded: *mut ::core::ffi::c_char = malloc(needed) as *mut ::core::ffi::c_char;
    if encoded.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if base64_encode(
        input,
        input_len,
        encoded,
        needed,
        ::core::ptr::null_mut::<size_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        free(encoded as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return encoded;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
