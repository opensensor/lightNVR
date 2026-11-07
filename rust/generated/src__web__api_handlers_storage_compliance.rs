use ::libc;
unsafe extern "C" {
    pub type sqlite3_stmt;
    pub type sqlite3;
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
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
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
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn difftime(__time1: time_t, __time0: time_t) -> ::core::ffi::c_double;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn db_storage_lifecycle_reconcile() -> ::core::ffi::c_int;
    unsafe fn db_storage_target_count() -> ::core::ffi::c_int;
    unsafe fn db_storage_target_list(
        targets: *mut storage_target_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_storage_target_get(
        uuid: *const ::core::ffi::c_char,
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
    unsafe fn db_storage_policy_count() -> ::core::ffi::c_int;
    unsafe fn db_storage_policy_list(
        policies: *mut storage_policy_t,
        max_count: ::core::ffi::c_int,
        enabled_only: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn db_storage_pool_get(
        uuid: *const ::core::ffi::c_char,
        pool: *mut storage_pool_t,
    ) -> db_storage_pool_result_t;
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
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_target_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub target_type: [::core::ffi::c_char; 24],
    pub root_path: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub is_default: bool,
    pub storage_class: [::core::ffi::c_char; 16],
    pub reserve_bytes: uint64_t,
    pub high_watermark_pct: ::core::ffi::c_double,
    pub low_watermark_pct: ::core::ffi::c_double,
    pub health_status: [::core::ffi::c_char; 16],
    pub capacity_bytes: uint64_t,
    pub available_bytes: uint64_t,
    pub filesystem_device: uint64_t,
    pub last_probe_at: int64_t,
    pub last_success_at: int64_t,
    pub last_error: [::core::ffi::c_char; 256],
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub recording_count: uint64_t,
    pub recording_bytes: uint64_t,
    pub mount_required: bool,
    pub mount_guard_path: [::core::ffi::c_char; 512],
    pub migration_bandwidth_bps: uint64_t,
    pub archival_window_start_minute: ::core::ffi::c_int,
    pub archival_window_end_minute: ::core::ffi::c_int,
    pub replica_count: uint64_t,
    pub replica_bytes: uint64_t,
    pub endpoint: [::core::ffi::c_char; 512],
    pub region: [::core::ffi::c_char; 64],
    pub bucket: [::core::ffi::c_char; 128],
    pub credential_ref: [::core::ffi::c_char; 128],
    pub archive_budget_bytes: uint64_t,
}
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct policy_history_t {
    pub recent_bytes: uint64_t,
    pub first_recent: int64_t,
    pub last_recent: int64_t,
    pub samples: ::core::ffi::c_int,
    pub total_recordings: ::core::ffi::c_int,
    pub total_bytes: uint64_t,
    pub oldest: int64_t,
    pub copy_deficits: ::core::ffi::c_int,
    pub active_violations: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_storage_target_result_t(pub ::core::ffi::c_int);
impl db_storage_target_result_t {
    pub const DB_STORAGE_TARGET_OK: Self = Self(0);
    pub const DB_STORAGE_TARGET_NOT_FOUND: Self = Self(-1);
    pub const DB_STORAGE_TARGET_CONFLICT: Self = Self(-2);
    pub const DB_STORAGE_TARGET_INVALID: Self = Self(-3);
    pub const DB_STORAGE_TARGET_STALE: Self = Self(-4);
    pub const DB_STORAGE_TARGET_LIMIT: Self = Self(-5);
    pub const DB_STORAGE_TARGET_IN_USE: Self = Self(-6);
    pub const DB_STORAGE_TARGET_UNAVAILABLE: Self = Self(-7);
    pub const DB_STORAGE_TARGET_ERROR: Self = Self(-8);
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
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
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
    pub const AUTHZ_ACTION_COUNT: Self = Self(19);
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const STORAGE_TARGET_MAX_COUNT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const STORAGE_POLICY_MAX_COUNT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
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
unsafe extern "C" fn confidence_for(
    mut sample_days: ::core::ffi::c_double,
    mut samples: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if samples < 3 as ::core::ffi::c_int || sample_days < 3.0f64 {
        return b"low\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if sample_days < 14.0f64 {
        return b"medium\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return b"high\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn add_nullable_number(
    mut object: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut available: bool,
    mut value: ::core::ffi::c_double,
) {
    if available {
        cJSON_AddNumberToObject(object, name, value);
    } else {
        cJSON_AddNullToObject(object, name);
    };
}
unsafe extern "C" fn target_history(
    mut uuid: *const ::core::ffi::c_char,
    mut bytes: *mut uint64_t,
    mut first: *mut int64_t,
    mut last: *mut int64_t,
    mut samples: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT COALESCE(SUM(size_bytes),0),MIN(observed_at),MAX(observed_at),count(*) FROM (SELECT size_bytes,start_time observed_at FROM recordings WHERE storage_target_uuid=? AND is_complete=1 AND start_time>=strftime('%s','now')-2592000 UNION ALL SELECT size_bytes,created_at FROM storage_recording_copies WHERE target_uuid=? AND created_at>=strftime('%s','now')-2592000);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        sqlite3_bind_text(
            statement,
            2 as ::core::ffi::c_int,
            uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    if result == SQLITE_ROW {
        *bytes = sqlite3_column_int64(statement, 0 as ::core::ffi::c_int) as uint64_t;
        *first = sqlite3_column_int64(statement, 1 as ::core::ffi::c_int) as int64_t;
        *last = sqlite3_column_int64(statement, 2 as ::core::ffi::c_int) as int64_t;
        *samples = sqlite3_column_int(statement, 3 as ::core::ffi::c_int);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return if result == SQLITE_ROW {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn target_forecast(mut target: *const storage_target_t) -> *mut cJSON {
    let mut bytes: uint64_t = 0 as uint64_t;
    let mut first: int64_t = 0 as int64_t;
    let mut last: int64_t = 0 as int64_t;
    let mut samples: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if target_history(
        &raw const (*target).uuid as *const ::core::ffi::c_char,
        &raw mut bytes,
        &raw mut first,
        &raw mut last,
        &raw mut samples,
    ) != 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut sample_days: ::core::ffi::c_double = if first > 0 as int64_t && last >= first {
        (last - first) as ::core::ffi::c_double / 86400.0f64 + 1.0f64
    } else {
        0.0f64
    };
    let mut daily_rate: ::core::ffi::c_double = if sample_days > 0.0f64 {
        bytes as ::core::ffi::c_double / sample_days
    } else {
        0.0f64
    };
    let mut threshold_free: ::core::ffi::c_double = (*target).capacity_bytes
        as ::core::ffi::c_double
        * (100.0f64 - (*target).high_watermark_pct)
        / 100.0f64;
    let mut headroom: ::core::ffi::c_double =
        if (*target).available_bytes as ::core::ffi::c_double > threshold_free {
            (*target).available_bytes as ::core::ffi::c_double - threshold_free
        } else {
            0.0f64
        };
    let mut usable: ::core::ffi::c_double =
        (*target).capacity_bytes as ::core::ffi::c_double * (*target).high_watermark_pct / 100.0f64;
    if usable > (*target).reserve_bytes as ::core::ffi::c_double {
        usable -= (*target).reserve_bytes as ::core::ffi::c_double;
    } else {
        usable = 0.0f64;
    }
    let mut object: *mut cJSON = cJSON_CreateObject();
    if object.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"target_name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).name as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        object,
        b"sample_window_days\0".as_ptr() as *const ::core::ffi::c_char,
        sample_days,
    );
    cJSON_AddNumberToObject(
        object,
        b"sample_count\0".as_ptr() as *const ::core::ffi::c_char,
        samples as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"observed_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        bytes as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"observed_daily_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        daily_rate,
    );
    cJSON_AddStringToObject(
        object,
        b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
        confidence_for(sample_days, samples),
    );
    add_nullable_number(
        object,
        b"days_to_high_watermark\0".as_ptr() as *const ::core::ffi::c_char,
        daily_rate > 0.0f64,
        if daily_rate > 0.0f64 {
            headroom / daily_rate
        } else {
            0.0f64
        },
    );
    add_nullable_number(
        object,
        b"expected_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        daily_rate > 0.0f64,
        if daily_rate > 0.0f64 {
            usable / daily_rate
        } else {
            0.0f64
        },
    );
    return object;
}
unsafe extern "C" fn policy_history(
    mut uuid: *const ::core::ffi::c_char,
    mut history: *mut policy_history_t,
) -> ::core::ffi::c_int {
    memset(
        history as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<policy_history_t>(),
    );
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut sql: *const ::core::ffi::c_char = b"SELECT COALESCE(SUM(CASE WHEN r.start_time>=strftime('%s','now')-2592000 THEN r.size_bytes ELSE 0 END),0),MIN(CASE WHEN r.start_time>=strftime('%s','now')-2592000 THEN r.start_time END),MAX(CASE WHEN r.start_time>=strftime('%s','now')-2592000 THEN r.start_time END),SUM(CASE WHEN r.start_time>=strftime('%s','now')-2592000 THEN 1 ELSE 0 END),count(r.id),COALESCE(SUM(r.size_bytes),0),MIN(r.start_time),SUM(CASE WHEN r.id IS NOT NULL AND p.required_copy_count > 1+(SELECT count(*) FROM storage_recording_copies c WHERE c.recording_id=r.id) THEN 1 ELSE 0 END),(SELECT count(*) FROM storage_policy_violations v WHERE v.policy_uuid=p.uuid AND v.resolved_at IS NULL) FROM storage_policies p LEFT JOIN recordings r ON p.uuid=substr(r.placement_reason,instr(r.placement_reason,':')+1) WHERE p.uuid=? GROUP BY p.uuid;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        sql,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if result == SQLITE_OK {
        sqlite3_bind_text(
            statement,
            1 as ::core::ffi::c_int,
            uuid,
            -1 as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
        result = sqlite3_step(statement);
    }
    if result == SQLITE_ROW {
        (*history).recent_bytes =
            sqlite3_column_int64(statement, 0 as ::core::ffi::c_int) as uint64_t;
        (*history).first_recent =
            sqlite3_column_int64(statement, 1 as ::core::ffi::c_int) as int64_t;
        (*history).last_recent =
            sqlite3_column_int64(statement, 2 as ::core::ffi::c_int) as int64_t;
        (*history).samples = sqlite3_column_int(statement, 3 as ::core::ffi::c_int);
        (*history).total_recordings = sqlite3_column_int(statement, 4 as ::core::ffi::c_int);
        (*history).total_bytes =
            sqlite3_column_int64(statement, 5 as ::core::ffi::c_int) as uint64_t;
        (*history).oldest = sqlite3_column_int64(statement, 6 as ::core::ffi::c_int) as int64_t;
        (*history).copy_deficits = sqlite3_column_int(statement, 7 as ::core::ffi::c_int);
        (*history).active_violations = sqlite3_column_int(statement, 8 as ::core::ffi::c_int);
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return if result == SQLITE_ROW {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn policy_usable_capacity(
    mut policy: *const storage_policy_t,
) -> ::core::ffi::c_double {
    if (*policy).primary_pool_uuid[0usize] != 0 {
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
            &raw const (*policy).primary_pool_uuid as *const ::core::ffi::c_char,
            &raw mut pool,
        )
        .0 != db_storage_pool_result_t::DB_STORAGE_POOL_OK.0
        {
            return 0.0f64;
        }
        let mut total: ::core::ffi::c_double = 0.0f64;
        let mut devices: [uint64_t; 128] = [
            0 as uint64_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
        let mut device_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while index < pool.member_count {
            let mut target: storage_target_t = storage_target_t {
                uuid: [0; 37],
                name: [0; 128],
                target_type: [0; 24],
                root_path: [0; 512],
                enabled: false,
                is_default: false,
                storage_class: [0; 16],
                reserve_bytes: 0,
                high_watermark_pct: 0.,
                low_watermark_pct: 0.,
                health_status: [0; 16],
                capacity_bytes: 0,
                available_bytes: 0,
                filesystem_device: 0,
                last_probe_at: 0,
                last_success_at: 0,
                last_error: [0; 256],
                revision: 0,
                created_at: 0,
                updated_at: 0,
                recording_count: 0,
                recording_bytes: 0,
                mount_required: false,
                mount_guard_path: [0; 512],
                migration_bandwidth_bps: 0,
                archival_window_start_minute: 0,
                archival_window_end_minute: 0,
                replica_count: 0,
                replica_bytes: 0,
                endpoint: [0; 512],
                region: [0; 64],
                bucket: [0; 128],
                credential_ref: [0; 128],
                archive_budget_bytes: 0,
            };
            if !(db_storage_target_get(
                &raw mut (*(&raw mut pool.members as *mut storage_pool_member_t)
                    .offset(index as isize))
                .target_uuid as *mut ::core::ffi::c_char,
                &raw mut target,
            )
            .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
                || !target.enabled)
            {
                let mut duplicate_device: bool = r#false != 0;
                if target.filesystem_device != 0 as uint64_t {
                    let mut seen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while seen < device_count {
                        if devices[seen as usize] == target.filesystem_device {
                            duplicate_device = r#true != 0;
                            break;
                        } else {
                            seen += 1;
                        }
                    }
                    if !duplicate_device {
                        let c2rust_fresh0 = device_count;
                        device_count += 1;
                        devices[c2rust_fresh0 as usize] = target.filesystem_device;
                    }
                }
                if !duplicate_device {
                    let mut usable: ::core::ffi::c_double =
                        target.capacity_bytes as ::core::ffi::c_double * target.high_watermark_pct
                            / 100.0f64;
                    if usable > target.reserve_bytes as ::core::ffi::c_double {
                        total += usable - target.reserve_bytes as ::core::ffi::c_double;
                    }
                }
            }
            index += 1;
        }
        return total;
    }
    let mut target_0: storage_target_t = storage_target_t {
        uuid: [0; 37],
        name: [0; 128],
        target_type: [0; 24],
        root_path: [0; 512],
        enabled: false,
        is_default: false,
        storage_class: [0; 16],
        reserve_bytes: 0,
        high_watermark_pct: 0.,
        low_watermark_pct: 0.,
        health_status: [0; 16],
        capacity_bytes: 0,
        available_bytes: 0,
        filesystem_device: 0,
        last_probe_at: 0,
        last_success_at: 0,
        last_error: [0; 256],
        revision: 0,
        created_at: 0,
        updated_at: 0,
        recording_count: 0,
        recording_bytes: 0,
        mount_required: false,
        mount_guard_path: [0; 512],
        migration_bandwidth_bps: 0,
        archival_window_start_minute: 0,
        archival_window_end_minute: 0,
        replica_count: 0,
        replica_bytes: 0,
        endpoint: [0; 512],
        region: [0; 64],
        bucket: [0; 128],
        credential_ref: [0; 128],
        archive_budget_bytes: 0,
    };
    if db_storage_target_get(
        &raw const (*policy).primary_target_uuid as *const ::core::ffi::c_char,
        &raw mut target_0,
    )
    .0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0
    {
        return 0.0f64;
    }
    let mut usable_0: ::core::ffi::c_double =
        target_0.capacity_bytes as ::core::ffi::c_double * target_0.high_watermark_pct / 100.0f64;
    return if usable_0 > target_0.reserve_bytes as ::core::ffi::c_double {
        usable_0 - target_0.reserve_bytes as ::core::ffi::c_double
    } else {
        0.0f64
    };
}
unsafe extern "C" fn policy_forecast(mut policy: *const storage_policy_t) -> *mut cJSON {
    let mut history: policy_history_t = policy_history_t {
        recent_bytes: 0,
        first_recent: 0,
        last_recent: 0,
        samples: 0,
        total_recordings: 0,
        total_bytes: 0,
        oldest: 0,
        copy_deficits: 0,
        active_violations: 0,
    };
    if policy_history(
        &raw const (*policy).uuid as *const ::core::ffi::c_char,
        &raw mut history,
    ) != 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut sample_days: ::core::ffi::c_double = if history.first_recent > 0 as int64_t
        && history.last_recent >= history.first_recent
    {
        (history.last_recent - history.first_recent) as ::core::ffi::c_double / 86400.0f64 + 1.0f64
    } else {
        0.0f64
    };
    let mut daily_rate: ::core::ffi::c_double = if sample_days > 0.0f64 {
        history.recent_bytes as ::core::ffi::c_double / sample_days
    } else {
        0.0f64
    };
    let mut expected_days: ::core::ffi::c_double = if daily_rate > 0.0f64 {
        policy_usable_capacity(policy) / daily_rate
    } else {
        0.0f64
    };
    let mut achieved_days: ::core::ffi::c_double = if history.oldest > 0 as int64_t {
        difftime(
            time(::core::ptr::null_mut::<time_t>()),
            history.oldest as time_t,
        ) / 86400.0f64
    } else {
        0.0f64
    };
    let mut object: *mut cJSON = cJSON_CreateObject();
    if object.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"policy_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*policy).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"policy_name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*policy).name as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        object,
        b"recording_count\0".as_ptr() as *const ::core::ffi::c_char,
        history.total_recordings as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"recording_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        history.total_bytes as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"sample_window_days\0".as_ptr() as *const ::core::ffi::c_char,
        sample_days,
    );
    cJSON_AddNumberToObject(
        object,
        b"observed_daily_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        daily_rate,
    );
    cJSON_AddStringToObject(
        object,
        b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
        confidence_for(sample_days, history.samples),
    );
    cJSON_AddNumberToObject(
        object,
        b"achieved_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        achieved_days,
    );
    add_nullable_number(
        object,
        b"expected_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        daily_rate > 0.0f64,
        expected_days,
    );
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
    cJSON_AddNumberToObject(
        object,
        b"copy_deficit_recordings\0".as_ptr() as *const ::core::ffi::c_char,
        history.copy_deficits as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"active_violation_count\0".as_ptr() as *const ::core::ffi::c_char,
        history.active_violations as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        object,
        b"minimum_forecast_met\0".as_ptr() as *const ::core::ffi::c_char,
        ((*policy).minimum_retention_days == 0 as ::core::ffi::c_int
            || daily_rate == 0.0f64
            || expected_days >= (*policy).minimum_retention_days as ::core::ffi::c_double)
            as ::core::ffi::c_int,
    );
    return object;
}
unsafe extern "C" fn active_violations() -> *mut cJSON {
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if items.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    pthread_mutex_lock(mutex);
    let mut statement: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut result: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT v.uuid,v.policy_uuid,p.name,COALESCE(v.recording_id,0),v.violation_type,v.details,v.first_seen_at,v.last_seen_at FROM storage_policy_violations v JOIN storage_policies p ON p.uuid=v.policy_uuid WHERE v.resolved_at IS NULL ORDER BY v.first_seen_at,v.uuid;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut statement,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    while result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW {
        let mut item: *mut cJSON = cJSON_CreateObject();
        if item.is_null() {
            cJSON_Delete(items);
            items = ::core::ptr::null_mut::<cJSON>();
            break;
        } else {
            cJSON_AddStringToObject(
                item,
                b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_column_text(statement, 0 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                item,
                b"policy_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_column_text(statement, 1 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                item,
                b"policy_name\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_column_text(statement, 2 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
            );
            let mut recording_id: sqlite3_int64 =
                sqlite3_column_int64(statement, 3 as ::core::ffi::c_int);
            if recording_id > 0 as sqlite3_int64 {
                cJSON_AddNumberToObject(
                    item,
                    b"recording_id\0".as_ptr() as *const ::core::ffi::c_char,
                    recording_id as ::core::ffi::c_double,
                );
            } else {
                cJSON_AddNullToObject(
                    item,
                    b"recording_id\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            cJSON_AddStringToObject(
                item,
                b"type\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_column_text(statement, 4 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                item,
                b"details\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_column_text(statement, 5 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
            );
            cJSON_AddNumberToObject(
                item,
                b"first_seen_at\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_column_int64(statement, 6 as ::core::ffi::c_int) as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                item,
                b"last_seen_at\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_column_int64(statement, 7 as ::core::ffi::c_int) as ::core::ffi::c_double,
            );
            cJSON_AddItemToArray(items, item);
        }
    }
    if !statement.is_null() {
        sqlite3_finalize(statement);
    }
    pthread_mutex_unlock(mutex);
    return items;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_storage_compliance(
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
    if db_storage_lifecycle_reconcile() < 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to refresh lifecycle compliance\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut target_total: ::core::ffi::c_int = db_storage_target_count();
    let mut policy_total: ::core::ffi::c_int = db_storage_policy_count();
    let mut targets: *mut storage_target_t =
        (if target_total > 0 as ::core::ffi::c_int && target_total <= STORAGE_TARGET_MAX_COUNT {
            calloc(
                target_total as size_t,
                ::core::mem::size_of::<storage_target_t>(),
            )
        } else {
            NULL
        }) as *mut storage_target_t;
    let mut policies: *mut storage_policy_t =
        (if policy_total > 0 as ::core::ffi::c_int && policy_total <= STORAGE_POLICY_MAX_COUNT {
            calloc(
                policy_total as size_t,
                ::core::mem::size_of::<storage_policy_t>(),
            )
        } else {
            NULL
        }) as *mut storage_policy_t;
    let mut target_count: ::core::ffi::c_int = if target_total < 0 as ::core::ffi::c_int
        || target_total > STORAGE_TARGET_MAX_COUNT
        || target_total > 0 as ::core::ffi::c_int && targets.is_null()
    {
        -1 as ::core::ffi::c_int
    } else if target_total > 0 as ::core::ffi::c_int {
        db_storage_target_list(targets, target_total)
    } else {
        0 as ::core::ffi::c_int
    };
    let mut policy_count: ::core::ffi::c_int = if policy_total < 0 as ::core::ffi::c_int
        || policy_total > STORAGE_POLICY_MAX_COUNT
        || policy_total > 0 as ::core::ffi::c_int && policies.is_null()
    {
        -1 as ::core::ffi::c_int
    } else if policy_total > 0 as ::core::ffi::c_int {
        db_storage_policy_list(policies, policy_total, r#false != 0)
    } else {
        0 as ::core::ffi::c_int
    };
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut target_items: *mut cJSON = cJSON_CreateArray();
    let mut policy_items: *mut cJSON = cJSON_CreateArray();
    let mut violations: *mut cJSON = active_violations();
    if target_count < 0 as ::core::ffi::c_int
        || policy_count < 0 as ::core::ffi::c_int
        || root.is_null()
        || target_items.is_null()
        || policy_items.is_null()
        || violations.is_null()
    {
        free(targets as *mut ::core::ffi::c_void);
        free(policies as *mut ::core::ffi::c_void);
        cJSON_Delete(root);
        cJSON_Delete(target_items);
        cJSON_Delete(policy_items);
        cJSON_Delete(violations);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build lifecycle compliance\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddNumberToObject(
        root,
        b"forecast_window_days\0".as_ptr() as *const ::core::ffi::c_char,
        30 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        root,
        b"forecast_note\0".as_ptr() as *const ::core::ffi::c_char,
        b"Observed write rate; sparse or changing workloads reduce confidence.\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    cJSON_AddItemToObject(
        root,
        b"targets\0".as_ptr() as *const ::core::ffi::c_char,
        target_items,
    );
    cJSON_AddItemToObject(
        root,
        b"policies\0".as_ptr() as *const ::core::ffi::c_char,
        policy_items,
    );
    cJSON_AddItemToObject(
        root,
        b"active_violations\0".as_ptr() as *const ::core::ffi::c_char,
        violations,
    );
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < target_count {
        let mut item: *mut cJSON = target_forecast(targets.offset(index as isize));
        if !item.is_null() {
            cJSON_AddItemToArray(target_items, item);
        }
        index += 1;
    }
    let mut index_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index_0 < policy_count {
        let mut item_0: *mut cJSON = policy_forecast(policies.offset(index_0 as isize));
        if !item_0.is_null() {
            cJSON_AddItemToArray(policy_items, item_0);
        }
        index_0 += 1;
    }
    free(targets as *mut ::core::ffi::c_void);
    free(policies as *mut ::core::ffi::c_void);
    let mut body: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if body.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize lifecycle compliance\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, body);
    free(body as *mut ::core::ffi::c_void);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
