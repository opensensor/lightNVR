unsafe extern "C" {
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
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_HasObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> cJSON_bool;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
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
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn db_storage_target_validate(
        target: *mut storage_target_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> db_storage_target_result_t;
    unsafe fn db_storage_target_count() -> ::core::ffi::c_int;
    unsafe fn db_storage_target_list(
        targets: *mut storage_target_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_storage_target_get(
        uuid: *const ::core::ffi::c_char,
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
    unsafe fn db_storage_target_create(target: *mut storage_target_t)
    -> db_storage_target_result_t;
    unsafe fn db_storage_target_update(
        target: *mut storage_target_t,
        expected_revision: int64_t,
    ) -> db_storage_target_result_t;
    unsafe fn db_storage_target_delete(
        uuid: *const ::core::ffi::c_char,
        expected_revision: int64_t,
    ) -> db_storage_target_result_t;
    unsafe fn storage_target_probe_and_publish(
        uuid: *const ::core::ffi::c_char,
        write_test: bool,
        target: *mut storage_target_t,
    ) -> db_storage_target_result_t;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct storage_target_pressure_t(pub ::core::ffi::c_uint);
impl storage_target_pressure_t {
    pub const STORAGE_TARGET_PRESSURE_UNAVAILABLE: Self = Self(0);
    pub const STORAGE_TARGET_PRESSURE_NORMAL: Self = Self(1);
    pub const STORAGE_TARGET_PRESSURE_HIGH: Self = Self(2);
    pub const STORAGE_TARGET_PRESSURE_RESERVE: Self = Self(3);
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
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const LIGHTNVR_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const STORAGE_TARGET_MAX_COUNT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn storage_target_pressure_evaluate(
    mut capacity_bytes: uint64_t,
    mut available_bytes: uint64_t,
    mut reserve_bytes: uint64_t,
    mut high_watermark_pct: ::core::ffi::c_double,
) -> storage_target_pressure_t {
    if capacity_bytes == 0 as uint64_t || available_bytes > capacity_bytes {
        return storage_target_pressure_t::STORAGE_TARGET_PRESSURE_UNAVAILABLE;
    }
    if available_bytes <= reserve_bytes {
        return storage_target_pressure_t::STORAGE_TARGET_PRESSURE_RESERVE;
    }
    let mut used_pct: ::core::ffi::c_double = 100.0f64
        * capacity_bytes.wrapping_sub(available_bytes) as ::core::ffi::c_double
        / capacity_bytes as ::core::ffi::c_double;
    return storage_target_pressure_t(
        (if used_pct >= high_watermark_pct {
            storage_target_pressure_t::STORAGE_TARGET_PRESSURE_HIGH.0 as ::core::ffi::c_int
        } else {
            storage_target_pressure_t::STORAGE_TARGET_PRESSURE_NORMAL.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
}
#[inline]
unsafe extern "C" fn storage_target_cleanup_goal_bytes(
    mut capacity_bytes: uint64_t,
    mut reserve_bytes: uint64_t,
    mut low_watermark_pct: ::core::ffi::c_double,
) -> uint64_t {
    if capacity_bytes == 0 as uint64_t {
        return reserve_bytes;
    }
    let mut free_pct: ::core::ffi::c_double = 100.0f64 - low_watermark_pct;
    if free_pct < 0.0f64 {
        free_pct = 0.0f64;
    }
    if free_pct > 100.0f64 {
        free_pct = 100.0f64;
    }
    let mut watermark_goal: uint64_t =
        (capacity_bytes as ::core::ffi::c_double * free_pct / 100.0f64) as uint64_t;
    return if reserve_bytes > watermark_goal {
        reserve_bytes
    } else {
        watermark_goal
    };
}
#[inline]
unsafe extern "C" fn storage_target_pressure_name(
    mut pressure: storage_target_pressure_t,
) -> *const ::core::ffi::c_char {
    match pressure {
        storage_target_pressure_t::STORAGE_TARGET_PRESSURE_NORMAL => {
            return b"normal\0".as_ptr() as *const ::core::ffi::c_char;
        }
        storage_target_pressure_t::STORAGE_TARGET_PRESSURE_HIGH => {
            return b"high\0".as_ptr() as *const ::core::ffi::c_char;
        }
        storage_target_pressure_t::STORAGE_TARGET_PRESSURE_RESERVE => {
            return b"reserve\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
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
unsafe extern "C" fn set_json_response(
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
            b"Failed to serialize storage target response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    http_response_set_json(res, status, body);
    free(body as *mut ::core::ffi::c_void);
    return r#true != 0;
}
unsafe extern "C" fn target_to_json(
    mut target: *const storage_target_t,
    mut duplicate_filesystem: bool,
) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_CreateObject();
    let mut health: *mut cJSON = cJSON_CreateObject();
    if object.is_null() || health.is_null() {
        cJSON_Delete(object);
        cJSON_Delete(health);
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).name as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"target_type\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).target_type as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"root_path\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).root_path as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"endpoint\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).endpoint as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"region\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).region as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"bucket\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).bucket as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"credential_ref\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).credential_ref as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        object,
        b"archive_budget_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).archive_budget_bytes as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        object,
        b"archive_only\0".as_ptr() as *const ::core::ffi::c_char,
        (strcmp(
            &raw const (*target).target_type as *const ::core::ffi::c_char,
            b"s3\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    cJSON_AddBoolToObject(
        object,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).enabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        object,
        b"is_default\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).is_default as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        object,
        b"mount_required\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).mount_required as cJSON_bool,
    );
    if (*target).mount_guard_path[0usize] != 0 {
        cJSON_AddStringToObject(
            object,
            b"mount_guard_path\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*target).mount_guard_path as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            object,
            b"mount_guard_path\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddStringToObject(
        object,
        b"storage_class\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).storage_class as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        object,
        b"reserve_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).reserve_bytes as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"high_watermark_pct\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).high_watermark_pct,
    );
    cJSON_AddNumberToObject(
        object,
        b"low_watermark_pct\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).low_watermark_pct,
    );
    cJSON_AddNumberToObject(
        object,
        b"recording_count\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).recording_count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"recording_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).recording_bytes as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"replica_count\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).replica_count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"replica_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).replica_bytes as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"migration_bandwidth_bps\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).migration_bandwidth_bps as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"archival_window_start_minute\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).archival_window_start_minute as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"archival_window_end_minute\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).archival_window_end_minute as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).revision as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).updated_at as ::core::ffi::c_double,
    );
    let mut used: uint64_t = if (*target).capacity_bytes > (*target).available_bytes {
        (*target)
            .capacity_bytes
            .wrapping_sub((*target).available_bytes)
    } else {
        0 as uint64_t
    };
    let mut used_pct: ::core::ffi::c_double = if (*target).capacity_bytes > 0 as uint64_t {
        100.0f64 * used as ::core::ffi::c_double / (*target).capacity_bytes as ::core::ffi::c_double
    } else {
        0.0f64
    };
    let mut pressure: storage_target_pressure_t = storage_target_pressure_t(
        if !(*target).enabled
            || strcmp(
                &raw const (*target).health_status as *const ::core::ffi::c_char,
                b"unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            storage_target_pressure_t::STORAGE_TARGET_PRESSURE_UNAVAILABLE.0
        } else {
            storage_target_pressure_evaluate(
                (*target).capacity_bytes,
                (*target).available_bytes,
                (*target).reserve_bytes,
                (*target).high_watermark_pct,
            )
            .0
        },
    );
    cJSON_AddStringToObject(
        health,
        b"status\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).health_status as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        health,
        b"capacity_known\0".as_ptr() as *const ::core::ffi::c_char,
        (strcmp(
            &raw const (*target).target_type as *const ::core::ffi::c_char,
            b"s3\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    cJSON_AddNumberToObject(
        health,
        b"capacity_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).capacity_bytes as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        health,
        b"available_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).available_bytes as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        health,
        b"used_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        used as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        health,
        b"used_pct\0".as_ptr() as *const ::core::ffi::c_char,
        used_pct,
    );
    cJSON_AddStringToObject(
        health,
        b"pressure\0".as_ptr() as *const ::core::ffi::c_char,
        storage_target_pressure_name(pressure),
    );
    cJSON_AddNumberToObject(
        health,
        b"cleanup_target_bytes\0".as_ptr() as *const ::core::ffi::c_char,
        storage_target_cleanup_goal_bytes(
            (*target).capacity_bytes,
            (*target).reserve_bytes,
            (*target).low_watermark_pct,
        ) as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        health,
        b"filesystem_device\0".as_ptr() as *const ::core::ffi::c_char,
        (*target).filesystem_device as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        health,
        b"duplicate_filesystem\0".as_ptr() as *const ::core::ffi::c_char,
        duplicate_filesystem as cJSON_bool,
    );
    if (*target).last_probe_at > 0 as int64_t {
        cJSON_AddNumberToObject(
            health,
            b"last_probe_at\0".as_ptr() as *const ::core::ffi::c_char,
            (*target).last_probe_at as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(
            health,
            b"last_probe_at\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*target).last_success_at > 0 as int64_t {
        cJSON_AddNumberToObject(
            health,
            b"last_success_at\0".as_ptr() as *const ::core::ffi::c_char,
            (*target).last_success_at as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(
            health,
            b"last_success_at\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddStringToObject(
        health,
        b"last_error\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*target).last_error as *const ::core::ffi::c_char,
    );
    cJSON_AddItemToObject(
        object,
        b"health\0".as_ptr() as *const ::core::ffi::c_char,
        health,
    );
    return object;
}
unsafe extern "C" fn set_db_error(
    mut res: *mut http_response_t,
    mut result: db_storage_target_result_t,
    mut validation_error: *const ::core::ffi::c_char,
) {
    match result {
        db_storage_target_result_t::DB_STORAGE_TARGET_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Storage target not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_storage_target_result_t::DB_STORAGE_TARGET_CONFLICT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"A storage target already uses that name or root path\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_storage_target_result_t::DB_STORAGE_TARGET_STALE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Storage target was changed by another administrator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_storage_target_result_t::DB_STORAGE_TARGET_LIMIT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Storage target limit reached\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_storage_target_result_t::DB_STORAGE_TARGET_IN_USE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Default or nonempty storage targets cannot be removed or repointed\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_storage_target_result_t::DB_STORAGE_TARGET_UNAVAILABLE => {
            http_response_set_json_error(
                res,
                422 as ::core::ffi::c_int,
                b"Storage target is unavailable or failed its write probe\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_storage_target_result_t::DB_STORAGE_TARGET_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                if !validation_error.is_null()
                    && *validation_error.offset(0isize) as ::core::ffi::c_int != 0
                {
                    validation_error
                } else {
                    b"Invalid storage target\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Storage target operation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn extract_target_uuid(
    mut req: *const http_request_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut res: *mut http_response_t,
) -> bool {
    let mut value: [::core::ffi::c_char; 512] = [0; 512];
    if http_request_extract_path_param(
        req,
        b"/api/storage-targets/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid storage target path\0".as_ptr() as *const ::core::ffi::c_char,
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
            b"Invalid storage target UUID\0".as_ptr() as *const ::core::ffi::c_char,
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
unsafe extern "C" fn json_string(
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
unsafe extern "C" fn json_bool(
    mut body: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut destination: *mut bool,
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
    if cJSON_IsBool(item) == 0 {
        let mut message_0: [::core::ffi::c_char; 160] = [0; 160];
        snprintf(
            &raw mut message_0 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
            b"%s must be boolean\0".as_ptr() as *const ::core::ffi::c_char,
            key,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            &raw mut message_0 as *mut ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *destination = cJSON_IsTrue(item) != 0;
    return r#true != 0;
}
unsafe extern "C" fn json_number(
    mut body: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut destination: *mut ::core::ffi::c_double,
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
    if cJSON_IsNumber(item) == 0 || (*item).valuedouble.is_finite() as ::core::ffi::c_int == 0 {
        let mut message_0: [::core::ffi::c_char; 160] = [0; 160];
        snprintf(
            &raw mut message_0 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
            b"%s must be a finite number\0".as_ptr() as *const ::core::ffi::c_char,
            key,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            &raw mut message_0 as *mut ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *destination = (*item).valuedouble;
    return r#true != 0;
}
unsafe extern "C" fn apply_body(
    mut body: *const cJSON,
    mut target: *mut storage_target_t,
    mut create: bool,
    mut revision: *mut int64_t,
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
    if !json_string(
        body,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*target).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        create,
        res,
    ) || !json_string(
        body,
        b"target_type\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*target).target_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        r#false != 0,
        res,
    ) || !json_string(
        body,
        b"endpoint\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*target).endpoint as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        r#false != 0,
        res,
    ) || !json_string(
        body,
        b"region\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*target).region as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        r#false != 0,
        res,
    ) || !json_string(
        body,
        b"bucket\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*target).bucket as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        r#false != 0,
        res,
    ) || !json_string(
        body,
        b"credential_ref\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*target).credential_ref as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        r#false != 0,
        res,
    ) || !json_string(
        body,
        b"root_path\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*target).root_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        create,
        res,
    ) || !json_string(
        body,
        b"storage_class\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*target).storage_class as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        r#false != 0,
        res,
    ) || !json_bool(
        body,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*target).enabled,
        r#false != 0,
        res,
    ) || !json_bool(
        body,
        b"mount_required\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*target).mount_required,
        r#false != 0,
        res,
    ) {
        return r#false != 0;
    }
    let mut number: ::core::ffi::c_double = 0.0f64;
    if cJSON_HasObjectItem(
        body,
        b"archive_budget_bytes\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0
    {
        if !json_number(
            body,
            b"archive_budget_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut number,
            r#true != 0,
            res,
        ) || number < 0 as ::core::ffi::c_int as ::core::ffi::c_double
            || number > 9007199254740991.0f64
            || floor(number) != number
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"archive_budget_bytes must be a nonnegative safe integer\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*target).archive_budget_bytes = number as uint64_t;
    }
    if cJSON_HasObjectItem(
        body,
        b"reserve_bytes\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0
    {
        if !json_number(
            body,
            b"reserve_bytes\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut number,
            r#true != 0,
            res,
        ) || number < 0.0f64
            || number > INT64_MAX as ::core::ffi::c_double
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"reserve_bytes is out of range\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*target).reserve_bytes = number as uint64_t;
    }
    if cJSON_HasObjectItem(
        body,
        b"high_watermark_pct\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0
        && !json_number(
            body,
            b"high_watermark_pct\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*target).high_watermark_pct,
            r#true != 0,
            res,
        )
    {
        return r#false != 0;
    }
    if cJSON_HasObjectItem(
        body,
        b"low_watermark_pct\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0
        && !json_number(
            body,
            b"low_watermark_pct\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*target).low_watermark_pct,
            r#true != 0,
            res,
        )
    {
        return r#false != 0;
    }
    if cJSON_HasObjectItem(
        body,
        b"migration_bandwidth_bps\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0
    {
        if !json_number(
            body,
            b"migration_bandwidth_bps\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut number,
            r#true != 0,
            res,
        ) || number < 0.0f64
            || number > INT64_MAX as ::core::ffi::c_double
            || floor(number) != number
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"migration_bandwidth_bps must be a nonnegative integer\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*target).migration_bandwidth_bps = number as uint64_t;
    }
    if cJSON_HasObjectItem(
        body,
        b"archival_window_start_minute\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0
    {
        if !json_number(
            body,
            b"archival_window_start_minute\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut number,
            r#true != 0,
            res,
        ) || number < 0.0f64
            || number > 1439.0f64
            || floor(number) != number
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"archival_window_start_minute must be 0 through 1439\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*target).archival_window_start_minute = number as ::core::ffi::c_int;
    }
    if cJSON_HasObjectItem(
        body,
        b"archival_window_end_minute\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0
    {
        if !json_number(
            body,
            b"archival_window_end_minute\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut number,
            r#true != 0,
            res,
        ) || number < 0.0f64
            || number > 1439.0f64
            || floor(number) != number
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"archival_window_end_minute must be 0 through 1439\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*target).archival_window_end_minute = number as ::core::ffi::c_int;
    }
    if !create {
        if !json_number(
            body,
            b"revision\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut number,
            r#true != 0,
            res,
        ) || number < 1.0f64
            || floor(number) != number
            || number > INT64_MAX as ::core::ffi::c_double
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"revision must be a positive integer\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        *revision = number as int64_t;
    }
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
    let mut result: db_storage_target_result_t = db_storage_target_validate(
        target,
        &raw mut validation_error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        set_db_error(
            res,
            result,
            &raw mut validation_error as *mut ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn audit_target(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut uuid: *const ::core::ffi::c_char,
    mut operation: *const ::core::ffi::c_char,
    mut outcome: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
) {
    let mut context: *mut cJSON = cJSON_CreateObject();
    if !context.is_null() && !reason.is_null() {
        cJSON_AddStringToObject(
            context,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        );
    }
    audit_log_operation(
        req,
        user,
        b"storage.configure\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage_target\0".as_ptr() as *const ::core::ffi::c_char,
        uuid,
        operation,
        outcome,
        context,
    );
    cJSON_Delete(context);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_storage_targets(
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
    let mut total: ::core::ffi::c_int = db_storage_target_count();
    if total < 0 as ::core::ffi::c_int || total > STORAGE_TARGET_MAX_COUNT {
        set_db_error(
            res,
            db_storage_target_result_t::DB_STORAGE_TARGET_ERROR,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return;
    }
    let mut targets: *mut storage_target_t = (if total > 0 as ::core::ffi::c_int {
        calloc(total as size_t, ::core::mem::size_of::<storage_target_t>())
    } else {
        NULL
    }) as *mut storage_target_t;
    if total > 0 as ::core::ffi::c_int && targets.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int = if total == 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        db_storage_target_list(targets, total)
    };
    if count < 0 as ::core::ffi::c_int {
        free(targets as *mut ::core::ffi::c_void);
        set_db_error(
            res,
            db_storage_target_result_t::DB_STORAGE_TARGET_ERROR,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || items.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(targets as *mut ::core::ffi::c_void);
        set_db_error(
            res,
            db_storage_target_result_t::DB_STORAGE_TARGET_ERROR,
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
        b"targets\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut duplicate: bool = r#false != 0;
        if (*targets.offset(index as isize)).filesystem_device != 0 as uint64_t {
            let mut other: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while other < count {
                if other != index
                    && (*targets.offset(other as isize)).filesystem_device
                        == (*targets.offset(index as isize)).filesystem_device
                {
                    duplicate = r#true != 0;
                    break;
                } else {
                    other += 1;
                }
            }
        }
        let mut item: *mut cJSON = target_to_json(targets.offset(index as isize), duplicate);
        if item.is_null() {
            cJSON_Delete(root);
            free(targets as *mut ::core::ffi::c_void);
            set_db_error(
                res,
                db_storage_target_result_t::DB_STORAGE_TARGET_ERROR,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            return;
        }
        cJSON_AddItemToArray(items, item);
        index += 1;
    }
    free(targets as *mut ::core::ffi::c_void);
    set_json_response(res, 200 as ::core::ffi::c_int, root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_storage_target(
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
    memset(
        &raw mut target as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_target_t>(),
    );
    safe_strcpy(
        &raw mut target.target_type as *mut ::core::ffi::c_char,
        b"filesystem\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut target.storage_class as *mut ::core::ffi::c_char,
        b"hot\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    target.enabled = r#true != 0;
    target.high_watermark_pct = 90.0f64;
    target.low_watermark_pct = 80.0f64;
    let mut unused_revision: int64_t = 0 as int64_t;
    if !apply_body(
        body,
        &raw mut target,
        r#true != 0,
        &raw mut unused_revision,
        res,
    ) {
        cJSON_Delete(body);
        audit_target(
            req,
            &raw mut user,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"target_create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_Delete(body);
    let mut result: db_storage_target_result_t = db_storage_target_create(&raw mut target);
    if result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        set_db_error(
            res,
            result,
            &raw mut target.last_error as *mut ::core::ffi::c_char,
        );
        audit_target(
            req,
            &raw mut user,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"target_create\0".as_ptr() as *const ::core::ffi::c_char,
            if result.0 == db_storage_target_result_t::DB_STORAGE_TARGET_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            b"persistence_or_probe_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_target(
        req,
        &raw mut user,
        &raw mut target.uuid as *mut ::core::ffi::c_char,
        b"target_create\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"created\0".as_ptr() as *const ::core::ffi::c_char,
    );
    set_json_response(
        res,
        201 as ::core::ffi::c_int,
        target_to_json(&raw mut target, r#false != 0),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_storage_target(
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
    if !extract_target_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res) {
        return;
    }
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
    let mut result: db_storage_target_result_t =
        db_storage_target_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut target);
    if result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        return;
    }
    set_json_response(
        res,
        200 as ::core::ffi::c_int,
        target_to_json(&raw mut target, r#false != 0),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_storage_target(
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
    if !extract_target_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res) {
        return;
    }
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
    let mut result: db_storage_target_result_t =
        db_storage_target_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut target);
    if result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut revision: int64_t = 0 as int64_t;
    if !apply_body(body, &raw mut target, r#false != 0, &raw mut revision, res) {
        cJSON_Delete(body);
        audit_target(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"target_update\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_Delete(body);
    result = db_storage_target_update(&raw mut target, revision);
    if result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        set_db_error(
            res,
            result,
            &raw mut target.last_error as *mut ::core::ffi::c_char,
        );
        audit_target(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"target_update\0".as_ptr() as *const ::core::ffi::c_char,
            if result.0 == db_storage_target_result_t::DB_STORAGE_TARGET_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            b"update_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_target(
        req,
        &raw mut user,
        &raw mut uuid as *mut ::core::ffi::c_char,
        b"target_update\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"updated\0".as_ptr() as *const ::core::ffi::c_char,
    );
    set_json_response(
        res,
        200 as ::core::ffi::c_int,
        target_to_json(&raw mut target, r#false != 0),
    );
}
unsafe extern "C" fn parse_revision_query(
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
pub unsafe extern "C" fn handle_delete_storage_target(
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
    if !extract_target_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res)
        || !parse_revision_query(req, &raw mut revision, res)
    {
        return;
    }
    let mut result: db_storage_target_result_t =
        db_storage_target_delete(&raw mut uuid as *mut ::core::ffi::c_char, revision);
    if result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        audit_target(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"target_delete\0".as_ptr() as *const ::core::ffi::c_char,
            if result.0 == db_storage_target_result_t::DB_STORAGE_TARGET_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            b"delete_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_target(
        req,
        &raw mut user,
        &raw mut uuid as *mut ::core::ffi::c_char,
        b"target_delete\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"deleted\0".as_ptr() as *const ::core::ffi::c_char,
    );
    http_response_set_json(
        res,
        200 as ::core::ffi::c_int,
        b"{\"success\":true}\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_storage_target_probe(
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
    if !extract_target_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res) {
        return;
    }
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
    memset(
        &raw mut target as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_target_t>(),
    );
    let mut result: db_storage_target_result_t = storage_target_probe_and_publish(
        &raw mut uuid as *mut ::core::ffi::c_char,
        r#true != 0,
        &raw mut target,
    );
    if result.0 != db_storage_target_result_t::DB_STORAGE_TARGET_OK.0 {
        set_db_error(
            res,
            result,
            if target.last_error[0usize] as ::core::ffi::c_int != 0 {
                &raw mut target.last_error as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
        );
        audit_target(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"target_probe\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"probe_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_target(
        req,
        &raw mut user,
        &raw mut uuid as *mut ::core::ffi::c_char,
        b"target_probe\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"healthy\0".as_ptr() as *const ::core::ffi::c_char,
    );
    set_json_response(
        res,
        200 as ::core::ffi::c_int,
        target_to_json(&raw mut target, r#false != 0),
    );
}
