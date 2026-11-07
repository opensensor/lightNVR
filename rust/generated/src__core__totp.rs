unsafe extern "C" {
    pub type mbedtls_md_info_t;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn __ctype_toupper_loc() -> *mut *const __int32_t;
    unsafe fn mbedtls_md_info_from_type(md_type: mbedtls_md_type_t) -> *const mbedtls_md_info_t;
    unsafe fn mbedtls_md_hmac(
        md_info: *const mbedtls_md_info_t,
        key: *const ::core::ffi::c_uchar,
        keylen: size_t,
        input: *const ::core::ffi::c_uchar,
        ilen: size_t,
        output: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_entropy_init(ctx: *mut mbedtls_entropy_context);
    unsafe fn mbedtls_entropy_free(ctx: *mut mbedtls_entropy_context);
    unsafe fn mbedtls_entropy_func(
        data: *mut ::core::ffi::c_void,
        output: *mut ::core::ffi::c_uchar,
        len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_ctr_drbg_init(ctx: *mut mbedtls_ctr_drbg_context);
    unsafe fn mbedtls_ctr_drbg_seed(
        ctx: *mut mbedtls_ctr_drbg_context,
        f_entropy: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_uchar,
                size_t,
            ) -> ::core::ffi::c_int,
        >,
        p_entropy: *mut ::core::ffi::c_void,
        custom: *const ::core::ffi::c_uchar,
        len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_ctr_drbg_free(ctx: *mut mbedtls_ctr_drbg_context);
    unsafe fn mbedtls_ctr_drbg_random(
        p_rng: *mut ::core::ffi::c_void,
        output: *mut ::core::ffi::c_uchar,
        output_len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
}
pub type size_t = usize;
pub type __int32_t = i32;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __glibc_reserved: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mbedtls_md_type_t(pub ::core::ffi::c_uint);
impl mbedtls_md_type_t {
    pub const MBEDTLS_MD_NONE: Self = Self(0);
    pub const MBEDTLS_MD_MD5: Self = Self(3);
    pub const MBEDTLS_MD_RIPEMD160: Self = Self(4);
    pub const MBEDTLS_MD_SHA1: Self = Self(5);
    pub const MBEDTLS_MD_SHA224: Self = Self(8);
    pub const MBEDTLS_MD_SHA256: Self = Self(9);
    pub const MBEDTLS_MD_SHA384: Self = Self(10);
    pub const MBEDTLS_MD_SHA512: Self = Self(11);
    pub const MBEDTLS_MD_SHA3_224: Self = Self(16);
    pub const MBEDTLS_MD_SHA3_256: Self = Self(17);
    pub const MBEDTLS_MD_SHA3_384: Self = Self(18);
    pub const MBEDTLS_MD_SHA3_512: Self = Self(19);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_entropy_context {
    pub private_accumulator: mbedtls_md_context_t,
    pub private_accumulator_started: ::core::ffi::c_int,
    pub private_source_count: ::core::ffi::c_int,
    pub private_source: [mbedtls_entropy_source_state; 20],
    pub private_mutex: mbedtls_threading_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_threading_mutex_t {
    pub private_mutex: pthread_mutex_t,
    pub private_state: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_entropy_source_state {
    pub private_f_source: mbedtls_entropy_f_source_ptr,
    pub private_p_source: *mut ::core::ffi::c_void,
    pub private_size: size_t,
    pub private_threshold: size_t,
    pub private_strong: ::core::ffi::c_int,
}
pub type mbedtls_entropy_f_source_ptr = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_uchar,
        size_t,
        *mut size_t,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_md_context_t {
    pub private_md_info: *const mbedtls_md_info_t,
    pub private_md_ctx: *mut ::core::ffi::c_void,
    pub private_hmac_ctx: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_ctr_drbg_context {
    pub private_counter: [::core::ffi::c_uchar; 16],
    pub private_reseed_counter: ::core::ffi::c_int,
    pub private_prediction_resistance: ::core::ffi::c_int,
    pub private_entropy_len: size_t,
    pub private_reseed_interval: ::core::ffi::c_int,
    pub private_aes_ctx: mbedtls_aes_context,
    pub private_f_entropy: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_uchar,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub private_p_entropy: *mut ::core::ffi::c_void,
    pub private_mutex: mbedtls_threading_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_aes_context {
    pub private_nr: ::core::ffi::c_int,
    pub private_rk_offset: size_t,
    pub private_buf: [uint32_t; 68],
}
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[inline]
unsafe extern "C" fn toupper(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -128 as ::core::ffi::c_int && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
static mut BASE32_ALPHABET: [::core::ffi::c_char; 33] = unsafe {
    ::core::mem::transmute::<[u8; 33], [::core::ffi::c_char; 33]>(
        *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567\0",
    )
};
unsafe extern "C" fn base32_encode(
    mut data: *const ::core::ffi::c_uchar,
    mut data_len: size_t,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) -> ::core::ffi::c_int {
    if data.is_null() || output.is_null() || output_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut out_len: size_t = data_len
        .wrapping_mul(8 as size_t)
        .wrapping_add(4 as size_t)
        .wrapping_div(5 as size_t);
    if out_len.wrapping_add(1 as size_t) > output_size {
        return -1 as ::core::ffi::c_int;
    }
    let mut i: size_t = 0;
    let mut j: size_t = 0 as size_t;
    let mut buffer: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut bits_left: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as size_t;
    while i < data_len {
        buffer =
            buffer << 8 as ::core::ffi::c_int | *data.offset(i as isize) as ::core::ffi::c_uint;
        bits_left += 8 as ::core::ffi::c_int;
        while bits_left >= 5 as ::core::ffi::c_int {
            bits_left -= 5 as ::core::ffi::c_int;
            let c2rust_fresh1 = j;
            j = j.wrapping_add(1);
            *output.offset(c2rust_fresh1 as isize) =
                BASE32_ALPHABET[(buffer >> bits_left & 0x1f as ::core::ffi::c_uint) as usize];
        }
        i = i.wrapping_add(1);
    }
    if bits_left > 0 as ::core::ffi::c_int {
        let c2rust_fresh2 = j;
        j = j.wrapping_add(1);
        *output.offset(c2rust_fresh2 as isize) =
            BASE32_ALPHABET[(buffer << 5 as ::core::ffi::c_int - bits_left
                & 0x1f as ::core::ffi::c_uint) as usize];
    }
    *output.offset(j as isize) = '\0' as ::core::ffi::c_char;
    return j as ::core::ffi::c_int;
}
unsafe extern "C" fn base32_decode(
    mut input: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_uchar,
    mut output_size: size_t,
) -> ::core::ffi::c_int {
    if input.is_null() || output.is_null() || output_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut input_len: size_t = strlen(input);
    let mut out_len: size_t = input_len
        .wrapping_mul(5 as size_t)
        .wrapping_div(8 as size_t);
    if out_len > output_size {
        return -1 as ::core::ffi::c_int;
    }
    let mut buffer: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut bits_left: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < input_len {
        let mut c: ::core::ffi::c_char = ({
            let mut __res: ::core::ffi::c_int = 0;
            if ::core::mem::size_of::<::core::ffi::c_uchar>() > 1usize {
                if 0 as ::core::ffi::c_int != 0 {
                    let mut __c: ::core::ffi::c_int =
                        *input.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int;
                    __res = (if __c < -128 as ::core::ffi::c_int || __c > 255 as ::core::ffi::c_int
                    {
                        __c as __int32_t
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    }) as ::core::ffi::c_int;
                } else {
                    __res = toupper(
                        *input.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int
                    );
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(*input.offset(i as isize)
                    as ::core::ffi::c_uchar
                    as ::core::ffi::c_int
                    as isize) as ::core::ffi::c_int;
            }
            __res
        }) as ::core::ffi::c_char;
        if !(c as ::core::ffi::c_int == '=' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == ' ' as ::core::ffi::c_int)
        {
            let mut val: ::core::ffi::c_int = 0;
            if c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
            {
                val = c as ::core::ffi::c_int - 'A' as ::core::ffi::c_int;
            } else if c as ::core::ffi::c_int >= '2' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= '7' as ::core::ffi::c_int
            {
                val =
                    c as ::core::ffi::c_int - '2' as ::core::ffi::c_int + 26 as ::core::ffi::c_int;
            } else {
                return -1 as ::core::ffi::c_int;
            }
            buffer = buffer << 5 as ::core::ffi::c_int | val as ::core::ffi::c_uint;
            bits_left += 5 as ::core::ffi::c_int;
            if bits_left >= 8 as ::core::ffi::c_int {
                bits_left -= 8 as ::core::ffi::c_int;
                if j < output_size {
                    let c2rust_fresh0 = j;
                    j = j.wrapping_add(1);
                    *output.offset(c2rust_fresh0 as isize) =
                        (buffer >> bits_left) as ::core::ffi::c_uchar;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    return j as ::core::ffi::c_int;
}
unsafe extern "C" fn hmac_sha1(
    mut key: *const ::core::ffi::c_uchar,
    mut key_len: size_t,
    mut data: *const ::core::ffi::c_uchar,
    mut data_len: size_t,
    mut output: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut md_info: *const mbedtls_md_info_t =
        mbedtls_md_info_from_type(mbedtls_md_type_t::MBEDTLS_MD_SHA1);
    if md_info.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    return mbedtls_md_hmac(md_info, key, key_len, data, data_len, output);
}
unsafe extern "C" fn totp_generate(
    mut secret_b32: *const ::core::ffi::c_char,
    mut time_step: uint64_t,
) -> ::core::ffi::c_int {
    let mut secret: [::core::ffi::c_uchar; 64] = [0; 64];
    let mut secret_len: ::core::ffi::c_int = base32_decode(
        secret_b32,
        &raw mut secret as *mut ::core::ffi::c_uchar,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 64]>(),
    );
    if secret_len <= 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"TOTP: Failed to decode base32 secret\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut msg: [::core::ffi::c_uchar; 8] = [0; 8];
    let mut i: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        msg[i as usize] = (time_step & 0xff as uint64_t) as ::core::ffi::c_uchar;
        time_step >>= 8 as ::core::ffi::c_int;
        i -= 1;
    }
    let mut hash: [::core::ffi::c_uchar; 20] = [0; 20];
    if hmac_sha1(
        &raw mut secret as *mut ::core::ffi::c_uchar,
        secret_len as size_t,
        &raw mut msg as *mut ::core::ffi::c_uchar,
        8 as size_t,
        &raw mut hash as *mut ::core::ffi::c_uchar,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"TOTP: HMAC-SHA1 failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut offset: ::core::ffi::c_int =
        hash[19usize] as ::core::ffi::c_int & 0xf as ::core::ffi::c_int;
    let mut code: uint32_t = ((hash[offset as usize] as ::core::ffi::c_int
        & 0x7f as ::core::ffi::c_int) as uint32_t)
        << 24 as ::core::ffi::c_int
        | ((hash[(offset + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int) as uint32_t)
            << 16 as ::core::ffi::c_int
        | ((hash[(offset + 2 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int) as uint32_t)
            << 8 as ::core::ffi::c_int
        | (hash[(offset + 3 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int) as uint32_t;
    return code.wrapping_rem(1000000 as uint32_t) as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn totp_match_step(
    mut secret_b32: *const ::core::ffi::c_char,
    mut code_str: *const ::core::ffi::c_char,
    mut now: time_t,
    mut matched_step: *mut int64_t,
) -> ::core::ffi::c_int {
    if !matched_step.is_null() {
        *matched_step = -1 as int64_t;
    }
    if secret_b32.is_null()
        || code_str.is_null()
        || matched_step.is_null()
        || now < 0 as time_t
        || strlen(code_str) != 6 as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 6 as ::core::ffi::c_int {
        if (*code_str.offset(i as isize) as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || *code_str.offset(i as isize) as ::core::ffi::c_int > '9' as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    let mut provided: ::core::ffi::c_int = strtol(
        code_str,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    let mut current: int64_t = now as int64_t / 30 as int64_t;
    let mut offset: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    while offset <= 1 as ::core::ffi::c_int {
        let mut step: int64_t = current + offset as int64_t;
        if step >= 0 as int64_t {
            let mut expected: ::core::ffi::c_int = totp_generate(secret_b32, step as uint64_t);
            if expected >= 0 as ::core::ffi::c_int && expected == provided {
                *matched_step = step;
            }
        }
        offset += 1;
    }
    return if *matched_step >= 0 as int64_t {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn totp_generate_secret(
    mut secret_b32: *mut ::core::ffi::c_char,
    mut secret_size: size_t,
) -> ::core::ffi::c_int {
    let mut raw_secret: [::core::ffi::c_uchar; 20] = [0; 20];
    let mut entropy: mbedtls_entropy_context = mbedtls_entropy_context {
        private_accumulator: mbedtls_md_context_t {
            private_md_info: ::core::ptr::null::<mbedtls_md_info_t>(),
            private_md_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            private_hmac_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
        private_accumulator_started: 0,
        private_source_count: 0,
        private_source: [mbedtls_entropy_source_state {
            private_f_source: None,
            private_p_source: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            private_size: 0,
            private_threshold: 0,
            private_strong: 0,
        }; 20],
        private_mutex: mbedtls_threading_mutex_t {
            private_mutex: pthread_mutex_t {
                __data: __pthread_mutex_s {
                    __lock: 0,
                    __count: 0,
                    __owner: 0,
                    __nusers: 0,
                    __kind: 0,
                    __spins: 0,
                    __glibc_reserved: 0,
                    __list: __pthread_list_t {
                        __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                        __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
                    },
                },
            },
            private_state: 0,
        },
    };
    let mut ctr_drbg: mbedtls_ctr_drbg_context = mbedtls_ctr_drbg_context {
        private_counter: [0; 16],
        private_reseed_counter: 0,
        private_prediction_resistance: 0,
        private_entropy_len: 0,
        private_reseed_interval: 0,
        private_aes_ctx: mbedtls_aes_context {
            private_nr: 0,
            private_rk_offset: 0,
            private_buf: [0; 68],
        },
        private_f_entropy: None,
        private_p_entropy: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        private_mutex: mbedtls_threading_mutex_t {
            private_mutex: pthread_mutex_t {
                __data: __pthread_mutex_s {
                    __lock: 0,
                    __count: 0,
                    __owner: 0,
                    __nusers: 0,
                    __kind: 0,
                    __spins: 0,
                    __glibc_reserved: 0,
                    __list: __pthread_list_t {
                        __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
                        __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
                    },
                },
            },
            private_state: 0,
        },
    };
    let mut pers: *const ::core::ffi::c_char =
        b"totp_secret_gen\0".as_ptr() as *const ::core::ffi::c_char;
    mbedtls_entropy_init(&raw mut entropy);
    mbedtls_ctr_drbg_init(&raw mut ctr_drbg);
    let mut ret: ::core::ffi::c_int = mbedtls_ctr_drbg_seed(
        &raw mut ctr_drbg,
        Some(
            mbedtls_entropy_func
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut entropy as *mut ::core::ffi::c_void,
        pers as *const ::core::ffi::c_uchar,
        strlen(pers),
    );
    if ret != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"TOTP: Failed to seed RNG: -0x%04x\0".as_ptr() as *const ::core::ffi::c_char,
            -ret,
        );
        mbedtls_ctr_drbg_free(&raw mut ctr_drbg);
        mbedtls_entropy_free(&raw mut entropy);
        return -1 as ::core::ffi::c_int;
    }
    ret = mbedtls_ctr_drbg_random(
        &raw mut ctr_drbg as *mut ::core::ffi::c_void,
        &raw mut raw_secret as *mut ::core::ffi::c_uchar,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 20]>(),
    );
    mbedtls_ctr_drbg_free(&raw mut ctr_drbg);
    mbedtls_entropy_free(&raw mut entropy);
    if ret != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"TOTP: Failed to generate random secret: -0x%04x\0".as_ptr()
                as *const ::core::ffi::c_char,
            -ret,
        );
        return -1 as ::core::ffi::c_int;
    }
    if base32_encode(
        &raw mut raw_secret as *mut ::core::ffi::c_uchar,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 20]>(),
        secret_b32,
        secret_size,
    ) < 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"TOTP: Failed to base32 encode secret\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
