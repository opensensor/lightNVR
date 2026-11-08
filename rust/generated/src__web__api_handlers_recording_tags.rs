unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateString(string: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_CreateArray() -> *mut cJSON;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_AddItemToObject(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        item: *mut cJSON,
    ) -> cJSON_bool;
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
    unsafe fn get_recording_metadata_by_id(
        id: uint64_t,
        metadata: *mut recording_metadata_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_unique_detection_labels_for_cameras(
        camera_uuids: *const *const ::core::ffi::c_char,
        camera_count: ::core::ffi::c_int,
        labels: *mut [::core::ffi::c_char; 32],
        max_labels: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_recording_tag_get(
        recording_id: uint64_t,
        tags: *mut [::core::ffi::c_char; 128],
        max_tags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_recording_tag_set(
        recording_id: uint64_t,
        tags: *mut *const ::core::ffi::c_char,
        tag_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_recording_tag_get_unique_for_cameras(
        camera_uuids: *const *const ::core::ffi::c_char,
        camera_count: ::core::ffi::c_int,
        tags: *mut [::core::ffi::c_char; 128],
        max_tags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_recording_tag_batch_add(
        recording_ids: *const uint64_t,
        count: ::core::ffi::c_int,
        tag: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_recording_tag_batch_remove(
        recording_ids: *const uint64_t,
        count: ::core::ffi::c_int,
        tag: *const ::core::ffi::c_char,
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
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
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
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const LOG_COMPONENT: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"RecordingsAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_UNIQUE_DETECTION_LABELS: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const MAX_RECORDING_TAGS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
unsafe extern "C" fn audit_recording_tag_operation(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut target_uuid: *const ::core::ffi::c_char,
    mut operation: *const ::core::ffi::c_char,
    mut outcome: *const ::core::ffi::c_char,
    mut requested_count: ::core::ffi::c_int,
    mut changed_count: ::core::ffi::c_int,
    mut reason: *const ::core::ffi::c_char,
) {
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddNumberToObject(
            details,
            b"requested_count\0".as_ptr() as *const ::core::ffi::c_char,
            requested_count as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            details,
            b"changed_count\0".as_ptr() as *const ::core::ffi::c_char,
            changed_count as ::core::ffi::c_double,
        );
        if !reason.is_null() {
            cJSON_AddStringToObject(
                details,
                b"reason\0".as_ptr() as *const ::core::ffi::c_char,
                reason,
            );
        }
    }
    audit_log_operation(
        req,
        user,
        b"evidence.protect\0".as_ptr() as *const ::core::ffi::c_char,
        if !target_uuid.is_null() {
            b"recording\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"recording_batch\0".as_ptr() as *const ::core::ffi::c_char
        },
        target_uuid,
        operation,
        outcome,
        details,
    );
    cJSON_Delete(details);
}
unsafe extern "C" fn load_replay_authorized_cameras(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut cameras: *mut *mut fleet_camera_t,
    mut stream_names: *mut *const ::core::ffi::c_char,
    mut stream_count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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
    *cameras = ::core::ptr::null_mut::<fleet_camera_t>();
    *stream_count = 0 as ::core::ffi::c_int;
    if httpd_check_action_access(req, &raw mut user) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if db_fleet_camera_load(cameras, &raw mut camera_count) != 0 as ::core::ffi::c_int
        || authorization_filter_cameras(
            &raw mut user,
            authorization_action_t::AUTHZ_RECORDINGS_REPLAY,
            *cameras,
            &raw mut camera_count,
        ) != 0 as ::core::ffi::c_int
    {
        free(*cameras as *mut ::core::ffi::c_void);
        *cameras = ::core::ptr::null_mut::<fleet_camera_t>();
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if camera_count > MAX_STREAMS {
        free(*cameras as *mut ::core::ffi::c_void);
        *cameras = ::core::ptr::null_mut::<fleet_camera_t>();
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Camera scope exceeds supported limit\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < camera_count {
        *stream_names.offset(i as isize) =
            &raw mut (*(*cameras).offset(i as isize)).camera_uuid as *mut ::core::ffi::c_char;
        i += 1;
    }
    *stream_count = camera_count;
    return 1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_recording_tags(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/recordings/tags\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut stream_names: [*const ::core::ffi::c_char; 1024] =
        [::core::ptr::null::<::core::ffi::c_char>(); 1024];
    let mut stream_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if load_replay_authorized_cameras(
        req,
        res,
        &raw mut cameras,
        &raw mut stream_names as *mut *const ::core::ffi::c_char,
        &raw mut stream_count,
    ) == 0
    {
        return;
    }
    let mut tags: [[::core::ffi::c_char; 128]; 64] = [[0; 128]; 64];
    let mut count: ::core::ffi::c_int = db_recording_tag_get_unique_for_cameras(
        &raw mut stream_names as *mut *const ::core::ffi::c_char,
        stream_count,
        &raw mut tags as *mut [::core::ffi::c_char; 128],
        MAX_RECORDING_TAGS,
    );
    free(cameras as *mut ::core::ffi::c_void);
    if count < 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get tags\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    let mut arr: *mut cJSON = cJSON_CreateArray();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        cJSON_AddItemToArray(
            arr,
            cJSON_CreateString(
                &raw mut *(&raw mut tags as *mut [::core::ffi::c_char; 128]).offset(i as isize)
                    as *mut ::core::ffi::c_char,
            ),
        );
        i += 1;
    }
    cJSON_AddItemToObject(
        response,
        b"tags\0".as_ptr() as *const ::core::ffi::c_char,
        arr,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_recording_detection_labels(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/recordings/detection-labels\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut stream_names: [*const ::core::ffi::c_char; 1024] =
        [::core::ptr::null::<::core::ffi::c_char>(); 1024];
    let mut stream_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if load_replay_authorized_cameras(
        req,
        res,
        &raw mut cameras,
        &raw mut stream_names as *mut *const ::core::ffi::c_char,
        &raw mut stream_count,
    ) == 0
    {
        return;
    }
    let mut labels: [[::core::ffi::c_char; 32]; 128] = [[0; 32]; 128];
    let mut count: ::core::ffi::c_int = get_unique_detection_labels_for_cameras(
        &raw mut stream_names as *mut *const ::core::ffi::c_char,
        stream_count,
        &raw mut labels as *mut [::core::ffi::c_char; 32],
        MAX_UNIQUE_DETECTION_LABELS,
    );
    free(cameras as *mut ::core::ffi::c_void);
    if count < 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get detection labels\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    let mut arr: *mut cJSON = cJSON_CreateArray();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        cJSON_AddItemToArray(
            arr,
            cJSON_CreateString(
                &raw mut *(&raw mut labels as *mut [::core::ffi::c_char; 32]).offset(i as isize)
                    as *mut ::core::ffi::c_char,
            ),
        );
        i += 1;
    }
    cJSON_AddItemToObject(
        response,
        b"labels\0".as_ptr() as *const ::core::ffi::c_char,
        arr,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_recording_tags_by_id(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/recordings/:id/tags\0".as_ptr() as *const ::core::ffi::c_char,
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
        b"/tags\0".as_ptr() as *const ::core::ffi::c_char,
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
    let mut recording: recording_metadata_t = recording_metadata_t {
        id: 0 as uint64_t,
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
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Recording not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut c2rust_lvalue: user_t = user_t {
        id: 0 as int64_t,
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
    let mut c2rust_lvalue_0: fleet_camera_t = fleet_camera_t {
        camera_uuid: [
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
        ],
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
    let mut c2rust_lvalue_1: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
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
        &raw mut recording.camera_uuid as *mut ::core::ffi::c_char,
        &raw mut recording.stream_name as *mut ::core::ffi::c_char,
        &raw mut c2rust_lvalue,
        &raw mut c2rust_lvalue_0,
        &raw mut c2rust_lvalue_1,
    ) == 0
    {
        return;
    }
    let mut tags: [[::core::ffi::c_char; 128]; 64] = [[0; 128]; 64];
    let mut count: ::core::ffi::c_int = db_recording_tag_get(
        id,
        &raw mut tags as *mut [::core::ffi::c_char; 128],
        MAX_RECORDING_TAGS,
    );
    if count < 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get tags for recording\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddNumberToObject(
        response,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        id as ::core::ffi::c_double,
    );
    let mut arr: *mut cJSON = cJSON_CreateArray();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        cJSON_AddItemToArray(
            arr,
            cJSON_CreateString(
                &raw mut *(&raw mut tags as *mut [::core::ffi::c_char; 128]).offset(i as isize)
                    as *mut ::core::ffi::c_char,
            ),
        );
        i += 1;
    }
    cJSON_AddItemToObject(
        response,
        b"tags\0".as_ptr() as *const ::core::ffi::c_char,
        arr,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_recording_tags(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling PUT /api/recordings/:id/tags\0".as_ptr() as *const ::core::ffi::c_char,
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
        b"/tags\0".as_ptr() as *const ::core::ffi::c_char,
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
    let mut recording: recording_metadata_t = recording_metadata_t {
        id: 0 as uint64_t,
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
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Recording not found\0".as_ptr() as *const ::core::ffi::c_char,
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
        authorization_action_t::AUTHZ_EVIDENCE_PROTECT,
        &raw mut recording.camera_uuid as *mut ::core::ffi::c_char,
        &raw mut recording.stream_name as *mut ::core::ffi::c_char,
        &raw mut user,
        &raw mut camera,
        &raw mut evaluation,
    ) == 0
    {
        return;
    }
    let mut recording_uuid: [::core::ffi::c_char; 32] = [0; 32];
    snprintf(
        &raw mut recording_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%llu\0".as_ptr() as *const ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
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
    let mut tags_json: *mut cJSON =
        cJSON_GetObjectItem(json, b"tags\0".as_ptr() as *const ::core::ffi::c_char);
    if tags_json.is_null() || cJSON_IsArray(tags_json) == 0 {
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing or invalid 'tags' field (array required)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut tag_count: ::core::ffi::c_int = cJSON_GetArraySize(tags_json);
    let mut tag_strs: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    if tag_count > 0 as ::core::ffi::c_int {
        tag_strs = malloc(
            (tag_count as size_t).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
        ) as *mut *const ::core::ffi::c_char;
        if tag_strs.is_null() {
            cJSON_Delete(json);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Memory allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut valid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut item: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
        item = (if !tags_json.is_null() {
            (*tags_json).child
        } else {
            ::core::ptr::null_mut::<cJSON>()
        }) as *mut cJSON;
        while !item.is_null() {
            if cJSON_IsString(item) != 0
                && *(*item).valuestring.offset(0isize) as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
            {
                let c2rust_fresh0 = valid;
                valid += 1;
                *tag_strs.offset(c2rust_fresh0 as isize) = (*item).valuestring;
            }
            item = (*item).next as *mut cJSON;
        }
        tag_count = valid;
    }
    let mut rc: ::core::ffi::c_int = db_recording_tag_set(id, tag_strs, tag_count);
    free(tag_strs as *mut ::core::ffi::c_void);
    cJSON_Delete(json);
    if rc != 0 as ::core::ffi::c_int {
        audit_recording_tag_operation(
            req,
            &raw mut user,
            &raw mut recording_uuid as *mut ::core::ffi::c_char,
            b"recording_tags.replace\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            tag_count,
            0 as ::core::ffi::c_int,
            b"database_update_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to set tags\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut result_tags: [[::core::ffi::c_char; 128]; 64] = [[0; 128]; 64];
    let mut count: ::core::ffi::c_int = db_recording_tag_get(
        id,
        &raw mut result_tags as *mut [::core::ffi::c_char; 128],
        MAX_RECORDING_TAGS,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddNumberToObject(
        response,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        id as ::core::ffi::c_double,
    );
    let mut arr: *mut cJSON = cJSON_CreateArray();
    if count > 0 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < count {
            cJSON_AddItemToArray(
                arr,
                cJSON_CreateString(
                    &raw mut *(&raw mut result_tags as *mut [::core::ffi::c_char; 128])
                        .offset(i as isize) as *mut ::core::ffi::c_char,
                ),
            );
            i += 1;
        }
    }
    cJSON_AddItemToObject(
        response,
        b"tags\0".as_ptr() as *const ::core::ffi::c_char,
        arr,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    audit_recording_tag_operation(
        req,
        &raw mut user,
        &raw mut recording_uuid as *mut ::core::ffi::c_char,
        b"recording_tags.replace\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        tag_count,
        if count > 0 as ::core::ffi::c_int {
            count
        } else {
            0 as ::core::ffi::c_int
        },
        b"completed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Set %d tags for recording %llu\0".as_ptr() as *const ::core::ffi::c_char,
        count,
        id as ::core::ffi::c_ulonglong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_batch_recording_tags(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/recordings/batch-tags\0".as_ptr() as *const ::core::ffi::c_char,
    );
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
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
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
    let mut id_count: ::core::ffi::c_int = cJSON_GetArraySize(ids_json);
    if id_count <= 0 as ::core::ffi::c_int || id_count > 10000 as ::core::ffi::c_int {
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid number of IDs (1-10000)\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut ids: *mut uint64_t =
        malloc((id_count as size_t).wrapping_mul(::core::mem::size_of::<uint64_t>()))
            as *mut uint64_t;
    if ids.is_null() {
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Memory allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut valid_ids: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut invalid_member: bool = r#false != 0;
    let mut denied_member: bool = r#false != 0;
    let mut evaluation_error: bool = r#false != 0;
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
            invalid_member = r#true != 0;
            break;
        } else {
            let mut id: uint64_t = (*id_item).valuedouble as uint64_t;
            let mut recording: recording_metadata_t = recording_metadata_t {
                id: 0 as uint64_t,
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
            if get_recording_metadata_by_id(id, &raw mut recording) != 0 as ::core::ffi::c_int {
                denied_member = r#true != 0;
                break;
            } else {
                let mut eval_result: ::core::ffi::c_int = httpd_evaluate_camera_identity_action(
                    &raw mut user,
                    authorization_action_t::AUTHZ_EVIDENCE_PROTECT,
                    &raw mut recording.camera_uuid as *mut ::core::ffi::c_char,
                    &raw mut recording.stream_name as *mut ::core::ffi::c_char,
                    &raw mut evaluation,
                );
                if eval_result < 0 as ::core::ffi::c_int {
                    evaluation_error = r#true != 0;
                    break;
                } else if eval_result > 0 as ::core::ffi::c_int
                    || evaluation.decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0
                {
                    denied_member = r#true != 0;
                    break;
                } else {
                    let c2rust_fresh1 = valid_ids;
                    valid_ids += 1;
                    *ids.offset(c2rust_fresh1 as isize) = id;
                    id_item = (*id_item).next as *mut cJSON;
                }
            }
        }
    }
    if invalid_member as ::core::ffi::c_int != 0
        || denied_member as ::core::ffi::c_int != 0
        || evaluation_error as ::core::ffi::c_int != 0
        || valid_ids != id_count
    {
        free(ids as *mut ::core::ffi::c_void);
        cJSON_Delete(json);
        audit_recording_tag_operation(
            req,
            &raw mut user,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"recording_tags.batch\0".as_ptr() as *const ::core::ffi::c_char,
            if evaluation_error as ::core::ffi::c_int != 0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            id_count,
            0 as ::core::ffi::c_int,
            if invalid_member as ::core::ffi::c_int != 0 {
                b"invalid_member\0".as_ptr() as *const ::core::ffi::c_char
            } else if evaluation_error as ::core::ffi::c_int != 0 {
                b"authorization_evaluation_failed\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"unauthorized_member\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        if invalid_member {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid recording IDs\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if evaluation_error {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            http_response_set_json_error(
                res,
                403 as ::core::ffi::c_int,
                b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    let mut add_success: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut remove_success: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut database_error: bool = r#false != 0;
    let mut add_json: *mut cJSON =
        cJSON_GetObjectItem(json, b"add\0".as_ptr() as *const ::core::ffi::c_char);
    if !add_json.is_null() && cJSON_IsArray(add_json) != 0 {
        let mut tag_item: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
        tag_item = (if !add_json.is_null() {
            (*add_json).child
        } else {
            ::core::ptr::null_mut::<cJSON>()
        }) as *mut cJSON;
        while !tag_item.is_null() {
            if cJSON_IsString(tag_item) != 0
                && *(*tag_item).valuestring.offset(0isize) as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
            {
                let mut r: ::core::ffi::c_int =
                    db_recording_tag_batch_add(ids, valid_ids, (*tag_item).valuestring);
                if r > 0 as ::core::ffi::c_int {
                    add_success += r;
                }
                if r < 0 as ::core::ffi::c_int {
                    database_error = r#true != 0;
                }
            }
            tag_item = (*tag_item).next as *mut cJSON;
        }
    }
    let mut remove_json: *mut cJSON =
        cJSON_GetObjectItem(json, b"remove\0".as_ptr() as *const ::core::ffi::c_char);
    if !remove_json.is_null() && cJSON_IsArray(remove_json) != 0 {
        let mut tag_item_0: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
        tag_item_0 = (if !remove_json.is_null() {
            (*remove_json).child
        } else {
            ::core::ptr::null_mut::<cJSON>()
        }) as *mut cJSON;
        while !tag_item_0.is_null() {
            if cJSON_IsString(tag_item_0) != 0
                && *(*tag_item_0).valuestring.offset(0isize) as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
            {
                let mut r_0: ::core::ffi::c_int =
                    db_recording_tag_batch_remove(ids, valid_ids, (*tag_item_0).valuestring);
                if r_0 > 0 as ::core::ffi::c_int {
                    remove_success += r_0;
                }
                if r_0 < 0 as ::core::ffi::c_int {
                    database_error = r#true != 0;
                }
            }
            tag_item_0 = (*tag_item_0).next as *mut cJSON;
        }
    }
    free(ids as *mut ::core::ffi::c_void);
    cJSON_Delete(json);
    audit_recording_tag_operation(
        req,
        &raw mut user,
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"recording_tags.batch\0".as_ptr() as *const ::core::ffi::c_char,
        if database_error as ::core::ffi::c_int != 0 {
            b"error\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"success\0".as_ptr() as *const ::core::ffi::c_char
        },
        valid_ids,
        add_success + remove_success,
        if database_error as ::core::ffi::c_int != 0 {
            b"database_update_failed\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"completed\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if database_error {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to update recording tags\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddNumberToObject(
        response,
        b"recordings_count\0".as_ptr() as *const ::core::ffi::c_char,
        valid_ids as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        response,
        b"tags_added\0".as_ptr() as *const ::core::ffi::c_char,
        add_success as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        response,
        b"tags_removed\0".as_ptr() as *const ::core::ffi::c_char,
        remove_success as ::core::ffi::c_double,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Batch tags: %d recordings, %d added, %d removed\0".as_ptr() as *const ::core::ffi::c_char,
        valid_ids,
        add_success,
        remove_success,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
