unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub const GO2RTC_CANDIDATE_SIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
unsafe extern "C" fn candidate_port_ok(mut p: *const ::core::ffi::c_char, mut n: size_t) -> bool {
    if n == 0 as size_t || n > 5 as size_t {
        return r#false != 0;
    }
    let mut v: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut i: size_t = 0 as size_t;
    while i < n {
        if (*p.offset(i as isize) as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || *p.offset(i as isize) as ::core::ffi::c_int > '9' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        v = v * 10 as ::core::ffi::c_long
            + (*p.offset(i as isize) as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                as ::core::ffi::c_long;
        i = i.wrapping_add(1);
    }
    return v >= 1 as ::core::ffi::c_long && v <= 65535 as ::core::ffi::c_long;
}
unsafe extern "C" fn candidate_host_ok(mut h: *const ::core::ffi::c_char, mut n: size_t) -> bool {
    if n == 0 as size_t || n > 253 as size_t {
        return r#false != 0;
    }
    let mut label: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < n {
        let mut c: ::core::ffi::c_char = *h.offset(i as isize);
        if c as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
            if label == 0 as size_t
                || *h.offset(i.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                    == '-' as ::core::ffi::c_int
            {
                return r#false != 0;
            }
            label = 0 as size_t;
        } else {
            let mut alnum: bool = c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
                || c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && c as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
                || c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int;
            if !alnum && c as ::core::ffi::c_int != '-' as ::core::ffi::c_int {
                return r#false != 0;
            }
            if c as ::core::ffi::c_int == '-' as ::core::ffi::c_int && label == 0 as size_t {
                return r#false != 0;
            }
            label = label.wrapping_add(1);
            if label > 63 as size_t {
                return r#false != 0;
            }
        }
        i = i.wrapping_add(1);
    }
    return label > 0 as size_t
        && *h.offset(n.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            != '-' as ::core::ffi::c_int;
}
unsafe extern "C" fn candidate_ipv6_ok(mut h: *const ::core::ffi::c_char, mut n: size_t) -> bool {
    if n < 2 as size_t || n > 45 as size_t {
        return r#false != 0;
    }
    let mut colons: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    while i < n {
        let mut c: ::core::ffi::c_char = *h.offset(i as isize);
        if c as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
            colons += 1;
        } else {
            let mut hex: bool = c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                || c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                    && c as ::core::ffi::c_int <= 'f' as ::core::ffi::c_int
                || c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && c as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int;
            if !hex && c as ::core::ffi::c_int != '.' as ::core::ffi::c_int {
                return r#false != 0;
            }
        }
        i = i.wrapping_add(1);
    }
    return colons >= 2 as ::core::ffi::c_int;
}
unsafe extern "C" fn candidate_address(
    mut e: *const ::core::ffi::c_char,
    mut n: size_t,
    mut default_port: ::core::ffi::c_int,
    mut port_optional_out: bool,
    mut out: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> bool {
    let mut host: *const ::core::ffi::c_char = e;
    let mut port: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut host_len: size_t = n;
    let mut port_len: size_t = 0 as size_t;
    let mut v6: bool = r#false != 0;
    if n > 0 as size_t && *e.offset(0isize) as ::core::ffi::c_int == '[' as ::core::ffi::c_int {
        let mut close: *const ::core::ffi::c_char = memchr(
            e as *const ::core::ffi::c_void,
            ']' as ::core::ffi::c_int,
            n,
        ) as *const ::core::ffi::c_char;
        if close.is_null() {
            return r#false != 0;
        }
        host = e.offset(1 as ::core::ffi::c_int as isize);
        host_len = close.offset_from(host) as size_t;
        let mut rest: size_t = n
            .wrapping_sub(close.offset_from(e) as size_t)
            .wrapping_sub(1 as size_t);
        if rest > 0 as size_t {
            if *close.offset(1isize) as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                return r#false != 0;
            }
            port = close.offset(2 as ::core::ffi::c_int as isize);
            port_len = rest.wrapping_sub(1 as size_t);
            if port_len == 0 as size_t {
                return r#false != 0;
            }
        }
        if !candidate_ipv6_ok(host, host_len) {
            return r#false != 0;
        }
        v6 = r#true != 0;
    } else {
        let mut first: *const ::core::ffi::c_char = memchr(
            e as *const ::core::ffi::c_void,
            ':' as ::core::ffi::c_int,
            n,
        ) as *const ::core::ffi::c_char;
        if !first.is_null()
            && !memchr(
                first.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                ':' as ::core::ffi::c_int,
                n.wrapping_sub(first.offset_from(e) as size_t)
                    .wrapping_sub(1 as size_t),
            )
            .is_null()
        {
            if !candidate_ipv6_ok(e, n) {
                return r#false != 0;
            }
            v6 = r#true != 0;
        } else if !first.is_null() {
            host_len = first.offset_from(e) as size_t;
            port = first.offset(1 as ::core::ffi::c_int as isize);
            port_len = n.wrapping_sub(host_len).wrapping_sub(1 as size_t);
            if port_len == 0 as size_t {
                return r#false != 0;
            }
        }
        if !v6
            && !(host_len == 1 as size_t
                && *host.offset(0isize) as ::core::ffi::c_int == '*' as ::core::ffi::c_int)
            && !candidate_host_ok(host, host_len)
        {
            return r#false != 0;
        }
    }
    if !port.is_null() && !candidate_port_ok(port, port_len) {
        return r#false != 0;
    }
    let mut w: ::core::ffi::c_int = 0;
    if !port.is_null() {
        w = snprintf(
            out,
            out_size,
            if v6 as ::core::ffi::c_int != 0 {
                b"[%.*s]:%.*s\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"%.*s:%.*s\0".as_ptr() as *const ::core::ffi::c_char
            },
            host_len as ::core::ffi::c_int,
            host,
            port_len as ::core::ffi::c_int,
            port,
        );
    } else if port_optional_out {
        w = snprintf(
            out,
            out_size,
            if v6 as ::core::ffi::c_int != 0 {
                b"[%.*s]\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"%.*s\0".as_ptr() as *const ::core::ffi::c_char
            },
            host_len as ::core::ffi::c_int,
            host,
        );
    } else {
        w = snprintf(
            out,
            out_size,
            if v6 as ::core::ffi::c_int != 0 {
                b"[%.*s]:%d\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"%.*s:%d\0".as_ptr() as *const ::core::ffi::c_char
            },
            host_len as ::core::ffi::c_int,
            host,
            default_port,
        );
    }
    return w > 0 as ::core::ffi::c_int && (w as size_t) < out_size;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn go2rtc_parse_external_candidates(
    mut list: *const ::core::ffi::c_char,
    mut port: ::core::ffi::c_int,
    mut out: *mut [::core::ffi::c_char; 128],
    mut max: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if list.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if port <= 0 as ::core::ffi::c_int || port > 65535 as ::core::ffi::c_int {
        port = 8555 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *const ::core::ffi::c_char = list;
    loop {
        while *p as ::core::ffi::c_int == ',' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == ';' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        if *p == 0 {
            break;
        }
        let mut start: *const ::core::ffi::c_char = p;
        while *p as ::core::ffi::c_int != 0
            && *p as ::core::ffi::c_int != ',' as ::core::ffi::c_int
            && *p as ::core::ffi::c_int != ';' as ::core::ffi::c_int
            && *p as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
            && *p as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
            && *p as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
            && *p as ::core::ffi::c_int != '\r' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        let mut n: size_t = p.offset_from(start) as size_t;
        let mut candidate: [::core::ffi::c_char; 128] = [0; 128];
        let mut ok: bool = false;
        if n == 1 as size_t
            && *start.offset(0isize) as ::core::ffi::c_int == '*' as ::core::ffi::c_int
            || n == 4 as size_t
                && strncasecmp(
                    start,
                    b"auto\0".as_ptr() as *const ::core::ffi::c_char,
                    4 as size_t,
                ) == 0 as ::core::ffi::c_int
        {
            ok = snprintf(
                &raw mut candidate as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                b"*:%d\0".as_ptr() as *const ::core::ffi::c_char,
                port,
            ) > 0 as ::core::ffi::c_int;
        } else if n > 5 as size_t
            && strncasecmp(
                start,
                b"stun:\0".as_ptr() as *const ::core::ffi::c_char,
                5 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            let mut addr: [::core::ffi::c_char; 128] = [0; 128];
            ok = candidate_address(
                start.offset(5 as ::core::ffi::c_int as isize),
                n.wrapping_sub(5 as size_t),
                port,
                r#true != 0,
                &raw mut addr as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            ) as ::core::ffi::c_int
                != 0
                && addr[0usize] as ::core::ffi::c_int != '*' as ::core::ffi::c_int
                && snprintf(
                    &raw mut candidate as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                    b"stun:%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut addr as *mut ::core::ffi::c_char,
                ) < ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as ::core::ffi::c_int;
        } else {
            ok = candidate_address(
                start,
                n,
                port,
                r#false != 0,
                &raw mut candidate as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            );
        }
        if !ok {
            return -1 as ::core::ffi::c_int;
        }
        let mut duplicate: bool = r#false != 0;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < count {
            if strcmp(
                &raw mut *out.offset(i as isize) as *mut ::core::ffi::c_char,
                &raw mut candidate as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                duplicate = r#true != 0;
                break;
            } else {
                i += 1;
            }
        }
        if duplicate {
            continue;
        }
        if count >= max {
            return -1 as ::core::ffi::c_int;
        }
        safe_strcpy(
            &raw mut *out.offset(count as isize) as *mut ::core::ffi::c_char,
            &raw mut candidate as *mut ::core::ffi::c_char,
            GO2RTC_CANDIDATE_SIZE as size_t,
            0 as size_t,
        );
        count += 1;
    }
    return count;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
