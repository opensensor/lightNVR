use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn sqlite3_changes(_: *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_blob(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        n: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_double(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_int(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_int64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_null(_: *mut sqlite3_stmt, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_blob(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_void;
    unsafe fn sqlite3_column_double(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    unsafe fn sqlite3_column_int64(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int)
    -> sqlite3_int64;
    unsafe fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    unsafe fn sqlite3_column_bytes(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_type(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn lpr_crypto_key_available() -> ::core::ffi::c_int;
    unsafe fn lpr_canonicalize_plate(
        plate: *const ::core::ffi::c_char,
        canonical: *mut ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lpr_crypto_encrypt(
        plaintext: *const ::core::ffi::c_char,
        aad: *const ::core::ffi::c_void,
        aad_len: size_t,
        nonce: *mut uint8_t,
        ciphertext: *mut uint8_t,
        ciphertext_size: size_t,
        ciphertext_len: *mut size_t,
        tag: *mut uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lpr_crypto_decrypt(
        ciphertext: *const uint8_t,
        ciphertext_len: size_t,
        aad: *const ::core::ffi::c_void,
        aad_len: size_t,
        nonce: *const uint8_t,
        tag: *const uint8_t,
        plaintext: *mut ::core::ffi::c_char,
        plaintext_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lpr_crypto_blind_index(
        canonical_plate: *const ::core::ffi::c_char,
        output: *mut uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lpr_crypto_fingerprint(
        data: *const ::core::ffi::c_void,
        data_len: size_t,
        output: *mut uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn secure_zero_memory(ptr: *mut ::core::ffi::c_void, size: size_t);
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type uint8_t = u8;
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct onvif_lpr_source_t(pub ::core::ffi::c_uint);
impl onvif_lpr_source_t {
    pub const ONVIF_LPR_SOURCE_PROFILE_M: Self = Self(0);
    pub const ONVIF_LPR_SOURCE_VENDOR_TOPIC: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_lpr_event_t {
    pub source: onvif_lpr_source_t,
    pub asserted: bool,
    pub observed_at_ms: int64_t,
    pub topic: [::core::ffi::c_char; 256],
    pub plate: [::core::ffi::c_char; 64],
    pub has_confidence: bool,
    pub confidence: ::core::ffi::c_float,
    pub country: [::core::ffi::c_char; 64],
    pub region: [::core::ffi::c_char; 64],
    pub plate_type: [::core::ffi::c_char; 64],
    pub direction: [::core::ffi::c_char; 64],
    pub lane: [::core::ffi::c_char; 64],
    pub vehicle_type: [::core::ffi::c_char; 64],
    pub vehicle_color: [::core::ffi::c_char; 64],
    pub object_id: [::core::ffi::c_char; 96],
    pub correlation_id: [::core::ffi::c_char; 96],
    pub has_bounding_box: bool,
    pub bbox_left: ::core::ffi::c_float,
    pub bbox_top: ::core::ffi::c_float,
    pub bbox_right: ::core::ffi::c_float,
    pub bbox_bottom: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lpr_read_input_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub stream_name: [::core::ffi::c_char; 256],
    pub observed_at_ms: int64_t,
    pub source: [::core::ffi::c_char; 32],
    pub vendor_topic: [::core::ffi::c_char; 256],
    pub plate: [::core::ffi::c_char; 64],
    pub has_confidence: bool,
    pub confidence: ::core::ffi::c_float,
    pub country: [::core::ffi::c_char; 64],
    pub region: [::core::ffi::c_char; 64],
    pub plate_type: [::core::ffi::c_char; 64],
    pub direction: [::core::ffi::c_char; 64],
    pub lane: [::core::ffi::c_char; 64],
    pub vehicle_type: [::core::ffi::c_char; 64],
    pub vehicle_color: [::core::ffi::c_char; 64],
    pub object_id: [::core::ffi::c_char; 96],
    pub correlation_id: [::core::ffi::c_char; 96],
    pub has_bounding_box: bool,
    pub bbox_left: ::core::ffi::c_float,
    pub bbox_top: ::core::ffi::c_float,
    pub bbox_right: ::core::ffi::c_float,
    pub bbox_bottom: ::core::ffi::c_float,
    pub recording_id: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lpr_read_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub read: lpr_read_input_t,
    pub received_at_ms: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct lpr_match_mode_t(pub ::core::ffi::c_uint);
impl lpr_match_mode_t {
    pub const LPR_MATCH_NONE: Self = Self(0);
    pub const LPR_MATCH_EXACT: Self = Self(1);
    pub const LPR_MATCH_PARTIAL: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lpr_read_query_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub start_at_ms: int64_t,
    pub end_at_ms: int64_t,
    pub match_mode: lpr_match_mode_t,
    pub plate_query: [::core::ffi::c_char; 64],
    pub limit: ::core::ffi::c_int,
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
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const LPR_READ_UUID_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const LPR_CRYPTO_NONCE_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const LPR_CRYPTO_TAG_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LPR_DB_CANDIDATE_LIMIT: ::core::ffi::c_int = 5000 as ::core::ffi::c_int;
pub const LPR_DB_RESULT_LIMIT: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const LPR_DEDUPE_WINDOW_MS: ::core::ffi::c_int = 5000 as ::core::ffi::c_int;
unsafe extern "C" fn current_time_ms() -> int64_t {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_REALTIME, &raw mut now) != 0 as ::core::ffi::c_int {
        return time(::core::ptr::null_mut::<time_t>()) as int64_t * 1000 as int64_t;
    }
    return now.tv_sec as int64_t * 1000 as int64_t + now.tv_nsec as int64_t / 1000000 as int64_t;
}
unsafe extern "C" fn valid_source(mut source: *const ::core::ffi::c_char) -> bool {
    static mut sources: [*const ::core::ffi::c_char; 4] = [
        b"onvif_profile_m\0".as_ptr() as *const ::core::ffi::c_char,
        b"onvif_vendor\0".as_ptr() as *const ::core::ffi::c_char,
        b"vendor_api\0".as_ptr() as *const ::core::ffi::c_char,
        b"metadata_track\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        if !source.is_null() && strcmp(source, sources[i]) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn terminated_text(
    mut value: *const ::core::ffi::c_char,
    mut size: size_t,
) -> bool {
    return !value.is_null()
        && !memchr(
            value as *const ::core::ffi::c_void,
            '\0' as ::core::ffi::c_int,
            size,
        )
        .is_null();
}
unsafe extern "C" fn contains_case_insensitive(
    mut value: *const ::core::ffi::c_char,
    mut needle: *const ::core::ffi::c_char,
) -> bool {
    if value.is_null() || needle.is_null() || *needle.offset(0isize) == 0 {
        return r#false != 0;
    }
    let mut length: size_t = strlen(needle);
    let mut cursor: *const ::core::ffi::c_char = value;
    while *cursor != 0 {
        if strncasecmp(cursor, needle, length) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn make_aad(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut observed_at_ms: int64_t,
    mut source: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) -> ::core::ffi::c_int {
    let mut written: ::core::ffi::c_int = snprintf(
        output,
        output_size,
        b"%s|%lld|%s\0".as_ptr() as *const ::core::ffi::c_char,
        camera_uuid,
        observed_at_ms as ::core::ffi::c_longlong,
        source,
    );
    return if written > 0 as ::core::ffi::c_int && (written as size_t) < output_size {
        written
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn make_dedupe(
    mut input: *const lpr_read_input_t,
    mut canonical: *const ::core::ffi::c_char,
    mut output: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut material: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut material as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"%s|%s|%s|%s|%s|%s|%lld\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*input).camera_uuid as *const ::core::ffi::c_char,
        canonical,
        &raw const (*input).source as *const ::core::ffi::c_char,
        &raw const (*input).vendor_topic as *const ::core::ffi::c_char,
        &raw const (*input).object_id as *const ::core::ffi::c_char,
        &raw const (*input).correlation_id as *const ::core::ffi::c_char,
        ((*input).observed_at_ms / LPR_DEDUPE_WINDOW_MS as int64_t) as ::core::ffi::c_longlong,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
    {
        return -1 as ::core::ffi::c_int;
    }
    return lpr_crypto_fingerprint(
        &raw mut material as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        written as size_t,
        output,
    );
}
unsafe extern "C" fn bind_nullable_text(
    mut stmt: *mut sqlite3_stmt,
    mut index: ::core::ffi::c_int,
    mut value: *const ::core::ffi::c_char,
) {
    if !value.is_null() && *value.offset(0isize) as ::core::ffi::c_int != 0 {
        sqlite3_bind_text(
            stmt,
            index,
            value,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    } else {
        sqlite3_bind_null(stmt, index);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_lpr_read_insert(
    mut input: *const lpr_read_input_t,
    mut uuid_out: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !uuid_out.is_null() {
        *uuid_out.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if input.is_null()
        || !terminated_text(
            &raw const (*input).camera_uuid as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        )
        || !lightnvr_uuid_is_valid(&raw const (*input).camera_uuid as *const ::core::ffi::c_char)
        || !terminated_text(
            &raw const (*input).stream_name as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        )
        || (*input).stream_name[0usize] == 0
        || !terminated_text(
            &raw const (*input).source as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        )
        || !terminated_text(
            &raw const (*input).plate as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        )
        || !terminated_text(
            &raw const (*input).vendor_topic as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        )
        || !terminated_text(
            &raw const (*input).country as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        )
        || !terminated_text(
            &raw const (*input).region as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        )
        || !terminated_text(
            &raw const (*input).plate_type as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        )
        || !terminated_text(
            &raw const (*input).direction as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        )
        || !terminated_text(
            &raw const (*input).lane as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        )
        || !terminated_text(
            &raw const (*input).vehicle_type as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        )
        || !terminated_text(
            &raw const (*input).vehicle_color as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        )
        || !terminated_text(
            &raw const (*input).object_id as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        )
        || !terminated_text(
            &raw const (*input).correlation_id as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        )
        || !valid_source(&raw const (*input).source as *const ::core::ffi::c_char)
        || (*input).observed_at_ms <= 0 as int64_t
        || (*input).has_confidence as ::core::ffi::c_int != 0
            && ((*input).confidence.is_finite() as ::core::ffi::c_int == 0
                || (*input).confidence < 0.0f32
                || (*input).confidence > 1.0f32)
        || (*input).has_bounding_box as ::core::ffi::c_int != 0
            && ((*input).bbox_left.is_finite() as ::core::ffi::c_int == 0
                || (*input).bbox_top.is_finite() as ::core::ffi::c_int == 0
                || (*input).bbox_right.is_finite() as ::core::ffi::c_int == 0
                || (*input).bbox_bottom.is_finite() as ::core::ffi::c_int == 0
                || (*input).bbox_left < -1.0f32
                || (*input).bbox_left > 1.0f32
                || (*input).bbox_top < -1.0f32
                || (*input).bbox_top > 1.0f32
                || (*input).bbox_right < -1.0f32
                || (*input).bbox_right > 1.0f32
                || (*input).bbox_bottom < -1.0f32
                || (*input).bbox_bottom > 1.0f32
                || (*input).bbox_right < (*input).bbox_left
                || (*input).bbox_bottom < (*input).bbox_top)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut canonical: [::core::ffi::c_char; 64] = [0; 64];
    if lpr_canonicalize_plate(
        &raw const (*input).plate as *const ::core::ffi::c_char,
        &raw mut canonical as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) < 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut aad: [::core::ffi::c_char; 256] = [0; 256];
    let mut aad_len: ::core::ffi::c_int = make_aad(
        &raw const (*input).camera_uuid as *const ::core::ffi::c_char,
        (*input).observed_at_ms,
        &raw const (*input).source as *const ::core::ffi::c_char,
        &raw mut aad as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if aad_len < 0 as ::core::ffi::c_int {
        secure_zero_memory(
            &raw mut canonical as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut nonce: [uint8_t; 12] = [0 as uint8_t, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut ciphertext: [uint8_t; 64] = [
        0 as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut tag: [uint8_t; 16] = [0 as uint8_t, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut exact_hmac: [uint8_t; 32] = [
        0 as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut dedupe_hmac: [uint8_t; 32] = [
        0 as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut ciphertext_len: size_t = 0 as size_t;
    if lpr_crypto_encrypt(
        &raw mut canonical as *mut ::core::ffi::c_char,
        &raw mut aad as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        aad_len as size_t,
        &raw mut nonce as *mut uint8_t,
        &raw mut ciphertext as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 64]>(),
        &raw mut ciphertext_len,
        &raw mut tag as *mut uint8_t,
    ) != 0 as ::core::ffi::c_int
        || lpr_crypto_blind_index(
            &raw mut canonical as *mut ::core::ffi::c_char,
            &raw mut exact_hmac as *mut uint8_t,
        ) != 0 as ::core::ffi::c_int
        || make_dedupe(
            input,
            &raw mut canonical as *mut ::core::ffi::c_char,
            &raw mut dedupe_hmac as *mut uint8_t,
        ) != 0 as ::core::ffi::c_int
    {
        secure_zero_memory(
            &raw mut canonical as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        );
        secure_zero_memory(
            &raw mut nonce as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 12]>(),
        );
        secure_zero_memory(
            &raw mut ciphertext as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 64]>(),
        );
        secure_zero_memory(
            &raw mut tag as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 16]>(),
        );
        secure_zero_memory(
            &raw mut exact_hmac as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>(),
        );
        secure_zero_memory(
            &raw mut dedupe_hmac as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>(),
        );
        return -1 as ::core::ffi::c_int;
    }
    secure_zero_memory(
        &raw mut canonical as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if lightnvr_uuid_generate_v4(&raw mut uuid as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        secure_zero_memory(
            &raw mut nonce as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 12]>(),
        );
        secure_zero_memory(
            &raw mut ciphertext as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 64]>(),
        );
        secure_zero_memory(
            &raw mut tag as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 16]>(),
        );
        secure_zero_memory(
            &raw mut exact_hmac as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>(),
        );
        secure_zero_memory(
            &raw mut dedupe_hmac as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>(),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut received_at_ms: int64_t = current_time_ms();
    let mut sql: *const ::core::ffi::c_char = b"INSERT OR IGNORE INTO lpr_reads (uuid,camera_uuid,stream_name,observed_at_ms,received_at_ms,source,vendor_topic,plate_nonce,plate_ciphertext,plate_tag,plate_exact_hmac,dedupe_hmac,confidence,country,region,plate_type,direction,lane,vehicle_type,vehicle_color,object_id,correlation_id,bbox_left,bbox_top,bbox_right,bbox_bottom,recording_id) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        secure_zero_memory(
            &raw mut nonce as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 12]>(),
        );
        secure_zero_memory(
            &raw mut ciphertext as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 64]>(),
        );
        secure_zero_memory(
            &raw mut tag as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 16]>(),
        );
        secure_zero_memory(
            &raw mut exact_hmac as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>(),
        );
        secure_zero_memory(
            &raw mut dedupe_hmac as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>(),
        );
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw mut uuid as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw const (*input).camera_uuid as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            &raw const (*input).stream_name as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            stmt,
            4 as ::core::ffi::c_int,
            (*input).observed_at_ms as sqlite3_int64,
        );
        sqlite3_bind_int64(
            stmt,
            5 as ::core::ffi::c_int,
            received_at_ms as sqlite3_int64,
        );
        sqlite3_bind_text(
            stmt,
            6 as ::core::ffi::c_int,
            &raw const (*input).source as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            7 as ::core::ffi::c_int,
            &raw const (*input).vendor_topic as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_blob(
            stmt,
            8 as ::core::ffi::c_int,
            &raw mut nonce as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 12]>() as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_blob(
            stmt,
            9 as ::core::ffi::c_int,
            &raw mut ciphertext as *mut uint8_t as *const ::core::ffi::c_void,
            ciphertext_len as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_blob(
            stmt,
            10 as ::core::ffi::c_int,
            &raw mut tag as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 16]>() as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_blob(
            stmt,
            11 as ::core::ffi::c_int,
            &raw mut exact_hmac as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>() as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_blob(
            stmt,
            12 as ::core::ffi::c_int,
            &raw mut dedupe_hmac as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>() as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if (*input).has_confidence {
            sqlite3_bind_double(
                stmt,
                13 as ::core::ffi::c_int,
                (*input).confidence as ::core::ffi::c_double,
            );
        } else {
            sqlite3_bind_null(stmt, 13 as ::core::ffi::c_int);
        }
        bind_nullable_text(
            stmt,
            14 as ::core::ffi::c_int,
            &raw const (*input).country as *const ::core::ffi::c_char,
        );
        bind_nullable_text(
            stmt,
            15 as ::core::ffi::c_int,
            &raw const (*input).region as *const ::core::ffi::c_char,
        );
        bind_nullable_text(
            stmt,
            16 as ::core::ffi::c_int,
            &raw const (*input).plate_type as *const ::core::ffi::c_char,
        );
        bind_nullable_text(
            stmt,
            17 as ::core::ffi::c_int,
            &raw const (*input).direction as *const ::core::ffi::c_char,
        );
        bind_nullable_text(
            stmt,
            18 as ::core::ffi::c_int,
            &raw const (*input).lane as *const ::core::ffi::c_char,
        );
        bind_nullable_text(
            stmt,
            19 as ::core::ffi::c_int,
            &raw const (*input).vehicle_type as *const ::core::ffi::c_char,
        );
        bind_nullable_text(
            stmt,
            20 as ::core::ffi::c_int,
            &raw const (*input).vehicle_color as *const ::core::ffi::c_char,
        );
        bind_nullable_text(
            stmt,
            21 as ::core::ffi::c_int,
            &raw const (*input).object_id as *const ::core::ffi::c_char,
        );
        bind_nullable_text(
            stmt,
            22 as ::core::ffi::c_int,
            &raw const (*input).correlation_id as *const ::core::ffi::c_char,
        );
        if (*input).has_bounding_box {
            sqlite3_bind_double(
                stmt,
                23 as ::core::ffi::c_int,
                (*input).bbox_left as ::core::ffi::c_double,
            );
            sqlite3_bind_double(
                stmt,
                24 as ::core::ffi::c_int,
                (*input).bbox_top as ::core::ffi::c_double,
            );
            sqlite3_bind_double(
                stmt,
                25 as ::core::ffi::c_int,
                (*input).bbox_right as ::core::ffi::c_double,
            );
            sqlite3_bind_double(
                stmt,
                26 as ::core::ffi::c_int,
                (*input).bbox_bottom as ::core::ffi::c_double,
            );
        } else {
            let mut i: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
            while i <= 26 as ::core::ffi::c_int {
                sqlite3_bind_null(stmt, i);
                i += 1;
            }
        }
        if (*input).recording_id != 0 {
            sqlite3_bind_int64(
                stmt,
                27 as ::core::ffi::c_int,
                (*input).recording_id as sqlite3_int64,
            );
        } else {
            sqlite3_bind_null(stmt, 27 as ::core::ffi::c_int);
        }
        rc = sqlite3_step(stmt);
    }
    let mut changed: ::core::ffi::c_int = sqlite3_changes(db);
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    secure_zero_memory(
        &raw mut nonce as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 12]>(),
    );
    secure_zero_memory(
        &raw mut ciphertext as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>(),
    );
    secure_zero_memory(
        &raw mut tag as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>(),
    );
    secure_zero_memory(
        &raw mut exact_hmac as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    secure_zero_memory(
        &raw mut dedupe_hmac as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    if rc != SQLITE_DONE {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to store protected LPR read: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    if changed == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if !uuid_out.is_null() {
        snprintf(
            uuid_out,
            LPR_READ_UUID_SIZE as size_t,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut uuid as *mut ::core::ffi::c_char,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn copy_sql_text(
    mut output: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut stmt: *mut sqlite3_stmt,
    mut column: ::core::ffi::c_int,
) {
    let mut value: *const ::core::ffi::c_uchar = sqlite3_column_text(stmt, column);
    snprintf(
        output,
        size,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        if !value.is_null() {
            value as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
}
unsafe extern "C" fn decode_row(
    mut stmt: *mut sqlite3_stmt,
    mut read: *mut lpr_read_t,
) -> ::core::ffi::c_int {
    memset(
        read as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<lpr_read_t>(),
    );
    copy_sql_text(
        &raw mut (*read).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        stmt,
        0 as ::core::ffi::c_int,
    );
    copy_sql_text(
        &raw mut (*read).read.camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        stmt,
        1 as ::core::ffi::c_int,
    );
    copy_sql_text(
        &raw mut (*read).read.stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        stmt,
        2 as ::core::ffi::c_int,
    );
    (*read).read.observed_at_ms = sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as int64_t;
    (*read).received_at_ms = sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as int64_t;
    copy_sql_text(
        &raw mut (*read).read.source as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        stmt,
        5 as ::core::ffi::c_int,
    );
    copy_sql_text(
        &raw mut (*read).read.vendor_topic as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        stmt,
        6 as ::core::ffi::c_int,
    );
    let mut nonce: *const uint8_t =
        sqlite3_column_blob(stmt, 7 as ::core::ffi::c_int) as *const uint8_t;
    let mut nonce_len: ::core::ffi::c_int = sqlite3_column_bytes(stmt, 7 as ::core::ffi::c_int);
    let mut ciphertext: *const uint8_t =
        sqlite3_column_blob(stmt, 8 as ::core::ffi::c_int) as *const uint8_t;
    let mut ciphertext_len: ::core::ffi::c_int =
        sqlite3_column_bytes(stmt, 8 as ::core::ffi::c_int);
    let mut tag: *const uint8_t =
        sqlite3_column_blob(stmt, 9 as ::core::ffi::c_int) as *const uint8_t;
    let mut tag_len: ::core::ffi::c_int = sqlite3_column_bytes(stmt, 9 as ::core::ffi::c_int);
    if nonce.is_null()
        || nonce_len != LPR_CRYPTO_NONCE_SIZE
        || ciphertext.is_null()
        || ciphertext_len <= 0 as ::core::ffi::c_int
        || tag.is_null()
        || tag_len != LPR_CRYPTO_TAG_SIZE
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut aad: [::core::ffi::c_char; 256] = [0; 256];
    let mut aad_len: ::core::ffi::c_int = make_aad(
        &raw mut (*read).read.camera_uuid as *mut ::core::ffi::c_char,
        (*read).read.observed_at_ms,
        &raw mut (*read).read.source as *mut ::core::ffi::c_char,
        &raw mut aad as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if aad_len < 0 as ::core::ffi::c_int
        || lpr_crypto_decrypt(
            ciphertext,
            ciphertext_len as size_t,
            &raw mut aad as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            aad_len as size_t,
            nonce,
            tag,
            &raw mut (*read).read.plate as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    (*read).read.has_confidence =
        sqlite3_column_type(stmt, 10 as ::core::ffi::c_int) != SQLITE_NULL;
    if (*read).read.has_confidence {
        (*read).read.confidence =
            sqlite3_column_double(stmt, 10 as ::core::ffi::c_int) as ::core::ffi::c_float;
    }
    copy_sql_text(
        &raw mut (*read).read.country as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        stmt,
        11 as ::core::ffi::c_int,
    );
    copy_sql_text(
        &raw mut (*read).read.region as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        stmt,
        12 as ::core::ffi::c_int,
    );
    copy_sql_text(
        &raw mut (*read).read.plate_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        stmt,
        13 as ::core::ffi::c_int,
    );
    copy_sql_text(
        &raw mut (*read).read.direction as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        stmt,
        14 as ::core::ffi::c_int,
    );
    copy_sql_text(
        &raw mut (*read).read.lane as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        stmt,
        15 as ::core::ffi::c_int,
    );
    copy_sql_text(
        &raw mut (*read).read.vehicle_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        stmt,
        16 as ::core::ffi::c_int,
    );
    copy_sql_text(
        &raw mut (*read).read.vehicle_color as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        stmt,
        17 as ::core::ffi::c_int,
    );
    copy_sql_text(
        &raw mut (*read).read.object_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        stmt,
        18 as ::core::ffi::c_int,
    );
    copy_sql_text(
        &raw mut (*read).read.correlation_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        stmt,
        19 as ::core::ffi::c_int,
    );
    (*read).read.has_bounding_box =
        sqlite3_column_type(stmt, 20 as ::core::ffi::c_int) != SQLITE_NULL;
    if (*read).read.has_bounding_box {
        (*read).read.bbox_left =
            sqlite3_column_double(stmt, 20 as ::core::ffi::c_int) as ::core::ffi::c_float;
        (*read).read.bbox_top =
            sqlite3_column_double(stmt, 21 as ::core::ffi::c_int) as ::core::ffi::c_float;
        (*read).read.bbox_right =
            sqlite3_column_double(stmt, 22 as ::core::ffi::c_int) as ::core::ffi::c_float;
        (*read).read.bbox_bottom =
            sqlite3_column_double(stmt, 23 as ::core::ffi::c_int) as ::core::ffi::c_float;
    }
    if sqlite3_column_type(stmt, 24 as ::core::ffi::c_int) != SQLITE_NULL {
        (*read).read.recording_id =
            sqlite3_column_int64(stmt, 24 as ::core::ffi::c_int) as uint64_t;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_lpr_reads_search(
    mut query: *const lpr_read_query_t,
    mut reads: *mut lpr_read_t,
    mut capacity: size_t,
) -> ::core::ffi::c_int {
    if query.is_null()
        || reads.is_null()
        || capacity == 0 as size_t
        || (*query).start_at_ms <= 0 as int64_t
        || (*query).end_at_ms < (*query).start_at_ms
        || (*query).limit < 1 as ::core::ffi::c_int
        || (*query).limit > LPR_DB_RESULT_LIMIT
        || (*query).camera_uuid[0usize] as ::core::ffi::c_int != 0
            && !lightnvr_uuid_is_valid(
                &raw const (*query).camera_uuid as *const ::core::ffi::c_char,
            )
        || (*query).match_mode.0 < lpr_match_mode_t::LPR_MATCH_NONE.0
        || (*query).match_mode.0 > lpr_match_mode_t::LPR_MATCH_PARTIAL.0
        || lpr_crypto_key_available() == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut canonical: [::core::ffi::c_char; 64] = [
        0 as ::core::ffi::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut exact_hmac: [uint8_t; 32] = [0; 32];
    if (*query).match_mode.0 != lpr_match_mode_t::LPR_MATCH_NONE.0 {
        let mut length: ::core::ffi::c_int = lpr_canonicalize_plate(
            &raw const (*query).plate_query as *const ::core::ffi::c_char,
            &raw mut canonical as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        );
        if length < 0 as ::core::ffi::c_int
            || (*query).match_mode.0 == lpr_match_mode_t::LPR_MATCH_PARTIAL.0
                && length < 3 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if (*query).match_mode.0 == lpr_match_mode_t::LPR_MATCH_EXACT.0
            && lpr_crypto_blind_index(
                &raw mut canonical as *mut ::core::ffi::c_char,
                &raw mut exact_hmac as *mut uint8_t,
            ) != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
    }
    let mut select: *const ::core::ffi::c_char = b"SELECT uuid,camera_uuid,stream_name,observed_at_ms,received_at_ms,source,vendor_topic,plate_nonce,plate_ciphertext,plate_tag,confidence,country,region,plate_type,direction,lane,vehicle_type,vehicle_color,object_id,correlation_id,bbox_left,bbox_top,bbox_right,bbox_bottom,recording_id FROM lpr_reads WHERE observed_at_ms >= ? AND observed_at_ms <= ? \0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut sql: [::core::ffi::c_char; 1024] = [0; 1024];
    snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"%s%s%sORDER BY observed_at_ms DESC LIMIT ?;\0".as_ptr() as *const ::core::ffi::c_char,
        select,
        if (*query).camera_uuid[0usize] as ::core::ffi::c_int != 0 {
            b"AND camera_uuid = ? \0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        if (*query).match_mode.0 == lpr_match_mode_t::LPR_MATCH_EXACT.0 {
            b"AND plate_exact_hmac = ? \0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut bind: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if rc == SQLITE_OK {
        let c2rust_fresh0 = bind;
        bind += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh0, (*query).start_at_ms as sqlite3_int64);
        let c2rust_fresh1 = bind;
        bind += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh1, (*query).end_at_ms as sqlite3_int64);
        if (*query).camera_uuid[0usize] != 0 {
            let c2rust_fresh2 = bind;
            bind += 1;
            sqlite3_bind_text(
                stmt,
                c2rust_fresh2,
                &raw const (*query).camera_uuid as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        if (*query).match_mode.0 == lpr_match_mode_t::LPR_MATCH_EXACT.0 {
            let c2rust_fresh3 = bind;
            bind += 1;
            sqlite3_bind_blob(
                stmt,
                c2rust_fresh3,
                &raw mut exact_hmac as *mut uint8_t as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[uint8_t; 32]>() as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        let mut candidate_limit: ::core::ffi::c_int =
            if (*query).match_mode.0 == lpr_match_mode_t::LPR_MATCH_PARTIAL.0 {
                LPR_DB_CANDIDATE_LIMIT
            } else {
                (*query).limit
            };
        sqlite3_bind_int(stmt, bind, candidate_limit);
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (rc == SQLITE_OK || rc == SQLITE_ROW) && {
        rc = sqlite3_step(stmt);
        rc == SQLITE_ROW
    } {
        let mut decoded: lpr_read_t = lpr_read_t {
            uuid: [0; 37],
            read: lpr_read_input_t {
                camera_uuid: [0; 37],
                stream_name: [0; 256],
                observed_at_ms: 0,
                source: [0; 32],
                vendor_topic: [0; 256],
                plate: [0; 64],
                has_confidence: false,
                confidence: 0.,
                country: [0; 64],
                region: [0; 64],
                plate_type: [0; 64],
                direction: [0; 64],
                lane: [0; 64],
                vehicle_type: [0; 64],
                vehicle_color: [0; 64],
                object_id: [0; 96],
                correlation_id: [0; 96],
                has_bounding_box: false,
                bbox_left: 0.,
                bbox_top: 0.,
                bbox_right: 0.,
                bbox_bottom: 0.,
                recording_id: 0,
            },
            received_at_ms: 0,
        };
        if decode_row(stmt, &raw mut decoded) != 0 as ::core::ffi::c_int {
            memset(
                &raw mut decoded as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<lpr_read_t>(),
            );
            rc = SQLITE_ERROR;
            break;
        } else {
            let mut r#match: bool = (*query).match_mode.0 != lpr_match_mode_t::LPR_MATCH_PARTIAL.0
                || !strstr(
                    &raw mut decoded.read.plate as *mut ::core::ffi::c_char,
                    &raw mut canonical as *mut ::core::ffi::c_char,
                )
                .is_null();
            if r#match as ::core::ffi::c_int != 0
                && (count as size_t) < capacity
                && count < (*query).limit
            {
                let c2rust_fresh4 = count;
                count += 1;
                *reads.offset(c2rust_fresh4 as isize) = decoded;
            }
            memset(
                &raw mut decoded as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<lpr_read_t>(),
            );
            if count >= (*query).limit {
                break;
            }
        }
    }
    let mut success: bool = rc == SQLITE_DONE || rc == SQLITE_ROW || count >= (*query).limit;
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    memset(
        &raw mut canonical as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    return if success as ::core::ffi::c_int != 0 {
        count
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_lpr_read_delete(
    mut uuid: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !lightnvr_uuid_is_valid(uuid) {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"DELETE FROM lpr_reads WHERE uuid = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
    }
    let mut changed: ::core::ffi::c_int = sqlite3_changes(db);
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return if rc == SQLITE_DONE && changed == 1 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_lpr_read_get_camera_uuid(
    mut uuid: *const ::core::ffi::c_char,
    mut camera_uuid: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !lightnvr_uuid_is_valid(uuid) || camera_uuid.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *camera_uuid.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT camera_uuid FROM lpr_reads WHERE uuid=?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
        if rc == SQLITE_ROW {
            copy_sql_text(
                camera_uuid,
                CAMERA_UUID_STRING_SIZE as size_t,
                stmt,
                0 as ::core::ffi::c_int,
            );
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return if rc == SQLITE_ROW && lightnvr_uuid_is_valid(camera_uuid) as ::core::ffi::c_int != 0 {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_lpr_reads_prune(
    mut received_before_ms: int64_t,
    mut batch_limit: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if received_before_ms <= 0 as int64_t
        || batch_limit < 1 as ::core::ffi::c_int
        || batch_limit > 10000 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"DELETE FROM lpr_reads WHERE uuid IN (SELECT uuid FROM lpr_reads WHERE received_at_ms < ? ORDER BY received_at_ms LIMIT ?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(
            stmt,
            1 as ::core::ffi::c_int,
            received_before_ms as sqlite3_int64,
        );
        sqlite3_bind_int(stmt, 2 as ::core::ffi::c_int, batch_limit);
        rc = sqlite3_step(stmt);
    }
    let mut changed: ::core::ffi::c_int = sqlite3_changes(db);
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return if rc == SQLITE_DONE {
        changed
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_lpr_read_from_onvif(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut event: *const onvif_lpr_event_t,
    mut input: *mut lpr_read_input_t,
) -> ::core::ffi::c_int {
    if !lightnvr_uuid_is_valid(camera_uuid)
        || stream_name.is_null()
        || *stream_name.offset(0isize) == 0
        || strlen(stream_name) >= MAX_STREAM_NAME as size_t
        || event.is_null()
        || !(*event).asserted
        || (*event).plate[0usize] == 0
        || input.is_null()
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        input as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<lpr_read_input_t>(),
    );
    snprintf(
        &raw mut (*input).camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        camera_uuid,
    );
    snprintf(
        &raw mut (*input).stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    (*input).observed_at_ms = if (*event).observed_at_ms > 0 as int64_t {
        (*event).observed_at_ms
    } else {
        current_time_ms()
    };
    snprintf(
        &raw mut (*input).source as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        if (*event).source.0 == onvif_lpr_source_t::ONVIF_LPR_SOURCE_PROFILE_M.0 {
            b"onvif_profile_m\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"onvif_vendor\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    snprintf(
        &raw mut (*input).vendor_topic as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        if contains_case_insensitive(
            &raw const (*event).topic as *const ::core::ffi::c_char,
            &raw const (*event).plate as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        {
            b"[plate-redacted-vendor-topic]\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            &raw const (*event).topic as *const ::core::ffi::c_char
        },
    );
    snprintf(
        &raw mut (*input).plate as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).plate as *const ::core::ffi::c_char,
    );
    (*input).has_confidence = (*event).has_confidence;
    (*input).confidence = (*event).confidence;
    snprintf(
        &raw mut (*input).country as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).country as *const ::core::ffi::c_char,
    );
    snprintf(
        &raw mut (*input).region as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).region as *const ::core::ffi::c_char,
    );
    snprintf(
        &raw mut (*input).plate_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).plate_type as *const ::core::ffi::c_char,
    );
    snprintf(
        &raw mut (*input).direction as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).direction as *const ::core::ffi::c_char,
    );
    snprintf(
        &raw mut (*input).lane as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).lane as *const ::core::ffi::c_char,
    );
    snprintf(
        &raw mut (*input).vehicle_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).vehicle_type as *const ::core::ffi::c_char,
    );
    snprintf(
        &raw mut (*input).vehicle_color as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).vehicle_color as *const ::core::ffi::c_char,
    );
    snprintf(
        &raw mut (*input).object_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).object_id as *const ::core::ffi::c_char,
    );
    snprintf(
        &raw mut (*input).correlation_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).correlation_id as *const ::core::ffi::c_char,
    );
    (*input).has_bounding_box = (*event).has_bounding_box;
    (*input).bbox_left = (*event).bbox_left;
    (*input).bbox_top = (*event).bbox_top;
    (*input).bbox_right = (*event).bbox_right;
    (*input).bbox_bottom = (*event).bbox_bottom;
    return 0 as ::core::ffi::c_int;
}
