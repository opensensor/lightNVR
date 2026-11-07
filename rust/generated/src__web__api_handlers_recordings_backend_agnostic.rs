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
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_HasObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateArray() -> *mut cJSON;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    unsafe fn cJSON_AddItemToObject(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
        item: *mut cJSON,
    ) -> cJSON_bool;
    unsafe fn cJSON_DeleteItemFromObjectCaseSensitive(
        object: *mut cJSON,
        string: *const ::core::ffi::c_char,
    );
    unsafe fn cJSON_Duplicate(item: *const cJSON, recurse: cJSON_bool) -> *mut cJSON;
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
    unsafe fn recording_id_parse_decimal(
        text: *const ::core::ffi::c_char,
        id: *mut uint64_t,
    ) -> bool;
    unsafe fn recording_id_parse_json(item: *const cJSON, id: *mut uint64_t) -> bool;
    unsafe fn authorization_filter_cameras(
        user: *const user_t,
        action: authorization_action_t,
        cameras: *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    unsafe fn batch_delete_progress_create_job_for_principal(
        total: ::core::ffi::c_int,
        owner_user_id: int64_t,
        owner_api_token_uuid: *const ::core::ffi::c_char,
        job_id_out: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn batch_delete_progress_set_total(
        job_id: *const ::core::ffi::c_char,
        total: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn batch_delete_progress_update(
        job_id: *const ::core::ffi::c_char,
        current: ::core::ffi::c_int,
        succeeded: ::core::ffi::c_int,
        failed: ::core::ffi::c_int,
        status_message: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn batch_delete_progress_complete_with_pending(
        job_id: *const ::core::ffi::c_char,
        succeeded: ::core::ffi::c_int,
        failed: ::core::ffi::c_int,
        pending: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn batch_delete_progress_complete(
        job_id: *const ::core::ffi::c_char,
        succeeded: ::core::ffi::c_int,
        failed: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn batch_delete_progress_error(
        job_id: *const ::core::ffi::c_char,
        error_message: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn batch_delete_progress_get(
        job_id: *const ::core::ffi::c_char,
        progress_out: *mut batch_delete_progress_t,
    ) -> ::core::ffi::c_int;
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
    unsafe fn get_recording_metadata_by_id(
        id: uint64_t,
        metadata: *mut recording_metadata_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_recording_detection_summaries_for_cameras(
        recordings: *const recording_metadata_t,
        count: ::core::ffi::c_int,
        summaries: *mut recording_detection_summary_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_load_readonly(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn storage_recording_delete(
        recording_id: uint64_t,
        reason: *const ::core::ffi::c_char,
        local_bytes_removed: *mut uint64_t,
    ) -> ::core::ffi::c_int;
    unsafe fn recording_source_add_status(object: *mut cJSON, recording_id: uint64_t);
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn trim_ascii_whitespace(value: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn delete_recording_thumbnails(recording_id: uint64_t);
    unsafe fn update_stream_storage_cache_remove_recording(
        stream_name: *const ::core::ffi::c_char,
        size_bytes: uint64_t,
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type int64_t = i64;
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const PTHREAD_CREATE_JOINABLE: Self = Self(0);
    pub const PTHREAD_CREATE_DETACHED: Self = Self(1);
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
pub type fleet_selector_t = fleet_selector;
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
pub struct batch_delete_status_t(pub ::core::ffi::c_uint);
impl batch_delete_status_t {
    pub const BATCH_DELETE_STATUS_PENDING: Self = Self(0);
    pub const BATCH_DELETE_STATUS_RUNNING: Self = Self(1);
    pub const BATCH_DELETE_STATUS_COMPLETE: Self = Self(2);
    pub const BATCH_DELETE_STATUS_ERROR: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct batch_delete_progress_t {
    pub job_id: [::core::ffi::c_char; 64],
    pub status: batch_delete_status_t,
    pub total: ::core::ffi::c_int,
    pub current: ::core::ffi::c_int,
    pub succeeded: ::core::ffi::c_int,
    pub pending_deletions: ::core::ffi::c_int,
    pub failed: ::core::ffi::c_int,
    pub status_message: [::core::ffi::c_char; 256],
    pub error_message: [::core::ffi::c_char; 256],
    pub created_at: time_t,
    pub updated_at: time_t,
    pub owner_user_id: int64_t,
    pub owner_api_token_uuid: [::core::ffi::c_char; 37],
    pub is_active: bool,
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
pub struct camera_collection_filter_result_t(pub ::core::ffi::c_int);
impl camera_collection_filter_result_t {
    pub const CAMERA_COLLECTION_FILTER_OK: Self = Self(0);
    pub const CAMERA_COLLECTION_FILTER_NOT_FOUND: Self = Self(-1);
    pub const CAMERA_COLLECTION_FILTER_INVALID_SELECTOR: Self = Self(-2);
    pub const CAMERA_COLLECTION_FILTER_OUT_OF_MEMORY: Self = Self(-3);
    pub const CAMERA_COLLECTION_FILTER_DATABASE_ERROR: Self = Self(-4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camera_collection_filter_t {
    pub active: bool,
    pub smart_selector: *mut fleet_selector_t,
    pub member_uuids: *mut [::core::ffi::c_char; 37],
    pub member_count: ::core::ffi::c_int,
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
pub struct detection_label_summary_t {
    pub label: [::core::ffi::c_char; 32],
    pub count: ::core::ffi::c_int,
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
pub struct batch_delete_thread_data_t {
    pub job_id: [::core::ffi::c_char; 64],
    pub json: *mut cJSON,
    pub preflight_error_count: ::core::ffi::c_int,
    pub start_time: time_t,
    pub end_time: time_t,
    pub camera_scope_count: ::core::ffi::c_int,
    pub camera_scope: [[::core::ffi::c_char; 37]; 1024],
    pub owner_user_id: int64_t,
    pub owner_username: [::core::ffi::c_char; 64],
    pub owner_auth_method: [::core::ffi::c_char; 24],
    pub owner_api_token_uuid: [::core::ffi::c_char; 37],
    pub audit_request_id: [::core::ffi::c_char; 65],
    pub audit_client_ip: [::core::ffi::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub name: *const ::core::ffi::c_char,
    pub max_length: size_t,
    pub numeric: bool,
}
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
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
pub const LOG_COMPONENT: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"RecordingsAPI\0") };
unsafe extern "C" fn audit_recording_delete_operation(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut camera: *const fleet_camera_t,
    mut recording_id: uint64_t,
    mut outcome: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
    mut file_cleanup: *const ::core::ffi::c_char,
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
        if !file_cleanup.is_null() {
            cJSON_AddStringToObject(
                details,
                b"file_cleanup\0".as_ptr() as *const ::core::ffi::c_char,
                file_cleanup,
            );
        }
    }
    audit_log_operation(
        req,
        user,
        b"recording.delete\0".as_ptr() as *const ::core::ffi::c_char,
        b"recording\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut recording_uuid as *mut ::core::ffi::c_char,
        b"delete_recording\0".as_ptr() as *const ::core::ffi::c_char,
        outcome,
        details,
    );
    cJSON_Delete(details);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_recording(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if is_shutdown_initiated() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_DEBUG,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Shutdown in progress, rejecting recording detail request\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            503 as ::core::ffi::c_int,
            b"Service shutting down\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut id_str: [::core::ffi::c_char; 32] = [0; 32];
    if http_request_extract_path_param(
        req,
        b"/api/recordings/\0".as_ptr() as *const ::core::ffi::c_char,
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
    let mut encoded_nul: *const ::core::ffi::c_char = strstr(
        &raw const (*req).path as *const ::core::ffi::c_char,
        b"%00\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut query: *const ::core::ffi::c_char = strchr(
        &raw const (*req).path as *const ::core::ffi::c_char,
        '?' as ::core::ffi::c_int,
    );
    let mut id: uint64_t = 0;
    if !encoded_nul.is_null() && (query.is_null() || encoded_nul < query)
        || !recording_id_parse_decimal(&raw mut id_str as *mut ::core::ffi::c_char, &raw mut id)
    {
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
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/recordings/%llu request\0".as_ptr() as *const ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
    );
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
        authorization_action_t::AUTHZ_RECORDINGS_REPLAY,
        &raw mut recording.camera_uuid as *mut ::core::ffi::c_char,
        &raw mut recording.stream_name as *mut ::core::ffi::c_char,
        &raw mut user,
        &raw mut camera,
        &raw mut evaluation,
    ) == 0
    {
        return;
    }
    let mut recording_obj: *mut cJSON = cJSON_CreateObject();
    if recording_obj.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create recording JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create recording JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let in_progress: bool = !recording.is_complete;
    let effective_end: time_t =
        if in_progress as ::core::ffi::c_int != 0 && recording.end_time <= 0 as time_t {
            time(::core::ptr::null_mut::<time_t>())
        } else {
            recording.end_time
        };
    let mut start_time_str: [::core::ffi::c_char; 32] = [
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
    let mut end_time_str: [::core::ffi::c_char; 32] = [
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
    tm_info = gmtime_r(&raw mut recording.start_time, &raw mut tm_info_buf);
    if !tm_info.is_null() {
        strftime(
            &raw mut start_time_str as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%Y-%m-%dT%H:%M:%SZ\0".as_ptr() as *const ::core::ffi::c_char,
            tm_info,
        );
    }
    if !in_progress {
        tm_info = gmtime_r(&raw mut recording.end_time, &raw mut tm_info_buf);
        if !tm_info.is_null() {
            strftime(
                &raw mut end_time_str as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
                b"%Y-%m-%dT%H:%M:%SZ\0".as_ptr() as *const ::core::ffi::c_char,
                tm_info,
            );
        }
    }
    let mut duration: ::core::ffi::c_int =
        difftime(effective_end, recording.start_time) as ::core::ffi::c_int;
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
    if recording.size_bytes < 1024 as uint64_t {
        snprintf(
            &raw mut size_str as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%lu B\0".as_ptr() as *const ::core::ffi::c_char,
            recording.size_bytes as ::core::ffi::c_ulong,
        );
    } else if recording.size_bytes
        < (1024 as ::core::ffi::c_int as uint64_t).wrapping_mul(1024 as uint64_t)
    {
        snprintf(
            &raw mut size_str as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%.1f KB\0".as_ptr() as *const ::core::ffi::c_char,
            recording.size_bytes as ::core::ffi::c_double / 1024.0f64,
        );
    } else if recording.size_bytes
        < (1024 as ::core::ffi::c_int as uint64_t)
            .wrapping_mul(1024 as uint64_t)
            .wrapping_mul(1024 as uint64_t)
    {
        snprintf(
            &raw mut size_str as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%.1f MB\0".as_ptr() as *const ::core::ffi::c_char,
            recording.size_bytes as ::core::ffi::c_double / (1024.0f64 * 1024.0f64),
        );
    } else {
        snprintf(
            &raw mut size_str as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%.1f GB\0".as_ptr() as *const ::core::ffi::c_char,
            recording.size_bytes as ::core::ffi::c_double / (1024.0f64 * 1024.0f64 * 1024.0f64),
        );
    }
    cJSON_AddNumberToObject(
        recording_obj,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        recording.id as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        recording_obj,
        b"stream\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut recording.stream_name as *mut ::core::ffi::c_char,
    );
    if recording.camera_uuid[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        cJSON_AddStringToObject(
            recording_obj,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut recording.camera_uuid as *mut ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            recording_obj,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddStringToObject(
        recording_obj,
        b"file_path\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut recording.file_path as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        recording_obj,
        b"start_time\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut start_time_str as *mut ::core::ffi::c_char,
    );
    if in_progress {
        cJSON_AddNullToObject(
            recording_obj,
            b"end_time\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddNullToObject(
            recording_obj,
            b"end_time_unix\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddStringToObject(
            recording_obj,
            b"end_time\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut end_time_str as *mut ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            recording_obj,
            b"end_time_unix\0".as_ptr() as *const ::core::ffi::c_char,
            recording.end_time as ::core::ffi::c_double,
        );
    }
    cJSON_AddNumberToObject(
        recording_obj,
        b"start_time_unix\0".as_ptr() as *const ::core::ffi::c_char,
        recording.start_time as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        recording_obj,
        b"in_progress\0".as_ptr() as *const ::core::ffi::c_char,
        in_progress as cJSON_bool,
    );
    cJSON_AddStringToObject(
        recording_obj,
        b"playback_issue\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut recording.playback_issue as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        recording_obj,
        b"duration\0".as_ptr() as *const ::core::ffi::c_char,
        duration as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        recording_obj,
        b"size\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut size_str as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        recording_obj,
        b"capture_method\0".as_ptr() as *const ::core::ffi::c_char,
        recording_capture_method(&raw mut recording),
    );
    if recording.schedule_restricted < 0 as ::core::ffi::c_int {
        cJSON_AddNullToObject(
            recording_obj,
            b"schedule_restricted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddBoolToObject(
            recording_obj,
            b"schedule_restricted\0".as_ptr() as *const ::core::ffi::c_char,
            (recording.schedule_restricted != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
    }
    let mut has_detection_flag: bool = strcmp(
        &raw mut recording.trigger_type as *mut ::core::ffi::c_char,
        b"detection\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    let mut summary: recording_detection_summary_t = recording_detection_summary_t {
        has_detection: false,
        label_count: 0,
        labels: [detection_label_summary_t {
            label: [0; 32],
            count: 0,
        }; 10],
    };
    let mut observed: recording_metadata_t = recording;
    observed.end_time = effective_end;
    if get_recording_detection_summaries_for_cameras(
        &raw mut observed,
        1 as ::core::ffi::c_int,
        &raw mut summary,
    ) != 0 as ::core::ffi::c_int
    {
        cJSON_Delete(recording_obj);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get recording detections\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut labels: *mut detection_label_summary_t =
        &raw mut summary.labels as *mut detection_label_summary_t;
    let mut label_count: ::core::ffi::c_int = summary.label_count;
    has_detection_flag = has_detection_flag as ::core::ffi::c_int != 0
        || summary.has_detection as ::core::ffi::c_int != 0;
    cJSON_AddBoolToObject(
        recording_obj,
        b"has_detection\0".as_ptr() as *const ::core::ffi::c_char,
        has_detection_flag as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        recording_obj,
        b"protected\0".as_ptr() as *const ::core::ffi::c_char,
        recording.protected as cJSON_bool,
    );
    recording_source_add_status(recording_obj, recording.id);
    if label_count > 0 as ::core::ffi::c_int {
        let mut labels_array: *mut cJSON = cJSON_CreateArray();
        if !labels_array.is_null() {
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < label_count {
                let mut label_obj: *mut cJSON = cJSON_CreateObject();
                if !label_obj.is_null() {
                    cJSON_AddStringToObject(
                        label_obj,
                        b"label\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut (*labels.offset(j as isize)).label as *mut ::core::ffi::c_char,
                    );
                    cJSON_AddNumberToObject(
                        label_obj,
                        b"count\0".as_ptr() as *const ::core::ffi::c_char,
                        (*labels.offset(j as isize)).count as ::core::ffi::c_double,
                    );
                    cJSON_AddItemToArray(labels_array, label_obj);
                }
                j += 1;
            }
            cJSON_AddItemToObject(
                recording_obj,
                b"detection_labels\0".as_ptr() as *const ::core::ffi::c_char,
                labels_array,
            );
        }
    }
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(recording_obj);
    if json_str.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to convert recording JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(recording_obj);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to convert recording JSON to string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(recording_obj);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_DEBUG,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/recordings/%llu request\0".as_ptr()
            as *const ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_recording(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut id_str: [::core::ffi::c_char; 32] = [0; 32];
    if http_request_extract_path_param(
        req,
        b"/api/recordings/\0".as_ptr() as *const ::core::ffi::c_char,
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
    let mut encoded_nul: *const ::core::ffi::c_char = strstr(
        &raw const (*req).path as *const ::core::ffi::c_char,
        b"%00\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut query: *const ::core::ffi::c_char = strchr(
        &raw const (*req).path as *const ::core::ffi::c_char,
        '?' as ::core::ffi::c_int,
    );
    let mut id: uint64_t = 0;
    if !encoded_nul.is_null() && (query.is_null() || encoded_nul < query)
        || !recording_id_parse_decimal(&raw mut id_str as *mut ::core::ffi::c_char, &raw mut id)
    {
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
        b"Handling DELETE /api/recordings/%llu request\0".as_ptr() as *const ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
    );
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
    memset(
        &raw mut recording as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<recording_metadata_t>(),
    );
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
        authorization_action_t::AUTHZ_RECORDING_DELETE,
        &raw mut recording.camera_uuid as *mut ::core::ffi::c_char,
        &raw mut recording.stream_name as *mut ::core::ffi::c_char,
        &raw mut user,
        &raw mut camera,
        &raw mut evaluation,
    ) == 0
    {
        return;
    }
    let mut deletion: ::core::ffi::c_int = storage_recording_delete(
        id,
        b"manual deletion\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<uint64_t>(),
    );
    if deletion < 0 as ::core::ffi::c_int {
        audit_recording_delete_operation(
            req,
            &raw mut user,
            &raw mut camera,
            id,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"deletion_blocked\0".as_ptr() as *const ::core::ffi::c_char,
            b"unchanged\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            if deletion == -2 as ::core::ffi::c_int {
                409 as ::core::ffi::c_int
            } else {
                500 as ::core::ffi::c_int
            },
            if deletion == -2 as ::core::ffi::c_int {
                b"Recording is protected, still recording, or has an active transfer\0".as_ptr()
                    as *const ::core::ffi::c_char
            } else {
                b"Failed to queue recording deletion\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    let mut file_cleanup: *const ::core::ffi::c_char = if deletion == 0 as ::core::ffi::c_int {
        b"deleted\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"pending\0".as_ptr() as *const ::core::ffi::c_char
    };
    delete_recording_thumbnails(id);
    if deletion == 0 as ::core::ffi::c_int {
        update_stream_storage_cache_remove_recording(
            &raw mut recording.stream_name as *mut ::core::ffi::c_char,
            recording.size_bytes,
        );
    }
    audit_recording_delete_operation(
        req,
        &raw mut user,
        &raw mut camera,
        id,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        if deletion == 0 as ::core::ffi::c_int {
            b"completed\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"deletion_pending\0".as_ptr() as *const ::core::ffi::c_char
        },
        file_cleanup,
    );
    http_response_set_json(
        res,
        if deletion == 0 as ::core::ffi::c_int {
            200 as ::core::ffi::c_int
        } else {
            202 as ::core::ffi::c_int
        },
        if deletion == 0 as ::core::ffi::c_int {
            b"{\"success\":true,\"message\":\"Recording deleted successfully\"}\0".as_ptr()
                as *const ::core::ffi::c_char
        } else {
            b"{\"success\":true,\"status\":\"deletion_pending\",\"message\":\"Recording deletion queued\"}\0"
                .as_ptr() as *const ::core::ffi::c_char
        },
    );
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Accepted recording deletion: %llu\0".as_ptr() as *const ::core::ffi::c_char,
        id as ::core::ffi::c_ulonglong,
    );
}
unsafe extern "C" fn audit_batch_delete_job(
    mut data: *const batch_delete_thread_data_t,
    mut outcome: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
    mut total_count: ::core::ffi::c_int,
    mut success_count: ::core::ffi::c_int,
    mut error_count: ::core::ffi::c_int,
) {
    if data.is_null() {
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
    user.id = (*data).owner_user_id;
    safe_strcpy(
        &raw mut user.username as *mut ::core::ffi::c_char,
        &raw const (*data).owner_username as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut user.authentication_method as *mut ::core::ffi::c_char,
        &raw const (*data).owner_auth_method as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        0 as size_t,
    );
    if (*data).owner_api_token_uuid[0usize] != 0 {
        user.authenticated_via_scoped_token = r#true != 0;
        safe_strcpy(
            &raw mut user.api_token_uuid as *mut ::core::ffi::c_char,
            &raw const (*data).owner_api_token_uuid as *const ::core::ffi::c_char,
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
        b"/api/recordings/batch-delete\0".as_ptr() as *const ::core::ffi::c_char,
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
        &raw const (*data).audit_client_ip as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut request.request_id as *mut ::core::ffi::c_char,
        &raw const (*data).audit_request_id as *const ::core::ffi::c_char,
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
            b"total_count\0".as_ptr() as *const ::core::ffi::c_char,
            total_count as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            details,
            b"success_count\0".as_ptr() as *const ::core::ffi::c_char,
            success_count as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            details,
            b"error_count\0".as_ptr() as *const ::core::ffi::c_char,
            error_count as ::core::ffi::c_double,
        );
    }
    audit_log_operation(
        &raw mut request,
        &raw mut user,
        b"recording.delete\0".as_ptr() as *const ::core::ffi::c_char,
        b"delete_job\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*data).job_id as *const ::core::ffi::c_char,
        b"batch_delete\0".as_ptr() as *const ::core::ffi::c_char,
        outcome,
        details,
    );
    cJSON_Delete(details);
}
unsafe extern "C" fn batch_delete_worker_thread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut data: *mut batch_delete_thread_data_t = arg as *mut batch_delete_thread_data_t;
    if data.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid thread data\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return NULL;
    }
    log_set_thread_context(
        b"BatchDelete\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let mut job_id: *mut ::core::ffi::c_char = &raw mut (*data).job_id as *mut ::core::ffi::c_char;
    let mut json: *mut cJSON = (*data).json;
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Batch delete worker thread started for job: %s\0".as_ptr() as *const ::core::ffi::c_char,
        job_id,
    );
    let mut ids_array: *mut cJSON =
        cJSON_GetObjectItem(json, b"ids\0".as_ptr() as *const ::core::ffi::c_char);
    let mut filter: *mut cJSON =
        cJSON_GetObjectItem(json, b"filter\0".as_ptr() as *const ::core::ffi::c_char);
    let mut pending_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !ids_array.is_null() && cJSON_IsArray(ids_array) != 0 {
        let mut array_size: ::core::ffi::c_int = cJSON_GetArraySize(ids_array);
        let mut total_count: ::core::ffi::c_int = array_size + (*data).preflight_error_count;
        batch_delete_progress_update(
            job_id,
            (*data).preflight_error_count,
            0 as ::core::ffi::c_int,
            (*data).preflight_error_count,
            b"Starting batch delete operation...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut success_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut error_count: ::core::ffi::c_int = (*data).preflight_error_count;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < array_size {
            let mut id_item: *mut cJSON = cJSON_GetArrayItem(ids_array, i);
            let mut id: uint64_t = 0;
            if !recording_id_parse_json(id_item, &raw mut id) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Invalid ID at index %d\0".as_ptr() as *const ::core::ffi::c_char,
                    i,
                );
                error_count += 1;
            } else {
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
                if get_recording_metadata_by_id(id, &raw mut recording) != 0 as ::core::ffi::c_int {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Recording not found: %llu\0".as_ptr() as *const ::core::ffi::c_char,
                        id as ::core::ffi::c_ulonglong,
                    );
                    error_count += 1;
                } else {
                    let mut deletion: ::core::ffi::c_int = storage_recording_delete(
                        id,
                        b"batch deletion\0".as_ptr() as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<uint64_t>(),
                    );
                    if deletion < 0 as ::core::ffi::c_int {
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_ERROR,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Failed to delete recording from database: %llu\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            id as ::core::ffi::c_ulonglong,
                        );
                        error_count += 1;
                    } else {
                        delete_recording_thumbnails(id);
                        if deletion == 0 as ::core::ffi::c_int {
                            update_stream_storage_cache_remove_recording(
                                &raw mut recording.stream_name as *mut ::core::ffi::c_char,
                                recording.size_bytes,
                            );
                        }
                        success_count += 1;
                        if deletion == 1 as ::core::ffi::c_int {
                            pending_count += 1;
                        }
                        _log_message_ctx(
                            log_level_t::LOG_LEVEL_INFO,
                            LOG_COMPONENT.as_ptr(),
                            _log_stream_name,
                            b"Accepted recording deletion: %llu\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            id as ::core::ffi::c_ulonglong,
                        );
                    }
                }
                if (i + 1 as ::core::ffi::c_int) % 10 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    || i + 1 as ::core::ffi::c_int == array_size
                {
                    let mut status_msg: [::core::ffi::c_char; 256] = [0; 256];
                    snprintf(
                        &raw mut status_msg as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                        b"Deleting recordings... %d/%d\0".as_ptr() as *const ::core::ffi::c_char,
                        i + 1 as ::core::ffi::c_int + (*data).preflight_error_count,
                        total_count,
                    );
                    batch_delete_progress_update(
                        job_id,
                        i + 1 as ::core::ffi::c_int + (*data).preflight_error_count,
                        success_count,
                        error_count,
                        &raw mut status_msg as *mut ::core::ffi::c_char,
                    );
                }
            }
            i += 1;
        }
        batch_delete_progress_complete_with_pending(
            job_id,
            success_count,
            error_count,
            pending_count,
        );
        audit_batch_delete_job(
            data,
            if error_count == 0 as ::core::ffi::c_int {
                b"success\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            if error_count == 0 as ::core::ffi::c_int {
                b"deletions_accepted\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"partial_failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            total_count,
            success_count,
            error_count,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Batch delete job completed: %s (succeeded: %d, failed: %d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            job_id,
            success_count,
            error_count,
        );
    } else if !filter.is_null() && cJSON_IsObject(filter) != 0 {
        let mut start_time: time_t = (*data).start_time;
        let mut end_time: time_t = (*data).end_time;
        let mut has_detection: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
        let mut tag_filter: [::core::ffi::c_char; 512] = [
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
        let mut capture_method_filter: [::core::ffi::c_char; 128] = [
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
        let mut protected_filter: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut detection: *mut cJSON = cJSON_GetObjectItem(
            filter,
            b"detection\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut detection_label_item: *mut cJSON = cJSON_GetObjectItem(
            filter,
            b"detection_label\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut tag_item: *mut cJSON =
            cJSON_GetObjectItem(filter, b"tag\0".as_ptr() as *const ::core::ffi::c_char);
        let mut capture_method_item: *mut cJSON = cJSON_GetObjectItem(
            filter,
            b"capture_method\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut protected_item: *mut cJSON = cJSON_GetObjectItem(
            filter,
            b"protected\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !detection.is_null() && cJSON_IsNumber(detection) != 0 {
            has_detection = (*detection).valueint;
        }
        if !detection_label_item.is_null() && cJSON_IsString(detection_label_item) != 0 {
            safe_strcpy(
                &raw mut detection_label as *mut ::core::ffi::c_char,
                (*detection_label_item).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                0 as size_t,
            );
        }
        if !tag_item.is_null() && cJSON_IsString(tag_item) != 0 {
            safe_strcpy(
                &raw mut tag_filter as *mut ::core::ffi::c_char,
                (*tag_item).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
                0 as size_t,
            );
        }
        if !capture_method_item.is_null() && cJSON_IsString(capture_method_item) != 0 {
            safe_strcpy(
                &raw mut capture_method_filter as *mut ::core::ffi::c_char,
                (*capture_method_item).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                0 as size_t,
            );
        }
        if detection_label[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
            has_detection = 1 as ::core::ffi::c_int;
        }
        if !protected_item.is_null() && cJSON_IsNumber(protected_item) != 0 {
            protected_filter = (*protected_item).valueint;
            if protected_filter < -1 as ::core::ffi::c_int {
                protected_filter = -1 as ::core::ffi::c_int;
            } else if protected_filter > 1 as ::core::ffi::c_int {
                protected_filter = 1 as ::core::ffi::c_int;
            }
            _log_message_ctx(
                log_level_t::LOG_LEVEL_INFO,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Batch delete: protected_filter=%d\0".as_ptr() as *const ::core::ffi::c_char,
                protected_filter,
            );
        }
        let mut camera_scope: [*const ::core::ffi::c_char; 1024] =
            [::core::ptr::null::<::core::ffi::c_char>(); 1024];
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*data).camera_scope_count {
            camera_scope[i_0 as usize] =
                &raw mut *(&raw mut (*data).camera_scope as *mut [::core::ffi::c_char; 37])
                    .offset(i_0 as isize) as *mut ::core::ffi::c_char;
            i_0 += 1;
        }
        let mut total_count_0: ::core::ffi::c_int = get_recording_count_for_cameras(
            start_time,
            end_time,
            ::core::ptr::null::<::core::ffi::c_char>(),
            has_detection,
            if detection_label[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                &raw mut detection_label as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            protected_filter,
            &raw mut camera_scope as *mut *const ::core::ffi::c_char,
            (*data).camera_scope_count,
            if tag_filter[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                &raw mut tag_filter as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            if capture_method_filter[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                &raw mut capture_method_filter as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            r#false != 0,
        );
        if total_count_0 < 0 as ::core::ffi::c_int {
            batch_delete_progress_error(
                job_id,
                b"Failed to count recordings\0".as_ptr() as *const ::core::ffi::c_char,
            );
            audit_batch_delete_job(
                data,
                b"error\0".as_ptr() as *const ::core::ffi::c_char,
                b"query_failed\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            cJSON_Delete(json);
            free(data as *mut ::core::ffi::c_void);
            return NULL;
        }
        if total_count_0 == 0 as ::core::ffi::c_int {
            batch_delete_progress_complete(
                job_id,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            audit_batch_delete_job(
                data,
                b"success\0".as_ptr() as *const ::core::ffi::c_char,
                b"no_matches\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            cJSON_Delete(json);
            free(data as *mut ::core::ffi::c_void);
            return NULL;
        }
        batch_delete_progress_set_total(job_id, total_count_0);
        batch_delete_progress_update(
            job_id,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"Loading recordings to delete...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut recordings: *mut recording_metadata_t = malloc(
            (total_count_0 as size_t).wrapping_mul(::core::mem::size_of::<recording_metadata_t>()),
        ) as *mut recording_metadata_t;
        if recordings.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to allocate memory for recordings\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            batch_delete_progress_error(
                job_id,
                b"Failed to allocate memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            audit_batch_delete_job(
                data,
                b"error\0".as_ptr() as *const ::core::ffi::c_char,
                b"allocation_failed\0".as_ptr() as *const ::core::ffi::c_char,
                total_count_0,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            cJSON_Delete(json);
            free(data as *mut ::core::ffi::c_void);
            return NULL;
        }
        let mut count: ::core::ffi::c_int = get_recording_metadata_paginated_for_cameras(
            start_time,
            end_time,
            ::core::ptr::null::<::core::ffi::c_char>(),
            has_detection,
            if detection_label[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                &raw mut detection_label as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            protected_filter,
            b"id\0".as_ptr() as *const ::core::ffi::c_char,
            b"asc\0".as_ptr() as *const ::core::ffi::c_char,
            recordings,
            total_count_0,
            0 as ::core::ffi::c_int,
            &raw mut camera_scope as *mut *const ::core::ffi::c_char,
            (*data).camera_scope_count,
            if tag_filter[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                &raw mut tag_filter as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            if capture_method_filter[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                &raw mut capture_method_filter as *mut ::core::ffi::c_char
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            },
            r#false != 0,
        );
        if count < 0 as ::core::ffi::c_int {
            free(recordings as *mut ::core::ffi::c_void);
            batch_delete_progress_error(
                job_id,
                b"Failed to load recordings\0".as_ptr() as *const ::core::ffi::c_char,
            );
            audit_batch_delete_job(
                data,
                b"error\0".as_ptr() as *const ::core::ffi::c_char,
                b"query_failed\0".as_ptr() as *const ::core::ffi::c_char,
                total_count_0,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            cJSON_Delete(json);
            free(data as *mut ::core::ffi::c_void);
            return NULL;
        }
        if count == 0 as ::core::ffi::c_int {
            free(recordings as *mut ::core::ffi::c_void);
            batch_delete_progress_complete(
                job_id,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            audit_batch_delete_job(
                data,
                b"success\0".as_ptr() as *const ::core::ffi::c_char,
                b"no_matches\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            cJSON_Delete(json);
            free(data as *mut ::core::ffi::c_void);
            return NULL;
        }
        let mut success_count_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut error_count_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < count {
            let mut id_0: uint64_t = (*recordings.offset(i_1 as isize)).id;
            let mut deletion_0: ::core::ffi::c_int = storage_recording_delete(
                id_0,
                b"batch deletion\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<uint64_t>(),
            );
            if deletion_0 < 0 as ::core::ffi::c_int {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_ERROR,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Failed to delete recording from database: %llu\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    id_0 as ::core::ffi::c_ulonglong,
                );
                error_count_0 += 1;
            } else {
                delete_recording_thumbnails(id_0);
                if deletion_0 == 0 as ::core::ffi::c_int {
                    update_stream_storage_cache_remove_recording(
                        &raw mut (*recordings.offset(i_1 as isize)).stream_name
                            as *mut ::core::ffi::c_char,
                        (*recordings.offset(i_1 as isize)).size_bytes,
                    );
                }
                success_count_0 += 1;
                if deletion_0 == 1 as ::core::ffi::c_int {
                    pending_count += 1;
                }
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_INFO,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Accepted recording deletion: %llu\0".as_ptr() as *const ::core::ffi::c_char,
                    id_0 as ::core::ffi::c_ulonglong,
                );
            }
            if (i_1 + 1 as ::core::ffi::c_int) % 10 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || i_1 + 1 as ::core::ffi::c_int == count
            {
                let mut status_msg_0: [::core::ffi::c_char; 256] = [0; 256];
                snprintf(
                    &raw mut status_msg_0 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
                    b"Deleting recordings... %d/%d\0".as_ptr() as *const ::core::ffi::c_char,
                    i_1 + 1 as ::core::ffi::c_int,
                    count,
                );
                batch_delete_progress_update(
                    job_id,
                    i_1 + 1 as ::core::ffi::c_int,
                    success_count_0,
                    error_count_0,
                    &raw mut status_msg_0 as *mut ::core::ffi::c_char,
                );
            }
            i_1 += 1;
        }
        free(recordings as *mut ::core::ffi::c_void);
        batch_delete_progress_complete_with_pending(
            job_id,
            success_count_0,
            error_count_0,
            pending_count,
        );
        audit_batch_delete_job(
            data,
            if error_count_0 == 0 as ::core::ffi::c_int {
                b"success\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            if error_count_0 == 0 as ::core::ffi::c_int {
                b"deletions_accepted\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"partial_failure\0".as_ptr() as *const ::core::ffi::c_char
            },
            count,
            success_count_0,
            error_count_0,
        );
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Batch delete job completed: %s (succeeded: %d, failed: %d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            job_id,
            success_count_0,
            error_count_0,
        );
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Invalid request format\0".as_ptr() as *const ::core::ffi::c_char,
        );
        batch_delete_progress_error(
            job_id,
            b"Invalid request format\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_batch_delete_job(
            data,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
    cJSON_Delete(json);
    free(data as *mut ::core::ffi::c_void);
    return NULL;
}
unsafe extern "C" fn authorize_batch_delete_ids(
    mut user: *const user_t,
    mut json: *mut cJSON,
    mut failure_count: *mut ::core::ffi::c_int,
    mut res: *mut http_response_t,
) -> ::core::ffi::c_int {
    let mut ids: *mut cJSON =
        cJSON_GetObjectItemCaseSensitive(json, b"ids\0".as_ptr() as *const ::core::ffi::c_char);
    let mut authorized: *mut cJSON = cJSON_CreateArray();
    if authorized.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to authorize batch\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    *failure_count = 0 as ::core::ffi::c_int;
    let mut item: *mut cJSON = ::core::ptr::null_mut::<cJSON>();
    item = (if !ids.is_null() {
        (*ids).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    }) as *mut cJSON;
    while !item.is_null() {
        let mut id: uint64_t = 0;
        if !recording_id_parse_json(item, &raw mut id) {
            *failure_count += 1;
        } else {
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
            if get_recording_metadata_by_id(id, &raw mut recording) != 0 as ::core::ffi::c_int {
                *failure_count += 1;
            } else {
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
                    user,
                    authorization_action_t::AUTHZ_RECORDING_DELETE,
                    &raw mut recording.camera_uuid as *mut ::core::ffi::c_char,
                    &raw mut recording.stream_name as *mut ::core::ffi::c_char,
                    &raw mut evaluation,
                );
                if result < 0 as ::core::ffi::c_int {
                    cJSON_Delete(authorized);
                    http_response_set_json_error(
                        res,
                        500 as ::core::ffi::c_int,
                        b"Authorization policy evaluation failed\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return 0 as ::core::ffi::c_int;
                }
                if result > 0 as ::core::ffi::c_int
                    || evaluation.decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0
                {
                    cJSON_Delete(authorized);
                    http_response_set_json_error(
                        res,
                        403 as ::core::ffi::c_int,
                        b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return 0 as ::core::ffi::c_int;
                }
                let mut copy: *mut cJSON = cJSON_Duplicate(item, r#true);
                if copy.is_null() || cJSON_AddItemToArray(authorized, copy) == 0 {
                    cJSON_Delete(copy);
                    cJSON_Delete(authorized);
                    http_response_set_json_error(
                        res,
                        500 as ::core::ffi::c_int,
                        b"Failed to authorize batch\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
        item = (*item).next as *mut cJSON;
    }
    cJSON_DeleteItemFromObjectCaseSensitive(json, b"ids\0".as_ptr() as *const ::core::ffi::c_char);
    cJSON_AddItemToObject(
        json,
        b"ids\0".as_ptr() as *const ::core::ffi::c_char,
        authorized,
    );
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_delete_time_bound(
    mut item: *const cJSON,
    mut value: *mut time_t,
) -> bool {
    *value = 0 as time_t;
    if item.is_null() {
        return r#true != 0;
    }
    if cJSON_IsString(item) == 0 || (*item).valuestring.is_null() {
        return r#false != 0;
    }
    let mut text: *const ::core::ffi::c_char = (*item).valuestring;
    let mut length: size_t = strlen(text);
    if length < 19 as size_t || length > 64 as size_t {
        return r#false != 0;
    }
    let mut i: size_t = 0 as size_t;
    while i < 19 as size_t {
        let mut separator: ::core::ffi::c_char = (if i == 4 as size_t || i == 7 as size_t {
            '-' as ::core::ffi::c_int
        } else if i == 10 as size_t {
            'T' as ::core::ffi::c_int
        } else if i == 13 as size_t || i == 16 as size_t {
            ':' as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_char;
        if if separator as ::core::ffi::c_int != 0 {
            (*text.offset(i as isize) as ::core::ffi::c_int != separator as ::core::ffi::c_int)
                as ::core::ffi::c_int
        } else {
            ((*text.offset(i as isize) as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
                || *text.offset(i as isize) as ::core::ffi::c_int > '9' as ::core::ffi::c_int)
                as ::core::ffi::c_int
        } != 0
        {
            return r#false != 0;
        }
        i = i.wrapping_add(1);
    }
    let mut parsed: tm = tm {
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
    let mut end: *mut ::core::ffi::c_char = strptime(
        text,
        b"%Y-%m-%dT%H:%M:%S\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut parsed,
    );
    if end.is_null() {
        return r#false != 0;
    }
    if *end as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
        end = end.offset(1);
        if (*end as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || *end as ::core::ffi::c_int > '9' as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        while *end as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *end as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            end = end.offset(1);
        }
    }
    if *end as ::core::ffi::c_int == 'Z' as ::core::ffi::c_int {
        end = end.offset(1);
    }
    if *end != 0 {
        return r#false != 0;
    }
    let mut original: tm = parsed;
    let mut result: time_t = timegm(&raw mut parsed);
    let mut normalized: tm = tm {
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
    if result <= 0 as time_t
        || gmtime_r(&raw mut result, &raw mut normalized).is_null()
        || normalized.tm_year != original.tm_year
        || normalized.tm_mon != original.tm_mon
        || normalized.tm_mday != original.tm_mday
        || normalized.tm_hour != original.tm_hour
        || normalized.tm_min != original.tm_min
        || normalized.tm_sec != original.tm_sec
    {
        return r#false != 0;
    }
    *value = result;
    return r#true != 0;
}
unsafe extern "C" fn validate_batch_delete_filter(mut filter: *const cJSON) -> bool {
    static mut fields: [C2Rust_Unnamed_1; 10] = [
        C2Rust_Unnamed_1 {
            name: b"start\0".as_ptr() as *const ::core::ffi::c_char,
            max_length: 64 as size_t,
            numeric: r#false != 0,
        },
        C2Rust_Unnamed_1 {
            name: b"end\0".as_ptr() as *const ::core::ffi::c_char,
            max_length: 64 as size_t,
            numeric: r#false != 0,
        },
        C2Rust_Unnamed_1 {
            name: b"stream\0".as_ptr() as *const ::core::ffi::c_char,
            max_length: 255 as size_t,
            numeric: r#false != 0,
        },
        C2Rust_Unnamed_1 {
            name: b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
            max_length: 255 as size_t,
            numeric: r#false != 0,
        },
        C2Rust_Unnamed_1 {
            name: b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            max_length: (CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int) as size_t,
            numeric: r#false != 0,
        },
        C2Rust_Unnamed_1 {
            name: b"detection\0".as_ptr() as *const ::core::ffi::c_char,
            max_length: 0 as size_t,
            numeric: r#true != 0,
        },
        C2Rust_Unnamed_1 {
            name: b"protected\0".as_ptr() as *const ::core::ffi::c_char,
            max_length: 0 as size_t,
            numeric: r#true != 0,
        },
        C2Rust_Unnamed_1 {
            name: b"detection_label\0".as_ptr() as *const ::core::ffi::c_char,
            max_length: 255 as size_t,
            numeric: r#false != 0,
        },
        C2Rust_Unnamed_1 {
            name: b"tag\0".as_ptr() as *const ::core::ffi::c_char,
            max_length: 511 as size_t,
            numeric: r#false != 0,
        },
        C2Rust_Unnamed_1 {
            name: b"capture_method\0".as_ptr() as *const ::core::ffi::c_char,
            max_length: 127 as size_t,
            numeric: r#false != 0,
        },
    ];
    let mut item: *const cJSON = ::core::ptr::null::<cJSON>();
    item = if !filter.is_null() {
        (*filter).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !item.is_null() {
        if (*item).string.is_null() {
            return r#false != 0;
        }
        let mut field: size_t = 0 as size_t;
        while field
            < ::core::mem::size_of::<[C2Rust_Unnamed_1; 10]>()
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_1>())
        {
            if if field < 2 as size_t {
                strcasecmp((*item).string, fields[field].name)
            } else {
                strcmp((*item).string, fields[field].name)
            } == 0 as ::core::ffi::c_int
            {
                break;
            }
            field = field.wrapping_add(1);
        }
        if field
            == ::core::mem::size_of::<[C2Rust_Unnamed_1; 10]>()
                .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_1>())
        {
            return r#false != 0;
        }
        let mut prior: *const cJSON = (*filter).child;
        while prior != item {
            if strcasecmp((*prior).string, (*item).string) == 0 as ::core::ffi::c_int {
                return r#false != 0;
            }
            prior = (*prior).next;
        }
        if fields[field].numeric {
            if cJSON_IsNumber(item) == 0
                || (*item).valuedouble != -1 as ::core::ffi::c_int as ::core::ffi::c_double
                    && (*item).valuedouble != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    && (*item).valuedouble != 1 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                return r#false != 0;
            }
        } else {
            if cJSON_IsString(item) == 0
                || (*item).valuestring.is_null()
                || strlen((*item).valuestring) > fields[field].max_length
            {
                return r#false != 0;
            }
            let mut has_value: bool = r#false != 0;
            let mut p: *const ::core::ffi::c_uchar =
                (*item).valuestring as *const ::core::ffi::c_uchar;
            while *p != 0 {
                if *p as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                    && *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                            as ::core::ffi::c_int
                        == 0
                {
                    has_value = r#true != 0;
                }
                p = p.offset(1);
            }
            if !has_value {
                return r#false != 0;
            }
        }
        item = (*item).next;
    }
    return !(cJSON_HasObjectItem(filter, b"stream\0".as_ptr() as *const ::core::ffi::c_char) != 0
        && cJSON_HasObjectItem(
            filter,
            b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0);
}
unsafe extern "C" fn authorize_batch_delete_filter(
    mut user: *const user_t,
    mut filter: *const cJSON,
    mut res: *mut http_response_t,
    mut camera_scope: *mut [::core::ffi::c_char; 37],
    mut scope_count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *scope_count = 0 as ::core::ffi::c_int;
    let mut stream: *mut cJSON = cJSON_GetObjectItemCaseSensitive(
        filter,
        b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if stream.is_null() {
        stream = cJSON_GetObjectItemCaseSensitive(
            filter,
            b"stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut has_stream: bool = cJSON_IsString(stream) != 0
        && *(*stream).valuestring.offset(0isize) as ::core::ffi::c_int != 0;
    if !has_stream {
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
        if authorization_evaluate(
            user,
            authorization_action_t::AUTHZ_RECORDING_DELETE,
            ::core::ptr::null::<fleet_camera_t>(),
            &raw mut evaluation,
        ) != 0 as ::core::ffi::c_int
        {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
        if evaluation.decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0 {
            http_response_set_json_error(
                res,
                403 as ::core::ffi::c_int,
                b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
    }
    let mut names: [::core::ffi::c_char; 256] = [
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
    if has_stream as ::core::ffi::c_int != 0
        && strlen((*stream).valuestring) >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Stream filter is too long\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if has_stream {
        safe_strcpy(
            &raw mut names as *mut ::core::ffi::c_char,
            (*stream).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
    }
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if db_fleet_camera_load_readonly(&raw mut cameras, &raw mut count) != 0 as ::core::ffi::c_int
        || count > MAX_STREAMS
        || authorization_filter_cameras(
            user,
            authorization_action_t::AUTHZ_RECORDING_DELETE,
            cameras,
            &raw mut count,
        ) != 0 as ::core::ffi::c_int
    {
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    let mut selected: [bool; 1024] = [
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
    if has_stream {
        let mut saveptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut any_name: bool = r#false != 0;
        let mut name: *mut ::core::ffi::c_char = strtok_r(
            &raw mut names as *mut ::core::ffi::c_char,
            b",\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut saveptr,
        );
        while !name.is_null() {
            name = trim_ascii_whitespace(name);
            let mut found: bool = r#false != 0;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < count {
                if strcmp(
                    name,
                    &raw mut (*cameras.offset(i as isize)).name as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    selected[i as usize] = r#true != 0;
                    found = r#true != 0;
                }
                i += 1;
            }
            if !found {
                free(cameras as *mut ::core::ffi::c_void);
                http_response_set_json_error(
                    res,
                    403 as ::core::ffi::c_int,
                    b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return 0 as ::core::ffi::c_int;
            }
            any_name = r#true != 0;
            name = strtok_r(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                b",\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut saveptr,
            );
        }
        if !any_name {
            free(cameras as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid stream filter\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
    } else {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < count {
            selected[i_0 as usize] = r#true != 0;
            i_0 += 1;
        }
    }
    let mut collection: camera_collection_filter_t = camera_collection_filter_t {
        active: false,
        smart_selector: ::core::ptr::null_mut::<fleet_selector_t>(),
        member_uuids: ::core::ptr::null_mut::<[::core::ffi::c_char; 37]>(),
        member_count: 0,
    };
    let mut collection_item: *mut cJSON = cJSON_GetObjectItemCaseSensitive(
        filter,
        b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut has_collection: bool = cJSON_IsString(collection_item) != 0
        && *(*collection_item).valuestring.offset(0isize) as ::core::ffi::c_int != 0;
    if has_collection {
        let mut result: camera_collection_filter_result_t = camera_collection_filter_load(
            (*collection_item).valuestring,
            user,
            &raw mut collection,
        );
        if result.0 != camera_collection_filter_result_t::CAMERA_COLLECTION_FILTER_OK.0 {
            free(cameras as *mut ::core::ffi::c_void);
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
            return 0 as ::core::ffi::c_int;
        }
    }
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < count {
        if selected[i_1 as usize] as ::core::ffi::c_int != 0
            && (!has_collection
                || camera_collection_filter_matches(
                    &raw mut collection,
                    cameras.offset(i_1 as isize),
                ) as ::core::ffi::c_int
                    != 0)
        {
            let c2rust_fresh0 = *scope_count;
            *scope_count += 1;
            safe_strcpy(
                &raw mut *camera_scope.offset(c2rust_fresh0 as isize) as *mut ::core::ffi::c_char,
                &raw mut (*cameras.offset(i_1 as isize)).camera_uuid as *mut ::core::ffi::c_char,
                CAMERA_UUID_STRING_SIZE as size_t,
                0 as size_t,
            );
        }
        i_1 += 1;
    }
    camera_collection_filter_free(&raw mut collection);
    free(cameras as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_batch_delete_recordings(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/recordings/batch-delete request\0".as_ptr()
            as *const ::core::ffi::c_char,
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
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to parse JSON body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut ids_array: *mut cJSON =
        cJSON_GetObjectItem(json, b"ids\0".as_ptr() as *const ::core::ffi::c_char);
    let mut filter: *mut cJSON =
        cJSON_GetObjectItem(json, b"filter\0".as_ptr() as *const ::core::ffi::c_char);
    let mut total_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut preflight_error_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut start_time: time_t = 0 as time_t;
    let mut end_time: time_t = 0 as time_t;
    let mut camera_scope: [[::core::ffi::c_char; 37]; 1024] = [
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
        ],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
        [0; 37],
    ];
    let mut camera_scope_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !ids_array.is_null() && cJSON_IsArray(ids_array) != 0 {
        total_count = cJSON_GetArraySize(ids_array);
        if total_count == 0 as ::core::ffi::c_int {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Empty 'ids' array in batch delete request\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            cJSON_Delete(json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Empty 'ids' array\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        if authorize_batch_delete_ids(&raw mut user, json, &raw mut preflight_error_count, res) == 0
        {
            cJSON_Delete(json);
            return;
        }
    } else if !filter.is_null() && cJSON_IsObject(filter) != 0 {
        total_count = 0 as ::core::ffi::c_int;
        if !validate_batch_delete_filter(filter) {
            cJSON_Delete(json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Unknown, duplicate, or invalid deletion filter field\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        if authorize_batch_delete_filter(
            &raw mut user,
            filter,
            res,
            &raw mut camera_scope as *mut [::core::ffi::c_char; 37],
            &raw mut camera_scope_count,
        ) == 0
        {
            cJSON_Delete(json);
            return;
        }
        if !parse_delete_time_bound(
            cJSON_GetObjectItem(filter, b"start\0".as_ptr() as *const ::core::ffi::c_char),
            &raw mut start_time,
        ) || !parse_delete_time_bound(
            cJSON_GetObjectItem(filter, b"end\0".as_ptr() as *const ::core::ffi::c_char),
            &raw mut end_time,
        ) || start_time != 0 && end_time != 0 && start_time > end_time
        {
            cJSON_Delete(json);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid deletion date range\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Request must contain either 'ids' array or 'filter' object\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Request must contain either 'ids' array or 'filter' object\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut job_id: [::core::ffi::c_char; 64] = [0; 64];
    let mut owner_token_uuid: *const ::core::ffi::c_char =
        if user.authenticated_via_scoped_token as ::core::ffi::c_int != 0 {
            &raw mut user.api_token_uuid as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        };
    if batch_delete_progress_create_job_for_principal(
        total_count,
        user.id,
        owner_token_uuid,
        &raw mut job_id as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create batch delete job\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create batch delete job\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Created batch delete job: %s (total: %d)\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut job_id as *mut ::core::ffi::c_char,
        total_count,
    );
    let mut thread_data: *mut batch_delete_thread_data_t =
        malloc(::core::mem::size_of::<batch_delete_thread_data_t>())
            as *mut batch_delete_thread_data_t;
    if thread_data.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to allocate memory for thread data\0".as_ptr() as *const ::core::ffi::c_char,
        );
        batch_delete_progress_error(
            &raw mut job_id as *mut ::core::ffi::c_char,
            b"Failed to allocate memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to allocate memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    memset(
        thread_data as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<batch_delete_thread_data_t>(),
    );
    safe_strcpy(
        &raw mut (*thread_data).job_id as *mut ::core::ffi::c_char,
        &raw mut job_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    (*thread_data).json = json;
    (*thread_data).preflight_error_count = preflight_error_count;
    (*thread_data).start_time = start_time;
    (*thread_data).end_time = end_time;
    (*thread_data).camera_scope_count = camera_scope_count;
    memcpy(
        &raw mut (*thread_data).camera_scope as *mut [::core::ffi::c_char; 37]
            as *mut ::core::ffi::c_void,
        &raw mut camera_scope as *mut [::core::ffi::c_char; 37] as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[[::core::ffi::c_char; 37]; 1024]>(),
    );
    (*thread_data).owner_user_id = user.id;
    safe_strcpy(
        &raw mut (*thread_data).owner_username as *mut ::core::ffi::c_char,
        &raw mut user.username as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*thread_data).owner_auth_method as *mut ::core::ffi::c_char,
        &raw mut user.authentication_method as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        0 as size_t,
    );
    if user.authenticated_via_scoped_token {
        safe_strcpy(
            &raw mut (*thread_data).owner_api_token_uuid as *mut ::core::ffi::c_char,
            &raw mut user.api_token_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
    }
    safe_strcpy(
        &raw mut (*thread_data).audit_request_id as *mut ::core::ffi::c_char,
        &raw const (*req).request_id as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>(),
        0 as size_t,
    );
    if httpd_get_effective_client_ip(
        req,
        &raw mut (*thread_data).audit_client_ip as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) != 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            &raw mut (*thread_data).audit_client_ip as *mut ::core::ffi::c_char,
            &raw const (*req).client_ip as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
            0 as size_t,
        );
    }
    let mut thread: pthread_t = 0;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    pthread_attr_init(&raw mut attr);
    pthread_attr_setdetachstate(
        &raw mut attr,
        C2Rust_Unnamed_0::PTHREAD_CREATE_DETACHED.0 as ::core::ffi::c_int,
    );
    if pthread_create(
        &raw mut thread,
        &raw mut attr,
        Some(
            batch_delete_worker_thread
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        thread_data as *mut ::core::ffi::c_void,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create worker thread\0".as_ptr() as *const ::core::ffi::c_char,
        );
        batch_delete_progress_error(
            &raw mut job_id as *mut ::core::ffi::c_char,
            b"Failed to create worker thread\0".as_ptr() as *const ::core::ffi::c_char,
        );
        audit_batch_delete_job(
            thread_data,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"worker_start_failed\0".as_ptr() as *const ::core::ffi::c_char,
            total_count,
            0 as ::core::ffi::c_int,
            preflight_error_count,
        );
        cJSON_Delete(json);
        free(thread_data as *mut ::core::ffi::c_void);
        pthread_attr_destroy(&raw mut attr);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create worker thread\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    pthread_attr_destroy(&raw mut attr);
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddStringToObject(
        response,
        b"job_id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut job_id as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        response,
        b"status\0".as_ptr() as *const ::core::ffi::c_char,
        b"started\0".as_ptr() as *const ::core::ffi::c_char,
    );
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
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 202 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Batch delete job started: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut job_id as *mut ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_batch_delete_progress(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/recordings/batch-delete/progress request\0".as_ptr()
            as *const ::core::ffi::c_char,
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
    let mut job_id: [::core::ffi::c_char; 64] = [
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
        b"/api/recordings/batch-delete/progress/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut job_id as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to extract job ID from URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing or invalid job ID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Getting progress for job: %s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut job_id as *mut ::core::ffi::c_char,
    );
    let mut progress: batch_delete_progress_t = batch_delete_progress_t {
        job_id: [0; 64],
        status: batch_delete_status_t::BATCH_DELETE_STATUS_PENDING,
        total: 0,
        current: 0,
        succeeded: 0,
        pending_deletions: 0,
        failed: 0,
        status_message: [0; 256],
        error_message: [0; 256],
        created_at: 0,
        updated_at: 0,
        owner_user_id: 0,
        owner_api_token_uuid: [0; 37],
        is_active: false,
    };
    if batch_delete_progress_get(
        &raw mut job_id as *mut ::core::ffi::c_char,
        &raw mut progress,
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Job not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut job_id as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Job not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut request_token_uuid: *const ::core::ffi::c_char =
        if user.authenticated_via_scoped_token as ::core::ffi::c_int != 0 {
            &raw mut user.api_token_uuid as *mut ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        };
    if progress.owner_user_id != user.id
        || strcmp(
            &raw mut progress.owner_api_token_uuid as *mut ::core::ffi::c_char,
            request_token_uuid,
        ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Job not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    if response.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddStringToObject(
        response,
        b"job_id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut progress.job_id as *mut ::core::ffi::c_char,
    );
    let mut status_str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    match progress.status {
        batch_delete_status_t::BATCH_DELETE_STATUS_PENDING => {
            status_str = b"pending\0".as_ptr() as *const ::core::ffi::c_char;
        }
        batch_delete_status_t::BATCH_DELETE_STATUS_RUNNING => {
            status_str = b"running\0".as_ptr() as *const ::core::ffi::c_char;
        }
        batch_delete_status_t::BATCH_DELETE_STATUS_COMPLETE => {
            status_str = b"complete\0".as_ptr() as *const ::core::ffi::c_char;
        }
        batch_delete_status_t::BATCH_DELETE_STATUS_ERROR => {
            status_str = b"error\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {
            status_str = b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    cJSON_AddStringToObject(
        response,
        b"status\0".as_ptr() as *const ::core::ffi::c_char,
        status_str,
    );
    cJSON_AddNumberToObject(
        response,
        b"total\0".as_ptr() as *const ::core::ffi::c_char,
        progress.total as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        response,
        b"current\0".as_ptr() as *const ::core::ffi::c_char,
        progress.current as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        response,
        b"succeeded\0".as_ptr() as *const ::core::ffi::c_char,
        progress.succeeded as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        response,
        b"pending_deletions\0".as_ptr() as *const ::core::ffi::c_char,
        progress.pending_deletions as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        response,
        b"failed\0".as_ptr() as *const ::core::ffi::c_char,
        progress.failed as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        response,
        b"status_message\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut progress.status_message as *mut ::core::ffi::c_char,
    );
    if progress.error_message[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        cJSON_AddStringToObject(
            response,
            b"error_message\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut progress.error_message as *mut ::core::ffi::c_char,
        );
    }
    cJSON_AddBoolToObject(
        response,
        b"complete\0".as_ptr() as *const ::core::ffi::c_char,
        (progress.status.0 == batch_delete_status_t::BATCH_DELETE_STATUS_COMPLETE.0
            || progress.status.0 == batch_delete_status_t::BATCH_DELETE_STATUS_ERROR.0)
            as ::core::ffi::c_int,
    );
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
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
    cJSON_Delete(response);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
