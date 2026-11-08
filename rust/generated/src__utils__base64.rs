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
static mut BASE64URL_ALPHABET: [::core::ffi::c_char; 65] = unsafe {
    ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
        *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_\0",
    )
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn base64url_encode(
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
    if input_len
        > (SIZE_MAX as size_t)
            .wrapping_sub(3 as size_t)
            .wrapping_div(4 as size_t)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut needed: size_t = input_len
        .wrapping_mul(4 as size_t)
        .wrapping_add(2 as size_t)
        .wrapping_div(3 as size_t)
        .wrapping_add(1 as size_t);
    if dst_size < needed {
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
        let c2rust_fresh12 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh12 as isize) =
            BASE64URL_ALPHABET[(triple >> 18 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        let c2rust_fresh13 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh13 as isize) =
            BASE64URL_ALPHABET[(triple >> 12 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        let c2rust_fresh14 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh14 as isize) =
            BASE64URL_ALPHABET[(triple >> 6 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        let c2rust_fresh15 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh15 as isize) =
            BASE64URL_ALPHABET[(triple & 0x3f as uint32_t) as usize];
        i = i.wrapping_add(3 as size_t);
    }
    let mut remaining: size_t = input_len.wrapping_sub(i);
    if remaining > 0 as size_t {
        let mut triple_0: uint32_t =
            (*input.offset(i as isize) as uint32_t) << 16 as ::core::ffi::c_int;
        if remaining == 2 as size_t {
            triple_0 |= (*input.offset(i.wrapping_add(1 as size_t) as isize) as uint32_t)
                << 8 as ::core::ffi::c_int;
        }
        let c2rust_fresh16 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh16 as isize) =
            BASE64URL_ALPHABET[(triple_0 >> 18 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        let c2rust_fresh17 = written;
        written = written.wrapping_add(1);
        *dst.offset(c2rust_fresh17 as isize) =
            BASE64URL_ALPHABET[(triple_0 >> 12 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        if remaining == 2 as size_t {
            let c2rust_fresh18 = written;
            written = written.wrapping_add(1);
            *dst.offset(c2rust_fresh18 as isize) = BASE64URL_ALPHABET
                [(triple_0 >> 6 as ::core::ffi::c_int & 0x3f as uint32_t) as usize];
        }
    }
    *dst.offset(written as isize) = '\0' as ::core::ffi::c_char;
    if !out_len.is_null() {
        *out_len = written;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn base64url_value(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    if c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
        && c as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
    {
        return c as ::core::ffi::c_int - 'A' as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
        && c as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
    {
        return c as ::core::ffi::c_int - 'a' as ::core::ffi::c_int + 26 as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
        && c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
    {
        return c as ::core::ffi::c_int - '0' as ::core::ffi::c_int + 52 as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
        return 62 as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int == '_' as ::core::ffi::c_int {
        return 63 as ::core::ffi::c_int;
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn base64url_decode(
    mut src: *const ::core::ffi::c_char,
    mut src_len: size_t,
    mut dst: *mut ::core::ffi::c_uchar,
    mut dst_size: size_t,
    mut out_len: *mut size_t,
) -> ::core::ffi::c_int {
    if !out_len.is_null() {
        *out_len = 0 as size_t;
    }
    if src.is_null() && src_len > 0 as size_t || dst.is_null() && src_len > 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    if src_len.wrapping_rem(4 as size_t) == 1 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut needed: size_t = src_len
        .wrapping_div(4 as size_t)
        .wrapping_mul(3 as size_t)
        .wrapping_add(if src_len.wrapping_rem(4 as size_t) != 0 {
            src_len.wrapping_rem(4 as size_t).wrapping_sub(1 as size_t)
        } else {
            0 as size_t
        });
    if needed > dst_size {
        return -1 as ::core::ffi::c_int;
    }
    let mut written: size_t = 0 as size_t;
    let mut buffer: uint32_t = 0 as uint32_t;
    let mut bits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    while i < src_len {
        let mut value: ::core::ffi::c_int = base64url_value(*src.offset(i as isize));
        if value < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        buffer = buffer << 6 as ::core::ffi::c_int | value as uint32_t;
        bits += 6 as ::core::ffi::c_int;
        if bits >= 8 as ::core::ffi::c_int {
            bits -= 8 as ::core::ffi::c_int;
            let c2rust_fresh19 = written;
            written = written.wrapping_add(1);
            *dst.offset(c2rust_fresh19 as isize) =
                (buffer >> bits & 0xff as uint32_t) as ::core::ffi::c_uchar;
        }
        i = i.wrapping_add(1);
    }
    if bits > 0 as ::core::ffi::c_int
        && buffer & ((1 as uint32_t) << bits).wrapping_sub(1 as uint32_t) != 0 as uint32_t
    {
        return -1 as ::core::ffi::c_int;
    }
    if !out_len.is_null() {
        *out_len = written;
    }
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
