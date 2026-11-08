unsafe extern "C" {
    pub type sqlite3_stmt;
    pub type sqlite3;
    pub type stream_handle_s;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
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
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
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
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_recording_tag_add(
        recording_id: uint64_t,
        tag: *const ::core::ffi::c_char,
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
    unsafe fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_authorize_stream_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn event_producer_publish_detection(
        camera_uuid: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
        result: *const detection_result_t,
        occurred_at: time_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_open_readonly_connection(connection: *mut *mut sqlite3) -> ::core::ffi::c_int;
    unsafe fn db_close_readonly_connection(connection: *mut sqlite3);
    unsafe fn store_external_motion_detections(
        stream_name: *const ::core::ffi::c_char,
        result: *const detection_result_t,
        timestamp: time_t,
        recording_id: uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn close_external_motion_detections(
        camera_uuid: *const ::core::ffi::c_char,
        stream_name: *const ::core::ffi::c_char,
        end_time: time_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_current_recording_id_for_stream(
        stream_name: *const ::core::ffi::c_char,
    ) -> uint64_t;
    unsafe fn get_stream_config(
        handle: stream_handle_t,
        config: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_by_name(name: *const ::core::ffi::c_char) -> stream_handle_t;
    unsafe fn is_detection_recording_scheduled(config: *const stream_config_t) -> bool;
    unsafe fn process_motion_event(
        stream_name: *const ::core::ffi::c_char,
        motion_detected: bool,
        timestamp: time_t,
        is_propagated: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn unified_detection_notify_motion(
        stream_name: *const ::core::ffi::c_char,
        motion_active: bool,
    );
    unsafe fn filter_detections_by_zones(
        stream_name: *const ::core::ffi::c_char,
        result: *mut detection_result_t,
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const PTHREAD_CREATE_JOINABLE: Self = Self(0);
    pub const PTHREAD_CREATE_DETACHED: Self = Self(1);
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
pub type uint8_t = u8;
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
pub struct stream_protocol_t(pub ::core::ffi::c_uint);
impl stream_protocol_t {
    pub const STREAM_PROTOCOL_TCP: Self = Self(0);
    pub const STREAM_PROTOCOL_UDP: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_config_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub location_uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 256],
    pub url: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub fps: ::core::ffi::c_int,
    pub codec: [::core::ffi::c_char; 16],
    pub priority: ::core::ffi::c_int,
    pub record: bool,
    pub segment_duration: ::core::ffi::c_int,
    pub detection_based_recording: bool,
    pub detection_model: [::core::ffi::c_char; 512],
    pub detection_interval: ::core::ffi::c_int,
    pub detection_threshold: ::core::ffi::c_float,
    pub pre_detection_buffer: ::core::ffi::c_int,
    pub post_detection_buffer: ::core::ffi::c_int,
    pub detection_api_url: [::core::ffi::c_char; 512],
    pub detection_object_filter: [::core::ffi::c_char; 16],
    pub detection_object_filter_list: [::core::ffi::c_char; 256],
    pub streaming_enabled: bool,
    pub protocol: stream_protocol_t,
    pub record_audio: bool,
    pub audio_voice_enhancement: bool,
    pub onvif_username: [::core::ffi::c_char; 64],
    pub onvif_password: [::core::ffi::c_char; 64],
    pub onvif_profile: [::core::ffi::c_char; 64],
    pub onvif_port: ::core::ffi::c_int,
    pub onvif_discovery_enabled: bool,
    pub is_onvif: bool,
    pub backchannel_enabled: bool,
    pub retention_days: ::core::ffi::c_int,
    pub detection_retention_days: ::core::ffi::c_int,
    pub max_storage_mb: ::core::ffi::c_int,
    pub tier_critical_multiplier: ::core::ffi::c_double,
    pub tier_important_multiplier: ::core::ffi::c_double,
    pub tier_ephemeral_multiplier: ::core::ffi::c_double,
    pub storage_priority: ::core::ffi::c_int,
    pub ptz_enabled: bool,
    pub ptz_max_x: ::core::ffi::c_int,
    pub ptz_max_y: ::core::ffi::c_int,
    pub ptz_max_z: ::core::ffi::c_int,
    pub ptz_has_home: bool,
    pub record_on_schedule: bool,
    pub recording_schedule: [uint8_t; 168],
    pub detection_record_on_schedule: bool,
    pub detection_recording_schedule: [uint8_t; 168],
    pub tags: [::core::ffi::c_char; 256],
    pub admin_url: [::core::ffi::c_char; 512],
    pub privacy_mode: bool,
    pub motion_trigger_source: [::core::ffi::c_char; 256],
    pub go2rtc_source_override: [::core::ffi::c_char; 8192],
    pub sub_stream_url: [::core::ffi::c_char; 512],
    pub detection_url: [::core::ffi::c_char; 512],
    pub publish_url: [::core::ffi::c_char; 512],
    pub playback_transport: [::core::ffi::c_char; 24],
    pub eptz_config: [::core::ffi::c_char; 1024],
    pub observation_evidence_enabled: bool,
    pub observation_labels: [::core::ffi::c_char; 512],
    pub observation_max_age_days: ::core::ffi::c_int,
    pub observation_max_bytes: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_t {
    pub label: [::core::ffi::c_char; 32],
    pub confidence: ::core::ffi::c_float,
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
    pub width: ::core::ffi::c_float,
    pub height: ::core::ffi::c_float,
    pub track_id: ::core::ffi::c_int,
    pub zone_id: [::core::ffi::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_result_t {
    pub count: ::core::ffi::c_int,
    pub detections: [detection_t; 20],
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
pub struct motion_action_t(pub ::core::ffi::c_uint);
impl motion_action_t {
    pub const MOTION_ACTION_START: Self = Self(0);
    pub const MOTION_ACTION_STOP: Self = Self(1);
    pub const MOTION_ACTION_PULSE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct motion_pulse_task_t {
    pub stream_name: [::core::ffi::c_char; 256],
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub duration_ms: ::core::ffi::c_int,
    pub generation: uint64_t,
    pub trigger_recording: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pulse_gen_entry_t {
    pub name: [::core::ffi::c_char; 256],
    pub generation: uint64_t,
}
pub type stream_handle_t = *mut stream_handle_s;
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
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const LOG_COMPONENT: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"MotionAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const MAX_TAG_LENGTH: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const MAX_DETECTIONS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const MAX_LABEL_LENGTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MOTION_MAX_TAGS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MOTION_PULSE_DEFAULT_MS: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
pub const MOTION_PULSE_MAX_MS: ::core::ffi::c_int = 600000 as ::core::ffi::c_int;
pub const MOTION_DEFAULT_CONFIDENCE: ::core::ffi::c_float = 1.0f32;
static mut g_pulse_gen: [pulse_gen_entry_t; 1024] = [pulse_gen_entry_t {
    name: [0; 256],
    generation: 0,
}; 1024];
static mut g_pulse_gen_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut g_pulse_gen_sequence: uint64_t = 0 as uint64_t;
static mut g_pulse_gen_mutex: pthread_mutex_t = pthread_mutex_t {
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
unsafe extern "C" fn pulse_gen_prune_stale_locked() {
    let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
    if db_open_readonly_connection(&raw mut db) != 0 as ::core::ffi::c_int {
        return;
    }
    let mut stmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut rc: ::core::ffi::c_int = sqlite3_prepare_v2(
        db,
        b"SELECT name FROM streams;\0".as_ptr() as *const ::core::ffi::c_char,
        -1 as ::core::ffi::c_int,
        &raw mut stmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    let mut present: [bool; 1024] = [
        r#false != 0,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    ];
    if rc == SQLITE_OK {
        loop {
            rc = sqlite3_step(stmt);
            if rc != SQLITE_ROW {
                break;
            }
            let mut name: *const ::core::ffi::c_char =
                sqlite3_column_text(stmt, 0 as ::core::ffi::c_int) as *const ::core::ffi::c_char;
            if name.is_null() {
                continue;
            }
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < g_pulse_gen_count {
                if strcmp(
                    &raw mut (*(&raw mut g_pulse_gen as *mut pulse_gen_entry_t).offset(i as isize))
                        .name as *mut ::core::ffi::c_char,
                    name,
                ) == 0 as ::core::ffi::c_int
                {
                    present[i as usize] = r#true != 0;
                    break;
                } else {
                    i += 1;
                }
            }
        }
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    if rc != SQLITE_DONE {
        return;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < g_pulse_gen_count {
        if present[i_0 as usize] {
            let c2rust_fresh1 = count;
            count += 1;
            g_pulse_gen[c2rust_fresh1 as usize] = g_pulse_gen[i_0 as usize];
        }
        i_0 += 1;
    }
    g_pulse_gen_count = count;
}
unsafe extern "C" fn pulse_gen_get(mut stream_name: *const ::core::ffi::c_char) -> uint64_t {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_pulse_gen_count {
        if strcmp(
            &raw mut (*(&raw mut g_pulse_gen as *mut pulse_gen_entry_t).offset(i as isize)).name
                as *mut ::core::ffi::c_char,
            stream_name,
        ) == 0 as ::core::ffi::c_int
        {
            return g_pulse_gen[i as usize].generation;
        }
        i += 1;
    }
    return 0 as uint64_t;
}
unsafe extern "C" fn pulse_gen_increment(mut stream_name: *const ::core::ffi::c_char) -> uint64_t {
    if g_pulse_gen_sequence == UINT64_MAX as uint64_t {
        return 0 as uint64_t;
    }
    if g_pulse_gen_count == MAX_STREAMS && pulse_gen_get(stream_name) == 0 as uint64_t {
        pulse_gen_prune_stale_locked();
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < g_pulse_gen_count {
        if strcmp(
            &raw mut (*(&raw mut g_pulse_gen as *mut pulse_gen_entry_t).offset(i as isize)).name
                as *mut ::core::ffi::c_char,
            stream_name,
        ) == 0 as ::core::ffi::c_int
        {
            g_pulse_gen_sequence = g_pulse_gen_sequence.wrapping_add(1);
            g_pulse_gen[i as usize].generation = g_pulse_gen_sequence;
            return g_pulse_gen[i as usize].generation;
        }
        i += 1;
    }
    if g_pulse_gen_count < MAX_STREAMS {
        let c2rust_fresh0 = g_pulse_gen_count;
        g_pulse_gen_count += 1;
        let mut i_0: ::core::ffi::c_int = c2rust_fresh0;
        safe_strcpy(
            &raw mut (*(&raw mut g_pulse_gen as *mut pulse_gen_entry_t).offset(i_0 as isize)).name
                as *mut ::core::ffi::c_char,
            stream_name,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        g_pulse_gen_sequence = g_pulse_gen_sequence.wrapping_add(1);
        g_pulse_gen[i_0 as usize].generation = g_pulse_gen_sequence;
        return g_pulse_gen[i_0 as usize].generation;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"pulse_gen table full; cannot track motion action for '%s'\0".as_ptr()
            as *const ::core::ffi::c_char,
        stream_name,
    );
    return 0 as uint64_t;
}
unsafe extern "C" fn motion_pulse_worker(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut task: *mut motion_pulse_task_t = arg as *mut motion_pulse_task_t;
    if task.is_null() {
        return NULL;
    }
    let mut remaining_ms: ::core::ffi::c_int = (*task).duration_ms;
    while remaining_ms > 0 as ::core::ffi::c_int {
        let mut chunk_ms: ::core::ffi::c_int = if remaining_ms > 500 as ::core::ffi::c_int {
            500 as ::core::ffi::c_int
        } else {
            remaining_ms
        };
        let mut req: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        req.tv_sec = (chunk_ms / 1000 as ::core::ffi::c_int) as __time_t;
        req.tv_nsec = ((chunk_ms % 1000 as ::core::ffi::c_int) as ::core::ffi::c_long
            * 1000000 as ::core::ffi::c_long) as __syscall_slong_t;
        let mut rc: ::core::ffi::c_int = 0;
        loop {
            rc = nanosleep(&raw mut req, &raw mut req);
            if !(rc == -1 as ::core::ffi::c_int && *__errno_location() == EINTR) {
                break;
            }
        }
        if rc == -1 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Pulse sleep failed for '%s': %s; ending pulse\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            break;
        } else {
            remaining_ms -= chunk_ms;
        }
    }
    let mut still_current: bool = false;
    pthread_mutex_lock(&raw mut g_pulse_gen_mutex);
    let mut current_gen: uint64_t =
        pulse_gen_get(&raw mut (*task).stream_name as *mut ::core::ffi::c_char);
    still_current = (*task).generation != 0 as uint64_t && current_gen == (*task).generation;
    if still_current {
        let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
        close_external_motion_detections(
            &raw mut (*task).camera_uuid as *mut ::core::ffi::c_char,
            &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
            now,
        );
        if (*task).trigger_recording {
            unified_detection_notify_motion(
                &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
                r#false != 0,
            );
        }
        process_motion_event(
            &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
            r#false != 0,
            now,
            r#false != 0,
        );
    }
    pthread_mutex_unlock(&raw mut g_pulse_gen_mutex);
    if still_current {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Pulse ended for stream '%s' after %d ms\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
            (*task).duration_ms,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Pulse for stream '%s' superseded by newer trigger; stop skipped\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
        );
    }
    free(task as *mut ::core::ffi::c_void);
    return NULL;
}
unsafe extern "C" fn parse_action(
    mut s: *const ::core::ffi::c_char,
    mut out: *mut motion_action_t,
) -> ::core::ffi::c_int {
    if s.is_null() || out.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if strcmp(s, b"start\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        *out = motion_action_t::MOTION_ACTION_START;
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(s, b"stop\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        *out = motion_action_t::MOTION_ACTION_STOP;
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(s, b"pulse\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        *out = motion_action_t::MOTION_ACTION_PULSE;
        return 0 as ::core::ffi::c_int;
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_confidence(
    mut j: *const cJSON,
    mut out: *mut ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    if j.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if cJSON_IsNumber(j) == 0 {
        return -1 as ::core::ffi::c_int;
    }
    let mut v: ::core::ffi::c_double = (*j).valuedouble;
    if v < 0.0f64 || v > 1.0f64 {
        return -1 as ::core::ffi::c_int;
    }
    *out = v as ::core::ffi::c_float;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn append_detection(
    mut result: *mut detection_result_t,
    mut label: *const ::core::ffi::c_char,
    mut confidence: ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    if result.is_null()
        || label.is_null()
        || *label.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if (*result).count >= MAX_DETECTIONS {
        return -1 as ::core::ffi::c_int;
    }
    let mut d: *mut detection_t =
        (&raw mut (*result).detections as *mut detection_t).offset((*result).count as isize);
    safe_strcpy(
        &raw mut (*d).label as *mut ::core::ffi::c_char,
        label,
        MAX_LABEL_LENGTH as size_t,
        0 as size_t,
    );
    (*d).confidence = confidence;
    (*d).x = 0.0f32;
    (*d).y = 0.0f32;
    (*d).width = 1.0f32;
    (*d).height = 1.0f32;
    (*result).count += 1;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn motion_trigger_parse_objects(
    mut body: *const cJSON,
    mut result: *mut detection_result_t,
    mut err: *mut ::core::ffi::c_char,
    mut err_sz: size_t,
) -> ::core::ffi::c_int {
    let mut default_confidence: ::core::ffi::c_float = MOTION_DEFAULT_CONFIDENCE;
    let mut j_conf: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if parse_confidence(j_conf, &raw mut default_confidence) != 0 as ::core::ffi::c_int {
        safe_strcpy(
            err,
            b"Field 'confidence' must be a number between 0 and 1\0".as_ptr()
                as *const ::core::ffi::c_char,
            err_sz,
            0 as size_t,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut j_label: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"label\0".as_ptr() as *const ::core::ffi::c_char);
    if !j_label.is_null() {
        if cJSON_IsString(j_label) == 0
            || *(*j_label).valuestring.offset(0isize) as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int
        {
            safe_strcpy(
                err,
                b"Field 'label' must be a non-empty string\0".as_ptr()
                    as *const ::core::ffi::c_char,
                err_sz,
                0 as size_t,
            );
            return -1 as ::core::ffi::c_int;
        }
        append_detection(result, (*j_label).valuestring, default_confidence);
    }
    let mut j_objects: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"objects\0".as_ptr() as *const ::core::ffi::c_char);
    if j_objects.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if cJSON_IsArray(j_objects) == 0 {
        safe_strcpy(
            err,
            b"Field 'objects' must be an array\0".as_ptr() as *const ::core::ffi::c_char,
            err_sz,
            0 as size_t,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut entry: *const cJSON = ::core::ptr::null::<cJSON>();
    entry = if !j_objects.is_null() {
        (*j_objects).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !entry.is_null() {
        let mut label: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut confidence: ::core::ffi::c_float = default_confidence;
        if cJSON_IsString(entry) != 0 {
            label = (*entry).valuestring;
        } else if cJSON_IsObject(entry) != 0 {
            let mut j_entry_label: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                entry,
                b"label\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if cJSON_IsString(j_entry_label) == 0 {
                safe_strcpy(
                    err,
                    b"Each 'objects' entry must be a string or an object with a 'label' string\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    err_sz,
                    0 as size_t,
                );
                return -1 as ::core::ffi::c_int;
            }
            label = (*j_entry_label).valuestring;
            if parse_confidence(
                cJSON_GetObjectItemCaseSensitive(
                    entry,
                    b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
                ),
                &raw mut confidence,
            ) != 0 as ::core::ffi::c_int
            {
                safe_strcpy(
                    err,
                    b"Each 'objects' entry 'confidence' must be a number between 0 and 1\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    err_sz,
                    0 as size_t,
                );
                return -1 as ::core::ffi::c_int;
            }
        } else {
            safe_strcpy(
                err,
                b"Each 'objects' entry must be a string or an object with a 'label' string\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                err_sz,
                0 as size_t,
            );
            return -1 as ::core::ffi::c_int;
        }
        if label.is_null()
            || *label.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        {
            safe_strcpy(
                err,
                b"Each 'objects' entry must have a non-empty label\0".as_ptr()
                    as *const ::core::ffi::c_char,
                err_sz,
                0 as size_t,
            );
            return -1 as ::core::ffi::c_int;
        }
        append_detection(result, label, confidence);
        entry = (*entry).next;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn motion_trigger_parse_tags(
    mut body: *const cJSON,
    mut tags: *mut [::core::ffi::c_char; 128],
    mut err: *mut ::core::ffi::c_char,
    mut err_sz: size_t,
) -> ::core::ffi::c_int {
    let mut j_tags: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"tags\0".as_ptr() as *const ::core::ffi::c_char);
    if j_tags.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if cJSON_IsArray(j_tags) == 0 {
        safe_strcpy(
            err,
            b"Field 'tags' must be an array of strings\0".as_ptr() as *const ::core::ffi::c_char,
            err_sz,
            0 as size_t,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut entry: *const cJSON = ::core::ptr::null::<cJSON>();
    entry = if !j_tags.is_null() {
        (*j_tags).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !entry.is_null() {
        if cJSON_IsString(entry) == 0
            || *(*entry).valuestring.offset(0isize) as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int
        {
            safe_strcpy(
                err,
                b"Each 'tags' entry must be a non-empty string\0".as_ptr()
                    as *const ::core::ffi::c_char,
                err_sz,
                0 as size_t,
            );
            return -1 as ::core::ffi::c_int;
        }
        if count >= MOTION_MAX_TAGS {
            break;
        }
        safe_strcpy(
            &raw mut *tags.offset(count as isize) as *mut ::core::ffi::c_char,
            (*entry).valuestring,
            MAX_TAG_LENGTH as size_t,
            0 as size_t,
        );
        count += 1;
        entry = (*entry).next;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_motion_trigger(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"POST /api/motion/trigger\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    if body.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid or missing JSON body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut j_stream: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"stream\0".as_ptr() as *const ::core::ffi::c_char);
    let mut j_action: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"action\0".as_ptr() as *const ::core::ffi::c_char);
    let mut j_duration: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"duration_ms\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsString(j_stream) == 0
        || *(*j_stream).valuestring.offset(0isize) as ::core::ffi::c_int
            == '\0' as ::core::ffi::c_int
    {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Field 'stream' is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if cJSON_IsString(j_action) == 0 {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Field 'action' is required (start|stop|pulse)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut action: motion_action_t = motion_action_t::MOTION_ACTION_START;
    if parse_action((*j_action).valuestring, &raw mut action) != 0 as ::core::ffi::c_int {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Field 'action' must be one of: start, stop, pulse\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut duration_ms: ::core::ffi::c_int = MOTION_PULSE_DEFAULT_MS;
    if action.0 == motion_action_t::MOTION_ACTION_PULSE.0 && !j_duration.is_null() {
        if cJSON_IsNumber(j_duration) == 0 {
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Field 'duration_ms' must be a positive integer\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut duration_value: ::core::ffi::c_double = (*j_duration).valuedouble;
        if duration_value <= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            || duration_value > MOTION_PULSE_MAX_MS as ::core::ffi::c_double
        {
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Field 'duration_ms' must be between 1 and 600000\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut d: ::core::ffi::c_int = duration_value as ::core::ffi::c_int;
        if d as ::core::ffi::c_double != duration_value {
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Field 'duration_ms' must be a positive integer\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        duration_ms = d;
    }
    let mut detections: detection_result_t = detection_result_t {
        count: 0,
        detections: [detection_t {
            label: [0; 32],
            confidence: 0.,
            x: 0.,
            y: 0.,
            width: 0.,
            height: 0.,
            track_id: 0,
            zone_id: [0; 32],
        }; 20],
    };
    memset(
        &raw mut detections as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<detection_result_t>(),
    );
    let mut tags: [[::core::ffi::c_char; 128]; 16] = [[0; 128]; 16];
    let mut parse_err: [::core::ffi::c_char; 160] = [
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
    ];
    if motion_trigger_parse_objects(
        body,
        &raw mut detections,
        &raw mut parse_err as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
    ) != 0 as ::core::ffi::c_int
    {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            &raw mut parse_err as *mut ::core::ffi::c_char,
        );
        return;
    }
    let mut tag_count: ::core::ffi::c_int = motion_trigger_parse_tags(
        body,
        &raw mut tags as *mut [::core::ffi::c_char; 128],
        &raw mut parse_err as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 160]>(),
    );
    if tag_count < 0 as ::core::ffi::c_int {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            &raw mut parse_err as *mut ::core::ffi::c_char,
        );
        return;
    }
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if strlen((*j_stream).valuestring) >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>() {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_strcpy(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        (*j_stream).valuestring,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as size_t,
    );
    cJSON_Delete(body);
    if httpd_authorize_stream_action(
        req,
        res,
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    ) == 0
    {
        return;
    }
    let mut handle: stream_handle_t =
        get_stream_by_name(&raw mut stream_name as *mut ::core::ffi::c_char);
    if handle.is_null() {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut cfg: stream_config_t = stream_config_t {
        camera_uuid: [0; 37],
        location_uuid: [0; 37],
        name: [0; 256],
        url: [0; 512],
        enabled: false,
        width: 0,
        height: 0,
        fps: 0,
        codec: [0; 16],
        priority: 0,
        record: false,
        segment_duration: 0,
        detection_based_recording: false,
        detection_model: [0; 512],
        detection_interval: 0,
        detection_threshold: 0.,
        pre_detection_buffer: 0,
        post_detection_buffer: 0,
        detection_api_url: [0; 512],
        detection_object_filter: [0; 16],
        detection_object_filter_list: [0; 256],
        streaming_enabled: false,
        protocol: stream_protocol_t::STREAM_PROTOCOL_TCP,
        record_audio: false,
        audio_voice_enhancement: false,
        onvif_username: [0; 64],
        onvif_password: [0; 64],
        onvif_profile: [0; 64],
        onvif_port: 0,
        onvif_discovery_enabled: false,
        is_onvif: false,
        backchannel_enabled: false,
        retention_days: 0,
        detection_retention_days: 0,
        max_storage_mb: 0,
        tier_critical_multiplier: 0.,
        tier_important_multiplier: 0.,
        tier_ephemeral_multiplier: 0.,
        storage_priority: 0,
        ptz_enabled: false,
        ptz_max_x: 0,
        ptz_max_y: 0,
        ptz_max_z: 0,
        ptz_has_home: false,
        record_on_schedule: false,
        recording_schedule: [0; 168],
        detection_record_on_schedule: false,
        detection_recording_schedule: [0; 168],
        tags: [0; 256],
        admin_url: [0; 512],
        privacy_mode: false,
        motion_trigger_source: [0; 256],
        go2rtc_source_override: [0; 8192],
        sub_stream_url: [0; 512],
        detection_url: [0; 512],
        publish_url: [0; 512],
        playback_transport: [0; 24],
        eptz_config: [0; 1024],
        observation_evidence_enabled: false,
        observation_labels: [0; 512],
        observation_max_age_days: 0,
        observation_max_bytes: 0,
    };
    memset(
        &raw mut cfg as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<stream_config_t>(),
    );
    if get_stream_config(handle, &raw mut cfg) != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to read stream config\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut schedule_allows_trigger: bool = is_detection_recording_scheduled(&raw mut cfg);
    let mut can_trigger_recording: bool = cfg.detection_based_recording as ::core::ffi::c_int != 0
        && schedule_allows_trigger as ::core::ffi::c_int != 0;
    if !cfg.detection_based_recording {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Stream '%s' has no detection-based recording; recording trigger skipped (detections/tags still recorded)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    } else if !schedule_allows_trigger {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Stream '%s' is outside its detection recording schedule; recording trigger skipped (detections/tags still recorded)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    let mut active: bool = action.0 != motion_action_t::MOTION_ACTION_STOP.0;
    pthread_mutex_lock(&raw mut g_pulse_gen_mutex);
    let mut new_gen: uint64_t =
        pulse_gen_increment(&raw mut stream_name as *mut ::core::ffi::c_char);
    if new_gen == 0 as uint64_t {
        pthread_mutex_unlock(&raw mut g_pulse_gen_mutex);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Motion trigger tracking capacity exhausted\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut stored_detections: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current_recording_id: uint64_t =
        get_current_recording_id_for_stream(&raw mut stream_name as *mut ::core::ffi::c_char);
    if active {
        close_external_motion_detections(
            &raw mut cfg.camera_uuid as *mut ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
            now,
        );
        let mut caller_supplied_objects: bool = detections.count > 0 as ::core::ffi::c_int;
        if detections.count > 0 as ::core::ffi::c_int
            && filter_detections_by_zones(
                &raw mut stream_name as *mut ::core::ffi::c_char,
                &raw mut detections,
            ) != 0 as ::core::ffi::c_int
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to filter detections by zones for '%s'; storing all\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        }
        if !caller_supplied_objects {
            detections.count = 1 as ::core::ffi::c_int;
            safe_strcpy(
                &raw mut (*(&raw mut detections.detections as *mut detection_t).offset(0isize))
                    .label as *mut ::core::ffi::c_char,
                b"motion\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                0 as size_t,
            );
            detections.detections[0usize].confidence = 1.0f32;
            detections.detections[0usize].track_id = -1 as ::core::ffi::c_int;
        }
        if detections.count > 0 as ::core::ffi::c_int {
            if store_external_motion_detections(
                &raw mut stream_name as *mut ::core::ffi::c_char,
                &raw mut detections,
                now,
                current_recording_id,
            ) == 0 as ::core::ffi::c_int
            {
                stored_detections = detections.count;
            } else {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to store external detections for stream '%s'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
            }
            if caller_supplied_objects {
                let mut event_error: [::core::ffi::c_char; 256] = [
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
                if event_producer_publish_detection(
                    &raw mut cfg.camera_uuid as *mut ::core::ffi::c_char,
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                    &raw mut detections,
                    now,
                    &raw mut event_error as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                ) != 0 as ::core::ffi::c_int
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_DEBUG,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Detection event enqueue failed for stream '%s': %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut stream_name as *mut ::core::ffi::c_char,
                        &raw mut event_error as *mut ::core::ffi::c_char,
                    );
                }
            }
        }
    } else {
        close_external_motion_detections(
            &raw mut cfg.camera_uuid as *mut ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
            now,
        );
    }
    let mut tags_applied: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if active as ::core::ffi::c_int != 0 && tag_count > 0 as ::core::ffi::c_int {
        if current_recording_id != 0 as uint64_t {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < tag_count {
                if db_recording_tag_add(
                    current_recording_id,
                    &raw mut *(&raw mut tags as *mut [::core::ffi::c_char; 128]).offset(i as isize)
                        as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    tags_applied += 1;
                }
                i += 1;
            }
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"No active recording for stream '%s'; %d tag(s) not applied\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
                tag_count,
            );
        }
    }
    if can_trigger_recording {
        unified_detection_notify_motion(&raw mut stream_name as *mut ::core::ffi::c_char, active);
    }
    process_motion_event(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        active,
        now,
        r#false != 0,
    );
    let mut action_str: *const ::core::ffi::c_char =
        if action.0 == motion_action_t::MOTION_ACTION_START.0 {
            b"start\0".as_ptr() as *const ::core::ffi::c_char
        } else if action.0 == motion_action_t::MOTION_ACTION_STOP.0 {
            b"stop\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"pulse\0".as_ptr() as *const ::core::ffi::c_char
        };
    if action.0 == motion_action_t::MOTION_ACTION_PULSE.0 {
        let mut task: *mut motion_pulse_task_t =
            calloc(1 as size_t, ::core::mem::size_of::<motion_pulse_task_t>())
                as *mut motion_pulse_task_t;
        if task.is_null() {
            if can_trigger_recording {
                unified_detection_notify_motion(
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                    r#false != 0,
                );
            }
            close_external_motion_detections(
                &raw mut cfg.camera_uuid as *mut ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
                now,
            );
            process_motion_event(
                &raw mut stream_name as *mut ::core::ffi::c_char,
                r#false != 0,
                now,
                r#false != 0,
            );
            pthread_mutex_unlock(&raw mut g_pulse_gen_mutex);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        safe_strcpy(
            &raw mut (*task).stream_name as *mut ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
        safe_strcpy(
            &raw mut (*task).camera_uuid as *mut ::core::ffi::c_char,
            &raw mut cfg.camera_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        (*task).duration_ms = duration_ms;
        (*task).generation = new_gen;
        (*task).trigger_recording = can_trigger_recording;
        let mut tid: pthread_t = 0;
        let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
        let mut rc: ::core::ffi::c_int = pthread_attr_init(&raw mut attr);
        if rc == 0 as ::core::ffi::c_int {
            rc = pthread_attr_setdetachstate(
                &raw mut attr,
                C2Rust_Unnamed::PTHREAD_CREATE_DETACHED.0 as ::core::ffi::c_int,
            );
            if rc == 0 as ::core::ffi::c_int {
                rc = pthread_create(
                    &raw mut tid,
                    &raw mut attr,
                    Some(
                        motion_pulse_worker
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                            )
                                -> *mut ::core::ffi::c_void,
                    ),
                    task as *mut ::core::ffi::c_void,
                );
            }
            pthread_attr_destroy(&raw mut attr);
        }
        if rc != 0 as ::core::ffi::c_int {
            if can_trigger_recording {
                unified_detection_notify_motion(
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                    r#false != 0,
                );
            }
            close_external_motion_detections(
                &raw mut cfg.camera_uuid as *mut ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
                now,
            );
            process_motion_event(
                &raw mut stream_name as *mut ::core::ffi::c_char,
                r#false != 0,
                now,
                r#false != 0,
            );
            free(task as *mut ::core::ffi::c_void);
            pthread_mutex_unlock(&raw mut g_pulse_gen_mutex);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to schedule pulse end\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Pulse started for stream '%s' (%d ms)\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
            duration_ms,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Motion %s for stream '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            action_str,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
    }
    pthread_mutex_unlock(&raw mut g_pulse_gen_mutex);
    let mut resp: *mut cJSON = cJSON_CreateObject();
    if resp.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        resp,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        resp,
        b"stream\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        resp,
        b"action\0".as_ptr() as *const ::core::ffi::c_char,
        action_str,
    );
    if action.0 == motion_action_t::MOTION_ACTION_PULSE.0 {
        cJSON_AddNumberToObject(
            resp,
            b"duration_ms\0".as_ptr() as *const ::core::ffi::c_char,
            duration_ms as ::core::ffi::c_double,
        );
    }
    cJSON_AddBoolToObject(
        resp,
        b"recording_triggered\0".as_ptr() as *const ::core::ffi::c_char,
        can_trigger_recording as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        resp,
        b"detections_stored\0".as_ptr() as *const ::core::ffi::c_char,
        stored_detections as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        resp,
        b"tags_applied\0".as_ptr() as *const ::core::ffi::c_char,
        tags_applied as ::core::ffi::c_double,
    );
    if !cfg.detection_based_recording {
        cJSON_AddStringToObject(
            resp,
            b"warning\0".as_ptr() as *const ::core::ffi::c_char,
            b"Stream does not have detection-based recording enabled; the event was recorded but no recording was triggered\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    } else if !schedule_allows_trigger {
        cJSON_AddStringToObject(
            resp,
            b"warning\0".as_ptr() as *const ::core::ffi::c_char,
            b"The event was recorded, but no recording was triggered because the stream is outside its detection recording schedule\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(resp);
    cJSON_Delete(resp);
    if json_str.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 202 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
}
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
