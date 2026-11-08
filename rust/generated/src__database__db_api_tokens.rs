use ::libc;
unsafe extern "C" {
    pub type mbedtls_md_info_t;
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type fleet_selector;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
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
    unsafe fn mbedtls_sha256(
        input: *const ::core::ffi::c_uchar,
        ilen: size_t,
        output: *mut ::core::ffi::c_uchar,
        is224: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_exec(
        _: *mut sqlite3,
        sql: *const ::core::ffi::c_char,
        callback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
        errmsg: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_changes(_: *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
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
    unsafe fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_int64(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int)
    -> sqlite3_int64;
    unsafe fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fleet_selector_parse(
        json: *const cJSON,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> *mut fleet_selector_t;
    unsafe fn fleet_selector_free(selector: *mut fleet_selector_t);
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn secure_zero_memory(ptr: *mut ::core::ffi::c_void, size: size_t);
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn copy_trimmed_value(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
        input: *const ::core::ffi::c_char,
        input_size: size_t,
    ) -> size_t;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cJSON {
    pub next: *mut cJSON,
    pub prev: *mut cJSON,
    pub child: *mut cJSON,
    pub r#type: ::core::ffi::c_int,
    pub valuestring: *mut ::core::ffi::c_char,
    pub valueint: ::core::ffi::c_int,
    pub valuedouble: ::core::ffi::c_double,
    pub string: *mut ::core::ffi::c_char,
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
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_aes_context {
    pub private_nr: ::core::ffi::c_int,
    pub private_rk_offset: size_t,
    pub private_buf: [uint32_t; 68],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_md_context_t {
    pub private_md_info: *const mbedtls_md_info_t,
    pub private_md_ctx: *mut ::core::ffi::c_void,
    pub private_hmac_ctx: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbedtls_threading_mutex_t {
    pub private_mutex: pthread_mutex_t,
    pub private_state: ::core::ffi::c_char,
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
pub struct mbedtls_entropy_source_state {
    pub private_f_source: mbedtls_entropy_f_source_ptr,
    pub private_p_source: *mut ::core::ffi::c_void,
    pub private_size: size_t,
    pub private_threshold: size_t,
    pub private_strong: ::core::ffi::c_int,
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
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type fleet_selector_t = fleet_selector;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_0 {
    pub const AUTHZ_ACTION_INVALID: Self = Self(-1);
    pub const AUTHZ_LIVE_VIEW: Self = Self(0);
    pub const AUTHZ_AUDIO_LISTEN: Self = Self(1);
    pub const AUTHZ_AUDIO_TALK: Self = Self(2);
    pub const AUTHZ_RECORDINGS_REPLAY: Self = Self(3);
    pub const AUTHZ_RECORDINGS_EXPORT: Self = Self(4);
    pub const AUTHZ_SNAPSHOT_CREATE: Self = Self(5);
    pub const AUTHZ_PTZ_CONTROL: Self = Self(6);
    pub const AUTHZ_EVIDENCE_PROTECT: Self = Self(7);
    pub const AUTHZ_RECORDING_DELETE: Self = Self(8);
    pub const AUTHZ_CAMERA_CONFIGURE: Self = Self(9);
    pub const AUTHZ_FLEET_EXECUTE_JOB: Self = Self(10);
    pub const AUTHZ_STORAGE_CONFIGURE: Self = Self(11);
    pub const AUTHZ_EVENTS_CONFIGURE: Self = Self(12);
    pub const AUTHZ_USERS_MANAGE: Self = Self(13);
    pub const AUTHZ_SYSTEM_ADMIN: Self = Self(14);
    pub const AUTHZ_LPR_READ: Self = Self(15);
    pub const AUTHZ_LPR_SEARCH: Self = Self(16);
    pub const AUTHZ_LPR_EXPORT: Self = Self(17);
    pub const AUTHZ_LPR_DELETE: Self = Self(18);
    pub const AUTHZ_OBSERVATIONS_ANNOTATE: Self = Self(19);
    pub const AUTHZ_ACTION_COUNT: Self = Self(20);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_api_token_result_t(pub ::core::ffi::c_int);
impl db_api_token_result_t {
    pub const DB_API_TOKEN_OK: Self = Self(0);
    pub const DB_API_TOKEN_ERROR: Self = Self(-1);
    pub const DB_API_TOKEN_NOT_FOUND: Self = Self(-2);
    pub const DB_API_TOKEN_INVALID: Self = Self(-3);
    pub const DB_API_TOKEN_LIMIT: Self = Self(-4);
    pub const DB_API_TOKEN_EXPIRED: Self = Self(-5);
    pub const DB_API_TOKEN_REVOKED: Self = Self(-6);
    pub const DB_API_TOKEN_INACTIVE_OWNER: Self = Self(-7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct api_token_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub user_id: int64_t,
    pub created_by_user_id: int64_t,
    pub description: [::core::ffi::c_char; 128],
    pub token_prefix: [::core::ffi::c_char; 16],
    pub action_mask: uint64_t,
    pub scope_type: [::core::ffi::c_char; 16],
    pub selector_json: [::core::ffi::c_char; 8192],
    pub collection_uuid: [::core::ffi::c_char; 37],
    pub expires_at: int64_t,
    pub revoked_at: int64_t,
    pub last_used_at: int64_t,
    pub created_at: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct api_token_create_t {
    pub user_id: int64_t,
    pub created_by_user_id: int64_t,
    pub description: *const ::core::ffi::c_char,
    pub action_mask: uint64_t,
    pub scope_type: *const ::core::ffi::c_char,
    pub selector_json: *const ::core::ffi::c_char,
    pub collection_uuid: *const ::core::ffi::c_char,
    pub expires_at: int64_t,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const API_TOKEN_SECRET_MAX: ::core::ffi::c_int = 80 as ::core::ffi::c_int;
pub const API_TOKEN_SELECTOR_MAX: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const API_TOKEN_MAX_PER_USER: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const API_TOKEN_MAX_LIFETIME_SECONDS: ::core::ffi::c_long = 366 as ::core::ffi::c_long
    * 24 as ::core::ffi::c_long
    * 60 as ::core::ffi::c_long
    * 60 as ::core::ffi::c_long;
pub const TOKEN_RANDOM_BYTES: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const TOKEN_HASH_HEX_SIZE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
unsafe extern "C" fn copy_column(
    mut destination: *mut ::core::ffi::c_char,
    mut destination_size: size_t,
    mut stmt: *mut sqlite3_stmt,
    mut column: ::core::ffi::c_int,
) {
    let mut value: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, column) as *const ::core::ffi::c_char;
    safe_strcpy(
        destination,
        if !value.is_null() {
            value
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        destination_size,
        0 as size_t,
    );
}
unsafe extern "C" fn populate_token(mut stmt: *mut sqlite3_stmt, mut token: *mut api_token_t) {
    memset(
        token as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<api_token_t>(),
    );
    copy_column(
        &raw mut (*token).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        stmt,
        0 as ::core::ffi::c_int,
    );
    (*token).user_id = sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as int64_t;
    (*token).created_by_user_id = sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int) as int64_t;
    copy_column(
        &raw mut (*token).description as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        stmt,
        3 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*token).token_prefix as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        stmt,
        4 as ::core::ffi::c_int,
    );
    (*token).action_mask = sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as uint64_t;
    copy_column(
        &raw mut (*token).scope_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        stmt,
        6 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*token).selector_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        stmt,
        7 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*token).collection_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        stmt,
        8 as ::core::ffi::c_int,
    );
    (*token).expires_at = sqlite3_column_int64(stmt, 9 as ::core::ffi::c_int) as int64_t;
    (*token).revoked_at = sqlite3_column_int64(stmt, 10 as ::core::ffi::c_int) as int64_t;
    (*token).last_used_at = sqlite3_column_int64(stmt, 11 as ::core::ffi::c_int) as int64_t;
    (*token).created_at = sqlite3_column_int64(stmt, 12 as ::core::ffi::c_int) as int64_t;
}
unsafe extern "C" fn token_select_fields() -> *const ::core::ffi::c_char {
    return b"uuid,user_id,COALESCE(created_by_user_id,0),description,token_prefix,action_mask,scope_type,COALESCE(selector_json,''),COALESCE(collection_uuid,''),expires_at,COALESCE(revoked_at,0),COALESCE(last_used_at,0),created_at\0"
        .as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn valid_selector(mut serialized: *const ::core::ffi::c_char) -> bool {
    if serialized.is_null()
        || *serialized.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strlen(serialized) >= API_TOKEN_SELECTOR_MAX as size_t
    {
        return r#false != 0;
    }
    let mut json: *mut cJSON = cJSON_Parse(serialized);
    if json.is_null() {
        return r#false != 0;
    }
    let mut error: [::core::ffi::c_char; 256] = [
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
        0,
        0,
        0,
    ];
    let mut selector: *mut fleet_selector_t = fleet_selector_parse(
        json,
        &raw mut error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    cJSON_Delete(json);
    if selector.is_null() {
        return r#false != 0;
    }
    fleet_selector_free(selector);
    return r#true != 0;
}
unsafe extern "C" fn valid_scope(mut input: *const api_token_create_t) -> bool {
    if strcmp(
        (*input).scope_type,
        b"all\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return (*input).selector_json.is_null() && (*input).collection_uuid.is_null();
    }
    if strcmp(
        (*input).scope_type,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return (*input).collection_uuid.is_null()
            && valid_selector((*input).selector_json) as ::core::ffi::c_int != 0;
    }
    return strcmp(
        (*input).scope_type,
        b"collection\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && (*input).selector_json.is_null()
        && !(*input).collection_uuid.is_null()
        && *(*input).collection_uuid.offset(0isize) as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int;
}
unsafe extern "C" fn shared_collection_exists_locked(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
) -> bool {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT 1 FROM camera_collections WHERE uuid=? AND is_shared=1;\0".as_ptr()
            as *const ::core::ffi::c_char,
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
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    return rc == SQLITE_ROW;
}
unsafe extern "C" fn bytes_to_hex(
    mut bytes: *const ::core::ffi::c_uchar,
    mut byte_count: size_t,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) -> ::core::ffi::c_int {
    static mut alphabet: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"0123456789abcdef\0")
    };
    if bytes.is_null()
        || output.is_null()
        || output_size
            < byte_count
                .wrapping_mul(2 as size_t)
                .wrapping_add(1 as size_t)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut i: size_t = 0 as size_t;
    while i < byte_count {
        *output.offset(i.wrapping_mul(2 as size_t) as isize) = alphabet
            [(*bytes.offset(i as isize) as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as usize];
        *output.offset(i.wrapping_mul(2 as size_t).wrapping_add(1 as size_t) as isize) = alphabet
            [(*bytes.offset(i as isize) as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                as usize];
        i = i.wrapping_add(1);
    }
    *output.offset(byte_count.wrapping_mul(2 as size_t) as isize) = '\0' as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn generate_secret(mut secret: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
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
    let mut random: mbedtls_ctr_drbg_context = mbedtls_ctr_drbg_context {
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
    mbedtls_entropy_init(&raw mut entropy);
    mbedtls_ctr_drbg_init(&raw mut random);
    static mut personalization: [::core::ffi::c_uchar; 19] = unsafe {
        ::core::mem::transmute::<[u8; 19], [::core::ffi::c_uchar; 19]>(*b"lightnvr-api-token\0")
    };
    let mut rc: ::core::ffi::c_int = mbedtls_ctr_drbg_seed(
        &raw mut random,
        Some(
            mbedtls_entropy_func
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_uchar,
                    size_t,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut entropy as *mut ::core::ffi::c_void,
        &raw const personalization as *const ::core::ffi::c_uchar,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 19]>().wrapping_sub(1 as size_t),
    );
    let mut bytes: [::core::ffi::c_uchar; 32] = [0; 32];
    if rc == 0 as ::core::ffi::c_int {
        rc = mbedtls_ctr_drbg_random(
            &raw mut random as *mut ::core::ffi::c_void,
            &raw mut bytes as *mut ::core::ffi::c_uchar,
            ::core::mem::size_of::<[::core::ffi::c_uchar; 32]>(),
        );
    }
    let mut hex: [::core::ffi::c_char; 65] = [0; 65];
    if rc == 0 as ::core::ffi::c_int {
        rc = bytes_to_hex(
            &raw mut bytes as *mut ::core::ffi::c_uchar,
            ::core::mem::size_of::<[::core::ffi::c_uchar; 32]>(),
            &raw mut hex as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
        );
    }
    if rc == 0 as ::core::ffi::c_int {
        let mut written: ::core::ffi::c_int = snprintf(
            secret,
            API_TOKEN_SECRET_MAX as size_t,
            b"lnvr_%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut hex as *mut ::core::ffi::c_char,
        );
        if written < 0 as ::core::ffi::c_int || written >= API_TOKEN_SECRET_MAX {
            rc = -1 as ::core::ffi::c_int;
        }
    }
    memset(
        &raw mut bytes as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 32]>(),
    );
    mbedtls_ctr_drbg_free(&raw mut random);
    mbedtls_entropy_free(&raw mut entropy);
    return if rc == 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn hash_secret(
    mut secret: *const ::core::ffi::c_char,
    mut hash: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if secret.is_null()
        || *secret.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut digest: [::core::ffi::c_uchar; 32] = [0; 32];
    if mbedtls_sha256(
        secret as *const ::core::ffi::c_uchar,
        strlen(secret),
        &raw mut digest as *mut ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int,
    ) != 0 as ::core::ffi::c_int
    {
        secure_zero_memory(
            &raw mut digest as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_uchar; 32]>(),
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut rc: ::core::ffi::c_int = bytes_to_hex(
        &raw mut digest as *mut ::core::ffi::c_uchar,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 32]>(),
        hash,
        TOKEN_HASH_HEX_SIZE as size_t,
    );
    secure_zero_memory(
        &raw mut digest as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 32]>(),
    );
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_api_token_create(
    mut input: *const api_token_create_t,
    mut token: *mut api_token_t,
    mut secret: *mut ::core::ffi::c_char,
) -> db_api_token_result_t {
    let mut now: int64_t = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    let mut valid_actions: uint64_t =
        ((1 as uint64_t) << C2Rust_Unnamed_0::AUTHZ_ACTION_COUNT.0).wrapping_sub(1 as uint64_t);
    let mut description: [::core::ffi::c_char; 128] = [0; 128];
    if input.is_null()
        || token.is_null()
        || secret.is_null()
        || (*input).user_id <= 0 as int64_t
        || (*input).created_by_user_id <= 0 as int64_t
        || (*input).description.is_null()
        || strlen((*input).description) >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
        || copy_trimmed_value(
            &raw mut description as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            (*input).description,
            0 as size_t,
        ) == 0 as size_t
        || (*input).action_mask == 0 as uint64_t
        || (*input).action_mask & !valid_actions != 0 as uint64_t
        || (*input).scope_type.is_null()
        || !valid_scope(input)
        || (*input).expires_at <= now
        || (*input).expires_at - now > API_TOKEN_MAX_LIFETIME_SECONDS as int64_t
    {
        return db_api_token_result_t::DB_API_TOKEN_INVALID;
    }
    let mut cursor: *const ::core::ffi::c_uchar =
        &raw mut description as *mut ::core::ffi::c_char as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_IScntrl.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            return db_api_token_result_t::DB_API_TOKEN_INVALID;
        }
        cursor = cursor.offset(1);
    }
    memset(
        token as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<api_token_t>(),
    );
    *secret.offset(0isize) = '\0' as ::core::ffi::c_char;
    if generate_secret(secret) != 0 as ::core::ffi::c_int {
        return db_api_token_result_t::DB_API_TOKEN_ERROR;
    }
    let mut hash: [::core::ffi::c_char; 65] = [0; 65];
    if hash_secret(secret, &raw mut hash as *mut ::core::ffi::c_char) != 0 as ::core::ffi::c_int {
        secure_zero_memory(
            secret as *mut ::core::ffi::c_void,
            API_TOKEN_SECRET_MAX as size_t,
        );
        return db_api_token_result_t::DB_API_TOKEN_ERROR;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        secure_zero_memory(
            secret as *mut ::core::ffi::c_void,
            API_TOKEN_SECRET_MAX as size_t,
        );
        return db_api_token_result_t::DB_API_TOKEN_ERROR;
    }
    pthread_mutex_lock(mutex);
    if sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        pthread_mutex_unlock(mutex);
        secure_zero_memory(
            secret as *mut ::core::ffi::c_void,
            API_TOKEN_SECRET_MAX as size_t,
        );
        return db_api_token_result_t::DB_API_TOKEN_ERROR;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT count(*) FROM authz_api_tokens WHERE user_id=? AND revoked_at IS NULL AND expires_at>strftime('%s','now');\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(
            stmt,
            1 as ::core::ffi::c_int,
            (*input).user_id as sqlite3_int64,
        );
        rc = sqlite3_step(stmt);
    }
    let mut active_count: ::core::ffi::c_int = if rc == SQLITE_ROW {
        sqlite3_column_int(stmt, 0 as ::core::ffi::c_int)
    } else {
        -1 as ::core::ffi::c_int
    };
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if active_count >= API_TOKEN_MAX_PER_USER {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        secure_zero_memory(
            secret as *mut ::core::ffi::c_void,
            API_TOKEN_SECRET_MAX as size_t,
        );
        return db_api_token_result_t::DB_API_TOKEN_LIMIT;
    }
    if active_count < 0 as ::core::ffi::c_int
        || strcmp(
            (*input).scope_type,
            b"collection\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && !shared_collection_exists_locked(db, (*input).collection_uuid)
    {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        secure_zero_memory(
            secret as *mut ::core::ffi::c_void,
            API_TOKEN_SECRET_MAX as size_t,
        );
        return db_api_token_result_t(if active_count < 0 as ::core::ffi::c_int {
            db_api_token_result_t::DB_API_TOKEN_ERROR.0
        } else {
            db_api_token_result_t::DB_API_TOKEN_INVALID.0
        });
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO authz_api_tokens (uuid,user_id,created_by_user_id,description,token_prefix,token_hash,action_mask,scope_type,selector_json,collection_uuid,expires_at) VALUES (lower(hex(randomblob(4))||'-'||hex(randomblob(2))||'-4'||substr(hex(randomblob(2)),2)||'-'||substr('89ab',(abs(random())%4)+1,1)||substr(hex(randomblob(2)),2)||'-'||hex(randomblob(6))),?,?,?,?,?,?,?,?,?,?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = sqlite3_prepare_v2(
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
            (*input).user_id as sqlite3_int64,
        );
        sqlite3_bind_int64(
            stmt,
            2 as ::core::ffi::c_int,
            (*input).created_by_user_id as sqlite3_int64,
        );
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            &raw mut description as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            4 as ::core::ffi::c_int,
            secret,
            13 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            5 as ::core::ffi::c_int,
            &raw mut hash as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(
            stmt,
            6 as ::core::ffi::c_int,
            (*input).action_mask as sqlite3_int64,
        );
        sqlite3_bind_text(
            stmt,
            7 as ::core::ffi::c_int,
            (*input).scope_type,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if !(*input).selector_json.is_null() {
            sqlite3_bind_text(
                stmt,
                8 as ::core::ffi::c_int,
                (*input).selector_json,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(stmt, 8 as ::core::ffi::c_int);
        }
        if !(*input).collection_uuid.is_null() {
            sqlite3_bind_text(
                stmt,
                9 as ::core::ffi::c_int,
                (*input).collection_uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(stmt, 9 as ::core::ffi::c_int);
        }
        sqlite3_bind_int64(
            stmt,
            10 as ::core::ffi::c_int,
            (*input).expires_at as sqlite3_int64,
        );
        rc = sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if rc == SQLITE_DONE {
        let mut query: [::core::ffi::c_char; 512] = [0; 512];
        snprintf(
            &raw mut query as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            b"SELECT %s FROM authz_api_tokens WHERE rowid=last_insert_rowid();\0".as_ptr()
                as *const ::core::ffi::c_char,
            token_select_fields(),
        );
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = sqlite3_prepare_v2(
            db,
            &raw mut query as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW {
            populate_token(stmt, token);
            rc = SQLITE_DONE;
        }
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
        }
    }
    let mut committed: bool = rc == SQLITE_DONE
        && (*token).uuid[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && sqlite3_exec(
            db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) == SQLITE_OK;
    if !committed {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    }
    pthread_mutex_unlock(mutex);
    secure_zero_memory(
        &raw mut hash as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    );
    if !committed {
        secure_zero_memory(
            secret as *mut ::core::ffi::c_void,
            API_TOKEN_SECRET_MAX as size_t,
        );
        return db_api_token_result_t::DB_API_TOKEN_ERROR;
    }
    return db_api_token_result_t::DB_API_TOKEN_OK;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_api_token_list(
    mut user_id: int64_t,
    mut tokens: *mut *mut api_token_t,
    mut token_count: *mut ::core::ffi::c_int,
) -> db_api_token_result_t {
    if user_id <= 0 as int64_t || tokens.is_null() || token_count.is_null() {
        return db_api_token_result_t::DB_API_TOKEN_INVALID;
    }
    *tokens = ::core::ptr::null_mut::<api_token_t>();
    *token_count = 0 as ::core::ffi::c_int;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_api_token_result_t::DB_API_TOKEN_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut sql: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"SELECT %s FROM authz_api_tokens WHERE user_id=? ORDER BY created_at DESC,uuid LIMIT ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        token_select_fields(),
    );
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
        sqlite3_bind_int(stmt, 2 as ::core::ffi::c_int, API_TOKEN_MAX_PER_USER);
    }
    let mut capacity: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut loaded: *mut api_token_t = (if rc == SQLITE_OK {
        calloc(capacity as size_t, ::core::mem::size_of::<api_token_t>())
    } else {
        NULL
    }) as *mut api_token_t;
    if rc == SQLITE_OK && loaded.is_null() {
        rc = SQLITE_NOMEM;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if rc == SQLITE_OK {
        loop {
            rc = sqlite3_step(stmt);
            if rc != SQLITE_ROW {
                break;
            }
            if count == capacity {
                capacity *= 2 as ::core::ffi::c_int;
                let mut resized: *mut api_token_t = realloc(
                    loaded as *mut ::core::ffi::c_void,
                    (capacity as size_t).wrapping_mul(::core::mem::size_of::<api_token_t>()),
                ) as *mut api_token_t;
                if resized.is_null() {
                    rc = SQLITE_NOMEM;
                    break;
                } else {
                    loaded = resized;
                }
            }
            let c2rust_fresh0 = count;
            count += 1;
            populate_token(stmt, loaded.offset(c2rust_fresh0 as isize));
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    if rc != SQLITE_DONE {
        free(loaded as *mut ::core::ffi::c_void);
        return db_api_token_result_t::DB_API_TOKEN_ERROR;
    }
    if count == 0 as ::core::ffi::c_int {
        free(loaded as *mut ::core::ffi::c_void);
        loaded = ::core::ptr::null_mut::<api_token_t>();
    }
    *tokens = loaded;
    *token_count = count;
    return db_api_token_result_t::DB_API_TOKEN_OK;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_api_token_revoke(
    mut user_id: int64_t,
    mut token_uuid: *const ::core::ffi::c_char,
) -> db_api_token_result_t {
    if user_id <= 0 as int64_t
        || token_uuid.is_null()
        || *token_uuid.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return db_api_token_result_t::DB_API_TOKEN_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_api_token_result_t::DB_API_TOKEN_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"UPDATE authz_api_tokens SET revoked_at=strftime('%s','now') WHERE uuid=? AND user_id=? AND revoked_at IS NULL;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            token_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, user_id as sqlite3_int64);
        rc = sqlite3_step(stmt);
    }
    let mut changes: ::core::ffi::c_int = if rc == SQLITE_DONE {
        sqlite3_changes(db)
    } else {
        0 as ::core::ffi::c_int
    };
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    if rc != SQLITE_DONE {
        return db_api_token_result_t::DB_API_TOKEN_ERROR;
    }
    return db_api_token_result_t(if changes == 1 as ::core::ffi::c_int {
        db_api_token_result_t::DB_API_TOKEN_OK.0
    } else {
        db_api_token_result_t::DB_API_TOKEN_NOT_FOUND.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_api_token_authenticate(
    mut secret: *const ::core::ffi::c_char,
    mut user_id: *mut int64_t,
    mut token_uuid: *mut ::core::ffi::c_char,
    mut usage_audit_due: *mut bool,
) -> db_api_token_result_t {
    if secret.is_null()
        || strlen(secret)
            != (5 as ::core::ffi::c_int + TOKEN_RANDOM_BYTES * 2 as ::core::ffi::c_int) as size_t
        || strncmp(
            secret,
            b"lnvr_\0".as_ptr() as *const ::core::ffi::c_char,
            5 as size_t,
        ) != 0 as ::core::ffi::c_int
        || user_id.is_null()
        || token_uuid.is_null()
    {
        return db_api_token_result_t::DB_API_TOKEN_NOT_FOUND;
    }
    *user_id = 0 as int64_t;
    *token_uuid.offset(0isize) = '\0' as ::core::ffi::c_char;
    if !usage_audit_due.is_null() {
        *usage_audit_due = r#false != 0;
    }
    let mut hash: [::core::ffi::c_char; 65] = [0; 65];
    if hash_secret(secret, &raw mut hash as *mut ::core::ffi::c_char) != 0 as ::core::ffi::c_int {
        return db_api_token_result_t::DB_API_TOKEN_ERROR;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_api_token_result_t::DB_API_TOKEN_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT t.uuid,t.user_id,COALESCE(t.revoked_at,0),t.expires_at,COALESCE(t.last_used_at,0),u.is_active FROM authz_api_tokens t JOIN users u ON u.id=t.user_id WHERE t.token_hash=? LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw mut hash as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
    }
    let mut result: db_api_token_result_t = db_api_token_result_t::DB_API_TOKEN_NOT_FOUND;
    let mut now: int64_t = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    if rc == SQLITE_ROW {
        copy_column(
            token_uuid,
            CAMERA_UUID_STRING_SIZE as size_t,
            stmt,
            0 as ::core::ffi::c_int,
        );
        *user_id = sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as int64_t;
        let mut revoked_at: int64_t =
            sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int) as int64_t;
        let mut expires_at: int64_t =
            sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as int64_t;
        let mut last_used_at: int64_t =
            sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as int64_t;
        let mut owner_active: bool =
            sqlite3_column_int(stmt, 5 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        if revoked_at > 0 as int64_t {
            result = db_api_token_result_t::DB_API_TOKEN_REVOKED;
        } else if expires_at <= now {
            result = db_api_token_result_t::DB_API_TOKEN_EXPIRED;
        } else if !owner_active {
            result = db_api_token_result_t::DB_API_TOKEN_INACTIVE_OWNER;
        } else {
            result = db_api_token_result_t::DB_API_TOKEN_OK;
            if !usage_audit_due.is_null() {
                *usage_audit_due =
                    last_used_at == 0 as int64_t || last_used_at < now - 60 as int64_t;
            }
        }
    } else if rc != SQLITE_DONE {
        result = db_api_token_result_t::DB_API_TOKEN_ERROR;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if result.0 == db_api_token_result_t::DB_API_TOKEN_OK.0 {
        rc = sqlite3_prepare_v2(
            db,
            b"UPDATE authz_api_tokens SET last_used_at=strftime('%s','now') WHERE uuid=? AND (last_used_at IS NULL OR last_used_at<strftime('%s','now')-60);\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_text(
                stmt,
                1 as ::core::ffi::c_int,
                token_uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            rc = sqlite3_step(stmt);
        }
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
        }
    }
    pthread_mutex_unlock(mutex);
    secure_zero_memory(
        &raw mut hash as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    );
    if result.0 != db_api_token_result_t::DB_API_TOKEN_OK.0 {
        return result;
    }
    return db_api_token_result_t(if rc == SQLITE_DONE {
        db_api_token_result_t::DB_API_TOKEN_OK.0
    } else {
        db_api_token_result_t::DB_API_TOKEN_ERROR.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_api_token_get_active(
    mut token_uuid: *const ::core::ffi::c_char,
    mut token: *mut api_token_t,
) -> db_api_token_result_t {
    if token_uuid.is_null() || *token_uuid.offset(0isize) == 0 || token.is_null() {
        return db_api_token_result_t::DB_API_TOKEN_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_api_token_result_t::DB_API_TOKEN_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut result: db_api_token_result_t = db_api_token_get_active_on_connection(
        db,
        time(::core::ptr::null_mut::<time_t>()) as int64_t,
        token_uuid,
        token,
    );
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_api_token_get_active_on_connection(
    mut db: *mut sqlite3,
    mut observed_at: int64_t,
    mut token_uuid: *const ::core::ffi::c_char,
    mut token: *mut api_token_t,
) -> db_api_token_result_t {
    if observed_at < 0 as int64_t {
        return db_api_token_result_t::DB_API_TOKEN_ERROR;
    }
    if db.is_null()
        || token_uuid.is_null()
        || *token_uuid.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || token.is_null()
    {
        return db_api_token_result_t::DB_API_TOKEN_INVALID;
    }
    let mut sql: [::core::ffi::c_char; 640] = [0; 640];
    snprintf(
        &raw mut sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 640]>(),
        b"SELECT %s FROM authz_api_tokens WHERE uuid=? AND revoked_at IS NULL AND expires_at>? LIMIT 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        token_select_fields(),
    );
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        &raw mut sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            token_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, observed_at as sqlite3_int64);
        rc = sqlite3_step(stmt);
    }
    let mut result: db_api_token_result_t = db_api_token_result_t::DB_API_TOKEN_NOT_FOUND;
    if rc == SQLITE_ROW {
        populate_token(stmt, token);
        result = db_api_token_result_t::DB_API_TOKEN_OK;
    } else if rc != SQLITE_DONE {
        result = db_api_token_result_t::DB_API_TOKEN_ERROR;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    return result;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
