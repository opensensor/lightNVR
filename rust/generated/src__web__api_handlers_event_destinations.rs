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
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNull(item: *const cJSON) -> cJSON_bool;
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
    unsafe fn db_event_destination_validate(
        destination: *const event_destination_t,
        password: *const ::core::ffi::c_char,
        validate_password: bool,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> db_event_destination_result_t;
    unsafe fn db_event_destination_count() -> ::core::ffi::c_int;
    unsafe fn db_event_destination_list(
        destinations: *mut event_destination_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_event_destination_get(
        uuid: *const ::core::ffi::c_char,
        destination: *mut event_destination_t,
    ) -> db_event_destination_result_t;
    unsafe fn db_event_destination_create(
        destination: *mut event_destination_t,
        password: *const ::core::ffi::c_char,
    ) -> db_event_destination_result_t;
    unsafe fn db_event_destination_update(
        destination: *mut event_destination_t,
        expected_revision: int64_t,
        password: *const ::core::ffi::c_char,
        replace_password: bool,
    ) -> db_event_destination_result_t;
    unsafe fn db_event_destination_delete(
        uuid: *const ::core::ffi::c_char,
        expected_revision: int64_t,
    ) -> db_event_destination_result_t;
    unsafe fn db_event_destination_make_key(
        uuid: *const ::core::ffi::c_char,
        key: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn secure_zero_memory(ptr: *mut ::core::ffi::c_void, size: size_t);
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn lightnvr_uuid_generate_v4(output: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_destination_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub description: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub destination_type: [::core::ffi::c_char; 16],
    pub broker_host: [::core::ffi::c_char; 256],
    pub broker_port: ::core::ffi::c_int,
    pub client_id: [::core::ffi::c_char; 128],
    pub topic_template: [::core::ffi::c_char; 512],
    pub status_topic_template: [::core::ffi::c_char; 512],
    pub username: [::core::ffi::c_char; 128],
    pub password_configured: bool,
    pub tls_mode: [::core::ffi::c_char; 16],
    pub ca_file: [::core::ffi::c_char; 512],
    pub cert_file: [::core::ffi::c_char; 512],
    pub key_file: [::core::ffi::c_char; 512],
    pub keepalive_seconds: ::core::ffi::c_int,
    pub qos: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_event_destination_result_t(pub ::core::ffi::c_int);
impl db_event_destination_result_t {
    pub const DB_EVENT_DESTINATION_OK: Self = Self(0);
    pub const DB_EVENT_DESTINATION_NOT_FOUND: Self = Self(-1);
    pub const DB_EVENT_DESTINATION_CONFLICT: Self = Self(-2);
    pub const DB_EVENT_DESTINATION_INVALID: Self = Self(-3);
    pub const DB_EVENT_DESTINATION_STALE: Self = Self(-4);
    pub const DB_EVENT_DESTINATION_LIMIT: Self = Self(-5);
    pub const DB_EVENT_DESTINATION_IN_USE: Self = Self(-6);
    pub const DB_EVENT_DESTINATION_ERROR: Self = Self(-7);
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
pub const EVENT_DESTINATION_UUID_MAX: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const EVENT_DESTINATION_PASSWORD_MAX: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const EVENT_DESTINATION_MAX_COUNT: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const EVENT_DESTINATION_DEFAULT_TOPIC_TEMPLATE: [::core::ffi::c_char; 39] = unsafe {
    ::core::mem::transmute::<[u8; 39], [::core::ffi::c_char; 39]>(
        *b"lightnvr/v1/events/{type}/{subject_id}\0",
    )
};
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
unsafe extern "C" fn extract_destination_uuid(
    mut req: *const http_request_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut res: *mut http_response_t,
) -> bool {
    let mut value: [::core::ffi::c_char; 512] = [0; 512];
    if http_request_extract_path_param(
        req,
        b"/api/event-destinations/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid event destination path\0".as_ptr() as *const ::core::ffi::c_char,
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
            b"Invalid event destination UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        uuid,
        &raw mut value as *mut ::core::ffi::c_char,
        EVENT_DESTINATION_UUID_MAX as size_t,
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn set_db_error(
    mut res: *mut http_response_t,
    mut result: db_event_destination_result_t,
    mut validation_error: *const ::core::ffi::c_char,
) {
    match result {
        db_event_destination_result_t::DB_EVENT_DESTINATION_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Event destination not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_event_destination_result_t::DB_EVENT_DESTINATION_CONFLICT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Event destination name or broker client ID already exists\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_event_destination_result_t::DB_EVENT_DESTINATION_STALE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Event destination was changed by another administrator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_event_destination_result_t::DB_EVENT_DESTINATION_LIMIT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Event destination limit reached\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_event_destination_result_t::DB_EVENT_DESTINATION_IN_USE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Event destination is referenced by an event route\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_event_destination_result_t::DB_EVENT_DESTINATION_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                if !validation_error.is_null()
                    && *validation_error.offset(0isize) as ::core::ffi::c_int != 0
                {
                    validation_error
                } else {
                    b"Invalid event destination\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Event destination operation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
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
            b"Failed to serialize event destination response\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    http_response_set_json(res, status, body);
    free(body as *mut ::core::ffi::c_void);
    return r#true != 0;
}
unsafe extern "C" fn destination_to_json(
    mut destination: *const event_destination_t,
) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_CreateObject();
    let mut broker: *mut cJSON = cJSON_CreateObject();
    let mut authentication: *mut cJSON = cJSON_CreateObject();
    let mut tls: *mut cJSON = cJSON_CreateObject();
    if object.is_null() || broker.is_null() || authentication.is_null() || tls.is_null() {
        cJSON_Delete(object);
        cJSON_Delete(broker);
        cJSON_Delete(authentication);
        cJSON_Delete(tls);
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut key: [::core::ffi::c_char; 64] = [
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
    if db_event_destination_make_key(
        &raw const (*destination).uuid as *const ::core::ffi::c_char,
        &raw mut key as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        cJSON_Delete(object);
        cJSON_Delete(broker);
        cJSON_Delete(authentication);
        cJSON_Delete(tls);
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*destination).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"key\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut key as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*destination).name as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"description\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*destination).description as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        object,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        (*destination).enabled as cJSON_bool,
    );
    cJSON_AddStringToObject(
        object,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*destination).destination_type as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        object,
        b"managed\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        broker,
        b"host\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*destination).broker_host as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        broker,
        b"port\0".as_ptr() as *const ::core::ffi::c_char,
        (*destination).broker_port as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        broker,
        b"client_id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*destination).client_id as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        broker,
        b"topic_template\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*destination).topic_template as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        broker,
        b"status_topic_template\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*destination).status_topic_template as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        broker,
        b"keepalive_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        (*destination).keepalive_seconds as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        broker,
        b"qos\0".as_ptr() as *const ::core::ffi::c_char,
        (*destination).qos as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        object,
        b"broker\0".as_ptr() as *const ::core::ffi::c_char,
        broker,
    );
    cJSON_AddStringToObject(
        authentication,
        b"username\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*destination).username as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        authentication,
        b"password_configured\0".as_ptr() as *const ::core::ffi::c_char,
        (*destination).password_configured as cJSON_bool,
    );
    cJSON_AddItemToObject(
        object,
        b"authentication\0".as_ptr() as *const ::core::ffi::c_char,
        authentication,
    );
    cJSON_AddStringToObject(
        tls,
        b"mode\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*destination).tls_mode as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        tls,
        b"ca_file\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*destination).ca_file as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        tls,
        b"cert_file\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*destination).cert_file as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        tls,
        b"key_file\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*destination).key_file as *const ::core::ffi::c_char,
    );
    cJSON_AddItemToObject(object, b"tls\0".as_ptr() as *const ::core::ffi::c_char, tls);
    cJSON_AddNumberToObject(
        object,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        (*destination).revision as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*destination).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*destination).updated_at as ::core::ffi::c_double,
    );
    return object;
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
unsafe extern "C" fn copy_string_field(
    mut object: *const cJSON,
    mut field: *const ::core::ffi::c_char,
    mut destination: *mut ::core::ffi::c_char,
    mut destination_size: size_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(object, field);
    if item.is_null() {
        return r#true != 0;
    }
    if cJSON_IsString(item) == 0
        || (*item).valuestring.is_null()
        || strlen((*item).valuestring) >= destination_size
    {
        let mut message: [::core::ffi::c_char; 160] = [0; 160];
        snprintf(
            &raw mut message as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
            b"%s must be a valid string\0".as_ptr() as *const ::core::ffi::c_char,
            field,
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
unsafe extern "C" fn apply_integer_field(
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
unsafe extern "C" fn initialize_defaults(mut destination: *mut event_destination_t) -> bool {
    memset(
        destination as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<event_destination_t>(),
    );
    (*destination).enabled = r#true != 0;
    safe_strcpy(
        &raw mut (*destination).destination_type as *mut ::core::ffi::c_char,
        b"mqtt\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    (*destination).broker_port = 8883 as ::core::ffi::c_int;
    let mut random_uuid: [::core::ffi::c_char; 37] = [0; 37];
    if lightnvr_uuid_generate_v4(&raw mut random_uuid as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut client_id_length: ::core::ffi::c_int = snprintf(
        &raw mut (*destination).client_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        b"lightnvr-%.8s%.4s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut random_uuid as *mut ::core::ffi::c_char,
        (&raw mut random_uuid as *mut ::core::ffi::c_char).offset(9 as ::core::ffi::c_int as isize),
    );
    if client_id_length < 0 as ::core::ffi::c_int
        || client_id_length as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
    {
        return r#false != 0;
    }
    safe_strcpy(
        &raw mut (*destination).topic_template as *mut ::core::ffi::c_char,
        EVENT_DESTINATION_DEFAULT_TOPIC_TEMPLATE.as_ptr(),
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*destination).tls_mode as *mut ::core::ffi::c_char,
        b"system\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    (*destination).keepalive_seconds = 60 as ::core::ffi::c_int;
    (*destination).qos = 1 as ::core::ffi::c_int;
    return r#true != 0;
}
unsafe extern "C" fn apply_broker(
    mut broker: *const cJSON,
    mut destination: *mut event_destination_t,
    mut res: *mut http_response_t,
) -> bool {
    let fields: [*const ::core::ffi::c_char; 7] = [
        b"host\0".as_ptr() as *const ::core::ffi::c_char,
        b"port\0".as_ptr() as *const ::core::ffi::c_char,
        b"client_id\0".as_ptr() as *const ::core::ffi::c_char,
        b"topic_template\0".as_ptr() as *const ::core::ffi::c_char,
        b"status_topic_template\0".as_ptr() as *const ::core::ffi::c_char,
        b"keepalive_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        b"qos\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    return object_has_only(
        broker,
        &raw const fields as *const *const ::core::ffi::c_char,
        7 as size_t,
        b"broker\0".as_ptr() as *const ::core::ffi::c_char,
        res,
    ) as ::core::ffi::c_int
        != 0
        && copy_string_field(
            broker,
            b"host\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*destination).broker_host as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            res,
        ) as ::core::ffi::c_int
            != 0
        && apply_integer_field(
            broker,
            b"port\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            65535 as ::core::ffi::c_int,
            &raw mut (*destination).broker_port,
            res,
        ) as ::core::ffi::c_int
            != 0
        && copy_string_field(
            broker,
            b"client_id\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*destination).client_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            res,
        ) as ::core::ffi::c_int
            != 0
        && copy_string_field(
            broker,
            b"topic_template\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*destination).topic_template as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            res,
        ) as ::core::ffi::c_int
            != 0
        && copy_string_field(
            broker,
            b"status_topic_template\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*destination).status_topic_template as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            res,
        ) as ::core::ffi::c_int
            != 0
        && apply_integer_field(
            broker,
            b"keepalive_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
            3600 as ::core::ffi::c_int,
            &raw mut (*destination).keepalive_seconds,
            res,
        ) as ::core::ffi::c_int
            != 0
        && apply_integer_field(
            broker,
            b"qos\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            &raw mut (*destination).qos,
            res,
        ) as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn apply_authentication(
    mut authentication: *const cJSON,
    mut destination: *mut event_destination_t,
    mut password: *mut ::core::ffi::c_char,
    mut replace_password: *mut bool,
    mut res: *mut http_response_t,
) -> bool {
    let fields: [*const ::core::ffi::c_char; 2] = [
        b"username\0".as_ptr() as *const ::core::ffi::c_char,
        b"password\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if !object_has_only(
        authentication,
        &raw const fields as *const *const ::core::ffi::c_char,
        2 as size_t,
        b"authentication\0".as_ptr() as *const ::core::ffi::c_char,
        res,
    ) || !copy_string_field(
        authentication,
        b"username\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*destination).username as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        res,
    ) {
        return r#false != 0;
    }
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        authentication,
        b"password\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if item.is_null() {
        return r#true != 0;
    }
    if cJSON_IsNull(item) != 0 {
        *password.offset(0isize) = '\0' as ::core::ffi::c_char;
    } else if cJSON_IsString(item) != 0
        && !(*item).valuestring.is_null()
        && strlen((*item).valuestring) < EVENT_DESTINATION_PASSWORD_MAX as size_t
    {
        safe_strcpy(
            password,
            (*item).valuestring,
            EVENT_DESTINATION_PASSWORD_MAX as size_t,
            0 as size_t,
        );
    } else {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"authentication.password must be a string or null\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *replace_password = r#true != 0;
    (*destination).password_configured =
        *password.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
    return r#true != 0;
}
unsafe extern "C" fn apply_tls(
    mut tls: *const cJSON,
    mut destination: *mut event_destination_t,
    mut res: *mut http_response_t,
) -> bool {
    let fields: [*const ::core::ffi::c_char; 4] = [
        b"mode\0".as_ptr() as *const ::core::ffi::c_char,
        b"ca_file\0".as_ptr() as *const ::core::ffi::c_char,
        b"cert_file\0".as_ptr() as *const ::core::ffi::c_char,
        b"key_file\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    return object_has_only(
        tls,
        &raw const fields as *const *const ::core::ffi::c_char,
        4 as size_t,
        b"tls\0".as_ptr() as *const ::core::ffi::c_char,
        res,
    ) as ::core::ffi::c_int
        != 0
        && copy_string_field(
            tls,
            b"mode\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*destination).tls_mode as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            res,
        ) as ::core::ffi::c_int
            != 0
        && copy_string_field(
            tls,
            b"ca_file\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*destination).ca_file as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            res,
        ) as ::core::ffi::c_int
            != 0
        && copy_string_field(
            tls,
            b"cert_file\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*destination).cert_file as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            res,
        ) as ::core::ffi::c_int
            != 0
        && copy_string_field(
            tls,
            b"key_file\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*destination).key_file as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            res,
        ) as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn parse_positive_revision(
    mut revision: *const cJSON,
    mut value: *mut int64_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut number: ::core::ffi::c_double = if cJSON_IsNumber(revision) != 0 {
        (*revision).valuedouble
    } else {
        0.0f64
    };
    if cJSON_IsNumber(revision) == 0
        || number < 1.0f64
        || number > 9007199254740991.0f64
        || number != number as int64_t as ::core::ffi::c_double
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"revision must be a positive integer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *value = number as int64_t;
    return r#true != 0;
}
unsafe extern "C" fn apply_body(
    mut body: *const cJSON,
    mut destination: *mut event_destination_t,
    mut creating: bool,
    mut expected_revision: *mut int64_t,
    mut password: *mut ::core::ffi::c_char,
    mut replace_password: *mut bool,
    mut res: *mut http_response_t,
) -> bool {
    let fields: [*const ::core::ffi::c_char; 8] = [
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        b"description\0".as_ptr() as *const ::core::ffi::c_char,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        b"broker\0".as_ptr() as *const ::core::ffi::c_char,
        b"authentication\0".as_ptr() as *const ::core::ffi::c_char,
        b"tls\0".as_ptr() as *const ::core::ffi::c_char,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if !object_has_only(
        body,
        &raw const fields as *const *const ::core::ffi::c_char,
        8 as size_t,
        b"request body\0".as_ptr() as *const ::core::ffi::c_char,
        res,
    ) {
        return r#false != 0;
    }
    if !copy_string_field(
        body,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*destination).name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        res,
    ) || !copy_string_field(
        body,
        b"description\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*destination).description as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        res,
    ) || !copy_string_field(
        body,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*destination).destination_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        res,
    ) {
        return r#false != 0;
    }
    let mut name: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"name\0".as_ptr() as *const ::core::ffi::c_char);
    let mut enabled: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"enabled\0".as_ptr() as *const ::core::ffi::c_char);
    let mut broker: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"broker\0".as_ptr() as *const ::core::ffi::c_char);
    let mut authentication: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"authentication\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut tls: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"tls\0".as_ptr() as *const ::core::ffi::c_char);
    let mut revision: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if creating as ::core::ffi::c_int != 0 && name.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Event destination name is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
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
        (*destination).enabled = cJSON_IsTrue(enabled) != 0;
    }
    if creating as ::core::ffi::c_int != 0 && broker.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"broker is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !broker.is_null() && !apply_broker(broker, destination, res)
        || !authentication.is_null()
            && !apply_authentication(authentication, destination, password, replace_password, res)
        || !tls.is_null() && !apply_tls(tls, destination, res)
    {
        return r#false != 0;
    }
    if creating {
        if !revision.is_null() {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"revision is assigned by the server\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        *replace_password = r#true != 0;
    } else if !parse_positive_revision(revision, expected_revision, res) {
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
    let mut validation: db_event_destination_result_t = db_event_destination_validate(
        destination,
        password,
        *replace_password,
        &raw mut validation_error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if validation.0 != db_event_destination_result_t::DB_EVENT_DESTINATION_OK.0 {
        set_db_error(
            res,
            validation,
            &raw mut validation_error as *mut ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn audit_destination(
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
        b"event_destination\0".as_ptr() as *const ::core::ffi::c_char,
        uuid,
        operation,
        outcome,
        context,
    );
    cJSON_Delete(context);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_event_destinations(
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
    let mut total: ::core::ffi::c_int = db_event_destination_count();
    if total < 0 as ::core::ffi::c_int {
        set_db_error(
            res,
            db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return;
    }
    if total > EVENT_DESTINATION_MAX_COUNT {
        set_db_error(
            res,
            db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return;
    }
    let mut destinations: *mut event_destination_t = (if total > 0 as ::core::ffi::c_int {
        calloc(
            total as size_t,
            ::core::mem::size_of::<event_destination_t>(),
        )
    } else {
        NULL
    }) as *mut event_destination_t;
    if total > 0 as ::core::ffi::c_int && destinations.is_null() {
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
        db_event_destination_list(destinations, total)
    };
    if count < 0 as ::core::ffi::c_int {
        free(destinations as *mut ::core::ffi::c_void);
        set_db_error(
            res,
            db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut legacy: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || legacy.is_null() || items.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(legacy);
        cJSON_Delete(items);
        free(destinations as *mut ::core::ffi::c_void);
        set_db_error(
            res,
            db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return;
    }
    cJSON_AddStringToObject(
        legacy,
        b"key\0".as_ptr() as *const ::core::ffi::c_char,
        b"mqtt:default\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        legacy,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        b"Default MQTT settings\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        legacy,
        b"configuration_source\0".as_ptr() as *const ::core::ffi::c_char,
        b"settings\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        legacy,
        b"managed\0".as_ptr() as *const ::core::ffi::c_char,
        r#false,
    );
    cJSON_AddItemToObject(
        root,
        b"default_destination\0".as_ptr() as *const ::core::ffi::c_char,
        legacy,
    );
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        count as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"destinations\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut item: *mut cJSON = destination_to_json(destinations.offset(index as isize));
        if item.is_null() {
            cJSON_Delete(root);
            free(destinations as *mut ::core::ffi::c_void);
            set_db_error(
                res,
                db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            return;
        }
        cJSON_AddItemToArray(items, item);
        index += 1;
    }
    free(destinations as *mut ::core::ffi::c_void);
    set_json_response(res, 200 as ::core::ffi::c_int, root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_event_destination(
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
    let mut destination: event_destination_t = event_destination_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        enabled: false,
        destination_type: [0; 16],
        broker_host: [0; 256],
        broker_port: 0,
        client_id: [0; 128],
        topic_template: [0; 512],
        status_topic_template: [0; 512],
        username: [0; 128],
        password_configured: false,
        tls_mode: [0; 16],
        ca_file: [0; 512],
        cert_file: [0; 512],
        key_file: [0; 512],
        keepalive_seconds: 0,
        qos: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    if !initialize_defaults(&raw mut destination) {
        cJSON_Delete(body);
        set_db_error(
            res,
            db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        audit_destination(
            req,
            &raw mut user,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"destination_create\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"default_generation_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut unused_revision: int64_t = 0 as int64_t;
    let mut password: [::core::ffi::c_char; 256] = [
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
    let mut replace_password: bool = r#false != 0;
    if !apply_body(
        body,
        &raw mut destination,
        r#true != 0,
        &raw mut unused_revision,
        &raw mut password as *mut ::core::ffi::c_char,
        &raw mut replace_password,
        res,
    ) {
        cJSON_Delete(body);
        secure_zero_memory(
            &raw mut password as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        );
        audit_destination(
            req,
            &raw mut user,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"destination_create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_Delete(body);
    let mut result: db_event_destination_result_t = db_event_destination_create(
        &raw mut destination,
        &raw mut password as *mut ::core::ffi::c_char,
    );
    secure_zero_memory(
        &raw mut password as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if result.0 != db_event_destination_result_t::DB_EVENT_DESTINATION_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        audit_destination(
            req,
            &raw mut user,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"destination_create\0".as_ptr() as *const ::core::ffi::c_char,
            if result.0 == db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            b"persistence_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_destination(
        req,
        &raw mut user,
        &raw mut destination.uuid as *mut ::core::ffi::c_char,
        b"destination_create\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"created\0".as_ptr() as *const ::core::ffi::c_char,
    );
    set_json_response(
        res,
        201 as ::core::ffi::c_int,
        destination_to_json(&raw mut destination),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_event_destination(
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
    if !extract_destination_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res) {
        return;
    }
    let mut destination: event_destination_t = event_destination_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        enabled: false,
        destination_type: [0; 16],
        broker_host: [0; 256],
        broker_port: 0,
        client_id: [0; 128],
        topic_template: [0; 512],
        status_topic_template: [0; 512],
        username: [0; 128],
        password_configured: false,
        tls_mode: [0; 16],
        ca_file: [0; 512],
        cert_file: [0; 512],
        key_file: [0; 512],
        keepalive_seconds: 0,
        qos: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_event_destination_result_t = db_event_destination_get(
        &raw mut uuid as *mut ::core::ffi::c_char,
        &raw mut destination,
    );
    if result.0 != db_event_destination_result_t::DB_EVENT_DESTINATION_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        return;
    }
    set_json_response(
        res,
        200 as ::core::ffi::c_int,
        destination_to_json(&raw mut destination),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_event_destination(
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
    if !extract_destination_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res) {
        return;
    }
    let mut destination: event_destination_t = event_destination_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        enabled: false,
        destination_type: [0; 16],
        broker_host: [0; 256],
        broker_port: 0,
        client_id: [0; 128],
        topic_template: [0; 512],
        status_topic_template: [0; 512],
        username: [0; 128],
        password_configured: false,
        tls_mode: [0; 16],
        ca_file: [0; 512],
        cert_file: [0; 512],
        key_file: [0; 512],
        keepalive_seconds: 0,
        qos: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_event_destination_result_t = db_event_destination_get(
        &raw mut uuid as *mut ::core::ffi::c_char,
        &raw mut destination,
    );
    if result.0 != db_event_destination_result_t::DB_EVENT_DESTINATION_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut expected_revision: int64_t = 0 as int64_t;
    let mut password: [::core::ffi::c_char; 256] = [
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
    let mut replace_password: bool = r#false != 0;
    if !apply_body(
        body,
        &raw mut destination,
        r#false != 0,
        &raw mut expected_revision,
        &raw mut password as *mut ::core::ffi::c_char,
        &raw mut replace_password,
        res,
    ) {
        cJSON_Delete(body);
        secure_zero_memory(
            &raw mut password as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        );
        audit_destination(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"destination_update\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_Delete(body);
    result = db_event_destination_update(
        &raw mut destination,
        expected_revision,
        &raw mut password as *mut ::core::ffi::c_char,
        replace_password,
    );
    secure_zero_memory(
        &raw mut password as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    if result.0 != db_event_destination_result_t::DB_EVENT_DESTINATION_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        audit_destination(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"destination_update\0".as_ptr() as *const ::core::ffi::c_char,
            if result.0 == db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            if result.0 == db_event_destination_result_t::DB_EVENT_DESTINATION_STALE.0 {
                b"stale_revision\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"persistence_failed\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    audit_destination(
        req,
        &raw mut user,
        &raw mut uuid as *mut ::core::ffi::c_char,
        b"destination_update\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"updated\0".as_ptr() as *const ::core::ffi::c_char,
    );
    set_json_response(
        res,
        200 as ::core::ffi::c_int,
        destination_to_json(&raw mut destination),
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
pub unsafe extern "C" fn handle_delete_event_destination(
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
    if !extract_destination_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res)
        || !parse_delete_revision(req, &raw mut revision, res)
    {
        return;
    }
    let mut result: db_event_destination_result_t =
        db_event_destination_delete(&raw mut uuid as *mut ::core::ffi::c_char, revision);
    if result.0 != db_event_destination_result_t::DB_EVENT_DESTINATION_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        audit_destination(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"destination_delete\0".as_ptr() as *const ::core::ffi::c_char,
            if result.0 == db_event_destination_result_t::DB_EVENT_DESTINATION_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            if result.0 == db_event_destination_result_t::DB_EVENT_DESTINATION_STALE.0 {
                b"stale_revision\0".as_ptr() as *const ::core::ffi::c_char
            } else if result.0 == db_event_destination_result_t::DB_EVENT_DESTINATION_IN_USE.0 {
                b"destination_in_use\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"persistence_failed\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    audit_destination(
        req,
        &raw mut user,
        &raw mut uuid as *mut ::core::ffi::c_char,
        b"destination_delete\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"deleted\0".as_ptr() as *const ::core::ffi::c_char,
    );
    http_response_set_json(
        res,
        200 as ::core::ffi::c_int,
        b"{\"success\":true}\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
