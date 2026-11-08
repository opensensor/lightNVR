use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddStringToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
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
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn authorization_action_catalog(
        action_count: *mut ::core::ffi::c_int,
    ) -> *const authorization_action_metadata_t;
    unsafe fn authorization_action_from_key(
        key: *const ::core::ffi::c_char,
    ) -> authorization_action_t;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn db_open_readonly_connection(connection: *mut *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn db_close_readonly_connection(connection: *mut sqlite3);
    unsafe fn db_get_system_setting_alloc(
        key: *const ::core::ffi::c_char,
        out: *mut *mut ::core::ffi::c_char,
        out_len: *mut size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_set_system_setting(
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcat(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
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
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
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
pub struct authorization_action_t(pub ::core::ffi::c_int);
impl authorization_action_t {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct authorization_action_metadata_t {
    pub action: authorization_action_t,
    pub key: *const ::core::ffi::c_char,
    pub category: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub camera_scoped: bool,
    pub destructive: bool,
    pub enforced: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audit_event_input_t {
    pub request_id: *const ::core::ffi::c_char,
    pub principal_user_id: int64_t,
    pub principal_username: *const ::core::ffi::c_char,
    pub auth_method: *const ::core::ffi::c_char,
    pub api_token_uuid: *const ::core::ffi::c_char,
    pub action: *const ::core::ffi::c_char,
    pub target_type: *const ::core::ffi::c_char,
    pub target_uuid: *const ::core::ffi::c_char,
    pub outcome: *const ::core::ffi::c_char,
    pub remote_address: *const ::core::ffi::c_char,
    pub details_json: *const ::core::ffi::c_char,
    pub occurred_at: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audit_event_t {
    pub id: int64_t,
    pub uuid: [::core::ffi::c_char; 37],
    pub occurred_at: int64_t,
    pub request_id: [::core::ffi::c_char; 65],
    pub principal_user_id: int64_t,
    pub principal_username: [::core::ffi::c_char; 64],
    pub auth_method: [::core::ffi::c_char; 24],
    pub api_token_uuid: [::core::ffi::c_char; 37],
    pub action: [::core::ffi::c_char; 96],
    pub target_type: [::core::ffi::c_char; 48],
    pub target_uuid: [::core::ffi::c_char; 128],
    pub outcome: [::core::ffi::c_char; 16],
    pub remote_address: [::core::ffi::c_char; 64],
    pub details_json: [::core::ffi::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audit_query_t {
    pub page: ::core::ffi::c_int,
    pub page_size: ::core::ffi::c_int,
    pub since: int64_t,
    pub until: int64_t,
    pub principal_user_id: int64_t,
    pub action: [::core::ffi::c_char; 96],
    pub outcome: [::core::ffi::c_char; 16],
    pub target_uuid: [::core::ffi::c_char; 128],
    pub request_id: [::core::ffi::c_char; 65],
    pub event_type: [::core::ffi::c_char; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audit_page_t {
    pub events: *mut audit_event_t,
    pub count: ::core::ffi::c_int,
    pub total: int64_t,
    pub page: ::core::ffi::c_int,
    pub page_size: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct audit_decision_mode_t(pub ::core::ffi::c_uint);
impl audit_decision_mode_t {
    pub const AUDIT_DECISION_MODE_RECORD: Self = Self(0);
    pub const AUDIT_DECISION_MODE_SUMMARIZE: Self = Self(1);
    pub const AUDIT_DECISION_MODE_OFF: Self = Self(2);
}
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const LOG_COMPONENT: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"AuditDB\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const AUDIT_EVENT_UUID_MAX: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const AUDIT_REQUEST_ID_MAX: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const AUDIT_ACTION_MAX: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const AUDIT_TARGET_TYPE_MAX: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const AUDIT_USERNAME_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const AUDIT_AUTH_METHOD_MAX: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const AUDIT_REMOTE_ADDRESS_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const AUDIT_DETAILS_MAX: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const AUDIT_QUERY_VALUE_MAX: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const AUDIT_PAGE_SIZE_MAX: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const AUDIT_RETENTION_DEFAULT_DAYS: ::core::ffi::c_int = 365 as ::core::ffi::c_int;
pub const AUDIT_RETENTION_MAX_DAYS: ::core::ffi::c_int = 3650 as ::core::ffi::c_int;
pub const AUDIT_PRUNE_BATCH_ROWS: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
pub const AUDIT_PRUNE_BUDGET_MS: ::core::ffi::c_int = 250 as ::core::ffi::c_int;
pub const AUDIT_PRUNE_INTERVAL_SECONDS: ::core::ffi::c_int = 3600 as ::core::ffi::c_int;
pub const AUDIT_PRUNE_BACKLOG_INTERVAL_SECONDS: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const AUDIT_PRUNE_BATCH_SQL: [::core::ffi::c_char; 123] = unsafe {
    ::core::mem::transmute::<
        [u8; 123],
        [::core::ffi::c_char; 123],
    >(
        *b"DELETE FROM audit_events WHERE id IN (SELECT id FROM audit_events WHERE occurred_at < ? ORDER BY occurred_at, id LIMIT ?);\0",
    )
};
pub const AUDIT_DECISION_MODES_SETTING_KEY: [::core::ffi::c_char; 29] = unsafe {
    ::core::mem::transmute::<[u8; 29], [::core::ffi::c_char; 29]>(
        *b"audit_allowed_decision_modes\0",
    )
};
static mut last_automatic_prune_at: int64_t = 0 as int64_t;
static mut automatic_prune_interval: int64_t = AUDIT_PRUNE_INTERVAL_SECONDS as int64_t;
unsafe extern "C" fn valid_outcome(mut outcome: *const ::core::ffi::c_char) -> bool {
    return !outcome.is_null()
        && (strcmp(outcome, b"allowed\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcmp(outcome, b"denied\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || strcmp(outcome, b"success\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || strcmp(outcome, b"failure\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || strcmp(outcome, b"error\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn valid_safe_text(
    mut value: *const ::core::ffi::c_char,
    mut maximum: size_t,
    mut required: bool,
) -> bool {
    if value.is_null() || *value.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return !required;
    }
    let mut length: size_t = strlen(value);
    if length >= maximum {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = value as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_IScntrl.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            return r#false != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_details(mut details: *const ::core::ffi::c_char) -> bool {
    if details.is_null()
        || *details.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strlen(details) >= AUDIT_DETAILS_MAX as size_t
    {
        return r#false != 0;
    }
    let mut parsed: *mut cJSON = cJSON_Parse(details);
    let mut valid: bool = cJSON_IsObject(parsed) != 0;
    cJSON_Delete(parsed);
    return valid;
}
unsafe extern "C" fn retention_days_locked(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut days: ::core::ffi::c_int = AUDIT_RETENTION_DEFAULT_DAYS;
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT value FROM system_settings WHERE key='audit_retention_days' LIMIT 1;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW {
        let mut value: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut parsed: ::core::ffi::c_long = if !value.is_null() {
            strtol(value, &raw mut end, 10 as ::core::ffi::c_int)
        } else {
            0 as ::core::ffi::c_long
        };
        if !end.is_null()
            && *end as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            && parsed >= 1 as ::core::ffi::c_long
            && parsed <= AUDIT_RETENTION_MAX_DAYS as ::core::ffi::c_long
        {
            days = parsed as ::core::ffi::c_int;
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    return days;
}
static mut prune_budget_ms_override: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_audit_set_prune_budget_ms_for_testing(
    mut budget_ms: ::core::ffi::c_int,
) {
    prune_budget_ms_override = budget_ms;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_audit_get_prune_interval_seconds_for_testing() -> ::core::ffi::c_int {
    return automatic_prune_interval as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_audit_set_automatic_prune_state_for_testing(
    mut last_prune_at: int64_t,
    mut interval_seconds: ::core::ffi::c_int,
) {
    last_automatic_prune_at = last_prune_at;
    automatic_prune_interval = interval_seconds as int64_t;
}
unsafe extern "C" fn prune_monotonic_ms() -> int64_t {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut ts);
    return ts.tv_sec as int64_t * 1000 as int64_t + ts.tv_nsec as int64_t / 1000000 as int64_t;
}
unsafe extern "C" fn prune_locked(
    mut db: *mut sqlite3,
    mut deleted_count: *mut ::core::ffi::c_int,
    mut more_remaining: *mut bool,
) -> ::core::ffi::c_int {
    let mut retention_days: ::core::ffi::c_int = retention_days_locked(db);
    let mut cutoff: int64_t = time(::core::ptr::null_mut::<time_t>()) as int64_t
        - retention_days as int64_t * 24 as int64_t * 60 as int64_t * 60 as int64_t;
    let mut budget_ms: ::core::ffi::c_int = if prune_budget_ms_override >= 0 as ::core::ffi::c_int {
        prune_budget_ms_override
    } else {
        AUDIT_PRUNE_BUDGET_MS
    };
    let mut deadline: int64_t = prune_monotonic_ms() + budget_ms as int64_t;
    let mut total_deleted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut more: bool = r#false != 0;
    let mut rc: ::core::ffi::c_int = SQLITE_DONE;
    static mut sql: *const ::core::ffi::c_char = AUDIT_PRUNE_BATCH_SQL.as_ptr();
    loop {
        let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = sqlite3_prepare_v2(
            db,
            sql,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc != SQLITE_OK {
            if !stmt.is_null() {
                sqlite3_finalize(stmt);
            }
            break;
        } else {
            sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, cutoff as sqlite3_int64);
            sqlite3_bind_int(stmt, 2 as ::core::ffi::c_int, AUDIT_PRUNE_BATCH_ROWS);
            rc = sqlite3_step(stmt);
            let mut batch_deleted: ::core::ffi::c_int = if rc == SQLITE_DONE {
                sqlite3_changes(db)
            } else {
                0 as ::core::ffi::c_int
            };
            sqlite3_finalize(stmt);
            if rc != SQLITE_DONE {
                break;
            }
            total_deleted += batch_deleted;
            if batch_deleted < AUDIT_PRUNE_BATCH_ROWS {
                break;
            }
            if prune_monotonic_ms() < deadline {
                continue;
            }
            more = r#true != 0;
            break;
        }
    }
    if !deleted_count.is_null() {
        *deleted_count = if rc == SQLITE_DONE {
            total_deleted
        } else {
            0 as ::core::ffi::c_int
        };
    }
    if !more_remaining.is_null() {
        *more_remaining = more;
    }
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_audit_append(
    mut input: *const audit_event_input_t,
    mut event_uuid: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !event_uuid.is_null() {
        *event_uuid.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    if input.is_null()
        || !valid_safe_text(
            (*input).request_id,
            AUDIT_REQUEST_ID_MAX as size_t,
            r#true != 0,
        )
        || !valid_safe_text(
            (*input).principal_username,
            AUDIT_USERNAME_MAX as size_t,
            r#false != 0,
        )
        || !valid_safe_text(
            (*input).auth_method,
            AUDIT_AUTH_METHOD_MAX as size_t,
            r#true != 0,
        )
        || !valid_safe_text(
            (*input).api_token_uuid,
            AUDIT_EVENT_UUID_MAX as size_t,
            r#false != 0,
        )
        || !valid_safe_text((*input).action, AUDIT_ACTION_MAX as size_t, r#true != 0)
        || !valid_safe_text(
            (*input).target_type,
            AUDIT_TARGET_TYPE_MAX as size_t,
            r#false != 0,
        )
        || !valid_safe_text(
            (*input).target_uuid,
            AUDIT_QUERY_VALUE_MAX as size_t,
            r#false != 0,
        )
        || !valid_outcome((*input).outcome)
        || !valid_safe_text(
            (*input).remote_address,
            AUDIT_REMOTE_ADDRESS_MAX as size_t,
            r#false != 0,
        )
        || !valid_details((*input).details_json)
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO audit_events (uuid,occurred_at,request_id,principal_user_id,principal_username,auth_method,api_token_uuid,action,target_type,target_uuid,outcome,remote_address,details_json) VALUES (lower(hex(randomblob(4))||'-'||hex(randomblob(2))||'-4'||substr(hex(randomblob(2)),2)||'-'||substr('89ab',(abs(random())%4)+1,1)||substr(hex(randomblob(2)),2)||'-'||hex(randomblob(6))),?,?,?,?,?,?,?,?,?,?,?,?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut occurred_at: int64_t = if (*input).occurred_at > 0 as int64_t {
        (*input).occurred_at
    } else {
        time(::core::ptr::null_mut::<time_t>()) as int64_t
    };
    if rc == SQLITE_OK {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, occurred_at as sqlite3_int64);
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            (*input).request_id,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if (*input).principal_user_id > 0 as int64_t {
            sqlite3_bind_int64(
                stmt,
                3 as ::core::ffi::c_int,
                (*input).principal_user_id as sqlite3_int64,
            );
        } else {
            sqlite3_bind_null(stmt, 3 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            stmt,
            4 as ::core::ffi::c_int,
            if !(*input).principal_username.is_null() {
                (*input).principal_username
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            5 as ::core::ffi::c_int,
            (*input).auth_method,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if !(*input).api_token_uuid.is_null()
            && *(*input).api_token_uuid.offset(0isize) as ::core::ffi::c_int != 0
        {
            sqlite3_bind_text(
                stmt,
                6 as ::core::ffi::c_int,
                (*input).api_token_uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(stmt, 6 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            stmt,
            7 as ::core::ffi::c_int,
            (*input).action,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if !(*input).target_type.is_null()
            && *(*input).target_type.offset(0isize) as ::core::ffi::c_int != 0
        {
            sqlite3_bind_text(
                stmt,
                8 as ::core::ffi::c_int,
                (*input).target_type,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(stmt, 8 as ::core::ffi::c_int);
        }
        if !(*input).target_uuid.is_null()
            && *(*input).target_uuid.offset(0isize) as ::core::ffi::c_int != 0
        {
            sqlite3_bind_text(
                stmt,
                9 as ::core::ffi::c_int,
                (*input).target_uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(stmt, 9 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            stmt,
            10 as ::core::ffi::c_int,
            (*input).outcome,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if !(*input).remote_address.is_null()
            && *(*input).remote_address.offset(0isize) as ::core::ffi::c_int != 0
        {
            sqlite3_bind_text(
                stmt,
                11 as ::core::ffi::c_int,
                (*input).remote_address,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(stmt, 11 as ::core::ffi::c_int);
        }
        sqlite3_bind_text(
            stmt,
            12 as ::core::ffi::c_int,
            (*input).details_json,
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
    if rc == SQLITE_DONE && !event_uuid.is_null() {
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = sqlite3_prepare_v2(
            db,
            b"SELECT uuid FROM audit_events WHERE id=last_insert_rowid();\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW {
            let mut uuid: *const ::core::ffi::c_char =
                sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            safe_strcpy(
                event_uuid,
                if !uuid.is_null() {
                    uuid
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                AUDIT_EVENT_UUID_MAX as size_t,
                0 as size_t,
            );
            rc = SQLITE_DONE;
        }
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
        }
    }
    let mut now: int64_t = time(::core::ptr::null_mut::<time_t>()) as int64_t;
    if rc == SQLITE_DONE && now - last_automatic_prune_at >= automatic_prune_interval {
        let mut deleted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut more_remaining: bool = r#false != 0;
        if prune_locked(db, &raw mut deleted, &raw mut more_remaining) == 0 as ::core::ffi::c_int {
            last_automatic_prune_at = now;
            automatic_prune_interval = (if more_remaining as ::core::ffi::c_int != 0 {
                AUDIT_PRUNE_BACKLOG_INTERVAL_SECONDS
            } else {
                AUDIT_PRUNE_INTERVAL_SECONDS
            }) as int64_t;
            if deleted > 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Pruned %d expired audit events%s\0".as_ptr() as *const ::core::ffi::c_char,
                    deleted,
                    if more_remaining as ::core::ffi::c_int != 0 {
                        b" (backlog remains)\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
            }
        }
    }
    pthread_mutex_unlock(mutex);
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn add_query_filters(
    mut sql: *mut ::core::ffi::c_char,
    mut sql_size: size_t,
    mut query: *const audit_query_t,
) {
    if (*query).since > 0 as int64_t {
        safe_strcat(
            sql,
            b" AND occurred_at>=?\0".as_ptr() as *const ::core::ffi::c_char,
            sql_size,
        );
    }
    if (*query).until > 0 as int64_t {
        safe_strcat(
            sql,
            b" AND occurred_at<=?\0".as_ptr() as *const ::core::ffi::c_char,
            sql_size,
        );
    }
    if (*query).principal_user_id > 0 as int64_t {
        safe_strcat(
            sql,
            b" AND principal_user_id=?\0".as_ptr() as *const ::core::ffi::c_char,
            sql_size,
        );
    }
    if (*query).action[0usize] != 0 {
        safe_strcat(
            sql,
            b" AND action=?\0".as_ptr() as *const ::core::ffi::c_char,
            sql_size,
        );
    }
    if (*query).outcome[0usize] != 0 {
        safe_strcat(
            sql,
            b" AND outcome=?\0".as_ptr() as *const ::core::ffi::c_char,
            sql_size,
        );
    }
    if (*query).target_uuid[0usize] != 0 {
        safe_strcat(
            sql,
            b" AND target_uuid=?\0".as_ptr() as *const ::core::ffi::c_char,
            sql_size,
        );
    }
    if (*query).request_id[0usize] != 0 {
        safe_strcat(
            sql,
            b" AND request_id=?\0".as_ptr() as *const ::core::ffi::c_char,
            sql_size,
        );
    }
    if (*query).event_type[0usize] != 0 {
        safe_strcat(
            sql,
            b" AND json_extract(details_json,'$.event_type')=?\0".as_ptr()
                as *const ::core::ffi::c_char,
            sql_size,
        );
    }
}
unsafe extern "C" fn bind_query_filters(
    mut stmt: *mut sqlite3_stmt,
    mut query: *const audit_query_t,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if (*query).since > 0 as int64_t {
        let c2rust_fresh2 = index;
        index += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh2, (*query).since as sqlite3_int64);
    }
    if (*query).until > 0 as int64_t {
        let c2rust_fresh3 = index;
        index += 1;
        sqlite3_bind_int64(stmt, c2rust_fresh3, (*query).until as sqlite3_int64);
    }
    if (*query).principal_user_id > 0 as int64_t {
        let c2rust_fresh4 = index;
        index += 1;
        sqlite3_bind_int64(
            stmt,
            c2rust_fresh4,
            (*query).principal_user_id as sqlite3_int64,
        );
    }
    if (*query).action[0usize] != 0 {
        let c2rust_fresh5 = index;
        index += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh5,
            &raw const (*query).action as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
    if (*query).outcome[0usize] != 0 {
        let c2rust_fresh6 = index;
        index += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh6,
            &raw const (*query).outcome as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
    if (*query).target_uuid[0usize] != 0 {
        let c2rust_fresh7 = index;
        index += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh7,
            &raw const (*query).target_uuid as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
    if (*query).request_id[0usize] != 0 {
        let c2rust_fresh8 = index;
        index += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh8,
            &raw const (*query).request_id as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
    if (*query).event_type[0usize] != 0 {
        let c2rust_fresh9 = index;
        index += 1;
        sqlite3_bind_text(
            stmt,
            c2rust_fresh9,
            &raw const (*query).event_type as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
    return index;
}
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
unsafe extern "C" fn populate_event(mut stmt: *mut sqlite3_stmt, mut event: *mut audit_event_t) {
    memset(
        event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<audit_event_t>(),
    );
    (*event).id = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
    copy_column(
        &raw mut (*event).uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        stmt,
        1 as ::core::ffi::c_int,
    );
    (*event).occurred_at = sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int) as int64_t;
    copy_column(
        &raw mut (*event).request_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
        stmt,
        3 as ::core::ffi::c_int,
    );
    (*event).principal_user_id = sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as int64_t;
    copy_column(
        &raw mut (*event).principal_username as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        stmt,
        5 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*event).auth_method as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        stmt,
        6 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*event).api_token_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        stmt,
        7 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*event).action as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
        stmt,
        8 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*event).target_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 48]>(),
        stmt,
        9 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*event).target_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        stmt,
        10 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*event).outcome as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        stmt,
        11 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*event).remote_address as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        stmt,
        12 as ::core::ffi::c_int,
    );
    copy_column(
        &raw mut (*event).details_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        stmt,
        13 as ::core::ffi::c_int,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_audit_query(
    mut query: *const audit_query_t,
    mut page: *mut audit_page_t,
) -> ::core::ffi::c_int {
    if query.is_null()
        || page.is_null()
        || (*query).page < 1 as ::core::ffi::c_int
        || (*query).page_size < 1 as ::core::ffi::c_int
        || (*query).page_size > AUDIT_PAGE_SIZE_MAX
        || (*query).outcome[0usize] as ::core::ffi::c_int != 0
            && !valid_outcome(&raw const (*query).outcome as *const ::core::ffi::c_char)
    {
        return -1 as ::core::ffi::c_int;
    }
    memset(
        page as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<audit_page_t>(),
    );
    (*page).page = (*query).page;
    (*page).page_size = (*query).page_size;
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if sqlite3_exec(
        db,
        b"BEGIN;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    let mut count_sql: [::core::ffi::c_char; 1024] = ::core::mem::transmute::<
        [u8; 1024],
        [::core::ffi::c_char; 1024],
    >(
        *b"SELECT count(*) FROM audit_events\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    if (*query).since > 0 as int64_t
        || (*query).until > 0 as int64_t
        || (*query).principal_user_id > 0 as int64_t
        || (*query).action[0usize] as ::core::ffi::c_int != 0
        || (*query).outcome[0usize] as ::core::ffi::c_int != 0
        || (*query).target_uuid[0usize] as ::core::ffi::c_int != 0
        || (*query).request_id[0usize] as ::core::ffi::c_int != 0
        || (*query).event_type[0usize] as ::core::ffi::c_int != 0
    {
        safe_strcat(
            &raw mut count_sql as *mut ::core::ffi::c_char,
            b" WHERE 1=1\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        );
    }
    add_query_filters(
        &raw mut count_sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        query,
    );
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        &raw mut count_sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        bind_query_filters(stmt, query);
    }
    if rc == SQLITE_OK {
        rc = sqlite3_step(stmt);
    }
    if rc == SQLITE_ROW {
        (*page).total = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if rc != SQLITE_ROW {
        db_close_readonly_connection(db);
        return -1 as ::core::ffi::c_int;
    }
    let mut select_sql: [::core::ffi::c_char; 2048] = ::core::mem::transmute::<
        [u8; 2048],
        [::core::ffi::c_char; 2048],
    >(
        *b"SELECT id,uuid,occurred_at,request_id,COALESCE(principal_user_id,0),principal_username,auth_method,COALESCE(api_token_uuid,''),action,COALESCE(target_type,''),COALESCE(target_uuid,''),outcome,COALESCE(remote_address,''),details_json FROM audit_events WHERE 1=1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    add_query_filters(
        &raw mut select_sql as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
        query,
    );
    safe_strcat(
        &raw mut select_sql as *mut ::core::ffi::c_char,
        b" ORDER BY occurred_at DESC,id DESC LIMIT ? OFFSET ?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
    );
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = sqlite3_prepare_v2(
        db,
        &raw mut select_sql as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut next_index: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if rc == SQLITE_OK {
        next_index = bind_query_filters(stmt, query);
    }
    if rc == SQLITE_OK {
        let c2rust_fresh0 = next_index;
        next_index += 1;
        sqlite3_bind_int(stmt, c2rust_fresh0, (*query).page_size);
        sqlite3_bind_int64(
            stmt,
            next_index,
            ((*query).page - 1 as ::core::ffi::c_int) as sqlite3_int64
                * (*query).page_size as sqlite3_int64,
        );
    }
    let mut events: *mut audit_event_t = (if rc == SQLITE_OK {
        calloc(
            (*query).page_size as size_t,
            ::core::mem::size_of::<audit_event_t>(),
        )
    } else {
        NULL
    }) as *mut audit_event_t;
    if rc == SQLITE_OK && events.is_null() {
        rc = SQLITE_NOMEM;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if rc == SQLITE_OK {
        while count < (*query).page_size && {
            rc = sqlite3_step(stmt);
            rc == SQLITE_ROW
        } {
            let c2rust_fresh1 = count;
            count += 1;
            populate_event(stmt, events.offset(c2rust_fresh1 as isize));
        }
        if rc == SQLITE_ROW {
            rc = SQLITE_DONE;
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    db_close_readonly_connection(db);
    if rc != SQLITE_DONE {
        free(events as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    if count == 0 as ::core::ffi::c_int {
        free(events as *mut ::core::ffi::c_void);
        events = ::core::ptr::null_mut::<audit_event_t>();
    }
    (*page).events = events;
    (*page).count = count;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_audit_page_free(mut page: *mut audit_page_t) {
    if page.is_null() {
        return;
    }
    free((*page).events as *mut ::core::ffi::c_void);
    memset(
        page as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<audit_page_t>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_audit_get_retention_days(
    mut retention_days: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if retention_days.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    *retention_days = retention_days_locked(db);
    pthread_mutex_unlock(mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_audit_set_retention_days(
    mut retention_days: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if retention_days < 1 as ::core::ffi::c_int || retention_days > AUDIT_RETENTION_MAX_DAYS {
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
        b"INSERT INTO system_settings(key,value,updated_at) VALUES('audit_retention_days',?,strftime('%s','now')) ON CONFLICT(key) DO UPDATE SET value=excluded.value,updated_at=excluded.updated_at;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut value: [::core::ffi::c_char; 16] = [0; 16];
    snprintf(
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
        retention_days,
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw mut value as *mut ::core::ffi::c_char,
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
    pthread_mutex_unlock(mutex);
    return if rc == SQLITE_DONE {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_audit_prune(
    mut deleted_count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !deleted_count.is_null() {
        *deleted_count = 0 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut more_remaining: bool = r#false != 0;
    let mut result: ::core::ffi::c_int = prune_locked(db, deleted_count, &raw mut more_remaining);
    if result == 0 as ::core::ffi::c_int {
        last_automatic_prune_at = time(::core::ptr::null_mut::<time_t>()) as int64_t;
        automatic_prune_interval = (if more_remaining as ::core::ffi::c_int != 0 {
            AUDIT_PRUNE_BACKLOG_INTERVAL_SECONDS
        } else {
            AUDIT_PRUNE_INTERVAL_SECONDS
        }) as int64_t;
    }
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_decision_mode_name(
    mut mode: audit_decision_mode_t,
) -> *const ::core::ffi::c_char {
    match mode {
        audit_decision_mode_t::AUDIT_DECISION_MODE_SUMMARIZE => {
            return b"summarize\0".as_ptr() as *const ::core::ffi::c_char;
        }
        audit_decision_mode_t::AUDIT_DECISION_MODE_OFF => {
            return b"off\0".as_ptr() as *const ::core::ffi::c_char;
        }
        audit_decision_mode_t::AUDIT_DECISION_MODE_RECORD | _ => {
            return b"record\0".as_ptr() as *const ::core::ffi::c_char;
        }
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_decision_mode_from_name(
    mut name: *const ::core::ffi::c_char,
    mut mode: *mut audit_decision_mode_t,
) -> ::core::ffi::c_int {
    if name.is_null() || mode.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if strcmp(name, b"record\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        *mode = audit_decision_mode_t::AUDIT_DECISION_MODE_RECORD;
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(name, b"summarize\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        *mode = audit_decision_mode_t::AUDIT_DECISION_MODE_SUMMARIZE;
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(name, b"off\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        *mode = audit_decision_mode_t::AUDIT_DECISION_MODE_OFF;
        return 0 as ::core::ffi::c_int;
    }
    return -1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_audit_load_decision_modes(
    mut modes: *mut audit_decision_mode_t,
) -> ::core::ffi::c_int {
    if modes.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < authorization_action_t::AUTHZ_ACTION_COUNT.0 {
        *modes.offset(i as isize) = audit_decision_mode_t::AUDIT_DECISION_MODE_RECORD;
        i += 1;
    }
    let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut value_len: size_t = 0 as size_t;
    let mut rc: ::core::ffi::c_int = db_get_system_setting_alloc(
        AUDIT_DECISION_MODES_SETTING_KEY.as_ptr(),
        &raw mut value,
        &raw mut value_len,
    );
    if rc == 1 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if rc != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut object: *mut cJSON = cJSON_Parse(value);
    free(value as *mut ::core::ffi::c_void);
    if cJSON_IsObject(object) == 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Ignoring unparsable %s setting\0".as_ptr() as *const ::core::ffi::c_char,
            b"audit_allowed_decision_modes\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(object);
        return 0 as ::core::ffi::c_int;
    }
    let mut item: *const cJSON = (*object).child;
    while !item.is_null() {
        let mut action: authorization_action_t = authorization_action_from_key((*item).string);
        let mut mode: audit_decision_mode_t = audit_decision_mode_t::AUDIT_DECISION_MODE_RECORD;
        if action.0 == authorization_action_t::AUTHZ_ACTION_INVALID.0
            || cJSON_IsString(item) == 0
            || audit_decision_mode_from_name((*item).valuestring, &raw mut mode)
                != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Ignoring invalid audit decision mode entry for %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if !(*item).string.is_null() {
                    (*item).string
                } else {
                    b"(null)\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        } else {
            *modes.offset(action.0 as isize) = mode;
        }
        item = (*item).next;
    }
    cJSON_Delete(object);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_audit_save_decision_modes(
    mut modes: *const audit_decision_mode_t,
) -> ::core::ffi::c_int {
    if modes.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut catalog: *const authorization_action_metadata_t =
        authorization_action_catalog(&raw mut count);
    let mut object: *mut cJSON = cJSON_CreateObject();
    if object.is_null() || catalog.is_null() {
        cJSON_Delete(object);
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut mode: audit_decision_mode_t =
            *modes.offset((*catalog.offset(i as isize)).action.0 as isize);
        if mode.0 != audit_decision_mode_t::AUDIT_DECISION_MODE_RECORD.0 {
            if cJSON_AddStringToObject(
                object,
                (*catalog.offset(i as isize)).key,
                audit_decision_mode_name(mode),
            )
            .is_null()
            {
                cJSON_Delete(object);
                return -1 as ::core::ffi::c_int;
            }
        }
        i += 1;
    }
    let mut serialized: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(object);
    cJSON_Delete(object);
    if serialized.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int =
        db_set_system_setting(AUDIT_DECISION_MODES_SETTING_KEY.as_ptr(), serialized);
    free(serialized as *mut ::core::ffi::c_void);
    return result;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
