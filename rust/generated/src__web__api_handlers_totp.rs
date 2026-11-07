unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddBoolToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        boolean: cJSON_bool,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddStringToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn totp_generate_secret(
        secret: *mut ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn curl_easy_escape(
        curl: *mut ::core::ffi::c_void,
        string: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn curl_free(p: *mut ::core::ffi::c_void);
    unsafe fn http_response_add_header(
        res: *mut http_response_t,
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
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
    unsafe fn db_auth_get_user_by_id(user_id: int64_t, user: *mut user_t) -> ::core::ffi::c_int;
    unsafe fn db_auth_verify_totp_enrollment(
        user_id: int64_t,
        code: *const ::core::ffi::c_char,
        keep_session: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_disable_totp(
        user_id: int64_t,
        keep_session: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_complete_totp_login(
        user_id: int64_t,
        code: *const ::core::ffi::c_char,
        challenge: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        ip: *const ::core::ffi::c_char,
        agent: *const ::core::ffi::c_char,
        trusted_seconds: ::core::ffi::c_int,
        session: *mut ::core::ffi::c_char,
        trusted: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_validate_mfa_challenge(
        token: *const ::core::ffi::c_char,
        user_id: *mut int64_t,
        ip_address: *const ::core::ffi::c_char,
        user_agent: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_delete_session(token: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn db_auth_get_totp_info(
        user_id: int64_t,
        secret: *mut ::core::ffi::c_char,
        secret_size: size_t,
        enabled: *mut bool,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_set_totp_secret(
        user_id: int64_t,
        secret: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_auth_ip_allowed_for_user(
        user: *const user_t,
        client_ip: *const ::core::ffi::c_char,
    ) -> bool;
    unsafe fn authorization_evaluate(
        user: *const user_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_get_effective_client_ip(
        req: *const http_request_t,
        client_ip: *mut ::core::ffi::c_char,
        client_ip_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_get_session_token(
        req: *const http_request_t,
        token: *mut ::core::ffi::c_char,
        token_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_trusted_device_lifetime_seconds() -> ::core::ffi::c_int;
    unsafe fn httpd_add_session_cookie(
        res: *mut http_response_t,
        token: *const ::core::ffi::c_char,
    );
    unsafe fn httpd_add_trusted_device_cookie(
        res: *mut http_response_t,
        token: *const ::core::ffi::c_char,
    );
    unsafe fn httpd_clear_trusted_device_cookie(res: *mut http_response_t);
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
    ) -> ::core::ffi::c_int;
    unsafe fn audit_log_login(
        req: *const http_request_t,
        user: *const user_t,
        attempted_username: *const ::core::ffi::c_char,
        authentication_method: *const ::core::ffi::c_char,
        outcome: *const ::core::ffi::c_char,
        reason: *const ::core::ffi::c_char,
    );
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type int64_t = i64;
pub type uint64_t = u64;
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
pub type CURL = ();
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
pub struct authorization_decision_t(pub ::core::ffi::c_uint);
impl authorization_decision_t {
    pub const AUTHZ_DECISION_DENY: Self = Self(0);
    pub const AUTHZ_DECISION_ALLOW: Self = Self(1);
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_int);
impl C2Rust_Unnamed {
    pub const DB_AUTH_IDENTITY_ERROR: Self = Self(-1);
    pub const DB_AUTH_IDENTITY_INVALID: Self = Self(-2);
    pub const DB_AUTH_IDENTITY_LIMITED: Self = Self(-3);
    pub const DB_AUTH_IDENTITY_CONFLICT: Self = Self(-4);
}
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const LOG_COMPONENT: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"AuthAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
unsafe extern "C" fn extract_totp_user_id(
    mut req: *const http_request_t,
    mut suffix: *const ::core::ffi::c_char,
    mut user_id: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut prefix: *const ::core::ffi::c_char =
        b"/api/auth/users/\0".as_ptr() as *const ::core::ffi::c_char;
    if req.is_null()
        || strncmp(
            &raw const (*req).path as *const ::core::ffi::c_char,
            prefix,
            strlen(prefix),
        ) != 0
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut text: *const ::core::ffi::c_char =
        (&raw const (*req).path as *const ::core::ffi::c_char).offset(strlen(prefix) as isize);
    if (*text as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
        || *text as ::core::ffi::c_int > '9' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut value: ::core::ffi::c_longlong = strtoll(text, &raw mut end, 10 as ::core::ffi::c_int);
    if *__errno_location() == ERANGE
        || value <= 0 as ::core::ffi::c_longlong
        || value > INT64_MAX as ::core::ffi::c_longlong
        || strncmp(end, suffix, strlen(suffix)) != 0
    {
        return -1 as ::core::ffi::c_int;
    }
    end = end.offset(strlen(suffix) as isize);
    while *end as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
        end = end.offset(1);
    }
    if *end as ::core::ffi::c_int != 0 && *end as ::core::ffi::c_int != '?' as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    *user_id = value as int64_t;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn identity_error(mut res: *mut http_response_t, mut result: ::core::ffi::c_int) {
    if result == C2Rust_Unnamed::DB_AUTH_IDENTITY_LIMITED.0 {
        http_response_set_json_error(
            res,
            429 as ::core::ffi::c_int,
            b"Too many TOTP attempts. Please try again later.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if result == C2Rust_Unnamed::DB_AUTH_IDENTITY_CONFLICT.0 {
        http_response_set_json_error(
            res,
            409 as ::core::ffi::c_int,
            b"TOTP state changed or setup expired. Check status and start setup again if needed.\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    } else if result == C2Rust_Unnamed::DB_AUTH_IDENTITY_INVALID.0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Invalid or already used TOTP code\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to update authentication state\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn current_owner_session(
    mut req: *const http_request_t,
    mut current: *const user_t,
    mut target: int64_t,
    mut session: *mut ::core::ffi::c_char,
) {
    *session.offset(0isize) = '\0' as ::core::ffi::c_char;
    if (*current).id == target && !(*current).authenticated_via_scoped_token {
        httpd_get_session_token(req, session, 64 as size_t);
    }
}
unsafe extern "C" fn check_totp_permission(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut target_user_id: int64_t,
    mut current_user: *mut user_t,
) -> ::core::ffi::c_int {
    if httpd_check_action_access(req, current_user) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
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
    let mut can_manage_users: bool = authorization_evaluate(
        current_user,
        authorization_action_t::AUTHZ_USERS_MANAGE,
        ::core::ptr::null::<fleet_camera_t>(),
        &raw mut evaluation,
    ) == 0 as ::core::ffi::c_int
        && evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0;
    let mut is_session_self: bool =
        !(*current_user).authenticated_via_scoped_token && (*current_user).id == target_user_id;
    if !can_manage_users && !is_session_self {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Forbidden: Can only manage your own MFA\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_totp_setup(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/auth/users/:id/totp/setup\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut target_user_id: int64_t = 0;
    if extract_totp_user_id(
        req,
        b"/totp/setup\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut target_user_id,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid user ID in path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut current_user: user_t = user_t {
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
    if check_totp_permission(req, res, target_user_id, &raw mut current_user) == 0 {
        return;
    }
    let mut target_user: user_t = user_t {
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
    if db_auth_get_user_by_id(target_user_id, &raw mut target_user) != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"User not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut secret_b32: [::core::ffi::c_char; 64] = [
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
    if totp_generate_secret(
        &raw mut secret_b32 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to generate TOTP secret\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut otpauth_uri: [::core::ffi::c_char; 512] = [0; 512];
    let mut label: *mut ::core::ffi::c_char = curl_easy_escape(
        NULL,
        &raw mut target_user.username as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    if label.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to encode TOTP label\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    snprintf(
        &raw mut otpauth_uri as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"otpauth://totp/LightNVR:%s?secret=%s&issuer=LightNVR&algorithm=SHA1&digits=6&period=30\0"
            .as_ptr() as *const ::core::ffi::c_char,
        label,
        &raw mut secret_b32 as *mut ::core::ffi::c_char,
    );
    curl_free(label as *mut ::core::ffi::c_void);
    let mut result: ::core::ffi::c_int = db_auth_set_totp_secret(
        target_user_id,
        &raw mut secret_b32 as *mut ::core::ffi::c_char,
    );
    if result != 0 as ::core::ffi::c_int {
        identity_error(res, result);
        return;
    }
    http_response_add_header(
        res,
        b"Cache-Control\0".as_ptr() as *const ::core::ffi::c_char,
        b"private, no-store\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"secret\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut secret_b32 as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        response,
        b"otpauth_uri\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut otpauth_uri as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"Scan the QR code with your authenticator app, then verify with a code\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"TOTP setup initiated for user %lld (%s)\0".as_ptr() as *const ::core::ffi::c_char,
        target_user_id as ::core::ffi::c_longlong,
        &raw mut target_user.username as *mut ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_totp_verify(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/auth/users/:id/totp/verify\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut target_user_id: int64_t = 0;
    if extract_totp_user_id(
        req,
        b"/totp/verify\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut target_user_id,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid user ID in path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut current_user: user_t = user_t {
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
    if check_totp_permission(req, res, target_user_id, &raw mut current_user) == 0 {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    if body.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut code_json: *mut cJSON =
        cJSON_GetObjectItem(body, b"code\0".as_ptr() as *const ::core::ffi::c_char);
    if code_json.is_null() || cJSON_IsString(code_json) == 0 {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing or invalid 'code' field\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut keep_session: [::core::ffi::c_char; 64] = [0; 64];
    current_owner_session(
        req,
        &raw mut current_user,
        target_user_id,
        &raw mut keep_session as *mut ::core::ffi::c_char,
    );
    let mut result: ::core::ffi::c_int = db_auth_verify_totp_enrollment(
        target_user_id,
        (*code_json).valuestring,
        &raw mut keep_session as *mut ::core::ffi::c_char,
    );
    cJSON_Delete(body);
    if result != 0 as ::core::ffi::c_int {
        identity_error(res, result);
        return;
    }
    if current_user.id == target_user_id {
        httpd_clear_trusted_device_cookie(res);
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"TOTP MFA enabled successfully\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"TOTP enabled for user %lld\0".as_ptr() as *const ::core::ffi::c_char,
        target_user_id as ::core::ffi::c_longlong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_totp_disable(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/auth/users/:id/totp/disable\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut target_user_id: int64_t = 0;
    if extract_totp_user_id(
        req,
        b"/totp/disable\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut target_user_id,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid user ID in path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut current_user: user_t = user_t {
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
    if check_totp_permission(req, res, target_user_id, &raw mut current_user) == 0 {
        return;
    }
    let mut keep_session: [::core::ffi::c_char; 64] = [0; 64];
    current_owner_session(
        req,
        &raw mut current_user,
        target_user_id,
        &raw mut keep_session as *mut ::core::ffi::c_char,
    );
    let mut result: ::core::ffi::c_int = db_auth_disable_totp(
        target_user_id,
        &raw mut keep_session as *mut ::core::ffi::c_char,
    );
    if result != 0 as ::core::ffi::c_int {
        identity_error(res, result);
        return;
    }
    if current_user.id == target_user_id {
        httpd_clear_trusted_device_cookie(res);
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"TOTP MFA disabled successfully\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"TOTP disabled for user %lld\0".as_ptr() as *const ::core::ffi::c_char,
        target_user_id as ::core::ffi::c_longlong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_totp_status(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/auth/users/:id/totp/status\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut target_user_id: int64_t = 0;
    if extract_totp_user_id(
        req,
        b"/totp/status\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut target_user_id,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid user ID in path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut current_user: user_t = user_t {
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
    if check_totp_permission(req, res, target_user_id, &raw mut current_user) == 0 {
        return;
    }
    let mut secret: [::core::ffi::c_char; 64] = [
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
    let mut enabled: bool = r#false != 0;
    let mut target_user: user_t = user_t {
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
    if db_auth_get_user_by_id(target_user_id, &raw mut target_user) != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"User not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if db_auth_get_totp_info(
        target_user_id,
        &raw mut secret as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        &raw mut enabled,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to read TOTP status\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_add_header(
        res,
        b"Cache-Control\0".as_ptr() as *const ::core::ffi::c_char,
        b"private, no-store\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"totp_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        enabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        response,
        b"totp_configured\0".as_ptr() as *const ::core::ffi::c_char,
        (secret[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_auth_login_totp(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/auth/login/totp\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut effective_client_ip: [::core::ffi::c_char; 64] = [
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
    if httpd_get_effective_client_ip(
        req,
        &raw mut effective_client_ip as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) != 0 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut effective_client_ip as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*req).client_ip as *const ::core::ffi::c_char,
        );
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    if body.is_null() {
        audit_log_login(
            req,
            ::core::ptr::null::<user_t>(),
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"password_totp\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut token_json: *mut cJSON =
        cJSON_GetObjectItem(body, b"totp_token\0".as_ptr() as *const ::core::ffi::c_char);
    let mut code_json: *mut cJSON =
        cJSON_GetObjectItem(body, b"code\0".as_ptr() as *const ::core::ffi::c_char);
    let mut remember_device_json: *mut cJSON = cJSON_GetObjectItem(
        body,
        b"remember_device\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut remember_device: bool = !remember_device_json.is_null()
        && cJSON_IsBool(remember_device_json) != 0
        && cJSON_IsTrue(remember_device_json) != 0;
    if token_json.is_null()
        || cJSON_IsString(token_json) == 0
        || code_json.is_null()
        || cJSON_IsString(code_json) == 0
    {
        cJSON_Delete(body);
        audit_log_login(
            req,
            ::core::ptr::null::<user_t>(),
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"password_totp\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing 'totp_token' or 'code'\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut user_id: int64_t = 0;
    let mut rc: ::core::ffi::c_int = db_auth_validate_mfa_challenge(
        (*token_json).valuestring,
        &raw mut user_id,
        &raw mut effective_client_ip as *mut ::core::ffi::c_char,
        &raw const (*req).user_agent as *const ::core::ffi::c_char,
    );
    if rc != 0 as ::core::ffi::c_int {
        cJSON_Delete(body);
        audit_log_login(
            req,
            ::core::ptr::null::<user_t>(),
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            b"password_totp\0".as_ptr() as *const ::core::ffi::c_char,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_mfa_session\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Invalid or expired MFA token. Please login again.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut challenge_owner: user_t = user_t {
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
    if db_auth_get_user_by_id(user_id, &raw mut challenge_owner) != 0 as ::core::ffi::c_int
        || !db_auth_ip_allowed_for_user(
            &raw mut challenge_owner,
            &raw mut effective_client_ip as *mut ::core::ffi::c_char,
        )
    {
        db_auth_delete_session((*token_json).valuestring);
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Invalid or expired MFA token. Please login again.\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut session_token: [::core::ffi::c_char; 33] = [0; 33];
    let mut trusted_token: [::core::ffi::c_char; 33] = [0; 33];
    let mut trusted_seconds: ::core::ffi::c_int = if remember_device as ::core::ffi::c_int != 0 {
        httpd_trusted_device_lifetime_seconds()
    } else {
        0 as ::core::ffi::c_int
    };
    rc = db_auth_complete_totp_login(
        user_id,
        (*code_json).valuestring,
        (*token_json).valuestring,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut effective_client_ip as *mut ::core::ffi::c_char,
        &raw const (*req).user_agent as *const ::core::ffi::c_char,
        trusted_seconds,
        &raw mut session_token as *mut ::core::ffi::c_char,
        &raw mut trusted_token as *mut ::core::ffi::c_char,
    );
    cJSON_Delete(body);
    if rc != 0 as ::core::ffi::c_int {
        audit_log_login(
            req,
            &raw mut challenge_owner,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"password_totp\0".as_ptr() as *const ::core::ffi::c_char,
            b"denied\0".as_ptr() as *const ::core::ffi::c_char,
            if rc == C2Rust_Unnamed::DB_AUTH_IDENTITY_LIMITED.0 {
                b"rate_limited\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"invalid_mfa_code\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        identity_error(res, rc);
        return;
    }
    httpd_add_session_cookie(res, &raw mut session_token as *mut ::core::ffi::c_char);
    if trusted_token[0usize] != 0 {
        httpd_add_trusted_device_cookie(res, &raw mut trusted_token as *mut ::core::ffi::c_char);
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"redirect\0".as_ptr() as *const ::core::ffi::c_char,
        b"/index.html\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"TOTP login completed for user ID %lld\0".as_ptr() as *const ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
    );
    let mut user: user_t = user_t {
        id: user_id,
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
    db_auth_get_user_by_id(user_id, &raw mut user);
    audit_log_login(
        req,
        &raw mut user,
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"password_totp\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"session_created\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
