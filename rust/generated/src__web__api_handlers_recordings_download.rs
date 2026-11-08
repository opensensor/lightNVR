unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
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
    unsafe fn http_serve_file(
        req: *const http_request_t,
        res: *const http_response_t,
        file_path: *const ::core::ffi::c_char,
        content_type: *const ::core::ffi::c_char,
        extra_headers: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
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
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
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
    unsafe fn httpd_sanitize_attachment_filename(
        input: *const ::core::ffi::c_char,
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
    );
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
    unsafe fn storage_source_resolve(
        recording_id: uint64_t,
        path: *mut ::core::ffi::c_char,
        error: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn recording_archive_serve(
        req: *const http_request_t,
        res: *mut http_response_t,
        recording_id: uint64_t,
        download: bool,
    ) -> bool;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_int);
impl C2Rust_Unnamed {
    pub const STORAGE_SOURCE_READY: Self = Self(0);
    pub const STORAGE_SOURCE_PREPARING: Self = Self(1);
    pub const STORAGE_SOURCE_ERROR: Self = Self(-1);
    pub const STORAGE_SOURCE_MISSING: Self = Self(-2);
    pub const STORAGE_SOURCE_DELETING: Self = Self(-3);
}
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[inline]
unsafe extern "C" fn recording_source_for_request(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut id: uint64_t,
    mut path: *mut ::core::ffi::c_char,
) -> bool {
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
    let mut result: ::core::ffi::c_int =
        storage_source_resolve(id, path, &raw mut error as *mut ::core::ffi::c_char);
    if result == C2Rust_Unnamed::STORAGE_SOURCE_READY.0 {
        return r#true != 0;
    }
    if result == C2Rust_Unnamed::STORAGE_SOURCE_PREPARING.0 {
        let mut prepare: [::core::ffi::c_char; 8] = [0; 8];
        let mut preparing: bool = http_request_get_query_param(
            req,
            b"prepare\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut prepare as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
        ) > 0 as ::core::ffi::c_int
            && strcmp(
                &raw mut prepare as *mut ::core::ffi::c_char,
                b"1\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0;
        http_response_add_header(
            res,
            b"Retry-After\0".as_ptr() as *const ::core::ffi::c_char,
            b"2\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json(
            res,
            if preparing as ::core::ffi::c_int != 0 {
                202 as ::core::ffi::c_int
            } else {
                503 as ::core::ffi::c_int
            },
            b"{\"status\":\"preparing\",\"source\":\"archive\",\"error\":\"Preparing archived recording\"}\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        http_response_set_json_error(
            res,
            if result == C2Rust_Unnamed::STORAGE_SOURCE_MISSING.0 {
                404 as ::core::ffi::c_int
            } else if result == C2Rust_Unnamed::STORAGE_SOURCE_DELETING.0 {
                409 as ::core::ffi::c_int
            } else {
                503 as ::core::ffi::c_int
            },
            if error[0usize] as ::core::ffi::c_int != 0 {
                &raw mut error as *mut ::core::ffi::c_char
            } else {
                b"Recording source is unavailable\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    return r#false != 0;
}
pub const LOG_COMPONENT: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"RecordingsAPI\0") };
unsafe extern "C" fn audit_recording_export(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut camera: *const fleet_camera_t,
    mut recording_id: uint64_t,
    mut outcome: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
    mut size_bytes: uint64_t,
    mut content_type: *const ::core::ffi::c_char,
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
        if size_bytes > 0 as uint64_t {
            cJSON_AddNumberToObject(
                details,
                b"size_bytes\0".as_ptr() as *const ::core::ffi::c_char,
                size_bytes as ::core::ffi::c_double,
            );
        }
        if !content_type.is_null() {
            cJSON_AddStringToObject(
                details,
                b"content_type\0".as_ptr() as *const ::core::ffi::c_char,
                content_type,
            );
        }
    }
    audit_log_operation(
        req,
        user,
        b"recordings.export\0".as_ptr() as *const ::core::ffi::c_char,
        b"recording\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut recording_uuid as *mut ::core::ffi::c_char,
        b"download_recording\0".as_ptr() as *const ::core::ffi::c_char,
        outcome,
        details,
    );
    cJSON_Delete(details);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_recordings_download(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if req.is_null() || res.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid parameters for handle_recordings_download\0".as_ptr()
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
    if httpd_check_action_access(req, &raw mut user) == 0 {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Authentication failed for GET /api/recordings/download request\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut id_str: [::core::ffi::c_char; 32] = [0; 32];
    if http_request_extract_path_param(
        req,
        b"/api/recordings/download/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut id_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to extract recording ID from URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid request path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut id: uint64_t = strtoull(
        &raw mut id_str as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        10 as ::core::ffi::c_int,
    ) as uint64_t;
    if id == 0 as uint64_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid recording ID: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut id_str as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid recording ID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/recordings/download/%llu request\0".as_ptr()
            as *const ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
    );
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
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Recording not found: %llu\0".as_ptr() as *const ::core::ffi::c_char,
            id as ::core::ffi::c_ulonglong,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Recording not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
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
        authorization_action_t::AUTHZ_RECORDINGS_EXPORT,
        &raw mut recording.camera_uuid as *mut ::core::ffi::c_char,
        &raw mut recording.stream_name as *mut ::core::ffi::c_char,
        &raw mut user,
        &raw mut camera,
        &raw mut evaluation,
    ) == 0
    {
        return;
    }
    if !recording.is_complete {
        http_response_set_json_error(
            res,
            409 as ::core::ffi::c_int,
            b"Recording is still in progress\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if recording_archive_serve(req, res, id, r#true != 0) {
        audit_recording_export(
            req,
            &raw mut user,
            &raw mut camera,
            id,
            if (*res).status_code >= 400 as ::core::ffi::c_int {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"success\0".as_ptr() as *const ::core::ffi::c_char
            },
            b"archive_export_requested\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint64_t,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return;
    }
    if !recording_source_for_request(
        req,
        res,
        id,
        &raw mut recording.file_path as *mut ::core::ffi::c_char,
    ) {
        audit_recording_export(
            req,
            &raw mut user,
            &raw mut camera,
            id,
            if (*res).status_code == 202 as ::core::ffi::c_int {
                b"success\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            if (*res).status_code == 202 as ::core::ffi::c_int {
                b"archive_preparing\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"recording_source_unavailable\0".as_ptr() as *const ::core::ffi::c_char
            },
            0 as uint64_t,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return;
    }
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if stat(
        &raw mut recording.file_path as *mut ::core::ffi::c_char,
        &raw mut st,
    ) != 0 as ::core::ffi::c_int
    {
        audit_recording_export(
            req,
            &raw mut user,
            &raw mut camera,
            id,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"recording_file_missing\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint64_t,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Recording file not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut recording.file_path as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Recording file not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut prepare: [::core::ffi::c_char; 8] = [0; 8];
    if http_request_get_query_param(
        req,
        b"prepare\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut prepare as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
    ) > 0 as ::core::ffi::c_int
        && strcmp(
            &raw mut prepare as *mut ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
    {
        http_response_set_json(
            res,
            200 as ::core::ffi::c_int,
            b"{\"status\":\"ready\"}\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut filename: [::core::ffi::c_char; 192] = [0; 192];
    httpd_sanitize_attachment_filename(
        &raw mut recording.file_path as *mut ::core::ffi::c_char,
        &raw mut filename as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 192]>(),
    );
    if filename[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        safe_strcpy(
            &raw mut filename as *mut ::core::ffi::c_char,
            b"recording.mp4\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 192]>(),
            0 as size_t,
        );
    }
    let mut content_type: *const ::core::ffi::c_char =
        b"video/mp4\0".as_ptr() as *const ::core::ffi::c_char;
    let mut file_ext: *const ::core::ffi::c_char = strrchr(
        &raw mut recording.file_path as *mut ::core::ffi::c_char,
        '.' as ::core::ffi::c_int,
    );
    if !file_ext.is_null() {
        file_ext = file_ext.offset(1);
        if strcasecmp(file_ext, b"mp4\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            content_type = b"video/mp4\0".as_ptr() as *const ::core::ffi::c_char;
        } else if strcasecmp(file_ext, b"mkv\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            content_type = b"video/x-matroska\0".as_ptr() as *const ::core::ffi::c_char;
        } else if strcasecmp(file_ext, b"webm\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            content_type = b"video/webm\0".as_ptr() as *const ::core::ffi::c_char;
        } else if strcasecmp(file_ext, b"avi\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            content_type = b"video/x-msvideo\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    let mut headers: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut headers as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"Content-Disposition: attachment; filename=\"%s\"\r\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut filename as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Serving file for download: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut recording.file_path as *mut ::core::ffi::c_char,
    );
    if http_serve_file(
        req,
        res,
        &raw mut recording.file_path as *mut ::core::ffi::c_char,
        content_type,
        &raw mut headers as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        audit_recording_export(
            req,
            &raw mut user,
            &raw mut camera,
            id,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"response_stream_failed\0".as_ptr() as *const ::core::ffi::c_char,
            st.st_size as uint64_t,
            content_type,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to serve file: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut recording.file_path as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serve file\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_recording_export(
        req,
        &raw mut user,
        &raw mut camera,
        id,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"completed\0".as_ptr() as *const ::core::ffi::c_char,
        st.st_size as uint64_t,
        content_type,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/recordings/download/%llu request\0".as_ptr()
            as *const ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
    );
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
