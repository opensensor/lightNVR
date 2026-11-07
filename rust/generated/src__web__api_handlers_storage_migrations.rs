use ::libc;
unsafe extern "C" {
    pub type sqlite3_stmt;
    pub type sqlite3;
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
    unsafe fn cJSON_ParseWithLength(
        value: *const ::core::ffi::c_char,
        buffer_length: size_t,
    ) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
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
    unsafe fn cJSON_AddArrayToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn sqlite3_changes(_: *mut sqlite3) -> ::core::ffi::c_int;
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
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn get_db_handle() -> *mut sqlite3;
    unsafe fn get_db_mutex() -> *mut pthread_mutex_t;
    unsafe fn storage_migration_worker_wake();
    unsafe fn db_storage_migration_create_operation(
        recording_id: uint64_t,
        destination_target_uuid: *const ::core::ffi::c_char,
        operation: *const ::core::ffi::c_char,
        owner_user_id: int64_t,
        job: *mut storage_migration_job_t,
    ) -> db_storage_migration_result_t;
    unsafe fn db_storage_migration_get(
        uuid: *const ::core::ffi::c_char,
        job: *mut storage_migration_job_t,
    ) -> db_storage_migration_result_t;
    unsafe fn db_storage_migration_list(
        jobs: *mut storage_migration_job_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_storage_migration_request_cancel(
        uuid: *const ::core::ffi::c_char,
        job: *mut storage_migration_job_t,
    ) -> db_storage_migration_result_t;
    unsafe fn db_storage_migration_retry(
        uuid: *const ::core::ffi::c_char,
        job: *mut storage_migration_job_t,
    ) -> db_storage_migration_result_t;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
pub struct storage_migration_job_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub recording_id: uint64_t,
    pub owner_user_id: int64_t,
    pub operation: [::core::ffi::c_char; 8],
    pub source_target_uuid: [::core::ffi::c_char; 37],
    pub source_object_key: [::core::ffi::c_char; 512],
    pub destination_target_uuid: [::core::ffi::c_char; 37],
    pub destination_object_key: [::core::ffi::c_char; 512],
    pub state: [::core::ffi::c_char; 24],
    pub checksum: [::core::ffi::c_char; 65],
    pub bytes_total: uint64_t,
    pub bytes_copied: uint64_t,
    pub attempt_count: ::core::ffi::c_int,
    pub max_attempts: ::core::ffi::c_int,
    pub next_attempt_at: int64_t,
    pub last_error: [::core::ffi::c_char; 256],
    pub cancel_requested: bool,
    pub bandwidth_limit_bps: uint64_t,
    pub window_start_minute: ::core::ffi::c_int,
    pub window_end_minute: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub started_at: int64_t,
    pub completed_at: int64_t,
}
pub type cJSON_bool = ::core::ffi::c_int;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_storage_migration_result_t(pub ::core::ffi::c_int);
impl db_storage_migration_result_t {
    pub const DB_STORAGE_MIGRATION_OK: Self = Self(0);
    pub const DB_STORAGE_MIGRATION_NOT_FOUND: Self = Self(-1);
    pub const DB_STORAGE_MIGRATION_CONFLICT: Self = Self(-2);
    pub const DB_STORAGE_MIGRATION_INVALID: Self = Self(-3);
    pub const DB_STORAGE_MIGRATION_SOURCE_INCOMPLETE: Self = Self(-4);
    pub const DB_STORAGE_MIGRATION_TARGET_UNAVAILABLE: Self = Self(-5);
    pub const DB_STORAGE_MIGRATION_SOURCE_CHANGED: Self = Self(-6);
    pub const DB_STORAGE_MIGRATION_ERROR: Self = Self(-7);
}
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const STORAGE_MIGRATION_MAX_VISIBLE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const LIGHTNVR_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn job_json(mut job: *const storage_migration_job_t) -> *mut cJSON {
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        root,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*job).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        root,
        b"recording_id\0".as_ptr() as *const ::core::ffi::c_char,
        (*job).recording_id as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        root,
        b"operation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*job).operation as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"source_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*job).source_target_uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"source_object_key\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*job).source_object_key as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"destination_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*job).destination_target_uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"destination_object_key\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*job).destination_object_key as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"state\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*job).state as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"checksum_mode\0".as_ptr() as *const ::core::ffi::c_char,
        b"sha256\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if (*job).checksum[0usize] != 0 {
        cJSON_AddStringToObject(
            root,
            b"checksum\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*job).checksum as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(root, b"checksum\0".as_ptr() as *const ::core::ffi::c_char);
    }
    cJSON_AddNumberToObject(
        root,
        b"bytes_total\0".as_ptr() as *const ::core::ffi::c_char,
        (*job).bytes_total as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"bytes_copied\0".as_ptr() as *const ::core::ffi::c_char,
        (*job).bytes_copied as ::core::ffi::c_double,
    );
    let mut progress: ::core::ffi::c_double = if (*job).bytes_total > 0 as uint64_t {
        (*job).bytes_copied as ::core::ffi::c_double / (*job).bytes_total as ::core::ffi::c_double
    } else {
        0.0f64
    };
    cJSON_AddNumberToObject(
        root,
        b"progress\0".as_ptr() as *const ::core::ffi::c_char,
        progress,
    );
    cJSON_AddNumberToObject(
        root,
        b"attempt_count\0".as_ptr() as *const ::core::ffi::c_char,
        (*job).attempt_count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"max_attempts\0".as_ptr() as *const ::core::ffi::c_char,
        (*job).max_attempts as ::core::ffi::c_double,
    );
    if (*job).next_attempt_at > 0 as int64_t {
        cJSON_AddNumberToObject(
            root,
            b"next_attempt_at\0".as_ptr() as *const ::core::ffi::c_char,
            (*job).next_attempt_at as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(
            root,
            b"next_attempt_at\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddStringToObject(
        root,
        b"last_error\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*job).last_error as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        root,
        b"cancel_requested\0".as_ptr() as *const ::core::ffi::c_char,
        (*job).cancel_requested as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        root,
        b"bandwidth_limit_bps\0".as_ptr() as *const ::core::ffi::c_char,
        (*job).bandwidth_limit_bps as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"window_start_minute\0".as_ptr() as *const ::core::ffi::c_char,
        (*job).window_start_minute as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"window_end_minute\0".as_ptr() as *const ::core::ffi::c_char,
        (*job).window_end_minute as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        (*job).revision as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*job).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*job).updated_at as ::core::ffi::c_double,
    );
    if (*job).started_at > 0 as int64_t {
        cJSON_AddNumberToObject(
            root,
            b"started_at\0".as_ptr() as *const ::core::ffi::c_char,
            (*job).started_at as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(root, b"started_at\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if (*job).completed_at > 0 as int64_t {
        cJSON_AddNumberToObject(
            root,
            b"completed_at\0".as_ptr() as *const ::core::ffi::c_char,
            (*job).completed_at as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(
            root,
            b"completed_at\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return root;
}
unsafe extern "C" fn send_json(
    mut res: *mut http_response_t,
    mut status: ::core::ffi::c_int,
    mut root: *mut cJSON,
) {
    let mut encoded: *mut ::core::ffi::c_char = if !root.is_null() {
        cJSON_PrintUnformatted(root)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    cJSON_Delete(root);
    if encoded.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize migration response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, status, encoded);
    free(encoded as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn set_db_error(
    mut res: *mut http_response_t,
    mut result: db_storage_migration_result_t,
) {
    match result {
        db_storage_migration_result_t::DB_STORAGE_MIGRATION_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Recording, target, or job not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_storage_migration_result_t::DB_STORAGE_MIGRATION_CONFLICT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Migration conflicts with the recording, target, or current job state\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_storage_migration_result_t::DB_STORAGE_MIGRATION_SOURCE_INCOMPLETE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Only complete recordings can be migrated\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_storage_migration_result_t::DB_STORAGE_MIGRATION_TARGET_UNAVAILABLE => {
            http_response_set_json_error(
                res,
                422 as ::core::ffi::c_int,
                b"Destination storage target is unavailable\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_storage_migration_result_t::DB_STORAGE_MIGRATION_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid storage migration request\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Storage migration operation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn extract_job_uuid(
    mut req: *const http_request_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut res: *mut http_response_t,
) -> bool {
    let mut value: [::core::ffi::c_char; 512] = [0; 512];
    if http_request_extract_path_param(
        req,
        b"/api/storage-migrations/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid storage migration path\0".as_ptr() as *const ::core::ffi::c_char,
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
            b"Invalid storage migration UUID\0".as_ptr() as *const ::core::ffi::c_char,
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_storage_migrations(
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
    let mut jobs: *mut storage_migration_job_t = calloc(
        STORAGE_MIGRATION_MAX_VISIBLE as size_t,
        ::core::mem::size_of::<storage_migration_job_t>(),
    ) as *mut storage_migration_job_t;
    if jobs.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int =
        db_storage_migration_list(jobs, STORAGE_MIGRATION_MAX_VISIBLE);
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = if !root.is_null() {
        cJSON_AddArrayToObject(root, b"jobs\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if count < 0 as ::core::ffi::c_int || root.is_null() || items.is_null() {
        free(jobs as *mut ::core::ffi::c_void);
        cJSON_Delete(root);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to list storage migrations\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut item: *mut cJSON = job_json(jobs.offset(index as isize));
        if item.is_null() || cJSON_AddItemToArray(items, item) == 0 {
            cJSON_Delete(item);
            free(jobs as *mut ::core::ffi::c_void);
            cJSON_Delete(root);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create migration response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        index += 1;
    }
    free(jobs as *mut ::core::ffi::c_void);
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"worker_concurrency\0".as_ptr() as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    send_json(res, 200 as ::core::ffi::c_int, root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_storage_migration(
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
    let mut recording: *const cJSON = if !body.is_null() {
        cJSON_GetObjectItemCaseSensitive(
            body,
            b"recording_id\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut destination: *const cJSON = if !body.is_null() {
        cJSON_GetObjectItemCaseSensitive(
            body,
            b"destination_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut operation: *const cJSON = if !body.is_null() {
        cJSON_GetObjectItemCaseSensitive(
            body,
            b"operation\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut valid_id: bool = cJSON_IsNumber(recording) != 0
        && (*recording).valuedouble.is_finite() as ::core::ffi::c_int != 0
        && (*recording).valuedouble >= 1.0f64
        && (*recording).valuedouble <= 9007199254740991.0f64
        && floor((*recording).valuedouble) == (*recording).valuedouble;
    let mut valid_destination: bool = cJSON_IsString(destination) != 0
        && !(*destination).valuestring.is_null()
        && lightnvr_uuid_is_valid((*destination).valuestring) as ::core::ffi::c_int != 0;
    let mut valid_operation: bool = operation.is_null()
        || cJSON_IsString(operation) != 0
            && !(*operation).valuestring.is_null()
            && (strcmp(
                (*operation).valuestring,
                b"move\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    (*operation).valuestring,
                    b"copy\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int);
    if cJSON_IsObject(body) == 0 || !valid_id || !valid_destination || !valid_operation {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"recording_id, destination_target_uuid, and a valid operation are required\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut recording_id: uint64_t = (*recording).valuedouble as uint64_t;
    let mut destination_uuid: [::core::ffi::c_char; 37] = [0; 37];
    safe_strcpy(
        &raw mut destination_uuid as *mut ::core::ffi::c_char,
        (*destination).valuestring,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    let mut operation_name: [::core::ffi::c_char; 8] = [0; 8];
    safe_strcpy(
        &raw mut operation_name as *mut ::core::ffi::c_char,
        if !operation.is_null() {
            (*operation).valuestring
        } else {
            b"move\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
        0 as size_t,
    );
    cJSON_Delete(body);
    let mut job: storage_migration_job_t = storage_migration_job_t {
        uuid: [0; 37],
        recording_id: 0,
        owner_user_id: 0,
        operation: [0; 8],
        source_target_uuid: [0; 37],
        source_object_key: [0; 512],
        destination_target_uuid: [0; 37],
        destination_object_key: [0; 512],
        state: [0; 24],
        checksum: [0; 65],
        bytes_total: 0,
        bytes_copied: 0,
        attempt_count: 0,
        max_attempts: 0,
        next_attempt_at: 0,
        last_error: [0; 256],
        cancel_requested: false,
        bandwidth_limit_bps: 0,
        window_start_minute: 0,
        window_end_minute: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
        started_at: 0,
        completed_at: 0,
    };
    let mut result: db_storage_migration_result_t = db_storage_migration_create_operation(
        recording_id,
        &raw mut destination_uuid as *mut ::core::ffi::c_char,
        &raw mut operation_name as *mut ::core::ffi::c_char,
        user.id,
        &raw mut job,
    );
    if result.0 != db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0 {
        set_db_error(res, result);
        audit_log_operation(
            req,
            &raw mut user,
            b"storage.configure\0".as_ptr() as *const ::core::ffi::c_char,
            b"storage_migration\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"migration_create\0".as_ptr() as *const ::core::ffi::c_char,
            if result.0 == db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::ptr::null::<cJSON>(),
        );
        return;
    }
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddNumberToObject(
            details,
            b"recording_id\0".as_ptr() as *const ::core::ffi::c_char,
            recording_id as ::core::ffi::c_double,
        );
        cJSON_AddStringToObject(
            details,
            b"destination_target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut destination_uuid as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"operation\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut operation_name as *mut ::core::ffi::c_char,
        );
    }
    audit_log_operation(
        req,
        &raw mut user,
        b"storage.configure\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage_migration\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut job.uuid as *mut ::core::ffi::c_char,
        b"migration_create\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        details,
    );
    cJSON_Delete(details);
    storage_migration_worker_wake();
    send_json(res, 202 as ::core::ffi::c_int, job_json(&raw mut job));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_storage_migration(
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
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !authorize_storage(req, res, &raw mut user)
        || !extract_job_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res)
    {
        return;
    }
    let mut job: storage_migration_job_t = storage_migration_job_t {
        uuid: [0; 37],
        recording_id: 0,
        owner_user_id: 0,
        operation: [0; 8],
        source_target_uuid: [0; 37],
        source_object_key: [0; 512],
        destination_target_uuid: [0; 37],
        destination_object_key: [0; 512],
        state: [0; 24],
        checksum: [0; 65],
        bytes_total: 0,
        bytes_copied: 0,
        attempt_count: 0,
        max_attempts: 0,
        next_attempt_at: 0,
        last_error: [0; 256],
        cancel_requested: false,
        bandwidth_limit_bps: 0,
        window_start_minute: 0,
        window_end_minute: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
        started_at: 0,
        completed_at: 0,
    };
    let mut result: db_storage_migration_result_t =
        db_storage_migration_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut job);
    if result.0 != db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0 {
        set_db_error(res, result);
        return;
    }
    send_json(res, 200 as ::core::ffi::c_int, job_json(&raw mut job));
}
unsafe extern "C" fn handle_job_action(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut retry: bool,
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
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !authorize_storage(req, res, &raw mut user)
        || !extract_job_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res)
    {
        return;
    }
    let mut job: storage_migration_job_t = storage_migration_job_t {
        uuid: [0; 37],
        recording_id: 0,
        owner_user_id: 0,
        operation: [0; 8],
        source_target_uuid: [0; 37],
        source_object_key: [0; 512],
        destination_target_uuid: [0; 37],
        destination_object_key: [0; 512],
        state: [0; 24],
        checksum: [0; 65],
        bytes_total: 0,
        bytes_copied: 0,
        attempt_count: 0,
        max_attempts: 0,
        next_attempt_at: 0,
        last_error: [0; 256],
        cancel_requested: false,
        bandwidth_limit_bps: 0,
        window_start_minute: 0,
        window_end_minute: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
        started_at: 0,
        completed_at: 0,
    };
    let mut result: db_storage_migration_result_t =
        db_storage_migration_result_t(if retry as ::core::ffi::c_int != 0 {
            db_storage_migration_retry(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut job).0
        } else {
            db_storage_migration_request_cancel(
                &raw mut uuid as *mut ::core::ffi::c_char,
                &raw mut job,
            )
            .0
        });
    if result.0 != db_storage_migration_result_t::DB_STORAGE_MIGRATION_OK.0 {
        set_db_error(res, result);
        audit_log_operation(
            req,
            &raw mut user,
            b"storage.configure\0".as_ptr() as *const ::core::ffi::c_char,
            b"storage_migration\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut uuid as *mut ::core::ffi::c_char,
            if retry as ::core::ffi::c_int != 0 {
                b"migration_retry\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"migration_cancel\0".as_ptr() as *const ::core::ffi::c_char
            },
            if result.0 == db_storage_migration_result_t::DB_STORAGE_MIGRATION_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            ::core::ptr::null::<cJSON>(),
        );
        return;
    }
    audit_log_operation(
        req,
        &raw mut user,
        b"storage.configure\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage_migration\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut uuid as *mut ::core::ffi::c_char,
        if retry as ::core::ffi::c_int != 0 {
            b"migration_retry\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"migration_cancel\0".as_ptr() as *const ::core::ffi::c_char
        },
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<cJSON>(),
    );
    if retry as ::core::ffi::c_int != 0 || job.cancel_requested as ::core::ffi::c_int != 0 {
        storage_migration_worker_wake();
    }
    send_json(res, 200 as ::core::ffi::c_int, job_json(&raw mut job));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_storage_migration_cancel(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    handle_job_action(req, res, r#false != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_storage_migration_retry(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    handle_job_action(req, res, r#true != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_storage_archive(
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
    let mut items: *mut cJSON = if !root.is_null() {
        cJSON_AddArrayToObject(
            root,
            b"pending_deletions\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if items.is_null() {
        cJSON_Delete(root);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        root,
        b"s3_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        (LIGHTNVR_ENABLE_S3 != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    let mut read_only: *const ::core::ffi::c_char =
        getenv(b"LIGHTNVR_STORAGE_READ_ONLY\0".as_ptr() as *const ::core::ffi::c_char);
    cJSON_AddBoolToObject(
        root,
        b"recovery_read_only\0".as_ptr() as *const ::core::ffi::c_char,
        (!read_only.is_null()
            && strcmp(read_only, b"1\0".as_ptr() as *const ::core::ffi::c_char) == 0)
            as ::core::ffi::c_int,
    );
    pthread_mutex_lock(mutex);
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT d.uuid,d.recording_id,d.created_at,count(*),MAX(o.attempt_count),MIN(o.next_attempt_at),MAX(o.last_error) FROM storage_deletions d JOIN storage_deletion_objects o ON o.deletion_uuid=d.uuid WHERE o.state<>'completed' GROUP BY d.uuid ORDER BY d.created_at LIMIT 100;\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK {
        loop {
            rc = sqlite3_step(stmt);
            if rc != SQLITE_ROW {
                break;
            }
            let mut item: *mut cJSON = cJSON_CreateObject();
            if item.is_null() {
                rc = SQLITE_NOMEM;
                break;
            } else {
                cJSON_AddStringToObject(
                    item,
                    b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_column_text(stmt, 0 as ::core::ffi::c_int)
                        as *const ::core::ffi::c_char,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"recording_id\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_column_int64(stmt, 1 as ::core::ffi::c_int) as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_column_int64(stmt, 2 as ::core::ffi::c_int) as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"remaining_objects\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_column_int(stmt, 3 as ::core::ffi::c_int) as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"attempts\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_column_int(stmt, 4 as ::core::ffi::c_int) as ::core::ffi::c_double,
                );
                cJSON_AddNumberToObject(
                    item,
                    b"next_attempt_at\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_column_int64(stmt, 5 as ::core::ffi::c_int) as ::core::ffi::c_double,
                );
                cJSON_AddStringToObject(
                    item,
                    b"last_error\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_column_text(stmt, 6 as ::core::ffi::c_int)
                        as *const ::core::ffi::c_char,
                );
                cJSON_AddItemToArray(items, item);
            }
        }
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    let mut ok: bool = rc == SQLITE_DONE;
    stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    rc = sqlite3_prepare_v2(
        db,
        b"SELECT COALESCE((SELECT sum(recording_bytes+replica_bytes) FROM storage_targets WHERE target_type='s3'),0),COALESCE((SELECT sum(bytes_total) FROM storage_migration_jobs WHERE state IN('queued','copying','verifying','committing','retry_wait')),0),COALESCE((SELECT sum(size_bytes) FROM storage_retrieval_jobs WHERE state='ready'),0),(SELECT count(*) FROM storage_retrieval_jobs WHERE state IN('queued','fetching')),(SELECT count(*) FROM storage_deletions WHERE completed_at IS NULL);\0"
            .as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc == SQLITE_OK && sqlite3_step(stmt) == SQLITE_ROW {
        let mut fields: [*const ::core::ffi::c_char; 5] = [
            b"archive_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            b"transfer_backlog_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            b"retrieval_cache_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            b"retrieval_jobs\0".as_ptr() as *const ::core::ffi::c_char,
            b"pending_deletion_count\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 5 as ::core::ffi::c_int {
            cJSON_AddNumberToObject(
                root,
                fields[i as usize],
                sqlite3_column_int64(stmt, i) as ::core::ffi::c_double,
            );
            i += 1;
        }
    } else {
        ok = r#false != 0;
    }
    if !stmt.is_null() {
        sqlite3_finalize(stmt);
    }
    pthread_mutex_unlock(mutex);
    if !ok {
        cJSON_Delete(root);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Unable to read archive operations\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    send_json(res, 200 as ::core::ffi::c_int, root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_storage_archive_retry(
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
    let mut body: *mut cJSON = if !(*req).body.is_null() {
        cJSON_ParseWithLength((*req).body as *const ::core::ffi::c_char, (*req).body_len)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut id: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"deletion_uuid\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsString(id) == 0 || !lightnvr_uuid_is_valid((*id).valuestring) {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"deletion_uuid is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut db: *mut sqlite3 = get_db_handle();
    let mut mutex: *mut pthread_mutex_t = get_db_mutex();
    let mut rc: ::core::ffi::c_int = SQLITE_ERROR;
    let mut changed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !db.is_null() && !mutex.is_null() {
        pthread_mutex_lock(mutex);
        let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        rc = sqlite3_prepare_v2(
            db,
            b"UPDATE storage_deletion_objects SET next_attempt_at=0 WHERE deletion_uuid=? AND state<>'completed';\0"
                .as_ptr() as *const ::core::ffi::c_char,
            -1 as ::core::ffi::c_int,
            &raw mut stmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        if rc == SQLITE_OK {
            sqlite3_bind_text(
                stmt,
                1 as ::core::ffi::c_int,
                (*id).valuestring,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            rc = sqlite3_step(stmt);
            changed = sqlite3_changes(db);
        }
        if !stmt.is_null() {
            sqlite3_finalize(stmt);
        }
        pthread_mutex_unlock(mutex);
    }
    audit_log_operation(
        req,
        &raw mut user,
        b"storage.configure\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage_deletion\0".as_ptr() as *const ::core::ffi::c_char,
        (*id).valuestring,
        b"retry\0".as_ptr() as *const ::core::ffi::c_char,
        if rc == SQLITE_DONE && changed != 0 {
            b"success\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"failure\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::ptr::null::<cJSON>(),
    );
    cJSON_Delete(body);
    if rc != SQLITE_DONE || changed == 0 {
        http_response_set_json_error(
            res,
            if rc == SQLITE_DONE {
                404 as ::core::ffi::c_int
            } else {
                500 as ::core::ffi::c_int
            },
            b"Pending deletion not found or unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    storage_migration_worker_wake();
    http_response_set_json(
        res,
        202 as ::core::ffi::c_int,
        b"{\"status\":\"retry_queued\"}\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
pub const LIGHTNVR_ENABLE_S3: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
