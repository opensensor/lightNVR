use ::c2rust_bitfields;
unsafe extern "C" {
    pub type mbedtls_cmac_context_t;
    pub type mbedtls_md_info_t;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn __ctype_toupper_loc() -> *mut *const __int32_t;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn getrandom(
        __buffer: *mut ::core::ffi::c_void,
        __length: size_t,
        __flags: ::core::ffi::c_uint,
    ) -> ssize_t;
    unsafe fn mbedtls_gcm_init(ctx: *mut mbedtls_gcm_context);
    unsafe fn mbedtls_gcm_setkey(
        ctx: *mut mbedtls_gcm_context,
        cipher: mbedtls_cipher_id_t,
        key: *const ::core::ffi::c_uchar,
        keybits: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_gcm_crypt_and_tag(
        ctx: *mut mbedtls_gcm_context,
        mode: ::core::ffi::c_int,
        length: size_t,
        iv: *const ::core::ffi::c_uchar,
        iv_len: size_t,
        add: *const ::core::ffi::c_uchar,
        add_len: size_t,
        input: *const ::core::ffi::c_uchar,
        output: *mut ::core::ffi::c_uchar,
        tag_len: size_t,
        tag: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_gcm_auth_decrypt(
        ctx: *mut mbedtls_gcm_context,
        length: size_t,
        iv: *const ::core::ffi::c_uchar,
        iv_len: size_t,
        add: *const ::core::ffi::c_uchar,
        add_len: size_t,
        tag: *const ::core::ffi::c_uchar,
        tag_len: size_t,
        input: *const ::core::ffi::c_uchar,
        output: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    unsafe fn mbedtls_gcm_free(ctx: *mut mbedtls_gcm_context);
    unsafe fn mbedtls_md_info_from_type(md_type: mbedtls_md_type_t) -> *const mbedtls_md_info_t;
    unsafe fn mbedtls_md_hmac(
        md_info: *const mbedtls_md_info_t,
        key: *const ::core::ffi::c_uchar,
        keylen: size_t,
        input: *const ::core::ffi::c_uchar,
        ilen: size_t,
        output: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    unsafe fn secure_zero_memory(ptr: *mut ::core::ffi::c_void, size: size_t);
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __int32_t = i32;
pub type ssize_t = isize;
pub type uint8_t = u8;
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mbedtls_cipher_id_t(pub ::core::ffi::c_uint);
impl mbedtls_cipher_id_t {
    pub const MBEDTLS_CIPHER_ID_NONE: Self = Self(0);
    pub const MBEDTLS_CIPHER_ID_NULL: Self = Self(1);
    pub const MBEDTLS_CIPHER_ID_AES: Self = Self(2);
    pub const MBEDTLS_CIPHER_ID_DES: Self = Self(3);
    pub const MBEDTLS_CIPHER_ID_3DES: Self = Self(4);
    pub const MBEDTLS_CIPHER_ID_CAMELLIA: Self = Self(5);
    pub const MBEDTLS_CIPHER_ID_ARIA: Self = Self(6);
    pub const MBEDTLS_CIPHER_ID_CHACHA20: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_gcm_context {
    pub private_cipher_ctx: mbedtls_cipher_context_t,
    pub private_H: [[uint64_t; 2]; 16],
    pub private_len: uint64_t,
    pub private_add_len: uint64_t,
    pub private_base_ectr: [::core::ffi::c_uchar; 16],
    pub private_y: [::core::ffi::c_uchar; 16],
    pub private_buf: [::core::ffi::c_uchar; 16],
    pub private_mode: ::core::ffi::c_uchar,
    pub private_acceleration: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_cipher_context_t {
    pub private_cipher_info: *const mbedtls_cipher_info_t,
    pub private_key_bitlen: ::core::ffi::c_int,
    pub private_operation: mbedtls_operation_t,
    pub private_add_padding:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_uchar, size_t, size_t) -> ()>,
    pub private_get_padding: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_uchar,
            size_t,
            *mut size_t,
            *mut size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub private_unprocessed_data: [::core::ffi::c_uchar; 16],
    pub private_unprocessed_len: size_t,
    pub private_iv: [::core::ffi::c_uchar; 16],
    pub private_iv_size: size_t,
    pub private_cipher_ctx: *mut ::core::ffi::c_void,
    pub private_cmac_ctx: *mut mbedtls_cmac_context_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct mbedtls_operation_t(pub ::core::ffi::c_int);
impl mbedtls_operation_t {
    pub const MBEDTLS_OPERATION_NONE: Self = Self(-1);
    pub const MBEDTLS_DECRYPT: Self = Self(0);
    pub const MBEDTLS_ENCRYPT: Self = Self(1);
}
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct mbedtls_cipher_info_t {
    pub private_name: *const ::core::ffi::c_char,
    #[bitfield(
        name = "private_block_size",
        ty = "::core::ffi::c_uint",
        bits = "0..=4"
    )]
    #[bitfield(name = "private_iv_size", ty = "::core::ffi::c_uint", bits = "5..=7")]
    #[bitfield(
        name = "private_key_bitlen",
        ty = "::core::ffi::c_uint",
        bits = "8..=11"
    )]
    #[bitfield(name = "private_mode", ty = "::core::ffi::c_uint", bits = "12..=15")]
    #[bitfield(name = "private_type", ty = "::core::ffi::c_uint", bits = "16..=23")]
    #[bitfield(name = "private_flags", ty = "::core::ffi::c_uint", bits = "24..=25")]
    #[bitfield(
        name = "private_base_idx",
        ty = "::core::ffi::c_uint",
        bits = "26..=30"
    )]
    pub private_block_size_private_iv_size_private_key_bitlen_private_mode_private_type_private_flags_private_base_idx:
        [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
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
pub const LPR_CRYPTO_KEY_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const LPR_CRYPTO_NONCE_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const LPR_CRYPTO_TAG_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn toupper(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -128 as ::core::ffi::c_int && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
pub const MBEDTLS_GCM_ENCRYPT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn hex_nibble(mut value: ::core::ffi::c_char) -> ::core::ffi::c_int {
    if value as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
        && value as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
    {
        return value as ::core::ffi::c_int - '0' as ::core::ffi::c_int;
    }
    if value as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
        && value as ::core::ffi::c_int <= 'f' as ::core::ffi::c_int
    {
        return value as ::core::ffi::c_int - 'a' as ::core::ffi::c_int + 10 as ::core::ffi::c_int;
    }
    if value as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
        && value as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
    {
        return value as ::core::ffi::c_int - 'A' as ::core::ffi::c_int + 10 as ::core::ffi::c_int;
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn load_master_key(mut key: *mut uint8_t) -> ::core::ffi::c_int {
    let mut hex: *const ::core::ffi::c_char =
        getenv(b"LIGHTNVR_LPR_MASTER_KEY_HEX\0".as_ptr() as *const ::core::ffi::c_char);
    if hex.is_null() || strlen(hex) != (LPR_CRYPTO_KEY_SIZE * 2 as ::core::ffi::c_int) as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut i: size_t = 0 as size_t;
    while i < LPR_CRYPTO_KEY_SIZE as size_t {
        let mut high: ::core::ffi::c_int =
            hex_nibble(*hex.offset(i.wrapping_mul(2 as size_t) as isize));
        let mut low: ::core::ffi::c_int =
            hex_nibble(*hex.offset(i.wrapping_mul(2 as size_t).wrapping_add(1 as size_t) as isize));
        if high < 0 as ::core::ffi::c_int || low < 0 as ::core::ffi::c_int {
            secure_zero_memory(
                key as *mut ::core::ffi::c_void,
                LPR_CRYPTO_KEY_SIZE as size_t,
            );
            return -1 as ::core::ffi::c_int;
        }
        *key.offset(i as isize) = (high << 4 as ::core::ffi::c_int | low) as uint8_t;
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn hmac_with_domain(
    mut domain: *const ::core::ffi::c_char,
    mut data: *const ::core::ffi::c_void,
    mut data_len: size_t,
    mut output: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut master: [uint8_t; 32] = [0; 32];
    let mut derived: [uint8_t; 32] = [0; 32];
    if load_master_key(&raw mut master as *mut uint8_t) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut info: *const mbedtls_md_info_t =
        mbedtls_md_info_from_type(mbedtls_md_type_t::MBEDTLS_MD_SHA256);
    if info.is_null()
        || mbedtls_md_hmac(
            info,
            &raw mut master as *mut uint8_t as *const ::core::ffi::c_uchar,
            ::core::mem::size_of::<[uint8_t; 32]>(),
            domain as *const ::core::ffi::c_uchar,
            strlen(domain),
            &raw mut derived as *mut ::core::ffi::c_uchar,
        ) != 0 as ::core::ffi::c_int
    {
        secure_zero_memory(
            &raw mut master as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>(),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = mbedtls_md_hmac(
        info,
        &raw mut derived as *mut uint8_t as *const ::core::ffi::c_uchar,
        ::core::mem::size_of::<[uint8_t; 32]>(),
        data as *const ::core::ffi::c_uchar,
        data_len,
        output as *mut ::core::ffi::c_uchar,
    );
    secure_zero_memory(
        &raw mut master as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    secure_zero_memory(
        &raw mut derived as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    return if result == 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn random_bytes(
    mut output: *mut uint8_t,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut offset: size_t = 0 as size_t;
    while offset < length {
        let mut count: ssize_t = getrandom(
            output.offset(offset as isize) as *mut ::core::ffi::c_void,
            length.wrapping_sub(offset),
            0 as ::core::ffi::c_uint,
        );
        if count > 0 as ssize_t {
            offset = offset.wrapping_add(count as size_t);
        } else {
            if count < 0 as ssize_t && *__errno_location() == EINTR {
                continue;
            }
            return -1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lpr_crypto_key_available() -> ::core::ffi::c_int {
    let mut key: [uint8_t; 32] = [0; 32];
    let mut available: ::core::ffi::c_int = (load_master_key(&raw mut key as *mut uint8_t)
        == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    secure_zero_memory(
        &raw mut key as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    return available;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lpr_canonicalize_plate(
    mut plate: *const ::core::ffi::c_char,
    mut canonical: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_int {
    if plate.is_null() || canonical.is_null() || size < 3 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    *canonical.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut output: size_t = 0 as size_t;
    let mut p: *const ::core::ffi::c_uchar = plate as *const ::core::ffi::c_uchar;
    while *p != 0 {
        if *p as ::core::ffi::c_int > 127 as ::core::ffi::c_int {
            secure_zero_memory(canonical as *mut ::core::ffi::c_void, size);
            return -1 as ::core::ffi::c_int;
        }
        if *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            if output.wrapping_add(1 as size_t) >= size {
                secure_zero_memory(canonical as *mut ::core::ffi::c_void, size);
                return -1 as ::core::ffi::c_int;
            }
            let c2rust_fresh0 = output;
            output = output.wrapping_add(1);
            *canonical.offset(c2rust_fresh0 as isize) = ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<::core::ffi::c_uchar>() > 1usize {
                    if 0 as ::core::ffi::c_int != 0 {
                        let mut __c: ::core::ffi::c_int = *p as ::core::ffi::c_int;
                        __res = (if __c < -128 as ::core::ffi::c_int
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = toupper(*p as ::core::ffi::c_int);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(*p as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            }) as ::core::ffi::c_char;
        }
        p = p.offset(1);
    }
    if output < 2 as size_t {
        secure_zero_memory(canonical as *mut ::core::ffi::c_void, size);
        return -1 as ::core::ffi::c_int;
    }
    *canonical.offset(output as isize) = '\0' as ::core::ffi::c_char;
    return output as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lpr_crypto_encrypt(
    mut plaintext: *const ::core::ffi::c_char,
    mut aad: *const ::core::ffi::c_void,
    mut aad_len: size_t,
    mut nonce: *mut uint8_t,
    mut ciphertext: *mut uint8_t,
    mut ciphertext_size: size_t,
    mut ciphertext_len: *mut size_t,
    mut tag: *mut uint8_t,
) -> ::core::ffi::c_int {
    if plaintext.is_null()
        || nonce.is_null()
        || ciphertext.is_null()
        || ciphertext_len.is_null()
        || tag.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut length: size_t = strlen(plaintext);
    if length == 0 as size_t || length > ciphertext_size {
        return -1 as ::core::ffi::c_int;
    }
    let mut key: [uint8_t; 32] = [0; 32];
    if load_master_key(&raw mut key as *mut uint8_t) != 0 as ::core::ffi::c_int
        || random_bytes(nonce, LPR_CRYPTO_NONCE_SIZE as size_t) != 0 as ::core::ffi::c_int
    {
        secure_zero_memory(
            &raw mut key as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>(),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut context: mbedtls_gcm_context = mbedtls_gcm_context {
        private_cipher_ctx: mbedtls_cipher_context_t {
            private_cipher_info: ::core::ptr::null::<mbedtls_cipher_info_t>(),
            private_key_bitlen: 0,
            private_operation: mbedtls_operation_t::MBEDTLS_DECRYPT,
            private_add_padding: None,
            private_get_padding: None,
            private_unprocessed_data: [0; 16],
            private_unprocessed_len: 0,
            private_iv: [0; 16],
            private_iv_size: 0,
            private_cipher_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            private_cmac_ctx: ::core::ptr::null_mut::<mbedtls_cmac_context_t>(),
        },
        private_H: [[0; 2]; 16],
        private_len: 0,
        private_add_len: 0,
        private_base_ectr: [0; 16],
        private_y: [0; 16],
        private_buf: [0; 16],
        private_mode: 0,
        private_acceleration: 0,
    };
    mbedtls_gcm_init(&raw mut context);
    let mut result: ::core::ffi::c_int = mbedtls_gcm_setkey(
        &raw mut context,
        mbedtls_cipher_id_t::MBEDTLS_CIPHER_ID_AES,
        &raw mut key as *mut uint8_t as *const ::core::ffi::c_uchar,
        (LPR_CRYPTO_KEY_SIZE * 8 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    if result == 0 as ::core::ffi::c_int {
        result = mbedtls_gcm_crypt_and_tag(
            &raw mut context,
            MBEDTLS_GCM_ENCRYPT,
            length,
            nonce as *const ::core::ffi::c_uchar,
            LPR_CRYPTO_NONCE_SIZE as size_t,
            aad as *const ::core::ffi::c_uchar,
            aad_len,
            plaintext as *const ::core::ffi::c_uchar,
            ciphertext as *mut ::core::ffi::c_uchar,
            LPR_CRYPTO_TAG_SIZE as size_t,
            tag as *mut ::core::ffi::c_uchar,
        );
    }
    mbedtls_gcm_free(&raw mut context);
    secure_zero_memory(
        &raw mut key as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    if result != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    *ciphertext_len = length;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lpr_crypto_decrypt(
    mut ciphertext: *const uint8_t,
    mut ciphertext_len: size_t,
    mut aad: *const ::core::ffi::c_void,
    mut aad_len: size_t,
    mut nonce: *const uint8_t,
    mut tag: *const uint8_t,
    mut plaintext: *mut ::core::ffi::c_char,
    mut plaintext_size: size_t,
) -> ::core::ffi::c_int {
    if ciphertext.is_null()
        || nonce.is_null()
        || tag.is_null()
        || plaintext.is_null()
        || ciphertext_len == 0 as size_t
        || ciphertext_len >= plaintext_size
    {
        return -1 as ::core::ffi::c_int;
    }
    secure_zero_memory(plaintext as *mut ::core::ffi::c_void, plaintext_size);
    let mut key: [uint8_t; 32] = [0; 32];
    if load_master_key(&raw mut key as *mut uint8_t) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut context: mbedtls_gcm_context = mbedtls_gcm_context {
        private_cipher_ctx: mbedtls_cipher_context_t {
            private_cipher_info: ::core::ptr::null::<mbedtls_cipher_info_t>(),
            private_key_bitlen: 0,
            private_operation: mbedtls_operation_t::MBEDTLS_DECRYPT,
            private_add_padding: None,
            private_get_padding: None,
            private_unprocessed_data: [0; 16],
            private_unprocessed_len: 0,
            private_iv: [0; 16],
            private_iv_size: 0,
            private_cipher_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            private_cmac_ctx: ::core::ptr::null_mut::<mbedtls_cmac_context_t>(),
        },
        private_H: [[0; 2]; 16],
        private_len: 0,
        private_add_len: 0,
        private_base_ectr: [0; 16],
        private_y: [0; 16],
        private_buf: [0; 16],
        private_mode: 0,
        private_acceleration: 0,
    };
    mbedtls_gcm_init(&raw mut context);
    let mut result: ::core::ffi::c_int = mbedtls_gcm_setkey(
        &raw mut context,
        mbedtls_cipher_id_t::MBEDTLS_CIPHER_ID_AES,
        &raw mut key as *mut uint8_t as *const ::core::ffi::c_uchar,
        (LPR_CRYPTO_KEY_SIZE * 8 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    if result == 0 as ::core::ffi::c_int {
        result = mbedtls_gcm_auth_decrypt(
            &raw mut context,
            ciphertext_len,
            nonce as *const ::core::ffi::c_uchar,
            LPR_CRYPTO_NONCE_SIZE as size_t,
            aad as *const ::core::ffi::c_uchar,
            aad_len,
            tag as *const ::core::ffi::c_uchar,
            LPR_CRYPTO_TAG_SIZE as size_t,
            ciphertext as *const ::core::ffi::c_uchar,
            plaintext as *mut ::core::ffi::c_uchar,
        );
    }
    mbedtls_gcm_free(&raw mut context);
    secure_zero_memory(
        &raw mut key as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    if result != 0 as ::core::ffi::c_int {
        secure_zero_memory(plaintext as *mut ::core::ffi::c_void, plaintext_size);
        return -1 as ::core::ffi::c_int;
    }
    *plaintext.offset(ciphertext_len as isize) = '\0' as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lpr_crypto_blind_index(
    mut canonical_plate: *const ::core::ffi::c_char,
    mut output: *mut uint8_t,
) -> ::core::ffi::c_int {
    if canonical_plate.is_null() || *canonical_plate.offset(0isize) == 0 || output.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    return hmac_with_domain(
        b"lightnvr:lpr:exact:v1\0".as_ptr() as *const ::core::ffi::c_char,
        canonical_plate as *const ::core::ffi::c_void,
        strlen(canonical_plate),
        output,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lpr_crypto_fingerprint(
    mut data: *const ::core::ffi::c_void,
    mut data_len: size_t,
    mut output: *mut uint8_t,
) -> ::core::ffi::c_int {
    if data.is_null() && data_len > 0 as size_t || output.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    return hmac_with_domain(
        b"lightnvr:lpr:fingerprint:v1\0".as_ptr() as *const ::core::ffi::c_char,
        data,
        data_len,
        output,
    );
}
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
