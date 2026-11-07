unsafe extern "C" {
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
}
pub type size_t = usize;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const _ISupper: Self = Self(256);
    pub const _ISlower: Self = Self(512);
    pub const _ISalpha: Self = Self(1024);
    pub const _ISdigit: Self = Self(2048);
    pub const _ISxdigit: Self = Self(4096);
    pub const _ISspace: Self = Self(8192);
    pub const _ISprint: Self = Self(16384);
    pub const _ISgraph: Self = Self(32768);
    pub const _ISblank: Self = Self(1);
    pub const _IScntrl: Self = Self(2);
    pub const _ISpunct: Self = Self(4);
    pub const _ISalnum: Self = Self(8);
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
#[inline]
unsafe extern "C" fn ltrim_pos(
    mut input: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if input.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let mut start: *mut ::core::ffi::c_uchar = input as *mut ::core::ffi::c_uchar;
    while *start as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc()).offset(*start as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISgraph.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
    {
        start = start.offset(1);
    }
    return start as *const ::core::ffi::c_char;
}
#[inline]
unsafe extern "C" fn rtrim_pos(
    mut input: *const ::core::ffi::c_char,
    mut input_size: size_t,
) -> *const ::core::ffi::c_char {
    if input.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if input_size > 0 as size_t {
        end = input
            .offset(strnlen(input, input_size) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
    } else {
        end = input
            .offset(strlen(input) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
    }
    while end > input
        && *(*__ctype_b_loc()).offset(*end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISgraph.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
    {
        end = end.offset(-1);
    }
    return end.offset(1 as ::core::ffi::c_int as isize);
}
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL_0 as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL_0 as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_strdup(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if str.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut ptr: *mut ::core::ffi::c_char = strdup(str);
    if ptr.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"String duplication failed for length %zu\0".as_ptr() as *const ::core::ffi::c_char,
            strlen(str),
        );
    }
    return ptr;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_strcpy(
    mut dest: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut dst_size: size_t,
    mut src_size: size_t,
) -> ::core::ffi::c_int {
    if dest.is_null() || src.is_null() || dst_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut src_len: size_t = 0;
    if src_size > 0 as size_t {
        src_len = strnlen(src, src_size);
    } else {
        src_len = strlen(src);
    }
    if src_len >= dst_size {
        src_len = dst_size.wrapping_sub(1 as size_t);
    }
    memcpy(
        dest as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        src_len,
    );
    *dest.offset(src_len as isize) = '\0' as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_strcat(
    mut dest: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_int {
    if dest.is_null() || src.is_null() || size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut dest_len: size_t = strnlen(dest, size);
    if dest_len >= size {
        return -1 as ::core::ffi::c_int;
    }
    return safe_strcpy(
        dest.offset(dest_len as isize),
        src,
        size.wrapping_sub(dest_len),
        size.wrapping_sub(dest_len),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ends_with(
    mut str: *const ::core::ffi::c_char,
    mut suffix: *const ::core::ffi::c_char,
) -> bool {
    if str.is_null() || suffix.is_null() {
        return r#false != 0;
    }
    let mut str_len: size_t = strlen(str);
    let mut suffix_len: size_t = strlen(suffix);
    if suffix_len > str_len {
        return r#false != 0;
    }
    return strcmp(
        str.offset(str_len as isize).offset(-(suffix_len as isize)),
        suffix,
    ) == 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn string_has_non_whitespace(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null() {
        return r#false != 0;
    }
    while *value != 0 {
        if *(*__ctype_b_loc()).offset(*value as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        {
            return r#true != 0;
        }
        value = value.offset(1);
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trim_ascii_whitespace(
    mut value: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if value.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut end: *mut ::core::ffi::c_char =
        rtrim_pos(value, 0 as size_t) as *mut ::core::ffi::c_char;
    *end = '\0' as ::core::ffi::c_char;
    return ltrim_pos(value) as *mut ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn copy_trimmed_value(
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
    mut input: *const ::core::ffi::c_char,
    mut input_size: size_t,
) -> size_t {
    if input.is_null() || output.is_null() || output_size == 0 as size_t {
        return r#false as size_t;
    }
    let mut start: *const ::core::ffi::c_char = ltrim_pos(input);
    let mut end: *const ::core::ffi::c_char = rtrim_pos(input, input_size);
    let mut len: size_t = end.offset_from(start) as size_t;
    if len >= output_size {
        len = output_size.wrapping_sub(1 as size_t);
    }
    memcpy(
        output as *mut ::core::ffi::c_void,
        start as *const ::core::ffi::c_void,
        len,
    );
    *output.offset(len as isize) = '\0' as ::core::ffi::c_char;
    return len;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
