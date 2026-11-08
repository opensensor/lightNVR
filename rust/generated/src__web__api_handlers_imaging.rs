unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
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
    unsafe fn cJSON_ParseWithLengthOpts(
        value: *const ::core::ffi::c_char,
        buffer_length: size_t,
        return_parse_end: *mut *const ::core::ffi::c_char,
        require_null_terminated: cJSON_bool,
    ) -> *mut cJSON;
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
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_CreateString(string: *const ::core::ffi::c_char) -> *mut cJSON;
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
    unsafe fn cJSON_AddObjectToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_AddArrayToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn httpd_authorize_stream_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    unsafe fn get_onvif_video_source_token(
        device_url: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        profile_token: *const ::core::ffi::c_char,
        video_source_token: *mut ::core::ffi::c_char,
        token_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_imaging_get_settings(
        imaging_url: *const ::core::ffi::c_char,
        video_source_token: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        settings: *mut onvif_imaging_settings_t,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_imaging_get_options(
        imaging_url: *const ::core::ffi::c_char,
        video_source_token: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        options: *mut onvif_imaging_options_t,
    ) -> ::core::ffi::c_int;
    unsafe fn onvif_imaging_set_settings(
        imaging_url: *const ::core::ffi::c_char,
        video_source_token: *const ::core::ffi::c_char,
        username: *const ::core::ffi::c_char,
        password: *const ::core::ffi::c_char,
        settings: *const onvif_imaging_settings_t,
        force_persistence: bool,
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
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
pub struct onvif_imaging_settings_t {
    pub video_source_token: [::core::ffi::c_char; 64],
    pub brightness: onvif_imaging_float_t,
    pub color_saturation: onvif_imaging_float_t,
    pub contrast: onvif_imaging_float_t,
    pub sharpness: onvif_imaging_float_t,
    pub noise_reduction: onvif_imaging_float_t,
    pub backlight_compensation: onvif_imaging_mode_level_t,
    pub wide_dynamic_range: onvif_imaging_mode_level_t,
    pub tone_compensation: onvif_imaging_mode_level_t,
    pub defogging: onvif_imaging_mode_level_t,
    pub has_ir_cut_filter: bool,
    pub ir_cut_filter: [::core::ffi::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_imaging_mode_level_t {
    pub present: bool,
    pub mode: [::core::ffi::c_char; 16],
    pub has_level: bool,
    pub level: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_imaging_float_t {
    pub present: bool,
    pub value: ::core::ffi::c_float,
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_imaging_options_t {
    pub video_source_token: [::core::ffi::c_char; 64],
    pub brightness: onvif_imaging_float_range_t,
    pub color_saturation: onvif_imaging_float_range_t,
    pub contrast: onvif_imaging_float_range_t,
    pub sharpness: onvif_imaging_float_range_t,
    pub noise_reduction: onvif_imaging_float_range_t,
    pub backlight_compensation: onvif_imaging_mode_level_options_t,
    pub wide_dynamic_range: onvif_imaging_mode_level_options_t,
    pub tone_compensation: onvif_imaging_mode_level_options_t,
    pub defogging: onvif_imaging_mode_level_options_t,
    pub ir_cut_filter_modes: [[::core::ffi::c_char; 16]; 4],
    pub ir_cut_filter_mode_count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_imaging_mode_level_options_t {
    pub present: bool,
    pub modes: [[::core::ffi::c_char; 16]; 4],
    pub mode_count: ::core::ffi::c_int,
    pub level: onvif_imaging_float_range_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct onvif_imaging_float_range_t {
    pub present: bool,
    pub min: ::core::ffi::c_float,
    pub max: ::core::ffi::c_float,
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
pub const LOG_COMPONENT: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"ImagingAPI\0") };
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FLT_MAX: ::core::ffi::c_float = __FLT_MAX__;
unsafe extern "C" fn extract_imaging_stream_name(
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
    let mut suffix: *mut ::core::ffi::c_char = strstr(
        stream_name,
        b"/imaging\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if suffix.is_null() {
        suffix = strstr(
            stream_name,
            b"/daynight\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !suffix.is_null() {
        *suffix = '\0' as ::core::ffi::c_char;
    }
    return if *stream_name.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        -1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn get_imaging_stream_config(
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
    if !(*config).is_onvif {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_ERROR,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"ONVIF is not enabled for stream: %s\0".as_ptr() as *const ::core::ffi::c_char,
            stream_name,
        );
        return -2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn build_imaging_url(
    mut config: *const stream_config_t,
    mut imaging_url: *mut ::core::ffi::c_char,
    mut url_size: size_t,
) -> ::core::ffi::c_int {
    return url_build_onvif_service_url(
        &raw const (*config).url as *const ::core::ffi::c_char,
        (*config).onvif_port,
        b"/onvif/imaging_service\0".as_ptr() as *const ::core::ffi::c_char,
        imaging_url,
        url_size,
    );
}
unsafe extern "C" fn build_device_url(
    mut config: *const stream_config_t,
    mut device_url: *mut ::core::ffi::c_char,
    mut url_size: size_t,
) -> ::core::ffi::c_int {
    return url_build_onvif_service_url(
        &raw const (*config).url as *const ::core::ffi::c_char,
        (*config).onvif_port,
        b"/onvif/device_service\0".as_ptr() as *const ::core::ffi::c_char,
        device_url,
        url_size,
    );
}
unsafe extern "C" fn config_username(
    mut config: *const stream_config_t,
) -> *const ::core::ffi::c_char {
    return if (*config).onvif_username[0usize] as ::core::ffi::c_int != 0 {
        &raw const (*config).onvif_username as *const ::core::ffi::c_char
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
unsafe extern "C" fn config_password(
    mut config: *const stream_config_t,
) -> *const ::core::ffi::c_char {
    return if (*config).onvif_password[0usize] as ::core::ffi::c_int != 0 {
        &raw const (*config).onvif_password as *const ::core::ffi::c_char
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    };
}
unsafe extern "C" fn resolve_video_source_token(
    mut config: *const stream_config_t,
    mut video_source_token: *mut ::core::ffi::c_char,
    mut token_size: size_t,
) -> ::core::ffi::c_int {
    let mut device_url: [::core::ffi::c_char; 512] = [0; 512];
    if build_device_url(
        config,
        &raw mut device_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) == 0 as ::core::ffi::c_int
    {
        if get_onvif_video_source_token(
            &raw mut device_url as *mut ::core::ffi::c_char,
            config_username(config),
            config_password(config),
            &raw const (*config).onvif_profile as *const ::core::ffi::c_char,
            video_source_token,
            token_size,
        ) == 0 as ::core::ffi::c_int
            && *video_source_token.offset(0isize) as ::core::ffi::c_int
                != '\0' as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*config).onvif_profile[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Falling back to ONVIF profile token as imaging VideoSourceToken for stream %s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw const (*config).name as *const ::core::ffi::c_char,
        );
        safe_strcpy(
            video_source_token,
            &raw const (*config).onvif_profile as *const ::core::ffi::c_char,
            token_size,
            0 as size_t,
        );
        return 0 as ::core::ffi::c_int;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_WARN,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"Falling back to default Thingino/onvif_simple_server VideoSourceToken for stream %s\0"
            .as_ptr() as *const ::core::ffi::c_char,
        &raw const (*config).name as *const ::core::ffi::c_char,
    );
    safe_strcpy(
        video_source_token,
        b"VideoSourceToken\0".as_ptr() as *const ::core::ffi::c_char,
        token_size,
        0 as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn add_float_setting(
    mut root: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut setting: *const onvif_imaging_float_t,
) {
    if !setting.is_null() && (*setting).present as ::core::ffi::c_int != 0 {
        cJSON_AddNumberToObject(root, name, (*setting).value as ::core::ffi::c_double);
    }
}
unsafe extern "C" fn add_mode_level_setting(
    mut root: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut setting: *const onvif_imaging_mode_level_t,
) {
    if setting.is_null() || !(*setting).present {
        return;
    }
    let mut obj: *mut cJSON = cJSON_AddObjectToObject(root, name);
    if obj.is_null() {
        return;
    }
    if (*setting).mode[0usize] as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        cJSON_AddStringToObject(
            obj,
            b"mode\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*setting).mode as *const ::core::ffi::c_char,
        );
    }
    if (*setting).has_level {
        cJSON_AddNumberToObject(
            obj,
            b"level\0".as_ptr() as *const ::core::ffi::c_char,
            (*setting).level as ::core::ffi::c_double,
        );
    }
}
unsafe extern "C" fn settings_to_json(mut settings: *const onvif_imaging_settings_t) -> *mut cJSON {
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        root,
        b"video_source_token\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*settings).video_source_token as *const ::core::ffi::c_char,
    );
    add_float_setting(
        root,
        b"brightness\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*settings).brightness,
    );
    add_float_setting(
        root,
        b"color_saturation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*settings).color_saturation,
    );
    add_float_setting(
        root,
        b"contrast\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*settings).contrast,
    );
    add_float_setting(
        root,
        b"sharpness\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*settings).sharpness,
    );
    add_float_setting(
        root,
        b"noise_reduction\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*settings).noise_reduction,
    );
    add_mode_level_setting(
        root,
        b"backlight_compensation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*settings).backlight_compensation,
    );
    add_mode_level_setting(
        root,
        b"wide_dynamic_range\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*settings).wide_dynamic_range,
    );
    add_mode_level_setting(
        root,
        b"tone_compensation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*settings).tone_compensation,
    );
    add_mode_level_setting(
        root,
        b"defogging\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*settings).defogging,
    );
    if (*settings).has_ir_cut_filter {
        cJSON_AddStringToObject(
            root,
            b"ir_cut_filter\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*settings).ir_cut_filter as *const ::core::ffi::c_char,
        );
    }
    return root;
}
unsafe extern "C" fn add_float_range(
    mut root: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut range: *const onvif_imaging_float_range_t,
) {
    if range.is_null() || !(*range).present {
        return;
    }
    let mut obj: *mut cJSON = cJSON_AddObjectToObject(root, name);
    if obj.is_null() {
        return;
    }
    cJSON_AddNumberToObject(
        obj,
        b"min\0".as_ptr() as *const ::core::ffi::c_char,
        (*range).min as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        obj,
        b"max\0".as_ptr() as *const ::core::ffi::c_char,
        (*range).max as ::core::ffi::c_double,
    );
}
unsafe extern "C" fn add_mode_level_options(
    mut root: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut options: *const onvif_imaging_mode_level_options_t,
) {
    if options.is_null() || !(*options).present {
        return;
    }
    let mut obj: *mut cJSON = cJSON_AddObjectToObject(root, name);
    if obj.is_null() {
        return;
    }
    let mut modes: *mut cJSON =
        cJSON_AddArrayToObject(obj, b"modes\0".as_ptr() as *const ::core::ffi::c_char);
    if !modes.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*options).mode_count {
            cJSON_AddItemToArray(
                modes,
                cJSON_CreateString(
                    &raw const *(&raw const (*options).modes as *const [::core::ffi::c_char; 16])
                        .offset(i as isize) as *const ::core::ffi::c_char,
                ),
            );
            i += 1;
        }
    }
    add_float_range(
        obj,
        b"level\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*options).level,
    );
}
unsafe extern "C" fn options_to_json(mut options: *const onvif_imaging_options_t) -> *mut cJSON {
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        root,
        b"video_source_token\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*options).video_source_token as *const ::core::ffi::c_char,
    );
    add_float_range(
        root,
        b"brightness\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*options).brightness,
    );
    add_float_range(
        root,
        b"color_saturation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*options).color_saturation,
    );
    add_float_range(
        root,
        b"contrast\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*options).contrast,
    );
    add_float_range(
        root,
        b"sharpness\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*options).sharpness,
    );
    add_float_range(
        root,
        b"noise_reduction\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*options).noise_reduction,
    );
    add_mode_level_options(
        root,
        b"backlight_compensation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*options).backlight_compensation,
    );
    add_mode_level_options(
        root,
        b"wide_dynamic_range\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*options).wide_dynamic_range,
    );
    add_mode_level_options(
        root,
        b"tone_compensation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*options).tone_compensation,
    );
    add_mode_level_options(
        root,
        b"defogging\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*options).defogging,
    );
    let mut ir_modes: *mut cJSON = cJSON_AddArrayToObject(
        root,
        b"ir_cut_filter_modes\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !ir_modes.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*options).ir_cut_filter_mode_count {
            cJSON_AddItemToArray(
                ir_modes,
                cJSON_CreateString(
                    &raw const *(&raw const (*options).ir_cut_filter_modes
                        as *const [::core::ffi::c_char; 16])
                        .offset(i as isize) as *const ::core::ffi::c_char,
                ),
            );
            i += 1;
        }
    }
    return root;
}
unsafe extern "C" fn send_json_object(
    mut res: *mut http_response_t,
    mut root: *mut cJSON,
) -> ::core::ffi::c_int {
    if root.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut json: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if json.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to generate JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json);
    free(json as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn read_number(
    mut body: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut value: *mut ::core::ffi::c_float,
) -> bool {
    let mut item: *mut cJSON = cJSON_GetObjectItem(body, name);
    if !item.is_null() && cJSON_IsNumber(item) != 0 {
        *value = (*item).valuedouble as ::core::ffi::c_float;
        return r#true != 0;
    }
    return r#false != 0;
}
unsafe extern "C" fn read_string(
    mut body: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut value: *mut ::core::ffi::c_char,
    mut value_size: size_t,
) -> bool {
    let mut item: *mut cJSON = cJSON_GetObjectItem(body, name);
    if !item.is_null() && cJSON_IsString(item) != 0 {
        safe_strcpy(value, (*item).valuestring, value_size, 0 as size_t);
        return r#true != 0;
    }
    return r#false != 0;
}
unsafe extern "C" fn canonicalize_ir_cut_filter(
    mut value: *mut ::core::ffi::c_char,
    mut value_size: size_t,
) {
    if value.is_null() || value_size == 0 as size_t {
        return;
    }
    if strcasecmp(value, b"on\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            value,
            b"On\0".as_ptr() as *const ::core::ffi::c_char,
            value_size,
            0 as size_t,
        );
    } else if strcasecmp(value, b"off\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            value,
            b"Off\0".as_ptr() as *const ::core::ffi::c_char,
            value_size,
            0 as size_t,
        );
    } else if strcasecmp(value, b"auto\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            value,
            b"Auto\0".as_ptr() as *const ::core::ffi::c_char,
            value_size,
            0 as size_t,
        );
    }
}
unsafe extern "C" fn valid_ir_cut_filter(mut value: *const ::core::ffi::c_char) -> bool {
    return !value.is_null()
        && (strcmp(value, b"On\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcmp(value, b"Off\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || strcmp(value, b"Auto\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn ir_cut_filter_to_daynight_mode(
    mut ir_cut_filter: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if ir_cut_filter.is_null() {
        return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(
        ir_cut_filter,
        b"Auto\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return b"auto\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(
        ir_cut_filter,
        b"On\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return b"day\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strcasecmp(
        ir_cut_filter,
        b"Off\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return b"night\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn daynight_mode_to_ir_cut_filter(
    mut mode: *const ::core::ffi::c_char,
    mut ir_cut_filter: *mut ::core::ffi::c_char,
    mut value_size: size_t,
) -> bool {
    if mode.is_null() || ir_cut_filter.is_null() || value_size == 0 as size_t {
        return r#false != 0;
    }
    if strcasecmp(mode, b"auto\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            ir_cut_filter,
            b"Auto\0".as_ptr() as *const ::core::ffi::c_char,
            value_size,
            0 as size_t,
        );
        return r#true != 0;
    }
    if strcasecmp(mode, b"day\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        || strcasecmp(mode, b"color\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            ir_cut_filter,
            b"On\0".as_ptr() as *const ::core::ffi::c_char,
            value_size,
            0 as size_t,
        );
        return r#true != 0;
    }
    if strcasecmp(mode, b"night\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcasecmp(mode, b"bw\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcasecmp(mode, b"blackwhite\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        safe_strcpy(
            ir_cut_filter,
            b"Off\0".as_ptr() as *const ::core::ffi::c_char,
            value_size,
            0 as size_t,
        );
        return r#true != 0;
    }
    safe_strcpy(ir_cut_filter, mode, value_size, 0 as size_t);
    canonicalize_ir_cut_filter(ir_cut_filter, value_size);
    return valid_ir_cut_filter(ir_cut_filter);
}
unsafe extern "C" fn add_supported_daynight_mode(
    mut array: *mut cJSON,
    mut ir_cut_filter: *const ::core::ffi::c_char,
) {
    if array.is_null() || ir_cut_filter.is_null() {
        return;
    }
    let mut mode: *const ::core::ffi::c_char = ir_cut_filter_to_daynight_mode(ir_cut_filter);
    if strcmp(mode, b"unknown\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        return;
    }
    let mut count: ::core::ffi::c_int = cJSON_GetArraySize(array);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut existing: *mut cJSON = cJSON_GetArrayItem(array, i);
        if !existing.is_null()
            && cJSON_IsString(existing) != 0
            && strcmp((*existing).valuestring, mode) == 0 as ::core::ffi::c_int
        {
            return;
        }
        i += 1;
    }
    cJSON_AddItemToArray(array, cJSON_CreateString(mode));
}
unsafe extern "C" fn add_default_supported_daynight_modes(mut array: *mut cJSON) {
    if array.is_null() || cJSON_GetArraySize(array) > 0 as ::core::ffi::c_int {
        return;
    }
    cJSON_AddItemToArray(
        array,
        cJSON_CreateString(b"day\0".as_ptr() as *const ::core::ffi::c_char),
    );
    cJSON_AddItemToArray(
        array,
        cJSON_CreateString(b"night\0".as_ptr() as *const ::core::ffi::c_char),
    );
    cJSON_AddItemToArray(
        array,
        cJSON_CreateString(b"auto\0".as_ptr() as *const ::core::ffi::c_char),
    );
}
unsafe extern "C" fn read_mode_level(
    mut body: *mut cJSON,
    mut name: *const ::core::ffi::c_char,
    mut setting: *mut onvif_imaging_mode_level_t,
) -> bool {
    let mut item: *mut cJSON = cJSON_GetObjectItem(body, name);
    if item.is_null() || setting.is_null() {
        return r#false != 0;
    }
    if cJSON_IsNumber(item) != 0 {
        (*setting).present = r#true != 0;
        (*setting).has_level = r#true != 0;
        (*setting).level = (*item).valuedouble as ::core::ffi::c_float;
        return r#true != 0;
    }
    if cJSON_IsObject(item) == 0 {
        return r#false != 0;
    }
    let mut changed: bool = r#false != 0;
    let mut mode: *mut cJSON =
        cJSON_GetObjectItem(item, b"mode\0".as_ptr() as *const ::core::ffi::c_char);
    if !mode.is_null() && cJSON_IsString(mode) != 0 {
        safe_strcpy(
            &raw mut (*setting).mode as *mut ::core::ffi::c_char,
            (*mode).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
        changed = r#true != 0;
    }
    let mut level: *mut cJSON =
        cJSON_GetObjectItem(item, b"level\0".as_ptr() as *const ::core::ffi::c_char);
    if !level.is_null() && cJSON_IsNumber(level) != 0 {
        (*setting).has_level = r#true != 0;
        (*setting).level = (*level).valuedouble as ::core::ffi::c_float;
        changed = r#true != 0;
    }
    (*setting).present = changed;
    return changed;
}
unsafe extern "C" fn valid_imaging_update(mut body: *const cJSON, mut daynight: bool) -> bool {
    static mut keys: [*const ::core::ffi::c_char; 17] = [
        b"brightness\0".as_ptr() as *const ::core::ffi::c_char,
        b"color_saturation\0".as_ptr() as *const ::core::ffi::c_char,
        b"saturation\0".as_ptr() as *const ::core::ffi::c_char,
        b"contrast\0".as_ptr() as *const ::core::ffi::c_char,
        b"sharpness\0".as_ptr() as *const ::core::ffi::c_char,
        b"noise_reduction\0".as_ptr() as *const ::core::ffi::c_char,
        b"backlight_compensation\0".as_ptr() as *const ::core::ffi::c_char,
        b"backlight\0".as_ptr() as *const ::core::ffi::c_char,
        b"wide_dynamic_range\0".as_ptr() as *const ::core::ffi::c_char,
        b"wdr\0".as_ptr() as *const ::core::ffi::c_char,
        b"tone_compensation\0".as_ptr() as *const ::core::ffi::c_char,
        b"tone\0".as_ptr() as *const ::core::ffi::c_char,
        b"defogging\0".as_ptr() as *const ::core::ffi::c_char,
        b"defog\0".as_ptr() as *const ::core::ffi::c_char,
        b"ir_cut_filter\0".as_ptr() as *const ::core::ffi::c_char,
        b"ircut\0".as_ptr() as *const ::core::ffi::c_char,
        b"force_persistence\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    static mut daykeys: [*const ::core::ffi::c_char; 5] = [
        b"mode\0".as_ptr() as *const ::core::ffi::c_char,
        b"daynight\0".as_ptr() as *const ::core::ffi::c_char,
        b"ir_cut_filter\0".as_ptr() as *const ::core::ffi::c_char,
        b"ircut\0".as_ptr() as *const ::core::ffi::c_char,
        b"force_persistence\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if !device_keys(
        body,
        if daynight as ::core::ffi::c_int != 0 {
            &raw mut daykeys as *mut *const ::core::ffi::c_char
        } else {
            &raw mut keys as *mut *const ::core::ffi::c_char
        },
        (if daynight as ::core::ffi::c_int != 0 {
            5 as ::core::ffi::c_int
        } else {
            17 as ::core::ffi::c_int
        }) as size_t,
    ) {
        return r#false != 0;
    }
    let mut field: *const cJSON = ::core::ptr::null::<cJSON>();
    field = if !body.is_null() {
        (*body).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !field.is_null() {
        let mut key: *const ::core::ffi::c_char = (*field).string;
        if strcmp(
            key,
            b"force_persistence\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        {
            if cJSON_IsBool(field) == 0 {
                return r#false != 0;
            }
        } else if daynight as ::core::ffi::c_int != 0
            || strcmp(
                key,
                b"ir_cut_filter\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(key, b"ircut\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            if !device_string(field, 15 as size_t, r#false != 0) {
                return r#false != 0;
            }
        } else if strcmp(key, b"brightness\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(
                key,
                b"color_saturation\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
            || strcmp(key, b"saturation\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"contrast\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(key, b"sharpness\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(
                key,
                b"noise_reduction\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0
        {
            if !device_number(field) {
                return r#false != 0;
            }
        } else if !device_number(field) {
            static mut nested: [*const ::core::ffi::c_char; 2] = [
                b"mode\0".as_ptr() as *const ::core::ffi::c_char,
                b"level\0".as_ptr() as *const ::core::ffi::c_char,
            ];
            if !device_keys(
                field,
                &raw mut nested as *mut *const ::core::ffi::c_char,
                2 as size_t,
            ) || (*field).child.is_null()
            {
                return r#false != 0;
            }
            let mut mode: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                field,
                b"mode\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut level: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                field,
                b"level\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !mode.is_null() && !device_string(mode, 31 as size_t, r#false != 0)
                || !level.is_null() && !device_number(level)
            {
                return r#false != 0;
            }
        }
        field = (*field).next;
    }
    return r#true != 0;
}
unsafe extern "C" fn parse_settings_update(
    mut body: *mut cJSON,
    mut settings: *mut onvif_imaging_settings_t,
    mut force_persistence: *mut bool,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
) -> bool {
    let mut changed: bool = r#false != 0;
    let mut value: ::core::ffi::c_float = 0.0f32;
    memset(
        settings as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<onvif_imaging_settings_t>(),
    );
    *force_persistence = r#true != 0;
    if read_number(
        body,
        b"brightness\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value,
    ) {
        (*settings).brightness.present = r#true != 0;
        (*settings).brightness.value = value;
        changed = r#true != 0;
    }
    if read_number(
        body,
        b"color_saturation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value,
    ) as ::core::ffi::c_int
        != 0
        || read_number(
            body,
            b"saturation\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut value,
        ) as ::core::ffi::c_int
            != 0
    {
        (*settings).color_saturation.present = r#true != 0;
        (*settings).color_saturation.value = value;
        changed = r#true != 0;
    }
    if read_number(
        body,
        b"contrast\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value,
    ) {
        (*settings).contrast.present = r#true != 0;
        (*settings).contrast.value = value;
        changed = r#true != 0;
    }
    if read_number(
        body,
        b"sharpness\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value,
    ) {
        (*settings).sharpness.present = r#true != 0;
        (*settings).sharpness.value = value;
        changed = r#true != 0;
    }
    if read_number(
        body,
        b"noise_reduction\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value,
    ) {
        (*settings).noise_reduction.present = r#true != 0;
        (*settings).noise_reduction.value = value;
        changed = r#true != 0;
    }
    changed = read_mode_level(
        body,
        b"backlight_compensation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).backlight_compensation,
    ) as ::core::ffi::c_int
        != 0
        || changed as ::core::ffi::c_int != 0;
    changed = read_mode_level(
        body,
        b"backlight\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).backlight_compensation,
    ) as ::core::ffi::c_int
        != 0
        || changed as ::core::ffi::c_int != 0;
    changed = read_mode_level(
        body,
        b"wide_dynamic_range\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).wide_dynamic_range,
    ) as ::core::ffi::c_int
        != 0
        || changed as ::core::ffi::c_int != 0;
    changed = read_mode_level(
        body,
        b"wdr\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).wide_dynamic_range,
    ) as ::core::ffi::c_int
        != 0
        || changed as ::core::ffi::c_int != 0;
    changed = read_mode_level(
        body,
        b"tone_compensation\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).tone_compensation,
    ) as ::core::ffi::c_int
        != 0
        || changed as ::core::ffi::c_int != 0;
    changed = read_mode_level(
        body,
        b"tone\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).tone_compensation,
    ) as ::core::ffi::c_int
        != 0
        || changed as ::core::ffi::c_int != 0;
    changed = read_mode_level(
        body,
        b"defogging\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).defogging,
    ) as ::core::ffi::c_int
        != 0
        || changed as ::core::ffi::c_int != 0;
    changed = read_mode_level(
        body,
        b"defog\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).defogging,
    ) as ::core::ffi::c_int
        != 0
        || changed as ::core::ffi::c_int != 0;
    if read_string(
        body,
        b"ir_cut_filter\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*settings).ir_cut_filter as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
    ) as ::core::ffi::c_int
        != 0
        || read_string(
            body,
            b"ircut\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*settings).ir_cut_filter as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        ) as ::core::ffi::c_int
            != 0
    {
        canonicalize_ir_cut_filter(
            &raw mut (*settings).ir_cut_filter as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        );
        if !valid_ir_cut_filter(&raw mut (*settings).ir_cut_filter as *mut ::core::ffi::c_char) {
            snprintf(
                error,
                error_size,
                b"ir_cut_filter must be On, Off, or Auto\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*settings).has_ir_cut_filter = r#true != 0;
        changed = r#true != 0;
    }
    let mut force: *mut cJSON = cJSON_GetObjectItem(
        body,
        b"force_persistence\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !force.is_null() && cJSON_IsBool(force) != 0 {
        *force_persistence = cJSON_IsTrue(force) != 0;
    }
    if !changed {
        snprintf(
            error,
            error_size,
            b"No imaging settings provided\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn prepare_imaging_request(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut action: authorization_action_t,
    mut config: *mut stream_config_t,
    mut imaging_url: *mut ::core::ffi::c_char,
    mut imaging_url_size: size_t,
    mut video_source_token: *mut ::core::ffi::c_char,
    mut token_size: size_t,
    mut discover: bool,
) -> ::core::ffi::c_int {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if extract_imaging_stream_name(
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
        return -1 as ::core::ffi::c_int;
    }
    if httpd_authorize_stream_action(
        req,
        res,
        action,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    ) == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    let mut rc: ::core::ffi::c_int =
        get_imaging_stream_config(&raw mut stream_name as *mut ::core::ffi::c_char, config);
    if rc == -1 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Stream not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    } else if rc == -2 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"ONVIF is not enabled for this stream\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if build_imaging_url(config, imaging_url, imaging_url_size) != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to build ONVIF Imaging URL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if discover as ::core::ffi::c_int != 0
        && resolve_video_source_token(config, video_source_token, token_size)
            != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to resolve ONVIF video source token\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_imaging_get_settings(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
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
    let mut imaging_url: [::core::ffi::c_char; 512] = [0; 512];
    let mut video_source_token: [::core::ffi::c_char; 64] = [0; 64];
    if prepare_imaging_request(
        req,
        res,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut config,
        &raw mut imaging_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        &raw mut video_source_token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        r#true != 0,
    ) != 0 as ::core::ffi::c_int
    {
        return;
    }
    let mut settings: onvif_imaging_settings_t = onvif_imaging_settings_t {
        video_source_token: [0; 64],
        brightness: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        color_saturation: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        contrast: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        sharpness: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        noise_reduction: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        backlight_compensation: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        wide_dynamic_range: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        tone_compensation: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        defogging: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        has_ir_cut_filter: false,
        ir_cut_filter: [0; 16],
    };
    if onvif_imaging_get_settings(
        &raw mut imaging_url as *mut ::core::ffi::c_char,
        &raw mut video_source_token as *mut ::core::ffi::c_char,
        config_username(&raw mut config),
        config_password(&raw mut config),
        &raw mut settings,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            502 as ::core::ffi::c_int,
            b"Failed to get ONVIF imaging settings\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut settings_json: *mut cJSON = settings_to_json(&raw mut settings);
    if settings_json.is_null() {
        cJSON_Delete(root);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        root,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddItemToObject(
        root,
        b"settings\0".as_ptr() as *const ::core::ffi::c_char,
        settings_json,
    );
    send_json_object(res, root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_imaging_get_options(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
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
    let mut imaging_url: [::core::ffi::c_char; 512] = [0; 512];
    let mut video_source_token: [::core::ffi::c_char; 64] = [0; 64];
    if prepare_imaging_request(
        req,
        res,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut config,
        &raw mut imaging_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        &raw mut video_source_token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        r#true != 0,
    ) != 0 as ::core::ffi::c_int
    {
        return;
    }
    let mut options: onvif_imaging_options_t = onvif_imaging_options_t {
        video_source_token: [0; 64],
        brightness: onvif_imaging_float_range_t {
            present: false,
            min: 0.,
            max: 0.,
        },
        color_saturation: onvif_imaging_float_range_t {
            present: false,
            min: 0.,
            max: 0.,
        },
        contrast: onvif_imaging_float_range_t {
            present: false,
            min: 0.,
            max: 0.,
        },
        sharpness: onvif_imaging_float_range_t {
            present: false,
            min: 0.,
            max: 0.,
        },
        noise_reduction: onvif_imaging_float_range_t {
            present: false,
            min: 0.,
            max: 0.,
        },
        backlight_compensation: onvif_imaging_mode_level_options_t {
            present: false,
            modes: [[0; 16]; 4],
            mode_count: 0,
            level: onvif_imaging_float_range_t {
                present: false,
                min: 0.,
                max: 0.,
            },
        },
        wide_dynamic_range: onvif_imaging_mode_level_options_t {
            present: false,
            modes: [[0; 16]; 4],
            mode_count: 0,
            level: onvif_imaging_float_range_t {
                present: false,
                min: 0.,
                max: 0.,
            },
        },
        tone_compensation: onvif_imaging_mode_level_options_t {
            present: false,
            modes: [[0; 16]; 4],
            mode_count: 0,
            level: onvif_imaging_float_range_t {
                present: false,
                min: 0.,
                max: 0.,
            },
        },
        defogging: onvif_imaging_mode_level_options_t {
            present: false,
            modes: [[0; 16]; 4],
            mode_count: 0,
            level: onvif_imaging_float_range_t {
                present: false,
                min: 0.,
                max: 0.,
            },
        },
        ir_cut_filter_modes: [[0; 16]; 4],
        ir_cut_filter_mode_count: 0,
    };
    if onvif_imaging_get_options(
        &raw mut imaging_url as *mut ::core::ffi::c_char,
        &raw mut video_source_token as *mut ::core::ffi::c_char,
        config_username(&raw mut config),
        config_password(&raw mut config),
        &raw mut options,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            502 as ::core::ffi::c_int,
            b"Failed to get ONVIF imaging options\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut options_json: *mut cJSON = options_to_json(&raw mut options);
    if options_json.is_null() {
        cJSON_Delete(root);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        root,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddItemToObject(
        root,
        b"options\0".as_ptr() as *const ::core::ffi::c_char,
        options_json,
    );
    send_json_object(res, root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_imaging_put_settings(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
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
    let mut imaging_url: [::core::ffi::c_char; 512] = [0; 512];
    let mut video_source_token: [::core::ffi::c_char; 64] = [0; 64];
    if prepare_imaging_request(
        req,
        res,
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        &raw mut config,
        &raw mut imaging_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        &raw mut video_source_token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        r#false != 0,
    ) != 0 as ::core::ffi::c_int
    {
        return;
    }
    let mut body: *mut cJSON = device_json_body(req);
    if !valid_imaging_update(body, r#false != 0) {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut settings: onvif_imaging_settings_t = onvif_imaging_settings_t {
        video_source_token: [0; 64],
        brightness: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        color_saturation: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        contrast: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        sharpness: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        noise_reduction: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        backlight_compensation: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        wide_dynamic_range: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        tone_compensation: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        defogging: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        has_ir_cut_filter: false,
        ir_cut_filter: [0; 16],
    };
    let mut force_persistence: bool = r#true != 0;
    let mut error: [::core::ffi::c_char; 128] = [
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
    if !parse_settings_update(
        body,
        &raw mut settings,
        &raw mut force_persistence,
        &raw mut error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
    ) {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            if error[0usize] as ::core::ffi::c_int != 0 {
                &raw mut error as *mut ::core::ffi::c_char
            } else {
                b"Invalid imaging settings\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return;
    }
    cJSON_Delete(body);
    if resolve_video_source_token(
        &raw mut config,
        &raw mut video_source_token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to resolve ONVIF video source token\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if onvif_imaging_set_settings(
        &raw mut imaging_url as *mut ::core::ffi::c_char,
        &raw mut video_source_token as *mut ::core::ffi::c_char,
        config_username(&raw mut config),
        config_password(&raw mut config),
        &raw mut settings,
        force_persistence,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            502 as ::core::ffi::c_int,
            b"Failed to set ONVIF imaging settings\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        root,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        root,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"ONVIF imaging settings updated\0".as_ptr() as *const ::core::ffi::c_char,
    );
    send_json_object(res, root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_daynight_get(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
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
    let mut imaging_url: [::core::ffi::c_char; 512] = [0; 512];
    let mut video_source_token: [::core::ffi::c_char; 64] = [0; 64];
    if prepare_imaging_request(
        req,
        res,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut config,
        &raw mut imaging_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        &raw mut video_source_token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        r#true != 0,
    ) != 0 as ::core::ffi::c_int
    {
        return;
    }
    let mut settings: onvif_imaging_settings_t = onvif_imaging_settings_t {
        video_source_token: [0; 64],
        brightness: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        color_saturation: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        contrast: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        sharpness: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        noise_reduction: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        backlight_compensation: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        wide_dynamic_range: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        tone_compensation: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        defogging: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        has_ir_cut_filter: false,
        ir_cut_filter: [0; 16],
    };
    if onvif_imaging_get_settings(
        &raw mut imaging_url as *mut ::core::ffi::c_char,
        &raw mut video_source_token as *mut ::core::ffi::c_char,
        config_username(&raw mut config),
        config_password(&raw mut config),
        &raw mut settings,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            502 as ::core::ffi::c_int,
            b"Failed to get ONVIF day/night settings\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        root,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        root,
        b"source\0".as_ptr() as *const ::core::ffi::c_char,
        b"onvif-imaging\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"video_source_token\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut video_source_token as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"mode\0".as_ptr() as *const ::core::ffi::c_char,
        if settings.has_ir_cut_filter as ::core::ffi::c_int != 0 {
            ir_cut_filter_to_daynight_mode(
                &raw mut settings.ir_cut_filter as *mut ::core::ffi::c_char,
            )
        } else {
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if settings.has_ir_cut_filter {
        cJSON_AddStringToObject(
            root,
            b"ir_cut_filter\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut settings.ir_cut_filter as *mut ::core::ffi::c_char,
        );
    }
    let mut supported_modes: *mut cJSON = cJSON_AddArrayToObject(
        root,
        b"supported_modes\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut options: onvif_imaging_options_t = onvif_imaging_options_t {
        video_source_token: [0; 64],
        brightness: onvif_imaging_float_range_t {
            present: false,
            min: 0.,
            max: 0.,
        },
        color_saturation: onvif_imaging_float_range_t {
            present: false,
            min: 0.,
            max: 0.,
        },
        contrast: onvif_imaging_float_range_t {
            present: false,
            min: 0.,
            max: 0.,
        },
        sharpness: onvif_imaging_float_range_t {
            present: false,
            min: 0.,
            max: 0.,
        },
        noise_reduction: onvif_imaging_float_range_t {
            present: false,
            min: 0.,
            max: 0.,
        },
        backlight_compensation: onvif_imaging_mode_level_options_t {
            present: false,
            modes: [[0; 16]; 4],
            mode_count: 0,
            level: onvif_imaging_float_range_t {
                present: false,
                min: 0.,
                max: 0.,
            },
        },
        wide_dynamic_range: onvif_imaging_mode_level_options_t {
            present: false,
            modes: [[0; 16]; 4],
            mode_count: 0,
            level: onvif_imaging_float_range_t {
                present: false,
                min: 0.,
                max: 0.,
            },
        },
        tone_compensation: onvif_imaging_mode_level_options_t {
            present: false,
            modes: [[0; 16]; 4],
            mode_count: 0,
            level: onvif_imaging_float_range_t {
                present: false,
                min: 0.,
                max: 0.,
            },
        },
        defogging: onvif_imaging_mode_level_options_t {
            present: false,
            modes: [[0; 16]; 4],
            mode_count: 0,
            level: onvif_imaging_float_range_t {
                present: false,
                min: 0.,
                max: 0.,
            },
        },
        ir_cut_filter_modes: [[0; 16]; 4],
        ir_cut_filter_mode_count: 0,
    };
    let mut options_available: bool = r#false != 0;
    if onvif_imaging_get_options(
        &raw mut imaging_url as *mut ::core::ffi::c_char,
        &raw mut video_source_token as *mut ::core::ffi::c_char,
        config_username(&raw mut config),
        config_password(&raw mut config),
        &raw mut options,
    ) == 0 as ::core::ffi::c_int
    {
        options_available = r#true != 0;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < options.ir_cut_filter_mode_count {
            add_supported_daynight_mode(
                supported_modes,
                &raw mut *(&raw mut options.ir_cut_filter_modes as *mut [::core::ffi::c_char; 16])
                    .offset(i as isize) as *mut ::core::ffi::c_char,
            );
            i += 1;
        }
    } else {
        _log_message_ctx(
            log_level_t::LOG_LEVEL_WARN,
            LOG_COMPONENT.as_ptr(),
            _log_stream_name,
            b"Failed to get ONVIF day/night options for stream %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut config.name as *mut ::core::ffi::c_char,
        );
    }
    add_default_supported_daynight_modes(supported_modes);
    cJSON_AddBoolToObject(
        root,
        b"options_available\0".as_ptr() as *const ::core::ffi::c_char,
        options_available as cJSON_bool,
    );
    send_json_object(res, root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_daynight_put(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
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
    let mut imaging_url: [::core::ffi::c_char; 512] = [0; 512];
    let mut video_source_token: [::core::ffi::c_char; 64] = [0; 64];
    if prepare_imaging_request(
        req,
        res,
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        &raw mut config,
        &raw mut imaging_url as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
        &raw mut video_source_token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        r#false != 0,
    ) != 0 as ::core::ffi::c_int
    {
        return;
    }
    let mut body: *mut cJSON = device_json_body(req);
    if !valid_imaging_update(body, r#true != 0) {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut ir_cut_filter: [::core::ffi::c_char; 16] = [
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
    let mut mode: *mut cJSON =
        cJSON_GetObjectItem(body, b"mode\0".as_ptr() as *const ::core::ffi::c_char);
    let mut daynight: *mut cJSON =
        cJSON_GetObjectItem(body, b"daynight\0".as_ptr() as *const ::core::ffi::c_char);
    let mut ir_cut: *mut cJSON = cJSON_GetObjectItem(
        body,
        b"ir_cut_filter\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut ircut: *mut cJSON =
        cJSON_GetObjectItem(body, b"ircut\0".as_ptr() as *const ::core::ffi::c_char);
    let mut valid: bool = r#false != 0;
    if !mode.is_null() && cJSON_IsString(mode) != 0 {
        valid = daynight_mode_to_ir_cut_filter(
            (*mode).valuestring,
            &raw mut ir_cut_filter as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        );
    } else if !daynight.is_null() && cJSON_IsString(daynight) != 0 {
        valid = daynight_mode_to_ir_cut_filter(
            (*daynight).valuestring,
            &raw mut ir_cut_filter as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        );
    } else if !ir_cut.is_null() && cJSON_IsString(ir_cut) != 0 {
        valid = daynight_mode_to_ir_cut_filter(
            (*ir_cut).valuestring,
            &raw mut ir_cut_filter as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        );
    } else if !ircut.is_null() && cJSON_IsString(ircut) != 0 {
        valid = daynight_mode_to_ir_cut_filter(
            (*ircut).valuestring,
            &raw mut ir_cut_filter as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        );
    }
    let mut force: *mut cJSON = cJSON_GetObjectItem(
        body,
        b"force_persistence\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut force_persistence: bool = r#true != 0;
    if !force.is_null() && cJSON_IsBool(force) != 0 {
        force_persistence = cJSON_IsTrue(force) != 0;
    }
    cJSON_Delete(body);
    if !valid {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"mode must be auto, day, or night; ir_cut_filter must be On, Off, or Auto\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut settings: onvif_imaging_settings_t = onvif_imaging_settings_t {
        video_source_token: [0; 64],
        brightness: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        color_saturation: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        contrast: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        sharpness: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        noise_reduction: onvif_imaging_float_t {
            present: false,
            value: 0.,
        },
        backlight_compensation: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        wide_dynamic_range: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        tone_compensation: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        defogging: onvif_imaging_mode_level_t {
            present: false,
            mode: [0; 16],
            has_level: false,
            level: 0.,
        },
        has_ir_cut_filter: false,
        ir_cut_filter: [0; 16],
    };
    memset(
        &raw mut settings as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<onvif_imaging_settings_t>(),
    );
    settings.has_ir_cut_filter = r#true != 0;
    safe_strcpy(
        &raw mut settings.ir_cut_filter as *mut ::core::ffi::c_char,
        &raw mut ir_cut_filter as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    if resolve_video_source_token(
        &raw mut config,
        &raw mut video_source_token as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to resolve ONVIF video source token\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if onvif_imaging_set_settings(
        &raw mut imaging_url as *mut ::core::ffi::c_char,
        &raw mut video_source_token as *mut ::core::ffi::c_char,
        config_username(&raw mut config),
        config_password(&raw mut config),
        &raw mut settings,
        force_persistence,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            502 as ::core::ffi::c_int,
            b"Failed to set ONVIF day/night mode\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        root,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        root,
        b"source\0".as_ptr() as *const ::core::ffi::c_char,
        b"onvif-imaging\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"mode\0".as_ptr() as *const ::core::ffi::c_char,
        ir_cut_filter_to_daynight_mode(&raw mut ir_cut_filter as *mut ::core::ffi::c_char),
    );
    cJSON_AddStringToObject(
        root,
        b"ir_cut_filter\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut ir_cut_filter as *mut ::core::ffi::c_char,
    );
    send_json_object(res, root);
}
pub const __FLT_MAX__: ::core::ffi::c_float = 3.40282347e+38f32;
