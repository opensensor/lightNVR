use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn ezxml_parse_str(s: *mut ::core::ffi::c_char, len: size_t) -> ezxml_t;
    unsafe fn ezxml_child(xml: ezxml_t, name: *const ::core::ffi::c_char) -> ezxml_t;
    unsafe fn ezxml_free(xml: ezxml_t);
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    unsafe fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn getrandom(
        __buffer: *mut ::core::ffi::c_void,
        __length: size_t,
        __flags: ::core::ffi::c_uint,
    ) -> ssize_t;
    unsafe fn mbedtls_sha1(
        input: *const ::core::ffi::c_uchar,
        ilen: size_t,
        output: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_base64_encode(
        dst: *mut ::core::ffi::c_uchar,
        dlen: size_t,
        olen: *mut size_t,
        src: *const ::core::ffi::c_uchar,
        slen: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type ssize_t = isize;
pub type time_t = __time_t;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
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
pub type ezxml_t = *mut ezxml;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ezxml {
    pub name: *mut ::core::ffi::c_char,
    pub attr: *mut *mut ::core::ffi::c_char,
    pub txt: *mut ::core::ffi::c_char,
    pub off: size_t,
    pub next: ezxml_t,
    pub sibling: ezxml_t,
    pub ordered: ezxml_t,
    pub child: ezxml_t,
    pub parent: ezxml_t,
    pub flags: ::core::ffi::c_short,
}
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn onvif_create_security_header(
    mut username: *const ::core::ffi::c_char,
    mut password: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if username.is_null()
        || password.is_null()
        || *username.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || *password.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"onvif_create_security_header: username and password must be non-empty\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let nonce_len: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    let mut nonce_bytes: [::core::ffi::c_uchar; 16] = [0; 16];
    if getrandom(
        &raw mut nonce_bytes as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        nonce_len as size_t,
        0 as ::core::ffi::c_uint,
    ) < 0 as ssize_t
    {
        let mut urandom: *mut FILE = fopen(
            b"/dev/urandom\0".as_ptr() as *const ::core::ffi::c_char,
            b"rb\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if urandom.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"onvif_create_security_header: cannot obtain random bytes\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        fread(
            &raw mut nonce_bytes as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            1 as size_t,
            nonce_len as size_t,
            urandom,
        );
        fclose(urandom);
    }
    let mut base64_nonce_buf_len: size_t = (4 as ::core::ffi::c_int * nonce_len
        / 3 as ::core::ffi::c_int
        + 5 as ::core::ffi::c_int) as size_t;
    let mut base64_nonce: *mut ::core::ffi::c_char =
        malloc(base64_nonce_buf_len) as *mut ::core::ffi::c_char;
    if base64_nonce.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut base64_nonce_len: size_t = 0 as size_t;
    mbedtls_base64_encode(
        base64_nonce as *mut ::core::ffi::c_uchar,
        base64_nonce_buf_len,
        &raw mut base64_nonce_len,
        &raw mut nonce_bytes as *mut ::core::ffi::c_uchar,
        nonce_len as size_t,
    );
    *base64_nonce.offset(base64_nonce_len as isize) = '\0' as ::core::ffi::c_char;
    let mut created: [::core::ffi::c_char; 30] = [0; 30];
    let mut now: time_t = 0;
    let mut tm_now_buf: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut tm_now: *const tm = ::core::ptr::null::<tm>();
    time(&raw mut now);
    tm_now = gmtime_r(&raw mut now, &raw mut tm_now_buf);
    strftime(
        &raw mut created as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 30]>(),
        b"%Y-%m-%dT%H:%M:%S.000Z\0".as_ptr() as *const ::core::ffi::c_char,
        tm_now,
    );
    let mut created_len: size_t = strlen(&raw mut created as *mut ::core::ffi::c_char);
    let mut password_len: size_t = strlen(password);
    let mut concat_len: size_t = (nonce_len as size_t)
        .wrapping_add(created_len)
        .wrapping_add(password_len);
    let mut concatenated: *mut ::core::ffi::c_char =
        malloc(concat_len.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    if concatenated.is_null() {
        free(base64_nonce as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    memcpy(
        concatenated as *mut ::core::ffi::c_void,
        &raw mut nonce_bytes as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        nonce_len as size_t,
    );
    memcpy(
        concatenated.offset(nonce_len as isize) as *mut ::core::ffi::c_void,
        &raw mut created as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        created_len,
    );
    memcpy(
        concatenated
            .offset(nonce_len as isize)
            .offset(created_len as isize) as *mut ::core::ffi::c_void,
        password as *const ::core::ffi::c_void,
        password_len,
    );
    let mut digest: [::core::ffi::c_uchar; 20] = [0; 20];
    mbedtls_sha1(
        concatenated as *mut ::core::ffi::c_uchar,
        concat_len,
        &raw mut digest as *mut ::core::ffi::c_uchar,
    );
    free(concatenated as *mut ::core::ffi::c_void);
    let mut base64_digest_buf_len: size_t = (4 as ::core::ffi::c_int * 20 as ::core::ffi::c_int
        / 3 as ::core::ffi::c_int
        + 5 as ::core::ffi::c_int) as size_t;
    let mut base64_digest: *mut ::core::ffi::c_char =
        malloc(base64_digest_buf_len) as *mut ::core::ffi::c_char;
    if base64_digest.is_null() {
        free(base64_nonce as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut base64_digest_len: size_t = 0 as size_t;
    mbedtls_base64_encode(
        base64_digest as *mut ::core::ffi::c_uchar,
        base64_digest_buf_len,
        &raw mut base64_digest_len,
        &raw mut digest as *mut ::core::ffi::c_uchar,
        20 as size_t,
    );
    *base64_digest.offset(base64_digest_len as isize) = '\0' as ::core::ffi::c_char;
    let mut header: *mut ::core::ffi::c_char = malloc(1024 as size_t) as *mut ::core::ffi::c_char;
    if header.is_null() {
        free(base64_nonce as *mut ::core::ffi::c_void);
        free(base64_digest as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    sprintf(
        header,
        b"<wsse:Security s:mustUnderstand=\"1\" xmlns:wsse=\"http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd\" xmlns:wsu=\"http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd\"><wsse:UsernameToken wsu:Id=\"UsernameToken-1\"><wsse:Username>%s</wsse:Username><wsse:Password Type=\"http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest\">%s</wsse:Password><wsse:Nonce EncodingType=\"http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-soap-message-security-1.0#Base64Binary\">%s</wsse:Nonce><wsu:Created>%s</wsu:Created></wsse:UsernameToken></wsse:Security>\0"
            .as_ptr() as *const ::core::ffi::c_char,
        username,
        base64_digest,
        base64_nonce,
        &raw mut created as *mut ::core::ffi::c_char,
    );
    free(base64_nonce as *mut ::core::ffi::c_void);
    free(base64_digest as *mut ::core::ffi::c_void);
    return header;
}
unsafe extern "C" fn find_with_ns(
    mut parent: ezxml_t,
    mut local_name: *const ::core::ffi::c_char,
    mut ns_prefixes: *mut *const ::core::ffi::c_char,
    mut ns_count: ::core::ffi::c_int,
) -> ezxml_t {
    if parent.is_null() {
        return ::core::ptr::null_mut::<ezxml>();
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < ns_count {
        let mut qname: [::core::ffi::c_char; 256] = [0; 256];
        snprintf(
            &raw mut qname as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            b"%s:%s\0".as_ptr() as *const ::core::ffi::c_char,
            *ns_prefixes.offset(i as isize),
            local_name,
        );
        let mut child: ezxml_t = ezxml_child(parent, &raw mut qname as *mut ::core::ffi::c_char);
        if !child.is_null() {
            return child;
        }
        i += 1;
    }
    return ezxml_child(parent, local_name);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn onvif_log_soap_fault(
    mut response: *const ::core::ffi::c_char,
    mut response_len: size_t,
    mut context: *const ::core::ffi::c_char,
) {
    if response.is_null() || response_len == 0 as size_t {
        return;
    }
    let mut ctx: *const ::core::ffi::c_char = if !context.is_null() {
        context
    } else {
        b"ONVIF\0".as_ptr() as *const ::core::ffi::c_char
    };
    let mut buf: *mut ::core::ffi::c_char =
        malloc(response_len.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    if buf.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] SOAP fault could not be parsed (allocation failed)\0".as_ptr()
                as *const ::core::ffi::c_char,
            ctx,
        );
        return;
    }
    memcpy(
        buf as *mut ::core::ffi::c_void,
        response as *const ::core::ffi::c_void,
        response_len,
    );
    *buf.offset(response_len as isize) = '\0' as ::core::ffi::c_char;
    let mut xml: ezxml_t = ezxml_parse_str(buf, response_len);
    if xml.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] SOAP fault response is not parseable XML\0".as_ptr()
                as *const ::core::ffi::c_char,
            ctx,
        );
        free(buf as *mut ::core::ffi::c_void);
        return;
    }
    let mut env_ns: [*const ::core::ffi::c_char; 5] = [
        b"s\0".as_ptr() as *const ::core::ffi::c_char,
        b"S\0".as_ptr() as *const ::core::ffi::c_char,
        b"SOAP-ENV\0".as_ptr() as *const ::core::ffi::c_char,
        b"soap\0".as_ptr() as *const ::core::ffi::c_char,
        b"env\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut env_ns_count: ::core::ffi::c_int =
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 5]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
            as ::core::ffi::c_int;
    let mut body: ezxml_t = find_with_ns(
        xml,
        b"Body\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut env_ns as *mut *const ::core::ffi::c_char,
        env_ns_count,
    );
    let mut fault: ezxml_t = if !body.is_null() {
        find_with_ns(
            body,
            b"Fault\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut env_ns as *mut *const ::core::ffi::c_char,
            env_ns_count,
        )
    } else {
        ::core::ptr::null_mut::<ezxml>()
    };
    if fault.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] SOAP error response has no Fault element\0".as_ptr()
                as *const ::core::ffi::c_char,
            ctx,
        );
        ezxml_free(xml);
        free(buf as *mut ::core::ffi::c_void);
        return;
    }
    let mut code_str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut subcode_str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut code_elem: ezxml_t = find_with_ns(
        fault,
        b"Code\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut env_ns as *mut *const ::core::ffi::c_char,
        env_ns_count,
    );
    if !code_elem.is_null() {
        let mut value_elem: ezxml_t = find_with_ns(
            code_elem,
            b"Value\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut env_ns as *mut *const ::core::ffi::c_char,
            env_ns_count,
        );
        if !value_elem.is_null() {
            code_str = if !value_elem.is_null() {
                (*value_elem).txt
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            };
        }
        let mut subcode_elem: ezxml_t = find_with_ns(
            code_elem,
            b"Subcode\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut env_ns as *mut *const ::core::ffi::c_char,
            env_ns_count,
        );
        if !subcode_elem.is_null() {
            let mut sub_value: ezxml_t = find_with_ns(
                subcode_elem,
                b"Value\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut env_ns as *mut *const ::core::ffi::c_char,
                env_ns_count,
            );
            if !sub_value.is_null() {
                subcode_str = if !sub_value.is_null() {
                    (*sub_value).txt
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                };
            }
        }
    }
    let mut reason_str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut reason_elem: ezxml_t = find_with_ns(
        fault,
        b"Reason\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut env_ns as *mut *const ::core::ffi::c_char,
        env_ns_count,
    );
    if !reason_elem.is_null() {
        let mut text_elem: ezxml_t = find_with_ns(
            reason_elem,
            b"Text\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut env_ns as *mut *const ::core::ffi::c_char,
            env_ns_count,
        );
        if !text_elem.is_null() {
            reason_str = if !text_elem.is_null() {
                (*text_elem).txt
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            };
        }
    }
    if reason_str.is_null()
        || *reason_str.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        let mut faultstring: ezxml_t = ezxml_child(
            fault,
            b"faultstring\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !faultstring.is_null() {
            reason_str = if !faultstring.is_null() {
                (*faultstring).txt
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            };
        }
    }
    let mut have_reason: bool = !reason_str.is_null()
        && *reason_str.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
    if !code_str.is_null()
        && *code_str.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        if !subcode_str.is_null()
            && *subcode_str.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] SOAP Fault: Code=%s, Subcode=%s, Reason=%s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ctx,
                code_str,
                subcode_str,
                if have_reason as ::core::ffi::c_int != 0 {
                    b"provided\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"none\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"[%s] SOAP Fault: Code=%s, Reason=%s\0".as_ptr() as *const ::core::ffi::c_char,
                ctx,
                code_str,
                if have_reason as ::core::ffi::c_int != 0 {
                    b"provided\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"none\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
    } else if have_reason {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] SOAP Fault: Reason provided\0".as_ptr() as *const ::core::ffi::c_char,
            ctx,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"[%s] SOAP Fault without structured code or reason\0".as_ptr()
                as *const ::core::ffi::c_char,
            ctx,
        );
    }
    ezxml_free(xml);
    free(buf as *mut ::core::ffi::c_void);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
