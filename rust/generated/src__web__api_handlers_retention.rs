unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
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
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
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
    unsafe fn authorization_filter_cameras(
        user: *const user_t,
        action: authorization_action_t,
        cameras: *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_authorize_stream_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_evaluate_camera_identity_action(
        user: *const user_t,
        action: authorization_action_t,
        camera_uuid: *const ::core::ffi::c_char,
        legacy_stream_name: *const ::core::ffi::c_char,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_authorize_camera_identity_action_with_context(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        camera_uuid: *const ::core::ffi::c_char,
        legacy_stream_name: *const ::core::ffi::c_char,
        user: *mut user_t,
        camera: *mut fleet_camera_t,
        evaluation: *mut authorization_evaluation_t,
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
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn get_stream_retention_config(
        stream_name: *const ::core::ffi::c_char,
        config: *mut stream_retention_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn set_stream_retention_config(
        stream_name: *const ::core::ffi::c_char,
        config: *const stream_retention_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_recording_metadata_by_id(
        id: uint64_t,
        metadata: *mut recording_metadata_t,
    ) -> ::core::ffi::c_int;
    unsafe fn set_recording_protected(id: uint64_t, protected: bool) -> ::core::ffi::c_int;
    unsafe fn set_recording_retention_override(
        id: uint64_t,
        days: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn get_protected_recordings_count_for_cameras(
        camera_uuids: *const *const ::core::ffi::c_char,
        camera_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
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
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_retention_config_t {
    pub retention_days: ::core::ffi::c_int,
    pub detection_retention_days: ::core::ffi::c_int,
    pub max_storage_mb: uint64_t,
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
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
pub struct recording_metadata_t {
    pub id: uint64_t,
    pub stream_name: [::core::ffi::c_char; 64],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub file_path: [::core::ffi::c_char; 512],
    pub storage_target_uuid: [::core::ffi::c_char; 37],
    pub object_key: [::core::ffi::c_char; 512],
    pub placement_reason: [::core::ffi::c_char; 64],
    pub storage_policy_version: int64_t,
    pub start_time: time_t,
    pub end_time: time_t,
    pub size_bytes: uint64_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub fps: ::core::ffi::c_int,
    pub codec: [::core::ffi::c_char; 16],
    pub is_complete: bool,
    pub playback_issue: [::core::ffi::c_char; 32],
    pub trigger_type: [::core::ffi::c_char; 16],
    pub protected: bool,
    pub retention_override_days: ::core::ffi::c_int,
    pub retention_tier: ::core::ffi::c_int,
    pub disk_pressure_eligible: bool,
    pub schedule_restricted: ::core::ffi::c_int,
}
pub const LOG_COMPONENT: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"RecordingsAPI\0") };
unsafe extern "C" fn authorize_recording_action(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut id: uint64_t,
    mut action: authorization_action_t,
    mut recording: *mut recording_metadata_t,
    mut user: *mut user_t,
    mut camera: *mut fleet_camera_t,
    mut evaluation: *mut authorization_evaluation_t,
) -> ::core::ffi::c_int {
    memset(
        recording as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<recording_metadata_t>(),
    );
    if get_recording_metadata_by_id(id, recording) != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Recording not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    return httpd_authorize_camera_identity_action_with_context(
        req,
        res,
        action,
        &raw mut (*recording).camera_uuid as *mut ::core::ffi::c_char,
        &raw mut (*recording).stream_name as *mut ::core::ffi::c_char,
        user,
        camera,
        evaluation,
    );
}
unsafe extern "C" fn audit_recording_policy_operation(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut camera: *const fleet_camera_t,
    mut recording_id: uint64_t,
    mut operation: *const ::core::ffi::c_char,
    mut outcome: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
    mut value: ::core::ffi::c_int,
    mut value_name: *const ::core::ffi::c_char,
) {
    let mut recording_uuid: [::core::ffi::c_char; 32] = [0; 32];
    snprintf(
        &raw mut recording_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%llu\0".as_ptr() as *const ::core::ffi::c_char,
        recording_id as ::core::ffi::c_ulonglong,
    );
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*camera).camera_uuid as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        );
        if !value_name.is_null() {
            cJSON_AddNumberToObject(details, value_name, value as ::core::ffi::c_double);
        }
    }
    audit_log_operation(
        req,
        user,
        b"evidence.protect\0".as_ptr() as *const ::core::ffi::c_char,
        b"recording\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut recording_uuid as *mut ::core::ffi::c_char,
        operation,
        outcome,
        details,
    );
    cJSON_Delete(details);
}
unsafe extern "C" fn audit_batch_policy_operation(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut protected: bool,
    mut requested_count: ::core::ffi::c_int,
    mut success_count: ::core::ffi::c_int,
    mut fail_count: ::core::ffi::c_int,
    mut outcome: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
) {
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        );
        cJSON_AddBoolToObject(
            details,
            b"protected\0".as_ptr() as *const ::core::ffi::c_char,
            protected as cJSON_bool,
        );
        cJSON_AddNumberToObject(
            details,
            b"requested_count\0".as_ptr() as *const ::core::ffi::c_char,
            requested_count as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            details,
            b"success_count\0".as_ptr() as *const ::core::ffi::c_char,
            success_count as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            details,
            b"fail_count\0".as_ptr() as *const ::core::ffi::c_char,
            fail_count as ::core::ffi::c_double,
        );
    }
    audit_log_operation(
        req,
        user,
        b"evidence.protect\0".as_ptr() as *const ::core::ffi::c_char,
        b"recording_batch\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        if protected as ::core::ffi::c_int != 0 {
            b"batch_protect\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"batch_unprotect\0".as_ptr() as *const ::core::ffi::c_char
        },
        outcome,
        details,
    );
    cJSON_Delete(details);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_stream_retention(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/streams/:name/retention request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut stream_name: [::core::ffi::c_char; 256] = [
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
    if http_request_extract_path_param(
        req,
        b"/api/streams/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name in URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut suffix: *mut ::core::ffi::c_char = strstr(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        b"/retention\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !suffix.is_null() {
        *suffix = '\0' as ::core::ffi::c_char;
    }
    if httpd_authorize_stream_action(
        req,
        res,
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    ) == 0
    {
        return;
    }
    let mut config: stream_retention_config_t = stream_retention_config_t {
        retention_days: 0,
        detection_retention_days: 0,
        max_storage_mb: 0,
    };
    if get_stream_retention_config(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut config,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found or failed to get retention config\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut json: *mut cJSON = cJSON_CreateObject();
    cJSON_AddStringToObject(
        json,
        b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        json,
        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        config.retention_days as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        json,
        b"detection_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        config.detection_retention_days as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        json,
        b"max_storage_mb\0".as_ptr() as *const ::core::ffi::c_char,
        config.max_storage_mb as ::core::ffi::c_double,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(json);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(json);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_stream_retention(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling PUT /api/streams/:name/retention request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut stream_name: [::core::ffi::c_char; 256] = [
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
    if http_request_extract_path_param(
        req,
        b"/api/streams/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name in URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut suffix: *mut ::core::ffi::c_char = strstr(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        b"/retention\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !suffix.is_null() {
        *suffix = '\0' as ::core::ffi::c_char;
    }
    if httpd_authorize_stream_action(
        req,
        res,
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    ) == 0
    {
        return;
    }
    let mut json: *mut cJSON = httpd_parse_json_body(req);
    if json.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON in request body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut config: stream_retention_config_t = stream_retention_config_t {
        retention_days: 0,
        detection_retention_days: 0,
        max_storage_mb: 0,
    };
    if get_stream_retention_config(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut config,
    ) != 0 as ::core::ffi::c_int
    {
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut fields: [*const ::core::ffi::c_char; 3] = [
        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        b"detection_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        b"max_storage_mb\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut valid: bool = cJSON_IsObject(json) != 0;
    let mut i: size_t = 0 as size_t;
    while valid as ::core::ffi::c_int != 0
        && i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        let mut value: *const cJSON = cJSON_GetObjectItemCaseSensitive(json, fields[i]);
        if !value.is_null() {
            let minimum: ::core::ffi::c_double = if i == 2 as size_t { 0.0f64 } else { -1.0f64 };
            let maximum: ::core::ffi::c_double = if i == 2 as size_t {
                9007199254740991.0f64
            } else {
                365.0f64
            };
            valid = cJSON_IsNumber(value) != 0
                && (*value).valuedouble.is_finite() as ::core::ffi::c_int != 0
                && floor((*value).valuedouble) == (*value).valuedouble
                && (*value).valuedouble >= minimum
                && (*value).valuedouble <= maximum;
        }
        i = i.wrapping_add(1);
    }
    if !valid {
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid retention configuration values\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut retention_days: *mut cJSON = cJSON_GetObjectItem(
        json,
        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !retention_days.is_null() && cJSON_IsNumber(retention_days) != 0 {
        if (*retention_days).valueint < -1 as ::core::ffi::c_int
            || (*retention_days).valueint > 365 as ::core::ffi::c_int
        {
            cJSON_Delete(json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"retention_days must be between -1 and 365\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        config.retention_days = (*retention_days).valueint;
    }
    let mut detection_retention_days: *mut cJSON = cJSON_GetObjectItem(
        json,
        b"detection_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !detection_retention_days.is_null() && cJSON_IsNumber(detection_retention_days) != 0 {
        if (*detection_retention_days).valueint < -1 as ::core::ffi::c_int
            || (*detection_retention_days).valueint > 365 as ::core::ffi::c_int
        {
            cJSON_Delete(json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"detection_retention_days must be between -1 and 365\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        config.detection_retention_days = (*detection_retention_days).valueint;
    }
    let mut max_storage_mb: *mut cJSON = cJSON_GetObjectItem(
        json,
        b"max_storage_mb\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !max_storage_mb.is_null() && cJSON_IsNumber(max_storage_mb) != 0 {
        config.max_storage_mb = (*max_storage_mb).valuedouble as uint64_t;
    }
    cJSON_Delete(json);
    if set_stream_retention_config(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut config,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to save retention config\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddStringToObject(
        response,
        b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        response,
        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        config.retention_days as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        response,
        b"detection_retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        config.detection_retention_days as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        response,
        b"max_storage_mb\0".as_ptr() as *const ::core::ffi::c_char,
        config.max_storage_mb as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"Retention config updated successfully\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Updated retention config for stream %s: retention=%d, detection_retention=%d, max_storage=%lu MB\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        config.retention_days,
        config.detection_retention_days,
        config.max_storage_mb as ::core::ffi::c_ulong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_recording_protect(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling PUT /api/recordings/:id/protect request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut id_str: [::core::ffi::c_char; 32] = [
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
    if http_request_extract_path_param(
        req,
        b"/api/recordings/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut id_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid recording ID in URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut suffix: *mut ::core::ffi::c_char = strstr(
        &raw mut id_str as *mut ::core::ffi::c_char,
        b"/protect\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !suffix.is_null() {
        *suffix = '\0' as ::core::ffi::c_char;
    }
    let mut id: uint64_t = strtoull(
        &raw mut id_str as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        10 as ::core::ffi::c_int,
    ) as uint64_t;
    if id == 0 as uint64_t {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid recording ID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut json: *mut cJSON = httpd_parse_json_body(req);
    if json.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON in request body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut protected_json: *mut cJSON =
        cJSON_GetObjectItem(json, b"protected\0".as_ptr() as *const ::core::ffi::c_char);
    if protected_json.is_null() || cJSON_IsBool(protected_json) == 0 {
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing or invalid 'protected' field (boolean required)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut protected: bool = cJSON_IsTrue(protected_json) != 0;
    cJSON_Delete(json);
    let mut recording: recording_metadata_t = recording_metadata_t {
        id: 0,
        stream_name: [0; 64],
        camera_uuid: [0; 37],
        file_path: [0; 512],
        storage_target_uuid: [0; 37],
        object_key: [0; 512],
        placement_reason: [0; 64],
        storage_policy_version: 0,
        start_time: 0,
        end_time: 0,
        size_bytes: 0,
        width: 0,
        height: 0,
        fps: 0,
        codec: [0; 16],
        is_complete: false,
        playback_issue: [0; 32],
        trigger_type: [0; 16],
        protected: false,
        retention_override_days: 0,
        retention_tier: 0,
        disk_pressure_eligible: false,
        schedule_restricted: 0,
    };
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
    let mut camera: fleet_camera_t = fleet_camera_t {
        camera_uuid: [0; 37],
        name: [0; 256],
        address: [0; 512],
        legacy_tags: [0; 256],
        location_uuid: [0; 37],
        location_name: [0; 128],
        location_path: [0; 1024],
        location_ancestor_uuids: [[0; 37]; 32],
        location_depth: 0,
        tags: [fleet_camera_tag_t {
            uuid: [0; 37],
            label: [0; 256],
        }; 64],
        tag_count: 0,
        manufacturer: [0; 128],
        model: [0; 128],
        enabled: false,
        streaming_enabled: false,
        record: false,
        detection_based_recording: false,
        is_onvif: false,
        ptz_enabled: false,
        backchannel_enabled: false,
        health: fleet_health_state_t::FLEET_HEALTH_UNKNOWN,
        availability: fleet_availability_state_t::FLEET_AVAILABILITY_LIVE,
        health_changed_at: 0,
        first_video_at: 0,
        last_frame_ts: 0,
        last_recording_at: 0,
        current_fps: 0.,
        recording_active: false,
    };
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
    if authorize_recording_action(
        req,
        res,
        id,
        authorization_action_t::AUTHZ_EVIDENCE_PROTECT,
        &raw mut recording,
        &raw mut user,
        &raw mut camera,
        &raw mut evaluation,
    ) == 0
    {
        return;
    }
    if set_recording_protected(id, protected) != 0 as ::core::ffi::c_int {
        audit_recording_policy_operation(
            req,
            &raw mut user,
            &raw mut camera,
            id,
            if protected as ::core::ffi::c_int != 0 {
                b"protect\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"unprotect\0".as_ptr() as *const ::core::ffi::c_char
            },
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"database_update_failed\0".as_ptr() as *const ::core::ffi::c_char,
            protected as ::core::ffi::c_int,
            b"protected\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to update recording protection status\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_recording_policy_operation(
        req,
        &raw mut user,
        &raw mut camera,
        id,
        if protected as ::core::ffi::c_int != 0 {
            b"protect\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"unprotect\0".as_ptr() as *const ::core::ffi::c_char
        },
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"completed\0".as_ptr() as *const ::core::ffi::c_char,
        protected as ::core::ffi::c_int,
        b"protected\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddNumberToObject(
        response,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        id as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        response,
        b"protected\0".as_ptr() as *const ::core::ffi::c_char,
        protected as cJSON_bool,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        if protected as ::core::ffi::c_int != 0 {
            b"Recording protected\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"Recording unprotected\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Recording %llu protection set to %s\0".as_ptr() as *const ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
        if protected as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_recording_retention(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling PUT /api/recordings/:id/retention request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut id_str: [::core::ffi::c_char; 32] = [
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
    if http_request_extract_path_param(
        req,
        b"/api/recordings/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut id_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid recording ID in URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut suffix: *mut ::core::ffi::c_char = strstr(
        &raw mut id_str as *mut ::core::ffi::c_char,
        b"/retention\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !suffix.is_null() {
        *suffix = '\0' as ::core::ffi::c_char;
    }
    let mut id: uint64_t = strtoull(
        &raw mut id_str as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        10 as ::core::ffi::c_int,
    ) as uint64_t;
    if id == 0 as uint64_t {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid recording ID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut json: *mut cJSON = httpd_parse_json_body(req);
    if json.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON in request body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut days_json: *mut cJSON = cJSON_GetObjectItem(
        json,
        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if days_json.is_null() || cJSON_IsNumber(days_json) == 0 {
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing or invalid 'retention_days' field (number required, -1 to remove override)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut days: ::core::ffi::c_int = (*days_json).valueint;
    cJSON_Delete(json);
    let mut recording: recording_metadata_t = recording_metadata_t {
        id: 0,
        stream_name: [0; 64],
        camera_uuid: [0; 37],
        file_path: [0; 512],
        storage_target_uuid: [0; 37],
        object_key: [0; 512],
        placement_reason: [0; 64],
        storage_policy_version: 0,
        start_time: 0,
        end_time: 0,
        size_bytes: 0,
        width: 0,
        height: 0,
        fps: 0,
        codec: [0; 16],
        is_complete: false,
        playback_issue: [0; 32],
        trigger_type: [0; 16],
        protected: false,
        retention_override_days: 0,
        retention_tier: 0,
        disk_pressure_eligible: false,
        schedule_restricted: 0,
    };
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
    let mut camera: fleet_camera_t = fleet_camera_t {
        camera_uuid: [0; 37],
        name: [0; 256],
        address: [0; 512],
        legacy_tags: [0; 256],
        location_uuid: [0; 37],
        location_name: [0; 128],
        location_path: [0; 1024],
        location_ancestor_uuids: [[0; 37]; 32],
        location_depth: 0,
        tags: [fleet_camera_tag_t {
            uuid: [0; 37],
            label: [0; 256],
        }; 64],
        tag_count: 0,
        manufacturer: [0; 128],
        model: [0; 128],
        enabled: false,
        streaming_enabled: false,
        record: false,
        detection_based_recording: false,
        is_onvif: false,
        ptz_enabled: false,
        backchannel_enabled: false,
        health: fleet_health_state_t::FLEET_HEALTH_UNKNOWN,
        availability: fleet_availability_state_t::FLEET_AVAILABILITY_LIVE,
        health_changed_at: 0,
        first_video_at: 0,
        last_frame_ts: 0,
        last_recording_at: 0,
        current_fps: 0.,
        recording_active: false,
    };
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
    if authorize_recording_action(
        req,
        res,
        id,
        authorization_action_t::AUTHZ_EVIDENCE_PROTECT,
        &raw mut recording,
        &raw mut user,
        &raw mut camera,
        &raw mut evaluation,
    ) == 0
    {
        return;
    }
    if set_recording_retention_override(id, days) != 0 as ::core::ffi::c_int {
        audit_recording_policy_operation(
            req,
            &raw mut user,
            &raw mut camera,
            id,
            if days < 0 as ::core::ffi::c_int {
                b"clear_retention_override\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"set_retention_override\0".as_ptr() as *const ::core::ffi::c_char
            },
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"database_update_failed\0".as_ptr() as *const ::core::ffi::c_char,
            days,
            b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to update recording retention override\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_recording_policy_operation(
        req,
        &raw mut user,
        &raw mut camera,
        id,
        if days < 0 as ::core::ffi::c_int {
            b"clear_retention_override\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"set_retention_override\0".as_ptr() as *const ::core::ffi::c_char
        },
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"completed\0".as_ptr() as *const ::core::ffi::c_char,
        days,
        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddNumberToObject(
        response,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        id as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        response,
        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        days as ::core::ffi::c_double,
    );
    if days < 0 as ::core::ffi::c_int {
        cJSON_AddStringToObject(
            response,
            b"message\0".as_ptr() as *const ::core::ffi::c_char,
            b"Retention override removed, using stream default\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddStringToObject(
            response,
            b"message\0".as_ptr() as *const ::core::ffi::c_char,
            b"Custom retention set\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Recording %llu retention override set to %d days\0".as_ptr()
            as *const ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
        days,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_protected_recordings(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/recordings/protected request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut stream_name: [::core::ffi::c_char; 64] = [
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
    http_request_get_query_param(
        req,
        b"stream\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    let mut count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if stream_name[0usize] != 0 {
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
        let mut camera: fleet_camera_t = fleet_camera_t {
            camera_uuid: [0; 37],
            name: [0; 256],
            address: [0; 512],
            legacy_tags: [0; 256],
            location_uuid: [0; 37],
            location_name: [0; 128],
            location_path: [0; 1024],
            location_ancestor_uuids: [[0; 37]; 32],
            location_depth: 0,
            tags: [fleet_camera_tag_t {
                uuid: [0; 37],
                label: [0; 256],
            }; 64],
            tag_count: 0,
            manufacturer: [0; 128],
            model: [0; 128],
            enabled: false,
            streaming_enabled: false,
            record: false,
            detection_based_recording: false,
            is_onvif: false,
            ptz_enabled: false,
            backchannel_enabled: false,
            health: fleet_health_state_t::FLEET_HEALTH_UNKNOWN,
            availability: fleet_availability_state_t::FLEET_AVAILABILITY_LIVE,
            health_changed_at: 0,
            first_video_at: 0,
            last_frame_ts: 0,
            last_recording_at: 0,
            current_fps: 0.,
            recording_active: false,
        };
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
        if httpd_authorize_camera_identity_action_with_context(
            req,
            res,
            authorization_action_t::AUTHZ_RECORDINGS_REPLAY,
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw mut stream_name as *mut ::core::ffi::c_char,
            &raw mut user,
            &raw mut camera,
            &raw mut evaluation,
        ) == 0
        {
            return;
        }
        let mut uuids: [*const ::core::ffi::c_char; 1] = [&raw mut camera.camera_uuid
            as *mut ::core::ffi::c_char
            as *const ::core::ffi::c_char];
        count = get_protected_recordings_count_for_cameras(
            &raw mut uuids as *mut *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
    } else {
        let mut user_0: user_t = user_t {
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
        if httpd_check_action_access(req, &raw mut user_0) == 0 {
            http_response_set_json_error(
                res,
                401 as ::core::ffi::c_int,
                b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
        let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if db_fleet_camera_load(&raw mut cameras, &raw mut camera_count) != 0 as ::core::ffi::c_int
            || authorization_filter_cameras(
                &raw mut user_0,
                authorization_action_t::AUTHZ_RECORDINGS_REPLAY,
                cameras,
                &raw mut camera_count,
            ) != 0 as ::core::ffi::c_int
        {
            free(cameras as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut names: *mut *const ::core::ffi::c_char =
            (if camera_count > 0 as ::core::ffi::c_int {
                calloc(
                    camera_count as size_t,
                    ::core::mem::size_of::<*const ::core::ffi::c_char>(),
                )
            } else {
                NULL
            }) as *mut *const ::core::ffi::c_char;
        if camera_count > 0 as ::core::ffi::c_int && names.is_null() {
            free(cameras as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < camera_count {
            *names.offset(i as isize) =
                &raw mut (*cameras.offset(i as isize)).camera_uuid as *mut ::core::ffi::c_char;
            i += 1;
        }
        count = get_protected_recordings_count_for_cameras(names, camera_count);
        free(names as *mut ::core::ffi::c_void);
        free(cameras as *mut ::core::ffi::c_void);
    }
    if count < 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get protected recordings count\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddNumberToObject(
        response,
        b"protected_count\0".as_ptr() as *const ::core::ffi::c_char,
        count as ::core::ffi::c_double,
    );
    if stream_name[0usize] != 0 {
        cJSON_AddStringToObject(
            response,
            b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_batch_protect_recordings(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/recordings/batch-protect request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut json: *mut cJSON = httpd_parse_json_body(req);
    if json.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON in request body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut ids_json: *mut cJSON =
        cJSON_GetObjectItem(json, b"ids\0".as_ptr() as *const ::core::ffi::c_char);
    if ids_json.is_null() || cJSON_IsArray(ids_json) == 0 {
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing or invalid 'ids' field (array required)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut protected_json: *mut cJSON =
        cJSON_GetObjectItem(json, b"protected\0".as_ptr() as *const ::core::ffi::c_char);
    if protected_json.is_null() || cJSON_IsBool(protected_json) == 0 {
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing or invalid 'protected' field (boolean required)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut protected: bool = cJSON_IsTrue(protected_json) != 0;
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
    if httpd_check_action_access(req, &raw mut user) == 0 {
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut item_count: ::core::ffi::c_int = cJSON_GetArraySize(ids_json);
    let mut authorized_ids: *mut uint64_t = (if item_count > 0 as ::core::ffi::c_int {
        calloc(item_count as size_t, ::core::mem::size_of::<uint64_t>())
    } else {
        NULL
    }) as *mut uint64_t;
    if item_count > 0 as ::core::ffi::c_int && authorized_ids.is_null() {
        audit_batch_policy_operation(
            req,
            &raw mut user,
            protected,
            item_count,
            0 as ::core::ffi::c_int,
            item_count,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"allocation_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to authorize batch\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut authorized_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut id_item: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    id_item = (if !ids_json.is_null() {
        (*ids_json).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    }) as *mut cJSON;
    while !id_item.is_null() {
        if cJSON_IsNumber(id_item) == 0
            || (*id_item).valuedouble <= 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            fail_count += 1;
        } else {
            let mut id: uint64_t = (*id_item).valuedouble as uint64_t;
            let mut recording: recording_metadata_t = recording_metadata_t {
                id: 0,
                stream_name: [0; 64],
                camera_uuid: [0; 37],
                file_path: [0; 512],
                storage_target_uuid: [0; 37],
                object_key: [0; 512],
                placement_reason: [0; 64],
                storage_policy_version: 0,
                start_time: 0,
                end_time: 0,
                size_bytes: 0,
                width: 0,
                height: 0,
                fps: 0,
                codec: [0; 16],
                is_complete: false,
                playback_issue: [0; 32],
                trigger_type: [0; 16],
                protected: false,
                retention_override_days: 0,
                retention_tier: 0,
                disk_pressure_eligible: false,
                schedule_restricted: 0,
            };
            if get_recording_metadata_by_id(id, &raw mut recording) != 0 as ::core::ffi::c_int {
                fail_count += 1;
            } else {
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
                let mut result: ::core::ffi::c_int = httpd_evaluate_camera_identity_action(
                    &raw mut user,
                    authorization_action_t::AUTHZ_EVIDENCE_PROTECT,
                    &raw mut recording.camera_uuid as *mut ::core::ffi::c_char,
                    &raw mut recording.stream_name as *mut ::core::ffi::c_char,
                    &raw mut evaluation,
                );
                if result < 0 as ::core::ffi::c_int {
                    audit_batch_policy_operation(
                        req,
                        &raw mut user,
                        protected,
                        item_count,
                        0 as ::core::ffi::c_int,
                        fail_count,
                        b"error\0".as_ptr() as *const ::core::ffi::c_char,
                        b"authorization_evaluation_failed\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    free(authorized_ids as *mut ::core::ffi::c_void);
                    cJSON_Delete(json);
                    http_response_set_json_error(
                        res,
                        500 as ::core::ffi::c_int,
                        b"Authorization policy evaluation failed\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return;
                }
                if result > 0 as ::core::ffi::c_int
                    || evaluation.decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0
                {
                    audit_batch_policy_operation(
                        req,
                        &raw mut user,
                        protected,
                        item_count,
                        0 as ::core::ffi::c_int,
                        fail_count,
                        b"failure\0".as_ptr() as *const ::core::ffi::c_char,
                        b"unauthorized_member\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    free(authorized_ids as *mut ::core::ffi::c_void);
                    cJSON_Delete(json);
                    http_response_set_json_error(
                        res,
                        403 as ::core::ffi::c_int,
                        b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return;
                }
                let c2rust_fresh0 = authorized_count;
                authorized_count += 1;
                *authorized_ids.offset(c2rust_fresh0 as isize) = id;
            }
        }
        id_item = (*id_item).next as *mut cJSON;
    }
    let mut success_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < authorized_count {
        if set_recording_protected(*authorized_ids.offset(i as isize), protected)
            == 0 as ::core::ffi::c_int
        {
            success_count += 1;
        } else {
            fail_count += 1;
        }
        i += 1;
    }
    free(authorized_ids as *mut ::core::ffi::c_void);
    cJSON_Delete(json);
    audit_batch_policy_operation(
        req,
        &raw mut user,
        protected,
        item_count,
        success_count,
        fail_count,
        if fail_count == 0 as ::core::ffi::c_int {
            b"success\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"failure\0".as_ptr() as *const ::core::ffi::c_char
        },
        if fail_count == 0 as ::core::ffi::c_int {
            b"completed\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"partial_failure\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddNumberToObject(
        response,
        b"success_count\0".as_ptr() as *const ::core::ffi::c_char,
        success_count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        response,
        b"fail_count\0".as_ptr() as *const ::core::ffi::c_char,
        fail_count as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        response,
        b"protected\0".as_ptr() as *const ::core::ffi::c_char,
        protected as cJSON_bool,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        if protected as ::core::ffi::c_int != 0 {
            b"Recordings protected\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"Recordings unprotected\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Batch protect: %d succeeded, %d failed, protected=%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        success_count,
        fail_count,
        if protected as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
}
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
