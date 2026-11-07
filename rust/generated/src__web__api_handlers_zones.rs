unsafe extern "C" {
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn _log_message_ctx(
        level: log_level_t,
        component: *const ::core::ffi::c_char,
        stream: *const ::core::ffi::c_char,
        format: *const ::core::ffi::c_char,
        ...
    );
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
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItem(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
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
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_authorize_stream_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn detection_zone_point_count_valid(point_count: ::core::ffi::c_int) -> bool;
    unsafe fn save_detection_zones(
        stream_name: *const ::core::ffi::c_char,
        zones: *const detection_zone_t,
        count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn get_detection_zones(
        stream_name: *const ::core::ffi::c_char,
        zones: *mut detection_zone_t,
        max_zones: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn delete_detection_zones(stream_name: *const ::core::ffi::c_char)
    -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
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
pub struct log_level_t(pub ::core::ffi::c_uint);
impl log_level_t {
    pub const LOG_LEVEL_ERROR: Self = Self(0);
    pub const LOG_LEVEL_WARN: Self = Self(1);
    pub const LOG_LEVEL_INFO: Self = Self(2);
    pub const LOG_LEVEL_DEBUG: Self = Self(3);
}
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
pub struct detection_zone_t {
    pub id: [::core::ffi::c_char; 64],
    pub stream_name: [::core::ffi::c_char; 256],
    pub name: [::core::ffi::c_char; 64],
    pub enabled: bool,
    pub color: [::core::ffi::c_char; 8],
    pub polygon: [zone_point_t; 32],
    pub polygon_count: ::core::ffi::c_int,
    pub filter_classes: [::core::ffi::c_char; 256],
    pub min_confidence: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zone_point_t {
    pub x: ::core::ffi::c_float,
    pub y: ::core::ffi::c_float,
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
pub const _log_stream_name: *const ::core::ffi::c_char = NULL as *const ::core::ffi::c_char;
pub const MAX_STREAM_NAME: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const LOG_COMPONENT: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"ZonesAPI\0") };
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_zones(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if http_request_extract_path_param(
        req,
        b"/api/streams/\0".as_ptr() as *const ::core::ffi::c_char,
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
    let mut zones_suffix: *mut ::core::ffi::c_char = strstr(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        b"/zones\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !zones_suffix.is_null() {
        *zones_suffix = '\0' as ::core::ffi::c_char;
    }
    if httpd_authorize_stream_action(
        req,
        res,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    ) == 0
    {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"GET /api/streams/%s/zones\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut zones: [detection_zone_t; 16] = [detection_zone_t {
        id: [0; 64],
        stream_name: [0; 256],
        name: [0; 64],
        enabled: false,
        color: [0; 8],
        polygon: [zone_point_t { x: 0., y: 0. }; 32],
        polygon_count: 0,
        filter_classes: [0; 256],
        min_confidence: 0.,
    }; 16];
    let mut count: ::core::ffi::c_int = get_detection_zones(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut zones as *mut detection_zone_t,
        MAX_ZONES_PER_STREAM,
    );
    if count < 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to get detection zones\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    if response.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create JSON response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut zones_array: *mut cJSON = cJSON_CreateArray();
    if zones_array.is_null() {
        cJSON_Delete(response);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create zones array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut zone_obj: *mut cJSON = cJSON_CreateObject();
        if !zone_obj.is_null() {
            cJSON_AddStringToObject(
                zone_obj,
                b"id\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*(&raw mut zones as *mut detection_zone_t).offset(i as isize)).id
                    as *mut ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                zone_obj,
                b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*(&raw mut zones as *mut detection_zone_t).offset(i as isize)).stream_name
                    as *mut ::core::ffi::c_char,
            );
            cJSON_AddStringToObject(
                zone_obj,
                b"name\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*(&raw mut zones as *mut detection_zone_t).offset(i as isize)).name
                    as *mut ::core::ffi::c_char,
            );
            cJSON_AddBoolToObject(
                zone_obj,
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
                zones[i as usize].enabled as cJSON_bool,
            );
            cJSON_AddStringToObject(
                zone_obj,
                b"color\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*(&raw mut zones as *mut detection_zone_t).offset(i as isize)).color
                    as *mut ::core::ffi::c_char,
            );
            let mut polygon_array: *mut cJSON = cJSON_CreateArray();
            if !polygon_array.is_null() {
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j < zones[i as usize].polygon_count {
                    let mut point: *mut cJSON = cJSON_CreateObject();
                    if !point.is_null() {
                        cJSON_AddNumberToObject(
                            point,
                            b"x\0".as_ptr() as *const ::core::ffi::c_char,
                            zones[i as usize].polygon[j as usize].x as ::core::ffi::c_double,
                        );
                        cJSON_AddNumberToObject(
                            point,
                            b"y\0".as_ptr() as *const ::core::ffi::c_char,
                            zones[i as usize].polygon[j as usize].y as ::core::ffi::c_double,
                        );
                        cJSON_AddItemToArray(polygon_array, point);
                    }
                    j += 1;
                }
                cJSON_AddItemToObject(
                    zone_obj,
                    b"polygon\0".as_ptr() as *const ::core::ffi::c_char,
                    polygon_array,
                );
            }
            cJSON_AddStringToObject(
                zone_obj,
                b"filter_classes\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*(&raw mut zones as *mut detection_zone_t).offset(i as isize))
                    .filter_classes as *mut ::core::ffi::c_char,
            );
            cJSON_AddNumberToObject(
                zone_obj,
                b"min_confidence\0".as_ptr() as *const ::core::ffi::c_char,
                zones[i as usize].min_confidence as ::core::ffi::c_double,
            );
            cJSON_AddItemToArray(zones_array, zone_obj);
        }
        i += 1;
    }
    cJSON_AddItemToObject(
        response,
        b"zones\0".as_ptr() as *const ::core::ffi::c_char,
        zones_array,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    if !json_str.is_null() {
        http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
        free(json_str as *mut ::core::ffi::c_void);
    } else {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_zones(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if http_request_extract_path_param(
        req,
        b"/api/streams/\0".as_ptr() as *const ::core::ffi::c_char,
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
    let mut zones_suffix: *mut ::core::ffi::c_char = strstr(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        b"/zones\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !zones_suffix.is_null() {
        *zones_suffix = '\0' as ::core::ffi::c_char;
    }
    if httpd_authorize_stream_action(
        req,
        res,
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    ) == 0
    {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"POST /api/streams/%s/zones\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    let mut json: *mut cJSON = httpd_parse_json_body(req);
    if json.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON in request body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut zones_array: *mut cJSON =
        cJSON_GetObjectItem(json, b"zones\0".as_ptr() as *const ::core::ffi::c_char);
    if zones_array.is_null() || cJSON_IsArray(zones_array) == 0 {
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Missing or invalid 'zones' array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut zone_count: ::core::ffi::c_int = cJSON_GetArraySize(zones_array);
    if zone_count > MAX_ZONES_PER_STREAM {
        cJSON_Delete(json);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Too many zones\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut zones: [detection_zone_t; 16] = [detection_zone_t {
        id: [0; 64],
        stream_name: [0; 256],
        name: [0; 64],
        enabled: false,
        color: [0; 8],
        polygon: [zone_point_t { x: 0., y: 0. }; 32],
        polygon_count: 0,
        filter_classes: [0; 256],
        min_confidence: 0.,
    }; 16];
    memset(
        &raw mut zones as *mut detection_zone_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[detection_zone_t; 16]>(),
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < zone_count {
        let mut zone_obj: *mut cJSON = cJSON_GetArrayItem(zones_array, i);
        if !zone_obj.is_null() {
            let mut zone: *mut detection_zone_t =
                (&raw mut zones as *mut detection_zone_t).offset(i as isize);
            let mut id: *mut cJSON =
                cJSON_GetObjectItem(zone_obj, b"id\0".as_ptr() as *const ::core::ffi::c_char);
            let mut name: *mut cJSON =
                cJSON_GetObjectItem(zone_obj, b"name\0".as_ptr() as *const ::core::ffi::c_char);
            let mut enabled: *mut cJSON = cJSON_GetObjectItem(
                zone_obj,
                b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut color: *mut cJSON =
                cJSON_GetObjectItem(zone_obj, b"color\0".as_ptr() as *const ::core::ffi::c_char);
            let mut polygon: *mut cJSON = cJSON_GetObjectItem(
                zone_obj,
                b"polygon\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut filter_classes: *mut cJSON = cJSON_GetObjectItem(
                zone_obj,
                b"filter_classes\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut min_confidence: *mut cJSON = cJSON_GetObjectItem(
                zone_obj,
                b"min_confidence\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if id.is_null()
                || cJSON_IsString(id) == 0
                || name.is_null()
                || cJSON_IsString(name) == 0
                || polygon.is_null()
                || cJSON_IsArray(polygon) == 0
            {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Invalid zone data at index %d\0".as_ptr() as *const ::core::ffi::c_char,
                    i,
                );
                cJSON_Delete(json);
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"Each zone needs an id, name and polygon\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return;
            }
            let mut valid_options: bool = *(*id).valuestring.offset(0isize) as ::core::ffi::c_int
                != 0
                && strlen((*id).valuestring) < ::core::mem::size_of::<[::core::ffi::c_char; 64]>()
                && strlen((*name).valuestring)
                    < ::core::mem::size_of::<[::core::ffi::c_char; 64]>()
                && (enabled.is_null() || cJSON_IsBool(enabled) != 0)
                && (filter_classes.is_null()
                    || cJSON_IsString(filter_classes) != 0
                        && strlen((*filter_classes).valuestring)
                            < ::core::mem::size_of::<[::core::ffi::c_char; 256]>())
                && (min_confidence.is_null()
                    || cJSON_IsNumber(min_confidence) != 0
                        && (*min_confidence).valuedouble.is_finite() as ::core::ffi::c_int != 0
                        && (*min_confidence).valuedouble
                            >= 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        && (*min_confidence).valuedouble
                            <= 1 as ::core::ffi::c_int as ::core::ffi::c_double);
            if !color.is_null() {
                valid_options = valid_options as ::core::ffi::c_int != 0
                    && cJSON_IsString(color) != 0
                    && strlen((*color).valuestring) == 7 as size_t
                    && *(*color).valuestring.offset(0isize) as ::core::ffi::c_int
                        == '#' as ::core::ffi::c_int;
                let mut c: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while valid_options as ::core::ffi::c_int != 0 && c < 7 as ::core::ffi::c_int {
                    valid_options =
                        *(*__ctype_b_loc()).offset(*(*color).valuestring.offset(c as isize)
                            as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            as isize) as ::core::ffi::c_int
                            & C2Rust_Unnamed::_ISxdigit.0 as ::core::ffi::c_int
                                as ::core::ffi::c_ushort
                                as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int;
                    c += 1;
                }
            }
            let mut previous: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while valid_options as ::core::ffi::c_int != 0 && previous < i {
                valid_options = strcmp(
                    &raw mut (*(&raw mut zones as *mut detection_zone_t).offset(previous as isize))
                        .id as *mut ::core::ffi::c_char,
                    (*id).valuestring,
                ) != 0 as ::core::ffi::c_int;
                previous += 1;
            }
            if !valid_options {
                cJSON_Delete(json);
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"Invalid or duplicate zone identifier or options\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return;
            }
            safe_strcpy(
                &raw mut (*zone).id as *mut ::core::ffi::c_char,
                (*id).valuestring,
                MAX_ZONE_ID as size_t,
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*zone).stream_name as *mut ::core::ffi::c_char,
                &raw mut stream_name as *mut ::core::ffi::c_char,
                MAX_STREAM_NAME as size_t,
                0 as size_t,
            );
            safe_strcpy(
                &raw mut (*zone).name as *mut ::core::ffi::c_char,
                (*name).valuestring,
                MAX_ZONE_NAME as size_t,
                0 as size_t,
            );
            (*zone).enabled = !enabled.is_null() && cJSON_IsTrue(enabled) != 0;
            if !color.is_null() && cJSON_IsString(color) != 0 {
                safe_strcpy(
                    &raw mut (*zone).color as *mut ::core::ffi::c_char,
                    (*color).valuestring,
                    8 as size_t,
                    0 as size_t,
                );
            } else {
                strcpy(
                    &raw mut (*zone).color as *mut ::core::ffi::c_char,
                    b"#3b82f6\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            let mut point_count: ::core::ffi::c_int = cJSON_GetArraySize(polygon);
            if !detection_zone_point_count_valid(point_count) {
                _log_message_ctx(
                    log_level_t::LOG_LEVEL_WARN,
                    LOG_COMPONENT.as_ptr(),
                    _log_stream_name,
                    b"Invalid polygon point count %d at zone index %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    point_count,
                    i,
                );
                cJSON_Delete(json);
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"Zone polygon must have 3 to 32 points\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return;
            }
            (*zone).polygon_count = 0 as ::core::ffi::c_int;
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < point_count {
                let mut point: *mut cJSON = cJSON_GetArrayItem(polygon, j);
                let mut x: *mut cJSON = if !point.is_null() {
                    cJSON_GetObjectItem(point, b"x\0".as_ptr() as *const ::core::ffi::c_char)
                } else {
                    ::core::ptr::null_mut::<cJSON>()
                };
                let mut y: *mut cJSON = if !point.is_null() {
                    cJSON_GetObjectItem(point, b"y\0".as_ptr() as *const ::core::ffi::c_char)
                } else {
                    ::core::ptr::null_mut::<cJSON>()
                };
                if x.is_null() || cJSON_IsNumber(x) == 0 || y.is_null() || cJSON_IsNumber(y) == 0 {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Invalid polygon point %d at zone index %d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        j,
                        i,
                    );
                    cJSON_Delete(json);
                    http_response_set_json_error(
                        res,
                        400 as ::core::ffi::c_int,
                        b"Zone polygon points need numeric x and y\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return;
                }
                let mut x_val: ::core::ffi::c_double = (*x).valuedouble;
                let mut y_val: ::core::ffi::c_double = (*y).valuedouble;
                if x_val.is_finite() as ::core::ffi::c_int == 0
                    || y_val.is_finite() as ::core::ffi::c_int == 0
                    || x_val < 0.0f64
                    || x_val > 1.0f64
                    || y_val < 0.0f64
                    || y_val > 1.0f64
                {
                    _log_message_ctx(
                        log_level_t::LOG_LEVEL_WARN,
                        LOG_COMPONENT.as_ptr(),
                        _log_stream_name,
                        b"Out-of-range polygon point %d at zone index %d: x=%f y=%f\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        j,
                        i,
                        x_val,
                        y_val,
                    );
                    cJSON_Delete(json);
                    http_response_set_json_error(
                        res,
                        400 as ::core::ffi::c_int,
                        b"Zone polygon points must be finite and within [0.0, 1.0]\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return;
                }
                (*zone).polygon[(*zone).polygon_count as usize].x = x_val as ::core::ffi::c_float;
                (*zone).polygon[(*zone).polygon_count as usize].y = y_val as ::core::ffi::c_float;
                (*zone).polygon_count += 1;
                j += 1;
            }
            if !filter_classes.is_null() && cJSON_IsString(filter_classes) != 0 {
                safe_strcpy(
                    &raw mut (*zone).filter_classes as *mut ::core::ffi::c_char,
                    (*filter_classes).valuestring,
                    256 as size_t,
                    0 as size_t,
                );
            } else {
                (*zone).filter_classes[0usize] = '\0' as ::core::ffi::c_char;
            }
            if !min_confidence.is_null() && cJSON_IsNumber(min_confidence) != 0 {
                (*zone).min_confidence = (*min_confidence).valuedouble as ::core::ffi::c_float;
            } else {
                (*zone).min_confidence = 0.0f32;
            }
        }
        i += 1;
    }
    cJSON_Delete(json);
    if save_detection_zones(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut zones as *mut detection_zone_t,
        zone_count,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to save detection zones\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"Zones saved successfully\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        response,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        zone_count as ::core::ffi::c_double,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    if !json_str.is_null() {
        http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
        free(json_str as *mut ::core::ffi::c_void);
    } else {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_zones(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if http_request_extract_path_param(
        req,
        b"/api/streams/\0".as_ptr() as *const ::core::ffi::c_char,
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
    let mut zones_suffix: *mut ::core::ffi::c_char = strstr(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        b"/zones\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !zones_suffix.is_null() {
        *zones_suffix = '\0' as ::core::ffi::c_char;
    }
    if httpd_authorize_stream_action(
        req,
        res,
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    ) == 0
    {
        return;
    }
    _log_message_ctx(
        log_level_t::LOG_LEVEL_INFO,
        LOG_COMPONENT.as_ptr(),
        _log_stream_name,
        b"DELETE /api/streams/%s/zones\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    if delete_detection_zones(&raw mut stream_name as *mut ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to delete detection zones\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
        b"Zones deleted successfully\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut json_str: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    if !json_str.is_null() {
        http_response_set_json(res, 200 as ::core::ffi::c_int, json_str);
        free(json_str as *mut ::core::ffi::c_void);
    } else {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize JSON\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
}
pub const MAX_ZONE_NAME: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MAX_ZONE_ID: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MAX_ZONES_PER_STREAM: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
