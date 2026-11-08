use ::libc;
unsafe extern "C" {
    pub type fleet_selector;
    pub type sqlite3;
    pub type sqlite3_stmt;
    unsafe fn http_request_get_query_param(
        req: *const http_request_t,
        name: *const ::core::ffi::c_char,
        value: *mut ::core::ffi::c_char,
        value_len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn http_request_extract_path_param(
        req: *const http_request_t,
        prefix: *const ::core::ffi::c_char,
        param_buf: *mut ::core::ffi::c_char,
        buf_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_set_json(
        res: *mut http_response_t,
        status_code: ::core::ffi::c_int,
        json_str: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_set_json_error(
        res: *mut http_response_t,
        status_code: ::core::ffi::c_int,
        error_message: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_ParseWithLength(
        value: *const ::core::ffi::c_char,
        buffer_length: size_t,
    ) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNull(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateNull() -> *mut cJSON;
    unsafe fn cJSON_CreateArray() -> *mut cJSON;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_AddItemToObject(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        item: *mut cJSON,
    ) -> cJSON_bool;
    unsafe fn cJSON_AddNullToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddBoolToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        boolean: cJSON_bool,
    ) -> *mut cJSON;
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
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
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
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_bind_int64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
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
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn fleet_selector_parse(
        json: *const cJSON,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> *mut fleet_selector_t;
    unsafe fn fleet_selector_free(selector: *mut fleet_selector_t);
    unsafe fn fleet_selector_matches(
        selector: *const fleet_selector_t,
        camera: *const fleet_camera_t,
        explanation: *mut fleet_selector_explanation_t,
    ) -> bool;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn storage_migration_worker_wake();
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fleet_camera_enrich_runtime_health(
        cameras: *mut fleet_camera_t,
        count: ::core::ffi::c_int,
    );
    unsafe fn db_storage_policy_validate(
        policy: *mut storage_policy_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> db_storage_policy_result_t;
    unsafe fn db_storage_policy_count() -> ::core::ffi::c_int;
    unsafe fn db_storage_policy_list(
        policies: *mut storage_policy_t,
        max_count: ::core::ffi::c_int,
        enabled_only: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn db_storage_policy_get(
        uuid: *const ::core::ffi::c_char,
        policy: *mut storage_policy_t,
    ) -> db_storage_policy_result_t;
    unsafe fn db_storage_policy_create(policy: *mut storage_policy_t)
    -> db_storage_policy_result_t;
    unsafe fn db_storage_policy_update(
        policy: *mut storage_policy_t,
        expected_revision: int64_t,
    ) -> db_storage_policy_result_t;
    unsafe fn db_storage_policy_delete(
        uuid: *const ::core::ffi::c_char,
        expected_revision: int64_t,
    ) -> db_storage_policy_result_t;
    unsafe fn db_storage_pool_get(
        uuid: *const ::core::ffi::c_char,
        pool: *mut storage_pool_t,
    ) -> db_storage_pool_result_t;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn audit_log_operation(
        req: *const http_request_t,
        user: *const user_t,
        action: *const ::core::ffi::c_char,
        target_type: *const ::core::ffi::c_char,
        target_uuid: *const ::core::ffi::c_char,
        operation: *const ::core::ffi::c_char,
        outcome: *const ::core::ffi::c_char,
        context: *const cJSON,
    );
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_authorize_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        user: *mut user_t,
        evaluation: *mut authorization_evaluation_t,
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
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
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct http_method_t(pub ::core::ffi::c_uint);
impl http_method_t {
    pub const HTTP_METHOD_GET: Self = Self(0);
    pub const HTTP_METHOD_POST: Self = Self(1);
    pub const HTTP_METHOD_PUT: Self = Self(2);
    pub const HTTP_METHOD_DELETE: Self = Self(3);
    pub const HTTP_METHOD_OPTIONS: Self = Self(4);
    pub const HTTP_METHOD_HEAD: Self = Self(5);
    pub const HTTP_METHOD_PATCH: Self = Self(6);
    pub const HTTP_METHOD_UNKNOWN: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_header_t {
    pub name: [::core::ffi::c_char; 128],
    pub value: [::core::ffi::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_request_t {
    pub method: http_method_t,
    pub method_str: [::core::ffi::c_char; 16],
    pub path: [::core::ffi::c_char; 512],
    pub uri: [::core::ffi::c_char; 1536],
    pub query_string: [::core::ffi::c_char; 1024],
    pub content_type: [::core::ffi::c_char; 128],
    pub content_length: uint64_t,
    pub user_agent: [::core::ffi::c_char; 256],
    pub body: *mut ::core::ffi::c_void,
    pub body_len: size_t,
    pub headers: [http_header_t; 50],
    pub num_headers: ::core::ffi::c_int,
    pub client_ip: [::core::ffi::c_char; 64],
    pub request_id: [::core::ffi::c_char; 65],
    pub user_data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_response_t {
    pub status_code: ::core::ffi::c_int,
    pub content_type: [::core::ffi::c_char; 128],
    pub body_length: size_t,
    pub body: *mut ::core::ffi::c_void,
    pub body_allocated: bool,
    pub headers: [http_header_t; 50],
    pub num_headers: ::core::ffi::c_int,
    pub user_data: *mut ::core::ffi::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_policy_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub enabled: bool,
    pub priority: ::core::ffi::c_int,
    pub selector_json: [::core::ffi::c_char; 8192],
    pub primary_target_uuid: [::core::ffi::c_char; 37],
    pub primary_pool_uuid: [::core::ffi::c_char; 37],
    pub fallback_mode: [::core::ffi::c_char; 16],
    pub fallback_target_uuid: [::core::ffi::c_char; 37],
    pub minimum_retention_days: ::core::ffi::c_int,
    pub desired_retention_days: ::core::ffi::c_int,
    pub maximum_retention_days: ::core::ffi::c_int,
    pub required_copy_count: ::core::ffi::c_int,
    pub replication_pool_uuid: [::core::ffi::c_char; 37],
    pub migration_after_days: ::core::ffi::c_int,
    pub migration_target_uuid: [::core::ffi::c_char; 37],
    pub pressure_priority: ::core::ffi::c_int,
    pub archive_after_seconds: ::core::ffi::c_int,
    pub hot_residency_seconds: ::core::ffi::c_int,
    pub archive_protected: bool,
    pub archive_on_pressure: bool,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_storage_policy_result_t(pub ::core::ffi::c_int);
impl db_storage_policy_result_t {
    pub const DB_STORAGE_POLICY_OK: Self = Self(0);
    pub const DB_STORAGE_POLICY_NOT_FOUND: Self = Self(-1);
    pub const DB_STORAGE_POLICY_CONFLICT: Self = Self(-2);
    pub const DB_STORAGE_POLICY_INVALID: Self = Self(-3);
    pub const DB_STORAGE_POLICY_STALE: Self = Self(-4);
    pub const DB_STORAGE_POLICY_LIMIT: Self = Self(-5);
    pub const DB_STORAGE_POLICY_ERROR: Self = Self(-6);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_t {
    pub id: int64_t,
    pub username: [::core::ffi::c_char; 64],
    pub email: [::core::ffi::c_char; 128],
    pub role: user_role_t,
    pub api_key: [::core::ffi::c_char; 64],
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub last_login: int64_t,
    pub is_active: bool,
    pub password_change_locked: bool,
    pub must_change_password: bool,
    pub totp_enabled: bool,
    pub allowed_login_cidrs: [::core::ffi::c_char; 1024],
    pub has_login_cidr_restriction: bool,
    pub authorization_mode: [::core::ffi::c_char; 16],
    pub authenticated_via_scoped_token: bool,
    pub api_token_uuid: [::core::ffi::c_char; 37],
    pub authentication_method: [::core::ffi::c_char; 24],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct user_role_t(pub ::core::ffi::c_uint);
impl user_role_t {
    pub const USER_ROLE_ADMIN: Self = Self(0);
    pub const USER_ROLE_USER: Self = Self(1);
    pub const USER_ROLE_VIEWER: Self = Self(2);
    pub const USER_ROLE_API: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct authorization_evaluation_t {
    pub decision: authorization_decision_t,
    pub source: authorization_decision_source_t,
    pub policy_version: int64_t,
    pub grant_uuid: [::core::ffi::c_char; 37],
    pub token_uuid: [::core::ffi::c_char; 37],
    pub role_uuid: [::core::ffi::c_char; 37],
    pub role_name: [::core::ffi::c_char; 128],
    pub explanation: [::core::ffi::c_char; 256],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct authorization_decision_source_t(pub ::core::ffi::c_uint);
impl authorization_decision_source_t {
    pub const AUTHZ_SOURCE_NONE: Self = Self(0);
    pub const AUTHZ_SOURCE_LEGACY_ROLE: Self = Self(1);
    pub const AUTHZ_SOURCE_POLICY_GRANT: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct authorization_decision_t(pub ::core::ffi::c_uint);
impl authorization_decision_t {
    pub const AUTHZ_DECISION_DENY: Self = Self(0);
    pub const AUTHZ_DECISION_ALLOW: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_camera_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 256],
    pub address: [::core::ffi::c_char; 512],
    pub legacy_tags: [::core::ffi::c_char; 256],
    pub location_uuid: [::core::ffi::c_char; 37],
    pub location_name: [::core::ffi::c_char; 128],
    pub location_path: [::core::ffi::c_char; 1024],
    pub location_ancestor_uuids: [[::core::ffi::c_char; 37]; 32],
    pub location_depth: ::core::ffi::c_int,
    pub tags: [fleet_camera_tag_t; 64],
    pub tag_count: ::core::ffi::c_int,
    pub manufacturer: [::core::ffi::c_char; 128],
    pub model: [::core::ffi::c_char; 128],
    pub enabled: bool,
    pub streaming_enabled: bool,
    pub record: bool,
    pub detection_based_recording: bool,
    pub is_onvif: bool,
    pub ptz_enabled: bool,
    pub backchannel_enabled: bool,
    pub health: fleet_health_state_t,
    pub availability: fleet_availability_state_t,
    pub health_changed_at: int64_t,
    pub first_video_at: int64_t,
    pub last_frame_ts: int64_t,
    pub last_recording_at: int64_t,
    pub current_fps: ::core::ffi::c_double,
    pub recording_active: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct fleet_availability_state_t(pub ::core::ffi::c_uint);
impl fleet_availability_state_t {
    pub const FLEET_AVAILABILITY_LIVE: Self = Self(0);
    pub const FLEET_AVAILABILITY_OFFLINE: Self = Self(1);
    pub const FLEET_AVAILABILITY_NEVER_CONNECTED: Self = Self(2);
    pub const FLEET_AVAILABILITY_DISABLED: Self = Self(3);
    pub const FLEET_AVAILABILITY_COUNT: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct fleet_health_state_t(pub ::core::ffi::c_uint);
impl fleet_health_state_t {
    pub const FLEET_HEALTH_UNKNOWN: Self = Self(0);
    pub const FLEET_HEALTH_UP: Self = Self(1);
    pub const FLEET_HEALTH_DEGRADED: Self = Self(2);
    pub const FLEET_HEALTH_DOWN: Self = Self(3);
    pub const FLEET_HEALTH_DISABLED: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_camera_tag_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub label: [::core::ffi::c_char; 256],
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
pub struct storage_pool_member_t {
    pub target_uuid: [::core::ffi::c_char; 37],
    pub position: ::core::ffi::c_int,
    pub weight: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_pool_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub strategy: [::core::ffi::c_char; 24],
    pub enabled: bool,
    pub allocation_cursor: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub member_count: ::core::ffi::c_int,
    pub members: [storage_pool_member_t; 128],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_storage_pool_result_t(pub ::core::ffi::c_int);
impl db_storage_pool_result_t {
    pub const DB_STORAGE_POOL_OK: Self = Self(0);
    pub const DB_STORAGE_POOL_NOT_FOUND: Self = Self(-1);
    pub const DB_STORAGE_POOL_CONFLICT: Self = Self(-2);
    pub const DB_STORAGE_POOL_INVALID: Self = Self(-3);
    pub const DB_STORAGE_POOL_STALE: Self = Self(-4);
    pub const DB_STORAGE_POOL_LIMIT: Self = Self(-5);
    pub const DB_STORAGE_POOL_IN_USE: Self = Self(-6);
    pub const DB_STORAGE_POOL_UNAVAILABLE: Self = Self(-7);
    pub const DB_STORAGE_POOL_ERROR: Self = Self(-8);
}
pub type fleet_selector_t = fleet_selector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_selector_explanation_t {
    pub clauses: [[::core::ffi::c_char; 160]; 64],
    pub clause_count: ::core::ffi::c_int,
}
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const STORAGE_TARGET_ERROR_MAX: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const LIGHTNVR_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const STORAGE_POLICY_MAX_COUNT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
unsafe extern "C" fn authorize_storage(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut user: *mut user_t,
) -> bool {
    let mut evaluation: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t::AUTHZ_DECISION_DENY,
        source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
        policy_version: 0,
        grant_uuid: [0; 37],
        token_uuid: [0; 37],
        role_uuid: [0; 37],
        role_name: [0; 128],
        explanation: [0; 256],
    };
    return httpd_authorize_action(
        req,
        res,
        authorization_action_t::AUTHZ_STORAGE_CONFIGURE,
        ::core::ptr::null::<fleet_camera_t>(),
        user,
        &raw mut evaluation,
    ) != 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn send_json(
    mut res: *mut http_response_t,
    mut status: ::core::ffi::c_int,
    mut object: *mut cJSON,
) -> bool {
    let mut body: *mut ::core::ffi::c_char = if !object.is_null() {
        cJSON_PrintUnformatted(object)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    cJSON_Delete(object);
    if body.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize storage policy response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    http_response_set_json(res, status, body);
    free(body as *mut ::core::ffi::c_void);
    return r#true != 0;
}
unsafe extern "C" fn policy_to_json(mut policy: *const storage_policy_t) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_CreateObject();
    let mut selector: *mut cJSON =
        cJSON_Parse(&raw const (*policy).selector_json as *const ::core::ffi::c_char);
    if object.is_null() || selector.is_null() {
        cJSON_Delete(object);
        cJSON_Delete(selector);
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*policy).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*policy).name as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        object,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).enabled as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        object,
        b"priority\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).priority as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        object,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
        selector,
    );
    cJSON_AddStringToObject(
        object,
        b"primary_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*policy).primary_target_uuid as *const ::core::ffi::c_char,
    );
    if (*policy).primary_pool_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            object,
            b"primary_pool_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*policy).primary_pool_uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            object,
            b"primary_pool_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddStringToObject(
        object,
        b"fallback_mode\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*policy).fallback_mode as *const ::core::ffi::c_char,
    );
    if (*policy).fallback_target_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            object,
            b"fallback_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*policy).fallback_target_uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            object,
            b"fallback_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddNumberToObject(
        object,
        b"minimum_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).minimum_retention_days as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"desired_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).desired_retention_days as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"maximum_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).maximum_retention_days as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"required_copy_count\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).required_copy_count as ::core::ffi::c_double,
    );
    if (*policy).replication_pool_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            object,
            b"replication_pool_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*policy).replication_pool_uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            object,
            b"replication_pool_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddNumberToObject(
        object,
        b"migration_after_days\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).migration_after_days as ::core::ffi::c_double,
    );
    if (*policy).migration_target_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            object,
            b"migration_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*policy).migration_target_uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            object,
            b"migration_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddNumberToObject(
        object,
        b"pressure_priority\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).pressure_priority as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"archive_after_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).archive_after_seconds as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"hot_residency_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).hot_residency_seconds as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        object,
        b"archive_protected\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).archive_protected as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        object,
        b"archive_on_pressure\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).archive_on_pressure as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        object,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).revision as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).updated_at as ::core::ffi::c_double,
    );
    return object;
}
unsafe extern "C" fn set_db_error(
    mut res: *mut http_response_t,
    mut result: db_storage_policy_result_t,
    mut validation_error: *const ::core::ffi::c_char,
) {
    match result {
        db_storage_policy_result_t::DB_STORAGE_POLICY_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Storage policy not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_storage_policy_result_t::DB_STORAGE_POLICY_CONFLICT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"A storage policy already uses that name\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_storage_policy_result_t::DB_STORAGE_POLICY_STALE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Storage policy was changed by another administrator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_storage_policy_result_t::DB_STORAGE_POLICY_LIMIT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Storage policy limit reached\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_storage_policy_result_t::DB_STORAGE_POLICY_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                if !validation_error.is_null()
                    && *validation_error.offset(0isize) as ::core::ffi::c_int != 0
                {
                    validation_error
                } else {
                    b"Invalid storage policy\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Storage policy operation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn extract_uuid(
    mut req: *const http_request_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut res: *mut http_response_t,
) -> bool {
    let mut value: [::core::ffi::c_char; 512] = [0; 512];
    if http_request_extract_path_param(
        req,
        b"/api/storage-policies/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid storage policy path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut slash: *mut ::core::ffi::c_char = strchr(
        &raw mut value as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if !slash.is_null() {
        *slash = '\0' as ::core::ffi::c_char;
    }
    if !lightnvr_uuid_is_valid(&raw mut value as *mut ::core::ffi::c_char) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid storage policy UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        uuid,
        &raw mut value as *mut ::core::ffi::c_char,
        LIGHTNVR_UUID_STRING_SIZE as size_t,
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn string_value(
    mut body: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut destination: *mut ::core::ffi::c_char,
    mut destination_size: size_t,
    mut required: bool,
    mut res: *mut http_response_t,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, key);
    if item.is_null() {
        if !required {
            return r#true != 0;
        }
        let mut message: [::core::ffi::c_char; 160] = [0; 160];
        snprintf(
            &raw mut message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
            b"%s is required\0".as_ptr() as *const ::core::ffi::c_char,
            key,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            &raw mut message as *mut ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if cJSON_IsString(item) == 0
        || (*item).valuestring.is_null()
        || strnlen((*item).valuestring, destination_size) >= destination_size
    {
        let mut message_0: [::core::ffi::c_char; 160] = [0; 160];
        snprintf(
            &raw mut message_0 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
            b"%s must be a valid string\0".as_ptr() as *const ::core::ffi::c_char,
            key,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            &raw mut message_0 as *mut ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        destination,
        (*item).valuestring,
        destination_size,
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn nullable_string_value(
    mut body: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut destination: *mut ::core::ffi::c_char,
    mut destination_size: size_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, key);
    if item.is_null() {
        return r#true != 0;
    }
    if cJSON_IsNull(item) != 0 {
        *destination.offset(0isize) = '\0' as ::core::ffi::c_char;
        return r#true != 0;
    }
    if cJSON_IsString(item) == 0
        || (*item).valuestring.is_null()
        || strnlen((*item).valuestring, destination_size) >= destination_size
    {
        let mut message: [::core::ffi::c_char; 160] = [0; 160];
        snprintf(
            &raw mut message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
            b"%s must be a UUID or null\0".as_ptr() as *const ::core::ffi::c_char,
            key,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            &raw mut message as *mut ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        destination,
        (*item).valuestring,
        destination_size,
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn integer_value(
    mut body: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut minimum: ::core::ffi::c_int,
    mut maximum: ::core::ffi::c_int,
    mut destination: *mut ::core::ffi::c_int,
    mut res: *mut http_response_t,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, key);
    if item.is_null() {
        return r#true != 0;
    }
    if cJSON_IsNumber(item) == 0
        || (*item).valuedouble.is_finite() as ::core::ffi::c_int == 0
        || floor((*item).valuedouble) != (*item).valuedouble
        || (*item).valuedouble < minimum as ::core::ffi::c_double
        || (*item).valuedouble > maximum as ::core::ffi::c_double
    {
        let mut message: [::core::ffi::c_char; 192] = [0; 192];
        snprintf(
            &raw mut message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 192]>(),
            b"%s must be an integer from %d to %d\0".as_ptr() as *const ::core::ffi::c_char,
            key,
            minimum,
            maximum,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            &raw mut message as *mut ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *destination = (*item).valueint;
    return r#true != 0;
}
unsafe extern "C" fn apply_body(
    mut body: *const cJSON,
    mut policy: *mut storage_policy_t,
    mut create: bool,
    mut revision: *mut int64_t,
    mut validation_error: *mut ::core::ffi::c_char,
    mut res: *mut http_response_t,
) -> bool {
    if cJSON_IsObject(body) == 0 {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Request body must be a JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !string_value(
        body,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*policy).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        create,
        res,
    ) || !string_value(
        body,
        b"primary_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*policy).primary_target_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        r#false != 0,
        res,
    ) || !string_value(
        body,
        b"fallback_mode\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*policy).fallback_mode as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        r#false != 0,
        res,
    ) || !nullable_string_value(
        body,
        b"primary_pool_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*policy).primary_pool_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) || !nullable_string_value(
        body,
        b"replication_pool_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*policy).replication_pool_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) || !nullable_string_value(
        body,
        b"migration_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*policy).migration_target_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) || !integer_value(
        body,
        b"minimum_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        36500 as ::core::ffi::c_int,
        &raw mut (*policy).minimum_retention_days,
        res,
    ) || !integer_value(
        body,
        b"desired_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        36500 as ::core::ffi::c_int,
        &raw mut (*policy).desired_retention_days,
        res,
    ) || !integer_value(
        body,
        b"maximum_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        36500 as ::core::ffi::c_int,
        &raw mut (*policy).maximum_retention_days,
        res,
    ) || !integer_value(
        body,
        b"required_copy_count\0".as_ptr() as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        &raw mut (*policy).required_copy_count,
        res,
    ) || !integer_value(
        body,
        b"migration_after_days\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        36500 as ::core::ffi::c_int,
        &raw mut (*policy).migration_after_days,
        res,
    ) || !integer_value(
        body,
        b"archive_after_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        2147483647 as ::core::ffi::c_int,
        &raw mut (*policy).archive_after_seconds,
        res,
    ) || !integer_value(
        body,
        b"hot_residency_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        2147483647 as ::core::ffi::c_int,
        &raw mut (*policy).hot_residency_seconds,
        res,
    ) || !integer_value(
        body,
        b"pressure_priority\0".as_ptr() as *const ::core::ffi::c_char,
        -1000000 as ::core::ffi::c_int,
        1000000 as ::core::ffi::c_int,
        &raw mut (*policy).pressure_priority,
        res,
    ) {
        return r#false != 0;
    }
    if create as ::core::ffi::c_int != 0
        && (*policy).primary_target_uuid[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        && (*policy).primary_pool_uuid[0usize] as ::core::ffi::c_int != 0
    {
        let mut pool: storage_pool_t = storage_pool_t {
            uuid: [0; 37],
            name: [0; 128],
            strategy: [0; 24],
            enabled: false,
            allocation_cursor: 0,
            revision: 0,
            created_at: 0,
            updated_at: 0,
            member_count: 0,
            members: [storage_pool_member_t {
                target_uuid: [0; 37],
                position: 0,
                weight: 0,
            }; 128],
        };
        if db_storage_pool_get(
            &raw mut (*policy).primary_pool_uuid as *mut ::core::ffi::c_char,
            &raw mut pool,
        )
        .0 == db_storage_pool_result_t::DB_STORAGE_POOL_OK.0
            && pool.member_count > 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*policy).primary_target_uuid as *mut ::core::ffi::c_char,
                &raw mut (*(&raw mut pool.members as *mut storage_pool_member_t).offset(0isize))
                    .target_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
        }
    }
    if create as ::core::ffi::c_int != 0
        && (*policy).primary_target_uuid[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"primary_target_uuid or primary_pool_uuid is required\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut fallback: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"fallback_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !fallback.is_null() {
        if cJSON_IsNull(fallback) != 0 {
            (*policy).fallback_target_uuid[0usize] = '\0' as ::core::ffi::c_char;
        } else if cJSON_IsString(fallback) == 0
            || (*fallback).valuestring.is_null()
            || strnlen(
                (*fallback).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            ) >= ::core::mem::size_of::<[::core::ffi::c_char; 37]>()
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"fallback_target_uuid must be a UUID or null\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        } else {
            safe_strcpy(
                &raw mut (*policy).fallback_target_uuid as *mut ::core::ffi::c_char,
                (*fallback).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
        }
    }
    let mut enabled: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"enabled\0".as_ptr() as *const ::core::ffi::c_char);
    let mut archive_flags: [*const ::core::ffi::c_char; 2] = [
        b"archive_protected\0".as_ptr() as *const ::core::ffi::c_char,
        b"archive_on_pressure\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut archive_values: [*mut bool; 2] = [
        &raw mut (*policy).archive_protected,
        &raw mut (*policy).archive_on_pressure,
    ];
    let mut i: size_t = 0 as size_t;
    while i < 2 as size_t {
        let mut flag: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, archive_flags[i]);
        if !flag.is_null() {
            if cJSON_IsBool(flag) == 0 {
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"Archive flags must be boolean\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return r#false != 0;
            }
            *archive_values[i] = cJSON_IsTrue(flag) != 0;
        }
        i = i.wrapping_add(1);
    }
    if !enabled.is_null() {
        if cJSON_IsBool(enabled) == 0 {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"enabled must be boolean\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*policy).enabled = cJSON_IsTrue(enabled) != 0;
    }
    let mut priority: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"priority\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !priority.is_null() {
        if cJSON_IsNumber(priority) == 0
            || (*priority).valuedouble.is_finite() as ::core::ffi::c_int == 0
            || floor((*priority).valuedouble) != (*priority).valuedouble
            || (*priority).valuedouble < -1000000 as ::core::ffi::c_int as ::core::ffi::c_double
            || (*priority).valuedouble > 1000000 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"priority must be an integer\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*policy).priority = (*priority).valueint;
    }
    let mut selector: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !selector.is_null() {
        let mut text: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(selector);
        if text.is_null()
            || strnlen(text, ::core::mem::size_of::<[::core::ffi::c_char; 8192]>())
                >= ::core::mem::size_of::<[::core::ffi::c_char; 8192]>()
        {
            free(text as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"selector is too large\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*policy).selector_json as *mut ::core::ffi::c_char,
            text,
            ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
            0 as size_t,
        );
        free(text as *mut ::core::ffi::c_void);
    } else if create {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"selector is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !create {
        let mut revision_item: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            body,
            b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if cJSON_IsNumber(revision_item) == 0
            || (*revision_item).valuedouble < 1 as ::core::ffi::c_int as ::core::ffi::c_double
            || floor((*revision_item).valuedouble) != (*revision_item).valuedouble
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"revision must be a positive integer\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        *revision = (*revision_item).valuedouble as int64_t;
    }
    let mut result: db_storage_policy_result_t =
        db_storage_policy_validate(policy, validation_error, STORAGE_TARGET_ERROR_MAX as size_t);
    if result.0 != db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0 {
        set_db_error(res, result, validation_error);
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn audit_policy(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut uuid: *const ::core::ffi::c_char,
    mut operation: *const ::core::ffi::c_char,
    mut outcome: *const ::core::ffi::c_char,
) {
    audit_log_operation(
        req,
        user,
        b"storage.configure\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage_policy\0".as_ptr() as *const ::core::ffi::c_char,
        uuid,
        operation,
        outcome,
        ::core::ptr::null::<cJSON>(),
    );
}
unsafe extern "C" fn policy_precedes(
    mut left: *const storage_policy_t,
    mut right: *const storage_policy_t,
) -> bool {
    if right.is_null() {
        return r#true != 0;
    }
    if (*left).priority != (*right).priority {
        return (*left).priority > (*right).priority;
    }
    let mut name_order: ::core::ffi::c_int = strcasecmp(
        &raw const (*left).name as *const ::core::ffi::c_char,
        &raw const (*right).name as *const ::core::ffi::c_char,
    );
    if name_order != 0 as ::core::ffi::c_int {
        return name_order < 0 as ::core::ffi::c_int;
    }
    return strcmp(
        &raw const (*left).uuid as *const ::core::ffi::c_char,
        &raw const (*right).uuid as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn preview_winner_json(
    mut policy: *const storage_policy_t,
    mut draft: *const storage_policy_t,
) -> *mut cJSON {
    if policy.is_null() {
        return cJSON_CreateNull();
    }
    let mut winner: *mut cJSON = cJSON_CreateObject();
    if winner.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut is_draft: bool = policy == draft;
    cJSON_AddBoolToObject(
        winner,
        b"draft\0".as_ptr() as *const ::core::ffi::c_char,
        is_draft as cJSON_bool,
    );
    if (*policy).uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            winner,
            b"policy_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*policy).uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            winner,
            b"policy_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddStringToObject(
        winner,
        b"policy_name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*policy).name as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        winner,
        b"priority\0".as_ptr() as *const ::core::ffi::c_char,
        (*policy).priority as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        winner,
        b"primary_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*policy).primary_target_uuid as *const ::core::ffi::c_char,
    );
    if (*policy).primary_pool_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            winner,
            b"primary_pool_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*policy).primary_pool_uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            winner,
            b"primary_pool_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddStringToObject(
        winner,
        b"fallback_mode\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*policy).fallback_mode as *const ::core::ffi::c_char,
    );
    if (*policy).fallback_target_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            winner,
            b"fallback_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*policy).fallback_target_uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            winner,
            b"fallback_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return winner;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_storage_policy_preview(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut user: user_t = user_t {
        id: 0,
        username: [0; 64],
        email: [0; 128],
        role: user_role_t::USER_ROLE_ADMIN,
        api_key: [0; 64],
        created_at: 0,
        updated_at: 0,
        last_login: 0,
        is_active: false,
        password_change_locked: false,
        must_change_password: false,
        totp_enabled: false,
        allowed_login_cidrs: [0; 1024],
        has_login_cidr_restriction: false,
        authorization_mode: [0; 16],
        authenticated_via_scoped_token: false,
        api_token_uuid: [0; 37],
        authentication_method: [0; 24],
    };
    if !authorize_storage(req, res, &raw mut user) {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    if cJSON_IsObject(body) == 0 {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Request body must be a JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut draft: storage_policy_t = storage_policy_t {
        uuid: [0; 37],
        name: [0; 128],
        enabled: false,
        priority: 0,
        selector_json: [0; 8192],
        primary_target_uuid: [0; 37],
        primary_pool_uuid: [0; 37],
        fallback_mode: [0; 16],
        fallback_target_uuid: [0; 37],
        minimum_retention_days: 0,
        desired_retention_days: 0,
        maximum_retention_days: 0,
        required_copy_count: 0,
        replication_pool_uuid: [0; 37],
        migration_after_days: 0,
        migration_target_uuid: [0; 37],
        pressure_priority: 0,
        archive_after_seconds: 0,
        hot_residency_seconds: 0,
        archive_protected: false,
        archive_on_pressure: false,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    memset(
        &raw mut draft as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_policy_t>(),
    );
    draft.enabled = r#true != 0;
    draft.priority = 100 as ::core::ffi::c_int;
    draft.required_copy_count = 1 as ::core::ffi::c_int;
    draft.pressure_priority = 100 as ::core::ffi::c_int;
    draft.archive_after_seconds = -1 as ::core::ffi::c_int;
    draft.hot_residency_seconds = -1 as ::core::ffi::c_int;
    safe_strcpy(
        &raw mut draft.fallback_mode as *mut ::core::ffi::c_char,
        b"default\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    let mut editing: bool = r#false != 0;
    let mut uuid_item: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"uuid\0".as_ptr() as *const ::core::ffi::c_char);
    if !uuid_item.is_null() {
        if cJSON_IsString(uuid_item) == 0
            || (*uuid_item).valuestring.is_null()
            || !lightnvr_uuid_is_valid((*uuid_item).valuestring)
        {
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"uuid must be a valid policy UUID\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut loaded: db_storage_policy_result_t =
            db_storage_policy_get((*uuid_item).valuestring, &raw mut draft);
        if loaded.0 != db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0 {
            cJSON_Delete(body);
            set_db_error(res, loaded, ::core::ptr::null::<::core::ffi::c_char>());
            return;
        }
        editing = r#true != 0;
    }
    let mut revision: int64_t = 0 as int64_t;
    let mut validation_error: [::core::ffi::c_char; 256] = [
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
    if !apply_body(
        body,
        &raw mut draft,
        !editing,
        &raw mut revision,
        &raw mut validation_error as *mut ::core::ffi::c_char,
        res,
    ) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    let mut draft_selector_json: *mut cJSON =
        cJSON_Parse(&raw mut draft.selector_json as *mut ::core::ffi::c_char);
    let mut selector_error: [::core::ffi::c_char; 256] = [
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
    let mut draft_selector: *mut fleet_selector_t = fleet_selector_parse(
        draft_selector_json,
        &raw mut selector_error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    cJSON_Delete(draft_selector_json);
    if draft_selector.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            if selector_error[0usize] as ::core::ffi::c_int != 0 {
                &raw mut selector_error as *mut ::core::ffi::c_char
            } else {
                b"Invalid selector\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    let mut total: ::core::ffi::c_int = db_storage_policy_count();
    let mut policies: *mut storage_policy_t =
        (if total > 0 as ::core::ffi::c_int && total <= STORAGE_POLICY_MAX_COUNT {
            calloc(total as size_t, ::core::mem::size_of::<storage_policy_t>())
        } else {
            NULL
        }) as *mut storage_policy_t;
    let mut selectors: *mut *mut fleet_selector_t =
        (if total > 0 as ::core::ffi::c_int && total <= STORAGE_POLICY_MAX_COUNT {
            calloc(
                total as size_t,
                ::core::mem::size_of::<*mut fleet_selector_t>(),
            )
        } else {
            NULL
        }) as *mut *mut fleet_selector_t;
    let mut overlaps: *mut ::core::ffi::c_int =
        (if total > 0 as ::core::ffi::c_int && total <= STORAGE_POLICY_MAX_COUNT {
            calloc(
                total as size_t,
                ::core::mem::size_of::<::core::ffi::c_int>(),
            )
        } else {
            NULL
        }) as *mut ::core::ffi::c_int;
    if total < 0 as ::core::ffi::c_int
        || total > STORAGE_POLICY_MAX_COUNT
        || total > 0 as ::core::ffi::c_int
            && (policies.is_null() || selectors.is_null() || overlaps.is_null())
    {
        fleet_selector_free(draft_selector);
        free(policies as *mut ::core::ffi::c_void);
        free(selectors as *mut ::core::ffi::c_void);
        free(overlaps as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load storage policies\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut policy_count: ::core::ffi::c_int = if total > 0 as ::core::ffi::c_int {
        db_storage_policy_list(policies, total, r#true != 0)
    } else {
        0 as ::core::ffi::c_int
    };
    if policy_count < 0 as ::core::ffi::c_int {
        fleet_selector_free(draft_selector);
        free(policies as *mut ::core::ffi::c_void);
        free(selectors as *mut ::core::ffi::c_void);
        free(overlaps as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load storage policies\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < policy_count {
        if !(editing as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*policies.offset(index as isize)).uuid as *mut ::core::ffi::c_char,
                &raw mut draft.uuid as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)
        {
            let mut json: *mut cJSON = cJSON_Parse(
                &raw mut (*policies.offset(index as isize)).selector_json
                    as *mut ::core::ffi::c_char,
            );
            *selectors.offset(index as isize) = if !json.is_null() {
                fleet_selector_parse(
                    json,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    0 as size_t,
                )
            } else {
                ::core::ptr::null_mut::<fleet_selector_t>()
            };
            cJSON_Delete(json);
            if (*selectors.offset(index as isize)).is_null() {
                let mut cleanup: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while cleanup < policy_count {
                    fleet_selector_free(*selectors.offset(cleanup as isize));
                    cleanup += 1;
                }
                fleet_selector_free(draft_selector);
                free(policies as *mut ::core::ffi::c_void);
                free(selectors as *mut ::core::ffi::c_void);
                free(overlaps as *mut ::core::ffi::c_void);
                http_response_set_json_error(
                    res,
                    500 as ::core::ffi::c_int,
                    b"Failed to compile existing storage policies\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return;
            }
        }
        index += 1;
    }
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if db_fleet_camera_load(&raw mut cameras, &raw mut camera_count) != 0 as ::core::ffi::c_int {
        let mut index_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while index_0 < policy_count {
            fleet_selector_free(*selectors.offset(index_0 as isize));
            index_0 += 1;
        }
        fleet_selector_free(draft_selector);
        free(policies as *mut ::core::ffi::c_void);
        free(selectors as *mut ::core::ffi::c_void);
        free(overlaps as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load camera inventory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    fleet_camera_enrich_runtime_health(cameras, camera_count);
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut samples: *mut cJSON = cJSON_CreateArray();
    let mut conflicts: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || samples.is_null() || conflicts.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(samples);
        cJSON_Delete(conflicts);
        root = ::core::ptr::null_mut::<cJSON>();
    } else {
        cJSON_AddItemToObject(
            root,
            b"sample\0".as_ptr() as *const ::core::ffi::c_char,
            samples,
        );
        cJSON_AddItemToObject(
            root,
            b"conflicts\0".as_ptr() as *const ::core::ffi::c_char,
            conflicts,
        );
    }
    let mut matched: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut effective: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut conflict_cameras: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut camera_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !root.is_null() && camera_index < camera_count {
        if fleet_selector_matches(
            draft_selector,
            cameras.offset(camera_index as isize),
            ::core::ptr::null_mut::<fleet_selector_explanation_t>(),
        ) {
            matched += 1;
            let mut winner: *const storage_policy_t = if draft.enabled as ::core::ffi::c_int != 0 {
                &raw mut draft
            } else {
                ::core::ptr::null_mut::<storage_policy_t>()
            };
            let mut camera_overlaps: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut policy_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while policy_index < policy_count {
                if !((*selectors.offset(policy_index as isize)).is_null()
                    || !fleet_selector_matches(
                        *selectors.offset(policy_index as isize),
                        cameras.offset(camera_index as isize),
                        ::core::ptr::null_mut::<fleet_selector_explanation_t>(),
                    ))
                {
                    if draft.enabled {
                        *overlaps.offset(policy_index as isize) += 1;
                        camera_overlaps += 1;
                    }
                    if policy_precedes(policies.offset(policy_index as isize), winner) {
                        winner = policies.offset(policy_index as isize);
                    }
                }
                policy_index += 1;
            }
            if camera_overlaps > 0 as ::core::ffi::c_int {
                conflict_cameras += 1;
            }
            if winner == &raw mut draft {
                effective += 1;
            }
            if cJSON_GetArraySize(samples) < 50 as ::core::ffi::c_int {
                let mut sample: *mut cJSON = cJSON_CreateObject();
                let mut winner_json: *mut cJSON = preview_winner_json(winner, &raw mut draft);
                if sample.is_null() || winner_json.is_null() {
                    cJSON_Delete(sample);
                    cJSON_Delete(winner_json);
                    cJSON_Delete(root);
                    root = ::core::ptr::null_mut::<cJSON>();
                    break;
                } else {
                    cJSON_AddStringToObject(
                        sample,
                        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*cameras.offset(camera_index as isize)).camera_uuid
                            as *mut ::core::ffi::c_char,
                    );
                    cJSON_AddStringToObject(
                        sample,
                        b"camera_name\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*cameras.offset(camera_index as isize)).name
                            as *mut ::core::ffi::c_char,
                    );
                    cJSON_AddNumberToObject(
                        sample,
                        b"overlapping_policy_count\0".as_ptr() as *const ::core::ffi::c_char,
                        camera_overlaps as ::core::ffi::c_double,
                    );
                    cJSON_AddItemToObject(
                        sample,
                        b"effective_policy\0".as_ptr() as *const ::core::ffi::c_char,
                        winner_json,
                    );
                    cJSON_AddItemToArray(samples, sample);
                }
            }
        }
        camera_index += 1;
    }
    let mut conflict_policy_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut index_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !root.is_null() && index_1 < policy_count {
        if *overlaps.offset(index_1 as isize) > 0 as ::core::ffi::c_int {
            let mut conflict: *mut cJSON = cJSON_CreateObject();
            if conflict.is_null() {
                cJSON_Delete(root);
                root = ::core::ptr::null_mut::<cJSON>();
                break;
            } else {
                cJSON_AddStringToObject(
                    conflict,
                    b"policy_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*policies.offset(index_1 as isize)).uuid as *mut ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    conflict,
                    b"policy_name\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*policies.offset(index_1 as isize)).name as *mut ::core::ffi::c_char,
                );
                cJSON_AddNumberToObject(
                    conflict,
                    b"priority\0".as_ptr() as *const ::core::ffi::c_char,
                    (*policies.offset(index_1 as isize)).priority as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    conflict,
                    b"overlap_camera_count\0".as_ptr() as *const ::core::ffi::c_char,
                    *overlaps.offset(index_1 as isize) as ::core::ffi::c_double,
                );
                cJSON_AddBoolToObject(
                    conflict,
                    b"draft_precedes\0".as_ptr() as *const ::core::ffi::c_char,
                    policy_precedes(&raw mut draft, policies.offset(index_1 as isize))
                        as cJSON_bool,
                );
                cJSON_AddItemToArray(conflicts, conflict);
                conflict_policy_count += 1;
            }
        }
        index_1 += 1;
    }
    if !root.is_null() {
        cJSON_AddBoolToObject(
            root,
            b"draft_enabled\0".as_ptr() as *const ::core::ffi::c_char,
            draft.enabled as cJSON_bool,
        );
        cJSON_AddNumberToObject(
            root,
            b"matched_camera_count\0".as_ptr() as *const ::core::ffi::c_char,
            matched as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            root,
            b"effective_camera_count\0".as_ptr() as *const ::core::ffi::c_char,
            effective as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            root,
            b"shadowed_camera_count\0".as_ptr() as *const ::core::ffi::c_char,
            (if draft.enabled as ::core::ffi::c_int != 0 {
                matched - effective
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            root,
            b"conflict_camera_count\0".as_ptr() as *const ::core::ffi::c_char,
            conflict_cameras as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            root,
            b"conflict_policy_count\0".as_ptr() as *const ::core::ffi::c_char,
            conflict_policy_count as ::core::ffi::c_double,
        );
        cJSON_AddBoolToObject(
            root,
            b"sample_truncated\0".as_ptr() as *const ::core::ffi::c_char,
            (matched > 50 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
    }
    free(cameras as *mut ::core::ffi::c_void);
    let mut index_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index_2 < policy_count {
        fleet_selector_free(*selectors.offset(index_2 as isize));
        index_2 += 1;
    }
    fleet_selector_free(draft_selector);
    free(policies as *mut ::core::ffi::c_void);
    free(selectors as *mut ::core::ffi::c_void);
    free(overlaps as *mut ::core::ffi::c_void);
    if root.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create policy preview\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    send_json(res, 200 as ::core::ffi::c_int, root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_storage_policies(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut user: user_t = user_t {
        id: 0,
        username: [0; 64],
        email: [0; 128],
        role: user_role_t::USER_ROLE_ADMIN,
        api_key: [0; 64],
        created_at: 0,
        updated_at: 0,
        last_login: 0,
        is_active: false,
        password_change_locked: false,
        must_change_password: false,
        totp_enabled: false,
        allowed_login_cidrs: [0; 1024],
        has_login_cidr_restriction: false,
        authorization_mode: [0; 16],
        authenticated_via_scoped_token: false,
        api_token_uuid: [0; 37],
        authentication_method: [0; 24],
    };
    if !authorize_storage(req, res, &raw mut user) {
        return;
    }
    let mut total: ::core::ffi::c_int = db_storage_policy_count();
    if total < 0 as ::core::ffi::c_int || total > STORAGE_POLICY_MAX_COUNT {
        set_db_error(
            res,
            db_storage_policy_result_t::DB_STORAGE_POLICY_ERROR,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return;
    }
    let mut policies: *mut storage_policy_t = (if total != 0 {
        calloc(total as size_t, ::core::mem::size_of::<storage_policy_t>())
    } else {
        NULL
    }) as *mut storage_policy_t;
    if total != 0 && policies.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int = if total != 0 {
        db_storage_policy_list(policies, total, r#false != 0)
    } else {
        0 as ::core::ffi::c_int
    };
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if count < 0 as ::core::ffi::c_int || root.is_null() || items.is_null() {
        free(policies as *mut ::core::ffi::c_void);
        cJSON_Delete(root);
        cJSON_Delete(items);
        set_db_error(
            res,
            db_storage_policy_result_t::DB_STORAGE_POLICY_ERROR,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return;
    }
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        count as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"policies\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut item: *mut cJSON = policy_to_json(policies.offset(index as isize));
        if item.is_null() {
            free(policies as *mut ::core::ffi::c_void);
            cJSON_Delete(root);
            set_db_error(
                res,
                db_storage_policy_result_t::DB_STORAGE_POLICY_ERROR,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            return;
        }
        cJSON_AddItemToArray(items, item);
        index += 1;
    }
    free(policies as *mut ::core::ffi::c_void);
    send_json(res, 200 as ::core::ffi::c_int, root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_storage_policy(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut user: user_t = user_t {
        id: 0,
        username: [0; 64],
        email: [0; 128],
        role: user_role_t::USER_ROLE_ADMIN,
        api_key: [0; 64],
        created_at: 0,
        updated_at: 0,
        last_login: 0,
        is_active: false,
        password_change_locked: false,
        must_change_password: false,
        totp_enabled: false,
        allowed_login_cidrs: [0; 1024],
        has_login_cidr_restriction: false,
        authorization_mode: [0; 16],
        authenticated_via_scoped_token: false,
        api_token_uuid: [0; 37],
        authentication_method: [0; 24],
    };
    if !authorize_storage(req, res, &raw mut user) {
        return;
    }
    let mut policy: storage_policy_t = storage_policy_t {
        uuid: [0; 37],
        name: [0; 128],
        enabled: false,
        priority: 0,
        selector_json: [0; 8192],
        primary_target_uuid: [0; 37],
        primary_pool_uuid: [0; 37],
        fallback_mode: [0; 16],
        fallback_target_uuid: [0; 37],
        minimum_retention_days: 0,
        desired_retention_days: 0,
        maximum_retention_days: 0,
        required_copy_count: 0,
        replication_pool_uuid: [0; 37],
        migration_after_days: 0,
        migration_target_uuid: [0; 37],
        pressure_priority: 0,
        archive_after_seconds: 0,
        hot_residency_seconds: 0,
        archive_protected: false,
        archive_on_pressure: false,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    memset(
        &raw mut policy as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_policy_t>(),
    );
    policy.enabled = r#true != 0;
    policy.priority = 100 as ::core::ffi::c_int;
    policy.required_copy_count = 1 as ::core::ffi::c_int;
    policy.pressure_priority = 100 as ::core::ffi::c_int;
    policy.archive_after_seconds = -1 as ::core::ffi::c_int;
    policy.hot_residency_seconds = -1 as ::core::ffi::c_int;
    safe_strcpy(
        &raw mut policy.fallback_mode as *mut ::core::ffi::c_char,
        b"default\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut revision: int64_t = 0 as int64_t;
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
    if !apply_body(
        body,
        &raw mut policy,
        r#true != 0,
        &raw mut revision,
        &raw mut error as *mut ::core::ffi::c_char,
        res,
    ) {
        cJSON_Delete(body);
        audit_policy(
            req,
            &raw mut user,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"policy_create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_Delete(body);
    let mut result: db_storage_policy_result_t = db_storage_policy_create(&raw mut policy);
    if result.0 != db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0 {
        set_db_error(res, result, &raw mut error as *mut ::core::ffi::c_char);
        audit_policy(
            req,
            &raw mut user,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"policy_create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_policy(
        req,
        &raw mut user,
        &raw mut policy.uuid as *mut ::core::ffi::c_char,
        b"policy_create\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
    );
    send_json(
        res,
        201 as ::core::ffi::c_int,
        policy_to_json(&raw mut policy),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_storage_policy(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut user: user_t = user_t {
        id: 0,
        username: [0; 64],
        email: [0; 128],
        role: user_role_t::USER_ROLE_ADMIN,
        api_key: [0; 64],
        created_at: 0,
        updated_at: 0,
        last_login: 0,
        is_active: false,
        password_change_locked: false,
        must_change_password: false,
        totp_enabled: false,
        allowed_login_cidrs: [0; 1024],
        has_login_cidr_restriction: false,
        authorization_mode: [0; 16],
        authenticated_via_scoped_token: false,
        api_token_uuid: [0; 37],
        authentication_method: [0; 24],
    };
    if !authorize_storage(req, res, &raw mut user) {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res) {
        return;
    }
    let mut policy: storage_policy_t = storage_policy_t {
        uuid: [0; 37],
        name: [0; 128],
        enabled: false,
        priority: 0,
        selector_json: [0; 8192],
        primary_target_uuid: [0; 37],
        primary_pool_uuid: [0; 37],
        fallback_mode: [0; 16],
        fallback_target_uuid: [0; 37],
        minimum_retention_days: 0,
        desired_retention_days: 0,
        maximum_retention_days: 0,
        required_copy_count: 0,
        replication_pool_uuid: [0; 37],
        migration_after_days: 0,
        migration_target_uuid: [0; 37],
        pressure_priority: 0,
        archive_after_seconds: 0,
        hot_residency_seconds: 0,
        archive_protected: false,
        archive_on_pressure: false,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_storage_policy_result_t =
        db_storage_policy_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut policy);
    if result.0 != db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        return;
    }
    send_json(
        res,
        200 as ::core::ffi::c_int,
        policy_to_json(&raw mut policy),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_storage_policy(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut user: user_t = user_t {
        id: 0,
        username: [0; 64],
        email: [0; 128],
        role: user_role_t::USER_ROLE_ADMIN,
        api_key: [0; 64],
        created_at: 0,
        updated_at: 0,
        last_login: 0,
        is_active: false,
        password_change_locked: false,
        must_change_password: false,
        totp_enabled: false,
        allowed_login_cidrs: [0; 1024],
        has_login_cidr_restriction: false,
        authorization_mode: [0; 16],
        authenticated_via_scoped_token: false,
        api_token_uuid: [0; 37],
        authentication_method: [0; 24],
    };
    if !authorize_storage(req, res, &raw mut user) {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res) {
        return;
    }
    let mut policy: storage_policy_t = storage_policy_t {
        uuid: [0; 37],
        name: [0; 128],
        enabled: false,
        priority: 0,
        selector_json: [0; 8192],
        primary_target_uuid: [0; 37],
        primary_pool_uuid: [0; 37],
        fallback_mode: [0; 16],
        fallback_target_uuid: [0; 37],
        minimum_retention_days: 0,
        desired_retention_days: 0,
        maximum_retention_days: 0,
        required_copy_count: 0,
        replication_pool_uuid: [0; 37],
        migration_after_days: 0,
        migration_target_uuid: [0; 37],
        pressure_priority: 0,
        archive_after_seconds: 0,
        hot_residency_seconds: 0,
        archive_protected: false,
        archive_on_pressure: false,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_storage_policy_result_t =
        db_storage_policy_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut policy);
    if result.0 != db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut revision: int64_t = 0 as int64_t;
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
    if !apply_body(
        body,
        &raw mut policy,
        r#false != 0,
        &raw mut revision,
        &raw mut error as *mut ::core::ffi::c_char,
        res,
    ) {
        cJSON_Delete(body);
        audit_policy(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"policy_update\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_Delete(body);
    result = db_storage_policy_update(&raw mut policy, revision);
    if result.0 != db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0 {
        set_db_error(res, result, &raw mut error as *mut ::core::ffi::c_char);
        audit_policy(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"policy_update\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_policy(
        req,
        &raw mut user,
        &raw mut uuid as *mut ::core::ffi::c_char,
        b"policy_update\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
    );
    send_json(
        res,
        200 as ::core::ffi::c_int,
        policy_to_json(&raw mut policy),
    );
}
unsafe extern "C" fn revision_query(
    mut req: *const http_request_t,
    mut revision: *mut int64_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut value: [::core::ffi::c_char; 64] = [0; 64];
    if http_request_get_query_param(
        req,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) < 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"revision query parameter is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parsed: ::core::ffi::c_longlong = strtoll(
        &raw mut value as *mut ::core::ffi::c_char,
        &raw mut end,
        10 as ::core::ffi::c_int,
    );
    if *__errno_location() == ERANGE
        || end.is_null()
        || *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || parsed < 1 as ::core::ffi::c_longlong
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"revision must be a positive integer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *revision = parsed as int64_t;
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_storage_policy(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut user: user_t = user_t {
        id: 0,
        username: [0; 64],
        email: [0; 128],
        role: user_role_t::USER_ROLE_ADMIN,
        api_key: [0; 64],
        created_at: 0,
        updated_at: 0,
        last_login: 0,
        is_active: false,
        password_change_locked: false,
        must_change_password: false,
        totp_enabled: false,
        allowed_login_cidrs: [0; 1024],
        has_login_cidr_restriction: false,
        authorization_mode: [0; 16],
        authenticated_via_scoped_token: false,
        api_token_uuid: [0; 37],
        authentication_method: [0; 24],
    };
    if !authorize_storage(req, res, &raw mut user) {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    let mut revision: int64_t = 0 as int64_t;
    if !extract_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res)
        || !revision_query(req, &raw mut revision, res)
    {
        return;
    }
    let mut result: db_storage_policy_result_t =
        db_storage_policy_delete(&raw mut uuid as *mut ::core::ffi::c_char, revision);
    if result.0 != db_storage_policy_result_t::DB_STORAGE_POLICY_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        audit_policy(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"policy_delete\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_policy(
        req,
        &raw mut user,
        &raw mut uuid as *mut ::core::ffi::c_char,
        b"policy_delete\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
    );
    http_response_set_json(
        res,
        200 as ::core::ffi::c_int,
        b"{\"success\":true}\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_storage_policy_apply(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut user: user_t = user_t {
        id: 0,
        username: [0; 64],
        email: [0; 128],
        role: user_role_t::USER_ROLE_ADMIN,
        api_key: [0; 64],
        created_at: 0,
        updated_at: 0,
        last_login: 0,
        is_active: false,
        password_change_locked: false,
        must_change_password: false,
        totp_enabled: false,
        allowed_login_cidrs: [0; 1024],
        has_login_cidr_restriction: false,
        authorization_mode: [0; 16],
        authenticated_via_scoped_token: false,
        api_token_uuid: [0; 37],
        authentication_method: [0; 24],
    };
    if !authorize_storage(req, res, &raw mut user) {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res) {
        return;
    }
    let mut body: *mut cJSON = if !(*req).body.is_null() {
        cJSON_ParseWithLength((*req).body as *const ::core::ffi::c_char, (*req).body_len)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut revision: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut preview: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"preview\0".as_ptr() as *const ::core::ffi::c_char);
    if cJSON_IsNumber(revision) == 0
        || (*revision).valuedouble.is_finite() as ::core::ffi::c_int == 0
        || (*revision).valuedouble < 1 as ::core::ffi::c_int as ::core::ffi::c_double
        || (*revision).valuedouble > 9007199254740991.0f64
        || floor((*revision).valuedouble) != (*revision).valuedouble
        || cJSON_IsBool(preview) == 0
    {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"revision and preview boolean are required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut dry_run: bool = cJSON_IsTrue(preview) != 0;
    let mut expected: int64_t = (*revision).valuedouble as int64_t;
    cJSON_Delete(body);
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    if db.is_null() || mutex.is_null() {
        http_response_set_json_error(
            res,
            503 as ::core::ffi::c_int,
            b"Catalog unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    pthread_mutex_lock(mutex);
    let mut rc: ::core::ffi::c_int = sqlite3_exec(
        db,
        b"BEGIN IMMEDIATE;\0".as_ptr() as *const ::core::ffi::c_char,
        None,
        NULL,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    if rc == SQLITE_OK {
        rc = sqlite3_prepare_v2(
            db,
            b"SELECT p.revision,count(r.id),COALESCE(sum(r.size_bytes),0),COALESCE(sum(r.protected=0 AND COALESCE(r.retention_override_days,-1)<>0 AND CASE WHEN r.retention_override_days>0 THEN r.start_time+r.retention_override_days*86400<strftime('%s','now') ELSE p.maximum_retention_days>0 AND r.start_time+p.maximum_retention_days*86400<strftime('%s','now') END),0),COALESCE(sum(EXISTS(SELECT 1 FROM storage_migration_jobs j WHERE j.recording_id=r.id AND j.state<>'completed')),0) FROM storage_policies p LEFT JOIN recordings r ON r.storage_policy_uuid=p.uuid AND r.deletion_pending=0 AND r.storage_policy_version<>p.revision WHERE p.uuid=? GROUP BY p.uuid;\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
    }
    let mut current: int64_t = 0 as int64_t;
    let mut active: int64_t = 0 as int64_t;
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
        rc = sqlite3_step(stmt);
        if rc == SQLITE_ROW {
            current = sqlite3_column_int64(stmt, 0 as ::core::ffi::c_int) as int64_t;
            cJSON_AddNumberToObject(
                root,
                b"revision\0".as_ptr() as *const ::core::ffi::c_char,
                current as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                root,
                b"recordings\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                root,
                b"bytes\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int) as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                root,
                b"past_new_retention_limit\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_column_int64(stmt, 3 as ::core::ffi::c_int) as ::core::ffi::c_double,
            );
            active = sqlite3_column_int64(stmt, 4 as ::core::ffi::c_int) as int64_t;
            cJSON_AddNumberToObject(
                root,
                b"unfinished_transfers\0".as_ptr() as *const ::core::ffi::c_char,
                active as ::core::ffi::c_double,
            );
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    let mut ok: bool = rc == SQLITE_ROW
        && current == expected
        && (dry_run as ::core::ffi::c_int != 0 || active == 0);
    if ok as ::core::ffi::c_int != 0 && !dry_run {
        stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = sqlite3_prepare_v2(
            db,
            b"UPDATE recordings SET storage_policy_version=?2 WHERE storage_policy_uuid=?1 AND deletion_pending=0 AND storage_policy_version<>?2;\0"
                .as_ptr() as *const ::core::ffi::c_char,
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
            sqlite3_bind_int64(stmt, 2 as ::core::ffi::c_int, current as sqlite3_int64);
            rc = sqlite3_step(stmt);
        }
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
        }
        ok = rc == SQLITE_DONE;
    }
    if ok {
        ok = sqlite3_exec(
            db,
            b"COMMIT;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ) == SQLITE_OK;
    }
    if !ok {
        sqlite3_exec(
            db,
            b"ROLLBACK;\0".as_ptr() as *const ::core::ffi::c_char,
            None,
            NULL,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
    }
    pthread_mutex_unlock(mutex);
    if !ok {
        cJSON_Delete(root);
        http_response_set_json_error(
            res,
            409 as ::core::ffi::c_int,
            b"Policy revision changed, policy is missing, or associated transfers must finish first\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !dry_run {
        audit_policy(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"policy_apply_existing\0".as_ptr() as *const ::core::ffi::c_char,
            b"success\0".as_ptr() as *const ::core::ffi::c_char,
        );
        storage_migration_worker_wake();
    }
    cJSON_AddBoolToObject(
        root,
        b"preview\0".as_ptr() as *const ::core::ffi::c_char,
        dry_run as cJSON_bool,
    );
    send_json(res, 200 as ::core::ffi::c_int, root);
}
