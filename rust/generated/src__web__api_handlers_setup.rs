unsafe extern "C" {
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
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
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
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
    unsafe fn httpd_authorize_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        user: *mut user_t,
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
    unsafe fn db_set_system_setting(
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_is_setup_complete() -> bool;
    unsafe fn db_get_setup_status(
        complete: *mut bool,
        completed_at: *mut ::core::ffi::c_longlong,
    ) -> ::core::ffi::c_int;
    unsafe fn db_mark_setup_complete() -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct api_phase_timer_t {
    pub started: timespec,
    pub request: *const http_request_t,
    pub phase: *const ::core::ffi::c_char,
    pub valid: bool,
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
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOG_COMPONENT: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"SetupAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[inline]
unsafe extern "C" fn api_phase_begin(
    mut request: *const http_request_t,
    mut phase: *const ::core::ffi::c_char,
) -> api_phase_timer_t {
    let mut timer: api_phase_timer_t = api_phase_timer_t {
        started: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        request: request,
        phase: phase,
        valid: false,
    };
    timer.valid = clock_gettime(CLOCK_MONOTONIC, &raw mut timer.started) == 0 as ::core::ffi::c_int;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"API phase starting: path=%s phase=%s request_id=%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw const (*request).path as *const ::core::ffi::c_char,
        phase,
        if (*request).request_id[0usize] as ::core::ffi::c_int != 0 {
            &raw const (*request).request_id as *const ::core::ffi::c_char
        } else {
            b"-\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return timer;
}
#[inline]
unsafe extern "C" fn api_phase_end(mut timer: *const api_phase_timer_t) {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if !(*timer).valid || clock_gettime(CLOCK_MONOTONIC, &raw mut now) != 0 as ::core::ffi::c_int {
        return;
    }
    let mut elapsed_ms: ::core::ffi::c_double =
        (now.tv_sec - (*timer).started.tv_sec) as ::core::ffi::c_double * 1000.0f64
            + (now.tv_nsec - (*timer).started.tv_nsec) as ::core::ffi::c_double / 1000000.0f64;
    if elapsed_ms >= 250.0f64 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Slow API phase: path=%s phase=%s duration_ms=%.0f request_id=%s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw const (*(*timer).request).path as *const ::core::ffi::c_char,
            (*timer).phase,
            elapsed_ms,
            if (*(*timer).request).request_id[0usize] as ::core::ffi::c_int != 0 {
                &raw const (*(*timer).request).request_id as *const ::core::ffi::c_char
            } else {
                b"-\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_setup_status(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut complete: bool = r#false != 0;
    let mut completed_at: ::core::ffi::c_longlong = 0 as ::core::ffi::c_longlong;
    let mut phase: api_phase_timer_t = api_phase_begin(
        req,
        b"setup status database read\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut result: ::core::ffi::c_int =
        db_get_setup_status(&raw mut complete, &raw mut completed_at);
    api_phase_end(&raw mut phase);
    if result != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            503 as ::core::ffi::c_int,
            b"Setup status temporarily unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut obj: *mut cJSON = cJSON_CreateObject();
    if obj.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to allocate JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        obj,
        b"complete\0".as_ptr() as *const ::core::ffi::c_char,
        complete as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        obj,
        b"setup_completed_at\0".as_ptr() as *const ::core::ffi::c_char,
        completed_at as ::core::ffi::c_double,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(obj);
    cJSON_Delete(obj);
    if json_str.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialise JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_setup_complete(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut mark_complete: bool = r#true != 0;
    let mut was_complete: bool = db_is_setup_complete();
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
    memset(
        &raw mut user as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    if was_complete {
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
            authorization_action_t::AUTHZ_SYSTEM_ADMIN,
            ::core::ptr::null::<fleet_camera_t>(),
            &raw mut user,
            &raw mut evaluation,
        ) == 0
        {
            return;
        }
    }
    if !(*req).body.is_null() && (*req).body_len > 0 as size_t {
        let mut body: *mut cJSON =
            cJSON_ParseWithLength((*req).body as *const ::core::ffi::c_char, (*req).body_len);
        if !body.is_null() {
            let mut complete_j: *mut cJSON =
                cJSON_GetObjectItem(body, b"complete\0".as_ptr() as *const ::core::ffi::c_char);
            if cJSON_IsBool(complete_j) != 0 {
                mark_complete = cJSON_IsTrue(complete_j) != 0;
            }
            cJSON_Delete(body);
        }
    }
    let mut rc: ::core::ffi::c_int = 0;
    if mark_complete {
        rc = db_mark_setup_complete();
    } else {
        rc = db_set_system_setting(
            b"setup_complete\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if rc == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Setup wizard reset \xE2\x80\x94 will show on next page load\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if rc != 0 as ::core::ffi::c_int {
        if was_complete {
            audit_log_operation(
                req,
                &raw mut user,
                b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
                b"system\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                if mark_complete as ::core::ffi::c_int != 0 {
                    b"setup.complete\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"setup.reset\0".as_ptr() as *const ::core::ffi::c_char
                },
                b"error\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<cJSON>(),
            );
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"handle_post_setup_complete: failed to persist setup state\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to save setup state\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if was_complete {
        audit_log_operation(
            req,
            &raw mut user,
            b"system.admin\0".as_ptr() as *const ::core::ffi::c_char,
            b"system\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            if mark_complete as ::core::ffi::c_int != 0 {
                b"setup.complete\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"setup.reset\0".as_ptr() as *const ::core::ffi::c_char
            },
            b"success\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<cJSON>(),
        );
    }
    let mut obj: *mut cJSON = cJSON_CreateObject();
    if obj.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to allocate JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        obj,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(obj);
    cJSON_Delete(obj);
    if json_str.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialise JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
