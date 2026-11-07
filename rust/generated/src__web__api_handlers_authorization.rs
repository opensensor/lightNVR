unsafe extern "C" {
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsNull(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateString(string: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_CreateArray() -> *mut cJSON;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_AddItemToObject(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        item: *mut cJSON,
    ) -> cJSON_bool;
    unsafe fn cJSON_Compare(
        a: *const cJSON,
        b: *const cJSON,
        case_sensitive: cJSON_bool,
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
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_get_user_by_id(user_id: int64_t, user: *mut user_t) -> ::core::ffi::c_int;
    unsafe fn authorization_action_catalog(
        action_count: *mut ::core::ffi::c_int,
    ) -> *const authorization_action_metadata_t;
    unsafe fn authorization_action_metadata(
        action: authorization_action_t,
    ) -> *const authorization_action_metadata_t;
    unsafe fn authorization_action_from_key(
        key: *const ::core::ffi::c_char,
    ) -> authorization_action_t;
    unsafe fn authorization_decision_source_name(
        source: authorization_decision_source_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn authorization_action_bit(action: authorization_action_t) -> uint64_t;
    unsafe fn authorization_effective_action_mask(
        user: *const user_t,
        mask: *mut uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn authorization_evaluate(
        user: *const user_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_api_token_create(
        input: *const api_token_create_t,
        token: *mut api_token_t,
        secret: *mut ::core::ffi::c_char,
    ) -> db_api_token_result_t;
    unsafe fn db_api_token_list(
        user_id: int64_t,
        tokens: *mut *mut api_token_t,
        token_count: *mut ::core::ffi::c_int,
    ) -> db_api_token_result_t;
    unsafe fn db_api_token_revoke(
        user_id: int64_t,
        token_uuid: *const ::core::ffi::c_char,
    ) -> db_api_token_result_t;
    unsafe fn db_authorization_load_roles(
        roles: *mut *mut authorization_role_t,
        role_count: *mut ::core::ffi::c_int,
        policy_version: *mut int64_t,
    ) -> db_authorization_result_t;
    unsafe fn db_authorization_role_get(
        uuid: *const ::core::ffi::c_char,
        role: *mut authorization_role_t,
    ) -> db_authorization_result_t;
    unsafe fn db_authorization_role_create(
        role: *mut authorization_role_t,
        expected_version: int64_t,
        new_version: *mut int64_t,
    ) -> db_authorization_result_t;
    unsafe fn db_authorization_role_update(
        role: *const authorization_role_t,
        expected_version: int64_t,
        new_version: *mut int64_t,
    ) -> db_authorization_result_t;
    unsafe fn db_authorization_role_delete(
        uuid: *const ::core::ffi::c_char,
        expected_version: int64_t,
        new_version: *mut int64_t,
    ) -> db_authorization_result_t;
    unsafe fn db_authorization_get_user_policy(
        user_id: int64_t,
        mode: *mut ::core::ffi::c_char,
        grants: *mut *mut authorization_grant_t,
        grant_count: *mut ::core::ffi::c_int,
        policy_version: *mut int64_t,
    ) -> db_authorization_result_t;
    unsafe fn db_authorization_replace_user_policy(
        user_id: int64_t,
        mode: *const ::core::ffi::c_char,
        grants: *const authorization_grant_input_t,
        grant_count: ::core::ffi::c_int,
        expected_version: int64_t,
        new_version: *mut int64_t,
    ) -> db_authorization_result_t;
    unsafe fn db_authorization_load_user_grants(
        user_id: int64_t,
        action_key: *const ::core::ffi::c_char,
        grants: *mut *mut authorization_grant_t,
        grant_count: *mut ::core::ffi::c_int,
        policy_version: *mut int64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fleet_camera_enrich_runtime_health(
        cameras: *mut fleet_camera_t,
        count: ::core::ffi::c_int,
    );
    unsafe fn secure_zero_memory(ptr: *mut ::core::ffi::c_void, size: size_t);
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
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
    unsafe fn audit_log_append(
        req: *const http_request_t,
        user: *const user_t,
        action: *const ::core::ffi::c_char,
        target_type: *const ::core::ffi::c_char,
        target_uuid: *const ::core::ffi::c_char,
        outcome: *const ::core::ffi::c_char,
        details: *const cJSON,
    );
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
    ) -> ::core::ffi::c_int;
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
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
pub type uint64_t = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_camera_tag_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub label: [::core::ffi::c_char; 256],
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
pub struct user_role_t(pub ::core::ffi::c_uint);
impl user_role_t {
    pub const USER_ROLE_ADMIN: Self = Self(0);
    pub const USER_ROLE_USER: Self = Self(1);
    pub const USER_ROLE_VIEWER: Self = Self(2);
    pub const USER_ROLE_API: Self = Self(3);
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
pub struct authorization_decision_t(pub ::core::ffi::c_uint);
impl authorization_decision_t {
    pub const AUTHZ_DECISION_DENY: Self = Self(0);
    pub const AUTHZ_DECISION_ALLOW: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct authorization_decision_source_t(pub ::core::ffi::c_uint);
impl authorization_decision_source_t {
    pub const AUTHZ_SOURCE_NONE: Self = Self(0);
    pub const AUTHZ_SOURCE_LEGACY_ROLE: Self = Self(1);
    pub const AUTHZ_SOURCE_POLICY_GRANT: Self = Self(2);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct grant_authority_result_t(pub ::core::ffi::c_uint);
impl grant_authority_result_t {
    pub const GRANT_AUTHORITY_OK: Self = Self(0);
    pub const GRANT_AUTHORITY_INVALID_ROLE: Self = Self(1);
    pub const GRANT_AUTHORITY_SCOPE_DENIED: Self = Self(2);
    pub const GRANT_AUTHORITY_RESOLUTION_ERROR: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct requester_scope_result_t(pub ::core::ffi::c_uint);
impl requester_scope_result_t {
    pub const REQUESTER_SCOPE_CONTAINS: Self = Self(0);
    pub const REQUESTER_SCOPE_DENIED: Self = Self(1);
    pub const REQUESTER_SCOPE_ERROR: Self = Self(2);
}
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const API_TOKEN_DESCRIPTION_MAX: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const API_TOKEN_SELECTOR_MAX: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const AUTHORIZATION_MAX_USER_GRANTS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
unsafe extern "C" fn action_to_json(
    mut metadata: *const authorization_action_metadata_t,
) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_CreateObject();
    if item.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        item,
        b"key\0".as_ptr() as *const ::core::ffi::c_char,
        (*metadata).key,
    );
    cJSON_AddStringToObject(
        item,
        b"category\0".as_ptr() as *const ::core::ffi::c_char,
        (*metadata).category,
    );
    cJSON_AddStringToObject(
        item,
        b"description\0".as_ptr() as *const ::core::ffi::c_char,
        (*metadata).description,
    );
    cJSON_AddBoolToObject(
        item,
        b"camera_scoped\0".as_ptr() as *const ::core::ffi::c_char,
        (*metadata).camera_scoped as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        item,
        b"destructive\0".as_ptr() as *const ::core::ffi::c_char,
        (*metadata).destructive as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        item,
        b"enforced\0".as_ptr() as *const ::core::ffi::c_char,
        (*metadata).enforced as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        item,
        b"mask_bit\0".as_ptr() as *const ::core::ffi::c_char,
        (*metadata).action.0 as ::core::ffi::c_double,
    );
    return item;
}
unsafe extern "C" fn response_outcome(
    mut res: *const http_response_t,
) -> *const ::core::ffi::c_char {
    return if !res.is_null()
        && (*res).status_code >= 200 as ::core::ffi::c_int
        && (*res).status_code < 300 as ::core::ffi::c_int
    {
        b"success\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"error\0".as_ptr() as *const ::core::ffi::c_char
    };
}
unsafe extern "C" fn set_json_response(mut res: *mut http_response_t, mut json: *mut cJSON) {
    let mut body: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(json);
    cJSON_Delete(json);
    if body.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, body);
    free(body as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn set_json_response_status(
    mut res: *mut http_response_t,
    mut status: ::core::ffi::c_int,
    mut json: *mut cJSON,
) {
    let mut body: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(json);
    cJSON_Delete(json);
    if body.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, status, body);
    free(body as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_authorization_actions(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut requester: user_t = user_t {
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
    let mut requester_evaluation: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t::AUTHZ_DECISION_DENY,
        source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
        policy_version: 0,
        grant_uuid: [0; 37],
        token_uuid: [0; 37],
        role_uuid: [0; 37],
        role_name: [0; 128],
        explanation: [0; 256],
    };
    if httpd_authorize_action(
        req,
        res,
        authorization_action_t::AUTHZ_USERS_MANAGE,
        ::core::ptr::null::<fleet_camera_t>(),
        &raw mut requester,
        &raw mut requester_evaluation,
    ) == 0
    {
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    let mut actions: *mut cJSON = cJSON_CreateArray();
    if response.is_null() || actions.is_null() {
        cJSON_Delete(response);
        cJSON_Delete(actions);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut catalog: *const authorization_action_metadata_t =
        authorization_action_catalog(&raw mut count);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut item: *mut cJSON = action_to_json(catalog.offset(i as isize));
        if item.is_null() {
            cJSON_Delete(actions);
            cJSON_Delete(response);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddItemToArray(actions, item);
        i += 1;
    }
    cJSON_AddItemToObject(
        response,
        b"actions\0".as_ptr() as *const ::core::ffi::c_char,
        actions,
    );
    cJSON_AddNumberToObject(
        response,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        count as ::core::ffi::c_double,
    );
    set_json_response(res, response);
}
unsafe extern "C" fn find_camera(
    mut cameras: *mut fleet_camera_t,
    mut count: ::core::ffi::c_int,
    mut camera_uuid: *const ::core::ffi::c_char,
) -> *mut fleet_camera_t {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if strcasecmp(
            &raw mut (*cameras.offset(i as isize)).camera_uuid as *mut ::core::ffi::c_char,
            camera_uuid,
        ) == 0 as ::core::ffi::c_int
        {
            return cameras.offset(i as isize);
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<fleet_camera_t>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_authorization_simulate(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut requester: user_t = user_t {
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
    let mut requester_evaluation: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t::AUTHZ_DECISION_DENY,
        source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
        policy_version: 0,
        grant_uuid: [0; 37],
        token_uuid: [0; 37],
        role_uuid: [0; 37],
        role_name: [0; 128],
        explanation: [0; 256],
    };
    if httpd_authorize_action(
        req,
        res,
        authorization_action_t::AUTHZ_USERS_MANAGE,
        ::core::ptr::null::<fleet_camera_t>(),
        &raw mut requester,
        &raw mut requester_evaluation,
    ) == 0
    {
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
    let mut user_id: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"user_id\0".as_ptr() as *const ::core::ffi::c_char);
    let mut action_key: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"action\0".as_ptr() as *const ::core::ffi::c_char);
    let mut camera_uuid: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsNumber(user_id) == 0
        || (*user_id).valuedouble != (*user_id).valueint as ::core::ffi::c_double
        || (*user_id).valueint <= 0 as ::core::ffi::c_int
        || cJSON_IsString(action_key) == 0
        || (*action_key).valuestring.is_null()
    {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"user_id and a valid action are required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut action: authorization_action_t =
        authorization_action_from_key((*action_key).valuestring);
    let mut metadata: *const authorization_action_metadata_t =
        authorization_action_metadata(action);
    if metadata.is_null() {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Unknown authorization action\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*metadata).camera_scoped as ::core::ffi::c_int != 0
        && (cJSON_IsString(camera_uuid) == 0
            || (*camera_uuid).valuestring.is_null()
            || *(*camera_uuid).valuestring.offset(0isize) as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int)
    {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"camera_uuid is required for this action\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*metadata).camera_scoped && !camera_uuid.is_null() {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"camera_uuid is not valid for this global action\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
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
    if db_auth_get_user_by_id((*user_id).valuedouble as int64_t, &raw mut user)
        != 0 as ::core::ffi::c_int
    {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"User not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*metadata).camera_scoped {
        if db_fleet_camera_load(&raw mut cameras, &raw mut camera_count) != 0 as ::core::ffi::c_int
        {
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to load camera inventory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        camera = find_camera(cameras, camera_count, (*camera_uuid).valuestring);
        if camera.is_null() {
            free(cameras as *mut ::core::ffi::c_void);
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Camera not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        fleet_camera_enrich_runtime_health(camera, 1 as ::core::ffi::c_int);
    }
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
    let mut result: ::core::ffi::c_int =
        authorization_evaluate(&raw mut user, action, camera, &raw mut evaluation);
    cJSON_Delete(body);
    if result != 0 as ::core::ffi::c_int {
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    let mut principal: *mut cJSON = cJSON_CreateObject();
    let mut resource: *mut cJSON = if (*metadata).camera_scoped as ::core::ffi::c_int != 0 {
        cJSON_CreateObject()
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if response.is_null()
        || principal.is_null()
        || (*metadata).camera_scoped as ::core::ffi::c_int != 0 && resource.is_null()
    {
        cJSON_Delete(response);
        cJSON_Delete(principal);
        cJSON_Delete(resource);
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        response,
        b"allowed\0".as_ptr() as *const ::core::ffi::c_char,
        (evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0)
            as ::core::ffi::c_int,
    );
    cJSON_AddStringToObject(
        response,
        b"action\0".as_ptr() as *const ::core::ffi::c_char,
        (*metadata).key,
    );
    cJSON_AddStringToObject(
        response,
        b"source\0".as_ptr() as *const ::core::ffi::c_char,
        authorization_decision_source_name(evaluation.source),
    );
    cJSON_AddNumberToObject(
        response,
        b"policy_version\0".as_ptr() as *const ::core::ffi::c_char,
        evaluation.policy_version as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        response,
        b"explanation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut evaluation.explanation as *mut ::core::ffi::c_char,
    );
    if evaluation.grant_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            response,
            b"grant_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut evaluation.grant_uuid as *mut ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            response,
            b"grant_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if evaluation.role_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            response,
            b"role_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut evaluation.role_uuid as *mut ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            response,
            b"role_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddStringToObject(
        response,
        b"role\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut evaluation.role_name as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        principal,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        user.id as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        principal,
        b"username\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut user.username as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        principal,
        b"authorization_mode\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut user.authorization_mode as *mut ::core::ffi::c_char,
    );
    cJSON_AddItemToObject(
        response,
        b"principal\0".as_ptr() as *const ::core::ffi::c_char,
        principal,
    );
    if !resource.is_null() {
        cJSON_AddStringToObject(
            resource,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*camera).camera_uuid as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            resource,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*camera).name as *mut ::core::ffi::c_char,
        );
        cJSON_AddItemToObject(
            response,
            b"resource\0".as_ptr() as *const ::core::ffi::c_char,
            resource,
        );
    } else {
        cJSON_AddNullToObject(
            response,
            b"resource\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut audit_details: *mut cJSON = cJSON_CreateObject();
    if !audit_details.is_null() {
        cJSON_AddStringToObject(
            audit_details,
            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
            b"authorization.simulation\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            audit_details,
            b"subject_user_id\0".as_ptr() as *const ::core::ffi::c_char,
            user.id as ::core::ffi::c_double,
        );
        cJSON_AddStringToObject(
            audit_details,
            b"subject_username\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut user.username as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            audit_details,
            b"simulated_action\0".as_ptr() as *const ::core::ffi::c_char,
            (*metadata).key,
        );
        cJSON_AddBoolToObject(
            audit_details,
            b"allowed\0".as_ptr() as *const ::core::ffi::c_char,
            (evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0)
                as ::core::ffi::c_int,
        );
        if !camera.is_null() {
            cJSON_AddStringToObject(
                audit_details,
                b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*camera).camera_uuid as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut subject_id: [::core::ffi::c_char; 32] = [0; 32];
    snprintf(
        &raw mut subject_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
        user.id as ::core::ffi::c_longlong,
    );
    audit_log_append(
        req,
        &raw mut requester,
        b"authorization.simulate\0".as_ptr() as *const ::core::ffi::c_char,
        b"user\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut subject_id as *mut ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        audit_details,
    );
    cJSON_Delete(audit_details);
    free(cameras as *mut ::core::ffi::c_void);
    set_json_response(res, response);
}
unsafe extern "C" fn authorize_policy_manager(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut requester: *mut user_t,
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
    if httpd_authorize_action(
        req,
        res,
        authorization_action_t::AUTHZ_USERS_MANAGE,
        ::core::ptr::null::<fleet_camera_t>(),
        requester,
        &raw mut evaluation,
    ) == 0
    {
        return r#false != 0;
    }
    if (*requester).authenticated_via_scoped_token {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Scoped API tokens cannot mutate authorization policy\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        let mut target: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut prefix: *const ::core::ffi::c_char =
            b"/api/authorization/users/\0".as_ptr() as *const ::core::ffi::c_char;
        if (*req).method.0 == http_method_t::HTTP_METHOD_PUT.0
            && strncmp(
                &raw const (*req).path as *const ::core::ffi::c_char,
                prefix,
                strlen(prefix),
            ) == 0 as ::core::ffi::c_int
        {
            target = (&raw const (*req).path as *const ::core::ffi::c_char)
                .offset(strlen(prefix) as isize);
            if *target.offset(0isize) == 0 || !strchr(target, '/' as ::core::ffi::c_int).is_null() {
                target = ::core::ptr::null::<::core::ffi::c_char>();
            }
        }
        let mut details: *mut cJSON = cJSON_CreateObject();
        if !details.is_null() {
            cJSON_AddStringToObject(
                details,
                b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
                b"authorization.policy_update\0".as_ptr() as *const ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                details,
                b"reason\0".as_ptr() as *const ::core::ffi::c_char,
                b"scoped_token_policy_mutation\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        audit_log_append(
            req,
            requester,
            if (*req).method.0 == http_method_t::HTTP_METHOD_PUT.0 && !target.is_null() {
                b"authorization.policy.update\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"authorization.policy.manage\0".as_ptr() as *const ::core::ffi::c_char
            },
            if !target.is_null() {
                b"user\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"authorization_policy\0".as_ptr() as *const ::core::ffi::c_char
            },
            target,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
            details,
        );
        cJSON_Delete(details);
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn valid_uuid(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null()
        || strlen(value) != (CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int) as size_t
    {
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int {
        let mut c: ::core::ffi::c_uchar = *value.offset(i as isize) as ::core::ffi::c_uchar;
        if i == 8 as ::core::ffi::c_int
            || i == 13 as ::core::ffi::c_int
            || i == 18 as ::core::ffi::c_int
            || i == 23 as ::core::ffi::c_int
        {
            if c as ::core::ffi::c_int != '-' as ::core::ffi::c_int {
                return r#false != 0;
            }
        } else if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISxdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        {
            return r#false != 0;
        }
        i += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn extract_role_uuid(
    mut req: *const http_request_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut res: *mut http_response_t,
) -> bool {
    let mut value: [::core::ffi::c_char; 512] = [0; 512];
    if http_request_extract_path_param(
        req,
        b"/api/authorization/roles/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
        || !strchr(
            &raw mut value as *mut ::core::ffi::c_char,
            '/' as ::core::ffi::c_int,
        )
        .is_null()
        || !valid_uuid(&raw mut value as *mut ::core::ffi::c_char)
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid role UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        uuid,
        &raw mut value as *mut ::core::ffi::c_char,
        CAMERA_UUID_STRING_SIZE as size_t,
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn extract_user_id(
    mut req: *const http_request_t,
    mut user_id: *mut int64_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut value: [::core::ffi::c_char; 512] = [0; 512];
    if http_request_extract_path_param(
        req,
        b"/api/authorization/users/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
        || value[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || !strchr(
            &raw mut value as *mut ::core::ffi::c_char,
            '/' as ::core::ffi::c_int,
        )
        .is_null()
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid user ID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut parsed: ::core::ffi::c_longlong = strtoll(
        &raw mut value as *mut ::core::ffi::c_char,
        &raw mut end,
        10 as ::core::ffi::c_int,
    );
    if *__errno_location() == ERANGE
        || end.is_null()
        || *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || parsed <= 0 as ::core::ffi::c_longlong
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid user ID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *user_id = parsed as int64_t;
    return r#true != 0;
}
unsafe extern "C" fn parse_expected_version(
    mut body: *const cJSON,
    mut version: *mut int64_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"expected_policy_version\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut numeric: ::core::ffi::c_double = if cJSON_IsNumber(item) != 0 {
        (*item).valuedouble
    } else {
        0 as ::core::ffi::c_int as ::core::ffi::c_double
    };
    let mut parsed: int64_t = if numeric >= 1 as ::core::ffi::c_int as ::core::ffi::c_double
        && numeric <= 9007199254740991.0f64
    {
        numeric as int64_t
    } else {
        0 as int64_t
    };
    if cJSON_IsNumber(item) == 0
        || parsed < 1 as int64_t
        || parsed as ::core::ffi::c_double != numeric
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"expected_policy_version must be a positive integer\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *version = parsed;
    return r#true != 0;
}
unsafe extern "C" fn role_to_json(mut role: *const authorization_role_t) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_CreateObject();
    let mut actions: *mut cJSON = cJSON_CreateArray();
    if object.is_null() || actions.is_null() {
        cJSON_Delete(object);
        cJSON_Delete(actions);
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*role).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*role).name as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"description\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*role).description as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        object,
        b"builtin\0".as_ptr() as *const ::core::ffi::c_char,
        (*role).is_builtin as cJSON_bool,
    );
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut catalog: *const authorization_action_metadata_t =
        authorization_action_catalog(&raw mut count);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if (*role).action_mask & (1 as uint64_t) << (*catalog.offset(i as isize)).action.0
            != 0 as uint64_t
        {
            cJSON_AddItemToArray(
                actions,
                cJSON_CreateString((*catalog.offset(i as isize)).key),
            );
        }
        i += 1;
    }
    cJSON_AddItemToObject(
        object,
        b"actions\0".as_ptr() as *const ::core::ffi::c_char,
        actions,
    );
    cJSON_AddNumberToObject(
        object,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*role).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*role).updated_at as ::core::ffi::c_double,
    );
    return object;
}
unsafe extern "C" fn parse_action_mask(
    mut body: *const cJSON,
    mut action_mask: *mut uint64_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut actions: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"actions\0".as_ptr() as *const ::core::ffi::c_char);
    let mut count: ::core::ffi::c_int = if cJSON_IsArray(actions) != 0 {
        cJSON_GetArraySize(actions)
    } else {
        -1 as ::core::ffi::c_int
    };
    if count < 1 as ::core::ffi::c_int || count > authorization_action_t::AUTHZ_ACTION_COUNT.0 {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"actions must be a non-empty action-key array\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut mask: uint64_t = 0 as uint64_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut item: *const cJSON = cJSON_GetArrayItem(actions, i);
        let mut action: authorization_action_t =
            authorization_action_t(if cJSON_IsString(item) != 0 {
                authorization_action_from_key((*item).valuestring).0
            } else {
                authorization_action_t::AUTHZ_ACTION_INVALID.0
            });
        if action.0 == authorization_action_t::AUTHZ_ACTION_INVALID.0
            || mask & (1 as uint64_t) << action.0 != 0 as uint64_t
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"actions contains an unknown or duplicate key\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        mask = (mask as ::core::ffi::c_ulong | (1 as ::core::ffi::c_ulong) << action.0) as uint64_t;
        i += 1;
    }
    *action_mask = mask;
    return r#true != 0;
}
unsafe extern "C" fn contains_non_space(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null() {
        return r#false != 0;
    }
    let mut cursor: *const ::core::ffi::c_uchar = value as *const ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        {
            return r#true != 0;
        }
        cursor = cursor.offset(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn parse_role_body(
    mut body: *const cJSON,
    mut role: *mut authorization_role_t,
    mut expected_version: *mut int64_t,
    mut res: *mut http_response_t,
) -> bool {
    if cJSON_IsObject(body) == 0 || !parse_expected_version(body, expected_version, res) {
        if cJSON_IsObject(body) == 0 {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Request body must be an object\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return r#false != 0;
    }
    let mut name: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"name\0".as_ptr() as *const ::core::ffi::c_char);
    let mut description: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"description\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsString(name) == 0
        || (*name).valuestring.is_null()
        || !contains_non_space((*name).valuestring)
        || strlen((*name).valuestring) >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid role name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !description.is_null()
        && (cJSON_IsString(description) == 0
            || (*description).valuestring.is_null()
            || strlen((*description).valuestring)
                >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>())
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid role description\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        &raw mut (*role).name as *mut ::core::ffi::c_char,
        (*name).valuestring,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*role).description as *mut ::core::ffi::c_char,
        if !description.is_null() {
            (*description).valuestring
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    return parse_action_mask(body, &raw mut (*role).action_mask, res);
}
unsafe extern "C" fn set_authorization_db_error(
    mut res: *mut http_response_t,
    mut result: db_authorization_result_t,
) {
    match result {
        db_authorization_result_t::DB_AUTHORIZATION_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Policy resource not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_authorization_result_t::DB_AUTHORIZATION_CONFLICT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"A role with that name already exists\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_authorization_result_t::DB_AUTHORIZATION_IMMUTABLE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Built-in roles are immutable\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_authorization_result_t::DB_AUTHORIZATION_IN_USE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Role is still referenced by a grant\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_authorization_result_t::DB_AUTHORIZATION_STALE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Policy changed; reload before saving again\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_authorization_result_t::DB_AUTHORIZATION_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid authorization policy request\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Authorization policy operation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_authorization_roles(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut requester: user_t = user_t {
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
    if !authorize_policy_manager(req, res, &raw mut requester) {
        return;
    }
    let mut roles: *mut authorization_role_t = ::core::ptr::null_mut::<authorization_role_t>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut policy_version: int64_t = 0 as int64_t;
    if db_authorization_load_roles(&raw mut roles, &raw mut count, &raw mut policy_version).0
        != db_authorization_result_t::DB_AUTHORIZATION_OK.0
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load roles\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || items.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(roles as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddNumberToObject(
        root,
        b"policy_version\0".as_ptr() as *const ::core::ffi::c_char,
        policy_version as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        count as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"roles\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut item: *mut cJSON = role_to_json(roles.offset(i as isize));
        if item.is_null() {
            cJSON_Delete(root);
            free(roles as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddItemToArray(items, item);
        i += 1;
    }
    free(roles as *mut ::core::ffi::c_void);
    set_json_response(res, root);
}
unsafe extern "C" fn requester_may_delegate(
    mut requester: *const user_t,
    mut requested_mask: uint64_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut held_mask: uint64_t = 0 as uint64_t;
    if authorization_effective_action_mask(requester, &raw mut held_mask) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut escalated: uint64_t = requested_mask & !held_mask;
    if escalated == 0 as uint64_t {
        return r#true != 0;
    }
    let mut detail: [::core::ffi::c_char; 256] = [0; 256];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut detail as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"You cannot grant actions you do not hold:\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut catalog: *const authorization_action_metadata_t =
        authorization_action_catalog(&raw mut count);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count
        && written > 0 as ::core::ffi::c_int
        && written < ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int
    {
        if escalated & authorization_action_bit((*catalog.offset(i as isize)).action)
            != 0 as uint64_t
        {
            written += snprintf(
                (&raw mut detail as *mut ::core::ffi::c_char).offset(written as isize),
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
                    .wrapping_sub(written as size_t),
                b" %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*catalog.offset(i as isize)).key,
            );
        }
        i += 1;
    }
    http_response_set_json_error(
        res,
        403 as ::core::ffi::c_int,
        &raw mut detail as *mut ::core::ffi::c_char,
    );
    return r#false != 0;
}
unsafe extern "C" fn set_role_mutation_response(
    mut res: *mut http_response_t,
    mut status: ::core::ffi::c_int,
    mut uuid: *const ::core::ffi::c_char,
    mut policy_version: int64_t,
) {
    let mut role: authorization_role_t = authorization_role_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 256],
        is_builtin: false,
        action_mask: 0,
        created_at: 0,
        updated_at: 0,
    };
    if db_authorization_role_get(uuid, &raw mut role).0
        != db_authorization_result_t::DB_AUTHORIZATION_OK.0
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to reload role\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut role_json: *mut cJSON = role_to_json(&raw mut role);
    if root.is_null() || role_json.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(role_json);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddNumberToObject(
        root,
        b"policy_version\0".as_ptr() as *const ::core::ffi::c_char,
        policy_version as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"role\0".as_ptr() as *const ::core::ffi::c_char,
        role_json,
    );
    set_json_response_status(res, status, root);
}
unsafe extern "C" fn retains_management_after_role_update(
    mut requester: *const user_t,
    mut proposed_role: *const authorization_role_t,
) -> ::core::ffi::c_int {
    if strcmp(
        &raw const (*requester).authorization_mode as *const ::core::ffi::c_char,
        b"policy\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    let mut mode: [::core::ffi::c_char; 16] = [0; 16];
    let mut grants: *mut authorization_grant_t = ::core::ptr::null_mut::<authorization_grant_t>();
    let mut grant_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut policy_version: int64_t = 0 as int64_t;
    if db_authorization_get_user_policy(
        (*requester).id,
        &raw mut mode as *mut ::core::ffi::c_char,
        &raw mut grants,
        &raw mut grant_count,
        &raw mut policy_version,
    )
    .0 != db_authorization_result_t::DB_AUTHORIZATION_OK.0
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut retained: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < grant_count && !retained {
        if !(!(*grants.offset(i as isize)).enabled
            || strcmp(
                &raw mut (*grants.offset(i as isize)).scope_type as *mut ::core::ffi::c_char,
                b"all\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int)
        {
            let mut action_mask: uint64_t = 0 as uint64_t;
            if strcmp(
                &raw mut (*grants.offset(i as isize)).role_uuid as *mut ::core::ffi::c_char,
                &raw const (*proposed_role).uuid as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                action_mask = (*proposed_role).action_mask;
            } else {
                let mut role: authorization_role_t = authorization_role_t {
                    uuid: [0; 37],
                    name: [0; 128],
                    description: [0; 256],
                    is_builtin: false,
                    action_mask: 0,
                    created_at: 0,
                    updated_at: 0,
                };
                if db_authorization_role_get(
                    &raw mut (*grants.offset(i as isize)).role_uuid as *mut ::core::ffi::c_char,
                    &raw mut role,
                )
                .0 != db_authorization_result_t::DB_AUTHORIZATION_OK.0
                {
                    free(grants as *mut ::core::ffi::c_void);
                    return -1 as ::core::ffi::c_int;
                }
                action_mask = role.action_mask;
            }
            retained = action_mask
                & (1 as uint64_t) << authorization_action_t::AUTHZ_USERS_MANAGE.0
                != 0 as uint64_t;
        }
        i += 1;
    }
    free(grants as *mut ::core::ffi::c_void);
    return if retained as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_authorization_role(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut requester: user_t = user_t {
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
    if !authorize_policy_manager(req, res, &raw mut requester) {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut role: authorization_role_t = authorization_role_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 256],
        is_builtin: false,
        action_mask: 0,
        created_at: 0,
        updated_at: 0,
    };
    memset(
        &raw mut role as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<authorization_role_t>(),
    );
    let mut expected_version: int64_t = 0 as int64_t;
    if !parse_role_body(body, &raw mut role, &raw mut expected_version, res) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    if !requester_may_delegate(&raw mut requester, role.action_mask, res) {
        return;
    }
    let mut new_version: int64_t = 0 as int64_t;
    let mut result: db_authorization_result_t =
        db_authorization_role_create(&raw mut role, expected_version, &raw mut new_version);
    if result.0 != db_authorization_result_t::DB_AUTHORIZATION_OK.0 {
        set_authorization_db_error(res, result);
        return;
    }
    set_role_mutation_response(
        res,
        201 as ::core::ffi::c_int,
        &raw mut role.uuid as *mut ::core::ffi::c_char,
        new_version,
    );
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
            b"authorization.role_create\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut role.name as *mut ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            details,
            b"policy_version\0".as_ptr() as *const ::core::ffi::c_char,
            new_version as ::core::ffi::c_double,
        );
    }
    audit_log_append(
        req,
        &raw mut requester,
        b"authorization.role.create\0".as_ptr() as *const ::core::ffi::c_char,
        b"role\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut role.uuid as *mut ::core::ffi::c_char,
        response_outcome(res),
        details,
    );
    cJSON_Delete(details);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_authorization_role(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut requester: user_t = user_t {
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
    if !authorize_policy_manager(req, res, &raw mut requester) {
        return;
    }
    let mut role: authorization_role_t = authorization_role_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 256],
        is_builtin: false,
        action_mask: 0,
        created_at: 0,
        updated_at: 0,
    };
    memset(
        &raw mut role as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<authorization_role_t>(),
    );
    if !extract_role_uuid(req, &raw mut role.uuid as *mut ::core::ffi::c_char, res) {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut expected_version: int64_t = 0 as int64_t;
    if !parse_role_body(body, &raw mut role, &raw mut expected_version, res) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    let mut retains_management: ::core::ffi::c_int =
        retains_management_after_role_update(&raw mut requester, &raw mut role);
    if retains_management <= 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            if retains_management == 0 as ::core::ffi::c_int {
                409 as ::core::ffi::c_int
            } else {
                500 as ::core::ffi::c_int
            },
            if retains_management == 0 as ::core::ffi::c_int {
                b"This change would remove your own policy-management access\0".as_ptr()
                    as *const ::core::ffi::c_char
            } else {
                b"Failed to verify policy-management access\0".as_ptr()
                    as *const ::core::ffi::c_char
            },
        );
        return;
    }
    if !requester_may_delegate(&raw mut requester, role.action_mask, res) {
        return;
    }
    let mut new_version: int64_t = 0 as int64_t;
    let mut result: db_authorization_result_t =
        db_authorization_role_update(&raw mut role, expected_version, &raw mut new_version);
    if result.0 != db_authorization_result_t::DB_AUTHORIZATION_OK.0 {
        set_authorization_db_error(res, result);
        return;
    }
    set_role_mutation_response(
        res,
        200 as ::core::ffi::c_int,
        &raw mut role.uuid as *mut ::core::ffi::c_char,
        new_version,
    );
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
            b"authorization.role_update\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut role.name as *mut ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            details,
            b"policy_version\0".as_ptr() as *const ::core::ffi::c_char,
            new_version as ::core::ffi::c_double,
        );
    }
    audit_log_append(
        req,
        &raw mut requester,
        b"authorization.role.update\0".as_ptr() as *const ::core::ffi::c_char,
        b"role\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut role.uuid as *mut ::core::ffi::c_char,
        response_outcome(res),
        details,
    );
    cJSON_Delete(details);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_authorization_role(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut requester: user_t = user_t {
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
    if !authorize_policy_manager(req, res, &raw mut requester) {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_role_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res) {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut expected_version: int64_t = 0 as int64_t;
    if cJSON_IsObject(body) == 0 || !parse_expected_version(body, &raw mut expected_version, res) {
        if cJSON_IsObject(body) == 0 {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Request body must be an object\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    let mut new_version: int64_t = 0 as int64_t;
    let mut result: db_authorization_result_t = db_authorization_role_delete(
        &raw mut uuid as *mut ::core::ffi::c_char,
        expected_version,
        &raw mut new_version,
    );
    if result.0 != db_authorization_result_t::DB_AUTHORIZATION_OK.0 {
        set_authorization_db_error(res, result);
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddNumberToObject(
        response,
        b"policy_version\0".as_ptr() as *const ::core::ffi::c_char,
        new_version as ::core::ffi::c_double,
    );
    set_json_response(res, response);
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
            b"authorization.role_delete\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            details,
            b"policy_version\0".as_ptr() as *const ::core::ffi::c_char,
            new_version as ::core::ffi::c_double,
        );
    }
    audit_log_append(
        req,
        &raw mut requester,
        b"authorization.role.delete\0".as_ptr() as *const ::core::ffi::c_char,
        b"role\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut uuid as *mut ::core::ffi::c_char,
        response_outcome(res),
        details,
    );
    cJSON_Delete(details);
}
unsafe extern "C" fn grant_to_json(mut grant: *const authorization_grant_t) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_CreateObject();
    let mut scope: *mut cJSON = cJSON_CreateObject();
    if object.is_null() || scope.is_null() {
        cJSON_Delete(object);
        cJSON_Delete(scope);
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*grant).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"role_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*grant).role_uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"role\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*grant).role_name as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        object,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        (*grant).enabled as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        object,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*grant).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*grant).updated_at as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        scope,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*grant).scope_type as *const ::core::ffi::c_char,
    );
    if strcmp(
        &raw const (*grant).scope_type as *const ::core::ffi::c_char,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut selector: *mut cJSON =
            cJSON_Parse(&raw const (*grant).selector_json as *const ::core::ffi::c_char);
        if selector.is_null() {
            cJSON_Delete(object);
            cJSON_Delete(scope);
            return ::core::ptr::null_mut::<cJSON>();
        }
        cJSON_AddItemToObject(
            scope,
            b"selector\0".as_ptr() as *const ::core::ffi::c_char,
            selector,
        );
    } else {
        cJSON_AddNullToObject(scope, b"selector\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if strcmp(
        &raw const (*grant).scope_type as *const ::core::ffi::c_char,
        b"collection\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        cJSON_AddStringToObject(
            scope,
            b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*grant).collection_uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            scope,
            b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddItemToObject(
        object,
        b"scope\0".as_ptr() as *const ::core::ffi::c_char,
        scope,
    );
    return object;
}
unsafe extern "C" fn set_user_policy_response(
    mut res: *mut http_response_t,
    mut user_id: int64_t,
    mut status: ::core::ffi::c_int,
) {
    let mut mode: [::core::ffi::c_char; 16] = [0; 16];
    let mut grants: *mut authorization_grant_t = ::core::ptr::null_mut::<authorization_grant_t>();
    let mut grant_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut policy_version: int64_t = 0 as int64_t;
    let mut result: db_authorization_result_t = db_authorization_get_user_policy(
        user_id,
        &raw mut mode as *mut ::core::ffi::c_char,
        &raw mut grants,
        &raw mut grant_count,
        &raw mut policy_version,
    );
    if result.0 != db_authorization_result_t::DB_AUTHORIZATION_OK.0 {
        set_authorization_db_error(res, result);
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || items.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(grants as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddNumberToObject(
        root,
        b"user_id\0".as_ptr() as *const ::core::ffi::c_char,
        user_id as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        root,
        b"mode\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut mode as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        root,
        b"policy_version\0".as_ptr() as *const ::core::ffi::c_char,
        policy_version as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"grant_count\0".as_ptr() as *const ::core::ffi::c_char,
        grant_count as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"grants\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < grant_count {
        let mut item: *mut cJSON = grant_to_json(grants.offset(i as isize));
        if item.is_null() {
            cJSON_Delete(root);
            free(grants as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddItemToArray(items, item);
        i += 1;
    }
    free(grants as *mut ::core::ffi::c_void);
    set_json_response_status(res, status, root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_user_authorization(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut requester: user_t = user_t {
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
    if !authorize_policy_manager(req, res, &raw mut requester) {
        return;
    }
    let mut user_id: int64_t = 0 as int64_t;
    if !extract_user_id(req, &raw mut user_id, res) {
        return;
    }
    set_user_policy_response(res, user_id, 200 as ::core::ffi::c_int);
}
unsafe extern "C" fn parse_grants(
    mut body: *const cJSON,
    mut grants_out: *mut *mut authorization_grant_input_t,
    mut grant_count_out: *mut ::core::ffi::c_int,
    mut res: *mut http_response_t,
) -> bool {
    let mut grants: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"grants\0".as_ptr() as *const ::core::ffi::c_char);
    if cJSON_IsArray(grants) == 0 {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"grants must be an array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut count: ::core::ffi::c_int = cJSON_GetArraySize(grants);
    if count > AUTHORIZATION_MAX_USER_GRANTS {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Grant limit exceeded\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut parsed: *mut authorization_grant_input_t = (if count > 0 as ::core::ffi::c_int {
        calloc(
            count as size_t,
            ::core::mem::size_of::<authorization_grant_input_t>(),
        )
    } else {
        NULL
    }) as *mut authorization_grant_input_t;
    if count > 0 as ::core::ffi::c_int && parsed.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut grant: *const cJSON = cJSON_GetArrayItem(grants, i);
        let mut role_uuid: *const cJSON = if cJSON_IsObject(grant) != 0 {
            cJSON_GetObjectItemCaseSensitive(
                grant,
                b"role_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            )
        } else {
            ::core::ptr::null_mut::<cJSON>()
        };
        let mut scope: *const cJSON = if cJSON_IsObject(grant) != 0 {
            cJSON_GetObjectItemCaseSensitive(
                grant,
                b"scope\0".as_ptr() as *const ::core::ffi::c_char,
            )
        } else {
            ::core::ptr::null_mut::<cJSON>()
        };
        let mut r#type: *const cJSON = if cJSON_IsObject(scope) != 0 {
            cJSON_GetObjectItemCaseSensitive(
                scope,
                b"type\0".as_ptr() as *const ::core::ffi::c_char,
            )
        } else {
            ::core::ptr::null_mut::<cJSON>()
        };
        let mut selector: *const cJSON = if cJSON_IsObject(scope) != 0 {
            cJSON_GetObjectItemCaseSensitive(
                scope,
                b"selector\0".as_ptr() as *const ::core::ffi::c_char,
            )
        } else {
            ::core::ptr::null_mut::<cJSON>()
        };
        let mut collection_uuid: *const cJSON = if cJSON_IsObject(scope) != 0 {
            cJSON_GetObjectItemCaseSensitive(
                scope,
                b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            )
        } else {
            ::core::ptr::null_mut::<cJSON>()
        };
        if cJSON_IsString(role_uuid) == 0
            || !valid_uuid((*role_uuid).valuestring)
            || cJSON_IsString(r#type) == 0
            || strcmp(
                (*r#type).valuestring,
                b"all\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
                && strcmp(
                    (*r#type).valuestring,
                    b"selector\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                && strcmp(
                    (*r#type).valuestring,
                    b"collection\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
        {
            free(parsed as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid grant\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*parsed.offset(i as isize)).role_uuid as *mut ::core::ffi::c_char,
            (*role_uuid).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*parsed.offset(i as isize)).scope_type as *mut ::core::ffi::c_char,
            (*r#type).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        if strcmp(
            (*r#type).valuestring,
            b"all\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if !selector.is_null() && cJSON_IsNull(selector) == 0
                || !collection_uuid.is_null() && cJSON_IsNull(collection_uuid) == 0
            {
                free(parsed as *mut ::core::ffi::c_void);
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"All-camera grants cannot include a selector or collection\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return r#false != 0;
            }
        } else if strcmp(
            (*r#type).valuestring,
            b"collection\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if !selector.is_null() && cJSON_IsNull(selector) == 0
                || cJSON_IsString(collection_uuid) == 0
                || !valid_uuid((*collection_uuid).valuestring)
            {
                free(parsed as *mut ::core::ffi::c_void);
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"Collection grants require a valid collection UUID and no selector\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return r#false != 0;
            }
            safe_strcpy(
                &raw mut (*parsed.offset(i as isize)).collection_uuid as *mut ::core::ffi::c_char,
                (*collection_uuid).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
        } else {
            if !collection_uuid.is_null() && cJSON_IsNull(collection_uuid) == 0 {
                free(parsed as *mut ::core::ffi::c_void);
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"Selector grants cannot include a collection\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return r#false != 0;
            }
            if selector.is_null() {
                free(parsed as *mut ::core::ffi::c_void);
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"Selector grants require a selector\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return r#false != 0;
            }
            let mut serialized: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(selector);
            if serialized.is_null()
                || strlen(serialized) >= ::core::mem::size_of::<[::core::ffi::c_char; 8192]>()
            {
                free(serialized as *mut ::core::ffi::c_void);
                free(parsed as *mut ::core::ffi::c_void);
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"Grant selector is too large\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return r#false != 0;
            }
            safe_strcpy(
                &raw mut (*parsed.offset(i as isize)).selector_json as *mut ::core::ffi::c_char,
                serialized,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
                0 as size_t,
            );
            free(serialized as *mut ::core::ffi::c_void);
        }
        i += 1;
    }
    *grants_out = parsed;
    *grant_count_out = count;
    return r#true != 0;
}
unsafe extern "C" fn grants_allow_self_management(
    mut grants: *const authorization_grant_input_t,
    mut grant_count: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < grant_count {
        if strcmp(
            &raw const (*grants.offset(i as isize)).scope_type as *const ::core::ffi::c_char,
            b"all\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            let mut role: authorization_role_t = authorization_role_t {
                uuid: [0; 37],
                name: [0; 128],
                description: [0; 256],
                is_builtin: false,
                action_mask: 0,
                created_at: 0,
                updated_at: 0,
            };
            if db_authorization_role_get(
                &raw const (*grants.offset(i as isize)).role_uuid as *const ::core::ffi::c_char,
                &raw mut role,
            )
            .0 == db_authorization_result_t::DB_AUTHORIZATION_OK.0
            {
                if role.action_mask
                    & (1 as uint64_t) << authorization_action_t::AUTHZ_USERS_MANAGE.0
                    != 0 as uint64_t
                {
                    return r#true != 0;
                }
            }
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn selector_scopes_equal(
    mut left: *const ::core::ffi::c_char,
    mut right: *const ::core::ffi::c_char,
) -> bool {
    if left.is_null()
        || right.is_null()
        || *left.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || *right.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut left_json: *mut cJSON = cJSON_Parse(left);
    let mut right_json: *mut cJSON = cJSON_Parse(right);
    let mut equal: bool = !left_json.is_null()
        && !right_json.is_null()
        && cJSON_Compare(left_json, right_json, r#true) != 0;
    cJSON_Delete(left_json);
    cJSON_Delete(right_json);
    return equal;
}
unsafe extern "C" fn requester_scope_contains(
    mut requester: *const user_t,
    mut action: authorization_action_t,
    mut scope_type: *const ::core::ffi::c_char,
    mut selector_json: *const ::core::ffi::c_char,
    mut collection_uuid: *const ::core::ffi::c_char,
) -> requester_scope_result_t {
    let mut metadata: *const authorization_action_metadata_t =
        authorization_action_metadata(action);
    if metadata.is_null() {
        return requester_scope_result_t::REQUESTER_SCOPE_ERROR;
    }
    if !(*metadata).camera_scoped
        || strcmp(scope_type, b"all\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(
            &raw const (*requester).authorization_mode as *const ::core::ffi::c_char,
            b"policy\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
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
        if authorization_evaluate(
            requester,
            action,
            ::core::ptr::null::<fleet_camera_t>(),
            &raw mut evaluation,
        ) != 0 as ::core::ffi::c_int
        {
            return requester_scope_result_t::REQUESTER_SCOPE_ERROR;
        }
        return requester_scope_result_t(
            (if evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0 {
                requester_scope_result_t::REQUESTER_SCOPE_CONTAINS.0 as ::core::ffi::c_int
            } else {
                requester_scope_result_t::REQUESTER_SCOPE_DENIED.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
    }
    let mut held: *mut authorization_grant_t = ::core::ptr::null_mut::<authorization_grant_t>();
    let mut held_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut policy_version: int64_t = 0 as int64_t;
    if db_authorization_load_user_grants(
        (*requester).id,
        (*metadata).key,
        &raw mut held,
        &raw mut held_count,
        &raw mut policy_version,
    ) != 0 as ::core::ffi::c_int
    {
        return requester_scope_result_t::REQUESTER_SCOPE_ERROR;
    }
    let mut contained: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < held_count && !contained {
        if strcmp(
            &raw mut (*held.offset(i as isize)).scope_type as *mut ::core::ffi::c_char,
            b"all\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            contained = r#true != 0;
        } else if strcmp(
            scope_type,
            b"collection\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && strcmp(
                &raw mut (*held.offset(i as isize)).scope_type as *mut ::core::ffi::c_char,
                b"collection\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            && !collection_uuid.is_null()
            && *collection_uuid.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && strcmp(
                &raw mut (*held.offset(i as isize)).collection_uuid as *mut ::core::ffi::c_char,
                collection_uuid,
            ) == 0 as ::core::ffi::c_int
        {
            contained = r#true != 0;
        } else if strcmp(
            scope_type,
            b"selector\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && strcmp(
                &raw mut (*held.offset(i as isize)).scope_type as *mut ::core::ffi::c_char,
                b"selector\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            && selector_scopes_equal(
                &raw mut (*held.offset(i as isize)).selector_json as *mut ::core::ffi::c_char,
                selector_json,
            ) as ::core::ffi::c_int
                != 0
        {
            contained = r#true != 0;
        }
        i += 1;
    }
    free(held as *mut ::core::ffi::c_void);
    return requester_scope_result_t(
        (if contained as ::core::ffi::c_int != 0 {
            requester_scope_result_t::REQUESTER_SCOPE_CONTAINS.0 as ::core::ffi::c_int
        } else {
            requester_scope_result_t::REQUESTER_SCOPE_DENIED.0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn grants_within_requester_authority(
    mut requester: *const user_t,
    mut grants: *const authorization_grant_input_t,
    mut grant_count: ::core::ffi::c_int,
    mut res: *mut http_response_t,
) -> grant_authority_result_t {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < grant_count {
        let mut role: authorization_role_t = authorization_role_t {
            uuid: [0; 37],
            name: [0; 128],
            description: [0; 256],
            is_builtin: false,
            action_mask: 0,
            created_at: 0,
            updated_at: 0,
        };
        let mut role_result: db_authorization_result_t = db_authorization_role_get(
            &raw const (*grants.offset(i as isize)).role_uuid as *const ::core::ffi::c_char,
            &raw mut role,
        );
        if role_result.0 == db_authorization_result_t::DB_AUTHORIZATION_NOT_FOUND.0 {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Unknown role in grant\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return grant_authority_result_t::GRANT_AUTHORITY_INVALID_ROLE;
        }
        if role_result.0 != db_authorization_result_t::DB_AUTHORIZATION_OK.0 {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to resolve grant authority\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return grant_authority_result_t::GRANT_AUTHORITY_RESOLUTION_ERROR;
        }
        let mut action: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while action < authorization_action_t::AUTHZ_ACTION_COUNT.0 {
            if role.action_mask & authorization_action_bit(authorization_action_t(action))
                != 0 as uint64_t
            {
                let mut scope_result: requester_scope_result_t = requester_scope_contains(
                    requester,
                    authorization_action_t(action),
                    &raw const (*grants.offset(i as isize)).scope_type
                        as *const ::core::ffi::c_char,
                    &raw const (*grants.offset(i as isize)).selector_json
                        as *const ::core::ffi::c_char,
                    &raw const (*grants.offset(i as isize)).collection_uuid
                        as *const ::core::ffi::c_char,
                );
                if scope_result.0 == requester_scope_result_t::REQUESTER_SCOPE_ERROR.0 {
                    http_response_set_json_error(
                        res,
                        500 as ::core::ffi::c_int,
                        b"Failed to resolve grant authority\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return grant_authority_result_t::GRANT_AUTHORITY_RESOLUTION_ERROR;
                }
                if scope_result.0 == requester_scope_result_t::REQUESTER_SCOPE_DENIED.0 {
                    let mut metadata: *const authorization_action_metadata_t =
                        authorization_action_metadata(authorization_action_t(action));
                    let mut message: [::core::ffi::c_char; 256] = [0; 256];
                    snprintf(
                        &raw mut message as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                        b"You cannot grant %s outside your own scope\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        if !metadata.is_null() {
                            (*metadata).key
                        } else {
                            b"this action\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                    http_response_set_json_error(
                        res,
                        403 as ::core::ffi::c_int,
                        &raw mut message as *mut ::core::ffi::c_char,
                    );
                    return grant_authority_result_t::GRANT_AUTHORITY_SCOPE_DENIED;
                }
            }
            action += 1;
        }
        i += 1;
    }
    return grant_authority_result_t::GRANT_AUTHORITY_OK;
}
unsafe extern "C" fn audit_policy_update_failure(
    mut req: *const http_request_t,
    mut requester: *const user_t,
    mut target_user_id: int64_t,
    mut outcome: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
) {
    let mut target: [::core::ffi::c_char; 32] = [
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
    ];
    if target_user_id > 0 as int64_t {
        snprintf(
            &raw mut target as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
            target_user_id as ::core::ffi::c_longlong,
        );
    }
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
            b"authorization.policy_update\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        );
    }
    audit_log_append(
        req,
        requester,
        b"authorization.policy.update\0".as_ptr() as *const ::core::ffi::c_char,
        b"user\0".as_ptr() as *const ::core::ffi::c_char,
        if target[0usize] as ::core::ffi::c_int != 0 {
            &raw mut target as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        outcome,
        details,
    );
    cJSON_Delete(details);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_user_authorization(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut requester: user_t = user_t {
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
    if !authorize_policy_manager(req, res, &raw mut requester) {
        return;
    }
    let mut user_id: int64_t = 0 as int64_t;
    if !extract_user_id(req, &raw mut user_id, res) {
        audit_policy_update_failure(
            req,
            &raw mut requester,
            0 as int64_t,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_target_user\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    if cJSON_IsObject(body) == 0 {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Request body must be an object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_policy_update_failure(
            req,
            &raw mut requester,
            user_id,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request_body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut expected_version: int64_t = 0 as int64_t;
    let mut mode: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"mode\0".as_ptr() as *const ::core::ffi::c_char);
    if !parse_expected_version(body, &raw mut expected_version, res) {
        cJSON_Delete(body);
        audit_policy_update_failure(
            req,
            &raw mut requester,
            user_id,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_policy_version\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if cJSON_IsString(mode) == 0
        || (*mode).valuestring.is_null()
        || strcmp(
            (*mode).valuestring,
            b"policy\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"mode must be policy; legacy authorization is retired\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        cJSON_Delete(body);
        audit_policy_update_failure(
            req,
            &raw mut requester,
            user_id,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_authorization_mode\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut grants: *mut authorization_grant_input_t =
        ::core::ptr::null_mut::<authorization_grant_input_t>();
    let mut grant_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !parse_grants(body, &raw mut grants, &raw mut grant_count, res) {
        cJSON_Delete(body);
        audit_policy_update_failure(
            req,
            &raw mut requester,
            user_id,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_grants\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut authority_result: grant_authority_result_t =
        grants_within_requester_authority(&raw mut requester, grants, grant_count, res);
    if authority_result.0 != grant_authority_result_t::GRANT_AUTHORITY_OK.0 {
        free(grants as *mut ::core::ffi::c_void);
        cJSON_Delete(body);
        audit_policy_update_failure(
            req,
            &raw mut requester,
            user_id,
            if authority_result.0 == grant_authority_result_t::GRANT_AUTHORITY_SCOPE_DENIED.0 {
                b"denied\0".as_ptr() as *const ::core::ffi::c_char
            } else if authority_result.0
                == grant_authority_result_t::GRANT_AUTHORITY_RESOLUTION_ERROR.0
            {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            if authority_result.0 == grant_authority_result_t::GRANT_AUTHORITY_INVALID_ROLE.0 {
                b"invalid_grant_role\0".as_ptr() as *const ::core::ffi::c_char
            } else if authority_result.0
                == grant_authority_result_t::GRANT_AUTHORITY_RESOLUTION_ERROR.0
            {
                b"scope_resolution_failed\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"scope_exceeds_authority\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    if user_id == requester.id && !grants_allow_self_management(grants, grant_count) {
        free(grants as *mut ::core::ffi::c_void);
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            409 as ::core::ffi::c_int,
            b"This change would remove your own policy-management access\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        audit_policy_update_failure(
            req,
            &raw mut requester,
            user_id,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"self_lockout_prevented\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut requested_mode: [::core::ffi::c_char; 16] = [0; 16];
    safe_strcpy(
        &raw mut requested_mode as *mut ::core::ffi::c_char,
        (*mode).valuestring,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    cJSON_Delete(body);
    let mut new_version: int64_t = 0 as int64_t;
    let mut result: db_authorization_result_t = db_authorization_replace_user_policy(
        user_id,
        &raw mut requested_mode as *mut ::core::ffi::c_char,
        grants,
        grant_count,
        expected_version,
        &raw mut new_version,
    );
    free(grants as *mut ::core::ffi::c_void);
    if result.0 != db_authorization_result_t::DB_AUTHORIZATION_OK.0 {
        set_authorization_db_error(res, result);
        audit_policy_update_failure(
            req,
            &raw mut requester,
            user_id,
            if result.0 == db_authorization_result_t::DB_AUTHORIZATION_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            if result.0 == db_authorization_result_t::DB_AUTHORIZATION_STALE.0 {
                b"policy_version_conflict\0".as_ptr() as *const ::core::ffi::c_char
            } else if result.0 == db_authorization_result_t::DB_AUTHORIZATION_NOT_FOUND.0 {
                b"target_not_found\0".as_ptr() as *const ::core::ffi::c_char
            } else if result.0 == db_authorization_result_t::DB_AUTHORIZATION_INVALID.0 {
                b"database_validation_failed\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"database_update_failed\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    set_user_policy_response(res, user_id, 200 as ::core::ffi::c_int);
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
            b"authorization.policy_update\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"mode\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut requested_mode as *mut ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            details,
            b"grant_count\0".as_ptr() as *const ::core::ffi::c_char,
            grant_count as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            details,
            b"policy_version\0".as_ptr() as *const ::core::ffi::c_char,
            new_version as ::core::ffi::c_double,
        );
    }
    let mut target_user_id: [::core::ffi::c_char; 32] = [0; 32];
    snprintf(
        &raw mut target_user_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
    );
    audit_log_append(
        req,
        &raw mut requester,
        b"authorization.policy.update\0".as_ptr() as *const ::core::ffi::c_char,
        b"user\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut target_user_id as *mut ::core::ffi::c_char,
        response_outcome(res),
        details,
    );
    cJSON_Delete(details);
}
unsafe extern "C" fn extract_token_path(
    mut req: *const http_request_t,
    mut user_id: *mut int64_t,
    mut token_uuid: *mut ::core::ffi::c_char,
    mut require_token: bool,
    mut res: *mut http_response_t,
) -> bool {
    let mut value: [::core::ffi::c_char; 512] = [0; 512];
    if http_request_extract_path_param(
        req,
        b"/api/authorization/users/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid token path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut separator: *mut ::core::ffi::c_char = strchr(
        &raw mut value as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if separator.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid token path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *separator = '\0' as ::core::ffi::c_char;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut parsed: ::core::ffi::c_longlong = strtoll(
        &raw mut value as *mut ::core::ffi::c_char,
        &raw mut end,
        10 as ::core::ffi::c_int,
    );
    if *__errno_location() == ERANGE
        || end.is_null()
        || *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || parsed <= 0 as ::core::ffi::c_longlong
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid user ID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *user_id = parsed as int64_t;
    let mut suffix: *const ::core::ffi::c_char = separator.offset(1 as ::core::ffi::c_int as isize);
    if !require_token {
        if strcmp(suffix, b"tokens\0".as_ptr() as *const ::core::ffi::c_char)
            != 0 as ::core::ffi::c_int
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid token path\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        *token_uuid.offset(0isize) = '\0' as ::core::ffi::c_char;
        return r#true != 0;
    }
    if strncmp(
        suffix,
        b"tokens/\0".as_ptr() as *const ::core::ffi::c_char,
        7 as size_t,
    ) != 0 as ::core::ffi::c_int
        || !valid_uuid(suffix.offset(7 as ::core::ffi::c_int as isize))
        || !strchr(
            suffix.offset(7 as ::core::ffi::c_int as isize),
            '/' as ::core::ffi::c_int,
        )
        .is_null()
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid token UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        token_uuid,
        suffix.offset(7 as ::core::ffi::c_int as isize),
        CAMERA_UUID_STRING_SIZE as size_t,
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn audit_token_management_outcome(
    mut req: *const http_request_t,
    mut requester: *const user_t,
    mut action: *const ::core::ffi::c_char,
    mut target_type: *const ::core::ffi::c_char,
    mut target_uuid: *const ::core::ffi::c_char,
    mut owner_user_id: int64_t,
    mut outcome: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
) {
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
            action,
        );
        if !reason.is_null() {
            cJSON_AddStringToObject(
                details,
                b"reason\0".as_ptr() as *const ::core::ffi::c_char,
                reason,
            );
        }
        cJSON_AddNumberToObject(
            details,
            b"owner_user_id\0".as_ptr() as *const ::core::ffi::c_char,
            owner_user_id as ::core::ffi::c_double,
        );
    }
    audit_log_append(
        req,
        requester,
        action,
        target_type,
        target_uuid,
        outcome,
        details,
    );
    cJSON_Delete(details);
}
unsafe extern "C" fn authorize_token_manager(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut target_user_id: int64_t,
    mut requester: *mut user_t,
) -> bool {
    if httpd_check_action_access(req, requester) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut action: *const ::core::ffi::c_char =
        if (*req).method.0 == http_method_t::HTTP_METHOD_POST.0 {
            b"api_token.create\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"api_token.manage\0".as_ptr() as *const ::core::ffi::c_char
        };
    let mut target: [::core::ffi::c_char; 32] = [0; 32];
    snprintf(
        &raw mut target as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
        target_user_id as ::core::ffi::c_longlong,
    );
    if (*requester).authenticated_via_scoped_token {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Scoped API tokens cannot manage API tokens\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_token_management_outcome(
            req,
            requester,
            action,
            b"user\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut target as *mut ::core::ffi::c_char,
            target_user_id,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
            b"scoped_token_management_denied\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if (*requester).id == target_user_id {
        return r#true != 0;
    }
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
    if authorization_evaluate(
        requester,
        authorization_action_t::AUTHZ_USERS_MANAGE,
        ::core::ptr::null::<fleet_camera_t>(),
        &raw mut evaluation,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_token_management_outcome(
            req,
            requester,
            action,
            b"user\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut target as *mut ::core::ffi::c_char,
            target_user_id,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"users_manage_evaluation_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if evaluation.decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0 {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_token_management_outcome(
            req,
            requester,
            action,
            b"user\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut target as *mut ::core::ffi::c_char,
            target_user_id,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
            b"users_manage_denied\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn api_token_to_json(mut token: *const api_token_t) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_CreateObject();
    let mut actions: *mut cJSON = cJSON_CreateArray();
    let mut scope: *mut cJSON = cJSON_CreateObject();
    if object.is_null() || actions.is_null() || scope.is_null() {
        cJSON_Delete(object);
        cJSON_Delete(actions);
        cJSON_Delete(scope);
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*token).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"description\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*token).description as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"prefix\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*token).token_prefix as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        object,
        b"expires_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*token).expires_at as ::core::ffi::c_double,
    );
    if (*token).revoked_at > 0 as int64_t {
        cJSON_AddNumberToObject(
            object,
            b"revoked_at\0".as_ptr() as *const ::core::ffi::c_char,
            (*token).revoked_at as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(
            object,
            b"revoked_at\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*token).last_used_at > 0 as int64_t {
        cJSON_AddNumberToObject(
            object,
            b"last_used_at\0".as_ptr() as *const ::core::ffi::c_char,
            (*token).last_used_at as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(
            object,
            b"last_used_at\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddNumberToObject(
        object,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*token).created_at as ::core::ffi::c_double,
    );
    let mut action_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut catalog: *const authorization_action_metadata_t =
        authorization_action_catalog(&raw mut action_count);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < action_count {
        if (*token).action_mask & (1 as uint64_t) << (*catalog.offset(i as isize)).action.0
            != 0 as uint64_t
        {
            cJSON_AddItemToArray(
                actions,
                cJSON_CreateString((*catalog.offset(i as isize)).key),
            );
        }
        i += 1;
    }
    cJSON_AddItemToObject(
        object,
        b"actions\0".as_ptr() as *const ::core::ffi::c_char,
        actions,
    );
    cJSON_AddStringToObject(
        scope,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*token).scope_type as *const ::core::ffi::c_char,
    );
    if strcmp(
        &raw const (*token).scope_type as *const ::core::ffi::c_char,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut selector: *mut cJSON =
            cJSON_Parse(&raw const (*token).selector_json as *const ::core::ffi::c_char);
        if selector.is_null() {
            cJSON_Delete(object);
            cJSON_Delete(scope);
            return ::core::ptr::null_mut::<cJSON>();
        }
        cJSON_AddItemToObject(
            scope,
            b"selector\0".as_ptr() as *const ::core::ffi::c_char,
            selector,
        );
    } else {
        cJSON_AddNullToObject(scope, b"selector\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if strcmp(
        &raw const (*token).scope_type as *const ::core::ffi::c_char,
        b"collection\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        cJSON_AddStringToObject(
            scope,
            b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*token).collection_uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            scope,
            b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddItemToObject(
        object,
        b"scope\0".as_ptr() as *const ::core::ffi::c_char,
        scope,
    );
    return object;
}
unsafe extern "C" fn set_api_token_error(
    mut res: *mut http_response_t,
    mut result: db_api_token_result_t,
) {
    match result {
        db_api_token_result_t::DB_API_TOKEN_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"API token not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_api_token_result_t::DB_API_TOKEN_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid API token request\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_api_token_result_t::DB_API_TOKEN_LIMIT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Active API token limit reached\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"API token operation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_user_api_tokens(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut user_id: int64_t = 0 as int64_t;
    let mut unused_uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_token_path(
        req,
        &raw mut user_id,
        &raw mut unused_uuid as *mut ::core::ffi::c_char,
        r#false != 0,
        res,
    ) {
        return;
    }
    let mut requester: user_t = user_t {
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
    if !authorize_token_manager(req, res, user_id, &raw mut requester) {
        return;
    }
    let mut target: user_t = user_t {
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
    if db_auth_get_user_by_id(user_id, &raw mut target) != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"User not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut tokens: *mut api_token_t = ::core::ptr::null_mut::<api_token_t>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut result: db_api_token_result_t =
        db_api_token_list(user_id, &raw mut tokens, &raw mut count);
    if result.0 != db_api_token_result_t::DB_API_TOKEN_OK.0 {
        set_api_token_error(res, result);
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || items.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(tokens as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddNumberToObject(
        root,
        b"user_id\0".as_ptr() as *const ::core::ffi::c_char,
        user_id as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        count as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"tokens\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut item: *mut cJSON = api_token_to_json(tokens.offset(i as isize));
        if item.is_null() {
            cJSON_Delete(root);
            free(tokens as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddItemToArray(items, item);
        i += 1;
    }
    free(tokens as *mut ::core::ffi::c_void);
    set_json_response(res, root);
}
unsafe extern "C" fn parse_token_actions(
    mut body: *const cJSON,
    mut action_mask: *mut uint64_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut actions: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"actions\0".as_ptr() as *const ::core::ffi::c_char);
    if cJSON_IsArray(actions) == 0 || cJSON_GetArraySize(actions) == 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"actions must be a non-empty array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *action_mask = 0 as uint64_t;
    let mut item: *const cJSON = ::core::ptr::null::<cJSON>();
    item = if !actions.is_null() {
        (*actions).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !item.is_null() {
        let mut action: authorization_action_t =
            authorization_action_t(if cJSON_IsString(item) != 0 {
                authorization_action_from_key((*item).valuestring).0
            } else {
                authorization_action_t::AUTHZ_ACTION_INVALID.0
            });
        if action.0 == authorization_action_t::AUTHZ_ACTION_INVALID.0 {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Unknown API token action\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        *action_mask = (*action_mask as ::core::ffi::c_ulong
            | (1 as ::core::ffi::c_ulong) << action.0) as uint64_t;
        item = (*item).next;
    }
    return r#true != 0;
}
unsafe extern "C" fn parse_token_scope(
    mut body: *const cJSON,
    mut scope_type: *mut ::core::ffi::c_char,
    mut scope_type_size: size_t,
    mut selector_json: *mut *mut ::core::ffi::c_char,
    mut collection_uuid: *mut ::core::ffi::c_char,
    mut res: *mut http_response_t,
) -> bool {
    let mut scope: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"scope\0".as_ptr() as *const ::core::ffi::c_char);
    let mut r#type: *const cJSON = if cJSON_IsObject(scope) != 0 {
        cJSON_GetObjectItemCaseSensitive(scope, b"type\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut selector: *const cJSON = if cJSON_IsObject(scope) != 0 {
        cJSON_GetObjectItemCaseSensitive(
            scope,
            b"selector\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut collection: *const cJSON = if cJSON_IsObject(scope) != 0 {
        cJSON_GetObjectItemCaseSensitive(
            scope,
            b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if cJSON_IsString(r#type) == 0
        || strcmp(
            (*r#type).valuestring,
            b"all\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            && strcmp(
                (*r#type).valuestring,
                b"selector\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            && strcmp(
                (*r#type).valuestring,
                b"collection\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid API token scope\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        scope_type,
        (*r#type).valuestring,
        scope_type_size,
        0 as size_t,
    );
    *selector_json = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *collection_uuid.offset(0isize) = '\0' as ::core::ffi::c_char;
    if strcmp(
        (*r#type).valuestring,
        b"all\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if !selector.is_null() && cJSON_IsNull(selector) == 0
            || !collection.is_null() && cJSON_IsNull(collection) == 0
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"All-camera token scope cannot include a resource\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        return r#true != 0;
    }
    if strcmp(
        (*r#type).valuestring,
        b"collection\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if !selector.is_null() && cJSON_IsNull(selector) == 0
            || cJSON_IsString(collection) == 0
            || !valid_uuid((*collection).valuestring)
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Collection token scope requires a valid UUID\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            collection_uuid,
            (*collection).valuestring,
            CAMERA_UUID_STRING_SIZE as size_t,
            0 as size_t,
        );
        return r#true != 0;
    }
    if selector.is_null() || !collection.is_null() && cJSON_IsNull(collection) == 0 {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Selector token scope requires only a selector\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *selector_json = cJSON_PrintUnformatted(selector);
    if (*selector_json).is_null() || strlen(*selector_json) >= API_TOKEN_SELECTOR_MAX as size_t {
        free(*selector_json as *mut ::core::ffi::c_void);
        *selector_json = ::core::ptr::null_mut::<::core::ffi::c_char>();
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"API token selector is too large\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn token_scope_within_user(
    mut user: *const user_t,
    mut action_mask: uint64_t,
    mut scope_type: *const ::core::ffi::c_char,
    mut selector_json: *const ::core::ffi::c_char,
    mut collection_uuid: *const ::core::ffi::c_char,
    mut res: *mut http_response_t,
) -> requester_scope_result_t {
    let mut action: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while action < authorization_action_t::AUTHZ_ACTION_COUNT.0 {
        let mut requested: authorization_action_t = authorization_action_t(action);
        if action_mask & authorization_action_bit(requested) != 0 as uint64_t {
            let mut result: requester_scope_result_t = requester_scope_contains(
                user,
                requested,
                scope_type,
                selector_json,
                collection_uuid,
            );
            if result.0 != requester_scope_result_t::REQUESTER_SCOPE_CONTAINS.0 {
                if result.0 == requester_scope_result_t::REQUESTER_SCOPE_ERROR.0 {
                    http_response_set_json_error(
                        res,
                        500 as ::core::ffi::c_int,
                        b"Failed to resolve API token authority\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return requester_scope_result_t::REQUESTER_SCOPE_ERROR;
                }
                let mut metadata: *const authorization_action_metadata_t =
                    authorization_action_metadata(requested);
                let mut message: [::core::ffi::c_char; 256] = [0; 256];
                snprintf(
                    &raw mut message as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    b"API token scope exceeds %s authority\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    if !metadata.is_null() {
                        (*metadata).key
                    } else {
                        b"owner\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                http_response_set_json_error(
                    res,
                    403 as ::core::ffi::c_int,
                    &raw mut message as *mut ::core::ffi::c_char,
                );
                return requester_scope_result_t::REQUESTER_SCOPE_DENIED;
            }
        }
        action += 1;
    }
    return requester_scope_result_t::REQUESTER_SCOPE_CONTAINS;
}
unsafe extern "C" fn audit_api_token_create_failure(
    mut req: *const http_request_t,
    mut requester: *const user_t,
    mut owner_user_id: int64_t,
    mut outcome: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
) {
    let mut target: [::core::ffi::c_char; 32] = [0; 32];
    snprintf(
        &raw mut target as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
        owner_user_id as ::core::ffi::c_longlong,
    );
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
            b"api_token.create\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        );
        cJSON_AddNumberToObject(
            details,
            b"owner_user_id\0".as_ptr() as *const ::core::ffi::c_char,
            owner_user_id as ::core::ffi::c_double,
        );
    }
    audit_log_append(
        req,
        requester,
        b"api_token.create\0".as_ptr() as *const ::core::ffi::c_char,
        b"user\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut target as *mut ::core::ffi::c_char,
        outcome,
        details,
    );
    cJSON_Delete(details);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_user_api_token(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut user_id: int64_t = 0 as int64_t;
    let mut unused_uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_token_path(
        req,
        &raw mut user_id,
        &raw mut unused_uuid as *mut ::core::ffi::c_char,
        r#false != 0,
        res,
    ) {
        return;
    }
    let mut requester: user_t = user_t {
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
    if !authorize_token_manager(req, res, user_id, &raw mut requester) {
        return;
    }
    let mut target: user_t = user_t {
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
    if db_auth_get_user_by_id(user_id, &raw mut target) != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"User not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_api_token_create_failure(
            req,
            &raw mut requester,
            user_id,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"owner_not_found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut description: *const cJSON = if cJSON_IsObject(body) != 0 {
        cJSON_GetObjectItemCaseSensitive(
            body,
            b"description\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut expires_at: *const cJSON = if cJSON_IsObject(body) != 0 {
        cJSON_GetObjectItemCaseSensitive(
            body,
            b"expires_at\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut expiry_number: ::core::ffi::c_double = if cJSON_IsNumber(expires_at) != 0 {
        (*expires_at).valuedouble
    } else {
        0 as ::core::ffi::c_int as ::core::ffi::c_double
    };
    let mut expiry: int64_t = if expiry_number > 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && expiry_number <= 9007199254740991.0f64
    {
        expiry_number as int64_t
    } else {
        0 as int64_t
    };
    let mut action_mask: uint64_t = 0 as uint64_t;
    let mut scope_type: [::core::ffi::c_char; 16] = [0; 16];
    let mut collection_uuid: [::core::ffi::c_char; 37] = [0; 37];
    let mut selector_json: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    if cJSON_IsObject(body) == 0
        || cJSON_IsString(description) == 0
        || (*description).valuestring.is_null()
        || *(*description).valuestring.offset(0isize) as ::core::ffi::c_int
            == '\0' as ::core::ffi::c_int
        || strlen((*description).valuestring) >= API_TOKEN_DESCRIPTION_MAX as size_t
        || expiry <= 0 as int64_t
        || expiry as ::core::ffi::c_double != expiry_number
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid API token request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(body);
        audit_api_token_create_failure(
            req,
            &raw mut requester,
            user_id,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !parse_token_actions(body, &raw mut action_mask, res)
        || !parse_token_scope(
            body,
            &raw mut scope_type as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            &raw mut selector_json,
            &raw mut collection_uuid as *mut ::core::ffi::c_char,
            res,
        )
    {
        free(selector_json as *mut ::core::ffi::c_void);
        cJSON_Delete(body);
        audit_api_token_create_failure(
            req,
            &raw mut requester,
            user_id,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_actions_or_scope\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut scope_result: requester_scope_result_t = token_scope_within_user(
        &raw mut target,
        action_mask,
        &raw mut scope_type as *mut ::core::ffi::c_char,
        selector_json,
        &raw mut collection_uuid as *mut ::core::ffi::c_char,
        res,
    );
    if scope_result.0 == requester_scope_result_t::REQUESTER_SCOPE_CONTAINS.0
        && requester.id != user_id
    {
        scope_result = token_scope_within_user(
            &raw mut requester,
            action_mask,
            &raw mut scope_type as *mut ::core::ffi::c_char,
            selector_json,
            &raw mut collection_uuid as *mut ::core::ffi::c_char,
            res,
        );
    }
    if scope_result.0 != requester_scope_result_t::REQUESTER_SCOPE_CONTAINS.0 {
        free(selector_json as *mut ::core::ffi::c_void);
        cJSON_Delete(body);
        audit_api_token_create_failure(
            req,
            &raw mut requester,
            user_id,
            if scope_result.0 == requester_scope_result_t::REQUESTER_SCOPE_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"denied\0".as_ptr() as *const ::core::ffi::c_char
            },
            if scope_result.0 == requester_scope_result_t::REQUESTER_SCOPE_ERROR.0 {
                b"scope_resolution_failed\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"scope_exceeds_authority\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    let mut input: api_token_create_t = api_token_create_t {
        user_id: user_id,
        created_by_user_id: if requester.id > 0 as int64_t {
            requester.id
        } else {
            user_id
        },
        description: (*description).valuestring,
        action_mask: action_mask,
        scope_type: &raw mut scope_type as *mut ::core::ffi::c_char,
        selector_json: selector_json,
        collection_uuid: if collection_uuid[0usize] as ::core::ffi::c_int != 0 {
            &raw mut collection_uuid as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        expires_at: expiry,
    };
    let mut token: api_token_t = api_token_t {
        uuid: [0; 37],
        user_id: 0,
        created_by_user_id: 0,
        description: [0; 128],
        token_prefix: [0; 16],
        action_mask: 0,
        scope_type: [0; 16],
        selector_json: [0; 8192],
        collection_uuid: [0; 37],
        expires_at: 0,
        revoked_at: 0,
        last_used_at: 0,
        created_at: 0,
    };
    let mut secret: [::core::ffi::c_char; 80] = [0; 80];
    let mut result: db_api_token_result_t = db_api_token_create(
        &raw mut input,
        &raw mut token,
        &raw mut secret as *mut ::core::ffi::c_char,
    );
    free(selector_json as *mut ::core::ffi::c_void);
    cJSON_Delete(body);
    if result.0 != db_api_token_result_t::DB_API_TOKEN_OK.0 {
        set_api_token_error(res, result);
        audit_api_token_create_failure(
            req,
            &raw mut requester,
            user_id,
            if result.0 == db_api_token_result_t::DB_API_TOKEN_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            b"database_rejected_request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut token_json: *mut cJSON = api_token_to_json(&raw mut token);
    let mut secret_json: *mut cJSON =
        cJSON_CreateString(&raw mut secret as *mut ::core::ffi::c_char);
    if root.is_null() || token_json.is_null() || secret_json.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(token_json);
        cJSON_Delete(secret_json);
        db_api_token_revoke(user_id, &raw mut token.uuid as *mut ::core::ffi::c_char);
        secure_zero_memory(
            &raw mut secret as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>(),
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_api_token_create_failure(
            req,
            &raw mut requester,
            user_id,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"response_allocation_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddItemToObject(
        root,
        b"secret\0".as_ptr() as *const ::core::ffi::c_char,
        secret_json,
    );
    cJSON_AddItemToObject(
        root,
        b"token\0".as_ptr() as *const ::core::ffi::c_char,
        token_json,
    );
    let mut response_body: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if response_body.is_null() {
        db_api_token_revoke(user_id, &raw mut token.uuid as *mut ::core::ffi::c_char);
        secure_zero_memory(
            &raw mut secret as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>(),
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_api_token_create_failure(
            req,
            &raw mut requester,
            user_id,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"response_serialization_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if http_response_set_json(res, 201 as ::core::ffi::c_int, response_body)
        != 0 as ::core::ffi::c_int
    {
        db_api_token_revoke(user_id, &raw mut token.uuid as *mut ::core::ffi::c_char);
        secure_zero_memory(
            &raw mut secret as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>(),
        );
        free(response_body as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_api_token_create_failure(
            req,
            &raw mut requester,
            user_id,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"response_write_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut audit_details: *mut cJSON = cJSON_CreateObject();
    if !audit_details.is_null() {
        cJSON_AddStringToObject(
            audit_details,
            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
            b"api_token.create\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            audit_details,
            b"owner_user_id\0".as_ptr() as *const ::core::ffi::c_char,
            user_id as ::core::ffi::c_double,
        );
        cJSON_AddStringToObject(
            audit_details,
            b"description\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut token.description as *mut ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            audit_details,
            b"expires_at\0".as_ptr() as *const ::core::ffi::c_char,
            token.expires_at as ::core::ffi::c_double,
        );
        cJSON_AddStringToObject(
            audit_details,
            b"scope_type\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut token.scope_type as *mut ::core::ffi::c_char,
        );
    }
    audit_log_append(
        req,
        &raw mut requester,
        b"api_token.create\0".as_ptr() as *const ::core::ffi::c_char,
        b"api_token\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut token.uuid as *mut ::core::ffi::c_char,
        response_outcome(res),
        audit_details,
    );
    cJSON_Delete(audit_details);
    secure_zero_memory(
        &raw mut secret as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 80]>(),
    );
    free(response_body as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_user_api_token(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut user_id: int64_t = 0 as int64_t;
    let mut token_uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_token_path(
        req,
        &raw mut user_id,
        &raw mut token_uuid as *mut ::core::ffi::c_char,
        r#true != 0,
        res,
    ) {
        return;
    }
    let mut requester: user_t = user_t {
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
    if !authorize_token_manager(req, res, user_id, &raw mut requester) {
        return;
    }
    let mut result: db_api_token_result_t =
        db_api_token_revoke(user_id, &raw mut token_uuid as *mut ::core::ffi::c_char);
    if result.0 != db_api_token_result_t::DB_API_TOKEN_OK.0 {
        set_api_token_error(res, result);
        audit_token_management_outcome(
            req,
            &raw mut requester,
            b"api_token.revoke\0".as_ptr() as *const ::core::ffi::c_char,
            b"api_token\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut token_uuid as *mut ::core::ffi::c_char,
            user_id,
            if result.0 == db_api_token_result_t::DB_API_TOKEN_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            if result.0 == db_api_token_result_t::DB_API_TOKEN_NOT_FOUND.0 {
                b"token_not_found\0".as_ptr() as *const ::core::ffi::c_char
            } else if result.0 == db_api_token_result_t::DB_API_TOKEN_INVALID.0 {
                b"invalid_token_request\0".as_ptr() as *const ::core::ffi::c_char
            } else if result.0 == db_api_token_result_t::DB_API_TOKEN_ERROR.0 {
                b"database_revoke_failed\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"token_revoke_failed\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    audit_token_management_outcome(
        req,
        &raw mut requester,
        b"api_token.revoke\0".as_ptr() as *const ::core::ffi::c_char,
        b"api_token\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut token_uuid as *mut ::core::ffi::c_char,
        user_id,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    http_response_set_json(
        res,
        200 as ::core::ffi::c_int,
        b"{\"success\":true}\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
