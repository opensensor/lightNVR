use ::c2rust_bitfields;
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn fseeko(
        __stream: *mut FILE,
        __off: __off_t,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn ftello(__stream: *mut FILE) -> __off_t;
    unsafe fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn storage_source_resolve(
        recording_id: uint64_t,
        path: *mut ::core::ffi::c_char,
        error: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn getrandom(
        __buffer: *mut ::core::ffi::c_void,
        __length: size_t,
        __flags: ::core::ffi::c_uint,
    ) -> ssize_t;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
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
    unsafe fn http_serve_file(
        req: *const http_request_t,
        res: *const http_response_t,
        file_path: *const ::core::ffi::c_char,
        content_type: *const ::core::ffi::c_char,
        extra_headers: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn recording_id_parse_json(item: *const cJSON, id: *mut uint64_t) -> bool;
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
    unsafe fn httpd_evaluate_camera_identity_action(
        user: *const user_t,
        action: authorization_action_t,
        camera_uuid: *const ::core::ffi::c_char,
        legacy_stream_name: *const ::core::ffi::c_char,
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
    unsafe fn log_set_thread_context(
        component: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
    );
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_recording_metadata_by_id(
        id: uint64_t,
        metadata: *mut recording_metadata_t,
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
    unsafe fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn mkstemp(__template: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type ssize_t = isize;
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
pub type pthread_t = ::core::ffi::c_ulong;
pub type pthread_once_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
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
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const PTHREAD_CREATE_JOINABLE: Self = Self(0);
    pub const PTHREAD_CREATE_DETACHED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
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
pub struct dl_status_t(pub ::core::ffi::c_uint);
impl dl_status_t {
    pub const DL_PENDING: Self = Self(0);
    pub const DL_RUNNING: Self = Self(1);
    pub const DL_COMPLETE: Self = Self(2);
    pub const DL_ERROR: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct batch_dl_job_t {
    pub job_id: [::core::ffi::c_char; 64],
    pub owner_user_id: int64_t,
    pub owner_username: [::core::ffi::c_char; 64],
    pub owner_api_token_uuid: [::core::ffi::c_char; 37],
    pub owner_auth_method: [::core::ffi::c_char; 24],
    pub audit_request_id: [::core::ffi::c_char; 65],
    pub audit_client_ip: [::core::ffi::c_char; 64],
    pub zip_filename: [::core::ffi::c_char; 192],
    pub tmp_path: [::core::ffi::c_char; 512],
    pub ids: [uint64_t; 200],
    pub id_count: ::core::ffi::c_int,
    pub status: dl_status_t,
    pub current: ::core::ffi::c_int,
    pub total: ::core::ffi::c_int,
    pub error: [::core::ffi::c_char; 256],
    pub created_at: time_t,
    pub updated_at: time_t,
    pub is_active: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dl_thread_arg_t {
    pub job_id: [::core::ffi::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entry_t {
    pub name: [::core::ffi::c_char; 128],
    pub crc: uint32_t,
    pub size: uint32_t,
    pub offset: uint32_t,
    pub dos_date: uint16_t,
    pub dos_time: uint16_t,
}
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const PTHREAD_ONCE_INIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_COMPONENT: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"RecordingsAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
static mut s_crc_table: [uint32_t; 256] = [0; 256];
static mut s_crc_once: pthread_once_t = PTHREAD_ONCE_INIT;
unsafe extern "C" fn init_crc32() {
    let mut i: uint32_t = 0 as uint32_t;
    while i < 256 as uint32_t {
        let mut c: uint32_t = i;
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < 8 as ::core::ffi::c_int {
            c = if c & 1 as uint32_t != 0 {
                0xedb88320 as uint32_t ^ c >> 1 as ::core::ffi::c_int
            } else {
                c >> 1 as ::core::ffi::c_int
            };
            j += 1;
        }
        s_crc_table[i as usize] = c;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn zip_write_u16(mut f: *mut FILE, mut v: uint16_t) -> ::core::ffi::c_int {
    let bytes: [uint8_t; 2] = [
        (v as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t,
        (v as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as uint8_t,
    ];
    return if fwrite(
        &raw const bytes as *const uint8_t as *const ::core::ffi::c_void,
        1 as size_t,
        ::core::mem::size_of::<[uint8_t; 2]>(),
        f,
    ) as usize
        == ::core::mem::size_of::<[uint8_t; 2]>()
    {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn zip_write_u32(mut f: *mut FILE, mut v: uint32_t) -> ::core::ffi::c_int {
    let bytes: [uint8_t; 4] = [
        (v & 0xff as uint32_t) as uint8_t,
        (v >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t,
        (v >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t,
        (v >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t,
    ];
    return if fwrite(
        &raw const bytes as *const uint8_t as *const ::core::ffi::c_void,
        1 as size_t,
        ::core::mem::size_of::<[uint8_t; 4]>(),
        f,
    ) as usize
        == ::core::mem::size_of::<[uint8_t; 4]>()
    {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn zip_write_local_header(
    mut zip: *mut FILE,
    mut name: *const ::core::ffi::c_char,
    mut dos_date: uint16_t,
    mut dos_time: uint16_t,
) -> ::core::ffi::c_int {
    let mut len: uint16_t = strlen(name) as uint16_t;
    return (zip_write_u32(zip, 0x4034b50 as uint32_t) != 0
        || zip_write_u16(zip, 20 as uint16_t) != 0
        || zip_write_u16(zip, 0 as uint16_t) != 0
        || zip_write_u16(zip, 0 as uint16_t) != 0
        || zip_write_u16(zip, dos_time) != 0
        || zip_write_u16(zip, dos_date) != 0
        || zip_write_u32(zip, 0 as uint32_t) != 0
        || zip_write_u32(zip, 0 as uint32_t) != 0
        || zip_write_u32(zip, 0 as uint32_t) != 0
        || zip_write_u16(zip, len) != 0
        || zip_write_u16(zip, 0 as uint16_t) != 0
        || fwrite(
            name as *const ::core::ffi::c_void,
            1 as size_t,
            len as size_t,
            zip,
        ) != len as ::core::ffi::c_ulong) as ::core::ffi::c_int;
}
unsafe extern "C" fn zip_write_central_entry(
    mut zip: *mut FILE,
    mut name: *const ::core::ffi::c_char,
    mut crc: uint32_t,
    mut size: uint32_t,
    mut dos_date: uint16_t,
    mut dos_time: uint16_t,
    mut local_offset: uint32_t,
) -> ::core::ffi::c_int {
    let mut len: uint16_t = strlen(name) as uint16_t;
    return (zip_write_u32(zip, 0x2014b50 as uint32_t) != 0
        || zip_write_u16(zip, 20 as uint16_t) != 0
        || zip_write_u16(zip, 20 as uint16_t) != 0
        || zip_write_u16(zip, 0 as uint16_t) != 0
        || zip_write_u16(zip, 0 as uint16_t) != 0
        || zip_write_u16(zip, dos_time) != 0
        || zip_write_u16(zip, dos_date) != 0
        || zip_write_u32(zip, crc) != 0
        || zip_write_u32(zip, size) != 0
        || zip_write_u32(zip, size) != 0
        || zip_write_u16(zip, len) != 0
        || zip_write_u16(zip, 0 as uint16_t) != 0
        || zip_write_u16(zip, 0 as uint16_t) != 0
        || zip_write_u16(zip, 0 as uint16_t) != 0
        || zip_write_u16(zip, 0 as uint16_t) != 0
        || zip_write_u32(zip, 0 as uint32_t) != 0
        || zip_write_u32(zip, local_offset) != 0
        || fwrite(
            name as *const ::core::ffi::c_void,
            1 as size_t,
            len as size_t,
            zip,
        ) != len as ::core::ffi::c_ulong) as ::core::ffi::c_int;
}
unsafe extern "C" fn zip_write_eocd(
    mut zip: *mut FILE,
    mut count: uint16_t,
    mut cd_size: uint32_t,
    mut cd_offset: uint32_t,
) -> ::core::ffi::c_int {
    return (zip_write_u32(zip, 0x6054b50 as uint32_t) != 0
        || zip_write_u16(zip, 0 as uint16_t) != 0
        || zip_write_u16(zip, 0 as uint16_t) != 0
        || zip_write_u16(zip, count) != 0
        || zip_write_u16(zip, count) != 0
        || zip_write_u32(zip, cd_size) != 0
        || zip_write_u32(zip, cd_offset) != 0
        || zip_write_u16(zip, 0 as uint16_t) != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn zip_copy_file(
    mut zip: *mut FILE,
    mut path: *const ::core::ffi::c_char,
    mut offset: uint32_t,
    mut max_size: uint32_t,
    mut size: *mut uint32_t,
    mut crc: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut src: *mut FILE = fopen(path, b"rb\0".as_ptr() as *const ::core::ffi::c_char);
    if src.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut before: stat = stat {
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
    let mut after: stat = stat {
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
    let mut failed: ::core::ffi::c_int =
        (fstat(fileno(src), &raw mut before) != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if failed == 0
        && (!(before.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
            || before.st_size < 0 as __off_t
            || before.st_size as uint64_t > max_size as uint64_t)
    {
        failed = 1 as ::core::ffi::c_int;
    }
    pthread_once(
        &raw mut s_crc_once,
        Some(init_crc32 as unsafe extern "C" fn() -> ()),
    );
    let mut buffer: [uint8_t; 65536] = [0; 65536];
    let mut total: uint64_t = 0 as uint64_t;
    let mut checksum: uint32_t = 0xffffffff as uint32_t;
    while failed == 0 {
        let mut n: size_t = fread(
            &raw mut buffer as *mut uint8_t as *mut ::core::ffi::c_void,
            1 as size_t,
            ::core::mem::size_of::<[uint8_t; 65536]>(),
            src,
        ) as size_t;
        if n == 0 as size_t {
            break;
        }
        total = (total as ::core::ffi::c_ulong).wrapping_add(n as ::core::ffi::c_ulong) as uint64_t;
        if total > before.st_size as uint64_t
            || fwrite(
                &raw mut buffer as *mut uint8_t as *const ::core::ffi::c_void,
                1 as size_t,
                n,
                zip,
            ) as size_t
                != n
        {
            failed = 1 as ::core::ffi::c_int;
            break;
        } else {
            let mut i: size_t = 0 as size_t;
            while i < n {
                checksum = checksum >> 8 as ::core::ffi::c_int
                    ^ s_crc_table[((checksum ^ buffer[i] as uint32_t) & 0xff as uint32_t) as usize];
                i = i.wrapping_add(1);
            }
        }
    }
    if ferror(src) != 0 {
        failed = 1 as ::core::ffi::c_int;
    }
    if failed == 0
        && (fstat(fileno(src), &raw mut after) != 0 as ::core::ffi::c_int
            || total != before.st_size as uint64_t
            || after.st_size != before.st_size
            || after.st_mtim.tv_sec != before.st_mtim.tv_sec
            || after.st_mtim.tv_nsec != before.st_mtim.tv_nsec
            || after.st_ctim.tv_sec != before.st_ctim.tv_sec
            || after.st_ctim.tv_nsec != before.st_ctim.tv_nsec)
    {
        failed = 1 as ::core::ffi::c_int;
    }
    if fclose(src) != 0 as ::core::ffi::c_int {
        failed = 1 as ::core::ffi::c_int;
    }
    if failed != 0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut end: off_t = ftello(zip);
    if end < 0 as off_t
        || fseeko(zip, offset as __off_t + 14 as __off_t, SEEK_SET) != 0 as ::core::ffi::c_int
        || zip_write_u32(zip, !checksum) != 0
        || zip_write_u32(zip, total as uint32_t) != 0
        || zip_write_u32(zip, total as uint32_t) != 0
        || fseeko(zip, end, SEEK_SET) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    *size = total as uint32_t;
    *crc = !checksum;
    return 0 as ::core::ffi::c_int;
}
pub const MAX_BATCH_DL_JOBS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MAX_DL_IDS: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
pub const JOB_DL_RETENTION: ::core::ffi::c_int = 600 as ::core::ffi::c_int;
static mut s_dl_jobs: [batch_dl_job_t; 8] = [batch_dl_job_t {
    job_id: [0; 64],
    owner_user_id: 0,
    owner_username: [0; 64],
    owner_api_token_uuid: [0; 37],
    owner_auth_method: [0; 24],
    audit_request_id: [0; 65],
    audit_client_ip: [0; 64],
    zip_filename: [0; 192],
    tmp_path: [0; 512],
    ids: [0; 200],
    id_count: 0,
    status: dl_status_t::DL_PENDING,
    current: 0,
    total: 0,
    error: [0; 256],
    created_at: 0,
    updated_at: 0,
    is_active: false,
}; 8];
static mut s_dl_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: C2Rust_Unnamed_1::PTHREAD_MUTEX_TIMED_NP.0 as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut s_dl_inited: bool = r#false != 0;
unsafe extern "C" fn audit_batch_archive_job(
    mut job: *const batch_dl_job_t,
    mut outcome: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
    mut completed_count: ::core::ffi::c_int,
) {
    if job.is_null() {
        return;
    }
    let mut user: user_t = user_t {
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
    user.id = (*job).owner_user_id;
    safe_strcpy(
        &raw mut user.username as *mut ::core::ffi::c_char,
        &raw const (*job).owner_username as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut user.authentication_method as *mut ::core::ffi::c_char,
        &raw const (*job).owner_auth_method as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        0 as size_t,
    );
    if (*job).owner_api_token_uuid[0usize] != 0 {
        user.authenticated_via_scoped_token = r#true != 0;
        safe_strcpy(
            &raw mut user.api_token_uuid as *mut ::core::ffi::c_char,
            &raw const (*job).owner_api_token_uuid as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
    }
    let mut request: http_request_t = http_request_t {
        method: http_method_t(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
        method_str: [0; 16],
        path: [0; 512],
        uri: [0; 1536],
        query_string: [0; 1024],
        content_type: [0; 128],
        content_length: 0,
        user_agent: [0; 256],
        body: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        body_len: 0,
        headers: [http_header_t {
            name: [0; 128],
            value: [0; 1024],
        }; 50],
        num_headers: 0,
        client_ip: [0; 64],
        request_id: [0; 65],
        user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    request.method = http_method_t::HTTP_METHOD_POST;
    safe_strcpy(
        &raw mut request.method_str as *mut ::core::ffi::c_char,
        b"POST\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut request.path as *mut ::core::ffi::c_char,
        b"/api/recordings/batch-download\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut request.uri as *mut ::core::ffi::c_char,
        &raw mut request.path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1536]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut request.client_ip as *mut ::core::ffi::c_char,
        &raw const (*job).audit_client_ip as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut request.request_id as *mut ::core::ffi::c_char,
        &raw const (*job).audit_request_id as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
        0 as size_t,
    );
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() {
        cJSON_AddStringToObject(
            details,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        );
        cJSON_AddNumberToObject(
            details,
            b"requested_count\0".as_ptr() as *const ::core::ffi::c_char,
            (*job).id_count as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            details,
            b"completed_count\0".as_ptr() as *const ::core::ffi::c_char,
            completed_count as ::core::ffi::c_double,
        );
    }
    audit_log_operation(
        &raw mut request,
        &raw mut user,
        b"recordings.export\0".as_ptr() as *const ::core::ffi::c_char,
        b"export_job\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*job).job_id as *const ::core::ffi::c_char,
        b"create_batch_archive\0".as_ptr() as *const ::core::ffi::c_char,
        outcome,
        details,
    );
    cJSON_Delete(details);
}
unsafe extern "C" fn audit_batch_archive_transfer(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut job_id: *const ::core::ffi::c_char,
    mut total: ::core::ffi::c_int,
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
        cJSON_AddNumberToObject(
            details,
            b"recording_count\0".as_ptr() as *const ::core::ffi::c_char,
            total as ::core::ffi::c_double,
        );
    }
    audit_log_operation(
        req,
        user,
        b"recordings.export\0".as_ptr() as *const ::core::ffi::c_char,
        b"export_job\0".as_ptr() as *const ::core::ffi::c_char,
        job_id,
        b"download_batch_archive\0".as_ptr() as *const ::core::ffi::c_char,
        outcome,
        details,
    );
    cJSON_Delete(details);
}
unsafe extern "C" fn dl_jobs_init() {
    if s_dl_inited {
        return;
    }
    memset(
        &raw mut s_dl_jobs as *mut batch_dl_job_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[batch_dl_job_t; 8]>(),
    );
    s_dl_inited = r#true != 0;
}
unsafe extern "C" fn gen_uuid(mut out: *mut ::core::ffi::c_char) -> bool {
    let mut b: [uint8_t; 16] = [0; 16];
    if getrandom(
        &raw mut b as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>(),
        0 as ::core::ffi::c_uint,
    ) != ::core::mem::size_of::<[uint8_t; 16]>() as ssize_t
    {
        let mut f: *mut FILE = fopen(
            b"/dev/urandom\0".as_ptr() as *const ::core::ffi::c_char,
            b"rb\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if f.is_null() {
            return r#false != 0;
        }
        let mut bytes_read: size_t = fread(
            &raw mut b as *mut uint8_t as *mut ::core::ffi::c_void,
            1 as size_t,
            ::core::mem::size_of::<[uint8_t; 16]>(),
            f,
        ) as size_t;
        fclose(f);
        if bytes_read != ::core::mem::size_of::<[uint8_t; 16]>() {
            return r#false != 0;
        }
    }
    b[6usize] = (b[6usize] as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
        | 0x40 as ::core::ffi::c_int) as uint8_t;
    b[8usize] = (b[8usize] as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int
        | 0x80 as ::core::ffi::c_int) as uint8_t;
    snprintf(
        out,
        37 as size_t,
        b"%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x\0".as_ptr()
            as *const ::core::ffi::c_char,
        b[0usize] as ::core::ffi::c_int,
        b[1usize] as ::core::ffi::c_int,
        b[2usize] as ::core::ffi::c_int,
        b[3usize] as ::core::ffi::c_int,
        b[4usize] as ::core::ffi::c_int,
        b[5usize] as ::core::ffi::c_int,
        b[6usize] as ::core::ffi::c_int,
        b[7usize] as ::core::ffi::c_int,
        b[8usize] as ::core::ffi::c_int,
        b[9usize] as ::core::ffi::c_int,
        b[10usize] as ::core::ffi::c_int,
        b[11usize] as ::core::ffi::c_int,
        b[12usize] as ::core::ffi::c_int,
        b[13usize] as ::core::ffi::c_int,
        b[14usize] as ::core::ffi::c_int,
        b[15usize] as ::core::ffi::c_int,
    );
    return r#true != 0;
}
unsafe extern "C" fn find_dl_slot() -> ::core::ffi::c_int {
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_BATCH_DL_JOBS {
        if !s_dl_jobs[i as usize].is_active {
            return i;
        }
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < MAX_BATCH_DL_JOBS {
        if (s_dl_jobs[i_0 as usize].status.0 == dl_status_t::DL_COMPLETE.0
            || s_dl_jobs[i_0 as usize].status.0 == dl_status_t::DL_ERROR.0)
            && now - s_dl_jobs[i_0 as usize].updated_at > JOB_DL_RETENTION as time_t
        {
            if s_dl_jobs[i_0 as usize].tmp_path[0usize] != 0 {
                unlink(
                    &raw mut (*(&raw mut s_dl_jobs as *mut batch_dl_job_t).offset(i_0 as isize))
                        .tmp_path as *mut ::core::ffi::c_char,
                );
                s_dl_jobs[i_0 as usize].tmp_path[0usize] = '\0' as ::core::ffi::c_char;
            }
            return i_0;
        }
        i_0 += 1;
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn find_dl_job(mut jid: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_BATCH_DL_JOBS {
        if s_dl_jobs[i as usize].is_active as ::core::ffi::c_int != 0
            && strcmp(
                &raw mut (*(&raw mut s_dl_jobs as *mut batch_dl_job_t).offset(i as isize)).job_id
                    as *mut ::core::ffi::c_char,
                jid,
            ) == 0 as ::core::ffi::c_int
        {
            return i;
        }
        i += 1;
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn dl_job_owned_by(
    mut job: *const batch_dl_job_t,
    mut user: *const user_t,
) -> bool {
    if job.is_null()
        || user.is_null()
        || (*job).owner_user_id != (*user).id
        || strcmp(
            &raw const (*job).owner_username as *const ::core::ffi::c_char,
            &raw const (*user).username as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    if (*job).owner_api_token_uuid[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return !(*user).authenticated_via_scoped_token;
    }
    return (*user).authenticated_via_scoped_token as ::core::ffi::c_int != 0
        && strcmp(
            &raw const (*job).owner_api_token_uuid as *const ::core::ffi::c_char,
            &raw const (*user).api_token_uuid as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn discard_dl_job(mut job_id: *const ::core::ffi::c_char) {
    pthread_mutex_lock(&raw mut s_dl_mutex);
    let mut slot: ::core::ffi::c_int = find_dl_job(job_id);
    if slot >= 0 as ::core::ffi::c_int {
        if s_dl_jobs[slot as usize].tmp_path[0usize] as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
        {
            unlink(
                &raw mut (*(&raw mut s_dl_jobs as *mut batch_dl_job_t).offset(slot as isize))
                    .tmp_path as *mut ::core::ffi::c_char,
            );
        }
        memset(
            (&raw mut s_dl_jobs as *mut batch_dl_job_t).offset(slot as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<batch_dl_job_t>(),
        );
    }
    pthread_mutex_unlock(&raw mut s_dl_mutex);
}
unsafe extern "C" fn to_dos_time(mut t: *const tm) -> uint16_t {
    return (((*t).tm_hour & 0x1f as ::core::ffi::c_int) << 11 as ::core::ffi::c_int
        | ((*t).tm_min & 0x3f as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
        | (*t).tm_sec / 2 as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
        as uint16_t;
}
unsafe extern "C" fn to_dos_date(mut t: *const tm) -> uint16_t {
    return (((*t).tm_year - 80 as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
        << 9 as ::core::ffi::c_int
        | ((*t).tm_mon + 1 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
            << 5 as ::core::ffi::c_int
        | (*t).tm_mday & 0x1f as ::core::ffi::c_int) as uint16_t;
}
unsafe extern "C" fn zip_worker(mut arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let mut source_error: [::core::ffi::c_char; 256] = [0; 256];
    let mut source: ::core::ffi::c_int = 0;
    let mut deadline: time_t = 0;
    let mut reserve: uint32_t = 0;
    let mut base: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut tm_info: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    log_set_thread_context(
        b"BatchDownload\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let mut a: *mut dl_thread_arg_t = arg as *mut dl_thread_arg_t;
    let mut job_id: [::core::ffi::c_char; 64] = [0; 64];
    safe_strcpy(
        &raw mut job_id as *mut ::core::ffi::c_char,
        &raw mut (*a).job_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    free(a as *mut ::core::ffi::c_void);
    pthread_mutex_lock(&raw mut s_dl_mutex);
    let mut slot: ::core::ffi::c_int = find_dl_job(&raw mut job_id as *mut ::core::ffi::c_char);
    if slot < 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut s_dl_mutex);
        return NULL;
    }
    let mut job: batch_dl_job_t = s_dl_jobs[slot as usize];
    s_dl_jobs[slot as usize].status = dl_status_t::DL_RUNNING;
    pthread_mutex_unlock(&raw mut s_dl_mutex);
    let mut tmp_template: [::core::ffi::c_char; 25] = ::core::mem::transmute::<
        [u8; 25],
        [::core::ffi::c_char; 25],
    >(*b"/tmp/lightnvr_zip_XXXXXX\0");
    let mut fd: ::core::ffi::c_int = mkstemp(&raw mut tmp_template as *mut ::core::ffi::c_char);
    if fd < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"zip_worker: mkstemp failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        pthread_mutex_lock(&raw mut s_dl_mutex);
        slot = find_dl_job(&raw mut job_id as *mut ::core::ffi::c_char);
        if slot >= 0 as ::core::ffi::c_int {
            s_dl_jobs[slot as usize].status = dl_status_t::DL_ERROR;
            safe_strcpy(
                &raw mut (*(&raw mut s_dl_jobs as *mut batch_dl_job_t).offset(slot as isize)).error
                    as *mut ::core::ffi::c_char,
                b"Failed to create temp file\0".as_ptr() as *const ::core::ffi::c_char,
                256 as size_t,
                0 as size_t,
            );
            s_dl_jobs[slot as usize].updated_at = time(::core::ptr::null_mut::<time_t>());
        }
        pthread_mutex_unlock(&raw mut s_dl_mutex);
        audit_batch_archive_job(
            &raw mut job,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"temp_file_create_failed\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        return NULL;
    }
    let mut zip: *mut FILE = fdopen(fd, b"wb\0".as_ptr() as *const ::core::ffi::c_char);
    if zip.is_null() {
        close(fd);
        unlink(&raw mut tmp_template as *mut ::core::ffi::c_char);
        pthread_mutex_lock(&raw mut s_dl_mutex);
        slot = find_dl_job(&raw mut job_id as *mut ::core::ffi::c_char);
        if slot >= 0 as ::core::ffi::c_int {
            s_dl_jobs[slot as usize].status = dl_status_t::DL_ERROR;
            safe_strcpy(
                &raw mut (*(&raw mut s_dl_jobs as *mut batch_dl_job_t).offset(slot as isize)).error
                    as *mut ::core::ffi::c_char,
                b"Failed to open temp archive\0".as_ptr() as *const ::core::ffi::c_char,
                256 as size_t,
                0 as size_t,
            );
            s_dl_jobs[slot as usize].updated_at = time(::core::ptr::null_mut::<time_t>());
        }
        pthread_mutex_unlock(&raw mut s_dl_mutex);
        audit_batch_archive_job(
            &raw mut job,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"temp_archive_open_failed\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        return NULL;
    }
    let mut entries: *mut entry_t =
        calloc(job.id_count as size_t, ::core::mem::size_of::<entry_t>()) as *mut entry_t;
    if entries.is_null() {
        fclose(zip);
        unlink(&raw mut tmp_template as *mut ::core::ffi::c_char);
        pthread_mutex_lock(&raw mut s_dl_mutex);
        slot = find_dl_job(&raw mut job_id as *mut ::core::ffi::c_char);
        if slot >= 0 as ::core::ffi::c_int {
            s_dl_jobs[slot as usize].status = dl_status_t::DL_ERROR;
            safe_strcpy(
                &raw mut (*(&raw mut s_dl_jobs as *mut batch_dl_job_t).offset(slot as isize)).error
                    as *mut ::core::ffi::c_char,
                b"Failed to allocate archive index\0".as_ptr() as *const ::core::ffi::c_char,
                256 as size_t,
                0 as size_t,
            );
            s_dl_jobs[slot as usize].updated_at = time(::core::ptr::null_mut::<time_t>());
        }
        pthread_mutex_unlock(&raw mut s_dl_mutex);
        audit_batch_archive_job(
            &raw mut job,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"allocation_failed\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        return NULL;
    }
    let mut entry_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut data_offset: uint32_t = 0 as uint32_t;
    let mut io_failed: bool = r#false != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < job.id_count {
        let mut rec: recording_metadata_t = recording_metadata_t {
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
        if get_recording_metadata_by_id(job.ids[i as usize], &raw mut rec)
            != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"zip_worker: recording %llu not found, skipping\0".as_ptr()
                    as *const ::core::ffi::c_char,
                job.ids[i as usize] as ::core::ffi::c_ulonglong,
            );
        } else {
            source_error = [
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
            source = C2Rust_Unnamed::STORAGE_SOURCE_PREPARING.0;
            deadline = time(::core::ptr::null_mut::<time_t>()) + 3600 as time_t;
            while source == C2Rust_Unnamed::STORAGE_SOURCE_PREPARING.0
                && time(::core::ptr::null_mut::<time_t>()) < deadline
            {
                source = storage_source_resolve(
                    rec.id,
                    &raw mut rec.file_path as *mut ::core::ffi::c_char,
                    &raw mut source_error as *mut ::core::ffi::c_char,
                );
                if source != C2Rust_Unnamed::STORAGE_SOURCE_PREPARING.0 {
                    continue;
                }
                pthread_mutex_lock(&raw mut s_dl_mutex);
                slot = find_dl_job(&raw mut job_id as *mut ::core::ffi::c_char);
                if slot >= 0 as ::core::ffi::c_int {
                    s_dl_jobs[slot as usize].updated_at = time(::core::ptr::null_mut::<time_t>());
                }
                pthread_mutex_unlock(&raw mut s_dl_mutex);
                if slot < 0 as ::core::ffi::c_int {
                    break;
                }
                let mut delay: timespec = timespec {
                    tv_sec: 1 as __time_t,
                    tv_nsec: 0,
                };
                nanosleep(&raw mut delay, ::core::ptr::null_mut::<timespec>());
            }
            if source == C2Rust_Unnamed::STORAGE_SOURCE_READY.0 {
                reserve = (MAX_DL_IDS * (46 as ::core::ffi::c_int + 127 as ::core::ffi::c_int)
                    + 30 as ::core::ffi::c_int
                    + 127 as ::core::ffi::c_int
                    + 22 as ::core::ffi::c_int) as uint32_t;
                if data_offset <= (UINT32_MAX as uint32_t).wrapping_sub(reserve) {
                    base = strrchr(
                        &raw mut rec.file_path as *mut ::core::ffi::c_char,
                        '/' as ::core::ffi::c_int,
                    );
                    base = if !base.is_null() {
                        base.offset(1 as ::core::ffi::c_int as isize)
                    } else {
                        &raw mut rec.file_path as *mut ::core::ffi::c_char
                    };
                    if strlen(base) < ::core::mem::size_of::<[::core::ffi::c_char; 128]>() {
                        safe_strcpy(
                            &raw mut (*entries.offset(entry_count as isize)).name
                                as *mut ::core::ffi::c_char,
                            base,
                            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                            0 as size_t,
                        );
                        (*entries.offset(entry_count as isize)).offset = data_offset;
                        tm_info = tm {
                            tm_sec: 0,
                            tm_min: 0,
                            tm_hour: 0,
                            tm_mday: 0,
                            tm_mon: 0,
                            tm_year: 0,
                            tm_wday: 0,
                            tm_yday: 0,
                            tm_isdst: 0,
                            tm_gmtoff: 0,
                            tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
                        };
                        if !localtime_r(&raw mut rec.start_time, &raw mut tm_info).is_null() {
                            (*entries.offset(entry_count as isize)).dos_date =
                                to_dos_date(&raw mut tm_info);
                            (*entries.offset(entry_count as isize)).dos_time =
                                to_dos_time(&raw mut tm_info);
                            if zip_write_local_header(
                                zip,
                                &raw mut (*entries.offset(entry_count as isize)).name
                                    as *mut ::core::ffi::c_char,
                                (*entries.offset(entry_count as isize)).dos_date,
                                (*entries.offset(entry_count as isize)).dos_time,
                            ) != 0
                                || zip_copy_file(
                                    zip,
                                    &raw mut rec.file_path as *mut ::core::ffi::c_char,
                                    data_offset,
                                    (UINT32_MAX as uint32_t)
                                        .wrapping_sub(data_offset)
                                        .wrapping_sub(reserve),
                                    &raw mut (*entries.offset(entry_count as isize)).size,
                                    &raw mut (*entries.offset(entry_count as isize)).crc,
                                ) != 0 as ::core::ffi::c_int
                            {
                                io_failed = r#true != 0;
                                break;
                            } else {
                                data_offset = data_offset.wrapping_add(
                                    (30 as uint32_t)
                                        .wrapping_add(strlen(
                                            &raw mut (*entries.offset(entry_count as isize)).name
                                                as *mut ::core::ffi::c_char,
                                        )
                                            as uint32_t)
                                        .wrapping_add((*entries.offset(entry_count as isize)).size),
                                );
                                entry_count += 1;
                            }
                        }
                    }
                }
            }
        }
        pthread_mutex_lock(&raw mut s_dl_mutex);
        slot = find_dl_job(&raw mut job_id as *mut ::core::ffi::c_char);
        if slot >= 0 as ::core::ffi::c_int {
            s_dl_jobs[slot as usize].current = i + 1 as ::core::ffi::c_int;
            s_dl_jobs[slot as usize].updated_at = time(::core::ptr::null_mut::<time_t>());
        }
        pthread_mutex_unlock(&raw mut s_dl_mutex);
        i += 1;
    }
    let mut cd_start: uint32_t = data_offset;
    let mut cd_size: uint32_t = 0 as uint32_t;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !io_failed && i_0 < entry_count {
        let mut namelen: uint16_t =
            strlen(&raw mut (*entries.offset(i_0 as isize)).name as *mut ::core::ffi::c_char)
                as uint16_t;
        if zip_write_central_entry(
            zip,
            &raw mut (*entries.offset(i_0 as isize)).name as *mut ::core::ffi::c_char,
            (*entries.offset(i_0 as isize)).crc,
            (*entries.offset(i_0 as isize)).size,
            (*entries.offset(i_0 as isize)).dos_date,
            (*entries.offset(i_0 as isize)).dos_time,
            (*entries.offset(i_0 as isize)).offset,
        ) != 0
        {
            io_failed = r#true != 0;
        }
        cd_size = cd_size
            .wrapping_add((46 as ::core::ffi::c_int + namelen as ::core::ffi::c_int) as uint32_t);
        i_0 += 1;
    }
    if !io_failed && zip_write_eocd(zip, entry_count as uint16_t, cd_size, cd_start) != 0 {
        io_failed = r#true != 0;
    }
    if fclose(zip) != 0 as ::core::ffi::c_int {
        io_failed = r#true != 0;
    }
    free(entries as *mut ::core::ffi::c_void);
    let mut complete: bool = !io_failed && entry_count == job.id_count;
    pthread_mutex_lock(&raw mut s_dl_mutex);
    slot = find_dl_job(&raw mut job_id as *mut ::core::ffi::c_char);
    if slot >= 0 as ::core::ffi::c_int {
        safe_strcpy(
            &raw mut (*(&raw mut s_dl_jobs as *mut batch_dl_job_t).offset(slot as isize)).tmp_path
                as *mut ::core::ffi::c_char,
            &raw mut tmp_template as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
        s_dl_jobs[slot as usize].status = dl_status_t(
            (if complete as ::core::ffi::c_int != 0 {
                dl_status_t::DL_COMPLETE.0 as ::core::ffi::c_int
            } else {
                dl_status_t::DL_ERROR.0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
        );
        if !complete {
            safe_strcpy(
                &raw mut (*(&raw mut s_dl_jobs as *mut batch_dl_job_t)
                    .offset(slot as isize))
                    .error as *mut ::core::ffi::c_char,
                b"Export incomplete: source unavailable, changed, oversized, or archive I/O failed\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                256 as size_t,
                0 as size_t,
            );
            unlink(&raw mut tmp_template as *mut ::core::ffi::c_char);
            s_dl_jobs[slot as usize].tmp_path[0usize] = 0 as ::core::ffi::c_char;
        }
        s_dl_jobs[slot as usize].current = job.id_count;
        s_dl_jobs[slot as usize].updated_at = time(::core::ptr::null_mut::<time_t>());
    }
    if slot < 0 as ::core::ffi::c_int {
        unlink(&raw mut tmp_template as *mut ::core::ffi::c_char);
    }
    pthread_mutex_unlock(&raw mut s_dl_mutex);
    audit_batch_archive_job(
        &raw mut job,
        if complete as ::core::ffi::c_int != 0 {
            b"success\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"failure\0".as_ptr() as *const ::core::ffi::c_char
        },
        if complete as ::core::ffi::c_int != 0 {
            b"completed\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"archive_failed\0".as_ptr() as *const ::core::ffi::c_char
        },
        entry_count,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"zip_worker: %s job %s -> %s (%d entries)\0".as_ptr() as *const ::core::ffi::c_char,
        if complete as ::core::ffi::c_int != 0 {
            b"completed\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"failed\0".as_ptr() as *const ::core::ffi::c_char
        },
        &raw mut job_id as *mut ::core::ffi::c_char,
        &raw mut tmp_template as *mut ::core::ffi::c_char,
        entry_count,
    );
    return NULL;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_batch_download_recordings(
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
            b"Invalid JSON body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut ids_arr: *mut cJSON =
        cJSON_GetObjectItem(json, b"ids\0".as_ptr() as *const ::core::ffi::c_char);
    if ids_arr.is_null() || cJSON_IsArray(ids_arr) == 0 {
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing or invalid 'ids' array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int = cJSON_GetArraySize(ids_arr);
    if count == 0 as ::core::ffi::c_int || count > MAX_DL_IDS {
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"ids must contain 1-200 entries\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut filename_raw: *const ::core::ffi::c_char =
        b"recordings.zip\0".as_ptr() as *const ::core::ffi::c_char;
    let mut r#fn: *mut cJSON =
        cJSON_GetObjectItem(json, b"filename\0".as_ptr() as *const ::core::ffi::c_char);
    if !r#fn.is_null()
        && cJSON_IsString(r#fn) != 0
        && *(*r#fn).valuestring.offset(0isize) as ::core::ffi::c_int != 0
    {
        filename_raw = (*r#fn).valuestring;
    }
    let mut parsed_ids: [uint64_t; 200] = [0; 200];
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut item: *mut cJSON = cJSON_GetArrayItem(ids_arr, i);
        if !recording_id_parse_json(
            item,
            (&raw mut parsed_ids as *mut uint64_t).offset(i as isize),
        ) {
            cJSON_Delete(json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid recording ID\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
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
        if get_recording_metadata_by_id(parsed_ids[i as usize], &raw mut recording)
            != 0 as ::core::ffi::c_int
        {
            cJSON_Delete(json);
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Recording not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
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
        let mut result: ::core::ffi::c_int = httpd_evaluate_camera_identity_action(
            &raw mut user,
            authorization_action_t::AUTHZ_RECORDINGS_EXPORT,
            &raw mut recording.camera_uuid as *mut ::core::ffi::c_char,
            &raw mut recording.stream_name as *mut ::core::ffi::c_char,
            &raw mut evaluation,
        );
        if result < 0 as ::core::ffi::c_int {
            cJSON_Delete(json);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        if result > 0 as ::core::ffi::c_int
            || evaluation.decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0
        {
            cJSON_Delete(json);
            http_response_set_json_error(
                res,
                403 as ::core::ffi::c_int,
                b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        if !recording.is_complete {
            cJSON_Delete(json);
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Recording is still in progress\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        i += 1;
    }
    pthread_mutex_lock(&raw mut s_dl_mutex);
    dl_jobs_init();
    let mut slot: ::core::ffi::c_int = find_dl_slot();
    if slot < 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut s_dl_mutex);
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            503 as ::core::ffi::c_int,
            b"Too many concurrent download jobs\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut job: *mut batch_dl_job_t =
        (&raw mut s_dl_jobs as *mut batch_dl_job_t).offset(slot as isize);
    memset(
        job as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<batch_dl_job_t>(),
    );
    if !gen_uuid(&raw mut (*job).job_id as *mut ::core::ffi::c_char) {
        pthread_mutex_unlock(&raw mut s_dl_mutex);
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create download token\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    (*job).owner_user_id = user.id;
    safe_strcpy(
        &raw mut (*job).owner_username as *mut ::core::ffi::c_char,
        &raw mut user.username as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*job).owner_auth_method as *mut ::core::ffi::c_char,
        &raw mut user.authentication_method as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        0 as size_t,
    );
    if user.authenticated_via_scoped_token {
        safe_strcpy(
            &raw mut (*job).owner_api_token_uuid as *mut ::core::ffi::c_char,
            &raw mut user.api_token_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
    }
    safe_strcpy(
        &raw mut (*job).audit_request_id as *mut ::core::ffi::c_char,
        &raw const (*req).request_id as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
        0 as size_t,
    );
    if httpd_get_effective_client_ip(
        req,
        &raw mut (*job).audit_client_ip as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut (*job).audit_client_ip as *mut ::core::ffi::c_char,
            &raw const (*req).client_ip as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    httpd_sanitize_attachment_filename(
        filename_raw,
        &raw mut (*job).zip_filename as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 192]>(),
    );
    if (*job).zip_filename[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        safe_strcpy(
            &raw mut (*job).zip_filename as *mut ::core::ffi::c_char,
            b"recordings.zip\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 192]>(),
            0 as size_t,
        );
    }
    (*job).id_count = count;
    (*job).total = count;
    (*job).status = dl_status_t::DL_PENDING;
    (*job).updated_at = time(::core::ptr::null_mut::<time_t>());
    (*job).created_at = (*job).updated_at;
    (*job).is_active = r#true != 0;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < count {
        (*job).ids[i_0 as usize] = parsed_ids[i_0 as usize];
        i_0 += 1;
    }
    let mut job_id: [::core::ffi::c_char; 64] = [0; 64];
    safe_strcpy(
        &raw mut job_id as *mut ::core::ffi::c_char,
        &raw mut (*job).job_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    pthread_mutex_unlock(&raw mut s_dl_mutex);
    cJSON_Delete(json);
    let mut targ: *mut dl_thread_arg_t =
        malloc(::core::mem::size_of::<dl_thread_arg_t>()) as *mut dl_thread_arg_t;
    if targ.is_null() {
        audit_batch_archive_job(
            job,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"allocation_failed\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        discard_dl_job(&raw mut job_id as *mut ::core::ffi::c_char);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_strcpy(
        &raw mut (*targ).job_id as *mut ::core::ffi::c_char,
        &raw mut job_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    let mut tid: pthread_t = 0;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    pthread_attr_init(&raw mut attr);
    pthread_attr_setdetachstate(
        &raw mut attr,
        C2Rust_Unnamed_0::PTHREAD_CREATE_DETACHED.0 as ::core::ffi::c_int,
    );
    if pthread_create(
        &raw mut tid,
        &raw mut attr,
        Some(
            zip_worker
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        targ as *mut ::core::ffi::c_void,
    ) != 0 as ::core::ffi::c_int
    {
        pthread_attr_destroy(&raw mut attr);
        free(targ as *mut ::core::ffi::c_void);
        audit_batch_archive_job(
            job,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"worker_start_failed\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        discard_dl_job(&raw mut job_id as *mut ::core::ffi::c_char);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to start ZIP worker\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    pthread_attr_destroy(&raw mut attr);
    let mut body: [::core::ffi::c_char; 256] = [0; 256];
    snprintf(
        &raw mut body as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        b"{\"token\":\"%s\",\"total\":%d}\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut job_id as *mut ::core::ffi::c_char,
        count,
    );
    http_response_set_json(
        res,
        202 as ::core::ffi::c_int,
        &raw mut body as *mut ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Batch download job started: %s (%d recordings)\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut job_id as *mut ::core::ffi::c_char,
        count,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_batch_download_status(
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
    if httpd_check_action_access(req, &raw mut user) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut token: [::core::ffi::c_char; 64] = [
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
    if http_request_extract_path_param(
        req,
        b"/api/recordings/batch-download/status/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing token\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    pthread_mutex_lock(&raw mut s_dl_mutex);
    let mut slot: ::core::ffi::c_int = find_dl_job(&raw mut token as *mut ::core::ffi::c_char);
    if slot < 0 as ::core::ffi::c_int
        || !dl_job_owned_by(
            (&raw mut s_dl_jobs as *mut batch_dl_job_t).offset(slot as isize),
            &raw mut user,
        )
    {
        pthread_mutex_unlock(&raw mut s_dl_mutex);
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Job not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut snap: batch_dl_job_t = s_dl_jobs[slot as usize];
    pthread_mutex_unlock(&raw mut s_dl_mutex);
    let mut status_str: *const ::core::ffi::c_char =
        b"pending\0".as_ptr() as *const ::core::ffi::c_char;
    if snap.status.0 == dl_status_t::DL_RUNNING.0 {
        status_str = b"running\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if snap.status.0 == dl_status_t::DL_COMPLETE.0 {
        status_str = b"complete\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if snap.status.0 == dl_status_t::DL_ERROR.0 {
        status_str = b"error\0".as_ptr() as *const ::core::ffi::c_char;
    }
    let mut body: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut body as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"{\"status\":\"%s\",\"current\":%d,\"total\":%d,\"error\":\"%s\"}\0".as_ptr()
            as *const ::core::ffi::c_char,
        status_str,
        snap.current,
        snap.total,
        &raw mut snap.error as *mut ::core::ffi::c_char,
    );
    http_response_set_json(
        res,
        200 as ::core::ffi::c_int,
        &raw mut body as *mut ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_batch_download_result(
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
    if httpd_check_action_access(req, &raw mut user) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut token: [::core::ffi::c_char; 64] = [
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
    if http_request_extract_path_param(
        req,
        b"/api/recordings/batch-download/result/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing token\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    pthread_mutex_lock(&raw mut s_dl_mutex);
    let mut slot: ::core::ffi::c_int = find_dl_job(&raw mut token as *mut ::core::ffi::c_char);
    if slot < 0 as ::core::ffi::c_int
        || !dl_job_owned_by(
            (&raw mut s_dl_jobs as *mut batch_dl_job_t).offset(slot as isize),
            &raw mut user,
        )
    {
        pthread_mutex_unlock(&raw mut s_dl_mutex);
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Job not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if s_dl_jobs[slot as usize].status.0 != dl_status_t::DL_COMPLETE.0 {
        let mut st: *const ::core::ffi::c_char =
            if s_dl_jobs[slot as usize].status.0 == dl_status_t::DL_ERROR.0 {
                b"error\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"not_ready\0".as_ptr() as *const ::core::ffi::c_char
            };
        let mut err: [::core::ffi::c_char; 64] = [0; 64];
        snprintf(
            &raw mut err as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            b"Job is %s\0".as_ptr() as *const ::core::ffi::c_char,
            st,
        );
        let mut job_total: ::core::ffi::c_int = s_dl_jobs[slot as usize].total;
        pthread_mutex_unlock(&raw mut s_dl_mutex);
        audit_batch_archive_transfer(
            req,
            &raw mut user,
            &raw mut token as *mut ::core::ffi::c_char,
            job_total,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"archive_not_ready\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            409 as ::core::ffi::c_int,
            &raw mut err as *mut ::core::ffi::c_char,
        );
        return;
    }
    let mut tmp_path: [::core::ffi::c_char; 512] = [0; 512];
    let mut zip_filename: [::core::ffi::c_char; 192] = [0; 192];
    let mut job_total_0: ::core::ffi::c_int = s_dl_jobs[slot as usize].total;
    safe_strcpy(
        &raw mut tmp_path as *mut ::core::ffi::c_char,
        &raw mut (*(&raw mut s_dl_jobs as *mut batch_dl_job_t).offset(slot as isize)).tmp_path
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut zip_filename as *mut ::core::ffi::c_char,
        &raw mut (*(&raw mut s_dl_jobs as *mut batch_dl_job_t).offset(slot as isize)).zip_filename
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 192]>(),
        0 as size_t,
    );
    s_dl_jobs[slot as usize].updated_at = time(::core::ptr::null_mut::<time_t>());
    pthread_mutex_unlock(&raw mut s_dl_mutex);
    let mut disp: [::core::ffi::c_char; 512] = [0; 512];
    snprintf(
        &raw mut disp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        b"Content-Disposition: attachment; filename=\"%s\"\r\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut zip_filename as *mut ::core::ffi::c_char,
    );
    if http_serve_file(
        req,
        res,
        &raw mut tmp_path as *mut ::core::ffi::c_char,
        b"application/zip\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut disp as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        audit_batch_archive_transfer(
            req,
            &raw mut user,
            &raw mut token as *mut ::core::ffi::c_char,
            job_total_0,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"response_stream_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"handle_batch_download_result: failed to serve %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut tmp_path as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serve ZIP file\0".as_ptr() as *const ::core::ffi::c_char,
        );
        unlink(&raw mut tmp_path as *mut ::core::ffi::c_char);
        pthread_mutex_lock(&raw mut s_dl_mutex);
        slot = find_dl_job(&raw mut token as *mut ::core::ffi::c_char);
        if slot >= 0 as ::core::ffi::c_int {
            s_dl_jobs[slot as usize].tmp_path[0usize] = '\0' as ::core::ffi::c_char;
            s_dl_jobs[slot as usize].is_active = r#false != 0;
        }
        pthread_mutex_unlock(&raw mut s_dl_mutex);
        return;
    }
    audit_batch_archive_transfer(
        req,
        &raw mut user,
        &raw mut token as *mut ::core::ffi::c_char,
        job_total_0,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"completed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Serving batch download ZIP: %s -> %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut token as *mut ::core::ffi::c_char,
        &raw mut zip_filename as *mut ::core::ffi::c_char,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
