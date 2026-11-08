use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_mutex;
    pub type sqlite3_stmt;
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
    unsafe fn sqlite3_last_insert_rowid(_: *mut sqlite3) -> sqlite3_int64;
    unsafe fn sqlite3_changes(_: *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
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
    unsafe fn sqlite3_clear_bindings(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_double(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
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
    unsafe fn sqlite3_column_type(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_mutex_enter(_: *mut sqlite3_mutex);
    unsafe fn sqlite3_mutex_leave(_: *mut sqlite3_mutex);
    unsafe fn sqlite3_db_mutex(_: *mut sqlite3) -> *mut sqlite3_mutex;
    unsafe fn cJSON_ParseWithLength(
        value: *const ::core::ffi::c_char,
        buffer_length: size_t,
    ) -> *mut cJSON;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn event_identity_get_source(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn db_open_readonly_connection(connection: *mut *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn db_close_readonly_connection(connection: *mut sqlite3);
    unsafe fn base64url_encode(
        input: *const ::core::ffi::c_uchar,
        input_len: size_t,
        dst: *mut ::core::ffi::c_char,
        dst_size: size_t,
        out_len: *mut size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn base64url_decode(
        src: *const ::core::ffi::c_char,
        src_len: size_t,
        dst: *mut ::core::ffi::c_uchar,
        dst_size: size_t,
        out_len: *mut size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_generate_v5(
        namespace_uuid: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
        name_len: size_t,
        output: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
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
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_int);
impl C2Rust_Unnamed {
    pub const DB_OBS_OK: Self = Self(0);
    pub const DB_OBS_DUPLICATE: Self = Self(1);
    pub const DB_OBS_CAS_LOST: Self = Self(2);
    pub const DB_OBS_ERROR: Self = Self(-1);
    pub const DB_OBS_INVALID: Self = Self(-2);
    pub const DB_OBS_NOT_FOUND: Self = Self(-3);
    pub const DB_OBS_INVALID_CURSOR: Self = Self(-4);
    pub const DB_OBS_CURSOR_EXPIRED: Self = Self(-5);
    pub const DB_OBS_CONFLICT: Self = Self(-6);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_asset_state_t(pub ::core::ffi::c_uint);
impl observation_asset_state_t {
    pub const OBS_ASSET_PENDING: Self = Self(0);
    pub const OBS_ASSET_READY: Self = Self(1);
    pub const OBS_ASSET_UNAVAILABLE: Self = Self(2);
    pub const OBS_ASSET_EXPIRED: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_change_cause_t(pub ::core::ffi::c_uint);
impl observation_change_cause_t {
    pub const OBS_CAUSE_CREATED: Self = Self(0);
    pub const OBS_CAUSE_EVIDENCE: Self = Self(1);
    pub const OBS_CAUSE_CLASSIFICATION: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_publication_state_t(pub ::core::ffi::c_uint);
impl observation_publication_state_t {
    pub const OBS_PUBLICATION_PENDING: Self = Self(0);
    pub const OBS_PUBLICATION_RELAYED: Self = Self(1);
    pub const OBS_PUBLICATION_RELAYED_NO_ROUTE: Self = Self(2);
    pub const OBS_PUBLICATION_EXPIRED: Self = Self(3);
    pub const OBS_PUBLICATION_FAILED: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub reason: *const ::core::ffi::c_char,
    pub state: observation_asset_state_t,
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
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_object_input_t {
    pub object_index: ::core::ffi::c_int,
    pub label: [::core::ffi::c_char; 32],
    pub confidence: ::core::ffi::c_double,
    pub has_box: bool,
    pub box_x: ::core::ffi::c_double,
    pub box_y: ::core::ffi::c_double,
    pub box_width: ::core::ffi::c_double,
    pub box_height: ::core::ffi::c_double,
    pub has_track: bool,
    pub track_id: [::core::ffi::c_char; 65],
    pub track_namespace: [::core::ffi::c_char; 65],
    pub track_session: [::core::ffi::c_char; 37],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_batch_input_t {
    pub batch_id: [::core::ffi::c_char; 37],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub stream_name: [::core::ffi::c_char; 64],
    pub observed_at_ms: int64_t,
    pub engine_key: [::core::ffi::c_char; 65],
    pub engine_type: [::core::ffi::c_char; 16],
    pub model: [::core::ffi::c_char; 129],
    pub model_version: [::core::ffi::c_char; 65],
    pub recording_id: int64_t,
    pub object_count: ::core::ffi::c_int,
    pub objects: [observation_object_input_t; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_asset_input_t {
    pub asset_uuid: [::core::ffi::c_char; 37],
    pub state: observation_asset_state_t,
    pub reason: [::core::ffi::c_char; 33],
    pub origin: [::core::ffi::c_char; 24],
    pub media_type: [::core::ffi::c_char; 24],
    pub captured_at_ms: int64_t,
    pub transform_json: [::core::ffi::c_char; 1024],
    pub expires_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_insert_result_t {
    pub asset_uuid: [::core::ffi::c_char; 37],
    pub object_count: ::core::ffi::c_int,
    pub observation_uuids: [[::core::ffi::c_char; 37]; 20],
    pub last_seq: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct obs_tx_t {
    pub mutex: *mut pthread_mutex_t,
    pub db: *mut sqlite3,
    pub connection_mutex: *mut sqlite3_mutex,
}
pub type db_observation_publication_hook_t = Option<unsafe extern "C" fn() -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_asset_media_t {
    pub path: *const ::core::ffi::c_char,
    pub bytes: int64_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub expires_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_transition_result_t {
    pub observations_bumped: ::core::ffi::c_int,
    pub last_seq: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_asset_record_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub state: observation_asset_state_t,
    pub reason: [::core::ffi::c_char; 33],
    pub origin: [::core::ffi::c_char; 24],
    pub media_type: [::core::ffi::c_char; 24],
    pub path: [::core::ffi::c_char; 512],
    pub has_bytes: bool,
    pub bytes: int64_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub captured_at_ms: int64_t,
    pub transform_json: [::core::ffi::c_char; 1024],
    pub is_protected: bool,
    pub expires_at_ms: int64_t,
    pub created_at_ms: int64_t,
    pub updated_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_record_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub stream_name_at_capture: [::core::ffi::c_char; 64],
    pub batch_id: [::core::ffi::c_char; 37],
    pub object_index: ::core::ffi::c_int,
    pub observed_at_ms: int64_t,
    pub created_at_ms: int64_t,
    pub engine_key: [::core::ffi::c_char; 65],
    pub engine_type: [::core::ffi::c_char; 16],
    pub model: [::core::ffi::c_char; 129],
    pub model_version: [::core::ffi::c_char; 65],
    pub label: [::core::ffi::c_char; 32],
    pub confidence: ::core::ffi::c_double,
    pub has_box: bool,
    pub box_x: ::core::ffi::c_double,
    pub box_y: ::core::ffi::c_double,
    pub box_width: ::core::ffi::c_double,
    pub box_height: ::core::ffi::c_double,
    pub has_track: bool,
    pub track_id: [::core::ffi::c_char; 65],
    pub track_namespace: [::core::ffi::c_char; 65],
    pub track_session: [::core::ffi::c_char; 37],
    pub recording_id: int64_t,
    pub revision: ::core::ffi::c_int,
    pub asset: observation_asset_record_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_list_query_t {
    pub camera_uuids: *const [::core::ffi::c_char; 37],
    pub camera_count: size_t,
    pub all_cameras: bool,
    pub labels: [[::core::ffi::c_char; 32]; 8],
    pub label_count: size_t,
    pub has_since: bool,
    pub since_ms: int64_t,
    pub has_until: bool,
    pub until_ms: int64_t,
    pub limit: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_page_t {
    pub items: *mut observation_record_t,
    pub count: size_t,
    pub next_cursor: [::core::ffi::c_char; 257],
    pub has_more: bool,
    pub oldest_cursor: [::core::ffi::c_char; 257],
    pub has_oldest_observed_at: bool,
    pub oldest_observed_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_meta_t {
    pub epoch_uuid: [::core::ffi::c_char; 37],
    pub oldest_seq: int64_t,
    pub head_seq: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_db_diagnostics_t {
    pub meta: observation_meta_t,
    pub has_oldest_observed_at: bool,
    pub oldest_observed_at_ms: int64_t,
    pub observations: int64_t,
    pub publications_pending: int64_t,
    pub publications_failed: int64_t,
    pub publications_expired: int64_t,
    pub oldest_pending_created_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_publication_t {
    pub change_seq: int64_t,
    pub message_id: [::core::ffi::c_char; 37],
    pub attempts: ::core::ffi::c_int,
    pub observation_uuid: [::core::ffi::c_char; 37],
    pub revision: ::core::ffi::c_int,
    pub cause: observation_change_cause_t,
    pub evidence_state: observation_asset_state_t,
    pub evidence_reason: [::core::ffi::c_char; 33],
    pub change_created_at_ms: int64_t,
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub label: [::core::ffi::c_char; 32],
    pub observed_at_ms: int64_t,
    pub asset_uuid: [::core::ffi::c_char; 37],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_prune_result_t {
    pub changes_deleted: ::core::ffi::c_int,
    pub observations_deleted: ::core::ffi::c_int,
    pub assets_deleted: ::core::ffi::c_int,
    pub oldest_seq: int64_t,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const MAX_PATH_LENGTH: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const OBS_UUID_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const OBS_MAX_BATCH: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const OBS_LABEL_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const OBS_CURSOR_SIZE: ::core::ffi::c_int = 257 as ::core::ffi::c_int;
pub const OBS_LIST_DEFAULT_LIMIT: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const OBS_LIST_MAX_LIMIT: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
pub const OBS_LIST_SCAN_CAP: ::core::ffi::c_int = 5000 as ::core::ffi::c_int;
pub const OBS_QUERY_MAX_LABELS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const OBS_MESSAGE_ID_NAMESPACE: [::core::ffi::c_char; 37] = unsafe {
    ::core::mem::transmute::<[u8; 37], [::core::ffi::c_char; 37]>(
        *b"0bc01005-a431-522b-9124-a114f67d187a\0",
    )
};
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OBS_SOURCE_SIZE: ::core::ffi::c_int = 160 as ::core::ffi::c_int;
pub const OBS_PRUNE_MAX_BATCH: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const OBS_MAX_SAFE_SEQ: ::core::ffi::c_longlong = 9007199254740991 as ::core::ffi::c_longlong;
static mut STATE_NAMES: [*const ::core::ffi::c_char; 4] = [
    b"pending\0".as_ptr() as *const ::core::ffi::c_char,
    b"ready\0".as_ptr() as *const ::core::ffi::c_char,
    b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
    b"expired\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut CAUSE_NAMES: [*const ::core::ffi::c_char; 3] = [
    b"created\0".as_ptr() as *const ::core::ffi::c_char,
    b"evidence\0".as_ptr() as *const ::core::ffi::c_char,
    b"classification\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut PUBLICATION_NAMES: [*const ::core::ffi::c_char; 5] = [
    b"pending\0".as_ptr() as *const ::core::ffi::c_char,
    b"relayed\0".as_ptr() as *const ::core::ffi::c_char,
    b"relayed_no_route\0".as_ptr() as *const ::core::ffi::c_char,
    b"expired\0".as_ptr() as *const ::core::ffi::c_char,
    b"failed\0".as_ptr() as *const ::core::ffi::c_char,
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_asset_state_name(
    mut state: observation_asset_state_t,
) -> *const ::core::ffi::c_char {
    return if state.0 < 4 as ::core::ffi::c_uint {
        STATE_NAMES[state.0 as usize]
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_asset_state_parse(
    mut name: *const ::core::ffi::c_char,
    mut state: *mut observation_asset_state_t,
) -> ::core::ffi::c_int {
    if name.is_null() || state.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        if strcmp(name, STATE_NAMES[i as usize]) == 0 as ::core::ffi::c_int {
            *state = observation_asset_state_t(i as ::core::ffi::c_uint);
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_change_cause_name(
    mut cause: observation_change_cause_t,
) -> *const ::core::ffi::c_char {
    return if cause.0 < 3 as ::core::ffi::c_uint {
        CAUSE_NAMES[cause.0 as usize]
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
unsafe extern "C" fn cause_parse(
    mut name: *const ::core::ffi::c_char,
    mut cause: *mut observation_change_cause_t,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !name.is_null() && i < 3 as ::core::ffi::c_int {
        if strcmp(name, CAUSE_NAMES[i as usize]) == 0 as ::core::ffi::c_int {
            *cause = observation_change_cause_t(i as ::core::ffi::c_uint);
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_publication_state_name(
    mut state: observation_publication_state_t,
) -> *const ::core::ffi::c_char {
    return if state.0 < 5 as ::core::ffi::c_uint {
        PUBLICATION_NAMES[state.0 as usize]
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
static mut REASONS: [C2Rust_Unnamed_0; 9] = [
    C2Rust_Unnamed_0 {
        reason: b"queue_full\0".as_ptr() as *const ::core::ffi::c_char,
        state: observation_asset_state_t::OBS_ASSET_UNAVAILABLE,
    },
    C2Rust_Unnamed_0 {
        reason: b"budget_exhausted\0".as_ptr() as *const ::core::ffi::c_char,
        state: observation_asset_state_t::OBS_ASSET_UNAVAILABLE,
    },
    C2Rust_Unnamed_0 {
        reason: b"encode_failed\0".as_ptr() as *const ::core::ffi::c_char,
        state: observation_asset_state_t::OBS_ASSET_UNAVAILABLE,
    },
    C2Rust_Unnamed_0 {
        reason: b"write_failed\0".as_ptr() as *const ::core::ffi::c_char,
        state: observation_asset_state_t::OBS_ASSET_UNAVAILABLE,
    },
    C2Rust_Unnamed_0 {
        reason: b"interrupted\0".as_ptr() as *const ::core::ffi::c_char,
        state: observation_asset_state_t::OBS_ASSET_UNAVAILABLE,
    },
    C2Rust_Unnamed_0 {
        reason: b"no_matching_media\0".as_ptr() as *const ::core::ffi::c_char,
        state: observation_asset_state_t::OBS_ASSET_UNAVAILABLE,
    },
    C2Rust_Unnamed_0 {
        reason: b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
        state: observation_asset_state_t::OBS_ASSET_UNAVAILABLE,
    },
    C2Rust_Unnamed_0 {
        reason: b"retention_expired\0".as_ptr() as *const ::core::ffi::c_char,
        state: observation_asset_state_t::OBS_ASSET_EXPIRED,
    },
    C2Rust_Unnamed_0 {
        reason: b"deleted\0".as_ptr() as *const ::core::ffi::c_char,
        state: observation_asset_state_t::OBS_ASSET_EXPIRED,
    },
];
unsafe extern "C" fn empty_text(mut value: *const ::core::ffi::c_char) -> bool {
    return value.is_null() || *value.offset(0isize) == 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_reason_is_valid(
    mut state: observation_asset_state_t,
    mut reason: *const ::core::ffi::c_char,
) -> bool {
    if state.0 == observation_asset_state_t::OBS_ASSET_PENDING.0
        || state.0 == observation_asset_state_t::OBS_ASSET_READY.0
    {
        return empty_text(reason);
    }
    if state.0 != observation_asset_state_t::OBS_ASSET_UNAVAILABLE.0
        && state.0 != observation_asset_state_t::OBS_ASSET_EXPIRED.0
    {
        return r#false != 0;
    }
    if empty_text(reason) {
        return r#false != 0;
    }
    let mut i: size_t = 0 as size_t;
    while i < ::core::mem::size_of::<[C2Rust_Unnamed_0; 9]>()
        .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_0>())
    {
        if strcmp(reason, REASONS[i].reason) == 0 as ::core::ffi::c_int {
            return REASONS[i].state.0 == state.0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn reason_in(
    mut reason: *const ::core::ffi::c_char,
    mut allowed: *const *const ::core::ffi::c_char,
    mut count: size_t,
) -> bool {
    let mut i: size_t = 0 as size_t;
    while !reason.is_null() && i < count {
        if strcmp(reason, *allowed.offset(i as isize)) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn transition_allowed(
    mut from: observation_asset_state_t,
    mut to: observation_asset_state_t,
    mut reason: *const ::core::ffi::c_char,
) -> bool {
    static mut pending_unavailable: [*const ::core::ffi::c_char; 4] = [
        b"encode_failed\0".as_ptr() as *const ::core::ffi::c_char,
        b"write_failed\0".as_ptr() as *const ::core::ffi::c_char,
        b"interrupted\0".as_ptr() as *const ::core::ffi::c_char,
        b"disabled\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut ready_unavailable: [*const ::core::ffi::c_char; 1] =
        [b"write_failed\0".as_ptr() as *const ::core::ffi::c_char];
    static mut ready_expired: [*const ::core::ffi::c_char; 2] = [
        b"retention_expired\0".as_ptr() as *const ::core::ffi::c_char,
        b"deleted\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if from.0 == observation_asset_state_t::OBS_ASSET_PENDING.0
        && to.0 == observation_asset_state_t::OBS_ASSET_READY.0
    {
        return empty_text(reason);
    }
    if from.0 == observation_asset_state_t::OBS_ASSET_PENDING.0
        && to.0 == observation_asset_state_t::OBS_ASSET_UNAVAILABLE.0
    {
        return reason_in(
            reason,
            &raw const pending_unavailable as *const *const ::core::ffi::c_char,
            4 as size_t,
        );
    }
    if from.0 == observation_asset_state_t::OBS_ASSET_READY.0
        && to.0 == observation_asset_state_t::OBS_ASSET_UNAVAILABLE.0
    {
        return reason_in(
            reason,
            &raw const ready_unavailable as *const *const ::core::ffi::c_char,
            1 as size_t,
        );
    }
    if from.0 == observation_asset_state_t::OBS_ASSET_READY.0
        && to.0 == observation_asset_state_t::OBS_ASSET_EXPIRED.0
    {
        return reason_in(
            reason,
            &raw const ready_expired as *const *const ::core::ffi::c_char,
            2 as size_t,
        );
    }
    return r#false != 0;
}
unsafe extern "C" fn terminated(mut value: *const ::core::ffi::c_char, mut size: size_t) -> bool {
    return !value.is_null()
        && !memchr(
            value as *const ::core::ffi::c_void,
            '\0' as ::core::ffi::c_int,
            size,
        )
        .is_null();
}
unsafe extern "C" fn text_len_between(
    mut value: *const ::core::ffi::c_char,
    mut size: size_t,
    mut min: size_t,
    mut max: size_t,
) -> bool {
    if !terminated(value, size) {
        return r#false != 0;
    }
    let mut length: size_t = strlen(value);
    return length >= min && length <= max;
}
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
unsafe extern "C" fn copy_column(
    mut stmt: *mut sqlite3_stmt,
    mut column: ::core::ffi::c_int,
    mut out: *mut ::core::ffi::c_char,
    mut size: size_t,
) {
    let mut value: *const ::core::ffi::c_uchar = sqlite3_column_text(stmt, column);
    safe_strcpy(
        out,
        if !value.is_null() {
            value as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        size,
        0 as size_t,
    );
}
unsafe extern "C" fn bind_text_or_null(
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
unsafe extern "C" fn bind_positive_or_null(
    mut stmt: *mut sqlite3_stmt,
    mut index: ::core::ffi::c_int,
    mut value: int64_t,
) {
    if value > 0 as int64_t {
        sqlite3_bind_int64(stmt, index, value as sqlite3_int64);
    } else {
        sqlite3_bind_null(stmt, index);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_message_id(
    mut source: *const ::core::ffi::c_char,
    mut observation_uuid: *const ::core::ffi::c_char,
    mut revision: ::core::ffi::c_int,
    mut out: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if out.is_null()
        || empty_text(source) as ::core::ffi::c_int != 0
        || !lightnvr_uuid_is_valid(observation_uuid)
        || revision < 1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut name: [::core::ffi::c_char; 221] = [0; 221];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 221]>(),
        b"%s|%s|%d\0".as_ptr() as *const ::core::ffi::c_char,
        source,
        observation_uuid,
        revision,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 221]>()
    {
        return -1 as ::core::ffi::c_int;
    }
    return lightnvr_uuid_generate_v5(
        OBS_MESSAGE_ID_NAMESPACE.as_ptr(),
        &raw mut name as *mut ::core::ffi::c_char,
        written as size_t,
        out,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_cursor_encode(
    mut seq: int64_t,
    mut epoch_uuid: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_char,
    mut out_size: size_t,
) -> ::core::ffi::c_int {
    if out.is_null() || out_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    *out.offset(0isize) = '\0' as ::core::ffi::c_char;
    if seq < 0 as int64_t
        || seq as ::core::ffi::c_longlong > OBS_MAX_SAFE_SEQ
        || !lightnvr_uuid_is_valid(epoch_uuid)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut json: [::core::ffi::c_char; 128] = [0; 128];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        b"{\"v\":1,\"seq\":%lld,\"epoch\":\"%s\"}\0".as_ptr() as *const ::core::ffi::c_char,
        seq as ::core::ffi::c_longlong,
        epoch_uuid,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
    {
        return -1 as ::core::ffi::c_int;
    }
    return base64url_encode(
        &raw mut json as *mut ::core::ffi::c_char as *const ::core::ffi::c_uchar,
        written as size_t,
        out,
        out_size,
        ::core::ptr::null_mut::<size_t>(),
    );
}
unsafe extern "C" fn json_integer(
    mut item: *const cJSON,
    mut min: ::core::ffi::c_double,
    mut max: ::core::ffi::c_double,
    mut value: *mut int64_t,
) -> bool {
    if cJSON_IsNumber(item) == 0 {
        return r#false != 0;
    }
    let mut number: ::core::ffi::c_double = (*item).valuedouble;
    if number.is_finite() as ::core::ffi::c_int == 0
        || number < min
        || number > max
        || floor(number) != number
    {
        return r#false != 0;
    }
    *value = number as int64_t;
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn observation_cursor_decode(
    mut cursor: *const ::core::ffi::c_char,
    mut seq: *mut int64_t,
    mut epoch_uuid: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if cursor.is_null() || seq.is_null() || epoch_uuid.is_null() {
        return C2Rust_Unnamed::DB_OBS_INVALID_CURSOR.0;
    }
    let mut length: size_t = strnlen(cursor, OBS_CURSOR_SIZE as size_t);
    if length == 0 as size_t || length > (OBS_CURSOR_SIZE - 1 as ::core::ffi::c_int) as size_t {
        return C2Rust_Unnamed::DB_OBS_INVALID_CURSOR.0;
    }
    let mut json: [::core::ffi::c_uchar; 257] = [0; 257];
    let mut json_length: size_t = 0 as size_t;
    if base64url_decode(
        cursor,
        length,
        &raw mut json as *mut ::core::ffi::c_uchar,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 257]>().wrapping_sub(1 as size_t),
        &raw mut json_length,
    ) != 0 as ::core::ffi::c_int
        || !memchr(
            &raw mut json as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            '\0' as ::core::ffi::c_int,
            json_length,
        )
        .is_null()
    {
        return C2Rust_Unnamed::DB_OBS_INVALID_CURSOR.0;
    }
    json[json_length] = '\0' as ::core::ffi::c_uchar;
    let mut root: *mut cJSON = cJSON_ParseWithLength(
        &raw mut json as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_char,
        json_length,
    );
    let mut result: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_INVALID_CURSOR.0;
    let mut version: int64_t = 0 as int64_t;
    let mut value: int64_t = 0 as int64_t;
    let mut epoch: *const cJSON = if !root.is_null() {
        cJSON_GetObjectItemCaseSensitive(root, b"epoch\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if cJSON_IsObject(root) != 0
        && json_integer(
            cJSON_GetObjectItemCaseSensitive(root, b"v\0".as_ptr() as *const ::core::ffi::c_char),
            1 as ::core::ffi::c_int as ::core::ffi::c_double,
            1 as ::core::ffi::c_int as ::core::ffi::c_double,
            &raw mut version,
        ) as ::core::ffi::c_int
            != 0
        && json_integer(
            cJSON_GetObjectItemCaseSensitive(root, b"seq\0".as_ptr() as *const ::core::ffi::c_char),
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            OBS_MAX_SAFE_SEQ as ::core::ffi::c_double,
            &raw mut value,
        ) as ::core::ffi::c_int
            != 0
        && cJSON_IsString(epoch) != 0
        && lightnvr_uuid_is_valid((*epoch).valuestring) as ::core::ffi::c_int != 0
    {
        *seq = value;
        safe_strcpy(
            epoch_uuid,
            (*epoch).valuestring,
            OBS_UUID_SIZE as size_t,
            0 as size_t,
        );
        result = C2Rust_Unnamed::DB_OBS_OK.0;
    }
    cJSON_Delete(root);
    return result;
}
static mut publication_hook: db_observation_publication_hook_t = None;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observations_set_publication_hook(
    mut hook: db_observation_publication_hook_t,
) {
    ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::Release }>(
        &raw mut publication_hook as *mut *mut (),
        ::core::mem::transmute::<db_observation_publication_hook_t, *mut ()>(hook),
    );
}
unsafe extern "C" fn notify_publications_committed() {
    let mut hook: db_observation_publication_hook_t =
        ::core::mem::transmute::<*mut (), db_observation_publication_hook_t>(
            ::core::intrinsics::atomic_load::<_, { ::core::intrinsics::AtomicOrdering::Acquire }>(
                &raw mut publication_hook as *mut *mut (),
            ),
        );
    if hook.is_some() {
        hook.expect("non-null function pointer")();
    }
}
unsafe extern "C" fn tx_begin(mut tx: *mut obs_tx_t) -> ::core::ffi::c_int {
    (*tx).mutex = get_db_mutex();
    if (*tx).mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock((*tx).mutex);
    (*tx).db = get_db_handle();
    if (*tx).db.is_null() {
        pthread_mutex_unlock((*tx).mutex);
        return -1 as ::core::ffi::c_int;
    }
    (*tx).connection_mutex = sqlite3_db_mutex((*tx).db);
    sqlite3_mutex_enter((*tx).connection_mutex);
    if sqlite3_exec(
        (*tx).db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to begin observation transaction: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg((*tx).db),
        );
        sqlite3_mutex_leave((*tx).connection_mutex);
        pthread_mutex_unlock((*tx).mutex);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn tx_end(
    mut tx: *mut obs_tx_t,
    mut rc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if rc == C2Rust_Unnamed::DB_OBS_OK.0
        && sqlite3_exec(
            (*tx).db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) != SQLITE_OK
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to commit observation transaction: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg((*tx).db),
        );
        rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        sqlite3_exec(
            (*tx).db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    }
    sqlite3_mutex_leave((*tx).connection_mutex);
    pthread_mutex_unlock((*tx).mutex);
    return rc;
}
unsafe extern "C" fn get_source(mut source: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    if event_identity_get_source(source, OBS_SOURCE_SIZE as size_t) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Observation write needs the event installation identity\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn write_change_locked(
    mut db: *mut sqlite3,
    mut source: *const ::core::ffi::c_char,
    mut observation_uuid: *const ::core::ffi::c_char,
    mut revision: ::core::ffi::c_int,
    mut cause: observation_change_cause_t,
    mut evidence_state: *const ::core::ffi::c_char,
    mut evidence_reason: *const ::core::ffi::c_char,
    mut now_ms: int64_t,
    mut seq_out: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut message_id: [::core::ffi::c_char; 37] = [0; 37];
    if observation_message_id(
        source,
        observation_uuid,
        revision,
        &raw mut message_id as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        b"INSERT INTO observation_changes (observation_uuid, revision, cause, evidence_state, evidence_reason, created_at_ms) VALUES (?, ?, ?, ?, ?, ?);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            observation_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(stmt, 2 as ::core::ffi::c_int, revision);
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            observation_change_cause_name(cause),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        sqlite3_bind_text(
            stmt,
            4 as ::core::ffi::c_int,
            evidence_state,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        bind_text_or_null(stmt, 5 as ::core::ffi::c_int, evidence_reason);
        sqlite3_bind_int64(stmt, 6 as ::core::ffi::c_int, now_ms as sqlite3_int64);
        if sqlite3_step(stmt) == SQLITE_DONE {
            rc = C2Rust_Unnamed::DB_OBS_OK.0;
        }
    }
    sqlite3_finalize(stmt);
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to insert observation change: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return rc;
    }
    let mut seq: int64_t = sqlite3_last_insert_rowid(db) as int64_t;
    rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        b"INSERT INTO observation_publications (change_seq, message_id, state, attempts, updated_at_ms) VALUES (?, ?, 'pending', 0, ?);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, seq as sqlite3_int64);
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw mut message_id as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, now_ms as sqlite3_int64);
        if sqlite3_step(stmt) == SQLITE_DONE {
            rc = C2Rust_Unnamed::DB_OBS_OK.0;
        }
    }
    sqlite3_finalize(stmt);
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to insert observation publication: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return rc;
    }
    if !seq_out.is_null() {
        *seq_out = seq;
    }
    return C2Rust_Unnamed::DB_OBS_OK.0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observation_bump_revision_locked(
    mut db: *mut sqlite3,
    mut source: *const ::core::ffi::c_char,
    mut observation_uuid: *const ::core::ffi::c_char,
    mut cause: observation_change_cause_t,
    mut now_ms: int64_t,
    mut seq_out: *mut int64_t,
    mut revision_out: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if db.is_null()
        || empty_text(source) as ::core::ffi::c_int != 0
        || !lightnvr_uuid_is_valid(observation_uuid)
        || cause.0 != observation_change_cause_t::OBS_CAUSE_EVIDENCE.0
            && cause.0 != observation_change_cause_t::OBS_CAUSE_CLASSIFICATION.0
    {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut revision: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut state: [::core::ffi::c_char; 16] = ::core::mem::transmute::<
        [u8; 16],
        [::core::ffi::c_char; 16],
    >(*b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    let mut reason: [::core::ffi::c_char; 33] =
        ::core::mem::transmute::<[u8; 33], [::core::ffi::c_char; 33]>(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        b"SELECT o.revision, a.state, a.reason FROM object_observations o JOIN observation_assets a ON a.uuid = o.asset_uuid WHERE o.uuid = ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            observation_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        let mut step: ::core::ffi::c_int = sqlite3_step(stmt);
        if step == SQLITE_ROW {
            revision = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
            copy_column(
                stmt,
                1 as ::core::ffi::c_int,
                &raw mut state as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            );
            copy_column(
                stmt,
                2 as ::core::ffi::c_int,
                &raw mut reason as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 33]>(),
            );
            rc = C2Rust_Unnamed::DB_OBS_OK.0;
        } else if step == SQLITE_DONE {
            rc = C2Rust_Unnamed::DB_OBS_NOT_FOUND.0;
        }
    }
    sqlite3_finalize(stmt);
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        return rc;
    }
    rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        b"UPDATE object_observations SET revision = ? WHERE uuid = ? AND revision = ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int(
            stmt,
            1 as ::core::ffi::c_int,
            revision + 1 as ::core::ffi::c_int,
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            observation_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(stmt, 3 as ::core::ffi::c_int, revision);
        if sqlite3_step(stmt) == SQLITE_DONE && sqlite3_changes(db) == 1 as ::core::ffi::c_int {
            rc = C2Rust_Unnamed::DB_OBS_OK.0;
        }
    }
    sqlite3_finalize(stmt);
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        return rc;
    }
    rc = write_change_locked(
        db,
        source,
        observation_uuid,
        revision + 1 as ::core::ffi::c_int,
        cause,
        &raw mut state as *mut ::core::ffi::c_char,
        &raw mut reason as *mut ::core::ffi::c_char,
        now_ms,
        seq_out,
    );
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 && !revision_out.is_null() {
        *revision_out = revision + 1 as ::core::ffi::c_int;
    }
    return rc;
}
unsafe extern "C" fn round6(mut value: ::core::ffi::c_double) -> ::core::ffi::c_double {
    return round(value * 1e6f64) / 1e6f64;
}
unsafe extern "C" fn valid_box(mut object: *const observation_object_input_t) -> bool {
    if !(*object).has_box {
        return r#true != 0;
    }
    let mut x: ::core::ffi::c_double = (*object).box_x;
    let mut y: ::core::ffi::c_double = (*object).box_y;
    let mut w: ::core::ffi::c_double = (*object).box_width;
    let mut h: ::core::ffi::c_double = (*object).box_height;
    return x.is_finite() as ::core::ffi::c_int != 0
        && y.is_finite() as ::core::ffi::c_int != 0
        && w.is_finite() as ::core::ffi::c_int != 0
        && h.is_finite() as ::core::ffi::c_int != 0
        && x >= 0.0f64
        && y >= 0.0f64
        && w > 0.0f64
        && h > 0.0f64
        && x + w <= 1.000001f64
        && y + h <= 1.000001f64;
}
unsafe extern "C" fn valid_object(mut object: *const observation_object_input_t) -> bool {
    if (*object).object_index < 0 as ::core::ffi::c_int || (*object).object_index >= OBS_MAX_BATCH {
        return r#false != 0;
    }
    if !text_len_between(
        &raw const (*object).label as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        1 as size_t,
        (OBS_LABEL_SIZE - 1 as ::core::ffi::c_int) as size_t,
    ) {
        return r#false != 0;
    }
    if (*object).confidence.is_finite() as ::core::ffi::c_int == 0
        || (*object).confidence < 0.0f64
        || (*object).confidence > 1.0f64
    {
        return r#false != 0;
    }
    if !valid_box(object) {
        return r#false != 0;
    }
    if (*object).has_track as ::core::ffi::c_int != 0
        && (!text_len_between(
            &raw const (*object).track_id as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
            1 as size_t,
            64 as size_t,
        ) || !text_len_between(
            &raw const (*object).track_namespace as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
            1 as size_t,
            64 as size_t,
        ) || !lightnvr_uuid_is_valid(
            &raw const (*object).track_session as *const ::core::ffi::c_char,
        ))
    {
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_engine_type(mut r#type: *const ::core::ffi::c_char) -> bool {
    static mut types: [*const ::core::ffi::c_char; 4] = [
        b"object\0".as_ptr() as *const ::core::ffi::c_char,
        b"api\0".as_ptr() as *const ::core::ffi::c_char,
        b"onvif\0".as_ptr() as *const ::core::ffi::c_char,
        b"external\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    return reason_in(
        r#type,
        &raw const types as *const *const ::core::ffi::c_char,
        4 as size_t,
    );
}
unsafe extern "C" fn valid_batch(mut batch: *const observation_batch_input_t) -> bool {
    if batch.is_null()
        || !terminated(
            &raw const (*batch).batch_id as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        )
        || !lightnvr_uuid_is_valid(&raw const (*batch).batch_id as *const ::core::ffi::c_char)
        || !terminated(
            &raw const (*batch).camera_uuid as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        )
        || !lightnvr_uuid_is_valid(&raw const (*batch).camera_uuid as *const ::core::ffi::c_char)
        || !text_len_between(
            &raw const (*batch).stream_name as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
            63 as size_t,
        )
        || (*batch).observed_at_ms <= 0 as int64_t
        || !text_len_between(
            &raw const (*batch).engine_key as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
            1 as size_t,
            64 as size_t,
        )
        || !terminated(
            &raw const (*batch).engine_type as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        )
        || !valid_engine_type(&raw const (*batch).engine_type as *const ::core::ffi::c_char)
        || !text_len_between(
            &raw const (*batch).model as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 129]>(),
            0 as size_t,
            128 as size_t,
        )
        || !strchr(
            &raw const (*batch).model as *const ::core::ffi::c_char,
            '/' as ::core::ffi::c_int,
        )
        .is_null()
        || !strchr(
            &raw const (*batch).model as *const ::core::ffi::c_char,
            '\\' as ::core::ffi::c_int,
        )
        .is_null()
        || !text_len_between(
            &raw const (*batch).model_version as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
            0 as size_t,
            64 as size_t,
        )
        || (*batch).recording_id < 0 as int64_t
        || (*batch).object_count < 1 as ::core::ffi::c_int
        || (*batch).object_count > OBS_MAX_BATCH
    {
        return r#false != 0;
    }
    let mut used: [bool; 20] = [
        r#false != 0,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    ];
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*batch).object_count {
        let mut object: *const observation_object_input_t =
            (&raw const (*batch).objects as *const observation_object_input_t).offset(i as isize);
        if !valid_object(object) || used[(*object).object_index as usize] as ::core::ffi::c_int != 0
        {
            return r#false != 0;
        }
        used[(*object).object_index as usize] = r#true != 0;
        i += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_asset_input(mut asset: *const observation_asset_input_t) -> bool {
    static mut origins: [*const ::core::ffi::c_char; 2] = [
        b"inference_input\0".as_ptr() as *const ::core::ffi::c_char,
        b"submitted_image\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if asset.is_null()
        || !terminated(
            &raw const (*asset).asset_uuid as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        )
        || (*asset).asset_uuid[0usize] as ::core::ffi::c_int != 0
            && !lightnvr_uuid_is_valid(&raw const (*asset).asset_uuid as *const ::core::ffi::c_char)
        || !terminated(
            &raw const (*asset).reason as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 33]>(),
        )
        || !terminated(
            &raw const (*asset).origin as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        )
        || !terminated(
            &raw const (*asset).media_type as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        )
        || !terminated(
            &raw const (*asset).transform_json as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        )
        || (*asset).captured_at_ms < 0 as int64_t
        || (*asset).expires_at_ms < 0 as int64_t
    {
        return r#false != 0;
    }
    if (*asset).state.0 != observation_asset_state_t::OBS_ASSET_PENDING.0
        && (*asset).state.0 != observation_asset_state_t::OBS_ASSET_UNAVAILABLE.0
    {
        return r#false != 0;
    }
    if !observation_reason_is_valid(
        (*asset).state,
        &raw const (*asset).reason as *const ::core::ffi::c_char,
    ) {
        return r#false != 0;
    }
    if (*asset).origin[0usize] as ::core::ffi::c_int != 0
        && !reason_in(
            &raw const (*asset).origin as *const ::core::ffi::c_char,
            &raw const origins as *const *const ::core::ffi::c_char,
            2 as size_t,
        )
    {
        return r#false != 0;
    }
    if (*asset).media_type[0usize] as ::core::ffi::c_int != 0
        && strcmp(
            &raw const (*asset).media_type as *const ::core::ffi::c_char,
            b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn find_existing_batch(
    mut db: *mut sqlite3,
    mut batch: *const observation_batch_input_t,
    mut result: *mut observation_insert_result_t,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        b"SELECT uuid, asset_uuid, camera_uuid FROM object_observations WHERE batch_id = ? ORDER BY object_index;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        &raw const (*batch).batch_id as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_OK.0;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut step: ::core::ffi::c_int = 0;
    loop {
        step = sqlite3_step(stmt);
        if step != SQLITE_ROW {
            break;
        }
        let mut camera: [::core::ffi::c_char; 37] = [0; 37];
        copy_column(
            stmt,
            2 as ::core::ffi::c_int,
            &raw mut camera as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        );
        if strcmp(
            &raw mut camera as *mut ::core::ffi::c_char,
            &raw const (*batch).camera_uuid as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            rc = C2Rust_Unnamed::DB_OBS_CONFLICT.0;
            break;
        } else {
            if count < OBS_MAX_BATCH {
                copy_column(
                    stmt,
                    0 as ::core::ffi::c_int,
                    &raw mut *(&raw mut (*result).observation_uuids
                        as *mut [::core::ffi::c_char; 37])
                        .offset(count as isize) as *mut ::core::ffi::c_char,
                    OBS_UUID_SIZE as size_t,
                );
                if count == 0 as ::core::ffi::c_int {
                    copy_column(
                        stmt,
                        1 as ::core::ffi::c_int,
                        &raw mut (*result).asset_uuid as *mut ::core::ffi::c_char,
                        OBS_UUID_SIZE as size_t,
                    );
                }
                count += 1;
            }
            rc = C2Rust_Unnamed::DB_OBS_DUPLICATE.0;
        }
    }
    if step != SQLITE_ROW && step != SQLITE_DONE {
        rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    sqlite3_finalize(stmt);
    if rc == C2Rust_Unnamed::DB_OBS_DUPLICATE.0 {
        (*result).object_count = count;
    }
    return rc;
}
unsafe extern "C" fn resolve_stream_name(
    mut db: *mut sqlite3,
    mut camera_uuid: *const ::core::ffi::c_char,
    mut name: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        b"SELECT name FROM streams WHERE camera_uuid = ?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        camera_uuid,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    let mut step: ::core::ffi::c_int = sqlite3_step(stmt);
    let mut rc: ::core::ffi::c_int = if step == SQLITE_ROW {
        C2Rust_Unnamed::DB_OBS_OK.0
    } else if step == SQLITE_DONE {
        C2Rust_Unnamed::DB_OBS_NOT_FOUND.0
    } else {
        C2Rust_Unnamed::DB_OBS_ERROR.0
    };
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        copy_column(stmt, 0 as ::core::ffi::c_int, name, size);
    }
    sqlite3_finalize(stmt);
    return rc;
}
unsafe extern "C" fn insert_asset_row(
    mut db: *mut sqlite3,
    mut asset_uuid: *const ::core::ffi::c_char,
    mut batch: *const observation_batch_input_t,
    mut asset: *const observation_asset_input_t,
    mut now_ms: int64_t,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        b"INSERT INTO observation_assets (uuid, camera_uuid, state, reason, origin, media_type, captured_at_ms, transform_json, protected, expires_at_ms, created_at_ms, updated_at_ms) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?, ?);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            asset_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw const (*batch).camera_uuid as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            observation_asset_state_name((*asset).state),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        bind_text_or_null(
            stmt,
            4 as ::core::ffi::c_int,
            &raw const (*asset).reason as *const ::core::ffi::c_char,
        );
        bind_text_or_null(
            stmt,
            5 as ::core::ffi::c_int,
            &raw const (*asset).origin as *const ::core::ffi::c_char,
        );
        bind_text_or_null(
            stmt,
            6 as ::core::ffi::c_int,
            &raw const (*asset).media_type as *const ::core::ffi::c_char,
        );
        sqlite3_bind_int64(
            stmt,
            7 as ::core::ffi::c_int,
            (if (*asset).captured_at_ms > 0 as int64_t {
                (*asset).captured_at_ms
            } else {
                (*batch).observed_at_ms
            }) as sqlite3_int64,
        );
        bind_text_or_null(
            stmt,
            8 as ::core::ffi::c_int,
            &raw const (*asset).transform_json as *const ::core::ffi::c_char,
        );
        bind_positive_or_null(stmt, 9 as ::core::ffi::c_int, (*asset).expires_at_ms);
        sqlite3_bind_int64(stmt, 10 as ::core::ffi::c_int, now_ms as sqlite3_int64);
        sqlite3_bind_int64(stmt, 11 as ::core::ffi::c_int, now_ms as sqlite3_int64);
        if sqlite3_step(stmt) == SQLITE_DONE {
            rc = C2Rust_Unnamed::DB_OBS_OK.0;
        }
    }
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to insert observation asset: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    sqlite3_finalize(stmt);
    return rc;
}
unsafe extern "C" fn insert_observation_row(
    mut db: *mut sqlite3,
    mut uuid: *const ::core::ffi::c_char,
    mut asset_uuid: *const ::core::ffi::c_char,
    mut stream_name: *const ::core::ffi::c_char,
    mut batch: *const observation_batch_input_t,
    mut object: *const observation_object_input_t,
    mut now_ms: int64_t,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        b"INSERT OR IGNORE INTO object_observations (uuid, camera_uuid, stream_name_at_capture, batch_id, object_index, observed_at_ms, created_at_ms, engine_key, engine_type, model, model_version, label, confidence, box_x, box_y, box_width, box_height, track_id, track_namespace, track_session, recording_id, asset_uuid, revision) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
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
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw const (*batch).camera_uuid as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            stream_name,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            4 as ::core::ffi::c_int,
            &raw const (*batch).batch_id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int(stmt, 5 as ::core::ffi::c_int, (*object).object_index);
        sqlite3_bind_int64(
            stmt,
            6 as ::core::ffi::c_int,
            (*batch).observed_at_ms as sqlite3_int64,
        );
        sqlite3_bind_int64(stmt, 7 as ::core::ffi::c_int, now_ms as sqlite3_int64);
        sqlite3_bind_text(
            stmt,
            8 as ::core::ffi::c_int,
            &raw const (*batch).engine_key as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            9 as ::core::ffi::c_int,
            &raw const (*batch).engine_type as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        bind_text_or_null(
            stmt,
            10 as ::core::ffi::c_int,
            &raw const (*batch).model as *const ::core::ffi::c_char,
        );
        bind_text_or_null(
            stmt,
            11 as ::core::ffi::c_int,
            &raw const (*batch).model_version as *const ::core::ffi::c_char,
        );
        sqlite3_bind_text(
            stmt,
            12 as ::core::ffi::c_int,
            &raw const (*object).label as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_double(stmt, 13 as ::core::ffi::c_int, (*object).confidence);
        if (*object).has_box {
            sqlite3_bind_double(stmt, 14 as ::core::ffi::c_int, round6((*object).box_x));
            sqlite3_bind_double(stmt, 15 as ::core::ffi::c_int, round6((*object).box_y));
            sqlite3_bind_double(
                stmt,
                16 as ::core::ffi::c_int,
                round6((*object).box_width),
            );
            sqlite3_bind_double(
                stmt,
                17 as ::core::ffi::c_int,
                round6((*object).box_height),
            );
        } else {
            let mut i: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
            while i <= 17 as ::core::ffi::c_int {
                sqlite3_bind_null(stmt, i);
                i += 1;
            }
        }
        if (*object).has_track {
            sqlite3_bind_text(
                stmt,
                18 as ::core::ffi::c_int,
                &raw const (*object).track_id as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                stmt,
                19 as ::core::ffi::c_int,
                &raw const (*object).track_namespace as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                stmt,
                20 as ::core::ffi::c_int,
                &raw const (*object).track_session as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            let mut i_0: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
            while i_0 <= 20 as ::core::ffi::c_int {
                sqlite3_bind_null(stmt, i_0);
                i_0 += 1;
            }
        }
        bind_positive_or_null(stmt, 21 as ::core::ffi::c_int, (*batch).recording_id);
        sqlite3_bind_text(
            stmt,
            22 as ::core::ffi::c_int,
            asset_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if sqlite3_step(stmt) == SQLITE_DONE
            && sqlite3_changes(db) == 1 as ::core::ffi::c_int
        {
            rc = C2Rust_Unnamed::DB_OBS_OK.0;
        }
    }
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to insert observation: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
    }
    sqlite3_finalize(stmt);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observation_insert_batch(
    mut batch: *const observation_batch_input_t,
    mut asset: *const observation_asset_input_t,
    mut result: *mut observation_insert_result_t,
) -> ::core::ffi::c_int {
    if result.is_null() {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_insert_result_t>(),
    );
    if !valid_batch(batch) || !valid_asset_input(asset) {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    let mut source: [::core::ffi::c_char; 160] = [0; 160];
    if get_source(&raw mut source as *mut ::core::ffi::c_char) != 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut asset_uuid: [::core::ffi::c_char; 37] = [0; 37];
    let mut uuids: [[::core::ffi::c_char; 37]; 20] = [[0; 37]; 20];
    if (*asset).asset_uuid[0usize] != 0 {
        safe_strcpy(
            &raw mut asset_uuid as *mut ::core::ffi::c_char,
            &raw const (*asset).asset_uuid as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        let mut c: *mut ::core::ffi::c_char = &raw mut asset_uuid as *mut ::core::ffi::c_char;
        while *c != 0 {
            if *c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && *c as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
            {
                *c = (*c as ::core::ffi::c_int - 'A' as ::core::ffi::c_int
                    + 'a' as ::core::ffi::c_int) as ::core::ffi::c_char;
            }
            c = c.offset(1);
        }
    } else if lightnvr_uuid_generate_v4(&raw mut asset_uuid as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*batch).object_count {
        if lightnvr_uuid_generate_v4(
            &raw mut *(&raw mut uuids as *mut [::core::ffi::c_char; 37]).offset(i as isize)
                as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            return C2Rust_Unnamed::DB_OBS_ERROR.0;
        }
        i += 1;
    }
    let mut order: [::core::ffi::c_int; 20] = [0; 20];
    let mut ordered: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < OBS_MAX_BATCH {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*batch).object_count {
            if (*batch).objects[i_0 as usize].object_index == index {
                let c2rust_fresh0 = ordered;
                ordered += 1;
                order[c2rust_fresh0 as usize] = i_0;
            }
            i_0 += 1;
        }
        index += 1;
    }
    let mut now_ms: int64_t = current_time_ms();
    let mut tx: obs_tx_t = obs_tx_t {
        mutex: ::core::ptr::null_mut::<pthread_mutex_t>(),
        db: ::core::ptr::null_mut::<sqlite3>(),
        connection_mutex: ::core::ptr::null_mut::<sqlite3_mutex>(),
    };
    if tx_begin(&raw mut tx) != 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut rc: ::core::ffi::c_int = find_existing_batch(tx.db, batch, result);
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        let mut final_rc: ::core::ffi::c_int = rc;
        tx_end(&raw mut tx, C2Rust_Unnamed::DB_OBS_ERROR.0);
        if final_rc != C2Rust_Unnamed::DB_OBS_DUPLICATE.0 {
            memset(
                result as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<observation_insert_result_t>(),
            );
        }
        return final_rc;
    }
    let mut stream_name: [::core::ffi::c_char; 64] = [0; 64];
    rc = resolve_stream_name(
        tx.db,
        &raw const (*batch).camera_uuid as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 && (*batch).stream_name[0usize] as ::core::ffi::c_int != 0
    {
        safe_strcpy(
            &raw mut stream_name as *mut ::core::ffi::c_char,
            &raw const (*batch).stream_name as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        rc = insert_asset_row(
            tx.db,
            &raw mut asset_uuid as *mut ::core::ffi::c_char,
            batch,
            asset,
            now_ms,
        );
    }
    let mut state: *const ::core::ffi::c_char = observation_asset_state_name((*asset).state);
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while rc == C2Rust_Unnamed::DB_OBS_OK.0 && n < ordered {
        let mut i_1: ::core::ffi::c_int = order[n as usize];
        rc = insert_observation_row(
            tx.db,
            &raw mut *(&raw mut uuids as *mut [::core::ffi::c_char; 37]).offset(i_1 as isize)
                as *mut ::core::ffi::c_char,
            &raw mut asset_uuid as *mut ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
            batch,
            (&raw const (*batch).objects as *const observation_object_input_t).offset(i_1 as isize),
            now_ms,
        );
        if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
            rc = write_change_locked(
                tx.db,
                &raw mut source as *mut ::core::ffi::c_char,
                &raw mut *(&raw mut uuids as *mut [::core::ffi::c_char; 37]).offset(i_1 as isize)
                    as *mut ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                observation_change_cause_t::OBS_CAUSE_CREATED,
                state,
                &raw const (*asset).reason as *const ::core::ffi::c_char,
                now_ms,
                &raw mut (*result).last_seq,
            );
        }
        if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
            safe_strcpy(
                &raw mut *(&raw mut (*result).observation_uuids as *mut [::core::ffi::c_char; 37])
                    .offset(n as isize) as *mut ::core::ffi::c_char,
                &raw mut *(&raw mut uuids as *mut [::core::ffi::c_char; 37]).offset(i_1 as isize)
                    as *mut ::core::ffi::c_char,
                OBS_UUID_SIZE as size_t,
                0 as size_t,
            );
        }
        n += 1;
    }
    rc = tx_end(&raw mut tx, rc);
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        safe_strcpy(
            &raw mut (*result).asset_uuid as *mut ::core::ffi::c_char,
            &raw mut asset_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        (*result).object_count = ordered;
        if ordered > 0 as ::core::ffi::c_int {
            notify_publications_committed();
        }
    } else {
        memset(
            result as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<observation_insert_result_t>(),
        );
    }
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observation_asset_transition(
    mut asset_uuid: *const ::core::ffi::c_char,
    mut from: observation_asset_state_t,
    mut to: observation_asset_state_t,
    mut reason: *const ::core::ffi::c_char,
    mut media: *const observation_asset_media_t,
    mut result: *mut observation_transition_result_t,
) -> ::core::ffi::c_int {
    if !result.is_null() {
        memset(
            result as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<observation_transition_result_t>(),
        );
    }
    if !lightnvr_uuid_is_valid(asset_uuid) || !transition_allowed(from, to, reason) {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    if to.0 == observation_asset_state_t::OBS_ASSET_READY.0
        && (media.is_null()
            || empty_text((*media).path) as ::core::ffi::c_int != 0
            || strlen((*media).path) >= MAX_PATH_LENGTH as size_t
            || (*media).bytes < 0 as int64_t
            || (*media).width <= 0 as ::core::ffi::c_int
            || (*media).height <= 0 as ::core::ffi::c_int)
    {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    if !media.is_null()
        && ((*media).bytes < 0 as int64_t
            || (*media).width < 0 as ::core::ffi::c_int
            || (*media).height < 0 as ::core::ffi::c_int
            || (*media).expires_at_ms < 0 as int64_t)
    {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    let mut source: [::core::ffi::c_char; 160] = [0; 160];
    if get_source(&raw mut source as *mut ::core::ffi::c_char) != 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut now_ms: int64_t = current_time_ms();
    let mut tx: obs_tx_t = obs_tx_t {
        mutex: ::core::ptr::null_mut::<pthread_mutex_t>(),
        db: ::core::ptr::null_mut::<sqlite3>(),
        connection_mutex: ::core::ptr::null_mut::<sqlite3_mutex>(),
    };
    if tx_begin(&raw mut tx) != 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        tx.db,
        b"UPDATE observation_assets SET state = ?, reason = ?, path = COALESCE(?, path), bytes = COALESCE(?, bytes), width = COALESCE(?, width), height = COALESCE(?, height), expires_at_ms = COALESCE(?, expires_at_ms), updated_at_ms = ? WHERE uuid = ? AND state = ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            observation_asset_state_name(to),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        bind_text_or_null(stmt, 2 as ::core::ffi::c_int, reason);
        bind_text_or_null(
            stmt,
            3 as ::core::ffi::c_int,
            if !media.is_null() {
                (*media).path
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            },
        );
        if !media.is_null() && to.0 == observation_asset_state_t::OBS_ASSET_READY.0 {
            sqlite3_bind_int64(
                stmt,
                4 as ::core::ffi::c_int,
                (*media).bytes as sqlite3_int64,
            );
        } else {
            sqlite3_bind_null(stmt, 4 as ::core::ffi::c_int);
        }
        bind_positive_or_null(
            stmt,
            5 as ::core::ffi::c_int,
            (if !media.is_null() { (*media).width } else { 0 as ::core::ffi::c_int })
                as int64_t,
        );
        bind_positive_or_null(
            stmt,
            6 as ::core::ffi::c_int,
            (if !media.is_null() { (*media).height } else { 0 as ::core::ffi::c_int })
                as int64_t,
        );
        bind_positive_or_null(
            stmt,
            7 as ::core::ffi::c_int,
            if !media.is_null() { (*media).expires_at_ms } else { 0 as int64_t },
        );
        sqlite3_bind_int64(stmt, 8 as ::core::ffi::c_int, now_ms as sqlite3_int64);
        sqlite3_bind_text(
            stmt,
            9 as ::core::ffi::c_int,
            asset_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            10 as ::core::ffi::c_int,
            observation_asset_state_name(from),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        if sqlite3_step(stmt) == SQLITE_DONE {
            rc = C2Rust_Unnamed::DB_OBS_OK.0;
        }
    }
    sqlite3_finalize(stmt);
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 && sqlite3_changes(tx.db) == 0 as ::core::ffi::c_int {
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
        if sqlite3_prepare_v2(
            tx.db,
            b"SELECT 1 FROM observation_assets WHERE uuid = ?;\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
        {
            sqlite3_bind_text(
                stmt,
                1 as ::core::ffi::c_int,
                asset_uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            let mut step: ::core::ffi::c_int = sqlite3_step(stmt);
            if step == SQLITE_ROW {
                rc = C2Rust_Unnamed::DB_OBS_CAS_LOST.0;
            } else if step == SQLITE_DONE {
                rc = C2Rust_Unnamed::DB_OBS_NOT_FOUND.0;
            }
        }
        sqlite3_finalize(stmt);
        tx_end(&raw mut tx, C2Rust_Unnamed::DB_OBS_ERROR.0);
        return rc;
    }
    let mut observations: *mut [::core::ffi::c_char; 37] =
        ::core::ptr::null_mut::<[::core::ffi::c_char; 37]>();
    let mut count: size_t = 0 as size_t;
    let mut capacity: size_t = 0 as size_t;
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
        if sqlite3_prepare_v2(
            tx.db,
            b"SELECT uuid FROM object_observations WHERE asset_uuid = ? ORDER BY batch_id, object_index;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
        {
            sqlite3_bind_text(
                stmt,
                1 as ::core::ffi::c_int,
                asset_uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            let mut step_0: ::core::ffi::c_int = 0;
            rc = C2Rust_Unnamed::DB_OBS_OK.0;
            loop {
                step_0 = sqlite3_step(stmt);
                if step_0 != SQLITE_ROW {
                    break;
                }
                if count == capacity {
                    let mut next: size_t = if capacity != 0 {
                        capacity.wrapping_mul(2 as size_t)
                    } else {
                        OBS_MAX_BATCH as size_t
                    };
                    let mut grown: *mut ::core::ffi::c_void = realloc(
                        observations as *mut ::core::ffi::c_void,
                        next
                            .wrapping_mul(
                                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                            ),
                    );
                    if grown.is_null() {
                        rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
                        break;
                    } else {
                        observations = grown as *mut [::core::ffi::c_char; 37];
                        capacity = next;
                    }
                }
                let c2rust_fresh1 = count;
                count = count.wrapping_add(1);
                copy_column(
                    stmt,
                    0 as ::core::ffi::c_int,
                    &raw mut *observations.offset(c2rust_fresh1 as isize)
                        as *mut ::core::ffi::c_char,
                    OBS_UUID_SIZE as size_t,
                );
            }
            if rc == C2Rust_Unnamed::DB_OBS_OK.0 && step_0 != SQLITE_DONE {
                rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
            }
        }
        sqlite3_finalize(stmt);
    }
    let mut last_seq: int64_t = 0 as int64_t;
    let mut i: size_t = 0 as size_t;
    while rc == C2Rust_Unnamed::DB_OBS_OK.0 && i < count {
        rc = db_observation_bump_revision_locked(
            tx.db,
            &raw mut source as *mut ::core::ffi::c_char,
            &raw mut *observations.offset(i as isize) as *mut ::core::ffi::c_char,
            observation_change_cause_t::OBS_CAUSE_EVIDENCE,
            now_ms,
            &raw mut last_seq,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        i = i.wrapping_add(1);
    }
    free(observations as *mut ::core::ffi::c_void);
    rc = tx_end(&raw mut tx, rc);
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 && !result.is_null() {
        (*result).observations_bumped = count as ::core::ffi::c_int;
        (*result).last_seq = last_seq;
    }
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 && count > 0 as size_t {
        notify_publications_committed();
    }
    return rc;
}
unsafe extern "C" fn ro_begin(mut db: *mut *mut sqlite3) -> ::core::ffi::c_int {
    if db_open_readonly_connection(db) != 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    if sqlite3_exec(
        *db,
        b"BEGIN;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        db_close_readonly_connection(*db);
        *db = ::core::ptr::null_mut::<sqlite3>();
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    return C2Rust_Unnamed::DB_OBS_OK.0;
}
unsafe extern "C" fn ro_end(mut db: *mut sqlite3) {
    if db.is_null() {
        return;
    }
    sqlite3_exec(
        db,
        b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    db_close_readonly_connection(db);
}
pub const RECORD_SELECT: [::core::ffi::c_char; 688] = unsafe {
    ::core::mem::transmute::<
        [u8; 688],
        [::core::ffi::c_char; 688],
    >(
        *b"SELECT o.uuid, o.camera_uuid, o.stream_name_at_capture, o.batch_id, o.object_index, o.observed_at_ms, o.created_at_ms, o.engine_key, o.engine_type, o.model, o.model_version, o.label, o.confidence, o.box_x, o.box_y, o.box_width, o.box_height, o.track_id, o.track_namespace, o.track_session, o.recording_id, o.revision, a.uuid, a.camera_uuid, a.state, a.reason, a.origin, a.media_type, a.path, a.bytes, a.width, a.height, a.captured_at_ms, a.transform_json, a.protected, a.expires_at_ms, a.created_at_ms, a.updated_at_ms FROM object_observations o JOIN observation_assets a ON a.uuid = o.asset_uuid WHERE o.uuid = ? AND EXISTS (SELECT 1 FROM streams s WHERE s.camera_uuid = o.camera_uuid);\0",
    )
};
unsafe extern "C" fn read_asset(
    mut stmt: *mut sqlite3_stmt,
    mut c: ::core::ffi::c_int,
    mut asset: *mut observation_asset_record_t,
) {
    memset(
        asset as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_asset_record_t>(),
    );
    copy_column(
        stmt,
        c + 0 as ::core::ffi::c_int,
        &raw mut (*asset).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
    );
    copy_column(
        stmt,
        c + 1 as ::core::ffi::c_int,
        &raw mut (*asset).camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
    );
    let mut state: [::core::ffi::c_char; 16] = [0; 16];
    copy_column(
        stmt,
        c + 2 as ::core::ffi::c_int,
        &raw mut state as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
    );
    if observation_asset_state_parse(
        &raw mut state as *mut ::core::ffi::c_char,
        &raw mut (*asset).state,
    ) != 0 as ::core::ffi::c_int
    {
        (*asset).state = observation_asset_state_t::OBS_ASSET_UNAVAILABLE;
    }
    copy_column(
        stmt,
        c + 3 as ::core::ffi::c_int,
        &raw mut (*asset).reason as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 33]>(),
    );
    copy_column(
        stmt,
        c + 4 as ::core::ffi::c_int,
        &raw mut (*asset).origin as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
    );
    copy_column(
        stmt,
        c + 5 as ::core::ffi::c_int,
        &raw mut (*asset).media_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
    );
    copy_column(
        stmt,
        c + 6 as ::core::ffi::c_int,
        &raw mut (*asset).path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    );
    (*asset).has_bytes = sqlite3_column_type(stmt, c + 7 as ::core::ffi::c_int) != SQLITE_NULL;
    (*asset).bytes = sqlite3_column_int64(stmt, c + 7 as ::core::ffi::c_int) as int64_t;
    (*asset).width = sqlite3_column_int(stmt, c + 8 as ::core::ffi::c_int);
    (*asset).height = sqlite3_column_int(stmt, c + 9 as ::core::ffi::c_int);
    (*asset).captured_at_ms = sqlite3_column_int64(stmt, c + 10 as ::core::ffi::c_int) as int64_t;
    copy_column(
        stmt,
        c + 11 as ::core::ffi::c_int,
        &raw mut (*asset).transform_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    );
    (*asset).is_protected =
        sqlite3_column_int(stmt, c + 12 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
    (*asset).expires_at_ms = sqlite3_column_int64(stmt, c + 13 as ::core::ffi::c_int) as int64_t;
    (*asset).created_at_ms = sqlite3_column_int64(stmt, c + 14 as ::core::ffi::c_int) as int64_t;
    (*asset).updated_at_ms = sqlite3_column_int64(stmt, c + 15 as ::core::ffi::c_int) as int64_t;
}
unsafe extern "C" fn read_record(mut stmt: *mut sqlite3_stmt, mut out: *mut observation_record_t) {
    memset(
        out as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_record_t>(),
    );
    copy_column(
        stmt,
        0 as ::core::ffi::c_int,
        &raw mut (*out).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
    );
    copy_column(
        stmt,
        1 as ::core::ffi::c_int,
        &raw mut (*out).camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
    );
    copy_column(
        stmt,
        2 as ::core::ffi::c_int,
        &raw mut (*out).stream_name_at_capture as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    copy_column(
        stmt,
        3 as ::core::ffi::c_int,
        &raw mut (*out).batch_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
    );
    (*out).object_index = sqlite3_column_int(stmt, 4 as ::core::ffi::c_int);
    (*out).observed_at_ms = sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as int64_t;
    (*out).created_at_ms = sqlite3_column_int64(stmt, 6 as ::core::ffi::c_int) as int64_t;
    copy_column(
        stmt,
        7 as ::core::ffi::c_int,
        &raw mut (*out).engine_key as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    );
    copy_column(
        stmt,
        8 as ::core::ffi::c_int,
        &raw mut (*out).engine_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
    );
    copy_column(
        stmt,
        9 as ::core::ffi::c_int,
        &raw mut (*out).model as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 129]>(),
    );
    copy_column(
        stmt,
        10 as ::core::ffi::c_int,
        &raw mut (*out).model_version as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    );
    copy_column(
        stmt,
        11 as ::core::ffi::c_int,
        &raw mut (*out).label as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    );
    (*out).confidence = sqlite3_column_double(stmt, 12 as ::core::ffi::c_int);
    (*out).has_box = sqlite3_column_type(stmt, 13 as ::core::ffi::c_int) != SQLITE_NULL;
    if (*out).has_box {
        (*out).box_x = sqlite3_column_double(stmt, 13 as ::core::ffi::c_int);
        (*out).box_y = sqlite3_column_double(stmt, 14 as ::core::ffi::c_int);
        (*out).box_width = sqlite3_column_double(stmt, 15 as ::core::ffi::c_int);
        (*out).box_height = sqlite3_column_double(stmt, 16 as ::core::ffi::c_int);
    }
    (*out).has_track = sqlite3_column_type(stmt, 17 as ::core::ffi::c_int) != SQLITE_NULL;
    copy_column(
        stmt,
        17 as ::core::ffi::c_int,
        &raw mut (*out).track_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    );
    copy_column(
        stmt,
        18 as ::core::ffi::c_int,
        &raw mut (*out).track_namespace as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    );
    copy_column(
        stmt,
        19 as ::core::ffi::c_int,
        &raw mut (*out).track_session as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
    );
    (*out).recording_id = sqlite3_column_int64(stmt, 20 as ::core::ffi::c_int) as int64_t;
    (*out).revision = sqlite3_column_int(stmt, 21 as ::core::ffi::c_int);
    read_asset(stmt, 22 as ::core::ffi::c_int, &raw mut (*out).asset);
}
unsafe extern "C" fn get_record_on(
    mut db: *mut sqlite3,
    mut cached: *mut *mut sqlite3_stmt,
    mut uuid: *const ::core::ffi::c_char,
    mut out: *mut observation_record_t,
) -> ::core::ffi::c_int {
    if (*cached).is_null()
        && sqlite3_prepare_v2(
            db,
            RECORD_SELECT.as_ptr(),
            -1 as ::core::ffi::c_int,
            cached,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) != SQLITE_OK
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare observation read: %s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    sqlite3_reset(*cached);
    sqlite3_clear_bindings(*cached);
    sqlite3_bind_text(
        *cached,
        1 as ::core::ffi::c_int,
        uuid,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    let mut step: ::core::ffi::c_int = sqlite3_step(*cached);
    if step == SQLITE_ROW {
        read_record(*cached, out);
        return C2Rust_Unnamed::DB_OBS_OK.0;
    }
    return if step == SQLITE_DONE {
        C2Rust_Unnamed::DB_OBS_NOT_FOUND.0
    } else {
        C2Rust_Unnamed::DB_OBS_ERROR.0
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observation_get(
    mut observation_uuid: *const ::core::ffi::c_char,
    mut out: *mut observation_record_t,
) -> ::core::ffi::c_int {
    if out.is_null() || !lightnvr_uuid_is_valid(observation_uuid) {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if ro_begin(&raw mut db) != C2Rust_Unnamed::DB_OBS_OK.0 {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = get_record_on(db, &raw mut stmt, observation_uuid, out);
    sqlite3_finalize(stmt);
    ro_end(db);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observation_asset_get(
    mut asset_uuid: *const ::core::ffi::c_char,
    mut out: *mut observation_asset_record_t,
) -> ::core::ffi::c_int {
    if out.is_null() || !lightnvr_uuid_is_valid(asset_uuid) {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if ro_begin(&raw mut db) != C2Rust_Unnamed::DB_OBS_OK.0 {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        b"SELECT a.uuid, a.camera_uuid, a.state, a.reason, a.origin, a.media_type, a.path, a.bytes, a.width, a.height, a.captured_at_ms, a.transform_json, a.protected, a.expires_at_ms, a.created_at_ms, a.updated_at_ms FROM observation_assets a WHERE a.uuid = ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            asset_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        let mut step: ::core::ffi::c_int = sqlite3_step(stmt);
        if step == SQLITE_ROW {
            read_asset(stmt, 0 as ::core::ffi::c_int, out);
            rc = C2Rust_Unnamed::DB_OBS_OK.0;
        } else if step == SQLITE_DONE {
            rc = C2Rust_Unnamed::DB_OBS_NOT_FOUND.0;
        }
    }
    sqlite3_finalize(stmt);
    ro_end(db);
    return rc;
}
unsafe extern "C" fn read_meta_on(
    mut db: *mut sqlite3,
    mut meta: *mut observation_meta_t,
) -> ::core::ffi::c_int {
    memset(
        meta as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_meta_t>(),
    );
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        b"SELECT m.epoch_uuid, m.oldest_seq, (SELECT MAX(seq) FROM observation_changes) FROM observation_meta m WHERE m.id = 1;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW
    {
        copy_column(
            stmt,
            0 as ::core::ffi::c_int,
            &raw mut (*meta).epoch_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        );
        (*meta).oldest_seq = sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int)
            as int64_t;
        let mut max_seq: int64_t = sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int)
            as int64_t;
        (*meta).head_seq = if max_seq > (*meta).oldest_seq - 1 as int64_t {
            max_seq
        } else {
            (*meta).oldest_seq - 1 as int64_t
        };
        rc = if lightnvr_uuid_is_valid(
            &raw mut (*meta).epoch_uuid as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_int != 0
        {
            C2Rust_Unnamed::DB_OBS_OK.0
        } else {
            C2Rust_Unnamed::DB_OBS_ERROR.0
        };
    }
    sqlite3_finalize(stmt);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observation_meta_get(
    mut out: *mut observation_meta_t,
) -> ::core::ffi::c_int {
    if out.is_null() {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if ro_begin(&raw mut db) != C2Rust_Unnamed::DB_OBS_OK.0 {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut rc: ::core::ffi::c_int = read_meta_on(db, out);
    ro_end(db);
    return rc;
}
unsafe extern "C" fn diag_scalar(
    mut db: *mut sqlite3,
    mut sql: *const ::core::ffi::c_char,
    mut value: *mut int64_t,
    mut is_null: *mut bool,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        let mut step: ::core::ffi::c_int = sqlite3_step(stmt);
        if step == SQLITE_ROW {
            if !is_null.is_null() {
                *is_null = sqlite3_column_type(stmt, 0 as ::core::ffi::c_int) == SQLITE_NULL;
            }
            *value = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
            rc = C2Rust_Unnamed::DB_OBS_OK.0;
        } else if step == SQLITE_DONE {
            if !is_null.is_null() {
                *is_null = r#true != 0;
            }
            *value = 0 as int64_t;
            rc = C2Rust_Unnamed::DB_OBS_OK.0;
        }
    }
    sqlite3_finalize(stmt);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observation_diagnostics_get(
    mut out: *mut observation_db_diagnostics_t,
) -> ::core::ffi::c_int {
    if out.is_null() {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    memset(
        out as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_db_diagnostics_t>(),
    );
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if ro_begin(&raw mut db) != C2Rust_Unnamed::DB_OBS_OK.0 {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut oldest_null: bool = r#true != 0;
    let mut rc: ::core::ffi::c_int = read_meta_on(db, &raw mut (*out).meta);
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        rc = diag_scalar(
            db,
            b"SELECT MIN(observed_at_ms) FROM object_observations;\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*out).oldest_observed_at_ms,
            &raw mut oldest_null,
        );
    }
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        rc = diag_scalar(
            db,
            b"SELECT COUNT(*) FROM object_observations;\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*out).observations,
            ::core::ptr::null_mut::<bool>(),
        );
    }
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        rc = diag_scalar(
            db,
            b"SELECT COUNT(*) FROM observation_publications WHERE state = 'pending';\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*out).publications_pending,
            ::core::ptr::null_mut::<bool>(),
        );
    }
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        rc = diag_scalar(
            db,
            b"SELECT COUNT(*) FROM observation_publications WHERE state = 'failed';\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*out).publications_failed,
            ::core::ptr::null_mut::<bool>(),
        );
    }
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        rc = diag_scalar(
            db,
            b"SELECT COUNT(*) FROM observation_publications WHERE state = 'expired';\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*out).publications_expired,
            ::core::ptr::null_mut::<bool>(),
        );
    }
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        rc = diag_scalar(
            db,
            b"SELECT c.created_at_ms FROM observation_publications p JOIN observation_changes c ON c.seq = p.change_seq WHERE p.state = 'pending' ORDER BY p.change_seq LIMIT 1;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*out).oldest_pending_created_at_ms,
            ::core::ptr::null_mut::<bool>(),
        );
    }
    ro_end(db);
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        memset(
            out as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<observation_db_diagnostics_t>(),
        );
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    (*out).has_oldest_observed_at = !oldest_null;
    if oldest_null {
        (*out).oldest_observed_at_ms = 0 as int64_t;
    }
    return C2Rust_Unnamed::DB_OBS_OK.0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observation_meta_regenerate_epoch() -> ::core::ffi::c_int {
    let mut epoch: [::core::ffi::c_char; 37] = [0; 37];
    if lightnvr_uuid_generate_v4(&raw mut epoch as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut tx: obs_tx_t = obs_tx_t {
        mutex: ::core::ptr::null_mut::<pthread_mutex_t>(),
        db: ::core::ptr::null_mut::<sqlite3>(),
        connection_mutex: ::core::ptr::null_mut::<sqlite3_mutex>(),
    };
    if tx_begin(&raw mut tx) != 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        tx.db,
        b"UPDATE observation_meta SET epoch_uuid = ? WHERE id = 1;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw mut epoch as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if sqlite3_step(stmt) == SQLITE_DONE && sqlite3_changes(tx.db) == 1 as ::core::ffi::c_int {
            rc = C2Rust_Unnamed::DB_OBS_OK.0;
        }
    }
    sqlite3_finalize(stmt);
    rc = tx_end(&raw mut tx, rc);
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Regenerated observation cursor epoch\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return rc;
}
unsafe extern "C" fn valid_query(mut query: *const observation_list_query_t) -> bool {
    if query.is_null() {
        return r#false != 0;
    }
    let mut limit: ::core::ffi::c_int = if (*query).limit == 0 as ::core::ffi::c_int {
        OBS_LIST_DEFAULT_LIMIT
    } else {
        (*query).limit
    };
    if limit < 1 as ::core::ffi::c_int || limit > OBS_LIST_MAX_LIMIT {
        return r#false != 0;
    }
    if (*query).label_count > OBS_QUERY_MAX_LABELS as size_t {
        return r#false != 0;
    }
    let mut i: size_t = 0 as size_t;
    while i < (*query).label_count {
        if !text_len_between(
            &raw const *(&raw const (*query).labels as *const [::core::ffi::c_char; 32])
                .offset(i as isize) as *const ::core::ffi::c_char,
            OBS_LABEL_SIZE as size_t,
            1 as size_t,
            (OBS_LABEL_SIZE - 1 as ::core::ffi::c_int) as size_t,
        ) {
            return r#false != 0;
        }
        i = i.wrapping_add(1);
    }
    if (*query).has_since as ::core::ffi::c_int != 0
        && (*query).has_until as ::core::ffi::c_int != 0
        && (*query).since_ms >= (*query).until_ms
    {
        return r#false != 0;
    }
    if !(*query).all_cameras
        && (*query).camera_count > 0 as size_t
        && (*query).camera_uuids.is_null()
    {
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn camera_allowed(
    mut query: *const observation_list_query_t,
    mut camera: *const ::core::ffi::c_char,
) -> bool {
    if (*query).all_cameras {
        return r#true != 0;
    }
    let mut i: size_t = 0 as size_t;
    while i < (*query).camera_count {
        if strcmp(
            &raw const *(*query).camera_uuids.offset(i as isize) as *const ::core::ffi::c_char,
            camera,
        ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn change_matches(
    mut query: *const observation_list_query_t,
    mut stmt: *mut sqlite3_stmt,
) -> bool {
    if sqlite3_column_int(stmt, 5 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    let mut camera: *const ::core::ffi::c_char =
        sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    if camera.is_null() || !camera_allowed(query, camera) {
        return r#false != 0;
    }
    let mut observed: int64_t = sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as int64_t;
    if (*query).has_since as ::core::ffi::c_int != 0 && observed < (*query).since_ms {
        return r#false != 0;
    }
    if (*query).has_until as ::core::ffi::c_int != 0 && observed >= (*query).until_ms {
        return r#false != 0;
    }
    if (*query).label_count > 0 as size_t {
        let mut label: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 3 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut found: bool = r#false != 0;
        let mut i: size_t = 0 as size_t;
        while !label.is_null() && i < (*query).label_count && !found {
            found = strcasecmp(
                label,
                &raw const *(&raw const (*query).labels as *const [::core::ffi::c_char; 32])
                    .offset(i as isize) as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
            i = i.wrapping_add(1);
        }
        if !found {
            return r#false != 0;
        }
    }
    return r#true != 0;
}
unsafe extern "C" fn fill_expired(
    mut db: *mut sqlite3,
    mut meta: *const observation_meta_t,
    mut page: *mut observation_page_t,
) -> ::core::ffi::c_int {
    if observation_cursor_encode(
        (*meta).oldest_seq - 1 as int64_t,
        &raw const (*meta).epoch_uuid as *const ::core::ffi::c_char,
        &raw mut (*page).oldest_cursor as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 257]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        b"SELECT MIN(observed_at_ms) FROM object_observations;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
        && sqlite3_step(stmt) == SQLITE_ROW
    {
        (*page).has_oldest_observed_at =
            sqlite3_column_type(stmt, 0 as ::core::ffi::c_int) != SQLITE_NULL;
        (*page).oldest_observed_at_ms =
            sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
        rc = C2Rust_Unnamed::DB_OBS_CURSOR_EXPIRED.0;
    }
    sqlite3_finalize(stmt);
    return rc;
}
unsafe extern "C" fn scan_page(
    mut db: *mut sqlite3,
    mut query: *const observation_list_query_t,
    mut start: int64_t,
    mut watermark: int64_t,
    mut limit: ::core::ffi::c_int,
    mut selected: *mut [::core::ffi::c_char; 37],
    mut selected_count: *mut size_t,
    mut last_scanned: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        b"SELECT c.seq, o.uuid, o.camera_uuid, o.label, o.observed_at_ms, EXISTS (SELECT 1 FROM streams s WHERE s.camera_uuid = o.camera_uuid) FROM observation_changes c JOIN object_observations o ON o.uuid = c.observation_uuid WHERE c.seq > ? AND c.seq <= ? ORDER BY c.seq LIMIT ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare observation scan: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, start as sqlite3_int64);
    sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, watermark as sqlite3_int64);
    sqlite3_bind_int(stmt, 3 as ::core::ffi::c_int, OBS_LIST_SCAN_CAP);
    *last_scanned = start;
    *selected_count = 0 as size_t;
    let mut step: ::core::ffi::c_int = 0;
    loop {
        step = sqlite3_step(stmt);
        if step != SQLITE_ROW {
            break;
        }
        if change_matches(query, stmt) {
            let mut uuid: *const ::core::ffi::c_char =
                sqlite3_column_text(stmt, 1 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            let mut present: bool = r#false != 0;
            let mut i: size_t = 0 as size_t;
            while !uuid.is_null() && i < *selected_count && !present {
                present = strcmp(
                    &raw mut *selected.offset(i as isize) as *mut ::core::ffi::c_char,
                    uuid,
                ) == 0 as ::core::ffi::c_int;
                i = i.wrapping_add(1);
            }
            if !present {
                if *selected_count == limit as size_t {
                    break;
                }
                let c2rust_fresh2 = *selected_count;
                *selected_count = (*selected_count).wrapping_add(1);
                safe_strcpy(
                    &raw mut *selected.offset(c2rust_fresh2 as isize) as *mut ::core::ffi::c_char,
                    if !uuid.is_null() {
                        uuid
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    OBS_UUID_SIZE as size_t,
                    0 as size_t,
                );
            }
        }
        *last_scanned = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
    }
    let mut rc: ::core::ffi::c_int = if step == SQLITE_ROW || step == SQLITE_DONE {
        C2Rust_Unnamed::DB_OBS_OK.0
    } else {
        C2Rust_Unnamed::DB_OBS_ERROR.0
    };
    sqlite3_finalize(stmt);
    return rc;
}
unsafe extern "C" fn changes_exist(
    mut db: *mut sqlite3,
    mut after: int64_t,
    mut watermark: int64_t,
    mut exists: *mut bool,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        b"SELECT EXISTS (SELECT 1 FROM observation_changes WHERE seq > ? AND seq <= ?);\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, after as sqlite3_int64);
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, watermark as sqlite3_int64);
        if sqlite3_step(stmt) == SQLITE_ROW {
            *exists = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
            rc = C2Rust_Unnamed::DB_OBS_OK.0;
        }
    }
    sqlite3_finalize(stmt);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observations_list(
    mut query: *const observation_list_query_t,
    mut cursor: *const ::core::ffi::c_char,
    mut page: *mut observation_page_t,
) -> ::core::ffi::c_int {
    if page.is_null() {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    memset(
        page as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_page_t>(),
    );
    if !valid_query(query) {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    let mut limit: ::core::ffi::c_int = if (*query).limit == 0 as ::core::ffi::c_int {
        OBS_LIST_DEFAULT_LIMIT
    } else {
        (*query).limit
    };
    let mut has_cursor: bool =
        !cursor.is_null() && *cursor.offset(0isize) as ::core::ffi::c_int != 0;
    let mut head: bool = has_cursor as ::core::ffi::c_int != 0
        && strcmp(cursor, b"head\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int;
    let mut start: int64_t = 0 as int64_t;
    let mut epoch: [::core::ffi::c_char; 37] =
        ::core::mem::transmute::<[u8; 37], [::core::ffi::c_char; 37]>(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
    if has_cursor as ::core::ffi::c_int != 0
        && !head
        && observation_cursor_decode(
            cursor,
            &raw mut start,
            &raw mut epoch as *mut ::core::ffi::c_char,
        ) != C2Rust_Unnamed::DB_OBS_OK.0
    {
        return C2Rust_Unnamed::DB_OBS_INVALID_CURSOR.0;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if ro_begin(&raw mut db) != C2Rust_Unnamed::DB_OBS_OK.0 {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut meta: observation_meta_t = observation_meta_t {
        epoch_uuid: [0; 37],
        oldest_seq: 0,
        head_seq: 0,
    };
    let mut rc: ::core::ffi::c_int = read_meta_on(db, &raw mut meta);
    let mut watermark: int64_t = meta.head_seq;
    if rc == C2Rust_Unnamed::DB_OBS_OK.0
        && has_cursor as ::core::ffi::c_int != 0
        && !head
        && (strcmp(
            &raw mut epoch as *mut ::core::ffi::c_char,
            &raw mut meta.epoch_uuid as *mut ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            || start < meta.oldest_seq - 1 as int64_t
            || start > watermark)
    {
        rc = fill_expired(db, &raw mut meta, page);
        ro_end(db);
        return rc;
    }
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 && head as ::core::ffi::c_int != 0 {
        rc = if observation_cursor_encode(
            watermark,
            &raw mut meta.epoch_uuid as *mut ::core::ffi::c_char,
            &raw mut (*page).next_cursor as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 257]>(),
        ) == 0 as ::core::ffi::c_int
        {
            C2Rust_Unnamed::DB_OBS_OK.0
        } else {
            C2Rust_Unnamed::DB_OBS_ERROR.0
        };
        ro_end(db);
        return rc;
    }
    if !has_cursor {
        start = meta.oldest_seq - 1 as int64_t;
    }
    let mut selected: *mut [::core::ffi::c_char; 37] =
        ::core::ptr::null_mut::<[::core::ffi::c_char; 37]>();
    let mut selected_count: size_t = 0 as size_t;
    let mut last_scanned: int64_t = start;
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        selected = calloc(
            limit as size_t,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        ) as *mut [::core::ffi::c_char; 37];
        if selected.is_null() {
            rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
        }
    }
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        rc = scan_page(
            db,
            query,
            start,
            watermark,
            limit,
            selected,
            &raw mut selected_count,
            &raw mut last_scanned,
        );
    }
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        rc = changes_exist(db, last_scanned, watermark, &raw mut (*page).has_more);
    }
    if rc == C2Rust_Unnamed::DB_OBS_OK.0
        && observation_cursor_encode(
            last_scanned,
            &raw mut meta.epoch_uuid as *mut ::core::ffi::c_char,
            &raw mut (*page).next_cursor as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 257]>(),
        ) != 0 as ::core::ffi::c_int
    {
        rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 && selected_count > 0 as size_t {
        (*page).items = calloc(
            selected_count,
            ::core::mem::size_of::<observation_record_t>(),
        ) as *mut observation_record_t;
        if (*page).items.is_null() {
            rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
        }
    }
    let mut record_stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut i: size_t = 0 as size_t;
    while rc == C2Rust_Unnamed::DB_OBS_OK.0 && i < selected_count {
        let mut got: ::core::ffi::c_int = get_record_on(
            db,
            &raw mut record_stmt,
            &raw mut *selected.offset(i as isize) as *mut ::core::ffi::c_char,
            (*page).items.offset((*page).count as isize),
        );
        if got == C2Rust_Unnamed::DB_OBS_OK.0 {
            (*page).count = (*page).count.wrapping_add(1);
        } else if got != C2Rust_Unnamed::DB_OBS_NOT_FOUND.0 {
            rc = got;
        }
        i = i.wrapping_add(1);
    }
    sqlite3_finalize(record_stmt);
    free(selected as *mut ::core::ffi::c_void);
    ro_end(db);
    if rc != C2Rust_Unnamed::DB_OBS_OK.0 {
        db_observation_page_free(page);
    }
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observation_page_free(mut page: *mut observation_page_t) {
    if page.is_null() {
        return;
    }
    free((*page).items as *mut ::core::ffi::c_void);
    (*page).items = ::core::ptr::null_mut::<observation_record_t>();
    (*page).count = 0 as size_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observation_pending_publications(
    mut out: *mut observation_publication_t,
    mut max: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if out.is_null() || max <= 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if ro_begin(&raw mut db) != C2Rust_Unnamed::DB_OBS_OK.0 {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        db,
        b"SELECT p.change_seq, p.message_id, p.attempts, c.observation_uuid, c.revision, c.cause, c.evidence_state, c.evidence_reason, c.created_at_ms, o.camera_uuid, o.label, o.observed_at_ms, o.asset_uuid FROM observation_publications p JOIN observation_changes c ON c.seq = p.change_seq JOIN object_observations o ON o.uuid = c.observation_uuid WHERE p.state = 'pending' ORDER BY p.change_seq LIMIT ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_int(stmt, 1 as ::core::ffi::c_int, max);
        let mut step: ::core::ffi::c_int = 0;
        loop {
            step = sqlite3_step(stmt);
            if step != SQLITE_ROW {
                break;
            }
            let mut r#pub: *mut observation_publication_t = out.offset(count as isize);
            memset(
                r#pub as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<observation_publication_t>(),
            );
            (*r#pub).change_seq = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int)
                as int64_t;
            copy_column(
                stmt,
                1 as ::core::ffi::c_int,
                &raw mut (*r#pub).message_id as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            );
            (*r#pub).attempts = sqlite3_column_int(stmt, 2 as ::core::ffi::c_int);
            copy_column(
                stmt,
                3 as ::core::ffi::c_int,
                &raw mut (*r#pub).observation_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            );
            (*r#pub).revision = sqlite3_column_int(stmt, 4 as ::core::ffi::c_int);
            let mut text: [::core::ffi::c_char; 16] = [0; 16];
            copy_column(
                stmt,
                5 as ::core::ffi::c_int,
                &raw mut text as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            );
            if cause_parse(
                &raw mut text as *mut ::core::ffi::c_char,
                &raw mut (*r#pub).cause,
            ) != 0 as ::core::ffi::c_int
            {
                continue;
            }
            copy_column(
                stmt,
                6 as ::core::ffi::c_int,
                &raw mut text as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            );
            if observation_asset_state_parse(
                &raw mut text as *mut ::core::ffi::c_char,
                &raw mut (*r#pub).evidence_state,
            ) != 0 as ::core::ffi::c_int
            {
                continue;
            }
            copy_column(
                stmt,
                7 as ::core::ffi::c_int,
                &raw mut (*r#pub).evidence_reason as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 33]>(),
            );
            (*r#pub).change_created_at_ms = sqlite3_column_int64(
                stmt,
                8 as ::core::ffi::c_int,
            ) as int64_t;
            copy_column(
                stmt,
                9 as ::core::ffi::c_int,
                &raw mut (*r#pub).camera_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            );
            copy_column(
                stmt,
                10 as ::core::ffi::c_int,
                &raw mut (*r#pub).label as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            );
            (*r#pub).observed_at_ms = sqlite3_column_int64(
                stmt,
                11 as ::core::ffi::c_int,
            ) as int64_t;
            copy_column(
                stmt,
                12 as ::core::ffi::c_int,
                &raw mut (*r#pub).asset_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            );
            count += 1;
        }
        if step == SQLITE_DONE {
            rc = C2Rust_Unnamed::DB_OBS_OK.0;
        }
    }
    sqlite3_finalize(stmt);
    ro_end(db);
    return if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        count
    } else {
        rc
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observation_publication_mark(
    mut change_seq: int64_t,
    mut state: observation_publication_state_t,
    mut last_error: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if change_seq <= 0 as int64_t || observation_publication_state_name(state).is_null() {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    let mut error_text: [::core::ffi::c_char; 256] = ::core::mem::transmute::<
        [u8; 256],
        [::core::ffi::c_char; 256],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    if !last_error.is_null() {
        safe_strcpy(
            &raw mut error_text as *mut ::core::ffi::c_char,
            last_error,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
    }
    let mut now_ms: int64_t = current_time_ms();
    let mut tx: obs_tx_t = obs_tx_t {
        mutex: ::core::ptr::null_mut::<pthread_mutex_t>(),
        db: ::core::ptr::null_mut::<sqlite3>(),
        connection_mutex: ::core::ptr::null_mut::<sqlite3_mutex>(),
    };
    if tx_begin(&raw mut tx) != 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    if sqlite3_prepare_v2(
        tx.db,
        b"UPDATE observation_publications SET state = ?, attempts = attempts + 1, last_error = ?, updated_at_ms = ? WHERE change_seq = ? AND state = 'pending';\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
    {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            observation_publication_state_name(state),
            -1 as ::core::ffi::c_int,
            SQLITE_STATIC,
        );
        bind_text_or_null(
            stmt,
            2 as ::core::ffi::c_int,
            &raw mut error_text as *mut ::core::ffi::c_char,
        );
        sqlite3_bind_int64(stmt, 3 as ::core::ffi::c_int, now_ms as sqlite3_int64);
        sqlite3_bind_int64(stmt, 4 as ::core::ffi::c_int, change_seq as sqlite3_int64);
        if sqlite3_step(stmt) == SQLITE_DONE {
            rc = C2Rust_Unnamed::DB_OBS_OK.0;
        }
    }
    sqlite3_finalize(stmt);
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 && sqlite3_changes(tx.db) == 0 as ::core::ffi::c_int {
        rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if sqlite3_prepare_v2(
            tx.db,
            b"SELECT 1 FROM observation_publications WHERE change_seq = ?;\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
        {
            sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, change_seq as sqlite3_int64);
            let mut step: ::core::ffi::c_int = sqlite3_step(stmt);
            if step == SQLITE_ROW {
                rc = C2Rust_Unnamed::DB_OBS_CAS_LOST.0;
            } else if step == SQLITE_DONE {
                rc = C2Rust_Unnamed::DB_OBS_NOT_FOUND.0;
            }
        }
        sqlite3_finalize(stmt);
        tx_end(&raw mut tx, C2Rust_Unnamed::DB_OBS_ERROR.0);
        return rc;
    }
    return tx_end(&raw mut tx, rc);
}
unsafe extern "C" fn collect_texts(
    mut db: *mut sqlite3,
    mut sql: *const ::core::ffi::c_char,
    mut bound: int64_t,
    mut out: *mut *mut [::core::ffi::c_char; 37],
    mut count: *mut size_t,
) -> ::core::ffi::c_int {
    *out = ::core::ptr::null_mut::<[::core::ffi::c_char; 37]>();
    *count = 0 as size_t;
    let mut capacity: size_t = 0 as size_t;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, bound as sqlite3_int64);
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_OK.0;
    let mut step: ::core::ffi::c_int = 0;
    loop {
        step = sqlite3_step(stmt);
        if step != SQLITE_ROW {
            break;
        }
        if sqlite3_column_type(stmt, 0 as ::core::ffi::c_int) == SQLITE_NULL {
            continue;
        }
        if *count == capacity {
            let mut next: size_t = if capacity != 0 {
                capacity.wrapping_mul(2 as size_t)
            } else {
                64 as size_t
            };
            let mut grown: *mut ::core::ffi::c_void = realloc(
                *out as *mut ::core::ffi::c_void,
                next.wrapping_mul(::core::mem::size_of::<[::core::ffi::c_char; 37]>()),
            );
            if grown.is_null() {
                rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
                break;
            } else {
                *out = grown as *mut [::core::ffi::c_char; 37];
                capacity = next;
            }
        }
        let c2rust_fresh3 = *count;
        *count = (*count).wrapping_add(1);
        copy_column(
            stmt,
            0 as ::core::ffi::c_int,
            &raw mut *(*out).offset(c2rust_fresh3 as isize) as *mut ::core::ffi::c_char,
            OBS_UUID_SIZE as size_t,
        );
    }
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 && step != SQLITE_DONE {
        rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    sqlite3_finalize(stmt);
    return rc;
}
unsafe extern "C" fn delete_each(
    mut db: *mut sqlite3,
    mut sql: *const ::core::ffi::c_char,
    mut uuids: *mut [::core::ffi::c_char; 37],
    mut count: size_t,
    mut deleted: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_OK.0;
    let mut i: size_t = 0 as size_t;
    while i < count && rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        sqlite3_reset(stmt);
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw mut *uuids.offset(i as isize) as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if sqlite3_step(stmt) != SQLITE_DONE {
            rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
        } else {
            *deleted += sqlite3_changes(db);
        }
        i = i.wrapping_add(1);
    }
    sqlite3_finalize(stmt);
    return rc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_observations_prune(
    mut before_ms: int64_t,
    mut batch: ::core::ffi::c_int,
    mut result: *mut observation_prune_result_t,
) -> ::core::ffi::c_int {
    if result.is_null() {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    memset(
        result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<observation_prune_result_t>(),
    );
    if batch <= 0 as ::core::ffi::c_int || batch > OBS_PRUNE_MAX_BATCH {
        return C2Rust_Unnamed::DB_OBS_INVALID.0;
    }
    let mut tx: obs_tx_t = obs_tx_t {
        mutex: ::core::ptr::null_mut::<pthread_mutex_t>(),
        db: ::core::ptr::null_mut::<sqlite3>(),
        connection_mutex: ::core::ptr::null_mut::<sqlite3_mutex>(),
    };
    if tx_begin(&raw mut tx) != 0 as ::core::ffi::c_int {
        return C2Rust_Unnamed::DB_OBS_ERROR.0;
    }
    let mut rc: ::core::ffi::c_int = C2Rust_Unnamed::DB_OBS_ERROR.0;
    let mut oldest_seq: int64_t = 1 as int64_t;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        tx.db,
        b"SELECT oldest_seq FROM observation_meta WHERE id = 1;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
        && sqlite3_step(stmt) == SQLITE_ROW
    {
        oldest_seq = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
        rc = C2Rust_Unnamed::DB_OBS_OK.0;
    }
    sqlite3_finalize(stmt);
    let mut last_pruned: int64_t = 0 as int64_t;
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
        if sqlite3_prepare_v2(
            tx.db,
            b"SELECT seq, created_at_ms FROM observation_changes ORDER BY seq LIMIT ?;\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
        {
            sqlite3_bind_int(stmt, 1 as ::core::ffi::c_int, batch);
            let mut step: ::core::ffi::c_int = 0;
            loop {
                step = sqlite3_step(stmt);
                if !(step == SQLITE_ROW
                    && sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int)
                        < before_ms as sqlite3_int64)
                {
                    break;
                }
                last_pruned = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
            }
            if step == SQLITE_ROW || step == SQLITE_DONE {
                rc = C2Rust_Unnamed::DB_OBS_OK.0;
            }
        }
        sqlite3_finalize(stmt);
    }
    let mut observations: *mut [::core::ffi::c_char; 37] =
        ::core::ptr::null_mut::<[::core::ffi::c_char; 37]>();
    let mut assets: *mut [::core::ffi::c_char; 37] =
        ::core::ptr::null_mut::<[::core::ffi::c_char; 37]>();
    let mut observation_count: size_t = 0 as size_t;
    let mut asset_count: size_t = 0 as size_t;
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 && last_pruned > 0 as int64_t {
        rc = collect_texts(
            tx.db,
            b"SELECT DISTINCT observation_uuid FROM observation_changes WHERE seq <= ?;\0".as_ptr()
                as *const ::core::ffi::c_char,
            last_pruned,
            &raw mut observations,
            &raw mut observation_count,
        );
        if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
            rc = collect_texts(
                tx.db,
                b"SELECT DISTINCT o.asset_uuid FROM object_observations o JOIN observation_changes c ON c.observation_uuid = o.uuid WHERE c.seq <= ?;\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                last_pruned,
                &raw mut assets,
                &raw mut asset_count,
            );
        }
        if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
            rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
            stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            if sqlite3_prepare_v2(
                tx.db,
                b"DELETE FROM observation_changes WHERE seq <= ?;\0".as_ptr()
                    as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                &raw mut stmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            ) == SQLITE_OK
            {
                sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, last_pruned as sqlite3_int64);
                if sqlite3_step(stmt) == SQLITE_DONE {
                    (*result).changes_deleted = sqlite3_changes(tx.db);
                    rc = C2Rust_Unnamed::DB_OBS_OK.0;
                }
            }
            sqlite3_finalize(stmt);
        }
        if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
            rc = delete_each(
                tx.db,
                b"DELETE FROM object_observations WHERE uuid = ?1 AND NOT EXISTS (SELECT 1 FROM observation_changes WHERE observation_uuid = ?1);\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                observations,
                observation_count,
                &raw mut (*result).observations_deleted,
            );
        }
        if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
            rc = delete_each(
                tx.db,
                b"DELETE FROM observation_assets WHERE uuid = ?1 AND protected = 0 AND (path IS NULL OR path = '') AND state IN ('unavailable', 'expired') AND NOT EXISTS (SELECT 1 FROM object_observations WHERE asset_uuid = ?1);\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                assets,
                asset_count,
                &raw mut (*result).assets_deleted,
            );
        }
        if rc == C2Rust_Unnamed::DB_OBS_OK.0 && last_pruned + 1 as int64_t > oldest_seq {
            rc = C2Rust_Unnamed::DB_OBS_ERROR.0;
            stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            if sqlite3_prepare_v2(
                tx.db,
                b"UPDATE observation_meta SET oldest_seq = ? WHERE id = 1;\0".as_ptr()
                    as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                &raw mut stmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            ) == SQLITE_OK
            {
                sqlite3_bind_int64(
                    stmt,
                    1 as ::core::ffi::c_int,
                    (last_pruned + 1 as int64_t) as sqlite3_int64,
                );
                if sqlite3_step(stmt) == SQLITE_DONE {
                    rc = C2Rust_Unnamed::DB_OBS_OK.0;
                }
            }
            sqlite3_finalize(stmt);
            if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
                oldest_seq = last_pruned + 1 as int64_t;
            }
        }
    }
    free(observations as *mut ::core::ffi::c_void);
    free(assets as *mut ::core::ffi::c_void);
    rc = tx_end(&raw mut tx, rc);
    if rc == C2Rust_Unnamed::DB_OBS_OK.0 {
        (*result).oldest_seq = oldest_seq;
    } else {
        memset(
            result as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<observation_prune_result_t>(),
        );
    }
    return rc;
}
pub const __ATOMIC_ACQUIRE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __ATOMIC_RELEASE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
