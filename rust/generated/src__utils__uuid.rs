use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type mbedtls_md_info_t;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn getrandom(
        __buffer: *mut ::core::ffi::c_void,
        __length: size_t,
        __flags: ::core::ffi::c_uint,
    ) -> ssize_t;
    unsafe fn mbedtls_md_info_from_type(md_type: mbedtls_md_type_t) -> *const mbedtls_md_info_t;
    unsafe fn mbedtls_md_init(ctx: *mut mbedtls_md_context_t);
    unsafe fn mbedtls_md_free(ctx: *mut mbedtls_md_context_t);
    unsafe fn mbedtls_md_setup(
        ctx: *mut mbedtls_md_context_t,
        md_info: *const mbedtls_md_info_t,
        hmac: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_md_starts(ctx: *mut mbedtls_md_context_t) -> ::core::ffi::c_int;
    unsafe fn mbedtls_md_update(
        ctx: *mut mbedtls_md_context_t,
        input: *const ::core::ffi::c_uchar,
        ilen: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_md_finish(
        ctx: *mut mbedtls_md_context_t,
        output: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type ssize_t = isize;
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
pub struct mbedtls_md_context_t {
    pub private_md_info: *const mbedtls_md_info_t,
    pub private_md_ctx: *mut ::core::ffi::c_void,
    pub private_hmac_ctx: *mut ::core::ffi::c_void,
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
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LIGHTNVR_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lightnvr_uuid_is_valid(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null() || strnlen(value, LIGHTNVR_UUID_STRING_SIZE as size_t) != 36 as size_t {
        return r#false != 0;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < 36 as ::core::ffi::c_int {
        if index == 8 as ::core::ffi::c_int
            || index == 13 as ::core::ffi::c_int
            || index == 18 as ::core::ffi::c_int
            || index == 23 as ::core::ffi::c_int
        {
            if *value.offset(index as isize) as ::core::ffi::c_int != '-' as ::core::ffi::c_int {
                return r#false != 0;
            }
        } else if *(*__ctype_b_loc()).offset(
            *value.offset(index as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
        ) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISxdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        {
            return r#false != 0;
        }
        index += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn random_bytes(
    mut buffer: *mut ::core::ffi::c_uchar,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut offset: size_t = 0 as size_t;
    while offset < length {
        let mut count: ssize_t = getrandom(
            buffer.offset(offset as isize) as *mut ::core::ffi::c_void,
            length.wrapping_sub(offset),
            0 as ::core::ffi::c_uint,
        );
        if count > 0 as ssize_t {
            offset = offset.wrapping_add(count as size_t);
        } else if !(count < 0 as ssize_t && *__errno_location() == EINTR) {
            break;
        }
    }
    if offset == length {
        return 0 as ::core::ffi::c_int;
    }
    let mut random: *mut FILE = fopen(
        b"/dev/urandom\0".as_ptr() as *const ::core::ffi::c_char,
        b"rb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if random.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut count_0: size_t = fread(
        buffer as *mut ::core::ffi::c_void,
        1 as size_t,
        length,
        random,
    ) as size_t;
    fclose(random);
    return if count_0 == length {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn hex_digit(mut value: ::core::ffi::c_uchar) -> ::core::ffi::c_char {
    static mut alphabet: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"0123456789abcdef\0")
    };
    return alphabet[(value as ::core::ffi::c_uint & 0xf as ::core::ffi::c_uint) as usize];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lightnvr_uuid_generate_v4(
    mut output: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if output.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut bytes: [::core::ffi::c_uchar; 16] = [0; 16];
    if random_bytes(
        &raw mut bytes as *mut ::core::ffi::c_uchar,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 16]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    bytes[6usize] = (bytes[6usize] as ::core::ffi::c_uint & 0xf as ::core::ffi::c_uint
        | 0x40 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    bytes[8usize] = (bytes[8usize] as ::core::ffi::c_uint & 0x3f as ::core::ffi::c_uint
        | 0x80 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    let mut position: size_t = 0 as size_t;
    let mut index: size_t = 0 as size_t;
    while index < ::core::mem::size_of::<[::core::ffi::c_uchar; 16]>() {
        if index == 4 as size_t
            || index == 6 as size_t
            || index == 8 as size_t
            || index == 10 as size_t
        {
            let c2rust_fresh0 = position;
            position = position.wrapping_add(1);
            *output.offset(c2rust_fresh0 as isize) = '-' as ::core::ffi::c_char;
        }
        let c2rust_fresh1 = position;
        position = position.wrapping_add(1);
        *output.offset(c2rust_fresh1 as isize) = hex_digit(
            (bytes[index] as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
        );
        let c2rust_fresh2 = position;
        position = position.wrapping_add(1);
        *output.offset(c2rust_fresh2 as isize) = hex_digit(bytes[index]);
        index = index.wrapping_add(1);
    }
    *output.offset(position as isize) = '\0' as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn hex_value(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    if c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
        && c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
    {
        return c as ::core::ffi::c_int - '0' as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
        && c as ::core::ffi::c_int <= 'f' as ::core::ffi::c_int
    {
        return c as ::core::ffi::c_int - 'a' as ::core::ffi::c_int + 10 as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
        && c as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
    {
        return c as ::core::ffi::c_int - 'A' as ::core::ffi::c_int + 10 as ::core::ffi::c_int;
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lightnvr_uuid_generate_v5(
    mut namespace_uuid: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut name_len: size_t,
    mut output: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if output.is_null() || name.is_null() || !lightnvr_uuid_is_valid(namespace_uuid) {
        return -1 as ::core::ffi::c_int;
    }
    let mut namespace_bytes: [::core::ffi::c_uchar; 16] = [0; 16];
    let mut byte: size_t = 0 as size_t;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < 36 as ::core::ffi::c_int {
        if !(index == 8 as ::core::ffi::c_int
            || index == 13 as ::core::ffi::c_int
            || index == 18 as ::core::ffi::c_int
            || index == 23 as ::core::ffi::c_int)
        {
            let mut high: ::core::ffi::c_int = hex_value(*namespace_uuid.offset(index as isize));
            index += 1;
            let mut low: ::core::ffi::c_int = hex_value(*namespace_uuid.offset(index as isize));
            let c2rust_fresh3 = byte;
            byte = byte.wrapping_add(1);
            namespace_bytes[c2rust_fresh3] =
                (high << 4 as ::core::ffi::c_int | low) as ::core::ffi::c_uchar;
        }
        index += 1;
    }
    let mut sha1: *const mbedtls_md_info_t =
        mbedtls_md_info_from_type(mbedtls_md_type_t::MBEDTLS_MD_SHA1);
    if sha1.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut context: mbedtls_md_context_t = mbedtls_md_context_t {
        private_md_info: ::core::ptr::null::<mbedtls_md_info_t>(),
        private_md_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        private_hmac_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    mbedtls_md_init(&raw mut context);
    let mut digest: [::core::ffi::c_uchar; 20] = [0; 20];
    let mut rc: ::core::ffi::c_int =
        mbedtls_md_setup(&raw mut context, sha1, 0 as ::core::ffi::c_int);
    if rc == 0 as ::core::ffi::c_int {
        rc = mbedtls_md_starts(&raw mut context);
    }
    if rc == 0 as ::core::ffi::c_int {
        rc = mbedtls_md_update(
            &raw mut context,
            &raw mut namespace_bytes as *mut ::core::ffi::c_uchar,
            ::core::mem::size_of::<[::core::ffi::c_uchar; 16]>(),
        );
    }
    if rc == 0 as ::core::ffi::c_int {
        rc = mbedtls_md_update(
            &raw mut context,
            name as *const ::core::ffi::c_uchar,
            name_len,
        );
    }
    if rc == 0 as ::core::ffi::c_int {
        rc = mbedtls_md_finish(
            &raw mut context,
            &raw mut digest as *mut ::core::ffi::c_uchar,
        );
    }
    mbedtls_md_free(&raw mut context);
    if rc != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    digest[6usize] = (digest[6usize] as ::core::ffi::c_uint & 0xf as ::core::ffi::c_uint
        | 0x50 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    digest[8usize] = (digest[8usize] as ::core::ffi::c_uint & 0x3f as ::core::ffi::c_uint
        | 0x80 as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
    let mut position: size_t = 0 as size_t;
    let mut index_0: size_t = 0 as size_t;
    while index_0 < 16 as size_t {
        if index_0 == 4 as size_t
            || index_0 == 6 as size_t
            || index_0 == 8 as size_t
            || index_0 == 10 as size_t
        {
            let c2rust_fresh4 = position;
            position = position.wrapping_add(1);
            *output.offset(c2rust_fresh4 as isize) = '-' as ::core::ffi::c_char;
        }
        let c2rust_fresh5 = position;
        position = position.wrapping_add(1);
        *output.offset(c2rust_fresh5 as isize) = hex_digit(
            (digest[index_0] as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar,
        );
        let c2rust_fresh6 = position;
        position = position.wrapping_add(1);
        *output.offset(c2rust_fresh6 as isize) = hex_digit(digest[index_0]);
        index_0 = index_0.wrapping_add(1);
    }
    *output.offset(position as isize) = '\0' as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
