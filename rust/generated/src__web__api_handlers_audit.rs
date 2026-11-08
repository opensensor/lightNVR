use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
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
    unsafe fn cJSON_AddArrayToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn open_memstream(
        __bufloc: *mut *mut ::core::ffi::c_char,
        __sizeloc: *mut size_t,
    ) -> *mut FILE;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn authorization_action_catalog(
        action_count: *mut ::core::ffi::c_int,
    ) -> *const authorization_action_metadata_t;
    unsafe fn authorization_action_from_key(
        key: *const ::core::ffi::c_char,
    ) -> authorization_action_t;
    unsafe fn db_audit_query(
        query: *const audit_query_t,
        page: *mut audit_page_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_audit_page_free(page: *mut audit_page_t);
    unsafe fn db_audit_get_retention_days(
        retention_days: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_audit_set_retention_days(retention_days: ::core::ffi::c_int)
    -> ::core::ffi::c_int;
    unsafe fn db_audit_prune(deleted_count: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn audit_decision_mode_name(mode: audit_decision_mode_t) -> *const ::core::ffi::c_char;
    unsafe fn audit_decision_mode_from_name(
        name: *const ::core::ffi::c_char,
        mode: *mut audit_decision_mode_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn copy_trimmed_value(
        output: *mut ::core::ffi::c_char,
        output_size: size_t,
        input: *const ::core::ffi::c_char,
        input_size: size_t,
    ) -> size_t;
    unsafe fn http_request_get_query_param(
        req: *const http_request_t,
        name: *const ::core::ffi::c_char,
        value: *mut ::core::ffi::c_char,
        value_len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_add_header(
        res: *mut http_response_t,
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_set_body(
        res: *mut http_response_t,
        body: *const ::core::ffi::c_char,
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
    unsafe fn http_response_add_cors_headers(res: *mut http_response_t);
    unsafe fn audit_log_append(
        req: *const http_request_t,
        user: *const user_t,
        action: *const ::core::ffi::c_char,
        target_type: *const ::core::ffi::c_char,
        target_uuid: *const ::core::ffi::c_char,
        outcome: *const ::core::ffi::c_char,
        details: *const cJSON,
    );
    unsafe fn audit_log_get_decision_mode(action: authorization_action_t) -> audit_decision_mode_t;
    unsafe fn audit_log_set_decision_modes(
        modes: *const audit_decision_mode_t,
    ) -> ::core::ffi::c_int;
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub struct audit_event_t {
    pub id: int64_t,
    pub uuid: [::core::ffi::c_char; 37],
    pub occurred_at: int64_t,
    pub request_id: [::core::ffi::c_char; 65],
    pub principal_user_id: int64_t,
    pub principal_username: [::core::ffi::c_char; 64],
    pub auth_method: [::core::ffi::c_char; 24],
    pub api_token_uuid: [::core::ffi::c_char; 37],
    pub action: [::core::ffi::c_char; 96],
    pub target_type: [::core::ffi::c_char; 48],
    pub target_uuid: [::core::ffi::c_char; 128],
    pub outcome: [::core::ffi::c_char; 16],
    pub remote_address: [::core::ffi::c_char; 64],
    pub details_json: [::core::ffi::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audit_query_t {
    pub page: ::core::ffi::c_int,
    pub page_size: ::core::ffi::c_int,
    pub since: int64_t,
    pub until: int64_t,
    pub principal_user_id: int64_t,
    pub action: [::core::ffi::c_char; 96],
    pub outcome: [::core::ffi::c_char; 16],
    pub target_uuid: [::core::ffi::c_char; 128],
    pub request_id: [::core::ffi::c_char; 65],
    pub event_type: [::core::ffi::c_char; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audit_page_t {
    pub events: *mut audit_event_t,
    pub count: ::core::ffi::c_int,
    pub total: int64_t,
    pub page: ::core::ffi::c_int,
    pub page_size: ::core::ffi::c_int,
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
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const AUDIT_PAGE_SIZE_MAX: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const AUDIT_RETENTION_DEFAULT_DAYS: ::core::ffi::c_int = 365 as ::core::ffi::c_int;
pub const AUDIT_RETENTION_MAX_DAYS: ::core::ffi::c_int = 3650 as ::core::ffi::c_int;
pub const AUDIT_SUMMARY_WINDOW_SECONDS: ::core::ffi::c_int = 900 as ::core::ffi::c_int;
static mut audit_settings_put_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
unsafe extern "C" fn authorize_audit_admin(
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
        authorization_action_t::AUTHZ_SYSTEM_ADMIN,
        ::core::ptr::null::<fleet_camera_t>(),
        user,
        &raw mut evaluation,
    ) != 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_int64_query(
    mut req: *const http_request_t,
    mut name: *const ::core::ffi::c_char,
    mut default_value: int64_t,
    mut minimum: int64_t,
    mut maximum: int64_t,
    mut output: *mut int64_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut value: [::core::ffi::c_char; 64] = [0; 64];
    if http_request_get_query_param(
        req,
        name,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) < 0 as ::core::ffi::c_int
    {
        *output = default_value;
        return r#true != 0;
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
        || parsed < minimum as ::core::ffi::c_longlong
        || parsed > maximum as ::core::ffi::c_longlong
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid audit query parameter\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    *output = parsed as int64_t;
    return r#true != 0;
}
unsafe extern "C" fn read_text_query(
    mut req: *const http_request_t,
    mut name: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut output_size: size_t,
) {
    *output.offset(0isize) = '\0' as ::core::ffi::c_char;
    let mut value: [::core::ffi::c_char; 1024] = [0; 1024];
    if http_request_get_query_param(
        req,
        name,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
    ) >= 0 as ::core::ffi::c_int
    {
        copy_trimmed_value(
            output,
            output_size,
            &raw mut value as *mut ::core::ffi::c_char,
            0 as size_t,
        );
    }
}
unsafe extern "C" fn parse_audit_query(
    mut req: *const http_request_t,
    mut query: *mut audit_query_t,
    mut res: *mut http_response_t,
) -> bool {
    memset(
        query as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<audit_query_t>(),
    );
    let mut page: int64_t = 1 as int64_t;
    let mut page_size: int64_t = 100 as int64_t;
    if !parse_int64_query(
        req,
        b"page\0".as_ptr() as *const ::core::ffi::c_char,
        1 as int64_t,
        1 as int64_t,
        1000000 as int64_t,
        &raw mut page,
        res,
    ) || !parse_int64_query(
        req,
        b"page_size\0".as_ptr() as *const ::core::ffi::c_char,
        100 as int64_t,
        1 as int64_t,
        AUDIT_PAGE_SIZE_MAX as int64_t,
        &raw mut page_size,
        res,
    ) || !parse_int64_query(
        req,
        b"since\0".as_ptr() as *const ::core::ffi::c_char,
        0 as int64_t,
        0 as int64_t,
        INT64_MAX as int64_t,
        &raw mut (*query).since,
        res,
    ) || !parse_int64_query(
        req,
        b"until\0".as_ptr() as *const ::core::ffi::c_char,
        0 as int64_t,
        0 as int64_t,
        INT64_MAX as int64_t,
        &raw mut (*query).until,
        res,
    ) || !parse_int64_query(
        req,
        b"principal_user_id\0".as_ptr() as *const ::core::ffi::c_char,
        0 as int64_t,
        0 as int64_t,
        INT64_MAX as int64_t,
        &raw mut (*query).principal_user_id,
        res,
    ) {
        return r#false != 0;
    }
    if (*query).since > 0 as int64_t
        && (*query).until > 0 as int64_t
        && (*query).since > (*query).until
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Audit since must not exceed until\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    (*query).page = page as ::core::ffi::c_int;
    (*query).page_size = page_size as ::core::ffi::c_int;
    read_text_query(
        req,
        b"action\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*query).action as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>(),
    );
    read_text_query(
        req,
        b"outcome\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*query).outcome as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
    );
    read_text_query(
        req,
        b"target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*query).target_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    );
    read_text_query(
        req,
        b"request_id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*query).request_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
    );
    read_text_query(
        req,
        b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*query).event_type as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    );
    return r#true != 0;
}
unsafe extern "C" fn audit_event_to_json(mut event: *const audit_event_t) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_CreateObject();
    if item.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        item,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        item,
        b"occurred_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*event).occurred_at as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        item,
        b"request_id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).request_id as *const ::core::ffi::c_char,
    );
    if (*event).principal_user_id > 0 as int64_t {
        cJSON_AddNumberToObject(
            item,
            b"principal_user_id\0".as_ptr() as *const ::core::ffi::c_char,
            (*event).principal_user_id as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(
            item,
            b"principal_user_id\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddStringToObject(
        item,
        b"principal_username\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).principal_username as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        item,
        b"auth_method\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).auth_method as *const ::core::ffi::c_char,
    );
    if (*event).api_token_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            item,
            b"api_token_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*event).api_token_uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            item,
            b"api_token_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddStringToObject(
        item,
        b"action\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).action as *const ::core::ffi::c_char,
    );
    if (*event).target_type[0usize] != 0 {
        cJSON_AddStringToObject(
            item,
            b"target_type\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*event).target_type as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            item,
            b"target_type\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*event).target_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            item,
            b"target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*event).target_uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            item,
            b"target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddStringToObject(
        item,
        b"outcome\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).outcome as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        item,
        b"remote_address\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*event).remote_address as *const ::core::ffi::c_char,
    );
    let mut details: *mut cJSON =
        cJSON_Parse(&raw const (*event).details_json as *const ::core::ffi::c_char);
    if cJSON_IsObject(details) != 0 {
        cJSON_AddItemToObject(
            item,
            b"details\0".as_ptr() as *const ::core::ffi::c_char,
            details,
        );
    } else {
        cJSON_Delete(details);
        cJSON_AddNullToObject(item, b"details\0".as_ptr() as *const ::core::ffi::c_char);
    }
    return item;
}
unsafe extern "C" fn set_page_json(mut res: *mut http_response_t, mut page: *const audit_page_t) {
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut events: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || events.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(events);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddNumberToObject(
        root,
        b"page\0".as_ptr() as *const ::core::ffi::c_char,
        (*page).page as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"page_size\0".as_ptr() as *const ::core::ffi::c_char,
        (*page).page_size as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        (*page).count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"total\0".as_ptr() as *const ::core::ffi::c_char,
        (*page).total as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"events\0".as_ptr() as *const ::core::ffi::c_char,
        events,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*page).count {
        let mut item: *mut cJSON = audit_event_to_json((*page).events.offset(i as isize));
        if item.is_null() {
            cJSON_Delete(root);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddItemToArray(events, item);
        i += 1;
    }
    let mut body: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_audit_events(
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
    if !authorize_audit_admin(req, res, &raw mut user) {
        return;
    }
    let mut query: audit_query_t = audit_query_t {
        page: 0,
        page_size: 0,
        since: 0,
        until: 0,
        principal_user_id: 0,
        action: [0; 96],
        outcome: [0; 16],
        target_uuid: [0; 128],
        request_id: [0; 65],
        event_type: [0; 128],
    };
    if !parse_audit_query(req, &raw mut query, res) {
        return;
    }
    let mut page: audit_page_t = audit_page_t {
        events: ::core::ptr::null_mut::<audit_event_t>(),
        count: 0,
        total: 0,
        page: 0,
        page_size: 0,
    };
    if db_audit_query(&raw mut query, &raw mut page) != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to query audit events\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    set_page_json(res, &raw mut page);
    db_audit_page_free(&raw mut page);
}
unsafe extern "C" fn csv_cell(mut stream: *mut FILE, mut value: *const ::core::ffi::c_char) {
    fputc('"' as ::core::ffi::c_int, stream);
    if !value.is_null() {
        let mut cursor: *const ::core::ffi::c_char = value;
        while *cursor as ::core::ffi::c_int != 0
            && (*cursor as ::core::ffi::c_uchar as ::core::ffi::c_int <= 0x20 as ::core::ffi::c_int
                || *cursor as ::core::ffi::c_uchar as ::core::ffi::c_int
                    == 0x7f as ::core::ffi::c_int)
        {
            cursor = cursor.offset(1);
        }
        if *cursor as ::core::ffi::c_int != 0
            && !strchr(
                b"=+-@\0".as_ptr() as *const ::core::ffi::c_char,
                *cursor as ::core::ffi::c_int,
            )
            .is_null()
        {
            fputc('\'' as ::core::ffi::c_int, stream);
        }
    }
    let mut cursor_0: *const ::core::ffi::c_char = if !value.is_null() {
        value
    } else {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    };
    while *cursor_0 != 0 {
        if *cursor_0 as ::core::ffi::c_int == '"' as ::core::ffi::c_int {
            fputc('"' as ::core::ffi::c_int, stream);
        }
        fputc(*cursor_0 as ::core::ffi::c_int, stream);
        cursor_0 = cursor_0.offset(1);
    }
    fputc('"' as ::core::ffi::c_int, stream);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_audit_export(
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
    if !authorize_audit_admin(req, res, &raw mut user) {
        return;
    }
    let mut query: audit_query_t = audit_query_t {
        page: 0,
        page_size: 0,
        since: 0,
        until: 0,
        principal_user_id: 0,
        action: [0; 96],
        outcome: [0; 16],
        target_uuid: [0; 128],
        request_id: [0; 65],
        event_type: [0; 128],
    };
    if !parse_audit_query(req, &raw mut query, res) {
        return;
    }
    let mut page: audit_page_t = audit_page_t {
        events: ::core::ptr::null_mut::<audit_event_t>(),
        count: 0,
        total: 0,
        page: 0,
        page_size: 0,
    };
    if db_audit_query(&raw mut query, &raw mut page) != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to query audit events\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut body_size: size_t = 0 as size_t;
    let mut stream: *mut FILE = open_memstream(&raw mut body, &raw mut body_size);
    if stream.is_null() {
        db_audit_page_free(&raw mut page);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create audit export\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    fputs(
        b"uuid,occurred_at,request_id,principal_user_id,principal_username,auth_method,api_token_uuid,action,target_type,target_uuid,outcome,remote_address,details_json\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stream,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < page.count {
        let mut event: *const audit_event_t = page.events.offset(i as isize);
        let mut occurred_at: [::core::ffi::c_char; 32] = [0; 32];
        let mut principal_user_id: [::core::ffi::c_char; 32] = [0; 32];
        snprintf(
            &raw mut occurred_at as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
            (*event).occurred_at as ::core::ffi::c_longlong,
        );
        if (*event).principal_user_id > 0 as int64_t {
            snprintf(
                &raw mut principal_user_id as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
                (*event).principal_user_id as ::core::ffi::c_longlong,
            );
        } else {
            principal_user_id[0usize] = '\0' as ::core::ffi::c_char;
        }
        let mut cells: [*const ::core::ffi::c_char; 13] = [
            &raw const (*event).uuid as *const ::core::ffi::c_char,
            &raw mut occurred_at as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*event).request_id as *const ::core::ffi::c_char,
            &raw mut principal_user_id as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
            &raw const (*event).principal_username as *const ::core::ffi::c_char,
            &raw const (*event).auth_method as *const ::core::ffi::c_char,
            &raw const (*event).api_token_uuid as *const ::core::ffi::c_char,
            &raw const (*event).action as *const ::core::ffi::c_char,
            &raw const (*event).target_type as *const ::core::ffi::c_char,
            &raw const (*event).target_uuid as *const ::core::ffi::c_char,
            &raw const (*event).outcome as *const ::core::ffi::c_char,
            &raw const (*event).remote_address as *const ::core::ffi::c_char,
            &raw const (*event).details_json as *const ::core::ffi::c_char,
        ];
        let mut column: size_t = 0 as size_t;
        while column
            < ::core::mem::size_of::<[*const ::core::ffi::c_char; 13]>()
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
        {
            if column > 0 as size_t {
                fputc(',' as ::core::ffi::c_int, stream);
            }
            csv_cell(stream, cells[column]);
            column = column.wrapping_add(1);
        }
        fputc('\n' as ::core::ffi::c_int, stream);
        i += 1;
    }
    if fclose(stream) != 0 as ::core::ffi::c_int || body.is_null() {
        free(body as *mut ::core::ffi::c_void);
        db_audit_page_free(&raw mut page);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create audit export\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if http_response_set_body(res, body) != 0 as ::core::ffi::c_int {
        free(body as *mut ::core::ffi::c_void);
        db_audit_page_free(&raw mut page);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create audit export response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    (*res).status_code = 200 as ::core::ffi::c_int;
    safe_strcpy(
        &raw mut (*res).content_type as *mut ::core::ffi::c_char,
        b"text/csv; charset=utf-8\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        0 as size_t,
    );
    http_response_add_header(
        res,
        b"Content-Disposition\0".as_ptr() as *const ::core::ffi::c_char,
        b"attachment; filename=lightnvr-audit.csv\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut total: [::core::ffi::c_char; 32] = [0; 32];
    snprintf(
        &raw mut total as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
        page.total as ::core::ffi::c_longlong,
    );
    http_response_add_header(
        res,
        b"X-Total-Count\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut total as *mut ::core::ffi::c_char,
    );
    http_response_add_cors_headers(res);
    free(body as *mut ::core::ffi::c_void);
    db_audit_page_free(&raw mut page);
}
unsafe extern "C" fn respond_audit_settings(
    mut res: *mut http_response_t,
    mut retention_days: ::core::ffi::c_int,
    mut pruned_events: ::core::ffi::c_int,
    mut include_pruned: bool,
) {
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut modes: *mut cJSON = if !root.is_null() {
        cJSON_AddArrayToObject(
            root,
            b"allowed_decision_modes\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut catalog: *const authorization_action_metadata_t =
        authorization_action_catalog(&raw mut count);
    if root.is_null() || modes.is_null() || catalog.is_null() {
        cJSON_Delete(root);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddNumberToObject(
        root,
        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
        retention_days as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"summary_window_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        AUDIT_SUMMARY_WINDOW_SECONDS as ::core::ffi::c_double,
    );
    if include_pruned {
        cJSON_AddNumberToObject(
            root,
            b"pruned_events\0".as_ptr() as *const ::core::ffi::c_char,
            pruned_events as ::core::ffi::c_double,
        );
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut item: *mut cJSON = cJSON_CreateObject();
        if !item.is_null() {
            cJSON_AddStringToObject(
                item,
                b"action\0".as_ptr() as *const ::core::ffi::c_char,
                (*catalog.offset(i as isize)).key,
            );
            cJSON_AddStringToObject(
                item,
                b"category\0".as_ptr() as *const ::core::ffi::c_char,
                (*catalog.offset(i as isize)).category,
            );
            cJSON_AddStringToObject(
                item,
                b"description\0".as_ptr() as *const ::core::ffi::c_char,
                (*catalog.offset(i as isize)).description,
            );
            cJSON_AddStringToObject(
                item,
                b"mode\0".as_ptr() as *const ::core::ffi::c_char,
                audit_decision_mode_name(audit_log_get_decision_mode(
                    (*catalog.offset(i as isize)).action,
                )),
            );
            cJSON_AddItemToArray(modes, item);
        }
        i += 1;
    }
    let mut body: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_audit_settings(
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
    if !authorize_audit_admin(req, res, &raw mut user) {
        return;
    }
    let mut retention_days: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if db_audit_get_retention_days(&raw mut retention_days) != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load audit settings\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    respond_audit_settings(res, retention_days, 0 as ::core::ffi::c_int, r#false != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_audit_settings(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut count: ::core::ffi::c_int = 0;
    let mut catalog: *const authorization_action_metadata_t =
        ::core::ptr::null::<authorization_action_metadata_t>();
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
    if !authorize_audit_admin(req, res, &raw mut user) {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    if cJSON_IsObject(body) == 0 {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Audit settings body must be a JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut retention: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut modes_json: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"allowed_decision_modes\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if retention.is_null() && modes_json.is_null() {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Provide retention_days, allowed_decision_modes, or both\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut retention_days: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !retention.is_null() {
        let mut number: ::core::ffi::c_double = if cJSON_IsNumber(retention) != 0 {
            (*retention).valuedouble
        } else {
            0 as ::core::ffi::c_int as ::core::ffi::c_double
        };
        retention_days = number as ::core::ffi::c_int;
        if cJSON_IsNumber(retention) == 0
            || number != retention_days as ::core::ffi::c_double
            || retention_days < 1 as ::core::ffi::c_int
            || retention_days > AUDIT_RETENTION_MAX_DAYS
        {
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"retention_days must be 1-3650\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    pthread_mutex_lock(&raw mut audit_settings_put_mutex);
    let mut changes: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    let mut deleted_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut previous: [audit_decision_mode_t; 20] =
        [audit_decision_mode_t::AUDIT_DECISION_MODE_RECORD; 20];
    let mut next: [audit_decision_mode_t; 20] =
        [audit_decision_mode_t::AUDIT_DECISION_MODE_RECORD; 20];
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < authorization_action_t::AUTHZ_ACTION_COUNT.0 {
        next[i as usize] = audit_log_get_decision_mode(authorization_action_t(i));
        previous[i as usize] = next[i as usize];
        i += 1;
    }
    '_unlock_and_return: {
        's_180: {
            if !modes_json.is_null() {
                if cJSON_IsObject(modes_json) == 0 {
                    cJSON_Delete(body);
                    http_response_set_json_error(
                        res,
                        400 as ::core::ffi::c_int,
                        b"allowed_decision_modes must be an object\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    break '_unlock_and_return;
                } else {
                    let mut item: *const cJSON = (*modes_json).child;
                    loop {
                        if item.is_null() {
                            break 's_180;
                        }
                        let mut message: [::core::ffi::c_char; 160] = [0; 160];
                        let mut action: authorization_action_t =
                            authorization_action_from_key((*item).string);
                        if action.0 == authorization_action_t::AUTHZ_ACTION_INVALID.0 {
                            snprintf(
                                &raw mut message as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
                                b"Unknown audit action: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                if !(*item).string.is_null() {
                                    (*item).string
                                } else {
                                    b"\0".as_ptr() as *const ::core::ffi::c_char
                                },
                            );
                            cJSON_Delete(body);
                            http_response_set_json_error(
                                res,
                                400 as ::core::ffi::c_int,
                                &raw mut message as *mut ::core::ffi::c_char,
                            );
                            break '_unlock_and_return;
                        } else {
                            let mut mode: audit_decision_mode_t =
                                audit_decision_mode_t::AUDIT_DECISION_MODE_RECORD;
                            if cJSON_IsString(item) == 0
                                || audit_decision_mode_from_name((*item).valuestring, &raw mut mode)
                                    != 0 as ::core::ffi::c_int
                            {
                                snprintf(
                                    &raw mut message as *mut ::core::ffi::c_char,
                                    ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
                                    b"Invalid audit decision mode for %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*item).string,
                                );
                                cJSON_Delete(body);
                                http_response_set_json_error(
                                    res,
                                    400 as ::core::ffi::c_int,
                                    &raw mut message as *mut ::core::ffi::c_char,
                                );
                                break '_unlock_and_return;
                            } else {
                                next[action.0 as usize] = mode;
                                item = (*item).next;
                            }
                        }
                    }
                }
            }
        }
        if !retention.is_null() {
            let mut previous_days: ::core::ffi::c_int = AUDIT_RETENTION_DEFAULT_DAYS;
            if db_audit_get_retention_days(&raw mut previous_days) != 0 as ::core::ffi::c_int {
                cJSON_Delete(body);
                http_response_set_json_error(
                    res,
                    500 as ::core::ffi::c_int,
                    b"Failed to load audit settings\0".as_ptr() as *const ::core::ffi::c_char,
                );
                break '_unlock_and_return;
            } else if db_audit_set_retention_days(retention_days) != 0 as ::core::ffi::c_int {
                cJSON_Delete(body);
                http_response_set_json_error(
                    res,
                    500 as ::core::ffi::c_int,
                    b"Failed to save audit settings\0".as_ptr() as *const ::core::ffi::c_char,
                );
                break '_unlock_and_return;
            } else if db_audit_prune(&raw mut deleted_count) != 0 as ::core::ffi::c_int {
                cJSON_Delete(body);
                http_response_set_json_error(
                    res,
                    500 as ::core::ffi::c_int,
                    b"Audit setting saved but pruning failed\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                break '_unlock_and_return;
            } else {
                let mut details: *mut cJSON = cJSON_CreateObject();
                if !details.is_null() {
                    cJSON_AddStringToObject(
                        details,
                        b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
                        b"audit.retention_update\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    cJSON_AddNumberToObject(
                        details,
                        b"previous_days\0".as_ptr() as *const ::core::ffi::c_char,
                        previous_days as ::core::ffi::c_double,
                    );
                    cJSON_AddNumberToObject(
                        details,
                        b"retention_days\0".as_ptr() as *const ::core::ffi::c_char,
                        retention_days as ::core::ffi::c_double,
                    );
                    cJSON_AddNumberToObject(
                        details,
                        b"pruned_events\0".as_ptr() as *const ::core::ffi::c_char,
                        deleted_count as ::core::ffi::c_double,
                    );
                }
                audit_log_append(
                    req,
                    &raw mut user,
                    b"audit.retention.update\0".as_ptr() as *const ::core::ffi::c_char,
                    b"audit_log\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"success\0".as_ptr() as *const ::core::ffi::c_char,
                    details,
                );
                cJSON_Delete(details);
            }
        } else if db_audit_get_retention_days(&raw mut retention_days) != 0 as ::core::ffi::c_int {
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to load audit settings\0".as_ptr() as *const ::core::ffi::c_char,
            );
            break '_unlock_and_return;
        }
        changes = cJSON_CreateArray();
        count = 0 as ::core::ffi::c_int;
        catalog = authorization_action_catalog(&raw mut count);
        's_311: {
            if changes.is_null() {
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                loop {
                    if !(!catalog.is_null() && i_0 < count) {
                        break 's_311;
                    }
                    if previous[(*catalog.offset(i_0 as isize)).action.0 as usize].0
                        != next[(*catalog.offset(i_0 as isize)).action.0 as usize].0
                    {
                        cJSON_Delete(body);
                        http_response_set_json_error(
                            res,
                            500 as ::core::ffi::c_int,
                            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        break '_unlock_and_return;
                    } else {
                        i_0 += 1;
                    }
                }
            }
        }
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !changes.is_null() && !catalog.is_null() && i_1 < count {
            let mut action_0: authorization_action_t = (*catalog.offset(i_1 as isize)).action;
            if previous[action_0.0 as usize].0 != next[action_0.0 as usize].0 {
                let mut change: *mut cJSON = cJSON_CreateObject();
                if change.is_null()
                    || cJSON_AddStringToObject(
                        change,
                        b"action\0".as_ptr() as *const ::core::ffi::c_char,
                        (*catalog.offset(i_1 as isize)).key,
                    )
                    .is_null()
                    || cJSON_AddStringToObject(
                        change,
                        b"previous\0".as_ptr() as *const ::core::ffi::c_char,
                        audit_decision_mode_name(previous[action_0.0 as usize]),
                    )
                    .is_null()
                    || cJSON_AddStringToObject(
                        change,
                        b"mode\0".as_ptr() as *const ::core::ffi::c_char,
                        audit_decision_mode_name(next[action_0.0 as usize]),
                    )
                    .is_null()
                    || cJSON_AddItemToArray(changes, change) == 0
                {
                    cJSON_Delete(change);
                    cJSON_Delete(changes);
                    changes = ::core::ptr::null_mut::<cJSON>();
                    cJSON_Delete(body);
                    http_response_set_json_error(
                        res,
                        500 as ::core::ffi::c_int,
                        b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    break '_unlock_and_return;
                }
            }
            i_1 += 1;
        }
        if !changes.is_null() && cJSON_GetArraySize(changes) > 0 as ::core::ffi::c_int {
            let mut details_0: *mut cJSON = cJSON_CreateObject();
            if details_0.is_null()
                || cJSON_AddStringToObject(
                    details_0,
                    b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
                    b"audit.decision_modes.update\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .is_null()
                || cJSON_AddItemToObject(
                    details_0,
                    b"changes\0".as_ptr() as *const ::core::ffi::c_char,
                    changes,
                ) == 0
            {
                cJSON_Delete(details_0);
                cJSON_Delete(changes);
                changes = ::core::ptr::null_mut::<cJSON>();
                cJSON_Delete(body);
                http_response_set_json_error(
                    res,
                    500 as ::core::ffi::c_int,
                    b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
                );
                break '_unlock_and_return;
            } else {
                changes = ::core::ptr::null_mut::<cJSON>();
                if audit_log_set_decision_modes(&raw mut next as *mut audit_decision_mode_t)
                    != 0 as ::core::ffi::c_int
                {
                    cJSON_Delete(details_0);
                    cJSON_Delete(body);
                    http_response_set_json_error(
                        res,
                        500 as ::core::ffi::c_int,
                        b"Failed to save audit decision modes\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    break '_unlock_and_return;
                } else {
                    audit_log_append(
                        req,
                        &raw mut user,
                        b"audit.settings.update\0".as_ptr() as *const ::core::ffi::c_char,
                        b"audit_log\0".as_ptr() as *const ::core::ffi::c_char,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"success\0".as_ptr() as *const ::core::ffi::c_char,
                        details_0,
                    );
                    cJSON_Delete(details_0);
                }
            }
        }
        cJSON_Delete(changes);
        changes = ::core::ptr::null_mut::<cJSON>();
        cJSON_Delete(body);
        body = ::core::ptr::null_mut::<cJSON>();
        respond_audit_settings(res, retention_days, deleted_count, !retention.is_null());
    }
    pthread_mutex_unlock(&raw mut audit_settings_put_mutex);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
