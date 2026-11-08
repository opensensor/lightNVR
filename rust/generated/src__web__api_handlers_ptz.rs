unsafe extern "C" {
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
    unsafe fn cJSON_ParseWithLengthOpts(
        value: *const ::core::ffi::c_char,
        buffer_length: size_t,
        return_parse_end: *mut *const ::core::ffi::c_char,
        require_null_terminated: cJSON_bool,
    ) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
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
    unsafe fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn httpd_authorize_stream_action_with_context(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        stream_name: *const ::core::ffi::c_char,
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
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn url_build_onvif_service_url(
        url: *const ::core::ffi::c_char,
        onvif_port: ::core::ffi::c_int,
        service_path: *const ::core::ffi::c_char,
        out_url: *mut ::core::ffi::c_char,
        out_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn get_stream_config_by_name(
        name: *const ::core::ffi::c_char,
        stream: *mut stream_config_t,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_ptz_get_service_url(
        device_url: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        ptz_url: *mut ::core::ffi::c_char,
        url_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_ptz_get_capabilities(
        ptz_url: *const ::core::ffi::c_char,
        profile_token: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        capabilities: *mut onvif_ptz_capabilities_t,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_ptz_continuous_move(
        ptz_url: *const ::core::ffi::c_char,
        profile_token: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        pan_velocity: ::core::ffi::c_float,
        tilt_velocity: ::core::ffi::c_float,
        zoom_velocity: ::core::ffi::c_float,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_ptz_stop(
        ptz_url: *const ::core::ffi::c_char,
        profile_token: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        stop_pan_tilt: bool,
        stop_zoom: bool,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_ptz_absolute_move_axes(
        ptz_url: *const ::core::ffi::c_char,
        profile_token: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        has_pan_tilt: bool,
        pan: ::core::ffi::c_float,
        tilt: ::core::ffi::c_float,
        has_zoom: bool,
        zoom: ::core::ffi::c_float,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_ptz_relative_move_axes(
        ptz_url: *const ::core::ffi::c_char,
        profile_token: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        has_pan_tilt: bool,
        pan_delta: ::core::ffi::c_float,
        tilt_delta: ::core::ffi::c_float,
        has_zoom: bool,
        zoom_delta: ::core::ffi::c_float,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_ptz_goto_home(
        ptz_url: *const ::core::ffi::c_char,
        profile_token: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_ptz_set_home(
        ptz_url: *const ::core::ffi::c_char,
        profile_token: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_ptz_get_presets(
        ptz_url: *const ::core::ffi::c_char,
        profile_token: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        presets: *mut onvif_ptz_preset_t,
        max_presets: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_ptz_goto_preset(
        ptz_url: *const ::core::ffi::c_char,
        profile_token: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        preset_token: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_ptz_set_preset(
        ptz_url: *const ::core::ffi::c_char,
        profile_token: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        preset_name: *const ::core::ffi::c_char,
        preset_token: *mut ::core::ffi::c_char,
        token_size: size_t,
    ) -> ::core::ffi::c_int;
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type int64_t = i64;
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
pub struct ptz_authorization_context_t {
    pub user: user_t,
    pub camera: fleet_camera_t,
    pub evaluation: authorization_evaluation_t,
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
pub struct authorization_decision_t(pub ::core::ffi::c_uint);
impl authorization_decision_t {
    pub const AUTHZ_DECISION_DENY: Self = Self(0);
    pub const AUTHZ_DECISION_ALLOW: Self = Self(1);
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
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
pub struct onvif_ptz_preset_t {
    pub token: [::core::ffi::c_char; 64],
    pub name: [::core::ffi::c_char; 128],
    pub pan: ::core::ffi::c_float,
    pub tilt: ::core::ffi::c_float,
    pub zoom: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_ptz_capabilities_t {
    pub queried: bool,
    pub has_pan_tilt: bool,
    pub has_zoom: bool,
    pub has_continuous_move: bool,
    pub has_absolute_move: bool,
    pub has_relative_move: bool,
    pub has_home_position: bool,
    pub has_presets: bool,
    pub max_presets: ::core::ffi::c_int,
    pub pan_min: ::core::ffi::c_float,
    pub pan_max: ::core::ffi::c_float,
    pub tilt_min: ::core::ffi::c_float,
    pub tilt_max: ::core::ffi::c_float,
    pub zoom_min: ::core::ffi::c_float,
    pub zoom_max: ::core::ffi::c_float,
}
#[inline]
unsafe extern "C" fn device_json_body(mut req: *const http_request_t) -> *mut cJSON {
    if (*req).body.is_null() || (*req).body_len == 0 {
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut limit: *const ::core::ffi::c_char =
        (*req).body.offset((*req).body_len as isize) as *const ::core::ffi::c_char;
    let mut body: *mut cJSON = cJSON_ParseWithLengthOpts(
        (*req).body as *const ::core::ffi::c_char,
        (*req).body_len,
        &raw mut end,
        0 as cJSON_bool,
    );
    while !end.is_null()
        && end < limit
        && *end as ::core::ffi::c_int != 0
        && !strchr(
            b" \t\r\n\0".as_ptr() as *const ::core::ffi::c_char,
            *end as ::core::ffi::c_int,
        )
        .is_null()
    {
        end = end.offset(1);
    }
    if body.is_null() || cJSON_IsObject(body) == 0 || end != limit {
        cJSON_Delete(body);
        return ::core::ptr::null_mut::<cJSON>();
    }
    return body;
}
#[inline]
unsafe extern "C" fn device_keys(
    mut body: *const cJSON,
    mut keys: *const *const ::core::ffi::c_char,
    mut count: size_t,
) -> bool {
    if cJSON_IsObject(body) == 0 {
        return r#false != 0;
    }
    let mut field: *const cJSON = ::core::ptr::null::<cJSON>();
    field = if !body.is_null() {
        (*body).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !field.is_null() {
        let mut known: bool = r#false != 0;
        let mut i: size_t = 0 as size_t;
        while i < count {
            if strcmp((*field).string, *keys.offset(i as isize)) == 0 {
                known = r#true != 0;
            }
            i = i.wrapping_add(1);
        }
        if !known {
            return r#false != 0;
        }
        let mut prior: *const cJSON = (*body).child;
        while prior != field {
            if strcmp((*prior).string, (*field).string) == 0 {
                return r#false != 0;
            }
            prior = (*prior).next;
        }
        field = (*field).next;
    }
    return r#true != 0;
}
#[inline]
unsafe extern "C" fn device_number(mut field: *const cJSON) -> bool {
    return cJSON_IsNumber(field) != 0
        && (*field).valuedouble.is_finite() as ::core::ffi::c_int != 0
        && fabs((*field).valuedouble) <= FLT_MAX as ::core::ffi::c_double;
}
#[inline]
unsafe extern "C" fn device_string(
    mut field: *const cJSON,
    mut maximum: size_t,
    mut empty: bool,
) -> bool {
    return cJSON_IsString(field) != 0
        && !(*field).valuestring.is_null()
        && (empty as ::core::ffi::c_int != 0 || *(*field).valuestring as ::core::ffi::c_int != 0)
        && strlen((*field).valuestring) <= maximum;
}
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FLT_MAX: ::core::ffi::c_float = __FLT_MAX__;
pub const LOG_COMPONENT: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"PTZAPI\0") };
unsafe extern "C" fn get_ptz_stream_config(
    mut stream_name: *const ::core::ffi::c_char,
    mut config: *mut stream_config_t,
) -> ::core::ffi::c_int {
    if stream_name.is_null() || config.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if get_stream_config_by_name(stream_name, config) != 0 as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Stream not found: %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -1 as ::core::ffi::c_int;
    }
    if !(*config).ptz_enabled {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"PTZ not enabled for stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn build_ptz_url(
    mut config: *const stream_config_t,
    mut ptz_url: *mut ::core::ffi::c_char,
    mut url_size: size_t,
) -> ::core::ffi::c_int {
    if config.is_null() || ptz_url.is_null() || url_size == 0 as size_t {
        return -1 as ::core::ffi::c_int;
    }
    let mut device_url: [::core::ffi::c_char; 512] = [0; 512];
    if url_build_onvif_service_url(
        &raw const (*config).url as *const ::core::ffi::c_char,
        (*config).onvif_port,
        b"/onvif/device_service\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut device_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    return onvif_ptz_get_service_url(
        &raw mut device_url as *mut ::core::ffi::c_char,
        &raw const (*config).onvif_username as *const ::core::ffi::c_char,
        &raw const (*config).onvif_password as *const ::core::ffi::c_char,
        ptz_url,
        url_size,
    );
}
unsafe extern "C" fn get_profile_token(
    mut config: *const stream_config_t,
) -> *const ::core::ffi::c_char {
    if !config.is_null()
        && (*config).onvif_profile[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        return &raw const (*config).onvif_profile as *const ::core::ffi::c_char;
    }
    return b"Profile_1\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn extract_ptz_stream_name(
    mut req: *const http_request_t,
    mut stream_name: *mut ::core::ffi::c_char,
    mut name_size: size_t,
) -> ::core::ffi::c_int {
    if http_request_extract_path_param(
        req,
        b"/api/streams/\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        name_size,
    ) != 0 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut ptz_suffix: *mut ::core::ffi::c_char = strstr(
        stream_name,
        b"/ptz\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !ptz_suffix.is_null() {
        *ptz_suffix = '\0' as ::core::ffi::c_char;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn authorize_ptz_stream(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut stream_name: *const ::core::ffi::c_char,
    mut action: authorization_action_t,
    mut context: *mut ptz_authorization_context_t,
) -> ::core::ffi::c_int {
    return httpd_authorize_stream_action_with_context(
        req,
        res,
        action,
        stream_name,
        &raw mut (*context).user,
        &raw mut (*context).camera,
        &raw mut (*context).evaluation,
    );
}
unsafe extern "C" fn audit_ptz_operation(
    mut req: *const http_request_t,
    mut context: *const ptz_authorization_context_t,
    mut operation: *const ::core::ffi::c_char,
    mut outcome: *const ::core::ffi::c_char,
    mut reason: *const ::core::ffi::c_char,
) {
    let mut details: *mut cJSON = cJSON_CreateObject();
    if !details.is_null() && !reason.is_null() {
        cJSON_AddStringToObject(
            details,
            b"reason\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        );
    }
    audit_log_operation(
        req,
        &raw const (*context).user,
        b"ptz.control\0".as_ptr() as *const ::core::ffi::c_char,
        b"camera\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*context).camera.camera_uuid as *const ::core::ffi::c_char,
        operation,
        outcome,
        details,
    );
    cJSON_Delete(details);
}
unsafe extern "C" fn valid_ptz_body(
    mut body: *const cJSON,
    mut action: *const ::core::ffi::c_char,
) -> bool {
    static mut axes: [*const ::core::ffi::c_char; 3] = [
        b"pan\0".as_ptr() as *const ::core::ffi::c_char,
        b"tilt\0".as_ptr() as *const ::core::ffi::c_char,
        b"zoom\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut token: [*const ::core::ffi::c_char; 1] =
        [b"token\0".as_ptr() as *const ::core::ffi::c_char];
    static mut name: [*const ::core::ffi::c_char; 1] =
        [b"name\0".as_ptr() as *const ::core::ffi::c_char];
    if strcmp(
        action,
        b"goto-preset\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        return device_keys(
            body,
            &raw mut token as *mut *const ::core::ffi::c_char,
            1 as size_t,
        ) as ::core::ffi::c_int
            != 0
            && device_string(
                cJSON_GetObjectItemCaseSensitive(
                    body,
                    b"token\0".as_ptr() as *const ::core::ffi::c_char,
                ),
                63 as size_t,
                r#false != 0,
            ) as ::core::ffi::c_int
                != 0;
    }
    if strcmp(action, b"preset\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
        let mut field: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            body,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return device_keys(
            body,
            &raw mut name as *mut *const ::core::ffi::c_char,
            1 as size_t,
        ) as ::core::ffi::c_int
            != 0
            && (field.is_null()
                || device_string(field, 63 as size_t, r#true != 0) as ::core::ffi::c_int != 0);
    }
    if !device_keys(
        body,
        &raw mut axes as *mut *const ::core::ffi::c_char,
        3 as size_t,
    ) || (*body).child.is_null()
    {
        return r#false != 0;
    }
    let mut field_0: *const cJSON = ::core::ptr::null::<cJSON>();
    field_0 = if !body.is_null() {
        (*body).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !field_0.is_null() {
        if !device_number(field_0) {
            return r#false != 0;
        }
        field_0 = (*field_0).next;
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_ptz_move(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if extract_ptz_stream_name(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/streams/%s/ptz/move\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut auth: ptz_authorization_context_t = ptz_authorization_context_t {
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
        camera: fleet_camera_t {
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
        },
        evaluation: authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        },
    };
    if authorize_ptz_stream(
        req,
        res,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        authorization_action_t::AUTHZ_PTZ_CONTROL,
        &raw mut auth,
    ) == 0
    {
        return;
    }
    let mut config: stream_config_t = stream_config_t {
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
    let mut rc: ::core::ffi::c_int = get_ptz_stream_config(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut config,
    );
    if rc == -1 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"continuous_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"stream_configuration_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    } else if rc == -2 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"continuous_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"ptz_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"PTZ not enabled for this stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = device_json_body(req);
    if !valid_ptz_body(body, b"move\0".as_ptr() as *const ::core::ffi::c_char) {
        cJSON_Delete(body);
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"continuous_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut pan: ::core::ffi::c_float = 0.0f32;
    let mut tilt: ::core::ffi::c_float = 0.0f32;
    let mut zoom: ::core::ffi::c_float = 0.0f32;
    let mut pan_json: *mut cJSON =
        cJSON_GetObjectItem(body, b"pan\0".as_ptr() as *const ::core::ffi::c_char);
    let mut tilt_json: *mut cJSON =
        cJSON_GetObjectItem(body, b"tilt\0".as_ptr() as *const ::core::ffi::c_char);
    let mut zoom_json: *mut cJSON =
        cJSON_GetObjectItem(body, b"zoom\0".as_ptr() as *const ::core::ffi::c_char);
    if !pan_json.is_null() && cJSON_IsNumber(pan_json) != 0 {
        pan = (*pan_json).valuedouble as ::core::ffi::c_float;
    }
    if !tilt_json.is_null() && cJSON_IsNumber(tilt_json) != 0 {
        tilt = (*tilt_json).valuedouble as ::core::ffi::c_float;
    }
    if !zoom_json.is_null() && cJSON_IsNumber(zoom_json) != 0 {
        zoom = (*zoom_json).valuedouble as ::core::ffi::c_float;
    }
    cJSON_Delete(body);
    let mut ptz_url: [::core::ffi::c_char; 512] = [0; 512];
    if build_ptz_url(
        &raw mut config,
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"continuous_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"service_discovery_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build PTZ URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut profile_token: *const ::core::ffi::c_char = get_profile_token(&raw mut config);
    rc = onvif_ptz_continuous_move(
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        profile_token,
        &raw mut config.onvif_username as *mut ::core::ffi::c_char,
        &raw mut config.onvif_password as *mut ::core::ffi::c_char,
        pan,
        tilt,
        zoom,
    );
    if rc != 0 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"continuous_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"device_operation_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"PTZ move failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_ptz_operation(
        req,
        &raw mut auth,
        b"continuous_move\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"completed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"PTZ move started\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_ptz_stop(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if extract_ptz_stream_name(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/streams/%s/ptz/stop\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut auth: ptz_authorization_context_t = ptz_authorization_context_t {
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
        camera: fleet_camera_t {
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
        },
        evaluation: authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        },
    };
    if authorize_ptz_stream(
        req,
        res,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        authorization_action_t::AUTHZ_PTZ_CONTROL,
        &raw mut auth,
    ) == 0
    {
        return;
    }
    if (*req).body_len != 0 {
        let mut body: *mut cJSON = device_json_body(req);
        let mut empty: bool = device_keys(
            body,
            ::core::ptr::null::<*const ::core::ffi::c_char>(),
            0 as size_t,
        );
        cJSON_Delete(body);
        if !empty {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Expected empty device command\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    let mut config: stream_config_t = stream_config_t {
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
    let mut rc: ::core::ffi::c_int = get_ptz_stream_config(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut config,
    );
    if rc == -1 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"stop\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"stream_configuration_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    } else if rc == -2 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"stop\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"ptz_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"PTZ not enabled for this stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut ptz_url: [::core::ffi::c_char; 512] = [0; 512];
    if build_ptz_url(
        &raw mut config,
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"stop\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"service_discovery_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build PTZ URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut profile_token: *const ::core::ffi::c_char = get_profile_token(&raw mut config);
    rc = onvif_ptz_stop(
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        profile_token,
        &raw mut config.onvif_username as *mut ::core::ffi::c_char,
        &raw mut config.onvif_password as *mut ::core::ffi::c_char,
        r#true != 0,
        r#true != 0,
    );
    if rc != 0 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"stop\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"device_operation_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"PTZ stop failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_ptz_operation(
        req,
        &raw mut auth,
        b"stop\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"completed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"PTZ stopped\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_ptz_absolute(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if extract_ptz_stream_name(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/streams/%s/ptz/absolute\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut auth: ptz_authorization_context_t = ptz_authorization_context_t {
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
        camera: fleet_camera_t {
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
        },
        evaluation: authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        },
    };
    if authorize_ptz_stream(
        req,
        res,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        authorization_action_t::AUTHZ_PTZ_CONTROL,
        &raw mut auth,
    ) == 0
    {
        return;
    }
    let mut config: stream_config_t = stream_config_t {
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
    let mut rc: ::core::ffi::c_int = get_ptz_stream_config(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut config,
    );
    if rc == -1 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"absolute_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"stream_configuration_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    } else if rc == -2 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"absolute_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"ptz_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"PTZ not enabled for this stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = device_json_body(req);
    if !valid_ptz_body(body, b"absolute\0".as_ptr() as *const ::core::ffi::c_char) {
        cJSON_Delete(body);
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"absolute_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut pan: ::core::ffi::c_float = 0.0f32;
    let mut tilt: ::core::ffi::c_float = 0.0f32;
    let mut zoom: ::core::ffi::c_float = 0.0f32;
    let mut has_pan: bool = r#false != 0;
    let mut has_tilt: bool = r#false != 0;
    let mut has_zoom: bool = r#false != 0;
    let mut pan_json: *mut cJSON =
        cJSON_GetObjectItem(body, b"pan\0".as_ptr() as *const ::core::ffi::c_char);
    let mut tilt_json: *mut cJSON =
        cJSON_GetObjectItem(body, b"tilt\0".as_ptr() as *const ::core::ffi::c_char);
    let mut zoom_json: *mut cJSON =
        cJSON_GetObjectItem(body, b"zoom\0".as_ptr() as *const ::core::ffi::c_char);
    if !pan_json.is_null() && cJSON_IsNumber(pan_json) != 0 {
        pan = (*pan_json).valuedouble as ::core::ffi::c_float;
        has_pan = r#true != 0;
    }
    if !tilt_json.is_null() && cJSON_IsNumber(tilt_json) != 0 {
        tilt = (*tilt_json).valuedouble as ::core::ffi::c_float;
        has_tilt = r#true != 0;
    }
    if !zoom_json.is_null() && cJSON_IsNumber(zoom_json) != 0 {
        zoom = (*zoom_json).valuedouble as ::core::ffi::c_float;
        has_zoom = r#true != 0;
    }
    cJSON_Delete(body);
    if !has_pan && !has_tilt && !has_zoom {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"absolute_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"missing_axis\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"At least one PTZ axis is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut ptz_url: [::core::ffi::c_char; 512] = [0; 512];
    if build_ptz_url(
        &raw mut config,
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"absolute_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"service_discovery_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build PTZ URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut profile_token: *const ::core::ffi::c_char = get_profile_token(&raw mut config);
    rc = onvif_ptz_absolute_move_axes(
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        profile_token,
        &raw mut config.onvif_username as *mut ::core::ffi::c_char,
        &raw mut config.onvif_password as *mut ::core::ffi::c_char,
        has_pan as ::core::ffi::c_int != 0 || has_tilt as ::core::ffi::c_int != 0,
        pan,
        tilt,
        has_zoom,
        zoom,
    );
    if rc != 0 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"absolute_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"device_operation_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"PTZ absolute move failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_ptz_operation(
        req,
        &raw mut auth,
        b"absolute_move\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"completed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"PTZ absolute move completed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_ptz_relative(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if extract_ptz_stream_name(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/streams/%s/ptz/relative\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut auth: ptz_authorization_context_t = ptz_authorization_context_t {
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
        camera: fleet_camera_t {
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
        },
        evaluation: authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        },
    };
    if authorize_ptz_stream(
        req,
        res,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        authorization_action_t::AUTHZ_PTZ_CONTROL,
        &raw mut auth,
    ) == 0
    {
        return;
    }
    let mut config: stream_config_t = stream_config_t {
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
    let mut rc: ::core::ffi::c_int = get_ptz_stream_config(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut config,
    );
    if rc == -1 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"relative_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"stream_configuration_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    } else if rc == -2 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"relative_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"ptz_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"PTZ not enabled for this stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = device_json_body(req);
    if !valid_ptz_body(body, b"relative\0".as_ptr() as *const ::core::ffi::c_char) {
        cJSON_Delete(body);
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"relative_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut pan: ::core::ffi::c_float = 0.0f32;
    let mut tilt: ::core::ffi::c_float = 0.0f32;
    let mut zoom: ::core::ffi::c_float = 0.0f32;
    let mut has_pan: bool = r#false != 0;
    let mut has_tilt: bool = r#false != 0;
    let mut has_zoom: bool = r#false != 0;
    let mut pan_json: *mut cJSON =
        cJSON_GetObjectItem(body, b"pan\0".as_ptr() as *const ::core::ffi::c_char);
    let mut tilt_json: *mut cJSON =
        cJSON_GetObjectItem(body, b"tilt\0".as_ptr() as *const ::core::ffi::c_char);
    let mut zoom_json: *mut cJSON =
        cJSON_GetObjectItem(body, b"zoom\0".as_ptr() as *const ::core::ffi::c_char);
    if !pan_json.is_null() && cJSON_IsNumber(pan_json) != 0 {
        pan = (*pan_json).valuedouble as ::core::ffi::c_float;
        has_pan = r#true != 0;
    }
    if !tilt_json.is_null() && cJSON_IsNumber(tilt_json) != 0 {
        tilt = (*tilt_json).valuedouble as ::core::ffi::c_float;
        has_tilt = r#true != 0;
    }
    if !zoom_json.is_null() && cJSON_IsNumber(zoom_json) != 0 {
        zoom = (*zoom_json).valuedouble as ::core::ffi::c_float;
        has_zoom = r#true != 0;
    }
    cJSON_Delete(body);
    if !has_pan && !has_tilt && !has_zoom {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"relative_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"missing_axis\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"At least one PTZ axis is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut ptz_url: [::core::ffi::c_char; 512] = [0; 512];
    if build_ptz_url(
        &raw mut config,
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"relative_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"service_discovery_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build PTZ URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut profile_token: *const ::core::ffi::c_char = get_profile_token(&raw mut config);
    rc = onvif_ptz_relative_move_axes(
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        profile_token,
        &raw mut config.onvif_username as *mut ::core::ffi::c_char,
        &raw mut config.onvif_password as *mut ::core::ffi::c_char,
        has_pan as ::core::ffi::c_int != 0 || has_tilt as ::core::ffi::c_int != 0,
        pan,
        tilt,
        has_zoom,
        zoom,
    );
    if rc != 0 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"relative_move\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"device_operation_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"PTZ relative move failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_ptz_operation(
        req,
        &raw mut auth,
        b"relative_move\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"completed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"PTZ relative move completed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_ptz_home(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if extract_ptz_stream_name(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/streams/%s/ptz/home\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut auth: ptz_authorization_context_t = ptz_authorization_context_t {
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
        camera: fleet_camera_t {
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
        },
        evaluation: authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        },
    };
    if authorize_ptz_stream(
        req,
        res,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        authorization_action_t::AUTHZ_PTZ_CONTROL,
        &raw mut auth,
    ) == 0
    {
        return;
    }
    if (*req).body_len != 0 {
        let mut body: *mut cJSON = device_json_body(req);
        let mut empty: bool = device_keys(
            body,
            ::core::ptr::null::<*const ::core::ffi::c_char>(),
            0 as size_t,
        );
        cJSON_Delete(body);
        if !empty {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Expected empty device command\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    let mut config: stream_config_t = stream_config_t {
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
    let mut rc: ::core::ffi::c_int = get_ptz_stream_config(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut config,
    );
    if rc == -1 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"goto_home\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"stream_configuration_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    } else if rc == -2 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"goto_home\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"ptz_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"PTZ not enabled for this stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut ptz_url: [::core::ffi::c_char; 512] = [0; 512];
    if build_ptz_url(
        &raw mut config,
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"goto_home\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"service_discovery_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build PTZ URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut profile_token: *const ::core::ffi::c_char = get_profile_token(&raw mut config);
    rc = onvif_ptz_goto_home(
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        profile_token,
        &raw mut config.onvif_username as *mut ::core::ffi::c_char,
        &raw mut config.onvif_password as *mut ::core::ffi::c_char,
    );
    if rc != 0 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"goto_home\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"device_operation_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"PTZ go to home failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_ptz_operation(
        req,
        &raw mut auth,
        b"goto_home\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"completed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"PTZ moved to home position\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_ptz_set_home(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if extract_ptz_stream_name(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/streams/%s/ptz/sethome\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut auth: ptz_authorization_context_t = ptz_authorization_context_t {
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
        camera: fleet_camera_t {
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
        },
        evaluation: authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        },
    };
    if authorize_ptz_stream(
        req,
        res,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        authorization_action_t::AUTHZ_PTZ_CONTROL,
        &raw mut auth,
    ) == 0
    {
        return;
    }
    if (*req).body_len != 0 {
        let mut body: *mut cJSON = device_json_body(req);
        let mut empty: bool = device_keys(
            body,
            ::core::ptr::null::<*const ::core::ffi::c_char>(),
            0 as size_t,
        );
        cJSON_Delete(body);
        if !empty {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Expected empty device command\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    let mut config: stream_config_t = stream_config_t {
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
    let mut rc: ::core::ffi::c_int = get_ptz_stream_config(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut config,
    );
    if rc == -1 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"set_home\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"stream_configuration_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    } else if rc == -2 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"set_home\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"ptz_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"PTZ not enabled for this stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut ptz_url: [::core::ffi::c_char; 512] = [0; 512];
    if build_ptz_url(
        &raw mut config,
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"set_home\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"service_discovery_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build PTZ URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut profile_token: *const ::core::ffi::c_char = get_profile_token(&raw mut config);
    rc = onvif_ptz_set_home(
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        profile_token,
        &raw mut config.onvif_username as *mut ::core::ffi::c_char,
        &raw mut config.onvif_password as *mut ::core::ffi::c_char,
    );
    if rc != 0 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"set_home\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"device_operation_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"PTZ set home failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_ptz_operation(
        req,
        &raw mut auth,
        b"set_home\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"completed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"PTZ home position set\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_ptz_get_presets(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if extract_ptz_stream_name(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/streams/%s/ptz/presets\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut config: stream_config_t = stream_config_t {
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
    let mut rc: ::core::ffi::c_int = get_ptz_stream_config(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut config,
    );
    if rc == -1 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    } else if rc == -2 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"PTZ not enabled for this stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut auth: ptz_authorization_context_t = ptz_authorization_context_t {
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
        camera: fleet_camera_t {
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
        },
        evaluation: authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        },
    };
    if authorize_ptz_stream(
        req,
        res,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut auth,
    ) == 0
    {
        return;
    }
    let mut ptz_url: [::core::ffi::c_char; 512] = [0; 512];
    if build_ptz_url(
        &raw mut config,
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build PTZ URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut profile_token: *const ::core::ffi::c_char = get_profile_token(&raw mut config);
    let mut presets: [onvif_ptz_preset_t; 32] = [onvif_ptz_preset_t {
        token: [0; 64],
        name: [0; 128],
        pan: 0.,
        tilt: 0.,
        zoom: 0.,
    }; 32];
    let mut count: ::core::ffi::c_int = onvif_ptz_get_presets(
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        profile_token,
        &raw mut config.onvif_username as *mut ::core::ffi::c_char,
        &raw mut config.onvif_password as *mut ::core::ffi::c_char,
        &raw mut presets as *mut onvif_ptz_preset_t,
        32 as ::core::ffi::c_int,
    );
    if count < 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            502 as ::core::ffi::c_int,
            b"Failed to get ONVIF PTZ presets\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    let mut presets_array: *mut cJSON = cJSON_CreateArray();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut preset: *mut cJSON = cJSON_CreateObject();
        cJSON_AddStringToObject(
            preset,
            b"token\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut presets as *mut onvif_ptz_preset_t).offset(i as isize)).token
                as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            preset,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*(&raw mut presets as *mut onvif_ptz_preset_t).offset(i as isize)).name
                as *mut ::core::ffi::c_char,
        );
        cJSON_AddItemToArray(presets_array, preset);
        i += 1;
    }
    cJSON_AddItemToObject(
        response,
        b"presets\0".as_ptr() as *const ::core::ffi::c_char,
        presets_array,
    );
    cJSON_AddNumberToObject(
        response,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        count as ::core::ffi::c_double,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_ptz_goto_preset(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if extract_ptz_stream_name(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling POST /api/streams/%s/ptz/preset\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut auth: ptz_authorization_context_t = ptz_authorization_context_t {
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
        camera: fleet_camera_t {
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
        },
        evaluation: authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        },
    };
    if authorize_ptz_stream(
        req,
        res,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        authorization_action_t::AUTHZ_PTZ_CONTROL,
        &raw mut auth,
    ) == 0
    {
        return;
    }
    let mut config: stream_config_t = stream_config_t {
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
    let mut rc: ::core::ffi::c_int = get_ptz_stream_config(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut config,
    );
    if rc == -1 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"goto_preset\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"stream_configuration_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    } else if rc == -2 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"goto_preset\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"ptz_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"PTZ not enabled for this stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = device_json_body(req);
    if !valid_ptz_body(
        body,
        b"goto-preset\0".as_ptr() as *const ::core::ffi::c_char,
    ) {
        cJSON_Delete(body);
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"goto_preset\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut token_json: *mut cJSON =
        cJSON_GetObjectItem(body, b"token\0".as_ptr() as *const ::core::ffi::c_char);
    if token_json.is_null() || cJSON_IsString(token_json) == 0 {
        cJSON_Delete(body);
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"goto_preset\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"missing_preset_token\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing preset token\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut preset_token: [::core::ffi::c_char; 64] = [0; 64];
    safe_strcpy(
        &raw mut preset_token as *mut ::core::ffi::c_char,
        (*token_json).valuestring,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    cJSON_Delete(body);
    let mut ptz_url: [::core::ffi::c_char; 512] = [0; 512];
    if build_ptz_url(
        &raw mut config,
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"goto_preset\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"service_discovery_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build PTZ URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut profile_token: *const ::core::ffi::c_char = get_profile_token(&raw mut config);
    rc = onvif_ptz_goto_preset(
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        profile_token,
        &raw mut config.onvif_username as *mut ::core::ffi::c_char,
        &raw mut config.onvif_password as *mut ::core::ffi::c_char,
        &raw mut preset_token as *mut ::core::ffi::c_char,
    );
    if rc != 0 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"goto_preset\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"device_operation_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"PTZ go to preset failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_ptz_operation(
        req,
        &raw mut auth,
        b"goto_preset\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"completed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"PTZ moved to preset\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_ptz_set_preset(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if extract_ptz_stream_name(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling PUT /api/streams/%s/ptz/preset\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut auth: ptz_authorization_context_t = ptz_authorization_context_t {
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
        camera: fleet_camera_t {
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
        },
        evaluation: authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        },
    };
    if authorize_ptz_stream(
        req,
        res,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        authorization_action_t::AUTHZ_PTZ_CONTROL,
        &raw mut auth,
    ) == 0
    {
        return;
    }
    let mut config: stream_config_t = stream_config_t {
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
    let mut rc: ::core::ffi::c_int = get_ptz_stream_config(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut config,
    );
    if rc == -1 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"set_preset\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"stream_configuration_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    } else if rc == -2 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"set_preset\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"ptz_disabled\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"PTZ not enabled for this stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = device_json_body(req);
    if !valid_ptz_body(body, b"preset\0".as_ptr() as *const ::core::ffi::c_char) {
        cJSON_Delete(body);
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"set_preset\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"invalid_request\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut preset_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut name_json: *mut cJSON =
        cJSON_GetObjectItem(body, b"name\0".as_ptr() as *const ::core::ffi::c_char);
    if !name_json.is_null() && cJSON_IsString(name_json) != 0 {
        preset_name = (*name_json).valuestring;
    }
    let mut ptz_url: [::core::ffi::c_char; 512] = [0; 512];
    if build_ptz_url(
        &raw mut config,
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        cJSON_Delete(body);
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"set_preset\0".as_ptr() as *const ::core::ffi::c_char,
            b"error\0".as_ptr() as *const ::core::ffi::c_char,
            b"service_discovery_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build PTZ URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut profile_token: *const ::core::ffi::c_char = get_profile_token(&raw mut config);
    let mut new_token: [::core::ffi::c_char; 64] = [
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
    rc = onvif_ptz_set_preset(
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        profile_token,
        &raw mut config.onvif_username as *mut ::core::ffi::c_char,
        &raw mut config.onvif_password as *mut ::core::ffi::c_char,
        preset_name,
        &raw mut new_token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    );
    cJSON_Delete(body);
    if rc != 0 as ::core::ffi::c_int {
        audit_ptz_operation(
            req,
            &raw mut auth,
            b"set_preset\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            b"device_operation_failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"PTZ set preset failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_ptz_operation(
        req,
        &raw mut auth,
        b"set_preset\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        b"completed\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"PTZ preset created\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        response,
        b"token\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut new_token as *mut ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_ptz_capabilities(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if extract_ptz_stream_name(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream name\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Handling GET /api/streams/%s/ptz/capabilities\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut config: stream_config_t = stream_config_t {
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
    let mut rc: ::core::ffi::c_int = get_ptz_stream_config(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut config,
    );
    if rc == -1 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    } else if rc == -2 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"PTZ not enabled for this stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut auth: ptz_authorization_context_t = ptz_authorization_context_t {
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
        camera: fleet_camera_t {
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
        },
        evaluation: authorization_evaluation_t {
            decision: authorization_decision_t::AUTHZ_DECISION_DENY,
            source: authorization_decision_source_t::AUTHZ_SOURCE_NONE,
            policy_version: 0,
            grant_uuid: [0; 37],
            token_uuid: [0; 37],
            role_uuid: [0; 37],
            role_name: [0; 128],
            explanation: [0; 256],
        },
    };
    if authorize_ptz_stream(
        req,
        res,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut auth,
    ) == 0
    {
        return;
    }
    let mut ptz_url: [::core::ffi::c_char; 512] = [0; 512];
    if build_ptz_url(
        &raw mut config,
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build PTZ URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut profile_token: *const ::core::ffi::c_char = get_profile_token(&raw mut config);
    let mut caps: onvif_ptz_capabilities_t = onvif_ptz_capabilities_t {
        queried: false,
        has_pan_tilt: false,
        has_zoom: false,
        has_continuous_move: false,
        has_absolute_move: false,
        has_relative_move: false,
        has_home_position: false,
        has_presets: false,
        max_presets: 0,
        pan_min: 0.,
        pan_max: 0.,
        tilt_min: 0.,
        tilt_max: 0.,
        zoom_min: 0.,
        zoom_max: 0.,
    };
    rc = onvif_ptz_get_capabilities(
        &raw mut ptz_url as *mut ::core::ffi::c_char,
        profile_token,
        &raw mut config.onvif_username as *mut ::core::ffi::c_char,
        &raw mut config.onvif_password as *mut ::core::ffi::c_char,
        &raw mut caps,
    );
    if rc != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            502 as ::core::ffi::c_int,
            b"Failed to get ONVIF PTZ capabilities\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"ptz_enabled\0".as_ptr() as *const ::core::ffi::c_char,
        config.ptz_enabled as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        response,
        b"onvif_queried\0".as_ptr() as *const ::core::ffi::c_char,
        caps.queried as cJSON_bool,
    );
    cJSON_AddStringToObject(
        response,
        b"ptz_service_url\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut ptz_url as *mut ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        response,
        b"has_pan_tilt\0".as_ptr() as *const ::core::ffi::c_char,
        caps.has_pan_tilt as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        response,
        b"has_zoom\0".as_ptr() as *const ::core::ffi::c_char,
        caps.has_zoom as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        response,
        b"has_continuous_move\0".as_ptr() as *const ::core::ffi::c_char,
        caps.has_continuous_move as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        response,
        b"has_absolute_move\0".as_ptr() as *const ::core::ffi::c_char,
        caps.has_absolute_move as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        response,
        b"has_relative_move\0".as_ptr() as *const ::core::ffi::c_char,
        caps.has_relative_move as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        response,
        b"has_home_position\0".as_ptr() as *const ::core::ffi::c_char,
        caps.has_home_position as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        response,
        b"has_presets\0".as_ptr() as *const ::core::ffi::c_char,
        caps.has_presets as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        response,
        b"max_presets\0".as_ptr() as *const ::core::ffi::c_char,
        caps.max_presets as ::core::ffi::c_double,
    );
    let mut pan_range: *mut cJSON = cJSON_CreateObject();
    cJSON_AddNumberToObject(
        pan_range,
        b"min\0".as_ptr() as *const ::core::ffi::c_char,
        caps.pan_min as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        pan_range,
        b"max\0".as_ptr() as *const ::core::ffi::c_char,
        caps.pan_max as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        response,
        b"pan_range\0".as_ptr() as *const ::core::ffi::c_char,
        pan_range,
    );
    let mut tilt_range: *mut cJSON = cJSON_CreateObject();
    cJSON_AddNumberToObject(
        tilt_range,
        b"min\0".as_ptr() as *const ::core::ffi::c_char,
        caps.tilt_min as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        tilt_range,
        b"max\0".as_ptr() as *const ::core::ffi::c_char,
        caps.tilt_max as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        response,
        b"tilt_range\0".as_ptr() as *const ::core::ffi::c_char,
        tilt_range,
    );
    let mut zoom_range: *mut cJSON = cJSON_CreateObject();
    cJSON_AddNumberToObject(
        zoom_range,
        b"min\0".as_ptr() as *const ::core::ffi::c_char,
        caps.zoom_min as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        zoom_range,
        b"max\0".as_ptr() as *const ::core::ffi::c_char,
        caps.zoom_max as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        response,
        b"zoom_range\0".as_ptr() as *const ::core::ffi::c_char,
        zoom_range,
    );
    cJSON_AddNumberToObject(
        response,
        b"max_x\0".as_ptr() as *const ::core::ffi::c_char,
        config.ptz_max_x as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        response,
        b"max_y\0".as_ptr() as *const ::core::ffi::c_char,
        config.ptz_max_y as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        response,
        b"max_z\0".as_ptr() as *const ::core::ffi::c_char,
        config.ptz_max_z as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        response,
        b"has_home\0".as_ptr() as *const ::core::ffi::c_char,
        config.ptz_has_home as cJSON_bool,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
    free(json_str as *mut ::core::ffi::c_void);
}
pub const __FLT_MAX__: ::core::ffi::c_float = 3.40282347e+38f32;
