unsafe extern "C" {
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn __ctype_tolower_loc() -> *mut *const __int32_t;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
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
    unsafe fn cJSON_Duplicate(item: *const cJSON, recurse: cJSON_bool) -> *mut cJSON;
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
    unsafe fn authorization_action_metadata(
        action: authorization_action_t,
    ) -> *const authorization_action_metadata_t;
    unsafe fn authorization_decision_source_name(
        source: authorization_decision_source_t,
    ) -> *const ::core::ffi::c_char;
    unsafe fn db_audit_append(
        input: *const audit_event_input_t,
        event_uuid: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn db_audit_load_decision_modes(modes: *mut audit_decision_mode_t)
    -> ::core::ffi::c_int;
    unsafe fn db_audit_save_decision_modes(
        modes: *const audit_decision_mode_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_find_by_name(
        stream_name: *const ::core::ffi::c_char,
        camera: *mut fleet_camera_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn url_decode(
        src: *const ::core::ffi::c_char,
        dst: *mut ::core::ffi::c_char,
        dst_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn audit_summary_window_start(now: int64_t) -> int64_t;
    unsafe fn audit_summary_key_init(key: *mut audit_summary_key_t);
    unsafe fn audit_summary_init(capacity: size_t) -> ::core::ffi::c_int;
    unsafe fn audit_summary_shutdown();
    unsafe fn audit_summary_add(
        key: *const audit_summary_key_t,
        now: int64_t,
        sample: *const audit_summary_sample_t,
    ) -> audit_summary_add_result_t;
    unsafe fn audit_summary_drain(
        all: bool,
        current_window_start: int64_t,
        out: *mut audit_summary_entry_t,
        out_capacity: size_t,
    ) -> size_t;
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_get_effective_client_ip(
        req: *const http_request_t,
        client_ip: *mut ::core::ffi::c_char,
        client_ip_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
    ) -> ::core::ffi::c_int;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __int32_t = i32;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const PTHREAD_MUTEX_TIMED_NP: Self = Self(0);
    pub const PTHREAD_MUTEX_RECURSIVE_NP: Self = Self(1);
    pub const PTHREAD_MUTEX_ERRORCHECK_NP: Self = Self(2);
    pub const PTHREAD_MUTEX_ADAPTIVE_NP: Self = Self(3);
    pub const PTHREAD_MUTEX_NORMAL: Self = Self(0);
    pub const PTHREAD_MUTEX_RECURSIVE: Self = Self(1);
    pub const PTHREAD_MUTEX_ERRORCHECK: Self = Self(2);
    pub const PTHREAD_MUTEX_DEFAULT: Self = Self(0);
    pub const PTHREAD_MUTEX_FAST_NP: Self = Self(0);
}
pub type uint64_t = u64;
pub type atomic_int = ::core::ffi::c_int;
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
    pub const AUTHZ_OBSERVATIONS_ANNOTATE: Self = Self(19);
    pub const AUTHZ_ACTION_COUNT: Self = Self(20);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audit_event_input_t {
    pub request_id: *const ::core::ffi::c_char,
    pub principal_user_id: int64_t,
    pub principal_username: *const ::core::ffi::c_char,
    pub auth_method: *const ::core::ffi::c_char,
    pub api_token_uuid: *const ::core::ffi::c_char,
    pub action: *const ::core::ffi::c_char,
    pub target_type: *const ::core::ffi::c_char,
    pub target_uuid: *const ::core::ffi::c_char,
    pub outcome: *const ::core::ffi::c_char,
    pub remote_address: *const ::core::ffi::c_char,
    pub details_json: *const ::core::ffi::c_char,
    pub occurred_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct audit_decision_mode_t(pub ::core::ffi::c_uint);
impl audit_decision_mode_t {
    pub const AUDIT_DECISION_MODE_RECORD: Self = Self(0);
    pub const AUDIT_DECISION_MODE_SUMMARIZE: Self = Self(1);
    pub const AUDIT_DECISION_MODE_OFF: Self = Self(2);
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
pub struct audit_summary_add_result_t(pub ::core::ffi::c_uint);
impl audit_summary_add_result_t {
    pub const AUDIT_SUMMARY_ADDED: Self = Self(0);
    pub const AUDIT_SUMMARY_FULL: Self = Self(1);
    pub const AUDIT_SUMMARY_UNAVAILABLE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audit_summary_sample_t {
    pub auth_method: *const ::core::ffi::c_char,
    pub api_token_uuid: *const ::core::ffi::c_char,
    pub request_id: *const ::core::ffi::c_char,
    pub method: *const ::core::ffi::c_char,
    pub path: *const ::core::ffi::c_char,
    pub decision_source: *const ::core::ffi::c_char,
    pub explanation: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audit_summary_key_t {
    pub principal_user_id: int64_t,
    pub principal_username: [::core::ffi::c_char; 64],
    pub action: ::core::ffi::c_int,
    pub target_type: [::core::ffi::c_char; 48],
    pub target_uuid: [::core::ffi::c_char; 128],
    pub remote_address: [::core::ffi::c_char; 64],
    pub reserved_before_window_start: [::core::ffi::c_char; 4],
    pub window_start: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audit_summary_entry_t {
    pub key: audit_summary_key_t,
    pub count: uint64_t,
    pub first_at: int64_t,
    pub last_at: int64_t,
    pub auth_method: [::core::ffi::c_char; 24],
    pub api_token_uuid: [::core::ffi::c_char; 37],
    pub request_id: [::core::ffi::c_char; 65],
    pub method: [::core::ffi::c_char; 16],
    pub path: [::core::ffi::c_char; 512],
    pub decision_source: [::core::ffi::c_char; 32],
    pub explanation: [::core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audit_sensitive_operation_context_t {
    pub applicable: bool,
    pub has_user: bool,
    pub identity_is_camera_uuid: bool,
    pub action: authorization_action_t,
    pub user: user_t,
    pub operation: [::core::ffi::c_char; 48],
    pub target_type: [::core::ffi::c_char; 32],
    pub identity: [::core::ffi::c_char; 256],
    pub target_uuid: [::core::ffi::c_char; 37],
}
#[inline]
unsafe extern "C" fn tolower(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -128 as ::core::ffi::c_int && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
pub const LOG_COMPONENT: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"Audit\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const AUDIT_DECISION_MODES_INIT_DB_FAILED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const AUDIT_DECISION_MODES_INIT_TABLE_FAILED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const AUDIT_SUMMARY_WINDOW_SECONDS: ::core::ffi::c_int = 900 as ::core::ffi::c_int;
pub const AUDIT_SUMMARY_DEFAULT_CAPACITY: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
unsafe extern "C" fn contains_case_insensitive(
    mut value: *const ::core::ffi::c_char,
    mut needle: *const ::core::ffi::c_char,
) -> bool {
    if value.is_null() || needle.is_null() || *needle.offset(0isize) == 0 {
        return r#false != 0;
    }
    let mut needle_length: size_t = strlen(needle);
    let mut start: *const ::core::ffi::c_char = value;
    while *start != 0 {
        let mut index: size_t = 0 as size_t;
        while index < needle_length
            && *start.offset(index as isize) as ::core::ffi::c_int != 0
            && ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<::core::ffi::c_uchar>() > 1usize {
                    if 0 as ::core::ffi::c_int != 0 {
                        let mut __c: ::core::ffi::c_int = *start.offset(index as isize)
                            as ::core::ffi::c_uchar
                            as ::core::ffi::c_int;
                        __res = (if __c < -128 as ::core::ffi::c_int
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = tolower(*start.offset(index as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*start.offset(index as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            }) == ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<::core::ffi::c_uchar>() > 1usize {
                    if 0 as ::core::ffi::c_int != 0 {
                        let mut __c: ::core::ffi::c_int = *needle.offset(index as isize)
                            as ::core::ffi::c_uchar
                            as ::core::ffi::c_int;
                        __res = (if __c < -128 as ::core::ffi::c_int
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = tolower(*needle.offset(index as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*needle.offset(index as isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            })
        {
            index = index.wrapping_add(1);
        }
        if index == needle_length {
            return r#true != 0;
        }
        start = start.offset(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn has_case_insensitive_suffix(
    mut value: *const ::core::ffi::c_char,
    mut suffix: *const ::core::ffi::c_char,
) -> bool {
    if value.is_null() || suffix.is_null() {
        return r#false != 0;
    }
    let mut value_length: size_t = strlen(value);
    let mut suffix_length: size_t = strlen(suffix);
    if suffix_length > value_length {
        return r#false != 0;
    }
    return contains_case_insensitive(
        value
            .offset(value_length as isize)
            .offset(-(suffix_length as isize)),
        suffix,
    );
}
unsafe extern "C" fn sensitive_detail_key(mut key: *const ::core::ffi::c_char) -> bool {
    if key.is_null() {
        return r#false != 0;
    }
    if contains_case_insensitive(key, b"password\0".as_ptr() as *const ::core::ffi::c_char)
        as ::core::ffi::c_int
        != 0
        || contains_case_insensitive(key, b"passwd\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(key, b"passphrase\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(key, b"secret\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(key, b"credential\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(
            key,
            b"authorization\0".as_ptr() as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(key, b"cookie\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(key, b"api_key\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(key, b"apikey\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
        || contains_case_insensitive(key, b"plate\0".as_ptr() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
    {
        return r#true != 0;
    }
    return contains_case_insensitive(key, b"token\0".as_ptr() as *const ::core::ffi::c_char)
        as ::core::ffi::c_int
        != 0
        && !has_case_insensitive_suffix(
            key,
            b"_token_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        )
        && !has_case_insensitive_suffix(
            key,
            b"_token_id\0".as_ptr() as *const ::core::ffi::c_char,
        );
}
unsafe extern "C" fn replace_control_characters(mut value: *mut ::core::ffi::c_char) {
    if value.is_null() {
        return;
    }
    let mut cursor: *mut ::core::ffi::c_uchar = value as *mut ::core::ffi::c_uchar;
    while *cursor != 0 {
        if *(*__ctype_b_loc()).offset(*cursor as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed::_IScntrl.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            *cursor = '_' as ::core::ffi::c_uchar;
        }
        cursor = cursor.offset(1);
    }
}
unsafe extern "C" fn redacted_details_copy(
    mut source: *const cJSON,
    mut property_name: *const ::core::ffi::c_char,
) -> *mut cJSON {
    if !property_name.is_null() && sensitive_detail_key(property_name) as ::core::ffi::c_int != 0 {
        return cJSON_CreateString(b"[REDACTED]\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if cJSON_IsObject(source) != 0 {
        let mut copy: *mut cJSON = cJSON_CreateObject();
        if copy.is_null() {
            return ::core::ptr::null_mut::<cJSON>();
        }
        let mut child: *const cJSON = (*source).child;
        while !child.is_null() {
            let mut safe_child: *mut cJSON = redacted_details_copy(child, (*child).string);
            if safe_child.is_null()
                || cJSON_AddItemToObject(
                    copy,
                    if !(*child).string.is_null() {
                        (*child).string
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    safe_child,
                ) == 0
            {
                cJSON_Delete(safe_child);
                cJSON_Delete(copy);
                return ::core::ptr::null_mut::<cJSON>();
            }
            child = (*child).next;
        }
        return copy;
    }
    if cJSON_IsArray(source) != 0 {
        let mut copy_0: *mut cJSON = cJSON_CreateArray();
        if copy_0.is_null() {
            return ::core::ptr::null_mut::<cJSON>();
        }
        let mut child_0: *const cJSON = (*source).child;
        while !child_0.is_null() {
            let mut safe_child_0: *mut cJSON =
                redacted_details_copy(child_0, ::core::ptr::null::<::core::ffi::c_char>());
            if safe_child_0.is_null() || cJSON_AddItemToArray(copy_0, safe_child_0) == 0 {
                cJSON_Delete(safe_child_0);
                cJSON_Delete(copy_0);
                return ::core::ptr::null_mut::<cJSON>();
            }
            child_0 = (*child_0).next;
        }
        return copy_0;
    }
    return cJSON_Duplicate(source, r#true);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_log_append(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut action: *const ::core::ffi::c_char,
    mut target_type: *const ::core::ffi::c_char,
    mut target_uuid: *const ::core::ffi::c_char,
    mut outcome: *const ::core::ffi::c_char,
    mut details: *const cJSON,
) {
    if req.is_null() || action.is_null() || outcome.is_null() {
        return;
    }
    let mut redacted: *mut cJSON = if !details.is_null() {
        redacted_details_copy(details, ::core::ptr::null::<::core::ffi::c_char>())
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut serialized: *mut ::core::ffi::c_char = if !redacted.is_null() {
        cJSON_PrintUnformatted(redacted)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    cJSON_Delete(redacted);
    let mut details_json: *const ::core::ffi::c_char = if !serialized.is_null() {
        serialized
    } else {
        b"{}\0".as_ptr() as *const ::core::ffi::c_char
    };
    let mut remote_address: [::core::ffi::c_char; 64] = [
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
        &raw mut remote_address as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut remote_address as *mut ::core::ffi::c_char,
            &raw const (*req).client_ip as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    let mut input: audit_event_input_t = audit_event_input_t {
        request_id: &raw const (*req).request_id as *const ::core::ffi::c_char,
        principal_user_id: if !user.is_null() {
            (*user).id
        } else {
            0 as int64_t
        },
        principal_username: if !user.is_null() {
            &raw const (*user).username as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        auth_method: if !user.is_null()
            && (*user).authentication_method[0usize] as ::core::ffi::c_int != 0
        {
            &raw const (*user).authentication_method as *const ::core::ffi::c_char
        } else {
            b"unauthenticated\0".as_ptr() as *const ::core::ffi::c_char
        },
        api_token_uuid: if !user.is_null()
            && (*user).authenticated_via_scoped_token as ::core::ffi::c_int != 0
        {
            &raw const (*user).api_token_uuid as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        },
        action: action,
        target_type: target_type,
        target_uuid: target_uuid,
        outcome: outcome,
        remote_address: &raw mut remote_address as *mut ::core::ffi::c_char,
        details_json: details_json,
        occurred_at: 0,
    };
    if db_audit_append(
        &raw mut input,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to persist audit event for action %s\0".as_ptr() as *const ::core::ffi::c_char,
            action,
        );
    }
    free(serialized as *mut ::core::ffi::c_void);
}
pub const AUDIT_SUMMARY_FLUSH_BATCH: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
static mut decision_modes: [atomic_int; 20] = [0; 20];
static mut decision_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_0::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
unsafe extern "C" fn system_clock() -> int64_t {
    return time(::core::ptr::null_mut::<time_t>()) as int64_t;
}
static mut audit_clock: Option<unsafe extern "C" fn() -> int64_t> =
    Some(system_clock as unsafe extern "C" fn() -> int64_t);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_log_set_clock_for_testing(
    mut clock_fn: Option<unsafe extern "C" fn() -> int64_t>,
) {
    audit_clock = (if clock_fn.is_some() {
        clock_fn
    } else {
        Some(system_clock as unsafe extern "C" fn() -> int64_t)
    }) as Option<unsafe extern "C" fn() -> int64_t>;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_log_reset_summaries_for_testing(
    mut capacity: size_t,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < authorization_action_t::AUTHZ_ACTION_COUNT.0 {
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            (&raw mut decision_modes as *mut atomic_int).offset(i as isize),
            audit_decision_mode_t::AUDIT_DECISION_MODE_RECORD.0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    return audit_summary_init(if capacity != 0 {
        capacity
    } else {
        AUDIT_SUMMARY_DEFAULT_CAPACITY as size_t
    });
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_log_decision_modes_init() -> ::core::ffi::c_int {
    let mut modes: [audit_decision_mode_t; 20] =
        [audit_decision_mode_t::AUDIT_DECISION_MODE_RECORD; 20];
    let mut rc: ::core::ffi::c_int =
        db_audit_load_decision_modes(&raw mut modes as *mut audit_decision_mode_t);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < authorization_action_t::AUTHZ_ACTION_COUNT.0 {
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            (&raw mut decision_modes as *mut atomic_int).offset(i as isize),
            modes[i as usize].0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if rc != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Could not read audit decision modes; recording every authorization decision\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        result |= AUDIT_DECISION_MODES_INIT_DB_FAILED;
    }
    if audit_summary_init(AUDIT_SUMMARY_DEFAULT_CAPACITY as size_t) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to allocate audit summary table; summarize falls back to recording individually, off modes still apply\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        result |= AUDIT_DECISION_MODES_INIT_TABLE_FAILED;
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_log_get_decision_mode(
    mut action: authorization_action_t,
) -> audit_decision_mode_t {
    if action.0 < 0 as ::core::ffi::c_int
        || action.0 >= authorization_action_t::AUTHZ_ACTION_COUNT.0
    {
        return audit_decision_mode_t::AUDIT_DECISION_MODE_RECORD;
    }
    return audit_decision_mode_t(::core::intrinsics::atomic_load::<
        _,
        { ::core::intrinsics::AtomicOrdering::SeqCst },
    >(
        (&raw mut decision_modes as *mut atomic_int).offset(action.0 as isize),
    ) as ::core::ffi::c_uint);
}
unsafe extern "C" fn write_summary_row(mut entry: *const audit_summary_entry_t) {
    let mut metadata: *const authorization_action_metadata_t =
        authorization_action_metadata(authorization_action_t((*entry).key.action));
    if metadata.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to persist audit summary for unknown action id %d; %llu decisions not recorded\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*entry).key.action,
            (*entry).count as ::core::ffi::c_ulonglong,
        );
        return;
    }
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
            b"authorization.summary\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            details,
            b"count\0".as_ptr() as *const ::core::ffi::c_char,
            (*entry).count as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            details,
            b"first_at\0".as_ptr() as *const ::core::ffi::c_char,
            (*entry).first_at as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            details,
            b"last_at\0".as_ptr() as *const ::core::ffi::c_char,
            (*entry).last_at as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            details,
            b"window_seconds\0".as_ptr() as *const ::core::ffi::c_char,
            AUDIT_SUMMARY_WINDOW_SECONDS as ::core::ffi::c_double,
        );
        cJSON_AddStringToObject(
            details,
            b"method\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*entry).method as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"path\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*entry).path as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"decision_source\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*entry).decision_source as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"explanation\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*entry).explanation as *const ::core::ffi::c_char,
        );
    }
    let mut redacted: *mut cJSON = if !details.is_null() {
        redacted_details_copy(details, ::core::ptr::null::<::core::ffi::c_char>())
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut serialized: *mut ::core::ffi::c_char = if !redacted.is_null() {
        cJSON_PrintUnformatted(redacted)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    cJSON_Delete(redacted);
    cJSON_Delete(details);
    if serialized.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to persist audit summary for %s on %s; %llu decisions not recorded\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*metadata).key,
            if (*entry).key.target_uuid[0usize] as ::core::ffi::c_int != 0 {
                &raw const (*entry).key.target_uuid as *const ::core::ffi::c_char
            } else {
                b"system\0".as_ptr() as *const ::core::ffi::c_char
            },
            (*entry).count as ::core::ffi::c_ulonglong,
        );
        return;
    }
    let mut input: audit_event_input_t = audit_event_input_t {
        request_id: if (*entry).request_id[0usize] as ::core::ffi::c_int != 0 {
            &raw const (*entry).request_id as *const ::core::ffi::c_char
        } else {
            b"authorization-summary\0".as_ptr() as *const ::core::ffi::c_char
        },
        principal_user_id: (*entry).key.principal_user_id,
        principal_username: &raw const (*entry).key.principal_username
            as *const ::core::ffi::c_char,
        auth_method: if (*entry).auth_method[0usize] as ::core::ffi::c_int != 0 {
            &raw const (*entry).auth_method as *const ::core::ffi::c_char
        } else {
            b"unauthenticated\0".as_ptr() as *const ::core::ffi::c_char
        },
        api_token_uuid: if (*entry).api_token_uuid[0usize] as ::core::ffi::c_int != 0 {
            &raw const (*entry).api_token_uuid as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        },
        action: (*metadata).key,
        target_type: &raw const (*entry).key.target_type as *const ::core::ffi::c_char,
        target_uuid: if (*entry).key.target_uuid[0usize] as ::core::ffi::c_int != 0 {
            &raw const (*entry).key.target_uuid as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        },
        outcome: b"allowed\0".as_ptr() as *const ::core::ffi::c_char,
        remote_address: &raw const (*entry).key.remote_address as *const ::core::ffi::c_char,
        details_json: serialized,
        occurred_at: (*entry).first_at,
    };
    if db_audit_append(
        &raw mut input,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to persist audit summary for %s on %s; %llu decisions not recorded\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*metadata).key,
            if (*entry).key.target_uuid[0usize] as ::core::ffi::c_int != 0 {
                &raw const (*entry).key.target_uuid as *const ::core::ffi::c_char
            } else {
                b"system\0".as_ptr() as *const ::core::ffi::c_char
            },
            (*entry).count as ::core::ffi::c_ulonglong,
        );
    }
    free(serialized as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn flush_summaries_locked(mut closed_windows_only: bool) -> size_t {
    let mut batch: [audit_summary_entry_t; 8] = [audit_summary_entry_t {
        key: audit_summary_key_t {
            principal_user_id: 0,
            principal_username: [0; 64],
            action: 0,
            target_type: [0; 48],
            target_uuid: [0; 128],
            remote_address: [0; 64],
            reserved_before_window_start: [0; 4],
            window_start: 0,
        },
        count: 0,
        first_at: 0,
        last_at: 0,
        auth_method: [0; 24],
        api_token_uuid: [0; 37],
        request_id: [0; 65],
        method: [0; 16],
        path: [0; 512],
        decision_source: [0; 32],
        explanation: [0; 256],
    }; 8];
    let mut current_window: int64_t =
        audit_summary_window_start(audit_clock.expect("non-null function pointer")());
    let mut flushed: size_t = 0 as size_t;
    loop {
        let mut drained: size_t = audit_summary_drain(
            !closed_windows_only,
            current_window,
            &raw mut batch as *mut audit_summary_entry_t,
            AUDIT_SUMMARY_FLUSH_BATCH as size_t,
        );
        let mut i: size_t = 0 as size_t;
        while i < drained {
            write_summary_row((&raw mut batch as *mut audit_summary_entry_t).offset(i as isize));
            i = i.wrapping_add(1);
        }
        flushed = flushed.wrapping_add(drained);
        if drained < AUDIT_SUMMARY_FLUSH_BATCH as size_t {
            break;
        }
    }
    return flushed;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_log_flush_summaries(mut closed_windows_only: bool) -> size_t {
    pthread_mutex_lock(&raw mut decision_mutex);
    let mut flushed: size_t = flush_summaries_locked(closed_windows_only);
    pthread_mutex_unlock(&raw mut decision_mutex);
    return flushed;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_log_set_decision_modes(
    mut modes: *const audit_decision_mode_t,
) -> ::core::ffi::c_int {
    if modes.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    pthread_mutex_lock(&raw mut decision_mutex);
    flush_summaries_locked(r#false != 0);
    if db_audit_save_decision_modes(modes) != 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut decision_mutex);
        return -1 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < authorization_action_t::AUTHZ_ACTION_COUNT.0 {
        ::core::intrinsics::atomic_store::<_, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            (&raw mut decision_modes as *mut atomic_int).offset(i as isize),
            (*modes.offset(i as isize)).0 as ::core::ffi::c_int,
        );
        i += 1;
    }
    pthread_mutex_unlock(&raw mut decision_mutex);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_log_shutdown_summaries() {
    let mut flushed: size_t = audit_log_flush_summaries(r#false != 0);
    if flushed > 0 as size_t {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Flushed %zu audit summaries at shutdown\0".as_ptr() as *const ::core::ffi::c_char,
            flushed,
        );
    }
    audit_summary_shutdown();
}
unsafe extern "C" fn summarize_decision(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut action: authorization_action_t,
    mut camera: *const fleet_camera_t,
    mut evaluation: *const authorization_evaluation_t,
) -> bool {
    let mut now: int64_t = audit_clock.expect("non-null function pointer")();
    let mut key: audit_summary_key_t = audit_summary_key_t {
        principal_user_id: 0,
        principal_username: [0; 64],
        action: 0,
        target_type: [0; 48],
        target_uuid: [0; 128],
        remote_address: [0; 64],
        reserved_before_window_start: [0; 4],
        window_start: 0,
    };
    audit_summary_key_init(&raw mut key);
    key.principal_user_id = if !user.is_null() {
        (*user).id
    } else {
        0 as int64_t
    };
    if !user.is_null() {
        safe_strcpy(
            &raw mut key.principal_username as *mut ::core::ffi::c_char,
            &raw const (*user).username as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    key.action = action.0;
    safe_strcpy(
        &raw mut key.target_type as *mut ::core::ffi::c_char,
        if !camera.is_null() {
            b"camera\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"system\0".as_ptr() as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 48]>(),
        0 as size_t,
    );
    if !camera.is_null() {
        safe_strcpy(
            &raw mut key.target_uuid as *mut ::core::ffi::c_char,
            &raw const (*camera).camera_uuid as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
    }
    if httpd_get_effective_client_ip(
        req,
        &raw mut key.remote_address as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) != 0 as ::core::ffi::c_int
    {
        memset(
            &raw mut key.remote_address as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        );
        safe_strcpy(
            &raw mut key.remote_address as *mut ::core::ffi::c_char,
            &raw const (*req).client_ip as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    key.window_start = audit_summary_window_start(now);
    let mut sample: audit_summary_sample_t = audit_summary_sample_t {
        auth_method: if !user.is_null()
            && (*user).authentication_method[0usize] as ::core::ffi::c_int != 0
        {
            &raw const (*user).authentication_method as *const ::core::ffi::c_char
        } else {
            b"unauthenticated\0".as_ptr() as *const ::core::ffi::c_char
        },
        api_token_uuid: if !user.is_null()
            && (*user).authenticated_via_scoped_token as ::core::ffi::c_int != 0
        {
            &raw const (*user).api_token_uuid as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        },
        request_id: &raw const (*req).request_id as *const ::core::ffi::c_char,
        method: &raw const (*req).method_str as *const ::core::ffi::c_char,
        path: &raw const (*req).path as *const ::core::ffi::c_char,
        decision_source: if !evaluation.is_null() {
            authorization_decision_source_name((*evaluation).source)
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        explanation: if !evaluation.is_null() {
            &raw const (*evaluation).explanation as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    };
    let mut result: audit_summary_add_result_t =
        audit_summary_add(&raw mut key, now, &raw mut sample);
    if result.0 == audit_summary_add_result_t::AUDIT_SUMMARY_FULL.0 {
        flush_summaries_locked(r#false != 0);
        result = audit_summary_add(&raw mut key, now, &raw mut sample);
    }
    return result.0 == audit_summary_add_result_t::AUDIT_SUMMARY_ADDED.0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_log_operation(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut action: *const ::core::ffi::c_char,
    mut target_type: *const ::core::ffi::c_char,
    mut target_uuid: *const ::core::ffi::c_char,
    mut operation: *const ::core::ffi::c_char,
    mut outcome: *const ::core::ffi::c_char,
    mut context: *const cJSON,
) {
    if req.is_null() || action.is_null() || operation.is_null() || outcome.is_null() {
        return;
    }
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
            b"operation.outcome\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"method\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*req).method_str as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"path\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*req).path as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"operation\0".as_ptr() as *const ::core::ffi::c_char,
            operation,
        );
        if !context.is_null() {
            let mut context_copy: *mut cJSON = cJSON_Duplicate(context, r#true);
            if !context_copy.is_null() {
                cJSON_AddItemToObject(
                    details,
                    b"context\0".as_ptr() as *const ::core::ffi::c_char,
                    context_copy,
                );
            }
        }
    }
    audit_log_append(
        req,
        user,
        action,
        target_type,
        target_uuid,
        outcome,
        details,
    );
    cJSON_Delete(details);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_log_authorization(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut action: authorization_action_t,
    mut camera: *const fleet_camera_t,
    mut evaluation: *const authorization_evaluation_t,
    mut outcome: *const ::core::ffi::c_char,
) {
    let mut metadata: *const authorization_action_metadata_t =
        authorization_action_metadata(action);
    if metadata.is_null() || req.is_null() || outcome.is_null() {
        return;
    }
    let mut read_only: bool = strcmp(
        &raw const (*req).method_str as *const ::core::ffi::c_char,
        b"GET\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcmp(
            &raw const (*req).method_str as *const ::core::ffi::c_char,
            b"HEAD\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
    if read_only as ::core::ffi::c_int != 0
        && strcmp(outcome, b"allowed\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        pthread_mutex_lock(&raw mut decision_mutex);
        let mut mode: audit_decision_mode_t = audit_log_get_decision_mode(action);
        let mut handled: bool = mode.0 == audit_decision_mode_t::AUDIT_DECISION_MODE_OFF.0
            || mode.0 == audit_decision_mode_t::AUDIT_DECISION_MODE_SUMMARIZE.0
                && summarize_decision(req, user, action, camera, evaluation) as ::core::ffi::c_int
                    != 0;
        pthread_mutex_unlock(&raw mut decision_mutex);
        if handled {
            return;
        }
    }
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
            b"authorization.decision\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"method\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*req).method_str as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"path\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*req).path as *const ::core::ffi::c_char,
        );
        if !evaluation.is_null() {
            cJSON_AddStringToObject(
                details,
                b"decision_source\0".as_ptr() as *const ::core::ffi::c_char,
                authorization_decision_source_name((*evaluation).source),
            );
            cJSON_AddNumberToObject(
                details,
                b"policy_version\0".as_ptr() as *const ::core::ffi::c_char,
                (*evaluation).policy_version as ::core::ffi::c_double,
            );
            cJSON_AddStringToObject(
                details,
                b"explanation\0".as_ptr() as *const ::core::ffi::c_char,
                &raw const (*evaluation).explanation as *const ::core::ffi::c_char,
            );
            if (*evaluation).grant_uuid[0usize] != 0 {
                cJSON_AddStringToObject(
                    details,
                    b"grant_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*evaluation).grant_uuid as *const ::core::ffi::c_char,
                );
            }
            if (*evaluation).role_uuid[0usize] != 0 {
                cJSON_AddStringToObject(
                    details,
                    b"role_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw const (*evaluation).role_uuid as *const ::core::ffi::c_char,
                );
            }
        }
    }
    audit_log_append(
        req,
        user,
        (*metadata).key,
        if !camera.is_null() {
            b"camera\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"system\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !camera.is_null() {
            &raw const (*camera).camera_uuid as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        },
        outcome,
        details,
    );
    cJSON_Delete(details);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_log_login(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut attempted_username: *const ::core::ffi::c_char,
    mut authentication_method: *const ::core::ffi::c_char,
    mut outcome: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
) {
    if req.is_null() || authentication_method.is_null() || outcome.is_null() || reason.is_null() {
        return;
    }
    let mut principal: user_t = user_t {
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
    if !user.is_null() {
        principal = *user;
    }
    if principal.username[0usize] == 0 && !attempted_username.is_null() {
        safe_strcpy(
            &raw mut principal.username as *mut ::core::ffi::c_char,
            attempted_username,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    replace_control_characters(&raw mut principal.username as *mut ::core::ffi::c_char);
    safe_strcpy(
        &raw mut principal.authentication_method as *mut ::core::ffi::c_char,
        authentication_method,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        0 as size_t,
    );
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
            b"authentication.login\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            details,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        );
    }
    let mut target_user_id: [::core::ffi::c_char; 32] = [
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
    if principal.id > 0 as int64_t {
        snprintf(
            &raw mut target_user_id as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
            principal.id as ::core::ffi::c_longlong,
        );
    }
    audit_log_append(
        req,
        &raw mut principal,
        b"auth.login\0".as_ptr() as *const ::core::ffi::c_char,
        b"user\0".as_ptr() as *const ::core::ffi::c_char,
        if target_user_id[0usize] as ::core::ffi::c_int != 0 {
            &raw mut target_user_id as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        outcome,
        details,
    );
    cJSON_Delete(details);
}
unsafe extern "C" fn path_segment(
    mut path: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
    mut value: *mut ::core::ffi::c_char,
    mut value_size: size_t,
) -> bool {
    if path.is_null()
        || prefix.is_null()
        || value.is_null()
        || value_size == 0 as size_t
        || strncmp(path, prefix, strlen(prefix)) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut start: *const ::core::ffi::c_char = path.offset(strlen(prefix) as isize);
    let mut end: *const ::core::ffi::c_char = strchr(start, '/' as ::core::ffi::c_int);
    let mut length: size_t = if !end.is_null() {
        end.offset_from(start) as size_t
    } else {
        strlen(start)
    };
    if length == 0 as size_t || length >= value_size {
        return r#false != 0;
    }
    let mut encoded: [::core::ffi::c_char; 256] = [
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
    if length >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>() {
        return r#false != 0;
    }
    memcpy(
        &raw mut encoded as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        start as *const ::core::ffi::c_void,
        length,
    );
    encoded[length] = '\0' as ::core::ffi::c_char;
    return url_decode(
        &raw mut encoded as *mut ::core::ffi::c_char,
        value,
        value_size,
    ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn json_string_field(
    mut req: *const http_request_t,
    mut key: *const ::core::ffi::c_char,
    mut value: *mut ::core::ffi::c_char,
    mut value_size: size_t,
) -> bool {
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    if body.is_null() {
        return r#false != 0;
    }
    let mut field: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, key);
    let mut copied: bool = cJSON_IsString(field) != 0
        && !(*field).valuestring.is_null()
        && safe_strcpy(value, (*field).valuestring, value_size, 0 as size_t)
            == 0 as ::core::ffi::c_int;
    cJSON_Delete(body);
    return copied;
}
unsafe extern "C" fn camera_configuration_operation(
    mut req: *const http_request_t,
    mut target_type: *mut ::core::ffi::c_char,
    mut target_type_size: size_t,
    mut identity: *mut ::core::ffi::c_char,
    mut identity_size: size_t,
    mut identity_is_camera_uuid: *mut bool,
    mut action: *mut authorization_action_t,
) -> *const ::core::ffi::c_char {
    let mut path: *const ::core::ffi::c_char = &raw const (*req).path as *const ::core::ffi::c_char;
    let mut method: *const ::core::ffi::c_char =
        &raw const (*req).method_str as *const ::core::ffi::c_char;
    *identity_is_camera_uuid = r#false != 0;
    *action = authorization_action_t::AUTHZ_CAMERA_CONFIGURE;
    safe_strcpy(
        target_type,
        b"camera\0".as_ptr() as *const ::core::ffi::c_char,
        target_type_size,
        0 as size_t,
    );
    if strcmp(
        path,
        b"/api/system/logs/clear\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && strcmp(method, b"POST\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        *action = authorization_action_t::AUTHZ_SYSTEM_ADMIN;
        safe_strcpy(
            target_type,
            b"system_logs\0".as_ptr() as *const ::core::ffi::c_char,
            target_type_size,
            0 as size_t,
        );
        return b"system.logs.clear\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcmp(
        path,
        b"/api/settings\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && strcmp(method, b"POST\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        *action = authorization_action_t::AUTHZ_SYSTEM_ADMIN;
        safe_strcpy(
            target_type,
            b"settings\0".as_ptr() as *const ::core::ffi::c_char,
            target_type_size,
            0 as size_t,
        );
        return b"settings.update\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcmp(
        path,
        b"/api/streams/test\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && strcmp(method, b"POST\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        return b"stream.test\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcmp(
        path,
        b"/api/streams\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && strcmp(method, b"POST\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        json_string_field(
            req,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            identity,
            identity_size,
        );
        return b"stream.create\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcmp(
        path,
        b"/api/motion/trigger\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && strcmp(method, b"POST\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        json_string_field(
            req,
            b"stream\0".as_ptr() as *const ::core::ffi::c_char,
            identity,
            identity_size,
        );
        return b"motion.trigger\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strncmp(
        path,
        b"/api/onvif/\0".as_ptr() as *const ::core::ffi::c_char,
        11 as size_t,
    ) == 0 as ::core::ffi::c_int
        && strcmp(method, b"POST\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        if !strstr(
            path,
            b"/discovery/discover\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            return b"onvif.discovery\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if !strstr(
            path,
            b"/device/add\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            return b"onvif.add\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if !strstr(
            path,
            b"/device/test\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            return b"onvif.test\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    if strncmp(
        path,
        b"/api/streams/\0".as_ptr() as *const ::core::ffi::c_char,
        13 as size_t,
    ) == 0 as ::core::ffi::c_int
        && path_segment(
            path,
            b"/api/streams/\0".as_ptr() as *const ::core::ffi::c_char,
            identity,
            identity_size,
        ) as ::core::ffi::c_int
            != 0
    {
        if !strstr(path, b"/refresh\0".as_ptr() as *const ::core::ffi::c_char).is_null()
            && strcmp(method, b"POST\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"stream.refresh\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if !strstr(path, b"/recording\0".as_ptr() as *const ::core::ffi::c_char).is_null()
            && strcmp(method, b"POST\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"manual_recording.update\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if !strstr(path, b"/retention\0".as_ptr() as *const ::core::ffi::c_char).is_null()
            && strcmp(method, b"PUT\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"stream.retention.update\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if !strstr(path, b"/zones\0".as_ptr() as *const ::core::ffi::c_char).is_null()
            && strcmp(method, b"POST\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"zones.update\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if !strstr(path, b"/zones\0".as_ptr() as *const ::core::ffi::c_char).is_null()
            && strcmp(method, b"DELETE\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"zones.delete\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if !strstr(
            path,
            b"/imaging/settings\0".as_ptr() as *const ::core::ffi::c_char,
        )
        .is_null()
            && strcmp(method, b"PUT\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"imaging.update\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if !strstr(path, b"/daynight\0".as_ptr() as *const ::core::ffi::c_char).is_null()
            && strcmp(method, b"PUT\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"daynight.update\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if strchr(
            path.offset(13 as ::core::ffi::c_int as isize),
            '/' as ::core::ffi::c_int,
        )
        .is_null()
            && strcmp(method, b"PUT\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"stream.update\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if strchr(
            path.offset(13 as ::core::ffi::c_int as isize),
            '/' as ::core::ffi::c_int,
        )
        .is_null()
            && strcmp(method, b"DELETE\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"stream.delete\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    if strncmp(
        path,
        b"/api/cameras/\0".as_ptr() as *const ::core::ffi::c_char,
        13 as size_t,
    ) == 0 as ::core::ffi::c_int
        && path_segment(
            path,
            b"/api/cameras/\0".as_ptr() as *const ::core::ffi::c_char,
            identity,
            identity_size,
        ) as ::core::ffi::c_int
            != 0
    {
        *identity_is_camera_uuid = r#true != 0;
        if !strstr(path, b"/location\0".as_ptr() as *const ::core::ffi::c_char).is_null()
            && strcmp(method, b"PUT\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"camera.location.assign\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if !strstr(path, b"/tags\0".as_ptr() as *const ::core::ffi::c_char).is_null()
            && strcmp(method, b"PUT\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"camera.tags.assign\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    if strncmp(
        path,
        b"/api/locations\0".as_ptr() as *const ::core::ffi::c_char,
        14 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            target_type,
            b"location\0".as_ptr() as *const ::core::ffi::c_char,
            target_type_size,
            0 as size_t,
        );
        if strcmp(
            path,
            b"/api/locations\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && strcmp(method, b"POST\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"location.create\0".as_ptr() as *const ::core::ffi::c_char;
        }
        path_segment(
            path,
            b"/api/locations/\0".as_ptr() as *const ::core::ffi::c_char,
            identity,
            identity_size,
        );
        if strcmp(method, b"PUT\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return b"location.update\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if strcmp(method, b"DELETE\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return b"location.delete\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    if strncmp(
        path,
        b"/api/live/plans\0".as_ptr() as *const ::core::ffi::c_char,
        15 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            target_type,
            b"operator_floor_plan\0".as_ptr() as *const ::core::ffi::c_char,
            target_type_size,
            0 as size_t,
        );
        if strcmp(
            path,
            b"/api/live/plans\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && strcmp(method, b"POST\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"operator_floor_plan.create\0".as_ptr() as *const ::core::ffi::c_char;
        }
        path_segment(
            path,
            b"/api/live/plans/\0".as_ptr() as *const ::core::ffi::c_char,
            identity,
            identity_size,
        );
        if strcmp(method, b"PUT\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return b"operator_floor_plan.update\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if strcmp(method, b"DELETE\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return b"operator_floor_plan.delete\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    if strncmp(
        path,
        b"/api/camera-tags\0".as_ptr() as *const ::core::ffi::c_char,
        16 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            target_type,
            b"camera_tag\0".as_ptr() as *const ::core::ffi::c_char,
            target_type_size,
            0 as size_t,
        );
        if strcmp(
            path,
            b"/api/camera-tags\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && strcmp(method, b"POST\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"camera_tag.create\0".as_ptr() as *const ::core::ffi::c_char;
        }
        path_segment(
            path,
            b"/api/camera-tags/\0".as_ptr() as *const ::core::ffi::c_char,
            identity,
            identity_size,
        );
        if !strstr(path, b"/merge\0".as_ptr() as *const ::core::ffi::c_char).is_null()
            && strcmp(method, b"POST\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"camera_tag.merge\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if strcmp(method, b"PUT\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return b"camera_tag.update\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if strcmp(method, b"DELETE\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return b"camera_tag.delete\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    if strncmp(
        path,
        b"/api/camera-collections\0".as_ptr() as *const ::core::ffi::c_char,
        23 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            target_type,
            b"camera_collection\0".as_ptr() as *const ::core::ffi::c_char,
            target_type_size,
            0 as size_t,
        );
        if strcmp(
            path,
            b"/api/camera-collections\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && strcmp(method, b"POST\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"camera_collection.create\0".as_ptr() as *const ::core::ffi::c_char;
        }
        path_segment(
            path,
            b"/api/camera-collections/\0".as_ptr() as *const ::core::ffi::c_char,
            identity,
            identity_size,
        );
        if !strstr(path, b"/members\0".as_ptr() as *const ::core::ffi::c_char).is_null()
            && strcmp(method, b"PUT\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            return b"camera_collection.members.update\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if strcmp(method, b"PUT\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return b"camera_collection.update\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if strcmp(method, b"DELETE\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return b"camera_collection.delete\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_log_sensitive_operation_begin(
    mut req: *const http_request_t,
    mut context: *mut audit_sensitive_operation_context_t,
) {
    if context.is_null() {
        return;
    }
    memset(
        context as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<audit_sensitive_operation_context_t>(),
    );
    if req.is_null() {
        return;
    }
    let mut operation: *const ::core::ffi::c_char = camera_configuration_operation(
        req,
        &raw mut (*context).target_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        &raw mut (*context).identity as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        &raw mut (*context).identity_is_camera_uuid,
        &raw mut (*context).action,
    );
    if operation.is_null() {
        return;
    }
    (*context).applicable = r#true != 0;
    safe_strcpy(
        &raw mut (*context).operation as *mut ::core::ffi::c_char,
        operation,
        ::core::mem::size_of::<[::core::ffi::c_char; 48]>(),
        0 as size_t,
    );
    (*context).has_user =
        httpd_check_action_access(req, &raw mut (*context).user) != 0 as ::core::ffi::c_int;
    if (*context).identity_is_camera_uuid {
        safe_strcpy(
            &raw mut (*context).target_uuid as *mut ::core::ffi::c_char,
            &raw mut (*context).identity as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
    } else if strcmp(
        &raw mut (*context).target_type as *mut ::core::ffi::c_char,
        b"camera\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && (*context).identity[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
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
        if db_fleet_camera_find_by_name(
            &raw mut (*context).identity as *mut ::core::ffi::c_char,
            &raw mut camera,
        ) == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*context).target_uuid as *mut ::core::ffi::c_char,
                &raw mut camera.camera_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
        }
    } else if (*context).identity[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        safe_strcpy(
            &raw mut (*context).target_uuid as *mut ::core::ffi::c_char,
            &raw mut (*context).identity as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
    }
}
unsafe extern "C" fn sensitive_target_from_response(
    mut res: *const http_response_t,
    mut context: *mut audit_sensitive_operation_context_t,
) {
    if res.is_null()
        || context.is_null()
        || (*context).target_uuid[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || (*res).status_code < 200 as ::core::ffi::c_int
        || (*res).status_code >= 400 as ::core::ffi::c_int
        || (*res).body.is_null()
        || (*res).body_length == 0 as size_t
    {
        return;
    }
    let mut body: *mut cJSON = cJSON_ParseWithLength(
        (*res).body as *const ::core::ffi::c_char,
        (*res).body_length,
    );
    if cJSON_IsObject(body) == 0 {
        cJSON_Delete(body);
        return;
    }
    let mut uuid: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsString(uuid) == 0 {
        uuid = cJSON_GetObjectItemCaseSensitive(
            body,
            b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if cJSON_IsString(uuid) != 0
        && !(*uuid).valuestring.is_null()
        && strlen((*uuid).valuestring)
            == (CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int) as size_t
    {
        safe_strcpy(
            &raw mut (*context).target_uuid as *mut ::core::ffi::c_char,
            (*uuid).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
    }
    let mut stream_name: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if (*context).target_uuid[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        && cJSON_IsString(stream_name) != 0
        && !(*stream_name).valuestring.is_null()
    {
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
        if db_fleet_camera_find_by_name((*stream_name).valuestring, &raw mut camera)
            == 0 as ::core::ffi::c_int
        {
            safe_strcpy(
                &raw mut (*context).target_uuid as *mut ::core::ffi::c_char,
                &raw mut camera.camera_uuid as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
        }
    }
    cJSON_Delete(body);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_log_sensitive_operation_end(
    mut req: *const http_request_t,
    mut res: *const http_response_t,
    mut context: *mut audit_sensitive_operation_context_t,
) {
    if req.is_null()
        || res.is_null()
        || context.is_null()
        || !(*context).applicable
        || !(*context).has_user
        || (*res).status_code <= 0 as ::core::ffi::c_int
        || (*res).status_code == 401 as ::core::ffi::c_int
        || (*res).status_code == 403 as ::core::ffi::c_int
    {
        return;
    }
    sensitive_target_from_response(res, context);
    let mut outcome: *const ::core::ffi::c_char = if (*res).status_code >= 200 as ::core::ffi::c_int
        && (*res).status_code < 400 as ::core::ffi::c_int
    {
        b"success\0".as_ptr() as *const ::core::ffi::c_char
    } else if (*res).status_code >= 500 as ::core::ffi::c_int {
        b"error\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"failure\0".as_ptr() as *const ::core::ffi::c_char
    };
    let mut audit_details: *mut cJSON = cJSON_CreateObject();
    if !audit_details.is_null() {
        cJSON_AddNumberToObject(
            audit_details,
            b"http_status\0".as_ptr() as *const ::core::ffi::c_char,
            (*res).status_code as ::core::ffi::c_double,
        );
    }
    let mut metadata: *const authorization_action_metadata_t =
        authorization_action_metadata((*context).action);
    audit_log_operation(
        req,
        &raw mut (*context).user,
        if !metadata.is_null() {
            (*metadata).key
        } else {
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char
        },
        &raw mut (*context).target_type as *mut ::core::ffi::c_char,
        if (*context).target_uuid[0usize] as ::core::ffi::c_int != 0 {
            &raw mut (*context).target_uuid as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        &raw mut (*context).operation as *mut ::core::ffi::c_char,
        outcome,
        audit_details,
    );
    cJSON_Delete(audit_details);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn audit_log_sensitive_operation_outcome(
    mut req: *const http_request_t,
    mut res: *const http_response_t,
) {
    let mut context: audit_sensitive_operation_context_t = audit_sensitive_operation_context_t {
        applicable: false,
        has_user: false,
        identity_is_camera_uuid: false,
        action: authorization_action_t::AUTHZ_LIVE_VIEW,
        user: user_t {
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
        },
        operation: [0; 48],
        target_type: [0; 32],
        identity: [0; 256],
        target_uuid: [0; 37],
    };
    audit_log_sensitive_operation_begin(req, &raw mut context);
    audit_log_sensitive_operation_end(req, res, &raw mut context);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
