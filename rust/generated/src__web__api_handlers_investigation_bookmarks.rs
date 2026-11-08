unsafe extern "C" {
    pub type authorization_context;
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsNull(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
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
    unsafe fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn authorization_context_create() -> *mut authorization_context_t;
    unsafe fn authorization_context_free(context: *mut authorization_context_t);
    unsafe fn authorization_evaluate_in_context(
        context: *mut authorization_context_t,
        user: *const user_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_find_by_name(
        stream_name: *const ::core::ffi::c_char,
        camera: *mut fleet_camera_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_investigation_bookmark_list(
        owner_user_id: int64_t,
        offset: ::core::ffi::c_int,
        bookmarks: *mut investigation_bookmark_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_investigation_bookmark_get(
        owner_user_id: int64_t,
        uuid: *const ::core::ffi::c_char,
        bookmark: *mut investigation_bookmark_t,
    ) -> db_investigation_bookmark_result_t;
    unsafe fn db_investigation_bookmark_list_cameras(
        bookmark_uuid: *const ::core::ffi::c_char,
        camera_uuids: *mut [::core::ffi::c_char; 37],
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_investigation_bookmark_create(
        bookmark: *mut investigation_bookmark_t,
        camera_uuids: *const [::core::ffi::c_char; 37],
        camera_count: ::core::ffi::c_int,
    ) -> db_investigation_bookmark_result_t;
    unsafe fn db_investigation_bookmark_update_metadata(
        bookmark: *mut investigation_bookmark_t,
        expected_revision: int64_t,
    ) -> db_investigation_bookmark_result_t;
    unsafe fn db_investigation_bookmark_delete(
        owner_user_id: int64_t,
        uuid: *const ::core::ffi::c_char,
        expected_revision: int64_t,
    ) -> db_investigation_bookmark_result_t;
    unsafe fn get_stream_config_by_uuid(
        camera_uuid: *const ::core::ffi::c_char,
        stream: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
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
    unsafe fn audit_log_authorization(
        req: *const http_request_t,
        user: *const user_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        evaluation: *const authorization_evaluation_t,
        outcome: *const ::core::ffi::c_char,
    );
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
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
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
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
pub type uint8_t = u8;
pub type uint64_t = u64;
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
pub type authorization_context_t = authorization_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct investigation_bookmark_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub owner_user_id: int64_t,
    pub title: [::core::ffi::c_char; 128],
    pub note: [::core::ffi::c_char; 2048],
    pub start_time: int64_t,
    pub end_time: int64_t,
    pub cursor_time: int64_t,
    pub primary_camera_uuid: [::core::ffi::c_char; 37],
    pub filters_json: [::core::ffi::c_char; 4096],
    pub representative_result_json: [::core::ffi::c_char; 2048],
    pub camera_count: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_investigation_bookmark_result_t(pub ::core::ffi::c_int);
impl db_investigation_bookmark_result_t {
    pub const DB_INVESTIGATION_BOOKMARK_OK: Self = Self(0);
    pub const DB_INVESTIGATION_BOOKMARK_NOT_FOUND: Self = Self(-1);
    pub const DB_INVESTIGATION_BOOKMARK_INVALID: Self = Self(-2);
    pub const DB_INVESTIGATION_BOOKMARK_STALE: Self = Self(-3);
    pub const DB_INVESTIGATION_BOOKMARK_LIMIT: Self = Self(-4);
    pub const DB_INVESTIGATION_BOOKMARK_ERROR: Self = Self(-5);
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
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const INVESTIGATION_BOOKMARK_MAX_CAMERAS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const INVESTIGATION_BOOKMARK_MAX_PER_OWNER: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const INVESTIGATION_BOOKMARK_LIST_BATCH: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const BOOKMARK_AUDIT_ACTION: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"investigation.bookmark\0")
};
pub const BOOKMARK_TARGET_TYPE: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"investigation_bookmark\0")
};
unsafe extern "C" fn authenticate(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut user: *mut user_t,
) -> bool {
    memset(
        user as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    if httpd_check_action_access(req, user) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn demo_read_only(mut user: *const user_t) -> bool {
    return !user.is_null()
        && strcmp(
            &raw const (*user).authentication_method as *const ::core::ffi::c_char,
            b"demo\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_int64(
    mut body: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut value: *mut int64_t,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, key);
    if cJSON_IsNumber(item) == 0
        || (*item).valuedouble.is_finite() as ::core::ffi::c_int == 0
        || floor((*item).valuedouble) != (*item).valuedouble
        || (*item).valuedouble < 1 as ::core::ffi::c_int as ::core::ffi::c_double
        || (*item).valuedouble > INT64_MAX as ::core::ffi::c_double
    {
        return r#false != 0;
    }
    *value = (*item).valuedouble as int64_t;
    return r#true != 0;
}
unsafe extern "C" fn copy_required_string(
    mut body: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut destination: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, key);
    if cJSON_IsString(item) == 0
        || (*item).valuestring.is_null()
        || *(*item).valuestring.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        || strlen((*item).valuestring) >= size
    {
        return r#false != 0;
    }
    safe_strcpy(destination, (*item).valuestring, size, 0 as size_t);
    return r#true != 0;
}
unsafe extern "C" fn copy_optional_string(
    mut body: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut destination: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, key);
    if item.is_null() {
        return r#true != 0;
    }
    if cJSON_IsString(item) == 0
        || (*item).valuestring.is_null()
        || strlen((*item).valuestring) >= size
    {
        return r#false != 0;
    }
    safe_strcpy(destination, (*item).valuestring, size, 0 as size_t);
    return r#true != 0;
}
unsafe extern "C" fn key_allowed(
    mut key: *const ::core::ffi::c_char,
    mut allowed: *const *const ::core::ffi::c_char,
    mut allowed_count: size_t,
) -> bool {
    let mut i: size_t = 0 as size_t;
    while i < allowed_count {
        if strcmp(key, *allowed.offset(i as isize)) == 0 as ::core::ffi::c_int {
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn valid_filter_value(mut item: *const cJSON) -> bool {
    if strcmp(
        (*item).string,
        b"min_confidence\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return cJSON_IsNumber(item) != 0
            && (*item).valuedouble.is_finite() as ::core::ffi::c_int != 0
            && (*item).valuedouble >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && (*item).valuedouble <= 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if strcmp(
        (*item).string,
        b"region\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if cJSON_IsObject(item) == 0 {
            return r#false != 0;
        }
        static mut region_keys: [*const ::core::ffi::c_char; 7] = [
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            b"x\0".as_ptr() as *const ::core::ffi::c_char,
            b"y\0".as_ptr() as *const ::core::ffi::c_char,
            b"width\0".as_ptr() as *const ::core::ffi::c_char,
            b"height\0".as_ptr() as *const ::core::ffi::c_char,
            b"match\0".as_ptr() as *const ::core::ffi::c_char,
            b"min_intersection\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut part: *const cJSON = ::core::ptr::null::<cJSON>();
        part = if !item.is_null() {
            (*item).child
        } else {
            ::core::ptr::null_mut::<cJSON>()
        };
        while !part.is_null() {
            if (*part).string.is_null()
                || !key_allowed(
                    (*part).string,
                    &raw const region_keys as *const *const ::core::ffi::c_char,
                    ::core::mem::size_of::<[*const ::core::ffi::c_char; 7]>()
                        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
                )
            {
                return r#false != 0;
            }
            if strcmp(
                (*part).string,
                b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                if cJSON_IsString(part) == 0 || !lightnvr_uuid_is_valid((*part).valuestring) {
                    return r#false != 0;
                }
            } else if strcmp(
                (*part).string,
                b"match\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                if cJSON_IsString(part) == 0
                    || strcmp(
                        (*part).valuestring,
                        b"center\0".as_ptr() as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                        && strcmp(
                            (*part).valuestring,
                            b"intersects\0".as_ptr() as *const ::core::ffi::c_char,
                        ) != 0 as ::core::ffi::c_int
                        && strcmp(
                            (*part).valuestring,
                            b"minimum_intersection\0".as_ptr() as *const ::core::ffi::c_char,
                        ) != 0 as ::core::ffi::c_int
                {
                    return r#false != 0;
                }
            } else if cJSON_IsNumber(part) == 0
                || (*part).valuedouble.is_finite() as ::core::ffi::c_int == 0
                || (*part).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
                || (*part).valuedouble > 1 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                return r#false != 0;
            }
            part = (*part).next;
        }
        let mut x: *const cJSON =
            cJSON_GetObjectItemCaseSensitive(item, b"x\0".as_ptr() as *const ::core::ffi::c_char);
        let mut y: *const cJSON =
            cJSON_GetObjectItemCaseSensitive(item, b"y\0".as_ptr() as *const ::core::ffi::c_char);
        let mut camera_uuid: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut width: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"width\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut height: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"height\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut r#match: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"match\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut minimum: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"min_intersection\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if cJSON_IsString(camera_uuid) == 0
            || !lightnvr_uuid_is_valid((*camera_uuid).valuestring)
            || cJSON_IsNumber(x) == 0
            || cJSON_IsNumber(y) == 0
            || cJSON_IsNumber(width) == 0
            || cJSON_IsNumber(height) == 0
            || (*width).valuedouble <= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            || (*height).valuedouble <= 0 as ::core::ffi::c_int as ::core::ffi::c_double
            || (*x).valuedouble + (*width).valuedouble
                > 1 as ::core::ffi::c_int as ::core::ffi::c_double
            || (*y).valuedouble + (*height).valuedouble
                > 1 as ::core::ffi::c_int as ::core::ffi::c_double
            || cJSON_IsString(r#match) != 0
                && strcmp(
                    (*r#match).valuestring,
                    b"minimum_intersection\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                && (cJSON_IsNumber(minimum) == 0
                    || (*minimum).valuedouble <= 0 as ::core::ffi::c_int as ::core::ffi::c_double)
        {
            return r#false != 0;
        }
        return r#true != 0;
    }
    return cJSON_IsString(item) != 0
        && !(*item).valuestring.is_null()
        && strlen((*item).valuestring) <= 256 as size_t;
}
unsafe extern "C" fn canonicalize_filters(
    mut filters: *const cJSON,
    mut destination: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> bool {
    static mut keys: [*const ::core::ffi::c_char; 10] = [
        b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
        b"location\0".as_ptr() as *const ::core::ffi::c_char,
        b"label\0".as_ptr() as *const ::core::ffi::c_char,
        b"zone\0".as_ptr() as *const ::core::ffi::c_char,
        b"source\0".as_ptr() as *const ::core::ffi::c_char,
        b"capture_method\0".as_ptr() as *const ::core::ffi::c_char,
        b"recording_tag\0".as_ptr() as *const ::core::ffi::c_char,
        b"protection\0".as_ptr() as *const ::core::ffi::c_char,
        b"min_confidence\0".as_ptr() as *const ::core::ffi::c_char,
        b"region\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if cJSON_IsObject(filters) == 0 {
        return r#false != 0;
    }
    let mut canonical: *mut cJSON = cJSON_CreateObject();
    if canonical.is_null() {
        return r#false != 0;
    }
    let mut item: *const cJSON = ::core::ptr::null::<cJSON>();
    item = if !filters.is_null() {
        (*filters).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !item.is_null() {
        if (*item).string.is_null()
            || !key_allowed(
                (*item).string,
                &raw const keys as *const *const ::core::ffi::c_char,
                ::core::mem::size_of::<[*const ::core::ffi::c_char; 10]>()
                    .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
            )
            || !valid_filter_value(item)
        {
            cJSON_Delete(canonical);
            return r#false != 0;
        }
        cJSON_AddItemToObject(
            canonical,
            (*item).string,
            cJSON_Duplicate(item, 1 as cJSON_bool),
        );
        item = (*item).next;
    }
    let mut encoded: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(canonical);
    cJSON_Delete(canonical);
    if encoded.is_null() || strlen(encoded) >= size {
        free(encoded as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    safe_strcpy(destination, encoded, size, 0 as size_t);
    free(encoded as *mut ::core::ffi::c_void);
    return r#true != 0;
}
unsafe extern "C" fn canonicalize_result(
    mut result: *const cJSON,
    mut destination: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> bool {
    static mut keys: [*const ::core::ffi::c_char; 8] = [
        b"result_id\0".as_ptr() as *const ::core::ffi::c_char,
        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        b"start_time\0".as_ptr() as *const ::core::ffi::c_char,
        b"end_time\0".as_ptr() as *const ::core::ffi::c_char,
        b"event_type\0".as_ptr() as *const ::core::ffi::c_char,
        b"recording_id\0".as_ptr() as *const ::core::ffi::c_char,
        b"detection_id\0".as_ptr() as *const ::core::ffi::c_char,
        b"label\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if result.is_null() || cJSON_IsNull(result) != 0 {
        *destination.offset(0isize) = '\0' as ::core::ffi::c_char;
        return r#true != 0;
    }
    if cJSON_IsObject(result) == 0 {
        return r#false != 0;
    }
    let mut canonical: *mut cJSON = cJSON_CreateObject();
    if canonical.is_null() {
        return r#false != 0;
    }
    let mut item: *const cJSON = ::core::ptr::null::<cJSON>();
    item = if !result.is_null() {
        (*result).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !item.is_null() {
        if (*item).string.is_null()
            || !key_allowed(
                (*item).string,
                &raw const keys as *const *const ::core::ffi::c_char,
                ::core::mem::size_of::<[*const ::core::ffi::c_char; 8]>()
                    .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>()),
            )
            || !(cJSON_IsString(item) != 0 || cJSON_IsNumber(item) != 0)
        {
            cJSON_Delete(canonical);
            return r#false != 0;
        }
        if strcmp(
            (*item).string,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && (cJSON_IsString(item) == 0 || !lightnvr_uuid_is_valid((*item).valuestring))
        {
            cJSON_Delete(canonical);
            return r#false != 0;
        }
        cJSON_AddItemToObject(
            canonical,
            (*item).string,
            cJSON_Duplicate(item, 1 as cJSON_bool),
        );
        item = (*item).next;
    }
    let mut encoded: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(canonical);
    cJSON_Delete(canonical);
    if encoded.is_null() || strlen(encoded) >= size {
        free(encoded as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    safe_strcpy(destination, encoded, size, 0 as size_t);
    free(encoded as *mut ::core::ffi::c_void);
    return r#true != 0;
}
unsafe extern "C" fn parse_create_body(
    mut body: *const cJSON,
    mut owner_user_id: int64_t,
    mut bookmark: *mut investigation_bookmark_t,
    mut camera_uuids: *mut [::core::ffi::c_char; 37],
    mut camera_count: *mut ::core::ffi::c_int,
    mut res: *mut http_response_t,
) -> bool {
    memset(
        bookmark as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<investigation_bookmark_t>(),
    );
    (*bookmark).owner_user_id = owner_user_id;
    if cJSON_IsObject(body) == 0
        || !copy_required_string(
            body,
            b"title\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*bookmark).title as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        )
        || !copy_optional_string(
            body,
            b"note\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*bookmark).note as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
        )
        || !parse_int64(
            body,
            b"start_time\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*bookmark).start_time,
        )
        || !parse_int64(
            body,
            b"end_time\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*bookmark).end_time,
        )
        || !parse_int64(
            body,
            b"cursor_time\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*bookmark).cursor_time,
        )
        || !copy_required_string(
            body,
            b"primary_camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*bookmark).primary_camera_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        )
        || !lightnvr_uuid_is_valid(
            &raw mut (*bookmark).primary_camera_uuid as *mut ::core::ffi::c_char,
        )
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid bookmark fields\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut cameras: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"camera_uuids\0".as_ptr() as *const ::core::ffi::c_char,
    );
    *camera_count = if cJSON_IsArray(cameras) != 0 {
        cJSON_GetArraySize(cameras)
    } else {
        0 as ::core::ffi::c_int
    };
    if *camera_count < 1 as ::core::ffi::c_int || *camera_count > INVESTIGATION_BOOKMARK_MAX_CAMERAS
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"camera_uuids must contain between 1 and 16 cameras\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < *camera_count {
        let mut item: *const cJSON = cJSON_GetArrayItem(cameras, i);
        if cJSON_IsString(item) == 0 || !lightnvr_uuid_is_valid((*item).valuestring) {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"camera_uuids contains invalid UUID\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        let mut previous: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while previous < i {
            if strcmp(
                &raw mut *camera_uuids.offset(previous as isize) as *mut ::core::ffi::c_char,
                (*item).valuestring,
            ) == 0 as ::core::ffi::c_int
            {
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"camera_uuids contains duplicates\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return r#false != 0;
            }
            previous += 1;
        }
        safe_strcpy(
            &raw mut *camera_uuids.offset(i as isize) as *mut ::core::ffi::c_char,
            (*item).valuestring,
            CAMERA_UUID_STRING_SIZE as size_t,
            0 as size_t,
        );
        i += 1;
    }
    let mut filters: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"filters\0".as_ptr() as *const ::core::ffi::c_char);
    let mut result: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"representative_result\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !canonicalize_filters(
        filters,
        &raw mut (*bookmark).filters_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
    ) || !canonicalize_result(
        result,
        &raw mut (*bookmark).representative_result_json as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
    ) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid bookmark investigation state\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut region: *const cJSON = if cJSON_IsObject(filters) != 0 {
        cJSON_GetObjectItemCaseSensitive(
            filters,
            b"region\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut region_camera: *const cJSON = if cJSON_IsObject(region) != 0 {
        cJSON_GetObjectItemCaseSensitive(
            region,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut result_camera: *const cJSON = if cJSON_IsObject(result) != 0 {
        cJSON_GetObjectItemCaseSensitive(
            result,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if !region_camera.is_null() || !result_camera.is_null() {
        let mut region_found: bool = region_camera.is_null();
        let mut result_found: bool = result_camera.is_null();
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < *camera_count {
            if !region_camera.is_null()
                && cJSON_IsString(region_camera) != 0
                && strcmp(
                    &raw mut *camera_uuids.offset(i_0 as isize) as *mut ::core::ffi::c_char,
                    (*region_camera).valuestring,
                ) == 0 as ::core::ffi::c_int
            {
                region_found = r#true != 0;
            }
            if !result_camera.is_null()
                && cJSON_IsString(result_camera) != 0
                && strcmp(
                    &raw mut *camera_uuids.offset(i_0 as isize) as *mut ::core::ffi::c_char,
                    (*result_camera).valuestring,
                ) == 0 as ::core::ffi::c_int
            {
                result_found = r#true != 0;
            }
            i_0 += 1;
        }
        if !region_found || !result_found {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Saved investigation state references another camera\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
    }
    return r#true != 0;
}
unsafe extern "C" fn authorize_cameras_in_context(
    mut context: *mut authorization_context_t,
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut user: *const user_t,
    mut camera_uuids: *mut [::core::ffi::c_char; 37],
    mut camera_count: ::core::ffi::c_int,
    mut conceal_denial: bool,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < camera_count {
        let mut stream: stream_config_t = stream_config_t {
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
        let mut camera: fleet_camera_t = fleet_camera_t {
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
        if get_stream_config_by_uuid(
            &raw mut *camera_uuids.offset(i as isize) as *mut ::core::ffi::c_char,
            &raw mut stream,
        ) != 0 as ::core::ffi::c_int
            || db_fleet_camera_find_by_name(
                &raw mut stream.name as *mut ::core::ffi::c_char,
                &raw mut camera,
            ) != 0 as ::core::ffi::c_int
        {
            if !res.is_null() && !conceal_denial {
                http_response_set_json_error(
                    res,
                    404 as ::core::ffi::c_int,
                    b"Camera not found\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return 0 as ::core::ffi::c_int;
        }
        let mut evaluation: authorization_evaluation_t = authorization_evaluation_t {
            decision: authorization_decision_t(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        };
        if authorization_evaluate_in_context(
            context,
            user,
            authorization_action_t::AUTHZ_RECORDINGS_REPLAY,
            &raw mut camera,
            &raw mut evaluation,
        ) != 0 as ::core::ffi::c_int
        {
            if !conceal_denial {
                audit_log_authorization(
                    req,
                    user,
                    authorization_action_t::AUTHZ_RECORDINGS_REPLAY,
                    &raw mut camera,
                    ::core::ptr::null::<authorization_evaluation_t>(),
                    b"error\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if !res.is_null() && !conceal_denial {
                http_response_set_json_error(
                    res,
                    500 as ::core::ffi::c_int,
                    b"Authorization policy evaluation failed\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            return -1 as ::core::ffi::c_int;
        }
        if evaluation.decision.0 != authorization_decision_t::AUTHZ_DECISION_ALLOW.0 {
            if !conceal_denial {
                audit_log_authorization(
                    req,
                    user,
                    authorization_action_t::AUTHZ_RECORDINGS_REPLAY,
                    &raw mut camera,
                    &raw mut evaluation,
                    b"denied\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if !res.is_null() && !conceal_denial {
                http_response_set_json_error(
                    res,
                    403 as ::core::ffi::c_int,
                    b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn authorize_cameras(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut user: *const user_t,
    mut camera_uuids: *mut [::core::ffi::c_char; 37],
    mut camera_count: ::core::ffi::c_int,
    mut conceal_denial: bool,
) -> ::core::ffi::c_int {
    let mut context: *mut authorization_context_t = authorization_context_create();
    if context.is_null() {
        if !res.is_null() {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Authorization context unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return -1 as ::core::ffi::c_int;
    }
    let mut result: ::core::ffi::c_int = authorize_cameras_in_context(
        context,
        req,
        res,
        user,
        camera_uuids,
        camera_count,
        conceal_denial,
    );
    authorization_context_free(context);
    return result;
}
unsafe extern "C" fn extract_uuid(
    mut req: *const http_request_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut res: *mut http_response_t,
) -> bool {
    if http_request_extract_path_param(
        req,
        b"/api/investigation-bookmarks/\0".as_ptr() as *const ::core::ffi::c_char,
        uuid,
        CAMERA_UUID_STRING_SIZE as size_t,
    ) != 0 as ::core::ffi::c_int
        || !lightnvr_uuid_is_valid(uuid)
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid bookmark UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn load_cameras(
    mut bookmark: *const investigation_bookmark_t,
    mut camera_uuids: *mut [::core::ffi::c_char; 37],
    mut res: *mut http_response_t,
) -> bool {
    let mut count: ::core::ffi::c_int = db_investigation_bookmark_list_cameras(
        &raw const (*bookmark).uuid as *const ::core::ffi::c_char,
        camera_uuids,
        INVESTIGATION_BOOKMARK_MAX_CAMERAS,
    );
    if count != (*bookmark).camera_count || count < 1 as ::core::ffi::c_int {
        if !res.is_null() {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to load bookmark cameras\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn bookmark_json(
    mut bookmark: *const investigation_bookmark_t,
    mut camera_uuids: *mut [::core::ffi::c_char; 37],
) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_CreateObject();
    let mut cameras: *mut cJSON = cJSON_CreateArray();
    if object.is_null() || cameras.is_null() {
        cJSON_Delete(object);
        cJSON_Delete(cameras);
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*bookmark).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"title\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*bookmark).title as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"note\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*bookmark).note as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        object,
        b"start_time\0".as_ptr() as *const ::core::ffi::c_char,
        (*bookmark).start_time as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"end_time\0".as_ptr() as *const ::core::ffi::c_char,
        (*bookmark).end_time as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"cursor_time\0".as_ptr() as *const ::core::ffi::c_char,
        (*bookmark).cursor_time as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        object,
        b"primary_camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*bookmark).primary_camera_uuid as *const ::core::ffi::c_char,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*bookmark).camera_count {
        cJSON_AddItemToArray(
            cameras,
            cJSON_CreateString(
                &raw mut *camera_uuids.offset(i as isize) as *mut ::core::ffi::c_char
            ),
        );
        i += 1;
    }
    cJSON_AddItemToObject(
        object,
        b"camera_uuids\0".as_ptr() as *const ::core::ffi::c_char,
        cameras,
    );
    let mut filters: *mut cJSON =
        cJSON_Parse(&raw const (*bookmark).filters_json as *const ::core::ffi::c_char);
    cJSON_AddItemToObject(
        object,
        b"filters\0".as_ptr() as *const ::core::ffi::c_char,
        if !filters.is_null() {
            filters
        } else {
            cJSON_CreateObject()
        },
    );
    let mut result: *mut cJSON = if (*bookmark).representative_result_json[0usize]
        as ::core::ffi::c_int
        != 0
    {
        cJSON_Parse(&raw const (*bookmark).representative_result_json as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if !result.is_null() {
        cJSON_AddItemToObject(
            object,
            b"representative_result\0".as_ptr() as *const ::core::ffi::c_char,
            result,
        );
    } else {
        cJSON_AddNullToObject(
            object,
            b"representative_result\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddNumberToObject(
        object,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        (*bookmark).revision as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*bookmark).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*bookmark).updated_at as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        object,
        b"holds_recordings\0".as_ptr() as *const ::core::ffi::c_char,
        r#false,
    );
    return object;
}
unsafe extern "C" fn send_json(
    mut res: *mut http_response_t,
    mut status: ::core::ffi::c_int,
    mut root: *mut cJSON,
) {
    let mut encoded: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    if encoded.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize bookmark\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, status, encoded);
    free(encoded as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn set_db_error(
    mut res: *mut http_response_t,
    mut result: db_investigation_bookmark_result_t,
) {
    match result {
        db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Bookmark not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_STALE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Bookmark was changed elsewhere\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_LIMIT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Bookmark limit reached\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid bookmark\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Bookmark operation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn audit_mutation(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut uuid: *const ::core::ffi::c_char,
    mut operation: *const ::core::ffi::c_char,
    mut outcome: *const ::core::ffi::c_char,
) {
    audit_log_operation(
        req,
        user,
        BOOKMARK_AUDIT_ACTION.as_ptr(),
        BOOKMARK_TARGET_TYPE.as_ptr(),
        uuid,
        operation,
        outcome,
        ::core::ptr::null::<cJSON>(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_investigation_bookmarks(
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
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || items.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(items);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddItemToObject(
        root,
        b"bookmarks\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    if !demo_read_only(&raw mut user) {
        let mut bookmarks: *mut investigation_bookmark_t = calloc(
            INVESTIGATION_BOOKMARK_LIST_BATCH as size_t,
            ::core::mem::size_of::<investigation_bookmark_t>(),
        )
            as *mut investigation_bookmark_t;
        let mut context: *mut authorization_context_t = authorization_context_create();
        if bookmarks.is_null() || context.is_null() {
            free(bookmarks as *mut ::core::ffi::c_void);
            authorization_context_free(context);
            cJSON_Delete(root);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while offset < INVESTIGATION_BOOKMARK_MAX_PER_OWNER {
            let mut count: ::core::ffi::c_int = db_investigation_bookmark_list(
                user.id,
                offset,
                bookmarks,
                INVESTIGATION_BOOKMARK_LIST_BATCH,
            );
            if count < 0 as ::core::ffi::c_int {
                free(bookmarks as *mut ::core::ffi::c_void);
                authorization_context_free(context);
                cJSON_Delete(root);
                http_response_set_json_error(
                    res,
                    500 as ::core::ffi::c_int,
                    b"Failed to list bookmarks\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < count {
                let mut cameras: [[::core::ffi::c_char; 37]; 16] = [
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
                ];
                if !(!load_cameras(
                    bookmarks.offset(i as isize),
                    &raw mut cameras as *mut [::core::ffi::c_char; 37],
                    ::core::ptr::null_mut::<http_response_t>(),
                ) || authorize_cameras_in_context(
                    context,
                    req,
                    ::core::ptr::null_mut::<http_response_t>(),
                    &raw mut user,
                    &raw mut cameras as *mut [::core::ffi::c_char; 37],
                    (*bookmarks.offset(i as isize)).camera_count,
                    r#true != 0,
                ) != 1 as ::core::ffi::c_int)
                {
                    let mut item: *mut cJSON = bookmark_json(
                        bookmarks.offset(i as isize),
                        &raw mut cameras as *mut [::core::ffi::c_char; 37],
                    );
                    if !item.is_null() {
                        cJSON_AddItemToArray(items, item);
                    }
                }
                i += 1;
            }
            if count < INVESTIGATION_BOOKMARK_LIST_BATCH {
                break;
            }
            offset += INVESTIGATION_BOOKMARK_LIST_BATCH;
        }
        free(bookmarks as *mut ::core::ffi::c_void);
        authorization_context_free(context);
    }
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        cJSON_GetArraySize(items) as ::core::ffi::c_double,
    );
    send_json(res, 200 as ::core::ffi::c_int, root);
    cJSON_Delete(root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_investigation_bookmark(
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
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    if demo_read_only(&raw mut user) {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Demo mode is read-only\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut bookmark: investigation_bookmark_t = investigation_bookmark_t {
        uuid: [0; 37],
        owner_user_id: 0,
        title: [0; 128],
        note: [0; 2048],
        start_time: 0,
        end_time: 0,
        cursor_time: 0,
        primary_camera_uuid: [0; 37],
        filters_json: [0; 4096],
        representative_result_json: [0; 2048],
        camera_count: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut cameras: [[::core::ffi::c_char; 37]; 16] = [
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
    ];
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !parse_create_body(
        body,
        user.id,
        &raw mut bookmark,
        &raw mut cameras as *mut [::core::ffi::c_char; 37],
        &raw mut camera_count,
        res,
    ) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    if authorize_cameras(
        req,
        res,
        &raw mut user,
        &raw mut cameras as *mut [::core::ffi::c_char; 37],
        camera_count,
        r#false != 0,
    ) != 1 as ::core::ffi::c_int
    {
        return;
    }
    let mut result: db_investigation_bookmark_result_t = db_investigation_bookmark_create(
        &raw mut bookmark,
        &raw mut cameras as *mut [::core::ffi::c_char; 37],
        camera_count,
    );
    if result.0 != db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_OK.0 {
        audit_mutation(
            req,
            &raw mut user,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"bookmark_create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
        );
        set_db_error(res, result);
        return;
    }
    audit_mutation(
        req,
        &raw mut user,
        &raw mut bookmark.uuid as *mut ::core::ffi::c_char,
        b"bookmark_create\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut root: *mut cJSON = bookmark_json(
        &raw mut bookmark,
        &raw mut cameras as *mut [::core::ffi::c_char; 37],
    );
    if root.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    send_json(res, 201 as ::core::ffi::c_int, root);
    cJSON_Delete(root);
}
unsafe extern "C" fn load_authorized_bookmark(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut user: *const user_t,
    mut uuid: *const ::core::ffi::c_char,
    mut bookmark: *mut investigation_bookmark_t,
    mut cameras: *mut [::core::ffi::c_char; 37],
) -> bool {
    let mut result: db_investigation_bookmark_result_t =
        db_investigation_bookmark_get((*user).id, uuid, bookmark);
    if result.0 != db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_OK.0 {
        set_db_error(res, result);
        return r#false != 0;
    }
    return load_cameras(bookmark, cameras, res) as ::core::ffi::c_int != 0
        && authorize_cameras(
            req,
            res,
            user,
            cameras,
            (*bookmark).camera_count,
            r#false != 0,
        ) == 1 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_investigation_bookmark(
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
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    if demo_read_only(&raw mut user) {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Bookmark not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [
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
    let mut bookmark: investigation_bookmark_t = investigation_bookmark_t {
        uuid: [0; 37],
        owner_user_id: 0,
        title: [0; 128],
        note: [0; 2048],
        start_time: 0,
        end_time: 0,
        cursor_time: 0,
        primary_camera_uuid: [0; 37],
        filters_json: [0; 4096],
        representative_result_json: [0; 2048],
        camera_count: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut cameras: [[::core::ffi::c_char; 37]; 16] = [
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
    ];
    if !extract_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res)
        || !load_authorized_bookmark(
            req,
            res,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            &raw mut bookmark,
            &raw mut cameras as *mut [::core::ffi::c_char; 37],
        )
    {
        return;
    }
    let mut root: *mut cJSON = bookmark_json(
        &raw mut bookmark,
        &raw mut cameras as *mut [::core::ffi::c_char; 37],
    );
    if root.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    send_json(res, 200 as ::core::ffi::c_int, root);
    cJSON_Delete(root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_investigation_bookmark(
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
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    if demo_read_only(&raw mut user) {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Demo mode is read-only\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [
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
    let mut bookmark: investigation_bookmark_t = investigation_bookmark_t {
        uuid: [0; 37],
        owner_user_id: 0,
        title: [0; 128],
        note: [0; 2048],
        start_time: 0,
        end_time: 0,
        cursor_time: 0,
        primary_camera_uuid: [0; 37],
        filters_json: [0; 4096],
        representative_result_json: [0; 2048],
        camera_count: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut cameras: [[::core::ffi::c_char; 37]; 16] = [
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
    ];
    if !extract_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res)
        || !load_authorized_bookmark(
            req,
            res,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            &raw mut bookmark,
            &raw mut cameras as *mut [::core::ffi::c_char; 37],
        )
    {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut revision: int64_t = 0 as int64_t;
    let mut valid: bool = cJSON_IsObject(body) != 0
        && copy_required_string(
            body,
            b"title\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut bookmark.title as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
        ) as ::core::ffi::c_int
            != 0
        && copy_optional_string(
            body,
            b"note\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut bookmark.note as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>(),
        ) as ::core::ffi::c_int
            != 0
        && parse_int64(
            body,
            b"revision\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut revision,
        ) as ::core::ffi::c_int
            != 0;
    cJSON_Delete(body);
    if !valid {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"title and revision are required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut result: db_investigation_bookmark_result_t =
        db_investigation_bookmark_update_metadata(&raw mut bookmark, revision);
    if result.0 != db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_OK.0 {
        audit_mutation(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"bookmark_update\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
        );
        set_db_error(res, result);
        return;
    }
    audit_mutation(
        req,
        &raw mut user,
        &raw mut uuid as *mut ::core::ffi::c_char,
        b"bookmark_update\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut root: *mut cJSON = bookmark_json(
        &raw mut bookmark,
        &raw mut cameras as *mut [::core::ffi::c_char; 37],
    );
    if root.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    send_json(res, 200 as ::core::ffi::c_int, root);
    cJSON_Delete(root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_investigation_bookmark(
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
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    if demo_read_only(&raw mut user) {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Demo mode is read-only\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [
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
    let mut bookmark: investigation_bookmark_t = investigation_bookmark_t {
        uuid: [0; 37],
        owner_user_id: 0,
        title: [0; 128],
        note: [0; 2048],
        start_time: 0,
        end_time: 0,
        cursor_time: 0,
        primary_camera_uuid: [0; 37],
        filters_json: [0; 4096],
        representative_result_json: [0; 2048],
        camera_count: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut cameras: [[::core::ffi::c_char; 37]; 16] = [
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
    ];
    if !extract_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res)
        || !load_authorized_bookmark(
            req,
            res,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            &raw mut bookmark,
            &raw mut cameras as *mut [::core::ffi::c_char; 37],
        )
    {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut revision: int64_t = 0 as int64_t;
    let mut valid: bool = cJSON_IsObject(body) != 0
        && parse_int64(
            body,
            b"revision\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut revision,
        ) as ::core::ffi::c_int
            != 0;
    cJSON_Delete(body);
    if !valid {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"revision is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut result: db_investigation_bookmark_result_t = db_investigation_bookmark_delete(
        user.id,
        &raw mut uuid as *mut ::core::ffi::c_char,
        revision,
    );
    if result.0 != db_investigation_bookmark_result_t::DB_INVESTIGATION_BOOKMARK_OK.0 {
        audit_mutation(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"bookmark_delete\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
        );
        set_db_error(res, result);
        return;
    }
    audit_mutation(
        req,
        &raw mut user,
        &raw mut uuid as *mut ::core::ffi::c_char,
        b"bookmark_delete\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
    );
    http_response_set_json(
        res,
        200 as ::core::ffi::c_int,
        b"{\"success\":true}\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
