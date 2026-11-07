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
    unsafe fn cJSON_Parse(value: *const ::core::ffi::c_char) -> *mut cJSON;
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
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
    unsafe fn db_detection_engines_list(
        stream_name: *const ::core::ffi::c_char,
        engines: *mut stream_detection_engine_t,
        capacity: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_detection_engines_replace_custom(
        stream_name: *const ::core::ffi::c_char,
        engines: *const stream_detection_engine_t,
        count: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_detection_engine_validate(
        engine: *const stream_detection_engine_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
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
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_authorize_stream_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        stream_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_authorize_stream_action_with_context(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        stream_name: *const ::core::ffi::c_char,
        user: *mut user_t,
        camera: *mut fleet_camera_t,
        evaluation: *mut authorization_evaluation_t,
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
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type int64_t = i64;
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
pub struct stream_detection_engine_t {
    pub id: uint64_t,
    pub engine_key: [::core::ffi::c_char; 64],
    pub engine_type: [::core::ffi::c_char; 16],
    pub model_path: [::core::ffi::c_char; 512],
    pub enabled: bool,
    pub threshold: ::core::ffi::c_float,
    pub interval_seconds: ::core::ffi::c_int,
    pub sort_order: ::core::ffi::c_int,
    pub config_json: [::core::ffi::c_char; 1024],
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
pub const MAX_DETECTION_ENGINES_PER_STREAM: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
unsafe extern "C" fn stream_from_path(
    mut req: *const http_request_t,
    mut stream_name: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> bool {
    if http_request_extract_path_param(
        req,
        b"/api/streams/\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
        size,
    ) != 0 as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut suffix: *mut ::core::ffi::c_char = strstr(
        stream_name,
        b"/detection-engines\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if suffix.is_null()
        || *suffix.offset(18isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    *suffix = '\0' as ::core::ffi::c_char;
    return *stream_name.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int;
}
unsafe extern "C" fn engine_json(mut engine: *const stream_detection_engine_t) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_CreateObject();
    if object.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"key\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*engine).engine_key as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*engine).engine_type as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"model_path\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*engine).model_path as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        object,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        (*engine).enabled as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        object,
        b"threshold\0".as_ptr() as *const ::core::ffi::c_char,
        (*engine).threshold as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
        (*engine).interval_seconds as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"sort_order\0".as_ptr() as *const ::core::ffi::c_char,
        (*engine).sort_order as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        object,
        b"managed_by_legacy_stream_fields\0".as_ptr() as *const ::core::ffi::c_char,
        (strcmp(
            &raw const (*engine).engine_key as *const ::core::ffi::c_char,
            b"legacy-primary\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    let mut config: *mut cJSON =
        cJSON_Parse(&raw const (*engine).config_json as *const ::core::ffi::c_char);
    cJSON_AddItemToObject(
        object,
        b"config\0".as_ptr() as *const ::core::ffi::c_char,
        if !config.is_null() {
            config
        } else {
            cJSON_CreateObject()
        },
    );
    return object;
}
unsafe extern "C" fn send_engines(
    mut res: *mut http_response_t,
    mut stream_name: *const ::core::ffi::c_char,
) {
    let mut engines: [stream_detection_engine_t; 8] = [stream_detection_engine_t {
        id: 0,
        engine_key: [0; 64],
        engine_type: [0; 16],
        model_path: [0; 512],
        enabled: false,
        threshold: 0.,
        interval_seconds: 0,
        sort_order: 0,
        config_json: [0; 1024],
    }; 8];
    let mut count: ::core::ffi::c_int = db_detection_engines_list(
        stream_name,
        &raw mut engines as *mut stream_detection_engine_t,
        MAX_DETECTION_ENGINES_PER_STREAM as size_t,
    );
    if count < 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load detection engines\0".as_ptr() as *const ::core::ffi::c_char,
        );
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
    cJSON_AddStringToObject(
        root,
        b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
        stream_name,
    );
    cJSON_AddItemToObject(
        root,
        b"engines\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut item: *mut cJSON =
            engine_json((&raw mut engines as *mut stream_detection_engine_t).offset(i as isize));
        if !item.is_null() {
            cJSON_AddItemToArray(items, item);
        }
        i += 1;
    }
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        count as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        root,
        b"trigger_policy\0".as_ptr() as *const ::core::ffi::c_char,
        b"any_of\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        root,
        b"restart_required\0".as_ptr() as *const ::core::ffi::c_char,
        r#false,
    );
    let mut encoded: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if encoded.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, encoded);
    free(encoded as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_detection_engines(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if !stream_from_path(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
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
    send_engines(res, &raw mut stream_name as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn parse_engine(
    mut item: *const cJSON,
    mut engine: *mut stream_detection_engine_t,
) -> bool {
    if cJSON_IsObject(item) == 0 {
        return r#false != 0;
    }
    let mut key: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(item, b"key\0".as_ptr() as *const ::core::ffi::c_char);
    let mut r#type: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(item, b"type\0".as_ptr() as *const ::core::ffi::c_char);
    let mut model: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        item,
        b"model_path\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut enabled: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(item, b"enabled\0".as_ptr() as *const ::core::ffi::c_char);
    let mut threshold: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        item,
        b"threshold\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut interval: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        item,
        b"interval_seconds\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut order: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        item,
        b"sort_order\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut config: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(item, b"config\0".as_ptr() as *const ::core::ffi::c_char);
    if cJSON_IsString(key) == 0
        || cJSON_IsString(r#type) == 0
        || !model.is_null() && cJSON_IsString(model) == 0
        || !enabled.is_null() && cJSON_IsBool(enabled) == 0
        || cJSON_IsNumber(threshold) == 0
        || (*threshold).valuedouble.is_finite() as ::core::ffi::c_int == 0
        || cJSON_IsNumber(interval) == 0
        || (*interval).valuedouble.is_finite() as ::core::ffi::c_int == 0
        || floor((*interval).valuedouble) != (*interval).valuedouble
        || !order.is_null()
            && (cJSON_IsNumber(order) == 0
                || (*order).valuedouble.is_finite() as ::core::ffi::c_int == 0
                || floor((*order).valuedouble) != (*order).valuedouble)
        || !config.is_null() && cJSON_IsObject(config) == 0
    {
        return r#false != 0;
    }
    if strlen((*key).valuestring) >= ::core::mem::size_of::<[::core::ffi::c_char; 64]>()
        || strlen((*r#type).valuestring) >= ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
        || !model.is_null()
            && strlen((*model).valuestring) >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        || (*threshold).valuedouble < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        || (*threshold).valuedouble > 1 as ::core::ffi::c_int as ::core::ffi::c_double
        || (*interval).valuedouble < 1 as ::core::ffi::c_int as ::core::ffi::c_double
        || (*interval).valuedouble > 86400 as ::core::ffi::c_int as ::core::ffi::c_double
        || !order.is_null()
            && ((*order).valuedouble < INT_MIN as ::core::ffi::c_double
                || (*order).valuedouble > INT_MAX as ::core::ffi::c_double)
    {
        return r#false != 0;
    }
    memset(
        engine as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<stream_detection_engine_t>(),
    );
    safe_strcpy(
        &raw mut (*engine).engine_key as *mut ::core::ffi::c_char,
        (*key).valuestring,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*engine).engine_type as *mut ::core::ffi::c_char,
        (*r#type).valuestring,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
        0 as size_t,
    );
    if !model.is_null() {
        safe_strcpy(
            &raw mut (*engine).model_path as *mut ::core::ffi::c_char,
            (*model).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    (*engine).enabled = if !enabled.is_null() {
        cJSON_IsTrue(enabled)
    } else {
        r#true
    } != 0;
    (*engine).threshold = (*threshold).valuedouble as ::core::ffi::c_float;
    (*engine).interval_seconds = (*interval).valueint;
    (*engine).sort_order = if !order.is_null() {
        (*order).valueint
    } else {
        0 as ::core::ffi::c_int
    };
    let mut config_text: *mut ::core::ffi::c_char = if !config.is_null() {
        cJSON_PrintUnformatted(config)
    } else {
        strdup(b"{}\0".as_ptr() as *const ::core::ffi::c_char)
    };
    if config_text.is_null()
        || strlen(config_text) >= ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
    {
        free(config_text as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    safe_strcpy(
        &raw mut (*engine).config_json as *mut ::core::ffi::c_char,
        config_text,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        0 as size_t,
    );
    free(config_text as *mut ::core::ffi::c_void);
    return db_detection_engine_validate(
        engine,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as size_t,
    ) == 0 as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_detection_engines(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut stream_name: [::core::ffi::c_char; 256] = [0; 256];
    if !stream_from_path(
        req,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    ) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid stream path\0".as_ptr() as *const ::core::ffi::c_char,
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
    if httpd_authorize_stream_action_with_context(
        req,
        res,
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut user,
        &raw mut camera,
        &raw mut evaluation,
    ) == 0
    {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut items: *const cJSON = if !body.is_null() {
        cJSON_GetObjectItemCaseSensitive(body, b"engines\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    let mut count: ::core::ffi::c_int = if cJSON_IsArray(items) != 0 {
        cJSON_GetArraySize(items)
    } else {
        -1 as ::core::ffi::c_int
    };
    if count < 0 as ::core::ffi::c_int || count >= MAX_DETECTION_ENGINES_PER_STREAM {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid engines array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut engines: [stream_detection_engine_t; 8] = [stream_detection_engine_t {
        id: 0,
        engine_key: [0; 64],
        engine_type: [0; 16],
        model_path: [0; 512],
        enabled: false,
        threshold: 0.,
        interval_seconds: 0,
        sort_order: 0,
        config_json: [0; 1024],
    }; 8];
    memset(
        &raw mut engines as *mut stream_detection_engine_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[stream_detection_engine_t; 8]>(),
    );
    let mut valid: bool = r#true != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if !parse_engine(
            cJSON_GetArrayItem(items, i),
            (&raw mut engines as *mut stream_detection_engine_t).offset(i as isize),
        ) {
            valid = r#false != 0;
            break;
        } else {
            i += 1;
        }
    }
    cJSON_Delete(body);
    if !valid {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid detection engine\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut result: ::core::ffi::c_int = db_detection_engines_replace_custom(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut engines as *mut stream_detection_engine_t,
        count as size_t,
    );
    let mut audit: *mut cJSON = cJSON_CreateObject();
    if !audit.is_null() {
        cJSON_AddNumberToObject(
            audit,
            b"custom_engine_count\0".as_ptr() as *const ::core::ffi::c_char,
            count as ::core::ffi::c_double,
        );
    }
    audit_log_operation(
        req,
        &raw mut user,
        b"camera.configure\0".as_ptr() as *const ::core::ffi::c_char,
        b"camera\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut camera.camera_uuid as *mut ::core::ffi::c_char,
        b"detection_engines_replace\0".as_ptr() as *const ::core::ffi::c_char,
        if result == 0 as ::core::ffi::c_int {
            b"success\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"failure\0".as_ptr() as *const ::core::ffi::c_char
        },
        audit,
    );
    cJSON_Delete(audit);
    if result != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to save detection engines\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut current: [stream_detection_engine_t; 8] = [stream_detection_engine_t {
        id: 0,
        engine_key: [0; 64],
        engine_type: [0; 16],
        model_path: [0; 512],
        enabled: false,
        threshold: 0.,
        interval_seconds: 0,
        sort_order: 0,
        config_json: [0; 1024],
    }; 8];
    let mut current_count: ::core::ffi::c_int = db_detection_engines_list(
        &raw mut stream_name as *mut ::core::ffi::c_char,
        &raw mut current as *mut stream_detection_engine_t,
        MAX_DETECTION_ENGINES_PER_STREAM as size_t,
    );
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut array: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || array.is_null() || current_count < 0 as ::core::ffi::c_int {
        cJSON_Delete(root);
        cJSON_Delete(array);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Saved but failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddStringToObject(
        root,
        b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut stream_name as *mut ::core::ffi::c_char,
    );
    cJSON_AddItemToObject(
        root,
        b"engines\0".as_ptr() as *const ::core::ffi::c_char,
        array,
    );
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < current_count {
        let mut engine: *mut cJSON =
            engine_json((&raw mut current as *mut stream_detection_engine_t).offset(i_0 as isize));
        if !engine.is_null() {
            cJSON_AddItemToArray(array, engine);
        }
        i_0 += 1;
    }
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        current_count as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        root,
        b"trigger_policy\0".as_ptr() as *const ::core::ffi::c_char,
        b"any_of\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        root,
        b"restart_required\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    let mut encoded: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if encoded.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Saved but failed to serialize response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, encoded);
    free(encoded as *mut ::core::ffi::c_void);
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
