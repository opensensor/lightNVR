unsafe extern "C" {
    pub type fleet_selector;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn difftime(__time1: time_t, __time0: time_t) -> ::core::ffi::c_double;
    unsafe fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    unsafe fn strptime(
        __s: *const ::core::ffi::c_char,
        __fmt: *const ::core::ffi::c_char,
        __tp: *mut tm,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn timegm(__tp: *mut tm) -> time_t;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_CreateString(string: *const ::core::ffi::c_char) -> *mut cJSON;
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
    unsafe fn http_request_get_query_param(
        req: *const http_request_t,
        name: *const ::core::ffi::c_char,
        value: *mut ::core::ffi::c_char,
        value_len: size_t,
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
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn camera_collection_filter_load(
        collection_uuid: *const ::core::ffi::c_char,
        user: *const user_t,
        filter: *mut camera_collection_filter_t,
    ) -> camera_collection_filter_result_t;
    unsafe fn camera_collection_filter_free(filter: *mut camera_collection_filter_t);
    unsafe fn camera_collection_filter_matches(
        filter: *const camera_collection_filter_t,
        camera: *const fleet_camera_t,
    ) -> bool;
    unsafe fn is_shutdown_initiated() -> bool;
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
    unsafe fn db_recordings_generation() -> uint64_t;
    unsafe fn get_recording_count_for_cameras(
        start_time: time_t,
        end_time: time_t,
        stream_name: *const ::core::ffi::c_char,
        has_detection: ::core::ffi::c_int,
        detection_label: *const ::core::ffi::c_char,
        protected_filter: ::core::ffi::c_int,
        camera_uuids: *const *const ::core::ffi::c_char,
        camera_count: ::core::ffi::c_int,
        tag_filter: *const ::core::ffi::c_char,
        capture_method_filter: *const ::core::ffi::c_char,
        include_in_progress: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn get_recording_metadata_paginated_for_cameras(
        start_time: time_t,
        end_time: time_t,
        stream_name: *const ::core::ffi::c_char,
        has_detection: ::core::ffi::c_int,
        detection_label: *const ::core::ffi::c_char,
        protected_filter: ::core::ffi::c_int,
        sort_field: *const ::core::ffi::c_char,
        sort_order: *const ::core::ffi::c_char,
        metadata: *mut recording_metadata_t,
        limit: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        camera_uuids: *const *const ::core::ffi::c_char,
        camera_count: ::core::ffi::c_int,
        tag_filter: *const ::core::ffi::c_char,
        capture_method_filter: *const ::core::ffi::c_char,
        include_in_progress: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn recording_source_add_status(object: *mut cJSON, recording_id: uint64_t);
    unsafe fn get_recording_detection_summaries_for_cameras(
        recordings: *const recording_metadata_t,
        count: ::core::ffi::c_int,
        summaries: *mut recording_detection_summary_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_recording_tag_get(
        recording_id: uint64_t,
        tags: *mut [::core::ffi::c_char; 128],
        max_tags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_recording_tag_get_batch(
        recording_ids: *const uint64_t,
        count: ::core::ffi::c_int,
        tag_lists: *mut recording_tag_list_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
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
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strtok_r(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
        __save_ptr: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
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
pub type uint64_t = u64;
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_tag_list_t {
    pub count: ::core::ffi::c_int,
    pub tags: [[::core::ffi::c_char; 128]; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_detection_summary_t {
    pub has_detection: bool,
    pub label_count: ::core::ffi::c_int,
    pub labels: [detection_label_summary_t; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct detection_label_summary_t {
    pub label: [::core::ffi::c_char; 32],
    pub count: ::core::ffi::c_int,
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
pub struct count_key_builder_t {
    pub data: *mut ::core::ffi::c_char,
    pub length: size_t,
    pub capacity: size_t,
    pub failed: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_count_cache_entry_t {
    pub key: *mut ::core::ffi::c_char,
    pub total: ::core::ffi::c_int,
    pub stored_at: time_t,
    pub generation: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camera_collection_filter_t {
    pub active: bool,
    pub smart_selector: *mut fleet_selector_t,
    pub member_uuids: *mut [::core::ffi::c_char; 37],
    pub member_count: ::core::ffi::c_int,
}
pub type fleet_selector_t = fleet_selector;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct camera_collection_filter_result_t(pub ::core::ffi::c_int);
impl camera_collection_filter_result_t {
    pub const CAMERA_COLLECTION_FILTER_OK: Self = Self(0);
    pub const CAMERA_COLLECTION_FILTER_NOT_FOUND: Self = Self(-1);
    pub const CAMERA_COLLECTION_FILTER_INVALID_SELECTOR: Self = Self(-2);
    pub const CAMERA_COLLECTION_FILTER_OUT_OF_MEMORY: Self = Self(-3);
    pub const CAMERA_COLLECTION_FILTER_DATABASE_ERROR: Self = Self(-4);
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
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const LOG_COMPONENT: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"RecordingsAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
#[inline]
unsafe extern "C" fn recording_capture_method(
    mut recording: *const recording_metadata_t,
) -> *const ::core::ffi::c_char {
    if recording.is_null()
        || (*recording).trigger_type[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return b"scheduled\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcmp(
        &raw const (*recording).trigger_type as *const ::core::ffi::c_char,
        b"scheduled\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && (*recording).schedule_restricted == 0 as ::core::ffi::c_int
    {
        return b"continuous\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return &raw const (*recording).trigger_type as *const ::core::ffi::c_char;
}
pub const MAX_RECORDING_TAGS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MAX_SELECTED_STREAM_FILTERS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MAX_SELECTED_STREAM_NAME_LEN: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MAX_BATCHED_DETECTION_RECORDINGS: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const MAX_BATCHED_TAG_RECORDINGS: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const RECORDING_COUNT_CACHE_TTL_SECONDS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const RECORDING_COUNT_CACHE_ENTRIES: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
static mut recording_count_cache: [recording_count_cache_entry_t; 32] =
    [recording_count_cache_entry_t {
        key: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        total: 0,
        stored_at: 0,
        generation: 0,
    }; 32];
static mut recording_count_cache_mutex: pthread_mutex_t = pthread_mutex_t {
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
static mut recording_count_cache_cursor: ::core::ffi::c_uint = 0;
unsafe extern "C" fn count_key_append(
    mut builder: *mut count_key_builder_t,
    mut bytes: *const ::core::ffi::c_char,
    mut count: size_t,
) {
    if (*builder).failed {
        return;
    }
    if (*builder)
        .length
        .wrapping_add(count)
        .wrapping_add(1 as size_t)
        > (*builder).capacity
    {
        let mut capacity: size_t = if (*builder).capacity != 0 {
            (*builder).capacity
        } else {
            256 as size_t
        };
        while capacity
            < (*builder)
                .length
                .wrapping_add(count)
                .wrapping_add(1 as size_t)
        {
            capacity = capacity.wrapping_mul(2 as size_t);
        }
        let mut grown: *mut ::core::ffi::c_char =
            realloc((*builder).data as *mut ::core::ffi::c_void, capacity)
                as *mut ::core::ffi::c_char;
        if grown.is_null() {
            (*builder).failed = r#true != 0;
            return;
        }
        (*builder).data = grown;
        (*builder).capacity = capacity;
    }
    memcpy(
        (*builder).data.offset((*builder).length as isize) as *mut ::core::ffi::c_void,
        bytes as *const ::core::ffi::c_void,
        count,
    );
    (*builder).length = (*builder).length.wrapping_add(count);
    *(*builder).data.offset((*builder).length as isize) = '\0' as ::core::ffi::c_char;
}
unsafe extern "C" fn count_key_append_text(
    mut builder: *mut count_key_builder_t,
    mut tag: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) {
    let mut header: [::core::ffi::c_char; 64] = [0; 64];
    let mut length: size_t = if !value.is_null() {
        strlen(value)
    } else {
        0 as size_t
    };
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut header as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        b"%s=%zu:\0".as_ptr() as *const ::core::ffi::c_char,
        tag,
        length,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 64]>()
    {
        (*builder).failed = r#true != 0;
        return;
    }
    count_key_append(
        builder,
        &raw mut header as *mut ::core::ffi::c_char,
        written as size_t,
    );
    if length > 0 as size_t {
        count_key_append(builder, value, length);
    }
    count_key_append(
        builder,
        b";\0".as_ptr() as *const ::core::ffi::c_char,
        1 as size_t,
    );
}
unsafe extern "C" fn count_key_append_int(
    mut builder: *mut count_key_builder_t,
    mut tag: *const ::core::ffi::c_char,
    mut value: ::core::ffi::c_longlong,
) {
    let mut text: [::core::ffi::c_char; 80] = [0; 80];
    let mut written: ::core::ffi::c_int = snprintf(
        &raw mut text as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 80]>(),
        b"%s=%lld;\0".as_ptr() as *const ::core::ffi::c_char,
        tag,
        value,
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= ::core::mem::size_of::<[::core::ffi::c_char; 80]>()
    {
        (*builder).failed = r#true != 0;
        return;
    }
    count_key_append(
        builder,
        &raw mut text as *mut ::core::ffi::c_char,
        written as size_t,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recordings_count_cache_build_key(
    mut user_id: int64_t,
    mut start_time: time_t,
    mut end_time: time_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut has_detection: ::core::ffi::c_int,
    mut detection_label: *const ::core::ffi::c_char,
    mut protected_filter: ::core::ffi::c_int,
    mut allowed_streams: *const *const ::core::ffi::c_char,
    mut allowed_streams_count: ::core::ffi::c_int,
    mut tag_filter: *const ::core::ffi::c_char,
    mut capture_method_filter: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut builder: count_key_builder_t = count_key_builder_t {
        data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        length: 0,
        capacity: 0,
        failed: false,
    };
    count_key_append_int(
        &raw mut builder,
        b"user\0".as_ptr() as *const ::core::ffi::c_char,
        user_id as ::core::ffi::c_longlong,
    );
    count_key_append_int(
        &raw mut builder,
        b"start\0".as_ptr() as *const ::core::ffi::c_char,
        start_time as ::core::ffi::c_longlong,
    );
    count_key_append_int(
        &raw mut builder,
        b"end\0".as_ptr() as *const ::core::ffi::c_char,
        end_time as ::core::ffi::c_longlong,
    );
    count_key_append_text(
        &raw mut builder,
        b"stream\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    count_key_append_int(
        &raw mut builder,
        b"detection\0".as_ptr() as *const ::core::ffi::c_char,
        has_detection as ::core::ffi::c_longlong,
    );
    count_key_append_text(
        &raw mut builder,
        b"label\0".as_ptr() as *const ::core::ffi::c_char,
        detection_label,
    );
    count_key_append_int(
        &raw mut builder,
        b"protected\0".as_ptr() as *const ::core::ffi::c_char,
        protected_filter as ::core::ffi::c_longlong,
    );
    count_key_append_text(
        &raw mut builder,
        b"tag\0".as_ptr() as *const ::core::ffi::c_char,
        tag_filter,
    );
    count_key_append_text(
        &raw mut builder,
        b"capture\0".as_ptr() as *const ::core::ffi::c_char,
        capture_method_filter,
    );
    let mut scope_count: ::core::ffi::c_int = if !allowed_streams.is_null() {
        allowed_streams_count
    } else {
        0 as ::core::ffi::c_int
    };
    count_key_append_int(
        &raw mut builder,
        b"scope\0".as_ptr() as *const ::core::ffi::c_char,
        scope_count as ::core::ffi::c_longlong,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < scope_count {
        count_key_append_text(
            &raw mut builder,
            b"n\0".as_ptr() as *const ::core::ffi::c_char,
            *allowed_streams.offset(i as isize),
        );
        i += 1;
    }
    if builder.failed {
        free(builder.data as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return builder.data;
}
unsafe extern "C" fn count_cache_find_locked(
    mut key: *const ::core::ffi::c_char,
) -> *mut recording_count_cache_entry_t {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < RECORDING_COUNT_CACHE_ENTRIES {
        let mut entry: *mut recording_count_cache_entry_t = (&raw mut recording_count_cache
            as *mut recording_count_cache_entry_t)
            .offset(i as isize);
        if !(*entry).key.is_null() && strcmp((*entry).key, key) == 0 as ::core::ffi::c_int {
            return entry;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<recording_count_cache_entry_t>();
}
unsafe extern "C" fn count_cache_entry_fresh(
    mut entry: *const recording_count_cache_entry_t,
    mut now: time_t,
    mut generation: uint64_t,
) -> bool {
    return !(*entry).key.is_null()
        && (*entry).generation == generation
        && now >= (*entry).stored_at
        && now - (*entry).stored_at < RECORDING_COUNT_CACHE_TTL_SECONDS as time_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recordings_count_cache_lookup(
    mut key: *const ::core::ffi::c_char,
    mut now: time_t,
    mut generation: uint64_t,
    mut total: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if key.is_null() || total.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let mut hit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    pthread_mutex_lock(&raw mut recording_count_cache_mutex);
    let mut entry: *mut recording_count_cache_entry_t = count_cache_find_locked(key);
    if !entry.is_null()
        && count_cache_entry_fresh(entry, now, generation) as ::core::ffi::c_int != 0
    {
        *total = (*entry).total;
        hit = 1 as ::core::ffi::c_int;
    }
    pthread_mutex_unlock(&raw mut recording_count_cache_mutex);
    return hit;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recordings_count_cache_store(
    mut key: *const ::core::ffi::c_char,
    mut now: time_t,
    mut generation: uint64_t,
    mut total: ::core::ffi::c_int,
) {
    if key.is_null() || total < 0 as ::core::ffi::c_int {
        return;
    }
    pthread_mutex_lock(&raw mut recording_count_cache_mutex);
    let mut entry: *mut recording_count_cache_entry_t = count_cache_find_locked(key);
    if entry.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < RECORDING_COUNT_CACHE_ENTRIES && entry.is_null() {
            if recording_count_cache[i as usize].key.is_null() {
                entry = (&raw mut recording_count_cache as *mut recording_count_cache_entry_t)
                    .offset(i as isize);
            }
            i += 1;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < RECORDING_COUNT_CACHE_ENTRIES && entry.is_null() {
            if !count_cache_entry_fresh(
                (&raw mut recording_count_cache as *mut recording_count_cache_entry_t)
                    .offset(i_0 as isize),
                now,
                generation,
            ) {
                entry = (&raw mut recording_count_cache as *mut recording_count_cache_entry_t)
                    .offset(i_0 as isize);
            }
            i_0 += 1;
        }
        if entry.is_null() {
            let c2rust_fresh1 = recording_count_cache_cursor;
            recording_count_cache_cursor = recording_count_cache_cursor.wrapping_add(1);
            entry = (&raw mut recording_count_cache as *mut recording_count_cache_entry_t).offset(
                c2rust_fresh1.wrapping_rem(RECORDING_COUNT_CACHE_ENTRIES as ::core::ffi::c_uint)
                    as isize,
            );
        }
        let mut copy: *mut ::core::ffi::c_char = strdup(key);
        if copy.is_null() {
            pthread_mutex_unlock(&raw mut recording_count_cache_mutex);
            return;
        }
        free((*entry).key as *mut ::core::ffi::c_void);
        (*entry).key = copy;
    }
    (*entry).total = total;
    (*entry).stored_at = now;
    (*entry).generation = generation;
    pthread_mutex_unlock(&raw mut recording_count_cache_mutex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recordings_count_cache_reset() {
    pthread_mutex_lock(&raw mut recording_count_cache_mutex);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < RECORDING_COUNT_CACHE_ENTRIES {
        free(recording_count_cache[i as usize].key as *mut ::core::ffi::c_void);
        memset(
            (&raw mut recording_count_cache as *mut recording_count_cache_entry_t)
                .offset(i as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<recording_count_cache_entry_t>(),
        );
        i += 1;
    }
    recording_count_cache_cursor = 0 as ::core::ffi::c_uint;
    pthread_mutex_unlock(&raw mut recording_count_cache_mutex);
}
unsafe extern "C" fn parse_selected_streams(
    mut csv: *const ::core::ffi::c_char,
    mut values: *mut [::core::ffi::c_char; 64],
    mut max_values: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if csv.is_null() || *csv == 0 || max_values <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut buffer: [::core::ffi::c_char; 2048] = [0; 2048];
    safe_strcpy(
        &raw mut buffer as *mut ::core::ffi::c_char,
        csv,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
        0 as size_t,
    );
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut token: *mut ::core::ffi::c_char = strtok_r(
        &raw mut buffer as *mut ::core::ffi::c_char,
        b",\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut saveptr,
    );
    while !token.is_null() && count < max_values {
        if copy_trimmed_value(
            &raw mut *values.offset(count as isize) as *mut ::core::ffi::c_char,
            MAX_SELECTED_STREAM_NAME_LEN as size_t,
            token,
            0 as size_t,
        ) != 0
        {
            count += 1;
        }
        token = strtok_r(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b",\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
    }
    return count;
}
unsafe extern "C" fn valid_uuid(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null()
        || strlen(value) != (CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int) as size_t
    {
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int {
        let mut c: ::core::ffi::c_char = *value.offset(i as isize);
        if i == 8 as ::core::ffi::c_int
            || i == 13 as ::core::ffi::c_int
            || i == 18 as ::core::ffi::c_int
            || i == 23 as ::core::ffi::c_int
        {
            if c as ::core::ffi::c_int != '-' as ::core::ffi::c_int {
                return r#false != 0;
            }
        } else if *(*__ctype_b_loc())
            .offset(c as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISxdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            == 0
        {
            return r#false != 0;
        }
        i += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn respond_empty_recordings(
    mut res: *mut http_response_t,
    mut page: ::core::ffi::c_int,
    mut limit: ::core::ffi::c_int,
) {
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut recordings: *mut cJSON = cJSON_CreateArray();
    let mut pagination: *mut cJSON = cJSON_CreateObject();
    if root.is_null() || recordings.is_null() || pagination.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(recordings);
        cJSON_Delete(pagination);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddItemToObject(
        root,
        b"recordings\0".as_ptr() as *const ::core::ffi::c_char,
        recordings,
    );
    cJSON_AddNumberToObject(
        pagination,
        b"page\0".as_ptr() as *const ::core::ffi::c_char,
        page as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        pagination,
        b"pages\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        pagination,
        b"total\0".as_ptr() as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        pagination,
        b"limit\0".as_ptr() as *const ::core::ffi::c_char,
        limit as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"pagination\0".as_ptr() as *const ::core::ffi::c_char,
        pagination,
    );
    let mut json: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    http_response_set_json(
        res,
        200 as ::core::ffi::c_int,
        if !json.is_null() {
            json
        } else {
            b"{}\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    free(json as *mut ::core::ffi::c_void);
    cJSON_Delete(root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_recordings(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if is_shutdown_initiated() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Shutdown in progress, rejecting recordings request\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            503 as ::core::ffi::c_int,
            b"Service shutting down\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Processing GET /api/recordings request\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut auth_user: user_t = user_t {
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
        &raw mut auth_user as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    if httpd_check_action_access(req, &raw mut auth_user) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut stream_name: [::core::ffi::c_char; 256] = [
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
    let mut start_time_str: [::core::ffi::c_char; 64] = [
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
    let mut end_time_str: [::core::ffi::c_char; 64] = [
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
    let mut page_str: [::core::ffi::c_char; 16] = [
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
    ];
    let mut limit_str: [::core::ffi::c_char; 16] = [
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
    ];
    let mut sort_field: [::core::ffi::c_char; 32] =
        ::core::mem::transmute::<[u8; 32], [::core::ffi::c_char; 32]>(
            *b"start_time\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
    let mut sort_order: [::core::ffi::c_char; 8] =
        ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"desc\0\0\0\0");
    let mut has_detection_str: [::core::ffi::c_char; 8] =
        [0 as ::core::ffi::c_char, 0, 0, 0, 0, 0, 0, 0];
    let mut detection_label: [::core::ffi::c_char; 256] = [
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
    let mut protected_str: [::core::ffi::c_char; 8] =
        [0 as ::core::ffi::c_char, 0, 0, 0, 0, 0, 0, 0];
    let mut tag_filter_str: [::core::ffi::c_char; 512] = [
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
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
    let mut capture_method_str: [::core::ffi::c_char; 128] = [
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
    ];
    let mut collection_uuid: [::core::ffi::c_char; 37] = [
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
    ];
    http_request_get_query_param(
        req,
        b"stream\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    http_request_get_query_param(
        req,
        b"start\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut start_time_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    http_request_get_query_param(
        req,
        b"end\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut end_time_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    http_request_get_query_param(
        req,
        b"page\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut page_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
    );
    http_request_get_query_param(
        req,
        b"limit\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut limit_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
    );
    http_request_get_query_param(
        req,
        b"sort\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sort_field as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    );
    http_request_get_query_param(
        req,
        b"order\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut sort_order as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
    );
    http_request_get_query_param(
        req,
        b"has_detection\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut has_detection_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
    );
    http_request_get_query_param(
        req,
        b"detection_label\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut detection_label as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    http_request_get_query_param(
        req,
        b"protected\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut protected_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
    );
    http_request_get_query_param(
        req,
        b"tag\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut tag_filter_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    );
    http_request_get_query_param(
        req,
        b"capture_method\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut capture_method_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    );
    http_request_get_query_param(
        req,
        b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut collection_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
    );
    if collection_uuid[0usize] as ::core::ffi::c_int != 0
        && !valid_uuid(&raw mut collection_uuid as *mut ::core::ffi::c_char)
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut page: ::core::ffi::c_int = if page_str[0usize] as ::core::ffi::c_int != 0 {
        strtol(
            &raw mut page_str as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    let mut all_limit_requested: ::core::ffi::c_int =
        (limit_str[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && strcasecmp(
                &raw mut limit_str as *mut ::core::ffi::c_char,
                b"all\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut limit: ::core::ffi::c_int = if all_limit_requested != 0 {
        20 as ::core::ffi::c_int
    } else if limit_str[0usize] as ::core::ffi::c_int != 0 {
        strtol(
            &raw mut limit_str as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int
    } else {
        20 as ::core::ffi::c_int
    };
    let mut recordings: *mut recording_metadata_t = ::core::ptr::null_mut::<recording_metadata_t>();
    let mut has_detection: ::core::ffi::c_int =
        if has_detection_str[0usize] as ::core::ffi::c_int != 0 {
            strtol(
                &raw mut has_detection_str as *mut ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    if has_detection < -1 as ::core::ffi::c_int {
        has_detection = -1 as ::core::ffi::c_int;
    } else if has_detection > 1 as ::core::ffi::c_int {
        has_detection = 1 as ::core::ffi::c_int;
    }
    let mut protected_filter: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if protected_str[0usize] as ::core::ffi::c_int == '0' as ::core::ffi::c_int {
        protected_filter = 0 as ::core::ffi::c_int;
    } else if protected_str[0usize] as ::core::ffi::c_int == '1' as ::core::ffi::c_int {
        protected_filter = 1 as ::core::ffi::c_int;
    }
    if page <= 0 as ::core::ffi::c_int {
        page = 1 as ::core::ffi::c_int;
    }
    if limit <= 0 as ::core::ffi::c_int {
        limit = 20 as ::core::ffi::c_int;
    }
    if limit > 1000 as ::core::ffi::c_int {
        limit = 1000 as ::core::ffi::c_int;
    }
    let mut offset: ::core::ffi::c_int = (page - 1 as ::core::ffi::c_int) * limit;
    let mut start_time: time_t = 0 as time_t;
    let mut end_time: time_t = 0 as time_t;
    if start_time_str[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Parsing start time string (decoded): %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut start_time_str as *mut ::core::ffi::c_char,
        );
        let mut tm: tm = tm {
            tm_sec: 0 as ::core::ffi::c_int,
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
        if !strptime(
            &raw mut start_time_str as *mut ::core::ffi::c_char,
            b"%Y-%m-%dT%H:%M:%S\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut tm,
        )
        .is_null()
            || !strptime(
                &raw mut start_time_str as *mut ::core::ffi::c_char,
                b"%Y-%m-%dT%H:%M:%S.000Z\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut tm,
            )
            .is_null()
            || !strptime(
                &raw mut start_time_str as *mut ::core::ffi::c_char,
                b"%Y-%m-%dT%H:%M:%S.000\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut tm,
            )
            .is_null()
            || !strptime(
                &raw mut start_time_str as *mut ::core::ffi::c_char,
                b"%Y-%m-%dT%H:%M:%SZ\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut tm,
            )
            .is_null()
        {
            tm.tm_isdst = 0 as ::core::ffi::c_int;
            start_time = timegm(&raw mut tm);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Parsed start time: %ld\0".as_ptr() as *const ::core::ffi::c_char,
                start_time,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to parse start time string: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut start_time_str as *mut ::core::ffi::c_char,
            );
        }
    }
    if end_time_str[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Parsing end time string (decoded): %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut end_time_str as *mut ::core::ffi::c_char,
        );
        let mut tm_0: tm = tm {
            tm_sec: 0 as ::core::ffi::c_int,
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
        if !strptime(
            &raw mut end_time_str as *mut ::core::ffi::c_char,
            b"%Y-%m-%dT%H:%M:%S\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut tm_0,
        )
        .is_null()
            || !strptime(
                &raw mut end_time_str as *mut ::core::ffi::c_char,
                b"%Y-%m-%dT%H:%M:%S.000Z\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut tm_0,
            )
            .is_null()
            || !strptime(
                &raw mut end_time_str as *mut ::core::ffi::c_char,
                b"%Y-%m-%dT%H:%M:%S.000\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut tm_0,
            )
            .is_null()
            || !strptime(
                &raw mut end_time_str as *mut ::core::ffi::c_char,
                b"%Y-%m-%dT%H:%M:%SZ\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut tm_0,
            )
            .is_null()
        {
            tm_0.tm_isdst = 0 as ::core::ffi::c_int;
            end_time = timegm(&raw mut tm_0);
            _log_message_ctx(
                log_level_t::LOG_LEVEL_DEBUG,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Parsed end time: %ld\0".as_ptr() as *const ::core::ffi::c_char,
                end_time,
            );
        } else {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to parse end time string: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut end_time_str as *mut ::core::ffi::c_char,
            );
        }
    }
    let mut label_filter: *const ::core::ffi::c_char =
        if detection_label[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            &raw mut detection_label as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
    if !label_filter.is_null() {
        has_detection = 1 as ::core::ffi::c_int;
    }
    let mut all_stream_cfgs: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut authorized_camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if db_fleet_camera_load(&raw mut all_stream_cfgs, &raw mut authorized_camera_count)
        != 0 as ::core::ffi::c_int
        || authorization_filter_cameras(
            &raw mut auth_user,
            authorization_action_t::AUTHZ_RECORDINGS_REPLAY,
            all_stream_cfgs,
            &raw mut authorized_camera_count,
        ) != 0 as ::core::ffi::c_int
    {
        free(all_stream_cfgs as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if authorized_camera_count > MAX_STREAMS {
        free(all_stream_cfgs as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Camera scope exceeds supported limit\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut requested_streams: [[::core::ffi::c_char; 64]; 32] = [
        [
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
        ],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
        [0; 64],
    ];
    let mut requested_stream_count: ::core::ffi::c_int =
        if stream_name[0usize] as ::core::ffi::c_int != 0 {
            parse_selected_streams(
                &raw mut stream_name as *mut ::core::ffi::c_char,
                &raw mut requested_streams as *mut [::core::ffi::c_char; 64],
                MAX_SELECTED_STREAM_FILTERS,
            )
        } else {
            0 as ::core::ffi::c_int
        };
    if stream_name[0usize] != 0 {
        let mut permitted: bool = requested_stream_count > 0 as ::core::ffi::c_int;
        let mut requested: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while requested < requested_stream_count && permitted as ::core::ffi::c_int != 0 {
            let mut found: bool = r#false != 0;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < authorized_camera_count {
                if strcmp(
                    &raw mut *(&raw mut requested_streams as *mut [::core::ffi::c_char; 64])
                        .offset(requested as isize) as *mut ::core::ffi::c_char,
                    &raw mut (*all_stream_cfgs.offset(i as isize)).name as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    found = r#true != 0;
                }
                i += 1;
            }
            if !found {
                permitted = r#false != 0;
            }
            requested += 1;
        }
        if !permitted {
            free(all_stream_cfgs as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                res,
                403 as ::core::ffi::c_int,
                b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    let mut collection: camera_collection_filter_t = camera_collection_filter_t {
        active: false,
        smart_selector: ::core::ptr::null_mut::<fleet_selector_t>(),
        member_uuids: ::core::ptr::null_mut::<[::core::ffi::c_char; 37]>(),
        member_count: 0,
    };
    if collection_uuid[0usize] != 0 {
        let mut result: camera_collection_filter_result_t = camera_collection_filter_load(
            &raw mut collection_uuid as *mut ::core::ffi::c_char,
            &raw mut auth_user,
            &raw mut collection,
        );
        if result.0 != camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_OK.0 {
            free(all_stream_cfgs as *mut ::core::ffi::c_void);
            camera_collection_filter_free(&raw mut collection);
            http_response_set_json_error(
                res,
                if result.0
                    == camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_NOT_FOUND.0
                {
                    404 as ::core::ffi::c_int
                } else {
                    500 as ::core::ffi::c_int
                },
                if result.0
                    == camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_NOT_FOUND.0
                {
                    b"Collection not found\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"Failed to load collection members\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            return;
        }
    }
    let mut camera_scope: [*const ::core::ffi::c_char; 1024] =
        [::core::ptr::null::<::core::ffi::c_char>(); 1024];
    let mut count_scope: [*const ::core::ffi::c_char; 2048] =
        [::core::ptr::null::<::core::ffi::c_char>(); 2048];
    let mut camera_scope_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < authorized_camera_count {
        let mut requested_0: bool = requested_stream_count == 0 as ::core::ffi::c_int;
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < requested_stream_count {
            if strcmp(
                &raw mut *(&raw mut requested_streams as *mut [::core::ffi::c_char; 64])
                    .offset(j as isize) as *mut ::core::ffi::c_char,
                &raw mut (*all_stream_cfgs.offset(i_0 as isize)).name as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                requested_0 = r#true != 0;
            }
            j += 1;
        }
        if requested_0 as ::core::ffi::c_int != 0
            && (collection_uuid[0usize] == 0
                || camera_collection_filter_matches(
                    &raw mut collection,
                    all_stream_cfgs.offset(i_0 as isize),
                ) as ::core::ffi::c_int
                    != 0)
        {
            count_scope[(camera_scope_count * 2 as ::core::ffi::c_int) as usize] =
                &raw mut (*all_stream_cfgs.offset(i_0 as isize)).camera_uuid
                    as *mut ::core::ffi::c_char;
            count_scope[(camera_scope_count * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as usize] =
                &raw mut (*all_stream_cfgs.offset(i_0 as isize)).name as *mut ::core::ffi::c_char;
            let c2rust_fresh0 = camera_scope_count;
            camera_scope_count += 1;
            camera_scope[c2rust_fresh0 as usize] = &raw mut (*all_stream_cfgs.offset(i_0 as isize))
                .camera_uuid
                as *mut ::core::ffi::c_char;
        }
        i_0 += 1;
    }
    camera_collection_filter_free(&raw mut collection);
    if camera_scope_count == 0 as ::core::ffi::c_int {
        free(all_stream_cfgs as *mut ::core::ffi::c_void);
        respond_empty_recordings(res, page, limit);
        return;
    }
    let mut streams_filter: *const *const ::core::ffi::c_char =
        &raw mut camera_scope as *mut *const ::core::ffi::c_char;
    let mut streams_filter_count: ::core::ffi::c_int = camera_scope_count;
    let mut tag_filt: *const ::core::ffi::c_char =
        if tag_filter_str[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            &raw mut tag_filter_str as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
    let mut capture_filt: *const ::core::ffi::c_char =
        if capture_method_str[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            &raw mut capture_method_str as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
    let mut stream_filt: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut recordings_generation: uint64_t = db_recordings_generation();
    let mut count_now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut count_key: *mut ::core::ffi::c_char = if all_limit_requested != 0 {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    } else {
        recordings_count_cache_build_key(
            auth_user.id,
            start_time,
            end_time,
            stream_filt,
            has_detection,
            label_filter,
            protected_filter,
            &raw mut count_scope as *mut *const ::core::ffi::c_char,
            camera_scope_count * 2 as ::core::ffi::c_int,
            tag_filt,
            capture_filt,
        )
    };
    let mut total_count: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    if count_key.is_null()
        || recordings_count_cache_lookup(
            count_key,
            count_now,
            recordings_generation,
            &raw mut total_count,
        ) == 0
    {
        total_count = get_recording_count_for_cameras(
            start_time,
            end_time,
            stream_filt,
            has_detection,
            label_filter,
            protected_filter,
            streams_filter,
            streams_filter_count,
            tag_filt,
            capture_filt,
            r#true != 0,
        );
        if !count_key.is_null() && total_count >= 0 as ::core::ffi::c_int {
            recordings_count_cache_store(count_key, count_now, recordings_generation, total_count);
        }
    }
    free(count_key as *mut ::core::ffi::c_void);
    if total_count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to get total recording count from database\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        if !all_stream_cfgs.is_null() {
            free(all_stream_cfgs as *mut ::core::ffi::c_void);
        }
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get recording count from database\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if all_limit_requested != 0 {
        page = 1 as ::core::ffi::c_int;
        limit = if total_count > 0 as ::core::ffi::c_int {
            total_count
        } else {
            1 as ::core::ffi::c_int
        };
        offset = 0 as ::core::ffi::c_int;
    }
    recordings =
        malloc((limit as size_t).wrapping_mul(::core::mem::size_of::<recording_metadata_t>()))
            as *mut recording_metadata_t;
    if recordings.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to allocate memory for recordings\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !all_stream_cfgs.is_null() {
            free(all_stream_cfgs as *mut ::core::ffi::c_void);
        }
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to allocate memory for recordings\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int = get_recording_metadata_paginated_for_cameras(
        start_time,
        end_time,
        stream_filt,
        has_detection,
        label_filter,
        protected_filter,
        &raw mut sort_field as *mut ::core::ffi::c_char,
        &raw mut sort_order as *mut ::core::ffi::c_char,
        recordings,
        limit,
        offset,
        streams_filter,
        streams_filter_count,
        tag_filt,
        capture_filt,
        r#true != 0,
    );
    if count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to get recordings from database\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(recordings as *mut ::core::ffi::c_void);
        if !all_stream_cfgs.is_null() {
            free(all_stream_cfgs as *mut ::core::ffi::c_void);
        }
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get recordings from database\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    if response.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create response JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(recordings as *mut ::core::ffi::c_void);
        if !all_stream_cfgs.is_null() {
            free(all_stream_cfgs as *mut ::core::ffi::c_void);
        }
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut recordings_array: *mut cJSON = cJSON_CreateArray();
    if recordings_array.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create recordings JSON array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(recordings as *mut ::core::ffi::c_void);
        if !all_stream_cfgs.is_null() {
            free(all_stream_cfgs as *mut ::core::ffi::c_void);
        }
        cJSON_Delete(response);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create recordings JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddItemToObject(
        response,
        b"recordings\0".as_ptr() as *const ::core::ffi::c_char,
        recordings_array,
    );
    let mut pagination: *mut cJSON = cJSON_CreateObject();
    if pagination.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create pagination JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        free(recordings as *mut ::core::ffi::c_void);
        if !all_stream_cfgs.is_null() {
            free(all_stream_cfgs as *mut ::core::ffi::c_void);
        }
        cJSON_Delete(response);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create pagination JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut total_pages: ::core::ffi::c_int =
        (total_count + limit - 1 as ::core::ffi::c_int) / limit;
    cJSON_AddNumberToObject(
        pagination,
        b"page\0".as_ptr() as *const ::core::ffi::c_char,
        page as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        pagination,
        b"pages\0".as_ptr() as *const ::core::ffi::c_char,
        total_pages as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        pagination,
        b"total\0".as_ptr() as *const ::core::ffi::c_char,
        total_count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        pagination,
        b"limit\0".as_ptr() as *const ::core::ffi::c_char,
        limit as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        response,
        b"pagination\0".as_ptr() as *const ::core::ffi::c_char,
        pagination,
    );
    let live_end: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < count {
        if !(*recordings.offset(i_1 as isize)).is_complete {
            (*recordings.offset(i_1 as isize)).end_time = live_end;
        }
        i_1 += 1;
    }
    let mut detection_summaries: *mut recording_detection_summary_t =
        ::core::ptr::null_mut::<recording_detection_summary_t>();
    let mut recording_tag_lists: *mut recording_tag_list_t =
        ::core::ptr::null_mut::<recording_tag_list_t>();
    if count > 0 as ::core::ffi::c_int && count <= MAX_BATCHED_DETECTION_RECORDINGS {
        detection_summaries = calloc(
            count as size_t,
            ::core::mem::size_of::<recording_detection_summary_t>(),
        ) as *mut recording_detection_summary_t;
        if !detection_summaries.is_null()
            && get_recording_detection_summaries_for_cameras(recordings, count, detection_summaries)
                != 0 as ::core::ffi::c_int
        {
            free(detection_summaries as *mut ::core::ffi::c_void);
            free(recordings as *mut ::core::ffi::c_void);
            free(all_stream_cfgs as *mut ::core::ffi::c_void);
            cJSON_Delete(response);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to get recording detections\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    if count > 0 as ::core::ffi::c_int && count <= MAX_BATCHED_TAG_RECORDINGS {
        recording_tag_lists = calloc(
            count as size_t,
            ::core::mem::size_of::<recording_tag_list_t>(),
        ) as *mut recording_tag_list_t;
        let mut recording_ids: *mut uint64_t =
            malloc((count as size_t).wrapping_mul(::core::mem::size_of::<uint64_t>()))
                as *mut uint64_t;
        if !recording_tag_lists.is_null() && !recording_ids.is_null() {
            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_2 < count {
                *recording_ids.offset(i_2 as isize) = (*recordings.offset(i_2 as isize)).id;
                i_2 += 1;
            }
            if db_recording_tag_get_batch(recording_ids, count, recording_tag_lists)
                < 0 as ::core::ffi::c_int
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Falling back to per-recording tag queries\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                free(recording_tag_lists as *mut ::core::ffi::c_void);
                recording_tag_lists = ::core::ptr::null_mut::<recording_tag_list_t>();
            }
        } else {
            free(recording_tag_lists as *mut ::core::ffi::c_void);
            recording_tag_lists = ::core::ptr::null_mut::<recording_tag_list_t>();
        }
        free(recording_ids as *mut ::core::ffi::c_void);
    }
    let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_3 < count {
        let mut recording: *mut cJSON = cJSON_CreateObject();
        if recording.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to create recording JSON object\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            let in_progress: bool = !(*recordings.offset(i_3 as isize)).is_complete;
            let mut start_time_formatted: [::core::ffi::c_char; 32] = [
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
            let mut end_time_formatted: [::core::ffi::c_char; 32] = [
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
            let mut tm_info_buf: tm = tm {
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
            let mut tm_info: *const tm = ::core::ptr::null::<tm>();
            tm_info = gmtime_r(
                &raw mut (*recordings.offset(i_3 as isize)).start_time,
                &raw mut tm_info_buf,
            );
            if !tm_info.is_null() {
                strftime(
                    &raw mut start_time_formatted as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                    b"%Y-%m-%dT%H:%M:%SZ\0".as_ptr() as *const ::core::ffi::c_char,
                    tm_info,
                );
            }
            if !in_progress {
                tm_info = gmtime_r(
                    &raw mut (*recordings.offset(i_3 as isize)).end_time,
                    &raw mut tm_info_buf,
                );
                if !tm_info.is_null() {
                    strftime(
                        &raw mut end_time_formatted as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                        b"%Y-%m-%dT%H:%M:%SZ\0".as_ptr() as *const ::core::ffi::c_char,
                        tm_info,
                    );
                }
            }
            let mut duration: ::core::ffi::c_int = difftime(
                (*recordings.offset(i_3 as isize)).end_time,
                (*recordings.offset(i_3 as isize)).start_time,
            ) as ::core::ffi::c_int;
            if duration < 0 as ::core::ffi::c_int {
                duration = 0 as ::core::ffi::c_int;
            }
            let mut size_str: [::core::ffi::c_char; 32] = [
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
            if (*recordings.offset(i_3 as isize)).size_bytes < 1024 as uint64_t {
                snprintf(
                    &raw mut size_str as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                    b"%lu B\0".as_ptr() as *const ::core::ffi::c_char,
                    (*recordings.offset(i_3 as isize)).size_bytes as ::core::ffi::c_ulong,
                );
            } else if (*recordings.offset(i_3 as isize)).size_bytes
                < (1024 as ::core::ffi::c_int as uint64_t).wrapping_mul(1024 as uint64_t)
            {
                snprintf(
                    &raw mut size_str as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                    b"%.1f KB\0".as_ptr() as *const ::core::ffi::c_char,
                    (*recordings.offset(i_3 as isize)).size_bytes as ::core::ffi::c_double
                        / 1024.0f64,
                );
            } else if (*recordings.offset(i_3 as isize)).size_bytes
                < (1024 as ::core::ffi::c_int as uint64_t)
                    .wrapping_mul(1024 as uint64_t)
                    .wrapping_mul(1024 as uint64_t)
            {
                snprintf(
                    &raw mut size_str as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                    b"%.1f MB\0".as_ptr() as *const ::core::ffi::c_char,
                    (*recordings.offset(i_3 as isize)).size_bytes as ::core::ffi::c_double
                        / (1024.0f64 * 1024.0f64),
                );
            } else {
                snprintf(
                    &raw mut size_str as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                    b"%.1f GB\0".as_ptr() as *const ::core::ffi::c_char,
                    (*recordings.offset(i_3 as isize)).size_bytes as ::core::ffi::c_double
                        / (1024.0f64 * 1024.0f64 * 1024.0f64),
                );
            }
            cJSON_AddNumberToObject(
                recording,
                b"id\0".as_ptr() as *const ::core::ffi::c_char,
                (*recordings.offset(i_3 as isize)).id as ::core::ffi::c_double,
            );
            cJSON_AddStringToObject(
                recording,
                b"stream\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*recordings.offset(i_3 as isize)).stream_name as *mut ::core::ffi::c_char,
            );
            if (*recordings.offset(i_3 as isize)).camera_uuid[0usize] as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
            {
                cJSON_AddStringToObject(
                    recording,
                    b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*recordings.offset(i_3 as isize)).camera_uuid
                        as *mut ::core::ffi::c_char,
                );
            } else {
                cJSON_AddNullToObject(
                    recording,
                    b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            cJSON_AddStringToObject(
                recording,
                b"file_path\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*recordings.offset(i_3 as isize)).file_path as *mut ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                recording,
                b"start_time\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut start_time_formatted as *mut ::core::ffi::c_char,
            );
            if in_progress {
                cJSON_AddNullToObject(
                    recording,
                    b"end_time\0".as_ptr() as *const ::core::ffi::c_char,
                );
                cJSON_AddNullToObject(
                    recording,
                    b"end_time_unix\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                cJSON_AddStringToObject(
                    recording,
                    b"end_time\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut end_time_formatted as *mut ::core::ffi::c_char,
                );
                cJSON_AddNumberToObject(
                    recording,
                    b"end_time_unix\0".as_ptr() as *const ::core::ffi::c_char,
                    (*recordings.offset(i_3 as isize)).end_time as ::core::ffi::c_double,
                );
            }
            cJSON_AddNumberToObject(
                recording,
                b"start_time_unix\0".as_ptr() as *const ::core::ffi::c_char,
                (*recordings.offset(i_3 as isize)).start_time as ::core::ffi::c_double,
            );
            cJSON_AddBoolToObject(
                recording,
                b"in_progress\0".as_ptr() as *const ::core::ffi::c_char,
                in_progress as cJSON_bool,
            );
            cJSON_AddStringToObject(
                recording,
                b"playback_issue\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*recordings.offset(i_3 as isize)).playback_issue
                    as *mut ::core::ffi::c_char,
            );
            cJSON_AddNumberToObject(
                recording,
                b"duration\0".as_ptr() as *const ::core::ffi::c_char,
                duration as ::core::ffi::c_double,
            );
            cJSON_AddStringToObject(
                recording,
                b"size\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut size_str as *mut ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                recording,
                b"capture_method\0".as_ptr() as *const ::core::ffi::c_char,
                recording_capture_method(recordings.offset(i_3 as isize)),
            );
            if (*recordings.offset(i_3 as isize)).schedule_restricted < 0 as ::core::ffi::c_int {
                cJSON_AddNullToObject(
                    recording,
                    b"schedule_restricted\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                cJSON_AddBoolToObject(
                    recording,
                    b"schedule_restricted\0".as_ptr() as *const ::core::ffi::c_char,
                    ((*recordings.offset(i_3 as isize)).schedule_restricted
                        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                );
            }
            let mut has_detection_flag: bool = strcmp(
                &raw mut (*recordings.offset(i_3 as isize)).trigger_type
                    as *mut ::core::ffi::c_char,
                b"detection\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int;
            let mut fallback_summary: recording_detection_summary_t =
                recording_detection_summary_t {
                    has_detection: false,
                    label_count: 0,
                    labels: [detection_label_summary_t {
                        label: [0; 32],
                        count: 0,
                    }; 10],
                };
            let mut labels: *mut detection_label_summary_t =
                &raw mut fallback_summary.labels as *mut detection_label_summary_t;
            let mut label_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if !detection_summaries.is_null() {
                labels = &raw mut (*detection_summaries.offset(i_3 as isize)).labels
                    as *mut detection_label_summary_t;
                label_count = (*detection_summaries.offset(i_3 as isize)).label_count;
                if (*detection_summaries.offset(i_3 as isize)).has_detection {
                    has_detection_flag = r#true != 0;
                }
            } else if (*recordings.offset(i_3 as isize)).start_time > 0 as time_t
                && (*recordings.offset(i_3 as isize)).end_time > 0 as time_t
            {
                if get_recording_detection_summaries_for_cameras(
                    recordings.offset(i_3 as isize),
                    1 as ::core::ffi::c_int,
                    &raw mut fallback_summary,
                ) != 0 as ::core::ffi::c_int
                {
                    free(detection_summaries as *mut ::core::ffi::c_void);
                    free(recording_tag_lists as *mut ::core::ffi::c_void);
                    free(recordings as *mut ::core::ffi::c_void);
                    free(all_stream_cfgs as *mut ::core::ffi::c_void);
                    cJSON_Delete(recording);
                    cJSON_Delete(response);
                    http_response_set_json_error(
                        res,
                        500 as ::core::ffi::c_int,
                        b"Failed to get recording detections\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return;
                }
                label_count = fallback_summary.label_count;
                has_detection_flag = has_detection_flag as ::core::ffi::c_int != 0
                    || fallback_summary.has_detection as ::core::ffi::c_int != 0;
            }
            cJSON_AddBoolToObject(
                recording,
                b"has_detection\0".as_ptr() as *const ::core::ffi::c_char,
                has_detection_flag as cJSON_bool,
            );
            cJSON_AddBoolToObject(
                recording,
                b"protected\0".as_ptr() as *const ::core::ffi::c_char,
                (*recordings.offset(i_3 as isize)).protected as cJSON_bool,
            );
            recording_source_add_status(recording, (*recordings.offset(i_3 as isize)).id);
            if label_count > 0 as ::core::ffi::c_int {
                let mut labels_array: *mut cJSON = cJSON_CreateArray();
                if !labels_array.is_null() {
                    let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while j_0 < label_count {
                        let mut label_obj: *mut cJSON = cJSON_CreateObject();
                        if !label_obj.is_null() {
                            cJSON_AddStringToObject(
                                label_obj,
                                b"label\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut (*labels.offset(j_0 as isize)).label
                                    as *mut ::core::ffi::c_char,
                            );
                            cJSON_AddNumberToObject(
                                label_obj,
                                b"count\0".as_ptr() as *const ::core::ffi::c_char,
                                (*labels.offset(j_0 as isize)).count as ::core::ffi::c_double,
                            );
                            cJSON_AddItemToArray(labels_array, label_obj);
                        }
                        j_0 += 1;
                    }
                    cJSON_AddItemToObject(
                        recording,
                        b"detection_labels\0".as_ptr() as *const ::core::ffi::c_char,
                        labels_array,
                    );
                }
            }
            let mut fallback_tag_list: recording_tag_list_t = recording_tag_list_t {
                count: 0 as ::core::ffi::c_int,
                tags: [[0; 128]; 64],
            };
            let mut tag_list: *mut recording_tag_list_t = if !recording_tag_lists.is_null() {
                recording_tag_lists.offset(i_3 as isize)
            } else {
                &raw mut fallback_tag_list
            };
            if recording_tag_lists.is_null() {
                (*tag_list).count = db_recording_tag_get(
                    (*recordings.offset(i_3 as isize)).id,
                    &raw mut (*tag_list).tags as *mut [::core::ffi::c_char; 128],
                    MAX_RECORDING_TAGS,
                );
            }
            let mut tag_count_val: ::core::ffi::c_int = (*tag_list).count;
            if tag_count_val > 0 as ::core::ffi::c_int {
                let mut tags_array: *mut cJSON = cJSON_CreateArray();
                if !tags_array.is_null() {
                    let mut j_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while j_1 < tag_count_val {
                        cJSON_AddItemToArray(
                            tags_array,
                            cJSON_CreateString(
                                &raw mut *(&raw mut (*tag_list).tags
                                    as *mut [::core::ffi::c_char; 128])
                                    .offset(j_1 as isize)
                                    as *mut ::core::ffi::c_char,
                            ),
                        );
                        j_1 += 1;
                    }
                    cJSON_AddItemToObject(
                        recording,
                        b"tags\0".as_ptr() as *const ::core::ffi::c_char,
                        tags_array,
                    );
                }
            } else {
                cJSON_AddItemToObject(
                    recording,
                    b"tags\0".as_ptr() as *const ::core::ffi::c_char,
                    cJSON_CreateArray(),
                );
            }
            cJSON_AddItemToArray(recordings_array, recording);
        }
        i_3 += 1;
    }
    free(detection_summaries as *mut ::core::ffi::c_void);
    free(recording_tag_lists as *mut ::core::ffi::c_void);
    free(recordings as *mut ::core::ffi::c_void);
    if !all_stream_cfgs.is_null() {
        free(all_stream_cfgs as *mut ::core::ffi::c_void);
    }
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert response JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(response);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert response JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/recordings request\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
