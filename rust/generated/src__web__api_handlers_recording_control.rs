unsafe extern "C" {
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
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
    unsafe fn get_stream_config_by_name(
        name: *const ::core::ffi::c_char,
        stream: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn go2rtc_integration_start_recording_with_trigger(
        stream_name: *const ::core::ffi::c_char,
        trigger_type: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn go2rtc_integration_stop_recording(
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn get_mp4_recording_runtime_info(
        stream_name: *const ::core::ffi::c_char,
        info: *mut recording_runtime_info_t,
    ) -> ::core::ffi::c_int;
    unsafe fn is_recording_scheduled(config: *const stream_config_t) -> bool;
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
    unsafe fn get_unified_detection_recording_id(
        stream_name: *const ::core::ffi::c_char,
    ) -> uint64_t;
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_authorize_stream_action_with_context(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        stream_name: *const ::core::ffi::c_char,
        user: *mut user_t,
        camera: *mut fleet_camera_t,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_evaluate_stream_action(
        user: *const user_t,
        action: authorization_action_t,
        stream_name: *const ::core::ffi::c_char,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recording_runtime_info_t {
    pub active: bool,
    pub initializing: bool,
    pub trigger_type: [::core::ffi::c_char; 16],
    pub recording_id: uint64_t,
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
pub struct authorization_decision_t(pub ::core::ffi::c_uint);
impl authorization_decision_t {
    pub const AUTHZ_DECISION_DENY: Self = Self(0);
    pub const AUTHZ_DECISION_ALLOW: Self = Self(1);
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
unsafe extern "C" fn extract_stream_name(
    mut req: *const http_request_t,
    mut stream_name: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_int {
    if http_request_extract_path_param(
        req,
        b"/api/streams/\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        size,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut suffix: *mut ::core::ffi::c_char = strstr(
        stream_name,
        b"/recording\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if suffix.is_null()
        || *suffix.offset(10isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    *suffix = '\0' as ::core::ffi::c_char;
    return if *stream_name.offset(0isize) as ::core::ffi::c_int != 0 {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn check_stream_access(
    mut req: *const http_request_t,
    mut config: *const stream_config_t,
    mut user: *mut user_t,
    mut write_access: bool,
    mut res: *mut http_response_t,
) -> bool {
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
    return httpd_authorize_stream_action_with_context(
        req,
        res,
        authorization_action_t(if write_access as ::core::ffi::c_int != 0 {
            authorization_action_t::AUTHZ_CAMERA_CONFIGURE.0
        } else {
            authorization_action_t::AUTHZ_LIVE_VIEW.0
        }),
        &raw const (*config).name as *const ::core::ffi::c_char,
        user,
        &raw mut camera,
        &raw mut evaluation,
    ) != 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn add_runtime_json(
    mut json: *mut cJSON,
    mut stream_name: *const ::core::ffi::c_char,
    mut info: *const recording_runtime_info_t,
    mut detection_id: uint64_t,
    mut manual_control_allowed: bool,
    mut manual_control_reason: *const ::core::ffi::c_char,
) {
    let mut active: bool = (*info).active as ::core::ffi::c_int != 0
        || (*info).initializing as ::core::ffi::c_int != 0
        || detection_id != 0 as uint64_t;
    let mut state: *const ::core::ffi::c_char = if (*info).initializing as ::core::ffi::c_int != 0 {
        b"starting\0".as_ptr() as *const ::core::ffi::c_char
    } else if active as ::core::ffi::c_int != 0 {
        b"recording\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"idle\0".as_ptr() as *const ::core::ffi::c_char
    };
    let mut capture_method: *const ::core::ffi::c_char = if detection_id != 0 as uint64_t {
        b"detection\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        &raw const (*info).trigger_type as *const ::core::ffi::c_char
    };
    cJSON_AddStringToObject(
        json,
        b"stream\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    cJSON_AddStringToObject(
        json,
        b"state\0".as_ptr() as *const ::core::ffi::c_char,
        state,
    );
    cJSON_AddBoolToObject(
        json,
        b"recording_active\0".as_ptr() as *const ::core::ffi::c_char,
        active as cJSON_bool,
    );
    if active as ::core::ffi::c_int != 0
        && *capture_method.offset(0isize) as ::core::ffi::c_int != 0
    {
        cJSON_AddStringToObject(
            json,
            b"capture_method\0".as_ptr() as *const ::core::ffi::c_char,
            capture_method,
        );
    } else {
        cJSON_AddNullToObject(
            json,
            b"capture_method\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut id: uint64_t = if detection_id != 0 {
        detection_id
    } else {
        (*info).recording_id
    };
    if id != 0 {
        cJSON_AddNumberToObject(
            json,
            b"recording_id\0".as_ptr() as *const ::core::ffi::c_char,
            id as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(
            json,
            b"recording_id\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddBoolToObject(
        json,
        b"manual_control_allowed\0".as_ptr() as *const ::core::ffi::c_char,
        manual_control_allowed as cJSON_bool,
    );
    if !manual_control_reason.is_null() {
        cJSON_AddStringToObject(
            json,
            b"manual_control_reason\0".as_ptr() as *const ::core::ffi::c_char,
            manual_control_reason,
        );
    } else {
        cJSON_AddNullToObject(
            json,
            b"manual_control_reason\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_stream_recording(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
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
    if extract_stream_name(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name in URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut config: stream_config_t = stream_config_t {
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
    if get_stream_config_by_name(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut config,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
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
    if !check_stream_access(req, &raw mut config, &raw mut user, r#false != 0, res) {
        return;
    }
    let mut info: recording_runtime_info_t = recording_runtime_info_t {
        active: false,
        initializing: false,
        trigger_type: [0; 16],
        recording_id: 0,
    };
    get_mp4_recording_runtime_info(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut info,
    );
    let mut detection_id: uint64_t =
        get_unified_detection_recording_id(&raw mut stream_name as *mut ::core::ffi::c_char);
    let mut write_evaluation: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t::AUTHZ_DECISION_DENY,
        source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
        policy_version: 0,
        grant_uuid: [0; 37],
        token_uuid: [0; 37],
        role_uuid: [0; 37],
        role_name: [0; 128],
        explanation: [0; 256],
    };
    let mut may_write: bool = httpd_evaluate_stream_action(
        &raw mut user,
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut write_evaluation,
    ) == 0 as ::core::ffi::c_int
        && write_evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0;
    let mut continuous_expected: bool = config.record as ::core::ffi::c_int != 0
        && is_recording_scheduled(&raw mut config) as ::core::ffi::c_int != 0;
    let mut manual_control_reason: *const ::core::ffi::c_char = if !may_write {
        b"read_only\0".as_ptr() as *const ::core::ffi::c_char
    } else if continuous_expected as ::core::ffi::c_int != 0 {
        b"continuous_recording\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
    let mut json: *mut cJSON = cJSON_CreateObject();
    if json.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    add_runtime_json(
        json,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut info,
        detection_id,
        may_write as ::core::ffi::c_int != 0 && !continuous_expected,
        manual_control_reason,
    );
    let mut body: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(json);
    cJSON_Delete(json);
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
pub unsafe extern "C" fn handle_post_stream_recording(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
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
    if extract_stream_name(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name in URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut config: stream_config_t = stream_config_t {
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
    if get_stream_config_by_name(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut config,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
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
    if !check_stream_access(req, &raw mut config, &raw mut user, r#true != 0, res) {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut action: *mut cJSON = if !body.is_null() {
        cJSON_GetObjectItemCaseSensitive(body, b"action\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if cJSON_IsString(action) == 0
        || strcmp(
            (*action).valuestring,
            b"start\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
            && strcmp(
                (*action).valuestring,
                b"stop\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
    {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"action must be 'start' or 'stop'\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut starting: bool = strcmp(
        (*action).valuestring,
        b"start\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
    if starting {
        if config.record as ::core::ffi::c_int != 0
            && is_recording_scheduled(&raw mut config) as ::core::ffi::c_int != 0
        {
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Continuous recording is configured for this stream\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    let mut info: recording_runtime_info_t = recording_runtime_info_t {
        active: false,
        initializing: false,
        trigger_type: [0; 16],
        recording_id: 0,
    };
    let mut runtime_rc: ::core::ffi::c_int = get_mp4_recording_runtime_info(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut info,
    );
    let mut detection_id: uint64_t =
        get_unified_detection_recording_id(&raw mut stream_name as *mut ::core::ffi::c_char);
    let mut result: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0;
    if starting {
        if runtime_rc == 0 as ::core::ffi::c_int || detection_id != 0 as uint64_t {
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"A continuous, detection, or manual recording is already active\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        result = go2rtc_integration_start_recording_with_trigger(
            &raw mut stream_name as *mut ::core::ffi::c_char,
            b"manual\0".as_ptr() as *const ::core::ffi::c_char,
        );
        status = 202 as ::core::ffi::c_int;
    } else {
        if runtime_rc != 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut info.trigger_type as *mut ::core::ffi::c_char,
                b"manual\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Only an active manual recording can be stopped here\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        result =
            go2rtc_integration_stop_recording(&raw mut stream_name as *mut ::core::ffi::c_char);
        status = 200 as ::core::ffi::c_int;
    }
    cJSON_Delete(body);
    if result != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to change recording state\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut json: *mut cJSON = cJSON_CreateObject();
    if json.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        json,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        json,
        b"stream\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        json,
        b"action\0".as_ptr() as *const ::core::ffi::c_char,
        if status == 202 as ::core::ffi::c_int {
            b"start\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"stop\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    cJSON_AddStringToObject(
        json,
        b"state\0".as_ptr() as *const ::core::ffi::c_char,
        if status == 202 as ::core::ffi::c_int {
            b"starting\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"idle\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let mut response_body: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(json);
    cJSON_Delete(json);
    if response_body.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, status, response_body);
    free(response_body as *mut ::core::ffi::c_void);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
