use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type fleet_selector;
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
    unsafe fn sqlite3_busy_timeout(_: *mut sqlite3, ms: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
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
    unsafe fn sqlite3_clear_bindings(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
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
    unsafe fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_CreateString(string: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_CreateArray() -> *mut cJSON;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_AddItemToObject(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        item: *mut cJSON,
    ) -> cJSON_bool;
    unsafe fn cJSON_AddNumberToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        number: ::core::ffi::c_double,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddStringToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn fleet_selector_parse(
        json: *const cJSON,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> *mut fleet_selector_t;
    unsafe fn fleet_selector_free(selector: *mut fleet_selector_t);
    unsafe fn authorization_action_catalog(
        action_count: *mut ::core::ffi::c_int,
    ) -> *const authorization_action_metadata_t;
    unsafe fn authorization_action_from_key(
        key: *const ::core::ffi::c_char,
    ) -> authorization_action_t;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn db_open_readonly_connection(connection: *mut *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn db_close_readonly_connection(connection: *mut sqlite3);
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
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
    unsafe fn strtok_r(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
        __save_ptr: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
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
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
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
pub type uint64_t = u64;
pub type fleet_selector_t = fleet_selector;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct user_role_t(pub ::core::ffi::c_uint);
impl user_role_t {
    pub const USER_ROLE_ADMIN: Self = Self(0);
    pub const USER_ROLE_USER: Self = Self(1);
    pub const USER_ROLE_VIEWER: Self = Self(2);
    pub const USER_ROLE_API: Self = Self(3);
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
pub struct db_authorization_result_t(pub ::core::ffi::c_int);
impl db_authorization_result_t {
    pub const DB_AUTHORIZATION_OK: Self = Self(0);
    pub const DB_AUTHORIZATION_ERROR: Self = Self(-1);
    pub const DB_AUTHORIZATION_NOT_FOUND: Self = Self(-2);
    pub const DB_AUTHORIZATION_CONFLICT: Self = Self(-3);
    pub const DB_AUTHORIZATION_IMMUTABLE: Self = Self(-4);
    pub const DB_AUTHORIZATION_IN_USE: Self = Self(-5);
    pub const DB_AUTHORIZATION_STALE: Self = Self(-6);
    pub const DB_AUTHORIZATION_INVALID: Self = Self(-7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct authorization_role_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub description: [::core::ffi::c_char; 256],
    pub is_builtin: bool,
    pub action_mask: uint64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct authorization_grant_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub user_id: int64_t,
    pub role_uuid: [::core::ffi::c_char; 37],
    pub role_name: [::core::ffi::c_char; 128],
    pub scope_type: [::core::ffi::c_char; 16],
    pub selector_json: [::core::ffi::c_char; 8192],
    pub collection_uuid: [::core::ffi::c_char; 37],
    pub enabled: bool,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct authorization_grant_input_t {
    pub role_uuid: [::core::ffi::c_char; 37],
    pub scope_type: [::core::ffi::c_char; 16],
    pub selector_json: [::core::ffi::c_char; 8192],
    pub collection_uuid: [::core::ffi::c_char; 37],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct legacy_principal_t {
    pub user_id: int64_t,
    pub role: user_role_t,
    pub allowed_tags: [::core::ffi::c_char; 256],
    pub has_allowed_tags: bool,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_TOOBIG: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const USER_AUTHORIZATION_MODE_MAX: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const LOG_COMPONENT: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const AUTHORIZATION_SELECTOR_MAX: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const AUTHORIZATION_MAX_USER_GRANTS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const AUTHZ_ADMIN_ROLE_UUID: [::core::ffi::c_char; 37] = unsafe {
    ::core::mem::transmute::<[u8; 37], [::core::ffi::c_char; 37]>(
        *b"00000000-0000-4000-8000-000000000001\0",
    )
};
pub const AUTHZ_OPERATOR_ROLE_UUID: [::core::ffi::c_char; 37] = unsafe {
    ::core::mem::transmute::<[u8; 37], [::core::ffi::c_char; 37]>(
        *b"00000000-0000-4000-8000-000000000002\0",
    )
};
pub const AUTHZ_VIEWER_ROLE_UUID: [::core::ffi::c_char; 37] = unsafe {
    ::core::mem::transmute::<[u8; 37], [::core::ffi::c_char; 37]>(
        *b"00000000-0000-4000-8000-000000000003\0",
    )
};
pub const AUTHZ_API_ROLE_UUID: [::core::ffi::c_char; 37] = unsafe {
    ::core::mem::transmute::<[u8; 37], [::core::ffi::c_char; 37]>(
        *b"00000000-0000-4000-8000-000000000004\0",
    )
};
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_load_user_grants(
    mut user_id: int64_t,
    mut action_key: *const ::core::ffi::c_char,
    mut grants: *mut *mut authorization_grant_t,
    mut grant_count: *mut ::core::ffi::c_int,
    mut policy_version: *mut int64_t,
) -> ::core::ffi::c_int {
    if grants.is_null()
        || grant_count.is_null()
        || user_id <= 0 as int64_t
        || action_key.is_null()
        || *action_key.offset(0isize) == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    *grants = ::core::ptr::null_mut::<authorization_grant_t>();
    *grant_count = 0 as ::core::ffi::c_int;
    if !policy_version.is_null() {
        *policy_version = 0 as int64_t;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut result: ::core::ffi::c_int = db_authorization_load_user_grants_on_connection(
        db,
        user_id,
        action_key,
        grants,
        grant_count,
        policy_version,
    );
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_load_user_grants_on_connection(
    mut db: *mut sqlite3,
    mut user_id: int64_t,
    mut action_key: *const ::core::ffi::c_char,
    mut grants: *mut *mut authorization_grant_t,
    mut grant_count: *mut ::core::ffi::c_int,
    mut policy_version: *mut int64_t,
) -> ::core::ffi::c_int {
    if db.is_null()
        || grants.is_null()
        || grant_count.is_null()
        || user_id <= 0 as int64_t
        || action_key.is_null()
        || *action_key.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    *grants = ::core::ptr::null_mut::<authorization_grant_t>();
    *grant_count = 0 as ::core::ffi::c_int;
    if !policy_version.is_null() {
        *policy_version = 0 as int64_t;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT g.uuid, g.role_uuid, r.name, g.scope_type,        COALESCE(g.selector_json, ''),        COALESCE(g.collection_uuid, '') FROM authz_grants g JOIN authz_roles r ON r.uuid = g.role_uuid JOIN authz_role_actions ra ON ra.role_uuid = g.role_uuid WHERE g.user_id = ? AND g.enabled = 1 AND ra.action_key = ? ORDER BY g.created_at, g.uuid LIMIT ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if !policy_version.is_null() {
        let mut version_rc: ::core::ffi::c_int = sqlite3_prepare_v2(
            db,
            b"SELECT version FROM authz_policy_state WHERE id = 1;\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if version_rc != SQLITE_OK || sqlite3_step(stmt) != SQLITE_ROW {
            if !stmt.is_null() {
                sqlite3_finalize(stmt);
            }
            return -1 as ::core::ffi::c_int;
        }
        *policy_version = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to prepare authorization grant query: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    sqlite3_bind_text(
        stmt,
        2 as ::core::ffi::c_int,
        action_key,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    sqlite3_bind_int(
        stmt,
        3 as ::core::ffi::c_int,
        AUTHORIZATION_MAX_USER_GRANTS + 1 as ::core::ffi::c_int,
    );
    let mut capacity: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut loaded: *mut authorization_grant_t = calloc(
        capacity as size_t,
        ::core::mem::size_of::<authorization_grant_t>(),
    ) as *mut authorization_grant_t;
    if loaded.is_null() {
        sqlite3_finalize(stmt);
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        rc = sqlite3_step(stmt);
        if rc != SQLITE_ROW {
            break;
        }
        if count >= AUTHORIZATION_MAX_USER_GRANTS {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"User %lld exceeds the authorization grant limit\0".as_ptr()
                    as *const ::core::ffi::c_char,
                user_id as ::core::ffi::c_longlong,
            );
            rc = SQLITE_TOOBIG;
            break;
        } else {
            if count == capacity {
                let mut next_capacity: ::core::ffi::c_int = capacity * 2 as ::core::ffi::c_int;
                let mut resized: *mut authorization_grant_t = realloc(
                    loaded as *mut ::core::ffi::c_void,
                    (next_capacity as size_t)
                        .wrapping_mul(::core::mem::size_of::<authorization_grant_t>()),
                )
                    as *mut authorization_grant_t;
                if resized.is_null() {
                    rc = SQLITE_NOMEM;
                    break;
                } else {
                    loaded = resized;
                    memset(
                        loaded.offset(capacity as isize) as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ((next_capacity - capacity) as size_t)
                            .wrapping_mul(::core::mem::size_of::<authorization_grant_t>()),
                    );
                    capacity = next_capacity;
                }
            }
            let c2rust_fresh2 = count;
            count += 1;
            let mut grant: *mut authorization_grant_t = loaded.offset(c2rust_fresh2 as isize);
            copy_column(
                &raw mut (*grant).uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                stmt,
                0 as ::core::ffi::c_int,
            );
            copy_column(
                &raw mut (*grant).role_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                stmt,
                1 as ::core::ffi::c_int,
            );
            copy_column(
                &raw mut (*grant).role_name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                stmt,
                2 as ::core::ffi::c_int,
            );
            copy_column(
                &raw mut (*grant).scope_type as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                stmt,
                3 as ::core::ffi::c_int,
            );
            let mut selector: *const ::core::ffi::c_char =
                sqlite3_column_text(stmt, 4 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            if !selector.is_null()
                && strlen(selector) >= ::core::mem::size_of::<[::core::ffi::c_char; 8192]>()
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT,
                    _log_stream_name,
                    b"Authorization selector exceeds supported size for grant %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut (*grant).uuid as *mut ::core::ffi::c_char,
                );
                rc = SQLITE_TOOBIG;
                break;
            } else {
                safe_strcpy(
                    &raw mut (*grant).selector_json as *mut ::core::ffi::c_char,
                    if !selector.is_null() {
                        selector
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
                    0 as size_t,
                );
                copy_column(
                    &raw mut (*grant).collection_uuid as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                    stmt,
                    5 as ::core::ffi::c_int,
                );
            }
        }
    }
    sqlite3_finalize(stmt);
    if rc != SQLITE_DONE {
        free(loaded as *mut ::core::ffi::c_void);
        return -1 as ::core::ffi::c_int;
    }
    if count == 0 as ::core::ffi::c_int {
        free(loaded as *mut ::core::ffi::c_void);
        loaded = ::core::ptr::null_mut::<authorization_grant_t>();
    }
    *grants = loaded;
    *grant_count = count;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_get_policy_version_on_connection(
    mut db: *mut sqlite3,
    mut version: *mut int64_t,
) -> ::core::ffi::c_int {
    if version.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *version = 0 as int64_t;
    if db.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT version FROM authz_policy_state WHERE id = 1;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW {
        *version = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
        rc = SQLITE_OK;
    } else {
        rc = SQLITE_ERROR;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    return if rc == SQLITE_OK {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_get_policy_version(
    mut version: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    pthread_mutex_lock(mutex);
    let mut result: ::core::ffi::c_int =
        db_authorization_get_policy_version_on_connection(get_db_handle(), version);
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_get_policy_version_readonly(
    mut version: *mut int64_t,
) -> ::core::ffi::c_int {
    if version.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    *version = 0 as int64_t;
    let mut connection: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut connection) != 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_busy_timeout(connection, 1000 as ::core::ffi::c_int);
    let mut result: ::core::ffi::c_int =
        db_authorization_get_policy_version_on_connection(connection, version);
    db_close_readonly_connection(connection);
    return result;
}
unsafe extern "C" fn valid_selector(mut selector_json: *const ::core::ffi::c_char) -> bool {
    if selector_json.is_null()
        || *selector_json.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strlen(selector_json) >= AUTHORIZATION_SELECTOR_MAX as size_t
    {
        return r#false != 0;
    }
    let mut json: *mut cJSON = cJSON_Parse(selector_json);
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
unsafe extern "C" fn begin_policy_change(mut db: *mut sqlite3) -> bool {
    return sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ) == SQLITE_OK;
}
unsafe extern "C" fn finish_policy_change(mut db: *mut sqlite3, mut success: bool) -> bool {
    if success as ::core::ffi::c_int != 0
        && sqlite3_exec(
            db,
            b"UPDATE authz_policy_state SET version = version + 1, updated_at = strftime('%s', 'now') WHERE id = 1;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) == SQLITE_OK
        && sqlite3_exec(
            db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) == SQLITE_OK
    {
        return r#true != 0;
    }
    sqlite3_exec(
        db,
        b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    return r#false != 0;
}
unsafe extern "C" fn sqlite_constraint_result(mut rc: ::core::ffi::c_int) -> bool {
    return rc & 0xff as ::core::ffi::c_int == SQLITE_CONSTRAINT;
}
unsafe extern "C" fn shared_collection_exists_locked(
    mut db: *mut sqlite3,
    mut collection_uuid: *const ::core::ffi::c_char,
) -> bool {
    if collection_uuid.is_null()
        || *collection_uuid.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
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
            collection_uuid,
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_create_user_grant(
    mut user_id: int64_t,
    mut role_uuid: *const ::core::ffi::c_char,
    mut scope_type: *const ::core::ffi::c_char,
    mut selector_json: *const ::core::ffi::c_char,
    mut collection_uuid: *const ::core::ffi::c_char,
    mut grant_uuid: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut all_scope: bool = !scope_type.is_null()
        && strcmp(scope_type, b"all\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int;
    let mut selector_scope: bool = !scope_type.is_null()
        && strcmp(
            scope_type,
            b"selector\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
    let mut collection_scope: bool = !scope_type.is_null()
        && strcmp(
            scope_type,
            b"collection\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
    if user_id <= 0 as int64_t
        || role_uuid.is_null()
        || !all_scope && !selector_scope && !collection_scope
        || all_scope as ::core::ffi::c_int != 0
            && (!selector_json.is_null() || !collection_uuid.is_null())
        || selector_scope as ::core::ffi::c_int != 0
            && (!valid_selector(selector_json) || !collection_uuid.is_null())
        || collection_scope as ::core::ffi::c_int != 0
            && (!selector_json.is_null()
                || collection_uuid.is_null()
                || *collection_uuid.offset(0isize) as ::core::ffi::c_int
                    == '\0' as ::core::ffi::c_int)
    {
        return -1 as ::core::ffi::c_int;
    }
    if !grant_uuid.is_null() {
        *grant_uuid.offset(0isize) = '\0' as ::core::ffi::c_char;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO authz_grants (uuid,user_id,role_uuid,scope_type,selector_json,collection_uuid) VALUES (lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)),2) || '-' || substr('89ab',(abs(random()) % 4) + 1,1) || substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),?,?,?,?,?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    if !begin_policy_change(db) {
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    if collection_scope as ::core::ffi::c_int != 0
        && !shared_collection_exists_locked(db, collection_uuid)
    {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            role_uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            scope_type,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        if !selector_json.is_null() {
            sqlite3_bind_text(
                stmt,
                4 as ::core::ffi::c_int,
                selector_json,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(stmt, 4 as ::core::ffi::c_int);
        }
        if !collection_uuid.is_null() {
            sqlite3_bind_text(
                stmt,
                5 as ::core::ffi::c_int,
                collection_uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        } else {
            sqlite3_bind_null(stmt, 5 as ::core::ffi::c_int);
        }
        rc = sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    let mut created_uuid: [::core::ffi::c_char; 37] = [
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
    ];
    if rc == SQLITE_DONE {
        rc = sqlite3_prepare_v2(
            db,
            b"SELECT uuid FROM authz_grants WHERE rowid = last_insert_rowid();\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW {
            copy_column(
                &raw mut created_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                stmt,
                0 as ::core::ffi::c_int,
            );
            rc = SQLITE_DONE;
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    let mut committed: bool = finish_policy_change(
        db,
        rc == SQLITE_DONE
            && created_uuid[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int,
    );
    pthread_mutex_unlock(mutex);
    if !committed {
        return -1 as ::core::ffi::c_int;
    }
    if !grant_uuid.is_null() {
        safe_strcpy(
            grant_uuid,
            &raw mut created_uuid as *mut ::core::ffi::c_char,
            CAMERA_UUID_STRING_SIZE as size_t,
            0 as size_t,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_set_user_mode(
    mut user_id: int64_t,
    mut mode: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if user_id <= 0 as int64_t
        || mode.is_null()
        || strcmp(mode, b"legacy\0".as_ptr() as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
            && strcmp(mode, b"policy\0".as_ptr() as *const ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    if !begin_policy_change(db) {
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"UPDATE users SET authorization_mode = ?, updated_at = strftime('%s', 'now') WHERE id = ?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            mode,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, user_id as sqlite3_int64);
        rc = sqlite3_step(stmt);
    }
    let mut changed: ::core::ffi::c_int = sqlite3_changes(db);
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    let mut committed: bool =
        finish_policy_change(db, rc == SQLITE_DONE && changed == 1 as ::core::ffi::c_int);
    pthread_mutex_unlock(mutex);
    return if committed as ::core::ffi::c_int != 0 {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn policy_version_locked(mut db: *mut sqlite3) -> int64_t {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut version: int64_t = -1 as int64_t;
    if sqlite3_prepare_v2(
        db,
        b"SELECT version FROM authz_policy_state WHERE id = 1;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
        && sqlite3_step(stmt) == SQLITE_ROW
    {
        version = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    return version;
}
unsafe extern "C" fn action_bit(mut action_key: *const ::core::ffi::c_char) -> uint64_t {
    let mut action: authorization_action_t = authorization_action_from_key(action_key);
    return if action.0 >= 0 as ::core::ffi::c_int && action.0 < 64 as ::core::ffi::c_int {
        (1 as uint64_t) << action.0
    } else {
        0 as uint64_t
    };
}
unsafe extern "C" fn valid_action_mask(mut action_mask: uint64_t) -> bool {
    if action_mask == 0 as uint64_t {
        return r#false != 0;
    }
    let mut valid_mask: uint64_t =
        if authorization_action_t::AUTHZ_ACTION_COUNT.0 == 64 as ::core::ffi::c_int {
            UINT64_MAX as uint64_t
        } else {
            ((1 as uint64_t) << authorization_action_t::AUTHZ_ACTION_COUNT.0)
                .wrapping_sub(1 as uint64_t)
        };
    return action_mask & !valid_mask == 0 as uint64_t;
}
unsafe extern "C" fn load_role_actions_locked(
    mut db: *mut sqlite3,
    mut role_uuid: *const ::core::ffi::c_char,
    mut action_mask: *mut uint64_t,
) -> bool {
    let mut sql: *const ::core::ffi::c_char =
        b"SELECT action_key FROM authz_role_actions WHERE role_uuid = ?;\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        return r#false != 0;
    }
    sqlite3_bind_text(
        stmt,
        1 as ::core::ffi::c_int,
        role_uuid,
        -1 as ::core::ffi::c_int,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    let mut mask: uint64_t = 0 as uint64_t;
    let mut rc: ::core::ffi::c_int = SQLITE_ROW;
    loop {
        rc = sqlite3_step(stmt);
        if rc != SQLITE_ROW {
            break;
        }
        let mut key: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut bit: uint64_t = action_bit(key);
        if bit == 0 as uint64_t {
            sqlite3_finalize(stmt);
            return r#false != 0;
        }
        mask |= bit;
    }
    sqlite3_finalize(stmt);
    if rc != SQLITE_DONE {
        return r#false != 0;
    }
    *action_mask = mask;
    return r#true != 0;
}
unsafe extern "C" fn insert_role_actions_locked(
    mut db: *mut sqlite3,
    mut role_uuid: *const ::core::ffi::c_char,
    mut action_mask: uint64_t,
) -> bool {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        b"INSERT INTO authz_role_actions (role_uuid,action_key) VALUES (?,?);\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        return r#false != 0;
    }
    let mut action_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut catalog: *const authorization_action_metadata_t =
        authorization_action_catalog(&raw mut action_count);
    let mut success: bool = r#true != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < action_count {
        if action_mask & (1 as uint64_t) << (*catalog.offset(i as isize)).action.0 != 0 as uint64_t
        {
            sqlite3_reset(stmt);
            sqlite3_clear_bindings(stmt);
            sqlite3_bind_text(
                stmt,
                1 as ::core::ffi::c_int,
                role_uuid,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                stmt,
                2 as ::core::ffi::c_int,
                (*catalog.offset(i as isize)).key,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            if sqlite3_step(stmt) != SQLITE_DONE {
                success = r#false != 0;
                break;
            }
        }
        i += 1;
    }
    sqlite3_finalize(stmt);
    return success;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_role_count() -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if sqlite3_prepare_v2(
        db,
        b"SELECT COUNT(*) FROM authz_roles;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) == SQLITE_OK
        && sqlite3_step(stmt) == SQLITE_ROW
    {
        count = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_role_list(
    mut roles: *mut authorization_role_t,
    mut max_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if roles.is_null() || max_count <= 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT uuid,name,description,is_builtin,created_at,updated_at FROM authz_roles ORDER BY is_builtin DESC,name COLLATE NOCASE LIMIT ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    sqlite3_bind_int(stmt, 1 as ::core::ffi::c_int, max_count);
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = SQLITE_ROW;
    loop {
        rc = sqlite3_step(stmt);
        if rc != SQLITE_ROW {
            break;
        }
        let mut role: *mut authorization_role_t = roles.offset(count as isize);
        memset(
            role as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<authorization_role_t>(),
        );
        copy_column(
            &raw mut (*role).uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            stmt,
            0 as ::core::ffi::c_int,
        );
        copy_column(
            &raw mut (*role).name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            stmt,
            1 as ::core::ffi::c_int,
        );
        copy_column(
            &raw mut (*role).description as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            stmt,
            2 as ::core::ffi::c_int,
        );
        (*role).is_builtin =
            sqlite3_column_int(stmt, 3 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        (*role).created_at = sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as int64_t;
        (*role).updated_at = sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as int64_t;
        count += 1;
    }
    sqlite3_finalize(stmt);
    let mut success: bool = rc == SQLITE_DONE;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while success as ::core::ffi::c_int != 0 && i < count {
        success = load_role_actions_locked(
            db,
            &raw mut (*roles.offset(i as isize)).uuid as *mut ::core::ffi::c_char,
            &raw mut (*roles.offset(i as isize)).action_mask,
        );
        i += 1;
    }
    pthread_mutex_unlock(mutex);
    return if success as ::core::ffi::c_int != 0 {
        count
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_load_roles(
    mut roles: *mut *mut authorization_role_t,
    mut role_count: *mut ::core::ffi::c_int,
    mut policy_version: *mut int64_t,
) -> db_authorization_result_t {
    if roles.is_null() || role_count.is_null() || policy_version.is_null() {
        return db_authorization_result_t::DB_AUTHORIZATION_INVALID;
    }
    *roles = ::core::ptr::null_mut::<authorization_role_t>();
    *role_count = 0 as ::core::ffi::c_int;
    *policy_version = 0 as int64_t;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    pthread_mutex_lock(mutex);
    let mut version: int64_t = policy_version_locked(db);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if version >= 0 as int64_t
        && sqlite3_prepare_v2(
            db,
            b"SELECT COUNT(*) FROM authz_roles;\0".as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
        && sqlite3_step(stmt) == SQLITE_ROW
    {
        count = sqlite3_column_int(stmt, 0 as ::core::ffi::c_int);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if count < 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    let mut loaded: *mut authorization_role_t = (if count > 0 as ::core::ffi::c_int {
        calloc(
            count as size_t,
            ::core::mem::size_of::<authorization_role_t>(),
        )
    } else {
        NULL
    }) as *mut authorization_role_t;
    if count > 0 as ::core::ffi::c_int && loaded.is_null() {
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT uuid,name,description,is_builtin,created_at,updated_at FROM authz_roles ORDER BY is_builtin DESC,name COLLATE NOCASE;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut loaded_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if rc == SQLITE_OK {
        loop {
            rc = sqlite3_step(stmt);
            if rc != SQLITE_ROW {
                break;
            }
            if loaded_count >= count {
                rc = SQLITE_TOOBIG;
                break;
            } else {
                let c2rust_fresh0 = loaded_count;
                loaded_count += 1;
                let mut role: *mut authorization_role_t = loaded.offset(c2rust_fresh0 as isize);
                copy_column(
                    &raw mut (*role).uuid as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                    stmt,
                    0 as ::core::ffi::c_int,
                );
                copy_column(
                    &raw mut (*role).name as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                    stmt,
                    1 as ::core::ffi::c_int,
                );
                copy_column(
                    &raw mut (*role).description as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    stmt,
                    2 as ::core::ffi::c_int,
                );
                (*role).is_builtin =
                    sqlite3_column_int(stmt, 3 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
                (*role).created_at = sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as int64_t;
                (*role).updated_at = sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as int64_t;
            }
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    let mut success: bool = rc == SQLITE_DONE && loaded_count == count;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while success as ::core::ffi::c_int != 0 && i < loaded_count {
        success = load_role_actions_locked(
            db,
            &raw mut (*loaded.offset(i as isize)).uuid as *mut ::core::ffi::c_char,
            &raw mut (*loaded.offset(i as isize)).action_mask,
        );
        i += 1;
    }
    pthread_mutex_unlock(mutex);
    if !success {
        free(loaded as *mut ::core::ffi::c_void);
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    *roles = loaded;
    *role_count = loaded_count;
    *policy_version = version;
    return db_authorization_result_t::DB_AUTHORIZATION_OK;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_role_get(
    mut uuid: *const ::core::ffi::c_char,
    mut role: *mut authorization_role_t,
) -> db_authorization_result_t {
    if uuid.is_null() || role.is_null() {
        return db_authorization_result_t::DB_AUTHORIZATION_INVALID;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = b"SELECT uuid,name,description,is_builtin,created_at,updated_at FROM authz_roles WHERE uuid = ?;\0"
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
    let mut result: db_authorization_result_t = db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    if rc == SQLITE_ROW {
        memset(
            role as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<authorization_role_t>(),
        );
        copy_column(
            &raw mut (*role).uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            stmt,
            0 as ::core::ffi::c_int,
        );
        copy_column(
            &raw mut (*role).name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            stmt,
            1 as ::core::ffi::c_int,
        );
        copy_column(
            &raw mut (*role).description as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            stmt,
            2 as ::core::ffi::c_int,
        );
        (*role).is_builtin =
            sqlite3_column_int(stmt, 3 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
        (*role).created_at = sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as int64_t;
        (*role).updated_at = sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as int64_t;
        result = db_authorization_result_t::DB_AUTHORIZATION_OK;
    } else if rc == SQLITE_DONE {
        result = db_authorization_result_t::DB_AUTHORIZATION_NOT_FOUND;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if result.0 == db_authorization_result_t::DB_AUTHORIZATION_OK.0
        && !load_role_actions_locked(db, uuid, &raw mut (*role).action_mask)
    {
        result = db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    pthread_mutex_unlock(mutex);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_role_create(
    mut role: *mut authorization_role_t,
    mut expected_version: int64_t,
    mut new_version: *mut int64_t,
) -> db_authorization_result_t {
    if role.is_null()
        || new_version.is_null()
        || expected_version < 1 as int64_t
        || (*role).name[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || !valid_action_mask((*role).action_mask)
    {
        return db_authorization_result_t::DB_AUTHORIZATION_INVALID;
    }
    *new_version = 0 as int64_t;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    let mut sql: *const ::core::ffi::c_char = b"INSERT INTO authz_roles (uuid,name,description,is_builtin) VALUES (lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)),2) || '-' || substr('89ab',(abs(random()) % 4) + 1,1) || substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),?,?,0);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    pthread_mutex_lock(mutex);
    if !begin_policy_change(db) {
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    let mut current_version: int64_t = policy_version_locked(db);
    if current_version != expected_version {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t(if current_version < 0 as int64_t {
            db_authorization_result_t::DB_AUTHORIZATION_ERROR.0
        } else {
            db_authorization_result_t::DB_AUTHORIZATION_STALE.0
        });
    }
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
            &raw mut (*role).name as *mut ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw mut (*role).description as *mut ::core::ffi::c_char,
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
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    }
    let mut uuid: [::core::ffi::c_char; 37] = [
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
    ];
    if rc == SQLITE_DONE
        && sqlite3_prepare_v2(
            db,
            b"SELECT uuid FROM authz_roles WHERE rowid=last_insert_rowid();\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK
        && sqlite3_step(stmt) == SQLITE_ROW
    {
        copy_column(
            &raw mut uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            stmt,
            0 as ::core::ffi::c_int,
        );
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    let mut success: bool = uuid[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        && insert_role_actions_locked(
            db,
            &raw mut uuid as *mut ::core::ffi::c_char,
            (*role).action_mask,
        ) as ::core::ffi::c_int
            != 0;
    let mut committed: bool = finish_policy_change(db, success);
    pthread_mutex_unlock(mutex);
    if !committed {
        return db_authorization_result_t(
            if sqlite_constraint_result(rc) as ::core::ffi::c_int != 0 {
                db_authorization_result_t::DB_AUTHORIZATION_CONFLICT.0
            } else {
                db_authorization_result_t::DB_AUTHORIZATION_ERROR.0
            },
        );
    }
    safe_strcpy(
        &raw mut (*role).uuid as *mut ::core::ffi::c_char,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    (*role).is_builtin = r#false != 0;
    *new_version = current_version + 1 as int64_t;
    return db_authorization_result_t::DB_AUTHORIZATION_OK;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_role_update(
    mut role: *const authorization_role_t,
    mut expected_version: int64_t,
    mut new_version: *mut int64_t,
) -> db_authorization_result_t {
    if role.is_null()
        || new_version.is_null()
        || expected_version < 1 as int64_t
        || (*role).uuid[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || (*role).name[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || !valid_action_mask((*role).action_mask)
    {
        return db_authorization_result_t::DB_AUTHORIZATION_INVALID;
    }
    *new_version = 0 as int64_t;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    pthread_mutex_lock(mutex);
    if !begin_policy_change(db) {
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    let mut current_version: int64_t = policy_version_locked(db);
    if current_version != expected_version {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t(if current_version < 0 as int64_t {
            db_authorization_result_t::DB_AUTHORIZATION_ERROR.0
        } else {
            db_authorization_result_t::DB_AUTHORIZATION_STALE.0
        });
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT is_builtin FROM authz_roles WHERE uuid=?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw const (*role).uuid as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
    }
    if rc == SQLITE_DONE {
        sqlite3_finalize(stmt);
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t::DB_AUTHORIZATION_NOT_FOUND;
    }
    if rc != SQLITE_ROW
        || sqlite3_column_int(stmt, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int
    {
        let mut immutable: bool = rc == SQLITE_ROW;
        sqlite3_finalize(stmt);
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t(if immutable as ::core::ffi::c_int != 0 {
            db_authorization_result_t::DB_AUTHORIZATION_IMMUTABLE.0
        } else {
            db_authorization_result_t::DB_AUTHORIZATION_ERROR.0
        });
    }
    sqlite3_finalize(stmt);
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = sqlite3_prepare_v2(
        db,
        b"UPDATE authz_roles SET name=?,description=?,updated_at=strftime('%s','now') WHERE uuid=?;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            &raw const (*role).name as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            2 as ::core::ffi::c_int,
            &raw const (*role).description as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            stmt,
            3 as ::core::ffi::c_int,
            &raw const (*role).uuid as *const ::core::ffi::c_char,
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
    let mut success: bool = rc == SQLITE_DONE;
    if success {
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        success = sqlite3_prepare_v2(
            db,
            b"DELETE FROM authz_role_actions WHERE role_uuid=?;\0".as_ptr()
                as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ) == SQLITE_OK;
        if success {
            sqlite3_bind_text(
                stmt,
                1 as ::core::ffi::c_int,
                &raw const (*role).uuid as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            success = sqlite3_step(stmt) == SQLITE_DONE;
        }
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
        }
    }
    if success {
        success = insert_role_actions_locked(
            db,
            &raw const (*role).uuid as *const ::core::ffi::c_char,
            (*role).action_mask,
        );
    }
    let mut committed: bool = finish_policy_change(db, success);
    pthread_mutex_unlock(mutex);
    if !committed {
        return db_authorization_result_t(
            if sqlite_constraint_result(rc) as ::core::ffi::c_int != 0 {
                db_authorization_result_t::DB_AUTHORIZATION_CONFLICT.0
            } else {
                db_authorization_result_t::DB_AUTHORIZATION_ERROR.0
            },
        );
    }
    *new_version = current_version + 1 as int64_t;
    return db_authorization_result_t::DB_AUTHORIZATION_OK;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_role_delete(
    mut uuid: *const ::core::ffi::c_char,
    mut expected_version: int64_t,
    mut new_version: *mut int64_t,
) -> db_authorization_result_t {
    if uuid.is_null()
        || *uuid.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || new_version.is_null()
        || expected_version < 1 as int64_t
    {
        return db_authorization_result_t::DB_AUTHORIZATION_INVALID;
    }
    *new_version = 0 as int64_t;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    pthread_mutex_lock(mutex);
    if !begin_policy_change(db) {
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    let mut current_version: int64_t = policy_version_locked(db);
    if current_version != expected_version {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t(if current_version < 0 as int64_t {
            db_authorization_result_t::DB_AUTHORIZATION_ERROR.0
        } else {
            db_authorization_result_t::DB_AUTHORIZATION_STALE.0
        });
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT is_builtin FROM authz_roles WHERE uuid=?;\0".as_ptr()
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
    if rc == SQLITE_DONE {
        sqlite3_finalize(stmt);
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t::DB_AUTHORIZATION_NOT_FOUND;
    }
    if rc != SQLITE_ROW
        || sqlite3_column_int(stmt, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int
    {
        let mut immutable: bool = rc == SQLITE_ROW;
        sqlite3_finalize(stmt);
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t(if immutable as ::core::ffi::c_int != 0 {
            db_authorization_result_t::DB_AUTHORIZATION_IMMUTABLE.0
        } else {
            db_authorization_result_t::DB_AUTHORIZATION_ERROR.0
        });
    }
    sqlite3_finalize(stmt);
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = sqlite3_prepare_v2(
        db,
        b"DELETE FROM authz_roles WHERE uuid=?;\0".as_ptr() as *const ::core::ffi::c_char,
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
    let mut success: bool = rc == SQLITE_DONE && sqlite3_changes(db) == 1 as ::core::ffi::c_int;
    let mut committed: bool = finish_policy_change(db, success);
    pthread_mutex_unlock(mutex);
    if committed {
        *new_version = current_version + 1 as int64_t;
        return db_authorization_result_t::DB_AUTHORIZATION_OK;
    }
    return db_authorization_result_t(if sqlite_constraint_result(rc) as ::core::ffi::c_int != 0 {
        db_authorization_result_t::DB_AUTHORIZATION_IN_USE.0
    } else {
        db_authorization_result_t::DB_AUTHORIZATION_ERROR.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_get_user_policy(
    mut user_id: int64_t,
    mut mode: *mut ::core::ffi::c_char,
    mut grants: *mut *mut authorization_grant_t,
    mut grant_count: *mut ::core::ffi::c_int,
    mut policy_version: *mut int64_t,
) -> db_authorization_result_t {
    if user_id <= 0 as int64_t
        || mode.is_null()
        || grants.is_null()
        || grant_count.is_null()
        || policy_version.is_null()
    {
        return db_authorization_result_t::DB_AUTHORIZATION_INVALID;
    }
    *mode.offset(0isize) = '\0' as ::core::ffi::c_char;
    *grants = ::core::ptr::null_mut::<authorization_grant_t>();
    *grant_count = 0 as ::core::ffi::c_int;
    *policy_version = 0 as int64_t;
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    pthread_mutex_lock(mutex);
    *policy_version = policy_version_locked(db);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT authorization_mode FROM users WHERE id=?;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
        rc = sqlite3_step(stmt);
    }
    if rc == SQLITE_DONE {
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t::DB_AUTHORIZATION_NOT_FOUND;
    }
    if rc != SQLITE_ROW || *policy_version < 0 as int64_t {
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
        }
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    copy_column(
        mode,
        USER_AUTHORIZATION_MODE_MAX as size_t,
        stmt,
        0 as ::core::ffi::c_int,
    );
    sqlite3_finalize(stmt);
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char = b"SELECT g.uuid,g.user_id,g.role_uuid,r.name,g.scope_type,COALESCE(g.selector_json,''),COALESCE(g.collection_uuid,''),g.enabled,g.created_at,g.updated_at FROM authz_grants g JOIN authz_roles r ON r.uuid=g.role_uuid WHERE g.user_id=? ORDER BY g.created_at,g.uuid LIMIT ?;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    if sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ) != SQLITE_OK
    {
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
        }
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
    sqlite3_bind_int(
        stmt,
        2 as ::core::ffi::c_int,
        AUTHORIZATION_MAX_USER_GRANTS + 1 as ::core::ffi::c_int,
    );
    let mut capacity: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut loaded: *mut authorization_grant_t = calloc(
        capacity as size_t,
        ::core::mem::size_of::<authorization_grant_t>(),
    ) as *mut authorization_grant_t;
    if loaded.is_null() {
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        rc = sqlite3_step(stmt);
        if rc != SQLITE_ROW {
            break;
        }
        if count >= AUTHORIZATION_MAX_USER_GRANTS {
            rc = SQLITE_TOOBIG;
            break;
        } else {
            if count == capacity {
                let mut next_capacity: ::core::ffi::c_int = capacity * 2 as ::core::ffi::c_int;
                let mut resized: *mut authorization_grant_t = realloc(
                    loaded as *mut ::core::ffi::c_void,
                    (next_capacity as size_t)
                        .wrapping_mul(::core::mem::size_of::<authorization_grant_t>()),
                )
                    as *mut authorization_grant_t;
                if resized.is_null() {
                    rc = SQLITE_NOMEM;
                    break;
                } else {
                    loaded = resized;
                    memset(
                        loaded.offset(capacity as isize) as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ((next_capacity - capacity) as size_t)
                            .wrapping_mul(::core::mem::size_of::<authorization_grant_t>()),
                    );
                    capacity = next_capacity;
                }
            }
            let c2rust_fresh1 = count;
            count += 1;
            let mut grant: *mut authorization_grant_t = loaded.offset(c2rust_fresh1 as isize);
            copy_column(
                &raw mut (*grant).uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                stmt,
                0 as ::core::ffi::c_int,
            );
            (*grant).user_id = sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as int64_t;
            copy_column(
                &raw mut (*grant).role_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                stmt,
                2 as ::core::ffi::c_int,
            );
            copy_column(
                &raw mut (*grant).role_name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                stmt,
                3 as ::core::ffi::c_int,
            );
            copy_column(
                &raw mut (*grant).scope_type as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
                stmt,
                4 as ::core::ffi::c_int,
            );
            let mut selector: *const ::core::ffi::c_char =
                sqlite3_column_text(stmt, 5 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            if !selector.is_null()
                && strlen(selector) >= ::core::mem::size_of::<[::core::ffi::c_char; 8192]>()
            {
                rc = SQLITE_TOOBIG;
                break;
            } else {
                safe_strcpy(
                    &raw mut (*grant).selector_json as *mut ::core::ffi::c_char,
                    if !selector.is_null() {
                        selector
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
                    0 as size_t,
                );
                copy_column(
                    &raw mut (*grant).collection_uuid as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                    stmt,
                    6 as ::core::ffi::c_int,
                );
                (*grant).enabled =
                    sqlite3_column_int(stmt, 7 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int;
                (*grant).created_at =
                    sqlite3_column_int64(stmt, 8 as ::core::ffi::c_int) as int64_t;
                (*grant).updated_at =
                    sqlite3_column_int64(stmt, 9 as ::core::ffi::c_int) as int64_t;
            }
        }
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    if rc != SQLITE_DONE {
        free(loaded as *mut ::core::ffi::c_void);
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    if count == 0 as ::core::ffi::c_int {
        free(loaded as *mut ::core::ffi::c_void);
        loaded = ::core::ptr::null_mut::<authorization_grant_t>();
    }
    *grants = loaded;
    *grant_count = count;
    return db_authorization_result_t::DB_AUTHORIZATION_OK;
}
unsafe extern "C" fn duplicate_grant_input(
    mut grants: *const authorization_grant_input_t,
    mut index: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < index {
        if strcmp(
            &raw const (*grants.offset(i as isize)).role_uuid as *const ::core::ffi::c_char,
            &raw const (*grants.offset(index as isize)).role_uuid as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && strcmp(
                &raw const (*grants.offset(i as isize)).scope_type as *const ::core::ffi::c_char,
                &raw const (*grants.offset(index as isize)).scope_type
                    as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            && strcmp(
                &raw const (*grants.offset(i as isize)).selector_json as *const ::core::ffi::c_char,
                &raw const (*grants.offset(index as isize)).selector_json
                    as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            && strcmp(
                &raw const (*grants.offset(i as isize)).collection_uuid
                    as *const ::core::ffi::c_char,
                &raw const (*grants.offset(index as isize)).collection_uuid
                    as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn valid_grant_input(mut grant: *const authorization_grant_input_t) -> bool {
    if grant.is_null()
        || (*grant).role_uuid[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    if strcmp(
        &raw const (*grant).scope_type as *const ::core::ffi::c_char,
        b"all\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return (*grant).selector_json[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            && (*grant).collection_uuid[0usize] as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int;
    }
    if strcmp(
        &raw const (*grant).scope_type as *const ::core::ffi::c_char,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return (*grant).collection_uuid[0usize] as ::core::ffi::c_int
            == '\0' as ::core::ffi::c_int
            && valid_selector(&raw const (*grant).selector_json as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0;
    }
    return strcmp(
        &raw const (*grant).scope_type as *const ::core::ffi::c_char,
        b"collection\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && (*grant).selector_json[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        && (*grant).collection_uuid[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_replace_user_policy(
    mut user_id: int64_t,
    mut mode: *const ::core::ffi::c_char,
    mut grants: *const authorization_grant_input_t,
    mut grant_count: ::core::ffi::c_int,
    mut expected_version: int64_t,
    mut new_version: *mut int64_t,
) -> db_authorization_result_t {
    if user_id <= 0 as int64_t
        || mode.is_null()
        || new_version.is_null()
        || expected_version < 1 as int64_t
        || grant_count < 0 as ::core::ffi::c_int
        || grant_count > AUTHORIZATION_MAX_USER_GRANTS
        || grant_count > 0 as ::core::ffi::c_int && grants.is_null()
        || strcmp(mode, b"legacy\0".as_ptr() as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
            && strcmp(mode, b"policy\0".as_ptr() as *const ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
    {
        return db_authorization_result_t::DB_AUTHORIZATION_INVALID;
    }
    *new_version = 0 as int64_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < grant_count {
        if !valid_grant_input(grants.offset(i as isize))
            || duplicate_grant_input(grants, i) as ::core::ffi::c_int != 0
        {
            return db_authorization_result_t::DB_AUTHORIZATION_INVALID;
        }
        i += 1;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    pthread_mutex_lock(mutex);
    if !begin_policy_change(db) {
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    let mut current_version: int64_t = policy_version_locked(db);
    if current_version != expected_version {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t(if current_version < 0 as int64_t {
            db_authorization_result_t::DB_AUTHORIZATION_ERROR.0
        } else {
            db_authorization_result_t::DB_AUTHORIZATION_STALE.0
        });
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT 1 FROM users WHERE id=?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
        rc = sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if rc == SQLITE_DONE {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t::DB_AUTHORIZATION_NOT_FOUND;
    }
    if rc != SQLITE_ROW {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return db_authorization_result_t::DB_AUTHORIZATION_ERROR;
    }
    let mut role_sql: *const ::core::ffi::c_char =
        b"SELECT 1 FROM authz_roles WHERE uuid=?;\0".as_ptr() as *const ::core::ffi::c_char;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < grant_count {
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = sqlite3_prepare_v2(
            db,
            role_sql,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_text(
                stmt,
                1 as ::core::ffi::c_int,
                &raw const (*grants.offset(i_0 as isize)).role_uuid as *const ::core::ffi::c_char,
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
        if rc != SQLITE_ROW {
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            pthread_mutex_unlock(mutex);
            return db_authorization_result_t(if rc == SQLITE_DONE {
                db_authorization_result_t::DB_AUTHORIZATION_INVALID.0
            } else {
                db_authorization_result_t::DB_AUTHORIZATION_ERROR.0
            });
        }
        if strcmp(
            &raw const (*grants.offset(i_0 as isize)).scope_type as *const ::core::ffi::c_char,
            b"collection\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && !shared_collection_exists_locked(
                db,
                &raw const (*grants.offset(i_0 as isize)).collection_uuid
                    as *const ::core::ffi::c_char,
            )
        {
            sqlite3_exec(
                db,
                b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                NULL,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            pthread_mutex_unlock(mutex);
            return db_authorization_result_t::DB_AUTHORIZATION_INVALID;
        }
        i_0 += 1;
    }
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = sqlite3_prepare_v2(
        db,
        b"DELETE FROM authz_grants WHERE user_id=?;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
        rc = sqlite3_step(stmt);
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    let mut insert_sql: *const ::core::ffi::c_char = b"INSERT INTO authz_grants (uuid,user_id,role_uuid,scope_type,selector_json,collection_uuid) VALUES (lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)),2) || '-' || substr('89ab',(abs(random()) % 4) + 1,1) || substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),?,?,?,?,?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while rc == SQLITE_DONE && i_1 < grant_count {
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = sqlite3_prepare_v2(
            db,
            insert_sql,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_int64(stmt, 1 as ::core::ffi::c_int, user_id as sqlite3_int64);
            sqlite3_bind_text(
                stmt,
                2 as ::core::ffi::c_int,
                &raw const (*grants.offset(i_1 as isize)).role_uuid as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_text(
                stmt,
                3 as ::core::ffi::c_int,
                &raw const (*grants.offset(i_1 as isize)).scope_type as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            if (*grants.offset(i_1 as isize)).selector_json[0usize] != 0 {
                sqlite3_bind_text(
                    stmt,
                    4 as ::core::ffi::c_int,
                    &raw const (*grants.offset(i_1 as isize)).selector_json
                        as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            } else {
                sqlite3_bind_null(stmt, 4 as ::core::ffi::c_int);
            }
            if (*grants.offset(i_1 as isize)).collection_uuid[0usize] != 0 {
                sqlite3_bind_text(
                    stmt,
                    5 as ::core::ffi::c_int,
                    &raw const (*grants.offset(i_1 as isize)).collection_uuid
                        as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            } else {
                sqlite3_bind_null(stmt, 5 as ::core::ffi::c_int);
            }
            rc = sqlite3_step(stmt);
        }
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
        }
        i_1 += 1;
    }
    if rc == SQLITE_DONE {
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = sqlite3_prepare_v2(
            db,
            b"UPDATE users SET authorization_mode=?,updated_at=strftime('%s','now') WHERE id=?;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_text(
                stmt,
                1 as ::core::ffi::c_int,
                mode,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, user_id as sqlite3_int64);
            rc = sqlite3_step(stmt);
        }
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
        }
    }
    let mut committed: bool = finish_policy_change(db, rc == SQLITE_DONE);
    if committed {
        *new_version = current_version + 1 as int64_t;
    }
    pthread_mutex_unlock(mutex);
    return db_authorization_result_t(if committed as ::core::ffi::c_int != 0 {
        db_authorization_result_t::DB_AUTHORIZATION_OK.0
    } else {
        db_authorization_result_t::DB_AUTHORIZATION_ERROR.0
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_verify_action_catalog() -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut catalog_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    authorization_action_catalog(&raw mut catalog_count);
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT action_key,bit_index FROM authz_actions;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Failed to read the authorization action catalog: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            sqlite3_errmsg(db),
        );
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
        }
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut matched: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mismatches: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        rc = sqlite3_step(stmt);
        if rc != SQLITE_ROW {
            break;
        }
        let mut key: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        let mut bit_index: int64_t = sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as int64_t;
        let mut action: authorization_action_t = authorization_action_from_key(key);
        if action.0 == authorization_action_t::AUTHZ_ACTION_INVALID.0 {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Action '%s' exists in authz_actions but not in this build's catalog\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if !key.is_null() {
                    key
                } else {
                    b"(null)\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            mismatches += 1;
        } else if bit_index != action.0 as int64_t {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT,
                _log_stream_name,
                b"Action '%s' is stored at mask bit %lld but this build uses bit %d; API token permissions would be re-mapped\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                key,
                bit_index as ::core::ffi::c_longlong,
                action.0,
            );
            mismatches += 1;
        } else {
            matched += 1;
        }
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    if rc != SQLITE_DONE {
        return -1 as ::core::ffi::c_int;
    }
    if matched != catalog_count {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT,
            _log_stream_name,
            b"Authorization action catalog has %d of %d expected actions\0".as_ptr()
                as *const ::core::ffi::c_char,
            matched,
            catalog_count,
        );
        return -1 as ::core::ffi::c_int;
    }
    return if mismatches == 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn compatibility_role_uuid(mut role: user_role_t) -> *const ::core::ffi::c_char {
    match role {
        user_role_t::USER_ROLE_ADMIN => return AUTHZ_ADMIN_ROLE_UUID.as_ptr(),
        user_role_t::USER_ROLE_USER => return AUTHZ_OPERATOR_ROLE_UUID.as_ptr(),
        user_role_t::USER_ROLE_VIEWER => return AUTHZ_VIEWER_ROLE_UUID.as_ptr(),
        user_role_t::USER_ROLE_API => return AUTHZ_API_ROLE_UUID.as_ptr(),
        _ => return ::core::ptr::null::<::core::ffi::c_char>(),
    };
}
unsafe extern "C" fn lookup_or_create_tag_uuid_locked(
    mut db: *mut sqlite3,
    mut label: *const ::core::ffi::c_char,
    mut uuid: *mut ::core::ffi::c_char,
) -> bool {
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT uuid FROM camera_tags WHERE label=? COLLATE NOCASE;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            label,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
    }
    if rc == SQLITE_ROW {
        copy_column(
            uuid,
            CAMERA_UUID_STRING_SIZE as size_t,
            stmt,
            0 as ::core::ffi::c_int,
        );
        sqlite3_finalize(stmt);
        return *uuid.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if rc != SQLITE_DONE {
        return r#false != 0;
    }
    let mut insert_sql: *const ::core::ffi::c_char = b"INSERT INTO camera_tags (uuid,label) VALUES (lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)),2) || '-' || substr('89ab',(abs(random()) % 4) + 1,1) || substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),?);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = sqlite3_prepare_v2(
        db,
        insert_sql,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            label,
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
    if rc != SQLITE_DONE {
        return r#false != 0;
    }
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = sqlite3_prepare_v2(
        db,
        b"SELECT uuid FROM camera_tags WHERE label=? COLLATE NOCASE;\0".as_ptr()
            as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        sqlite3_bind_text(
            stmt,
            1 as ::core::ffi::c_int,
            label,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        rc = sqlite3_step(stmt);
    }
    if rc == SQLITE_ROW {
        copy_column(
            uuid,
            CAMERA_UUID_STRING_SIZE as size_t,
            stmt,
            0 as ::core::ffi::c_int,
        );
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    return rc == SQLITE_ROW
        && *uuid.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
}
unsafe extern "C" fn trim_legacy_tag(
    mut value: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if value.is_null() {
        return value;
    }
    while *value as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || *value as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        || *value as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
        || *value as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
    {
        value = value.offset(1);
    }
    let mut length: size_t = strlen(value);
    while length > 0 as size_t
        && (*value.offset(length.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            == ' ' as ::core::ffi::c_int
            || *value.offset(length.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                == '\t' as ::core::ffi::c_int
            || *value.offset(length.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                == '\r' as ::core::ffi::c_int
            || *value.offset(length.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                == '\n' as ::core::ffi::c_int)
    {
        length = length.wrapping_sub(1);
        *value.offset(length as isize) = '\0' as ::core::ffi::c_char;
    }
    return value;
}
unsafe extern "C" fn legacy_tag_selector_locked(
    mut db: *mut sqlite3,
    mut allowed_tags: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if allowed_tags.is_null()
        || *allowed_tags.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut copy: [::core::ffi::c_char; 256] = [0; 256];
    safe_strcpy(
        &raw mut copy as *mut ::core::ffi::c_char,
        allowed_tags,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut expression: *mut cJSON = cJSON_CreateObject();
    let mut uuids: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || expression.is_null() || uuids.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(expression);
        cJSON_Delete(uuids);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    cJSON_AddNumberToObject(
        root,
        b"version\0".as_ptr() as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        expression,
        b"op\0".as_ptr() as *const ::core::ffi::c_char,
        b"tag_any\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddItemToObject(
        expression,
        b"uuids\0".as_ptr() as *const ::core::ffi::c_char,
        uuids,
    );
    cJSON_AddItemToObject(
        root,
        b"expression\0".as_ptr() as *const ::core::ffi::c_char,
        expression,
    );
    let mut tag_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut item: *mut ::core::ffi::c_char = strtok_r(
        &raw mut copy as *mut ::core::ffi::c_char,
        b",\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut saveptr,
    );
    while !item.is_null() {
        let mut label: *mut ::core::ffi::c_char = trim_legacy_tag(item);
        if *label.offset(0isize) != 0 {
            let mut uuid: [::core::ffi::c_char; 37] = [
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
            ];
            if !lookup_or_create_tag_uuid_locked(
                db,
                label,
                &raw mut uuid as *mut ::core::ffi::c_char,
            ) || cJSON_AddItemToArray(
                uuids,
                cJSON_CreateString(&raw mut uuid as *mut ::core::ffi::c_char),
            ) == 0
            {
                cJSON_Delete(root);
                return ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            tag_count += 1;
        }
        item = strtok_r(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b",\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
    }
    if tag_count == 0 as ::core::ffi::c_int {
        cJSON_Delete(root);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut serialized: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if !serialized.is_null() && strlen(serialized) >= AUTHORIZATION_SELECTOR_MAX as size_t {
        free(serialized as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return serialized;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn db_authorization_migrate_legacy_users(
    mut migrated_count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !migrated_count.is_null() {
        *migrated_count = 0 as ::core::ffi::c_int;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(mutex);
    if !begin_policy_change(db) {
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT id,role,allowed_tags FROM users WHERE authorization_mode='legacy' ORDER BY id;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut principals: *mut legacy_principal_t = ::core::ptr::null_mut::<legacy_principal_t>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut capacity: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut step_rc: ::core::ffi::c_int = if rc == SQLITE_OK { SQLITE_ROW } else { rc };
    while rc == SQLITE_OK && {
        step_rc = sqlite3_step(stmt);
        step_rc == SQLITE_ROW
    } {
        if count == capacity {
            let mut next: ::core::ffi::c_int = if capacity == 0 as ::core::ffi::c_int {
                8 as ::core::ffi::c_int
            } else {
                capacity * 2 as ::core::ffi::c_int
            };
            let mut resized: *mut ::core::ffi::c_void = realloc(
                principals as *mut ::core::ffi::c_void,
                (next as size_t).wrapping_mul(::core::mem::size_of::<legacy_principal_t>()),
            );
            if resized.is_null() {
                rc = SQLITE_NOMEM;
                break;
            } else {
                principals = resized as *mut legacy_principal_t;
                capacity = next;
            }
        }
        let c2rust_fresh3 = count;
        count += 1;
        let mut principal: *mut legacy_principal_t = principals.offset(c2rust_fresh3 as isize);
        memset(
            principal as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<legacy_principal_t>(),
        );
        (*principal).user_id = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
        (*principal).role =
            user_role_t(sqlite3_column_int(stmt, 1 as ::core::ffi::c_int) as ::core::ffi::c_uint);
        let mut tags: *const ::core::ffi::c_char =
            sqlite3_column_text(stmt, 2 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
        (*principal).has_allowed_tags = !tags.is_null()
            && *tags.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
        safe_strcpy(
            &raw mut (*principal).allowed_tags as *mut ::core::ffi::c_char,
            if !tags.is_null() {
                tags
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    if rc != SQLITE_OK || step_rc != SQLITE_DONE {
        free(principals as *mut ::core::ffi::c_void);
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        pthread_mutex_unlock(mutex);
        return -1 as ::core::ffi::c_int;
    }
    if count == 0 as ::core::ffi::c_int {
        free(principals as *mut ::core::ffi::c_void);
        let mut committed: bool = sqlite3_exec(
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
        return if committed as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int
        } else {
            -1 as ::core::ffi::c_int
        };
    }
    let mut insert_sql: *const ::core::ffi::c_char = b"INSERT INTO authz_grants (uuid,user_id,role_uuid,scope_type,selector_json,collection_uuid) VALUES (lower(hex(randomblob(4)) || '-' || hex(randomblob(2)) || '-4' || substr(hex(randomblob(2)),2) || '-' || substr('89ab',(abs(random()) % 4) + 1,1) || substr(hex(randomblob(2)),2) || '-' || hex(randomblob(6))),?,?,?,?,NULL);\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut success: bool = r#true != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while success as ::core::ffi::c_int != 0 && i < count {
        let mut role_uuid: *const ::core::ffi::c_char =
            compatibility_role_uuid((*principals.offset(i as isize)).role);
        if role_uuid.is_null() {
            success = r#false != 0;
            break;
        } else {
            stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            rc = sqlite3_prepare_v2(
                db,
                b"DELETE FROM authz_grants WHERE user_id=?;\0".as_ptr()
                    as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                &raw mut stmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            if rc == SQLITE_OK {
                sqlite3_bind_int64(
                    stmt,
                    1 as ::core::ffi::c_int,
                    (*principals.offset(i as isize)).user_id as sqlite3_int64,
                );
                rc = sqlite3_step(stmt);
            }
            if !stmt.is_null() {
                sqlite3_finalize(stmt);
            }
            if rc != SQLITE_DONE {
                success = r#false != 0;
                break;
            } else {
                let mut selector: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                if (*principals.offset(i as isize)).has_allowed_tags {
                    selector = legacy_tag_selector_locked(
                        db,
                        &raw mut (*principals.offset(i as isize)).allowed_tags
                            as *mut ::core::ffi::c_char,
                    );
                    if selector.is_null() {
                        success = r#false != 0;
                        break;
                    }
                }
                stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
                rc = sqlite3_prepare_v2(
                    db,
                    insert_sql,
                    -1 as ::core::ffi::c_int,
                    &raw mut stmt,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                if rc == SQLITE_OK {
                    sqlite3_bind_int64(
                        stmt,
                        1 as ::core::ffi::c_int,
                        (*principals.offset(i as isize)).user_id as sqlite3_int64,
                    );
                    sqlite3_bind_text(
                        stmt,
                        2 as ::core::ffi::c_int,
                        role_uuid,
                        -1 as ::core::ffi::c_int,
                        SQLITE_STATIC,
                    );
                    sqlite3_bind_text(
                        stmt,
                        3 as ::core::ffi::c_int,
                        if !selector.is_null() {
                            b"selector\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"all\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        -1 as ::core::ffi::c_int,
                        SQLITE_STATIC,
                    );
                    if !selector.is_null() {
                        sqlite3_bind_text(
                            stmt,
                            4 as ::core::ffi::c_int,
                            selector,
                            -1 as ::core::ffi::c_int,
                            ::core::mem::transmute::<
                                ::libc::intptr_t,
                                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                            >(
                                -1 as ::core::ffi::c_int as ::libc::intptr_t
                            ),
                        );
                    } else {
                        sqlite3_bind_null(stmt, 4 as ::core::ffi::c_int);
                    }
                    rc = sqlite3_step(stmt);
                }
                if !stmt.is_null() {
                    sqlite3_finalize(stmt);
                }
                if rc != SQLITE_DONE {
                    success = r#false != 0;
                }
                free(selector as *mut ::core::ffi::c_void);
                if !success {
                    break;
                }
                stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
                rc = sqlite3_prepare_v2(
                    db,
                    b"UPDATE users SET authorization_mode='policy',allowed_tags=NULL,updated_at=strftime('%s','now') WHERE id=? AND authorization_mode='legacy';\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                    &raw mut stmt,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                if rc == SQLITE_OK {
                    sqlite3_bind_int64(
                        stmt,
                        1 as ::core::ffi::c_int,
                        (*principals.offset(i as isize)).user_id as sqlite3_int64,
                    );
                    rc = sqlite3_step(stmt);
                }
                if !stmt.is_null() {
                    sqlite3_finalize(stmt);
                }
                if rc != SQLITE_DONE || sqlite3_changes(db) != 1 as ::core::ffi::c_int {
                    success = r#false != 0;
                }
                i += 1;
            }
        }
    }
    free(principals as *mut ::core::ffi::c_void);
    let mut committed_0: bool = finish_policy_change(db, success);
    pthread_mutex_unlock(mutex);
    if !committed_0 {
        return -1 as ::core::ffi::c_int;
    }
    if !migrated_count.is_null() {
        *migrated_count = count;
    }
    if count > 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT,
            _log_stream_name,
            b"Migrated %d legacy authorization principal(s) to policy grants\0".as_ptr()
                as *const ::core::ffi::c_char,
            count,
        );
    }
    return 0 as ::core::ffi::c_int;
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
