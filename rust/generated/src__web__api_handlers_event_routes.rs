unsafe extern "C" {
    pub type fleet_selector;
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
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
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
    unsafe fn fleet_selector_matches(
        selector: *const fleet_selector_t,
        camera: *const fleet_camera_t,
        explanation: *mut fleet_selector_explanation_t,
    ) -> bool;
    unsafe fn event_registry_all(count: *mut ::core::ffi::c_int) -> *const event_type_definition_t;
    unsafe fn event_registry_find(
        r#type: *const ::core::ffi::c_char,
    ) -> *const event_type_definition_t;
    unsafe fn event_severity_name(severity: event_severity_t) -> *const ::core::ffi::c_char;
    unsafe fn event_sensitivity_name(
        sensitivity: event_sensitivity_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn event_media_policy_name(policy: event_media_policy_t) -> *const ::core::ffi::c_char;
    unsafe fn event_expected_rate_name(rate: event_expected_rate_t) -> *const ::core::ffi::c_char;
    unsafe fn db_event_route_validate(
        route: *const event_route_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> db_event_route_result_t;
    unsafe fn db_event_route_count() -> ::core::ffi::c_int;
    unsafe fn db_event_route_list(
        routes: *mut event_route_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_route_get(
        uuid: *const ::core::ffi::c_char,
        route: *mut event_route_t,
    ) -> db_event_route_result_t;
    unsafe fn db_event_route_create(route: *mut event_route_t) -> db_event_route_result_t;
    unsafe fn db_event_route_update(
        route: *mut event_route_t,
        expected_revision: int64_t,
    ) -> db_event_route_result_t;
    unsafe fn db_event_route_delete(
        uuid: *const ::core::ffi::c_char,
        expected_revision: int64_t,
    ) -> db_event_route_result_t;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fleet_camera_enrich_runtime_health(
        cameras: *mut fleet_camera_t,
        count: ::core::ffi::c_int,
    );
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_type_definition_t {
    pub r#type: *const ::core::ffi::c_char,
    pub family: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub severity: event_severity_t,
    pub dynamic_severity: bool,
    pub minimum_severity: event_severity_t,
    pub maximum_severity: event_severity_t,
    pub sensitivity: event_sensitivity_t,
    pub media_policy: event_media_policy_t,
    pub expected_rate: event_expected_rate_t,
    pub subject_kind: event_subject_kind_t,
    pub default_expiry_seconds: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_subject_kind_t(pub ::core::ffi::c_uint);
impl event_subject_kind_t {
    pub const EVENT_SUBJECT_CAMERA: Self = Self(0);
    pub const EVENT_SUBJECT_STORAGE: Self = Self(1);
    pub const EVENT_SUBJECT_SYSTEM: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_expected_rate_t(pub ::core::ffi::c_uint);
impl event_expected_rate_t {
    pub const EVENT_RATE_LOW: Self = Self(0);
    pub const EVENT_RATE_MEDIUM: Self = Self(1);
    pub const EVENT_RATE_HIGH: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_media_policy_t(pub ::core::ffi::c_uint);
impl event_media_policy_t {
    pub const EVENT_MEDIA_FORBIDDEN: Self = Self(0);
    pub const EVENT_MEDIA_REFERENCE_ALLOWED: Self = Self(1);
    pub const EVENT_MEDIA_BINARY_OPT_IN: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_sensitivity_t(pub ::core::ffi::c_uint);
impl event_sensitivity_t {
    pub const EVENT_SENSITIVITY_OPERATIONAL: Self = Self(0);
    pub const EVENT_SENSITIVITY_INTERNAL: Self = Self(1);
    pub const EVENT_SENSITIVITY_RESTRICTED: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct event_severity_t(pub ::core::ffi::c_uint);
impl event_severity_t {
    pub const EVENT_SEVERITY_INFO: Self = Self(0);
    pub const EVENT_SEVERITY_WARNING: Self = Self(1);
    pub const EVENT_SEVERITY_ERROR: Self = Self(2);
    pub const EVENT_SEVERITY_CRITICAL: Self = Self(3);
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
    pub const AUTHZ_ACTION_COUNT: Self = Self(19);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_route_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub description: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub destination_key: [::core::ffi::c_char; 128],
    pub scope_type: [::core::ffi::c_char; 16],
    pub selector_json: [::core::ffi::c_char; 8192],
    pub predicate_json: [::core::ffi::c_char; 4096],
    pub schedule_json: [::core::ffi::c_char; 8192],
    pub event_types: [[::core::ffi::c_char; 96]; 32],
    pub event_type_count: ::core::ffi::c_int,
    pub debounce_seconds: ::core::ffi::c_int,
    pub cooldown_seconds: ::core::ffi::c_int,
    pub grouping_window_seconds: ::core::ffi::c_int,
    pub max_events_per_minute: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_event_route_result_t(pub ::core::ffi::c_int);
impl db_event_route_result_t {
    pub const DB_EVENT_ROUTE_OK: Self = Self(0);
    pub const DB_EVENT_ROUTE_NOT_FOUND: Self = Self(-1);
    pub const DB_EVENT_ROUTE_CONFLICT: Self = Self(-2);
    pub const DB_EVENT_ROUTE_INVALID: Self = Self(-3);
    pub const DB_EVENT_ROUTE_STALE: Self = Self(-4);
    pub const DB_EVENT_ROUTE_LIMIT: Self = Self(-5);
    pub const DB_EVENT_ROUTE_ERROR: Self = Self(-6);
}
pub type fleet_selector_t = fleet_selector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_selector_explanation_t {
    pub clauses: [[::core::ffi::c_char; 160]; 64],
    pub clause_count: ::core::ffi::c_int,
}
pub const EVENT_TYPE_MAX: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const EVENT_ROUTE_UUID_MAX: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const EVENT_ROUTE_MAX_TYPES: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const EVENT_ROUTE_MAX_COUNT: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const EVENT_ROUTE_DEFAULT_DESTINATION: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"mqtt:default\0") };
pub const EVENT_ROUTE_PREVIEW_MAX_CAMERAS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
unsafe extern "C" fn subject_kind_name(
    mut kind: event_subject_kind_t,
) -> *const ::core::ffi::c_char {
    match kind {
        event_subject_kind_t::EVENT_SUBJECT_CAMERA => {
            return b"camera\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_subject_kind_t::EVENT_SUBJECT_STORAGE => {
            return b"storage\0".as_ptr() as *const ::core::ffi::c_char;
        }
        event_subject_kind_t::EVENT_SUBJECT_SYSTEM => {
            return b"system\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn definition_minimum_severity(
    mut definition: *const event_type_definition_t,
) -> event_severity_t {
    return event_severity_t(
        if (*definition).dynamic_severity as ::core::ffi::c_int != 0 {
            (*definition).minimum_severity.0
        } else {
            (*definition).severity.0
        },
    );
}
unsafe extern "C" fn definition_maximum_severity(
    mut definition: *const event_type_definition_t,
) -> event_severity_t {
    return event_severity_t(
        if (*definition).dynamic_severity as ::core::ffi::c_int != 0 {
            (*definition).maximum_severity.0
        } else {
            (*definition).severity.0
        },
    );
}
unsafe extern "C" fn route_has_camera_event(mut route: *const event_route_t) -> bool {
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < (*route).event_type_count {
        let mut definition: *const event_type_definition_t = event_registry_find(
            &raw const *(&raw const (*route).event_types as *const [::core::ffi::c_char; 96])
                .offset(index as isize) as *const ::core::ffi::c_char,
        );
        if !definition.is_null()
            && (*definition).subject_kind.0 == event_subject_kind_t::EVENT_SUBJECT_CAMERA.0
        {
            return r#true != 0;
        }
        index += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn authorize_events(
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
        authorization_action_t::AUTHZ_EVENTS_CONFIGURE,
        ::core::ptr::null::<fleet_camera_t>(),
        user,
        &raw mut evaluation,
    ) != 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn extract_route_uuid(
    mut req: *const http_request_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut res: *mut http_response_t,
) -> bool {
    let mut value: [::core::ffi::c_char; 512] = [0; 512];
    if http_request_extract_path_param(
        req,
        b"/api/event-routes/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid event route path\0".as_ptr() as *const ::core::ffi::c_char,
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
            b"Invalid event route UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        uuid,
        &raw mut value as *mut ::core::ffi::c_char,
        EVENT_ROUTE_UUID_MAX as size_t,
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn set_db_error(
    mut res: *mut http_response_t,
    mut result: db_event_route_result_t,
    mut validation_error: *const ::core::ffi::c_char,
) {
    match result {
        db_event_route_result_t::DB_EVENT_ROUTE_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Event route not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_event_route_result_t::DB_EVENT_ROUTE_CONFLICT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Event route name already exists\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_event_route_result_t::DB_EVENT_ROUTE_STALE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Event route was changed by another administrator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_event_route_result_t::DB_EVENT_ROUTE_LIMIT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Event route limit reached\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_event_route_result_t::DB_EVENT_ROUTE_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                if !validation_error.is_null()
                    && *validation_error.offset(0isize) as ::core::ffi::c_int != 0
                {
                    validation_error
                } else {
                    b"Invalid event route\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Event route operation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn parse_stored_object(mut json: *const ::core::ffi::c_char) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_Parse(json);
    if cJSON_IsObject(object) == 0 {
        cJSON_Delete(object);
        return ::core::ptr::null_mut::<cJSON>();
    }
    return object;
}
unsafe extern "C" fn route_to_json(mut route: *const event_route_t) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_CreateObject();
    let mut event_types: *mut cJSON = cJSON_CreateArray();
    let mut scope: *mut cJSON = cJSON_CreateObject();
    let mut predicate: *mut cJSON =
        parse_stored_object(&raw const (*route).predicate_json as *const ::core::ffi::c_char);
    let mut schedule: *mut cJSON =
        parse_stored_object(&raw const (*route).schedule_json as *const ::core::ffi::c_char);
    let mut suppression: *mut cJSON = cJSON_CreateObject();
    if object.is_null()
        || event_types.is_null()
        || scope.is_null()
        || predicate.is_null()
        || schedule.is_null()
        || suppression.is_null()
    {
        cJSON_Delete(object);
        cJSON_Delete(event_types);
        cJSON_Delete(scope);
        cJSON_Delete(predicate);
        cJSON_Delete(schedule);
        cJSON_Delete(suppression);
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*route).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*route).name as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"description\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*route).description as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        object,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        (*route).enabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        object,
        b"destination\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*route).destination_key as *const ::core::ffi::c_char,
    );
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < (*route).event_type_count {
        cJSON_AddItemToArray(
            event_types,
            cJSON_CreateString(
                &raw const *(&raw const (*route).event_types as *const [::core::ffi::c_char; 96])
                    .offset(index as isize) as *const ::core::ffi::c_char,
            ),
        );
        index += 1;
    }
    cJSON_AddItemToObject(
        object,
        b"event_types\0".as_ptr() as *const ::core::ffi::c_char,
        event_types,
    );
    cJSON_AddStringToObject(
        scope,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*route).scope_type as *const ::core::ffi::c_char,
    );
    if strcmp(
        &raw const (*route).scope_type as *const ::core::ffi::c_char,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut selector: *mut cJSON =
            parse_stored_object(&raw const (*route).selector_json as *const ::core::ffi::c_char);
        if selector.is_null() {
            cJSON_Delete(object);
            cJSON_Delete(scope);
            cJSON_Delete(predicate);
            cJSON_Delete(schedule);
            cJSON_Delete(suppression);
            return ::core::ptr::null_mut::<cJSON>();
        }
        cJSON_AddItemToObject(
            scope,
            b"selector\0".as_ptr() as *const ::core::ffi::c_char,
            selector,
        );
    }
    cJSON_AddItemToObject(
        object,
        b"camera_scope\0".as_ptr() as *const ::core::ffi::c_char,
        scope,
    );
    cJSON_AddItemToObject(
        object,
        b"predicate\0".as_ptr() as *const ::core::ffi::c_char,
        predicate,
    );
    cJSON_AddItemToObject(
        object,
        b"schedule\0".as_ptr() as *const ::core::ffi::c_char,
        schedule,
    );
    cJSON_AddNumberToObject(
        suppression,
        b"debounce_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        (*route).debounce_seconds as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        suppression,
        b"cooldown_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        (*route).cooldown_seconds as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        suppression,
        b"grouping_window_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        (*route).grouping_window_seconds as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        suppression,
        b"max_events_per_minute\0".as_ptr() as *const ::core::ffi::c_char,
        (*route).max_events_per_minute as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        object,
        b"suppression\0".as_ptr() as *const ::core::ffi::c_char,
        suppression,
    );
    cJSON_AddNumberToObject(
        object,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        (*route).revision as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*route).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*route).updated_at as ::core::ffi::c_double,
    );
    return object;
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
            b"Failed to serialize event route response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    http_response_set_json(res, status, body);
    free(body as *mut ::core::ffi::c_void);
    return r#true != 0;
}
unsafe extern "C" fn object_has_only(
    mut object: *const cJSON,
    mut fields: *const *const ::core::ffi::c_char,
    mut field_count: size_t,
    mut context: *const ::core::ffi::c_char,
    mut res: *mut http_response_t,
) -> bool {
    if cJSON_IsObject(object) == 0 {
        let mut message: [::core::ffi::c_char; 128] = [0; 128];
        snprintf(
            &raw mut message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            b"%s must be an object\0".as_ptr() as *const ::core::ffi::c_char,
            context,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            &raw mut message as *mut ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut item: *const cJSON = (*object).child;
    while !item.is_null() {
        let mut known: bool = r#false != 0;
        let mut index: size_t = 0 as size_t;
        while index < field_count {
            if !(*item).string.is_null()
                && strcmp((*item).string, *fields.offset(index as isize)) == 0 as ::core::ffi::c_int
            {
                known = r#true != 0;
                break;
            } else {
                index = index.wrapping_add(1);
            }
        }
        if !known {
            let mut message_0: [::core::ffi::c_char; 192] = [0; 192];
            snprintf(
                &raw mut message_0 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 192]>(),
                b"%s contains unknown field '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                context,
                if !(*item).string.is_null() {
                    (*item).string
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                &raw mut message_0 as *mut ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        item = (*item).next;
    }
    return r#true != 0;
}
unsafe extern "C" fn store_json_object(
    mut object: *const cJSON,
    mut destination: *mut ::core::ffi::c_char,
    mut destination_size: size_t,
    mut field: *const ::core::ffi::c_char,
    mut res: *mut http_response_t,
) -> bool {
    if cJSON_IsObject(object) == 0 {
        let mut message: [::core::ffi::c_char; 128] = [0; 128];
        snprintf(
            &raw mut message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            b"%s must be an object\0".as_ptr() as *const ::core::ffi::c_char,
            field,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            &raw mut message as *mut ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut serialized: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(object);
    if serialized.is_null() || strlen(serialized) >= destination_size {
        free(serialized as *mut ::core::ffi::c_void);
        let mut message_0: [::core::ffi::c_char; 128] = [0; 128];
        snprintf(
            &raw mut message_0 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            b"%s is too large\0".as_ptr() as *const ::core::ffi::c_char,
            field,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            &raw mut message_0 as *mut ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(destination, serialized, destination_size, 0 as size_t);
    free(serialized as *mut ::core::ffi::c_void);
    return r#true != 0;
}
unsafe extern "C" fn parse_integer(
    mut object: *const cJSON,
    mut field: *const ::core::ffi::c_char,
    mut minimum: ::core::ffi::c_int,
    mut maximum: ::core::ffi::c_int,
    mut destination: *mut ::core::ffi::c_int,
    mut res: *mut http_response_t,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(object, field);
    if item.is_null() {
        return r#true != 0;
    }
    if cJSON_IsNumber(item) == 0
        || (*item).valuedouble != (*item).valueint as ::core::ffi::c_double
        || (*item).valueint < minimum
        || (*item).valueint > maximum
    {
        let mut message: [::core::ffi::c_char; 160] = [0; 160];
        snprintf(
            &raw mut message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
            b"%s must be an integer from %d to %d\0".as_ptr() as *const ::core::ffi::c_char,
            field,
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
unsafe extern "C" fn apply_event_types(
    mut array: *const cJSON,
    mut route: *mut event_route_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut count: ::core::ffi::c_int = if cJSON_IsArray(array) != 0 {
        cJSON_GetArraySize(array)
    } else {
        -1 as ::core::ffi::c_int
    };
    if count < 1 as ::core::ffi::c_int || count > EVENT_ROUTE_MAX_TYPES {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"event_types must contain 1-32 values\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    memset(
        &raw mut (*route).event_types as *mut [::core::ffi::c_char; 96] as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[[::core::ffi::c_char; 96]; 32]>(),
    );
    (*route).event_type_count = 0 as ::core::ffi::c_int;
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut item: *const cJSON = cJSON_GetArrayItem(array, index);
        if cJSON_IsString(item) == 0
            || (*item).valuestring.is_null()
            || strlen((*item).valuestring) >= EVENT_TYPE_MAX as size_t
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"event_types contains an invalid value\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        let c2rust_fresh0 = (*route).event_type_count;
        (*route).event_type_count += 1;
        safe_strcpy(
            &raw mut *(&raw mut (*route).event_types as *mut [::core::ffi::c_char; 96])
                .offset(c2rust_fresh0 as isize) as *mut ::core::ffi::c_char,
            (*item).valuestring,
            EVENT_TYPE_MAX as size_t,
            0 as size_t,
        );
        index += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn apply_camera_scope(
    mut scope: *const cJSON,
    mut route: *mut event_route_t,
    mut res: *mut http_response_t,
) -> bool {
    let fields: [*const ::core::ffi::c_char; 2] = [
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if !object_has_only(
        scope,
        &raw const fields as *const *const ::core::ffi::c_char,
        2 as size_t,
        b"camera_scope\0".as_ptr() as *const ::core::ffi::c_char,
        res,
    ) {
        return r#false != 0;
    }
    let mut r#type: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(scope, b"type\0".as_ptr() as *const ::core::ffi::c_char);
    let mut selector: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        scope,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsString(r#type) == 0
        || strcmp(
            (*r#type).valuestring,
            b"all\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            && strcmp(
                (*r#type).valuestring,
                b"selector\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"camera_scope.type must be all or selector\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        &raw mut (*route).scope_type as *mut ::core::ffi::c_char,
        (*r#type).valuestring,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    if strcmp(
        &raw mut (*route).scope_type as *mut ::core::ffi::c_char,
        b"all\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if !selector.is_null() {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"all camera scope cannot include selector\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*route).selector_json[0usize] = '\0' as ::core::ffi::c_char;
        return r#true != 0;
    }
    if selector.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"selector camera scope requires selector\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return store_json_object(
        selector,
        &raw mut (*route).selector_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        b"camera selector\0".as_ptr() as *const ::core::ffi::c_char,
        res,
    );
}
unsafe extern "C" fn apply_suppression(
    mut suppression: *const cJSON,
    mut route: *mut event_route_t,
    mut res: *mut http_response_t,
) -> bool {
    let fields: [*const ::core::ffi::c_char; 4] = [
        b"debounce_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        b"cooldown_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        b"grouping_window_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        b"max_events_per_minute\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    return object_has_only(
        suppression,
        &raw const fields as *const *const ::core::ffi::c_char,
        4 as size_t,
        b"suppression\0".as_ptr() as *const ::core::ffi::c_char,
        res,
    ) as ::core::ffi::c_int
        != 0
        && parse_integer(
            suppression,
            b"debounce_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            86400 as ::core::ffi::c_int,
            &raw mut (*route).debounce_seconds,
            res,
        ) as ::core::ffi::c_int
            != 0
        && parse_integer(
            suppression,
            b"cooldown_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            604800 as ::core::ffi::c_int,
            &raw mut (*route).cooldown_seconds,
            res,
        ) as ::core::ffi::c_int
            != 0
        && parse_integer(
            suppression,
            b"grouping_window_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            3600 as ::core::ffi::c_int,
            &raw mut (*route).grouping_window_seconds,
            res,
        ) as ::core::ffi::c_int
            != 0
        && parse_integer(
            suppression,
            b"max_events_per_minute\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            60000 as ::core::ffi::c_int,
            &raw mut (*route).max_events_per_minute,
            res,
        ) as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn initialize_route_defaults(mut route: *mut event_route_t) {
    memset(
        route as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_route_t>(),
    );
    (*route).enabled = r#true != 0;
    safe_strcpy(
        &raw mut (*route).destination_key as *mut ::core::ffi::c_char,
        EVENT_ROUTE_DEFAULT_DESTINATION.as_ptr(),
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*route).scope_type as *mut ::core::ffi::c_char,
        b"all\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*route).predicate_json as *mut ::core::ffi::c_char,
        b"{\"version\":1}\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*route).schedule_json as *mut ::core::ffi::c_char,
        b"{\"version\":1,\"timezone\":\"UTC\",\"windows\":[]}\0".as_ptr()
            as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        0 as size_t,
    );
}
unsafe extern "C" fn apply_route_body(
    mut body: *const cJSON,
    mut route: *mut event_route_t,
    mut creating: bool,
    mut expected_revision: *mut int64_t,
    mut res: *mut http_response_t,
) -> bool {
    let fields: [*const ::core::ffi::c_char; 10] = [
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        b"description\0".as_ptr() as *const ::core::ffi::c_char,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        b"destination\0".as_ptr() as *const ::core::ffi::c_char,
        b"event_types\0".as_ptr() as *const ::core::ffi::c_char,
        b"camera_scope\0".as_ptr() as *const ::core::ffi::c_char,
        b"predicate\0".as_ptr() as *const ::core::ffi::c_char,
        b"schedule\0".as_ptr() as *const ::core::ffi::c_char,
        b"suppression\0".as_ptr() as *const ::core::ffi::c_char,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if !object_has_only(
        body,
        &raw const fields as *const *const ::core::ffi::c_char,
        10 as size_t,
        b"request body\0".as_ptr() as *const ::core::ffi::c_char,
        res,
    ) {
        return r#false != 0;
    }
    let mut name: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"name\0".as_ptr() as *const ::core::ffi::c_char);
    if !name.is_null() {
        if cJSON_IsString(name) == 0
            || (*name).valuestring.is_null()
            || strlen((*name).valuestring) >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid event route name\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*route).name as *mut ::core::ffi::c_char,
            (*name).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
    } else if creating {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Event route name is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut description: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"description\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !description.is_null() {
        if cJSON_IsString(description) == 0
            || (*description).valuestring.is_null()
            || strlen((*description).valuestring)
                >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid event route description\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*route).description as *mut ::core::ffi::c_char,
            (*description).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    let mut enabled: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"enabled\0".as_ptr() as *const ::core::ffi::c_char);
    if !enabled.is_null() {
        if cJSON_IsBool(enabled) == 0 {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"enabled must be boolean\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*route).enabled = cJSON_IsTrue(enabled) != 0;
    }
    let mut destination: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"destination\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !destination.is_null() {
        if cJSON_IsString(destination) == 0
            || (*destination).valuestring.is_null()
            || strlen((*destination).valuestring)
                >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid destination\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*route).destination_key as *mut ::core::ffi::c_char,
            (*destination).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
    }
    let mut event_types: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"event_types\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !event_types.is_null() {
        if !apply_event_types(event_types, route, res) {
            return r#false != 0;
        }
    } else if creating {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"event_types is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut scope: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"camera_scope\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !scope.is_null() && !apply_camera_scope(scope, route, res) {
        return r#false != 0;
    }
    let mut predicate: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"predicate\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !predicate.is_null()
        && !store_json_object(
            predicate,
            &raw mut (*route).predicate_json as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
            b"predicate\0".as_ptr() as *const ::core::ffi::c_char,
            res,
        )
    {
        return r#false != 0;
    }
    let mut schedule: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"schedule\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !schedule.is_null()
        && !store_json_object(
            schedule,
            &raw mut (*route).schedule_json as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
            b"schedule\0".as_ptr() as *const ::core::ffi::c_char,
            res,
        )
    {
        return r#false != 0;
    }
    let mut suppression: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"suppression\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !suppression.is_null() && !apply_suppression(suppression, route, res) {
        return r#false != 0;
    }
    let mut revision: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !creating {
        let mut value: ::core::ffi::c_double = if cJSON_IsNumber(revision) != 0 {
            (*revision).valuedouble
        } else {
            0.0f64
        };
        if cJSON_IsNumber(revision) == 0
            || value < 1.0f64
            || value > 9007199254740991.0f64
            || value != value as int64_t as ::core::ffi::c_double
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"revision must be a positive integer\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        *expected_revision = value as int64_t;
    } else if !revision.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"revision is assigned by the server\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
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
    let mut validation: db_event_route_result_t = db_event_route_validate(
        route,
        &raw mut validation_error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if validation.0 != db_event_route_result_t::DB_EVENT_ROUTE_OK.0 {
        set_db_error(
            res,
            validation,
            &raw mut validation_error as *mut ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn audit_route(
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
        b"events.configure\0".as_ptr() as *const ::core::ffi::c_char,
        b"event_route\0".as_ptr() as *const ::core::ffi::c_char,
        uuid,
        operation,
        outcome,
        context,
    );
    cJSON_Delete(context);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_event_catalog(
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
    if !authorize_events(req, res, &raw mut user) {
        return;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut definitions: *const event_type_definition_t = event_registry_all(&raw mut count);
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut types: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || types.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(types);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create event catalog\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddNumberToObject(
        root,
        b"schema_version\0".as_ptr() as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        count as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"event_types\0".as_ptr() as *const ::core::ffi::c_char,
        types,
    );
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut item: *mut cJSON = cJSON_CreateObject();
        if item.is_null() {
            cJSON_Delete(root);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create event catalog\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddStringToObject(
            item,
            b"type\0".as_ptr() as *const ::core::ffi::c_char,
            (*definitions.offset(index as isize)).r#type,
        );
        cJSON_AddStringToObject(
            item,
            b"family\0".as_ptr() as *const ::core::ffi::c_char,
            (*definitions.offset(index as isize)).family,
        );
        cJSON_AddStringToObject(
            item,
            b"description\0".as_ptr() as *const ::core::ffi::c_char,
            (*definitions.offset(index as isize)).description,
        );
        cJSON_AddStringToObject(
            item,
            b"severity\0".as_ptr() as *const ::core::ffi::c_char,
            event_severity_name((*definitions.offset(index as isize)).severity),
        );
        cJSON_AddBoolToObject(
            item,
            b"dynamic_severity\0".as_ptr() as *const ::core::ffi::c_char,
            (*definitions.offset(index as isize)).dynamic_severity as cJSON_bool,
        );
        cJSON_AddStringToObject(
            item,
            b"minimum_severity\0".as_ptr() as *const ::core::ffi::c_char,
            event_severity_name(definition_minimum_severity(
                definitions.offset(index as isize),
            )),
        );
        cJSON_AddStringToObject(
            item,
            b"maximum_severity\0".as_ptr() as *const ::core::ffi::c_char,
            event_severity_name(definition_maximum_severity(
                definitions.offset(index as isize),
            )),
        );
        cJSON_AddStringToObject(
            item,
            b"sensitivity\0".as_ptr() as *const ::core::ffi::c_char,
            event_sensitivity_name((*definitions.offset(index as isize)).sensitivity),
        );
        cJSON_AddStringToObject(
            item,
            b"media_policy\0".as_ptr() as *const ::core::ffi::c_char,
            event_media_policy_name((*definitions.offset(index as isize)).media_policy),
        );
        cJSON_AddStringToObject(
            item,
            b"expected_rate\0".as_ptr() as *const ::core::ffi::c_char,
            event_expected_rate_name((*definitions.offset(index as isize)).expected_rate),
        );
        cJSON_AddStringToObject(
            item,
            b"subject_kind\0".as_ptr() as *const ::core::ffi::c_char,
            subject_kind_name((*definitions.offset(index as isize)).subject_kind),
        );
        cJSON_AddNumberToObject(
            item,
            b"default_expiry_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            (*definitions.offset(index as isize)).default_expiry_seconds as ::core::ffi::c_double,
        );
        cJSON_AddItemToArray(types, item);
        index += 1;
    }
    set_json_response(res, 200 as ::core::ffi::c_int, root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_event_routes(
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
    if !authorize_events(req, res, &raw mut user) {
        return;
    }
    let mut total: ::core::ffi::c_int = db_event_route_count();
    if total < 0 as ::core::ffi::c_int || total > EVENT_ROUTE_MAX_COUNT {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to count event routes\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut routes: *mut event_route_t = (if total > 0 as ::core::ffi::c_int {
        calloc(total as size_t, ::core::mem::size_of::<event_route_t>())
    } else {
        NULL
    }) as *mut event_route_t;
    if total > 0 as ::core::ffi::c_int && routes.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int = if total > 0 as ::core::ffi::c_int {
        db_event_route_list(routes, total)
    } else {
        0 as ::core::ffi::c_int
    };
    if count < 0 as ::core::ffi::c_int {
        free(routes as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to list event routes\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || items.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(routes as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
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
        b"routes\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut item: *mut cJSON = route_to_json(routes.offset(index as isize));
        if item.is_null() {
            cJSON_Delete(root);
            free(routes as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddItemToArray(items, item);
        index += 1;
    }
    free(routes as *mut ::core::ffi::c_void);
    set_json_response(res, 200 as ::core::ffi::c_int, root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_event_route(
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
    if !authorize_events(req, res, &raw mut user) {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut route: event_route_t = event_route_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        enabled: false,
        destination_key: [0; 128],
        scope_type: [0; 16],
        selector_json: [0; 8192],
        predicate_json: [0; 4096],
        schedule_json: [0; 8192],
        event_types: [[0; 96]; 32],
        event_type_count: 0,
        debounce_seconds: 0,
        cooldown_seconds: 0,
        grouping_window_seconds: 0,
        max_events_per_minute: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    initialize_route_defaults(&raw mut route);
    let mut unused_revision: int64_t = 0 as int64_t;
    if !apply_route_body(
        body,
        &raw mut route,
        r#true != 0,
        &raw mut unused_revision,
        res,
    ) {
        cJSON_Delete(body);
        audit_route(
            req,
            &raw mut user,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"route_create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_Delete(body);
    let mut result: db_event_route_result_t = db_event_route_create(&raw mut route);
    if result.0 != db_event_route_result_t::DB_EVENT_ROUTE_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        audit_route(
            req,
            &raw mut user,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"route_create\0".as_ptr() as *const ::core::ffi::c_char,
            if result.0 == db_event_route_result_t::DB_EVENT_ROUTE_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            b"persistence_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_route(
        req,
        &raw mut user,
        &raw mut route.uuid as *mut ::core::ffi::c_char,
        b"route_create\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"created\0".as_ptr() as *const ::core::ffi::c_char,
    );
    set_json_response(
        res,
        201 as ::core::ffi::c_int,
        route_to_json(&raw mut route),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_event_route(
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
    if !authorize_events(req, res, &raw mut user) {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_route_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res) {
        return;
    }
    let mut route: event_route_t = event_route_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        enabled: false,
        destination_key: [0; 128],
        scope_type: [0; 16],
        selector_json: [0; 8192],
        predicate_json: [0; 4096],
        schedule_json: [0; 8192],
        event_types: [[0; 96]; 32],
        event_type_count: 0,
        debounce_seconds: 0,
        cooldown_seconds: 0,
        grouping_window_seconds: 0,
        max_events_per_minute: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_event_route_result_t =
        db_event_route_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut route);
    if result.0 != db_event_route_result_t::DB_EVENT_ROUTE_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        return;
    }
    set_json_response(
        res,
        200 as ::core::ffi::c_int,
        route_to_json(&raw mut route),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_event_route(
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
    if !authorize_events(req, res, &raw mut user) {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_route_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res) {
        return;
    }
    let mut route: event_route_t = event_route_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        enabled: false,
        destination_key: [0; 128],
        scope_type: [0; 16],
        selector_json: [0; 8192],
        predicate_json: [0; 4096],
        schedule_json: [0; 8192],
        event_types: [[0; 96]; 32],
        event_type_count: 0,
        debounce_seconds: 0,
        cooldown_seconds: 0,
        grouping_window_seconds: 0,
        max_events_per_minute: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_event_route_result_t =
        db_event_route_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut route);
    if result.0 != db_event_route_result_t::DB_EVENT_ROUTE_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut expected_revision: int64_t = 0 as int64_t;
    if !apply_route_body(
        body,
        &raw mut route,
        r#false != 0,
        &raw mut expected_revision,
        res,
    ) {
        cJSON_Delete(body);
        audit_route(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"route_update\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_Delete(body);
    result = db_event_route_update(&raw mut route, expected_revision);
    if result.0 != db_event_route_result_t::DB_EVENT_ROUTE_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        audit_route(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"route_update\0".as_ptr() as *const ::core::ffi::c_char,
            if result.0 == db_event_route_result_t::DB_EVENT_ROUTE_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            if result.0 == db_event_route_result_t::DB_EVENT_ROUTE_STALE.0 {
                b"stale_revision\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"persistence_failed\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    audit_route(
        req,
        &raw mut user,
        &raw mut uuid as *mut ::core::ffi::c_char,
        b"route_update\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"updated\0".as_ptr() as *const ::core::ffi::c_char,
    );
    set_json_response(
        res,
        200 as ::core::ffi::c_int,
        route_to_json(&raw mut route),
    );
}
unsafe extern "C" fn parse_delete_revision(
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
pub unsafe extern "C" fn handle_delete_event_route(
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
    if !authorize_events(req, res, &raw mut user) {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    let mut revision: int64_t = 0 as int64_t;
    if !extract_route_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res)
        || !parse_delete_revision(req, &raw mut revision, res)
    {
        return;
    }
    let mut result: db_event_route_result_t =
        db_event_route_delete(&raw mut uuid as *mut ::core::ffi::c_char, revision);
    if result.0 != db_event_route_result_t::DB_EVENT_ROUTE_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        audit_route(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"route_delete\0".as_ptr() as *const ::core::ffi::c_char,
            if result.0 == db_event_route_result_t::DB_EVENT_ROUTE_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            if result.0 == db_event_route_result_t::DB_EVENT_ROUTE_STALE.0 {
                b"stale_revision\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"persistence_failed\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    audit_route(
        req,
        &raw mut user,
        &raw mut uuid as *mut ::core::ffi::c_char,
        b"route_delete\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn handle_post_event_route_preview(
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
    if !authorize_events(req, res, &raw mut user) {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut route: event_route_t = event_route_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        enabled: false,
        destination_key: [0; 128],
        scope_type: [0; 16],
        selector_json: [0; 8192],
        predicate_json: [0; 4096],
        schedule_json: [0; 8192],
        event_types: [[0; 96]; 32],
        event_type_count: 0,
        debounce_seconds: 0,
        cooldown_seconds: 0,
        grouping_window_seconds: 0,
        max_events_per_minute: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    initialize_route_defaults(&raw mut route);
    let mut unused_revision: int64_t = 0 as int64_t;
    if !apply_route_body(
        body,
        &raw mut route,
        r#true != 0,
        &raw mut unused_revision,
        res,
    ) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if db_fleet_camera_load(&raw mut cameras, &raw mut camera_count) != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load fleet for route preview\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    fleet_camera_enrich_runtime_health(cameras, camera_count);
    let mut selector: *mut fleet_selector_t = ::core::ptr::null_mut::<fleet_selector_t>();
    if strcmp(
        &raw mut route.scope_type as *mut ::core::ffi::c_char,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut selector_json: *mut cJSON =
            cJSON_Parse(&raw mut route.selector_json as *mut ::core::ffi::c_char);
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
        selector = fleet_selector_parse(
            selector_json,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        );
        cJSON_Delete(selector_json);
        if selector.is_null() {
            free(cameras as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                if error[0usize] as ::core::ffi::c_int != 0 {
                    &raw mut error as *mut ::core::ffi::c_char
                } else {
                    b"Invalid selector\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            return;
        }
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut sample: *mut cJSON = cJSON_CreateArray();
    let mut types: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || sample.is_null() || types.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(sample);
        cJSON_Delete(types);
        fleet_selector_free(selector);
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create preview response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut matched_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut includes_camera_events: bool = route_has_camera_event(&raw mut route);
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while includes_camera_events as ::core::ffi::c_int != 0 && index < camera_count {
        let mut matches: bool = selector.is_null()
            || fleet_selector_matches(
                selector,
                cameras.offset(index as isize),
                ::core::ptr::null_mut::<fleet_selector_explanation_t>(),
            ) as ::core::ffi::c_int
                != 0;
        if matches {
            if matched_count < EVENT_ROUTE_PREVIEW_MAX_CAMERAS {
                let mut item: *mut cJSON = cJSON_CreateObject();
                cJSON_AddStringToObject(
                    item,
                    b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*cameras.offset(index as isize)).camera_uuid
                        as *mut ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    item,
                    b"name\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*cameras.offset(index as isize)).name as *mut ::core::ffi::c_char,
                );
                cJSON_AddStringToObject(
                    item,
                    b"location_path\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*cameras.offset(index as isize)).location_path
                        as *mut ::core::ffi::c_char,
                );
                cJSON_AddItemToArray(sample, item);
            }
            matched_count += 1;
        }
        index += 1;
    }
    let mut index_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index_0 < route.event_type_count {
        let mut definition: *const event_type_definition_t = event_registry_find(
            &raw mut *(&raw mut route.event_types as *mut [::core::ffi::c_char; 96])
                .offset(index_0 as isize) as *mut ::core::ffi::c_char,
        );
        let mut item_0: *mut cJSON = cJSON_CreateObject();
        cJSON_AddStringToObject(
            item_0,
            b"type\0".as_ptr() as *const ::core::ffi::c_char,
            (*definition).r#type,
        );
        cJSON_AddStringToObject(
            item_0,
            b"expected_rate\0".as_ptr() as *const ::core::ffi::c_char,
            event_expected_rate_name((*definition).expected_rate),
        );
        cJSON_AddStringToObject(
            item_0,
            b"severity\0".as_ptr() as *const ::core::ffi::c_char,
            event_severity_name((*definition).severity),
        );
        cJSON_AddBoolToObject(
            item_0,
            b"dynamic_severity\0".as_ptr() as *const ::core::ffi::c_char,
            (*definition).dynamic_severity as cJSON_bool,
        );
        cJSON_AddStringToObject(
            item_0,
            b"minimum_severity\0".as_ptr() as *const ::core::ffi::c_char,
            event_severity_name(definition_minimum_severity(definition)),
        );
        cJSON_AddStringToObject(
            item_0,
            b"maximum_severity\0".as_ptr() as *const ::core::ffi::c_char,
            event_severity_name(definition_maximum_severity(definition)),
        );
        cJSON_AddStringToObject(
            item_0,
            b"subject_kind\0".as_ptr() as *const ::core::ffi::c_char,
            subject_kind_name((*definition).subject_kind),
        );
        cJSON_AddItemToArray(types, item_0);
        index_0 += 1;
    }
    cJSON_AddNumberToObject(
        root,
        b"matched_camera_count\0".as_ptr() as *const ::core::ffi::c_char,
        matched_count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"sample_limit\0".as_ptr() as *const ::core::ffi::c_char,
        EVENT_ROUTE_PREVIEW_MAX_CAMERAS as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"camera_sample\0".as_ptr() as *const ::core::ffi::c_char,
        sample,
    );
    cJSON_AddItemToObject(
        root,
        b"event_types\0".as_ptr() as *const ::core::ffi::c_char,
        types,
    );
    cJSON_AddBoolToObject(
        root,
        b"would_publish\0".as_ptr() as *const ::core::ffi::c_char,
        r#false,
    );
    fleet_selector_free(selector);
    free(cameras as *mut ::core::ffi::c_void);
    set_json_response(res, 200 as ::core::ffi::c_int, root);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
