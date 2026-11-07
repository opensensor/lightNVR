unsafe extern "C" {
    pub type Curl_URL;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn curl_free(p: *mut ::core::ffi::c_void);
    unsafe fn curl_url() -> *mut CURLU;
    unsafe fn curl_url_cleanup(u: *mut CURLU);
    unsafe fn curl_url_get(
        u: *const CURLU,
        what: CURLUPart,
        part: *mut *mut ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
    ) -> CURLUcode;
    unsafe fn curl_url_set(
        u: *mut CURLU,
        what: CURLUPart,
        part: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
    ) -> CURLUcode;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strndup(
        __string: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strpbrk(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type CURLU = Curl_URL;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CURLUcode(pub ::core::ffi::c_uint);
impl CURLUcode {
    pub const CURLUE_OK: Self = Self(0);
    pub const CURLUE_BAD_HANDLE: Self = Self(1);
    pub const CURLUE_BAD_PARTPOINTER: Self = Self(2);
    pub const CURLUE_MALFORMED_INPUT: Self = Self(3);
    pub const CURLUE_BAD_PORT_NUMBER: Self = Self(4);
    pub const CURLUE_UNSUPPORTED_SCHEME: Self = Self(5);
    pub const CURLUE_URLDECODE: Self = Self(6);
    pub const CURLUE_OUT_OF_MEMORY: Self = Self(7);
    pub const CURLUE_USER_NOT_ALLOWED: Self = Self(8);
    pub const CURLUE_UNKNOWN_PART: Self = Self(9);
    pub const CURLUE_NO_SCHEME: Self = Self(10);
    pub const CURLUE_NO_USER: Self = Self(11);
    pub const CURLUE_NO_PASSWORD: Self = Self(12);
    pub const CURLUE_NO_OPTIONS: Self = Self(13);
    pub const CURLUE_NO_HOST: Self = Self(14);
    pub const CURLUE_NO_PORT: Self = Self(15);
    pub const CURLUE_NO_QUERY: Self = Self(16);
    pub const CURLUE_NO_FRAGMENT: Self = Self(17);
    pub const CURLUE_NO_ZONEID: Self = Self(18);
    pub const CURLUE_BAD_FILE_URL: Self = Self(19);
    pub const CURLUE_BAD_FRAGMENT: Self = Self(20);
    pub const CURLUE_BAD_HOSTNAME: Self = Self(21);
    pub const CURLUE_BAD_IPV6: Self = Self(22);
    pub const CURLUE_BAD_LOGIN: Self = Self(23);
    pub const CURLUE_BAD_PASSWORD: Self = Self(24);
    pub const CURLUE_BAD_PATH: Self = Self(25);
    pub const CURLUE_BAD_QUERY: Self = Self(26);
    pub const CURLUE_BAD_SCHEME: Self = Self(27);
    pub const CURLUE_BAD_SLASHES: Self = Self(28);
    pub const CURLUE_BAD_USER: Self = Self(29);
    pub const CURLUE_LACKS_IDN: Self = Self(30);
    pub const CURLUE_TOO_LARGE: Self = Self(31);
    pub const CURLUE_BACKSLASH: Self = Self(32);
    pub const CURLUE_LAST: Self = Self(33);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CURLUPart(pub ::core::ffi::c_uint);
impl CURLUPart {
    pub const CURLUPART_URL: Self = Self(0);
    pub const CURLUPART_SCHEME: Self = Self(1);
    pub const CURLUPART_USER: Self = Self(2);
    pub const CURLUPART_PASSWORD: Self = Self(3);
    pub const CURLUPART_OPTIONS: Self = Self(4);
    pub const CURLUPART_HOST: Self = Self(5);
    pub const CURLUPART_PORT: Self = Self(6);
    pub const CURLUPART_PATH: Self = Self(7);
    pub const CURLUPART_QUERY: Self = Self(8);
    pub const CURLUPART_FRAGMENT: Self = Self(9);
    pub const CURLUPART_ZONEID: Self = Self(10);
}
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
pub const PUBLISH_URL_MAX_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const CURLU_NON_SUPPORT_SCHEME: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const CURLU_DISALLOW_USER: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const CURLU_URLDECODE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const CURLU_URLENCODE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
unsafe extern "C" fn url_has_controls(mut url: *const ::core::ffi::c_char) -> bool {
    if url.is_null() {
        return r#true != 0;
    }
    let mut p: *const ::core::ffi::c_uchar = url as *const ::core::ffi::c_uchar;
    while *p != 0 {
        if *p as ::core::ffi::c_int <= 0x20 as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == 0x7f as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        p = p.offset(1);
    }
    return r#false != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn url_is_http_endpoint(mut url: *const ::core::ffi::c_char) -> bool {
    if url_has_controls(url) {
        return r#false != 0;
    }
    let mut parsed: *mut CURLU = curl_url();
    if parsed.is_null() {
        return r#false != 0;
    }
    let mut scheme: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut host: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut valid: bool = curl_url_set(
        parsed,
        CURLUPart::CURLUPART_URL,
        url,
        CURLU_DISALLOW_USER as ::core::ffi::c_uint,
    )
    .0 == CURLUcode::CURLUE_OK.0
        && curl_url_get(
            parsed,
            CURLUPart::CURLUPART_SCHEME,
            &raw mut scheme,
            0 as ::core::ffi::c_uint,
        )
        .0 == CURLUcode::CURLUE_OK.0
        && (strcasecmp(scheme, b"http\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcasecmp(scheme, b"https\0".as_ptr() as *const ::core::ffi::c_char) == 0)
        && curl_url_get(
            parsed,
            CURLUPart::CURLUPART_HOST,
            &raw mut host,
            0 as ::core::ffi::c_uint,
        )
        .0 == CURLUcode::CURLUE_OK.0
        && *host.offset(0isize) as ::core::ffi::c_int != 0;
    curl_free(scheme as *mut ::core::ffi::c_void);
    curl_free(host as *mut ::core::ffi::c_void);
    curl_url_cleanup(parsed);
    return valid;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn url_rtsp_endpoint(
    mut url: *const ::core::ffi::c_char,
    mut host: *mut ::core::ffi::c_char,
    mut host_size: size_t,
    mut port: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: CURLUcode = CURLUcode::CURLUE_OK;
    let mut number: ::core::ffi::c_long = 0;
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut length: size_t = 0;
    if host.is_null() || host_size == 0 || port.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *host.offset(0isize) = '\0' as ::core::ffi::c_char;
    *port = 0 as ::core::ffi::c_int;
    if url_has_controls(url) {
        return -1 as ::core::ffi::c_int;
    }
    let mut parsed: *mut CURLU = curl_url();
    if parsed.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut scheme: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut hostname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut port_text: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if !(curl_url_set(
        parsed,
        CURLUPart::CURLUPART_URL,
        url,
        CURLU_NON_SUPPORT_SCHEME as ::core::ffi::c_uint,
    )
    .0 != CURLUcode::CURLUE_OK.0
        || curl_url_get(
            parsed,
            CURLUPart::CURLUPART_SCHEME,
            &raw mut scheme,
            0 as ::core::ffi::c_uint,
        )
        .0 != CURLUcode::CURLUE_OK.0
        || strcasecmp(scheme, b"rtsp\0".as_ptr() as *const ::core::ffi::c_char) != 0
        || curl_url_get(
            parsed,
            CURLUPart::CURLUPART_HOST,
            &raw mut hostname,
            0 as ::core::ffi::c_uint,
        )
        .0 != CURLUcode::CURLUE_OK.0)
    {
        rc = curl_url_get(
            parsed,
            CURLUPart::CURLUPART_PORT,
            &raw mut port_text,
            0 as ::core::ffi::c_uint,
        );
        if !(rc.0 != CURLUcode::CURLUE_OK.0 && rc.0 != CURLUcode::CURLUE_NO_PORT.0) {
            number = if !port_text.is_null() {
                strtol(
                    port_text,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    10 as ::core::ffi::c_int,
                )
            } else {
                554 as ::core::ffi::c_long
            };
            if !(number < 1 as ::core::ffi::c_long || number > 65535 as ::core::ffi::c_long) {
                start = hostname;
                length = strlen(hostname);
                if length > 2 as size_t
                    && *hostname.offset(0isize) as ::core::ffi::c_int == '[' as ::core::ffi::c_int
                    && *hostname.offset(length.wrapping_sub(1 as size_t) as isize)
                        as ::core::ffi::c_int
                        == ']' as ::core::ffi::c_int
                {
                    start = start.offset(1);
                    length = length.wrapping_sub(2 as size_t);
                }
                if !(length == 0 || length >= host_size) {
                    memcpy(
                        host as *mut ::core::ffi::c_void,
                        start as *const ::core::ffi::c_void,
                        length,
                    );
                    *host.offset(length as isize) = '\0' as ::core::ffi::c_char;
                    *port = number as ::core::ffi::c_int;
                    result = 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    curl_free(scheme as *mut ::core::ffi::c_void);
    curl_free(hostname as *mut ::core::ffi::c_void);
    curl_free(port_text as *mut ::core::ffi::c_void);
    curl_url_cleanup(parsed);
    return result;
}
unsafe extern "C" fn copy_string(
    mut src: *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_char,
    mut dst_size: size_t,
) -> ::core::ffi::c_int {
    if src.is_null() || dst.is_null() || dst_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut written: ::core::ffi::c_int = snprintf(
        dst,
        dst_size,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        src,
    );
    return if written < 0 as ::core::ffi::c_int || written as size_t >= dst_size {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn append_suffix(
    mut base: *const ::core::ffi::c_char,
    mut suffix: *const ::core::ffi::c_char,
    mut out_url: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> ::core::ffi::c_int {
    if base.is_null() || out_url.is_null() || out_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut written: ::core::ffi::c_int = snprintf(
        out_url,
        out_size,
        b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        base,
        if !suffix.is_null() {
            suffix
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return if written < 0 as ::core::ffi::c_int || written as size_t >= out_size {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn split_url_base(
    mut url: *const ::core::ffi::c_char,
    mut base_url: *mut *mut ::core::ffi::c_char,
    mut suffix: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if url.is_null() || base_url.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut fragment: *const ::core::ffi::c_char = strchr(url, '#' as ::core::ffi::c_int);
    let mut base_len: size_t = if !fragment.is_null() {
        fragment.offset_from(url) as size_t
    } else {
        strlen(url)
    };
    let mut base: *mut ::core::ffi::c_char = strndup(url, base_len);
    if base.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *base_url = base;
    if !suffix.is_null() {
        *suffix = fragment;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn find_userinfo_bounds(
    mut url: *const ::core::ffi::c_char,
    mut userinfo_start: *mut *const ::core::ffi::c_char,
    mut userinfo_end: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut scheme_end: *const ::core::ffi::c_char =
        strstr(url, b"://\0".as_ptr() as *const ::core::ffi::c_char);
    if scheme_end.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut authority_start: *const ::core::ffi::c_char =
        scheme_end.offset(3 as ::core::ffi::c_int as isize);
    let mut authority_end: *const ::core::ffi::c_char = strpbrk(
        authority_start,
        b"/?#\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut at: *const ::core::ffi::c_char = strchr(authority_start, '@' as ::core::ffi::c_int);
    if at.is_null() || !authority_end.is_null() && at >= authority_end {
        return -1 as ::core::ffi::c_int;
    }
    if !userinfo_start.is_null() {
        *userinfo_start = authority_start;
    }
    if !userinfo_end.is_null() {
        *userinfo_end = at;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn manual_strip_credentials(
    mut url: *const ::core::ffi::c_char,
    mut out_url: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> ::core::ffi::c_int {
    let mut userinfo_start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut userinfo_end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if find_userinfo_bounds(url, &raw mut userinfo_start, &raw mut userinfo_end)
        != 0 as ::core::ffi::c_int
    {
        return copy_string(url, out_url, out_size);
    }
    let mut prefix_len: size_t = userinfo_start.offset_from(url) as size_t;
    let mut written: ::core::ffi::c_int = snprintf(
        out_url,
        out_size,
        b"%.*s%s\0".as_ptr() as *const ::core::ffi::c_char,
        prefix_len as ::core::ffi::c_int,
        url,
        userinfo_end.offset(1 as ::core::ffi::c_int as isize),
    );
    return if written < 0 as ::core::ffi::c_int || written as size_t >= out_size {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn manual_extract_credentials(
    mut url: *const ::core::ffi::c_char,
    mut username: *mut ::core::ffi::c_char,
    mut username_size: size_t,
    mut password: *mut ::core::ffi::c_char,
    mut password_size: size_t,
) -> ::core::ffi::c_int {
    let mut userinfo_start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut userinfo_end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut separator: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut username_len: size_t = 0;
    let mut password_len: size_t = 0;
    if username.is_null()
        || password.is_null()
        || username_size == 0 as size_t
        || password_size == 0 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    *username.offset(0isize) = '\0' as ::core::ffi::c_char;
    *password.offset(0isize) = '\0' as ::core::ffi::c_char;
    if find_userinfo_bounds(url, &raw mut userinfo_start, &raw mut userinfo_end)
        != 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    separator = memchr(
        userinfo_start as *const ::core::ffi::c_void,
        ':' as ::core::ffi::c_int,
        userinfo_end.offset_from(userinfo_start) as size_t,
    ) as *const ::core::ffi::c_char;
    username_len = if !separator.is_null() {
        separator.offset_from(userinfo_start) as size_t
    } else {
        userinfo_end.offset_from(userinfo_start) as size_t
    };
    password_len = if !separator.is_null() {
        (userinfo_end.offset_from(separator) - 1isize) as size_t
    } else {
        0 as size_t
    };
    if username_len >= username_size || password_len >= password_size {
        return -1 as ::core::ffi::c_int;
    }
    memcpy(
        username as *mut ::core::ffi::c_void,
        userinfo_start as *const ::core::ffi::c_void,
        username_len,
    );
    *username.offset(username_len as isize) = '\0' as ::core::ffi::c_char;
    if !separator.is_null() && password_len > 0 as size_t {
        memcpy(
            password as *mut ::core::ffi::c_void,
            separator.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            password_len,
        );
        *password.offset(password_len as isize) = '\0' as ::core::ffi::c_char;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn manual_apply_credentials(
    mut url: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut out_url: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> ::core::ffi::c_int {
    let mut scheme_end: *const ::core::ffi::c_char =
        strstr(url, b"://\0".as_ptr() as *const ::core::ffi::c_char);
    let mut host_start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut stripped_url: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut written: ::core::ffi::c_int = 0;
    if scheme_end.is_null() {
        return copy_string(url, out_url, out_size);
    }
    stripped_url = malloc(strlen(url).wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    if stripped_url.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if manual_strip_credentials(url, stripped_url, strlen(url).wrapping_add(1 as size_t))
        != 0 as ::core::ffi::c_int
    {
        free(stripped_url as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    scheme_end = strstr(
        stripped_url,
        b"://\0".as_ptr() as *const ::core::ffi::c_char,
    );
    host_start = if !scheme_end.is_null() {
        scheme_end.offset(3 as ::core::ffi::c_int as isize)
    } else {
        stripped_url
    };
    written = snprintf(
        out_url,
        out_size,
        b"%.*s://%s%s%s@%s\0".as_ptr() as *const ::core::ffi::c_char,
        scheme_end.offset_from(stripped_url) as ::core::ffi::c_int,
        stripped_url,
        username,
        if !password.is_null()
            && *password.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            b":\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !password.is_null()
            && *password.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            password
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        host_start,
    );
    free(stripped_url as *mut ::core::ffi::c_void);
    return if written < 0 as ::core::ffi::c_int || written as size_t >= out_size {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn parse_url_handle(
    mut url: *const ::core::ffi::c_char,
    mut base_url: *mut *mut ::core::ffi::c_char,
    mut suffix: *mut *const ::core::ffi::c_char,
) -> *mut CURLU {
    let mut handle: *mut CURLU = ::core::ptr::null_mut::<CURLU>();
    if split_url_base(url, base_url, suffix) != 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<CURLU>();
    }
    handle = curl_url();
    if handle.is_null() {
        free(*base_url as *mut ::core::ffi::c_void);
        *base_url = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return ::core::ptr::null_mut::<CURLU>();
    }
    if curl_url_set(
        handle,
        CURLUPart::CURLUPART_URL,
        *base_url,
        CURLU_NON_SUPPORT_SCHEME as ::core::ffi::c_uint,
    )
    .0 != CURLUcode::CURLUE_OK.0
    {
        curl_url_cleanup(handle);
        free(*base_url as *mut ::core::ffi::c_void);
        *base_url = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return ::core::ptr::null_mut::<CURLU>();
    }
    return handle;
}
unsafe extern "C" fn rebuild_url(
    mut handle: *mut CURLU,
    mut suffix: *const ::core::ffi::c_char,
    mut out_url: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> ::core::ffi::c_int {
    let mut rebuilt: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if curl_url_get(
        handle,
        CURLUPart::CURLUPART_URL,
        &raw mut rebuilt,
        0 as ::core::ffi::c_uint,
    )
    .0 != CURLUcode::CURLUE_OK.0
        || rebuilt.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    result = append_suffix(rebuilt, suffix, out_url, out_size);
    curl_free(rebuilt as *mut ::core::ffi::c_void);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn url_apply_credentials(
    mut url: *const ::core::ffi::c_char,
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
    mut out_url: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> ::core::ffi::c_int {
    let mut base_url: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut suffix: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut handle: *mut CURLU = ::core::ptr::null_mut::<CURLU>();
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if url.is_null() || out_url.is_null() || out_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    if username.is_null()
        || *username.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return copy_string(url, out_url, out_size);
    }
    handle = parse_url_handle(url, &raw mut base_url, &raw mut suffix);
    if handle.is_null() {
        return manual_apply_credentials(url, username, password, out_url, out_size);
    }
    '_cleanup: {
        if curl_url_set(
            handle,
            CURLUPart::CURLUPART_USER,
            username,
            CURLU_URLENCODE as ::core::ffi::c_uint,
        )
        .0 == CURLUcode::CURLUE_OK.0
        {
            if !password.is_null()
                && *password.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            {
                if curl_url_set(
                    handle,
                    CURLUPart::CURLUPART_PASSWORD,
                    password,
                    CURLU_URLENCODE as ::core::ffi::c_uint,
                )
                .0 != CURLUcode::CURLUE_OK.0
                {
                    break '_cleanup;
                }
            } else if curl_url_set(
                handle,
                CURLUPart::CURLUPART_PASSWORD,
                ::core::ptr::null::<::core::ffi::c_char>(),
                0 as ::core::ffi::c_uint,
            )
            .0 != CURLUcode::CURLUE_OK.0
            {
                break '_cleanup;
            }
            result = rebuild_url(handle, suffix, out_url, out_size);
        }
    }
    curl_url_cleanup(handle);
    free(base_url as *mut ::core::ffi::c_void);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn url_strip_credentials(
    mut url: *const ::core::ffi::c_char,
    mut out_url: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> ::core::ffi::c_int {
    let mut base_url: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut suffix: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut handle: *mut CURLU = ::core::ptr::null_mut::<CURLU>();
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if url.is_null() || out_url.is_null() || out_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    handle = parse_url_handle(url, &raw mut base_url, &raw mut suffix);
    if handle.is_null() {
        return manual_strip_credentials(url, out_url, out_size);
    }
    if !(curl_url_set(
        handle,
        CURLUPart::CURLUPART_USER,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_uint,
    )
    .0 != CURLUcode::CURLUE_OK.0
        || curl_url_set(
            handle,
            CURLUPart::CURLUPART_PASSWORD,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_uint,
        )
        .0 != CURLUcode::CURLUE_OK.0)
    {
        result = rebuild_url(handle, suffix, out_url, out_size);
    }
    curl_url_cleanup(handle);
    free(base_url as *mut ::core::ffi::c_void);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn url_extract_credentials(
    mut url: *const ::core::ffi::c_char,
    mut username: *mut ::core::ffi::c_char,
    mut username_size: size_t,
    mut password: *mut ::core::ffi::c_char,
    mut password_size: size_t,
) -> ::core::ffi::c_int {
    let mut base_url: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut handle: *mut CURLU = ::core::ptr::null_mut::<CURLU>();
    let mut curl_user: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut curl_password: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if url.is_null()
        || username.is_null()
        || password.is_null()
        || username_size == 0 as size_t
        || password_size == 0 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    *username.offset(0isize) = '\0' as ::core::ffi::c_char;
    *password.offset(0isize) = '\0' as ::core::ffi::c_char;
    handle = parse_url_handle(
        url,
        &raw mut base_url,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if handle.is_null() {
        return manual_extract_credentials(url, username, username_size, password, password_size);
    }
    '_cleanup: {
        if curl_url_get(
            handle,
            CURLUPart::CURLUPART_USER,
            &raw mut curl_user,
            CURLU_URLDECODE as ::core::ffi::c_uint,
        )
        .0 == CURLUcode::CURLUE_OK.0
            && !curl_user.is_null()
        {
            if copy_string(curl_user, username, username_size) != 0 as ::core::ffi::c_int {
                result = -1 as ::core::ffi::c_int;
                break '_cleanup;
            }
        }
        if curl_url_get(
            handle,
            CURLUPart::CURLUPART_PASSWORD,
            &raw mut curl_password,
            CURLU_URLDECODE as ::core::ffi::c_uint,
        )
        .0 == CURLUcode::CURLUE_OK.0
            && !curl_password.is_null()
        {
            if copy_string(curl_password, password, password_size) != 0 as ::core::ffi::c_int {
                result = -1 as ::core::ffi::c_int;
            }
        }
    }
    if !curl_user.is_null() {
        curl_free(curl_user as *mut ::core::ffi::c_void);
    }
    if !curl_password.is_null() {
        curl_free(curl_password as *mut ::core::ffi::c_void);
    }
    curl_url_cleanup(handle);
    free(base_url as *mut ::core::ffi::c_void);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn url_build_onvif_service_url(
    mut url: *const ::core::ffi::c_char,
    mut onvif_port: ::core::ffi::c_int,
    mut service_path: *const ::core::ffi::c_char,
    mut out_url: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> ::core::ffi::c_int {
    let mut base_url: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut handle: *mut CURLU = ::core::ptr::null_mut::<CURLU>();
    let mut scheme: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut port_str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut port_buffer: [::core::ffi::c_char; 16] = [0; 16];
    let mut result: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if url.is_null() || out_url.is_null() || out_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    handle = parse_url_handle(
        url,
        &raw mut base_url,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if handle.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    '_cleanup: {
        if !(curl_url_get(
            handle,
            CURLUPart::CURLUPART_SCHEME,
            &raw mut scheme,
            0 as ::core::ffi::c_uint,
        )
        .0 != CURLUcode::CURLUE_OK.0
            || scheme.is_null())
        {
            let mut target_scheme: *const ::core::ffi::c_char =
                if strcasecmp(scheme, b"rtsps\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcasecmp(scheme, b"https\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                {
                    b"https\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"http\0".as_ptr() as *const ::core::ffi::c_char
                };
            if curl_url_set(
                handle,
                CURLUPart::CURLUPART_SCHEME,
                target_scheme,
                0 as ::core::ffi::c_uint,
            )
            .0 == CURLUcode::CURLUE_OK.0
            {
                if onvif_port > 0 as ::core::ffi::c_int {
                    snprintf(
                        &raw mut port_buffer as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                        onvif_port,
                    );
                    if curl_url_set(
                        handle,
                        CURLUPart::CURLUPART_PORT,
                        &raw mut port_buffer as *mut ::core::ffi::c_char,
                        0 as ::core::ffi::c_uint,
                    )
                    .0 != CURLUcode::CURLUE_OK.0
                    {
                        break '_cleanup;
                    }
                } else if curl_url_get(
                    handle,
                    CURLUPart::CURLUPART_PORT,
                    &raw mut port_str,
                    0 as ::core::ffi::c_uint,
                )
                .0 == CURLUcode::CURLUE_OK.0
                    && !port_str.is_null()
                {
                    let mut p: ::core::ffi::c_long = strtol(
                        port_str,
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                        10 as ::core::ffi::c_int,
                    );
                    if p == 554 as ::core::ffi::c_long {
                        if curl_url_set(
                            handle,
                            CURLUPart::CURLUPART_PORT,
                            b"80\0".as_ptr() as *const ::core::ffi::c_char,
                            0 as ::core::ffi::c_uint,
                        )
                        .0 != CURLUcode::CURLUE_OK.0
                        {
                            break '_cleanup;
                        }
                    } else if p == 322 as ::core::ffi::c_long {
                        if curl_url_set(
                            handle,
                            CURLUPart::CURLUPART_PORT,
                            b"443\0".as_ptr() as *const ::core::ffi::c_char,
                            0 as ::core::ffi::c_uint,
                        )
                        .0 != CURLUcode::CURLUE_OK.0
                        {
                            break '_cleanup;
                        }
                    }
                }
                if !(curl_url_set(
                    handle,
                    CURLUPart::CURLUPART_USER,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    0 as ::core::ffi::c_uint,
                )
                .0 != CURLUcode::CURLUE_OK.0
                    || curl_url_set(
                        handle,
                        CURLUPart::CURLUPART_PASSWORD,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        0 as ::core::ffi::c_uint,
                    )
                    .0 != CURLUcode::CURLUE_OK.0)
                {
                    let mut path: *const ::core::ffi::c_char =
                        b"/\0".as_ptr() as *const ::core::ffi::c_char;
                    if !service_path.is_null()
                        && *service_path.offset(0isize) as ::core::ffi::c_int
                            != '\0' as ::core::ffi::c_int
                    {
                        if *service_path.offset(0isize) as ::core::ffi::c_int
                            != '/' as ::core::ffi::c_int
                        {
                            break '_cleanup;
                        } else {
                            path = service_path;
                        }
                    }
                    if curl_url_set(
                        handle,
                        CURLUPart::CURLUPART_PATH,
                        path,
                        0 as ::core::ffi::c_uint,
                    )
                    .0 == CURLUcode::CURLUE_OK.0
                    {
                        if !(curl_url_set(
                            handle,
                            CURLUPart::CURLUPART_QUERY,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            0 as ::core::ffi::c_uint,
                        )
                        .0 != CURLUcode::CURLUE_OK.0
                            || curl_url_set(
                                handle,
                                CURLUPart::CURLUPART_FRAGMENT,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                0 as ::core::ffi::c_uint,
                            )
                            .0 != CURLUcode::CURLUE_OK.0)
                        {
                            result = rebuild_url(
                                handle,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                out_url,
                                out_size,
                            );
                            if result == 0 as ::core::ffi::c_int
                                && (service_path.is_null()
                                    || *service_path.offset(0isize) as ::core::ffi::c_int
                                        != '/' as ::core::ffi::c_int)
                            {
                                let mut len: size_t = strlen(out_url);
                                while len > 0 as size_t
                                    && *out_url.offset(len.wrapping_sub(1 as size_t) as isize)
                                        as ::core::ffi::c_int
                                        == '/' as ::core::ffi::c_int
                                {
                                    len = len.wrapping_sub(1);
                                    *out_url.offset(len as isize) = '\0' as ::core::ffi::c_char;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !scheme.is_null() {
        curl_free(scheme as *mut ::core::ffi::c_void);
    }
    if !port_str.is_null() {
        curl_free(port_str as *mut ::core::ffi::c_void);
    }
    curl_url_cleanup(handle);
    free(base_url as *mut ::core::ffi::c_void);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn url_build_onvif_device_service_url(
    mut url: *const ::core::ffi::c_char,
    mut onvif_port: ::core::ffi::c_int,
    mut out_url: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> ::core::ffi::c_int {
    if url.is_null() || out_url.is_null() || out_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    if onvif_port <= 0 as ::core::ffi::c_int {
        return url_strip_credentials(url, out_url, out_size);
    }
    return url_build_onvif_service_url(
        url,
        onvif_port,
        b"/onvif/device_service\0".as_ptr() as *const ::core::ffi::c_char,
        out_url,
        out_size,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn url_redact_for_logging(
    mut url: *const ::core::ffi::c_char,
    mut out_url: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> ::core::ffi::c_int {
    if url.is_null() || out_url.is_null() || out_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    if !url_has_controls(url)
        && url_strip_credentials(url, out_url, out_size) == 0 as ::core::ffi::c_int
    {
        let mut private_suffix: *mut ::core::ffi::c_char =
            strpbrk(out_url, b"?#\0".as_ptr() as *const ::core::ffi::c_char);
        if !private_suffix.is_null() {
            *private_suffix = '\0' as ::core::ffi::c_char;
        }
        return 0 as ::core::ffi::c_int;
    }
    return copy_string(
        b"[invalid-url]\0".as_ptr() as *const ::core::ffi::c_char,
        out_url,
        out_size,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_url_escape(
    mut input: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if input.is_null() || output.is_null() || output_size == 0 as size_t {
        return;
    }
    *output.offset(0isize) = '\0' as ::core::ffi::c_char;
    p = input;
    q = output;
    while *p != 0 {
        let mut used: size_t = q.offset_from(output) as size_t;
        let mut remaining: size_t = if used < output_size {
            output_size.wrapping_sub(used)
        } else {
            0 as size_t
        };
        let mut c: ::core::ffi::c_uchar = 0;
        if remaining <= 1 as size_t {
            break;
        }
        c = *p as ::core::ffi::c_uchar;
        if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
            || c as ::core::ffi::c_int == '-' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '_' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '~' as ::core::ffi::c_int
        {
            let c2rust_fresh0 = q;
            q = q.offset(1);
            *c2rust_fresh0 = c as ::core::ffi::c_char;
        } else {
            if remaining <= 3 as size_t {
                break;
            }
            snprintf(
                q,
                remaining,
                b"%%%02X\0".as_ptr() as *const ::core::ffi::c_char,
                c as ::core::ffi::c_int,
            );
            q = q.offset(3 as ::core::ffi::c_int as isize);
        }
        p = p.offset(1);
    }
    if (q.offset_from(output) as size_t) < output_size {
        *q = '\0' as ::core::ffi::c_char;
    } else {
        *output.offset(output_size.wrapping_sub(1 as size_t) as isize) =
            '\0' as ::core::ffi::c_char;
    };
}
unsafe extern "C" fn is_publish_url_separator(mut c: ::core::ffi::c_char) -> bool {
    return c as ::core::ffi::c_int == ',' as ::core::ffi::c_int
        || *(*__ctype_b_loc()).offset(c as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn publish_url_next(
    mut cursor: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> *const ::core::ffi::c_char {
    if cursor.is_null() || out.is_null() || out_size == 0 as size_t {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    while *cursor as ::core::ffi::c_int != 0
        && is_publish_url_separator(*cursor) as ::core::ffi::c_int != 0
    {
        cursor = cursor.offset(1);
    }
    if *cursor as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        *out.offset(0isize) = '\0' as ::core::ffi::c_char;
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let mut len: size_t = 0 as size_t;
    while *cursor.offset(len as isize) as ::core::ffi::c_int != 0
        && !is_publish_url_separator(*cursor.offset(len as isize))
    {
        len = len.wrapping_add(1);
    }
    let mut copy: size_t = if len < out_size.wrapping_sub(1 as size_t) {
        len
    } else {
        out_size.wrapping_sub(1 as size_t)
    };
    memcpy(
        out as *mut ::core::ffi::c_void,
        cursor as *const ::core::ffi::c_void,
        copy,
    );
    *out.offset(copy as isize) = '\0' as ::core::ffi::c_char;
    return cursor.offset(len as isize);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn publish_url_list_is_valid(mut list: *const ::core::ffi::c_char) -> bool {
    if list.is_null() {
        return r#true != 0;
    }
    let mut cursor: *const ::core::ffi::c_char = list;
    while *cursor != 0 {
        while *cursor as ::core::ffi::c_int != 0
            && is_publish_url_separator(*cursor) as ::core::ffi::c_int != 0
        {
            cursor = cursor.offset(1);
        }
        if *cursor as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            break;
        }
        let mut start: *const ::core::ffi::c_char = cursor;
        while *cursor as ::core::ffi::c_int != 0 && !is_publish_url_separator(*cursor) {
            let mut c: ::core::ffi::c_uchar = *cursor as ::core::ffi::c_uchar;
            if (c as ::core::ffi::c_int) < 0x21 as ::core::ffi::c_int
                || c as ::core::ffi::c_int > 0x7e as ::core::ffi::c_int
                || c as ::core::ffi::c_int == '"' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == '\'' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
            {
                return r#false != 0;
            }
            cursor = cursor.offset(1);
        }
        let mut len: size_t = cursor.offset_from(start) as size_t;
        if len >= PUBLISH_URL_MAX_LENGTH as size_t {
            return r#false != 0;
        }
        if !(len > 7 as size_t
            && strncasecmp(
                start,
                b"rtmp://\0".as_ptr() as *const ::core::ffi::c_char,
                7 as size_t,
            ) == 0 as ::core::ffi::c_int)
            && !(len > 8 as size_t
                && strncasecmp(
                    start,
                    b"rtmps://\0".as_ptr() as *const ::core::ffi::c_char,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int)
        {
            return r#false != 0;
        }
    }
    return r#true != 0;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
