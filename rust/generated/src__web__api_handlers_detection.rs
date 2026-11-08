unsafe extern "C" {
    pub type stream_handle_s;
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
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
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
    unsafe fn httpd_authorize_stream_action_with_context(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        stream_name: *const ::core::ffi::c_char,
        user: *mut user_t,
        camera: *mut fleet_camera_t,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    unsafe fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    unsafe fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    unsafe fn get_stream_status(handle: stream_handle_t) -> stream_status_t;
    unsafe fn get_stream_config(
        handle: stream_handle_t,
        config: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_by_name(name: *const ::core::ffi::c_char) -> stream_handle_t;
    unsafe fn go2rtc_integration_is_initialized() -> bool;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn get_unified_detection_effective_status(
        stream_name: *const ::core::ffi::c_char,
    ) -> stream_status_t;
    unsafe fn db_observation_link_detections(
        camera_uuid: *const ::core::ffi::c_char,
        refs: *const observation_detection_ref_t,
        count: size_t,
        out: *mut observation_detection_link_t,
    ) -> ::core::ffi::c_int;
    unsafe fn observation_detection_link_to_json(
        link: *const observation_detection_link_t,
    ) -> *mut cJSON;
    unsafe fn get_detection_results_for_camera_readonly(
        camera_uuid: *const ::core::ffi::c_char,
        legacy_stream_name: *const ::core::ffi::c_char,
        result: *mut detection_result_t,
        timestamps: *mut time_t,
        end_timestamps: *mut time_t,
        max_age: uint64_t,
        start_time: time_t,
        end_time: time_t,
    ) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
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
pub struct detection_result_t {
    pub count: ::core::ffi::c_int,
    pub detections: [detection_t; 20],
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
pub struct observation_detection_link_t {
    pub linked: bool,
    pub observation_uuid: [::core::ffi::c_char; 37],
    pub has_classification: bool,
    pub owner_confirmed: bool,
    pub classification: observation_classification_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_classification_t {
    pub key: [::core::ffi::c_char; 66],
    pub writer_namespace: [::core::ffi::c_char; 33],
    pub writer_user_id: int64_t,
    pub classifier: [::core::ffi::c_char; 65],
    pub model_version: [::core::ffi::c_char; 65],
    pub label: [::core::ffi::c_char; 129],
    pub taxonomy_id: [::core::ffi::c_char; 65],
    pub confidence: ::core::ffi::c_double,
    pub classified_at_ms: int64_t,
    pub input_asset_uuid: [::core::ffi::c_char; 37],
    pub input_revision: ::core::ffi::c_int,
    pub kind: observation_classification_kind_t,
    pub annotation_revision: ::core::ffi::c_int,
    pub created_at_ms: int64_t,
    pub updated_at_ms: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct observation_classification_kind_t(pub ::core::ffi::c_uint);
impl observation_classification_kind_t {
    pub const OBS_CLS_KIND_AUTOMATED: Self = Self(0);
    pub const OBS_CLS_KIND_OWNER_CONFIRMED: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct api_phase_timer_t {
    pub started: timespec,
    pub request: *const http_request_t,
    pub phase: *const ::core::ffi::c_char,
    pub valid: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct observation_detection_ref_t {
    pub label: *const ::core::ffi::c_char,
    pub timestamp: int64_t,
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
pub struct stream_status_t(pub ::core::ffi::c_uint);
impl stream_status_t {
    pub const STREAM_STATUS_UNKNOWN: Self = Self(0);
    pub const STREAM_STATUS_STOPPED: Self = Self(1);
    pub const STREAM_STATUS_STARTING: Self = Self(2);
    pub const STREAM_STATUS_RUNNING: Self = Self(3);
    pub const STREAM_STATUS_STOPPING: Self = Self(4);
    pub const STREAM_STATUS_ERROR: Self = Self(5);
    pub const STREAM_STATUS_RECONNECTING: Self = Self(6);
}
pub type stream_handle_t = *mut stream_handle_s;
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
pub struct C2Rust_Unnamed(pub ::core::ffi::c_int);
impl C2Rust_Unnamed {
    pub const DB_OBS_OK: Self = Self(0);
    pub const DB_OBS_DUPLICATE: Self = Self(1);
    pub const DB_OBS_CAS_LOST: Self = Self(2);
    pub const DB_OBS_ERROR: Self = Self(-1);
    pub const DB_OBS_INVALID: Self = Self(-2);
    pub const DB_OBS_NOT_FOUND: Self = Self(-3);
    pub const DB_OBS_INVALID_CURSOR: Self = Self(-4);
    pub const DB_OBS_CURSOR_EXPIRED: Self = Self(-5);
    pub const DB_OBS_CONFLICT: Self = Self(-6);
}
pub const LOG_COMPONENT: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"DetectionAPI\0") };
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MIN_DETECTION_AGE: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_detection_results(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if http_request_extract_path_param(
        req,
        b"/api/detection/results/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to extract stream name from URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid request path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut phase: api_phase_timer_t = api_phase_begin(
        req,
        b"detection camera authorization\0".as_ptr() as *const ::core::ffi::c_char,
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
    let mut authorized: bool = httpd_authorize_stream_action_with_context(
        req,
        res,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut user,
        &raw mut camera,
        &raw mut evaluation,
    ) != 0;
    api_phase_end(&raw mut phase);
    if !authorized {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/detection/results/%s request\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut start_time: time_t = 0 as time_t;
    let mut end_time: time_t = 0 as time_t;
    let mut start_str: [::core::ffi::c_char; 32] = [
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
    if http_request_get_query_param(
        req,
        b"start\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut start_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    ) > 0 as ::core::ffi::c_int
        && start_str[0usize] as ::core::ffi::c_int != 0
    {
        start_time = strtoll(
            &raw mut start_str as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        ) as time_t;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Using start_time filter: %lld (str='%s')\0".as_ptr() as *const ::core::ffi::c_char,
            start_time as ::core::ffi::c_longlong,
            &raw mut start_str as *mut ::core::ffi::c_char,
        );
    }
    let mut end_str: [::core::ffi::c_char; 32] = [
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
    if http_request_get_query_param(
        req,
        b"end\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut end_str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
    ) > 0 as ::core::ffi::c_int
        && end_str[0usize] as ::core::ffi::c_int != 0
    {
        end_time = strtoll(
            &raw mut end_str as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        ) as time_t;
        _log_message_ctx(
            log_level_t::LOG_LEVEL_INFO,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Using end_time filter: %lld (str='%s')\0".as_ptr() as *const ::core::ffi::c_char,
            end_time as ::core::ffi::c_longlong,
            &raw mut end_str as *mut ::core::ffi::c_char,
        );
    }
    let mut max_age: uint64_t = MIN_DETECTION_AGE as uint64_t;
    if start_time > 0 as time_t || end_time > 0 as time_t {
        max_age = 0 as uint64_t;
    } else {
        let mut stream: stream_handle_t =
            get_stream_by_name(&raw mut stream_name as *mut ::core::ffi::c_char);
        if stream.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Stream not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut stream_cfg: stream_config_t = stream_config_t {
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
        if get_stream_config(stream, &raw mut stream_cfg) == 0 as ::core::ffi::c_int
            && stream_cfg.detection_interval > 0 as ::core::ffi::c_int
        {
            let mut interval_window: uint64_t =
                (stream_cfg.detection_interval as uint64_t).wrapping_mul(2 as uint64_t);
            if interval_window > max_age {
                max_age = interval_window;
            }
        }
    }
    let mut result: detection_result_t = detection_result_t {
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
        &raw mut result as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<detection_result_t>(),
    );
    let mut timestamps: [time_t; 20] = [0; 20];
    let mut end_timestamps: [time_t; 20] = [0; 20];
    memset(
        &raw mut timestamps as *mut time_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[time_t; 20]>(),
    );
    memset(
        &raw mut end_timestamps as *mut time_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[time_t; 20]>(),
    );
    phase = api_phase_begin(
        req,
        b"detection result database snapshot\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut count: ::core::ffi::c_int = get_detection_results_for_camera_readonly(
        &raw mut camera.camera_uuid as *mut ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut result,
        &raw mut timestamps as *mut time_t,
        &raw mut end_timestamps as *mut time_t,
        max_age,
        start_time,
        end_time,
    );
    api_phase_end(&raw mut phase);
    if count < 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to get detections from database for stream: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut stream_name as *mut ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get detection results\0".as_ptr() as *const ::core::ffi::c_char,
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
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut detections_array: *mut cJSON = cJSON_CreateArray();
    if detections_array.is_null() {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to create detections JSON array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_Delete(response);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create detections JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddItemToObject(
        response,
        b"detections\0".as_ptr() as *const ::core::ffi::c_char,
        detections_array,
    );
    let mut timestamp: [::core::ffi::c_char; 32] = [0; 32];
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut tm_buf: tm = tm {
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
    let mut tm_info: *const tm = localtime_r(&raw mut now, &raw mut tm_buf);
    strftime(
        &raw mut timestamp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        b"%Y-%m-%d %H:%M:%S\0".as_ptr() as *const ::core::ffi::c_char,
        tm_info,
    );
    cJSON_AddStringToObject(
        response,
        b"timestamp\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut timestamp as *mut ::core::ffi::c_char,
    );
    let mut status_str: *const ::core::ffi::c_char =
        b"Unknown\0".as_ptr() as *const ::core::ffi::c_char;
    let mut sh: stream_handle_t =
        get_stream_by_name(&raw mut stream_name as *mut ::core::ffi::c_char);
    if !sh.is_null() {
        let mut scfg: stream_config_t = stream_config_t {
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
            &raw mut scfg as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<stream_config_t>(),
        );
        if get_stream_config(sh, &raw mut scfg) == 0 as ::core::ffi::c_int {
            let mut ss: stream_status_t = get_stream_status(sh);
            if ss.0 == stream_status_t::STREAM_STATUS_STOPPED.0
                && scfg.enabled as ::core::ffi::c_int != 0
                && go2rtc_integration_is_initialized() as ::core::ffi::c_int != 0
            {
                let mut udt: stream_status_t = get_unified_detection_effective_status(
                    &raw mut stream_name as *mut ::core::ffi::c_char,
                );
                ss = stream_status_t(if udt.0 != stream_status_t::STREAM_STATUS_STOPPED.0 {
                    udt.0
                } else {
                    stream_status_t::STREAM_STATUS_RUNNING.0
                });
            }
            match ss {
                stream_status_t::STREAM_STATUS_STOPPED => {
                    status_str = b"Stopped\0".as_ptr() as *const ::core::ffi::c_char;
                }
                stream_status_t::STREAM_STATUS_STARTING => {
                    status_str = b"Starting\0".as_ptr() as *const ::core::ffi::c_char;
                }
                stream_status_t::STREAM_STATUS_RUNNING => {
                    status_str = b"Running\0".as_ptr() as *const ::core::ffi::c_char;
                }
                stream_status_t::STREAM_STATUS_STOPPING => {
                    status_str = b"Stopping\0".as_ptr() as *const ::core::ffi::c_char;
                }
                stream_status_t::STREAM_STATUS_ERROR => {
                    status_str = b"Error\0".as_ptr() as *const ::core::ffi::c_char;
                }
                stream_status_t::STREAM_STATUS_RECONNECTING => {
                    status_str = b"Reconnecting\0".as_ptr() as *const ::core::ffi::c_char;
                }
                _ => {
                    status_str = b"Unknown\0".as_ptr() as *const ::core::ffi::c_char;
                }
            }
        }
    }
    cJSON_AddStringToObject(
        response,
        b"stream_status\0".as_ptr() as *const ::core::ffi::c_char,
        status_str,
    );
    let mut links: *mut observation_detection_link_t =
        ::core::ptr::null_mut::<observation_detection_link_t>();
    if result.count > 0 as ::core::ffi::c_int
        && lightnvr_uuid_is_valid(&raw mut camera.camera_uuid as *mut ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
    {
        links = calloc(
            result.count as size_t,
            ::core::mem::size_of::<observation_detection_link_t>(),
        ) as *mut observation_detection_link_t;
    }
    if !links.is_null() {
        let mut refs: [observation_detection_ref_t; 20] = [observation_detection_ref_t {
            label: ::core::ptr::null::<::core::ffi::c_char>(),
            timestamp: 0,
        }; 20];
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < result.count {
            refs[i as usize].label = &raw mut (*(&raw mut result.detections as *mut detection_t)
                .offset(i as isize))
            .label as *mut ::core::ffi::c_char;
            refs[i as usize].timestamp = timestamps[i as usize] as int64_t;
            i += 1;
        }
        phase = api_phase_begin(
            req,
            b"detection observation link\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if db_observation_link_detections(
            &raw mut camera.camera_uuid as *mut ::core::ffi::c_char,
            &raw mut refs as *mut observation_detection_ref_t,
            result.count as size_t,
            links,
        ) != C2Rust_Unnamed::DB_OBS_OK.0
        {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_WARN,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Observation links unavailable for stream %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
            );
        }
        api_phase_end(&raw mut phase);
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < result.count {
        let mut detection: *mut cJSON = cJSON_CreateObject();
        if detection.is_null() {
            _log_message_ctx(
                log_level_t::LOG_LEVEL_ERROR,
                LOG_COMPONENT.as_ptr(),
                _log_stream_name,
                b"Failed to create detection JSON object\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            cJSON_AddStringToObject(
                detection,
                b"label\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*(&raw mut result.detections as *mut detection_t).offset(i_0 as isize))
                    .label as *mut ::core::ffi::c_char,
            );
            cJSON_AddNumberToObject(
                detection,
                b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
                result.detections[i_0 as usize].confidence as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                detection,
                b"x\0".as_ptr() as *const ::core::ffi::c_char,
                result.detections[i_0 as usize].x as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                detection,
                b"y\0".as_ptr() as *const ::core::ffi::c_char,
                result.detections[i_0 as usize].y as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                detection,
                b"width\0".as_ptr() as *const ::core::ffi::c_char,
                result.detections[i_0 as usize].width as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                detection,
                b"height\0".as_ptr() as *const ::core::ffi::c_char,
                result.detections[i_0 as usize].height as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                detection,
                b"timestamp\0".as_ptr() as *const ::core::ffi::c_char,
                timestamps[i_0 as usize] as ::core::ffi::c_double,
            );
            cJSON_AddNumberToObject(
                detection,
                b"end_timestamp\0".as_ptr() as *const ::core::ffi::c_char,
                end_timestamps[i_0 as usize] as ::core::ffi::c_double,
            );
            let mut observation: *mut cJSON = if !links.is_null() {
                observation_detection_link_to_json(links.offset(i_0 as isize))
            } else {
                ::core::ptr::null_mut::<cJSON>()
            };
            if !observation.is_null() {
                cJSON_AddItemToObject(
                    detection,
                    b"observation\0".as_ptr() as *const ::core::ffi::c_char,
                    observation,
                );
            }
            cJSON_AddItemToArray(detections_array, detection);
        }
        i_0 += 1;
    }
    free(links as *mut ::core::ffi::c_void);
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
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Successfully handled GET /api/detection/results/%s request, returned %d detections\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        result.count,
    );
}
