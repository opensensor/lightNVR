unsafe extern "C" {
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
    unsafe fn cJSON_GetArrayItem(array: *const cJSON, index: ::core::ffi::c_int) -> *mut cJSON;
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
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
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn db_camera_tag_count() -> ::core::ffi::c_int;
    unsafe fn db_camera_tag_list(
        tags: *mut camera_tag_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_camera_tag_get(
        uuid: *const ::core::ffi::c_char,
        tag: *mut camera_tag_t,
    ) -> db_camera_tag_result_t;
    unsafe fn db_camera_tag_create(tag: *mut camera_tag_t) -> db_camera_tag_result_t;
    unsafe fn db_camera_tag_update(tag: *mut camera_tag_t) -> db_camera_tag_result_t;
    unsafe fn db_camera_tag_delete(uuid: *const ::core::ffi::c_char) -> db_camera_tag_result_t;
    unsafe fn db_camera_tag_merge(
        source_uuid: *const ::core::ffi::c_char,
        target_uuid: *const ::core::ffi::c_char,
    ) -> db_camera_tag_result_t;
    unsafe fn db_camera_tag_list_for_camera(
        camera_uuid: *const ::core::ffi::c_char,
        tags: *mut camera_tag_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_camera_tag_set_for_camera(
        camera_uuid: *const ::core::ffi::c_char,
        tag_uuids: *const *const ::core::ffi::c_char,
        tag_count: ::core::ffi::c_int,
    ) -> db_camera_tag_result_t;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
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
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_authorize_global_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camera_tag_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub label: [::core::ffi::c_char; 256],
    pub color: [::core::ffi::c_char; 16],
    pub description: [::core::ffi::c_char; 512],
    pub camera_count: ::core::ffi::c_int,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_camera_tag_result_t(pub ::core::ffi::c_int);
impl db_camera_tag_result_t {
    pub const DB_CAMERA_TAG_OK: Self = Self(0);
    pub const DB_CAMERA_TAG_NOT_FOUND: Self = Self(-1);
    pub const DB_CAMERA_TAG_CONFLICT: Self = Self(-2);
    pub const DB_CAMERA_TAG_INVALID: Self = Self(-3);
    pub const DB_CAMERA_TAG_ERROR: Self = Self(-4);
    pub const DB_CAMERA_TAG_LIMIT: Self = Self(-5);
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
pub const CAMERA_TAG_MAX_ASSIGNMENTS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
unsafe extern "C" fn valid_color(mut color: *const ::core::ffi::c_char) -> bool {
    if color.is_null() || *color.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        return r#true != 0;
    }
    if strlen(color) != 7 as size_t
        || *color.offset(0isize) as ::core::ffi::c_int != '#' as ::core::ffi::c_int
    {
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i < 7 as ::core::ffi::c_int {
        if *(*__ctype_b_loc()).offset(
            *color.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize
        ) as ::core::ffi::c_int
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
unsafe extern "C" fn tag_to_json(mut tag: *const camera_tag_t) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_CreateObject();
    if object.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*tag).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"label\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*tag).label as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"color\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*tag).color as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"description\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*tag).description as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        object,
        b"camera_count\0".as_ptr() as *const ::core::ffi::c_char,
        (*tag).camera_count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*tag).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*tag).updated_at as ::core::ffi::c_double,
    );
    return object;
}
unsafe extern "C" fn set_tag_json(
    mut res: *mut http_response_t,
    mut status: ::core::ffi::c_int,
    mut tag: *const camera_tag_t,
) {
    let mut object: *mut cJSON = tag_to_json(tag);
    let mut json: *mut ::core::ffi::c_char = if !object.is_null() {
        cJSON_PrintUnformatted(object)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    cJSON_Delete(object);
    if json.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize camera tag\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, status, json);
    free(json as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn set_db_error(
    mut res: *mut http_response_t,
    mut result: db_camera_tag_result_t,
) {
    match result {
        db_camera_tag_result_t::DB_CAMERA_TAG_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Camera or tag not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_camera_tag_result_t::DB_CAMERA_TAG_CONFLICT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"A tag with that label already exists\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_camera_tag_result_t::DB_CAMERA_TAG_LIMIT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Assigned tag labels exceed the legacy compatibility limit\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_camera_tag_result_t::DB_CAMERA_TAG_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid camera tag request\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Camera tag database operation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn apply_tag_fields(
    mut body: *mut cJSON,
    mut tag: *mut camera_tag_t,
    mut creating: bool,
    mut res: *mut http_response_t,
) -> bool {
    if cJSON_IsObject(body) == 0 {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Request body must be an object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut label: *mut cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"label\0".as_ptr() as *const ::core::ffi::c_char);
    if !label.is_null() {
        if cJSON_IsString(label) == 0
            || (*label).valuestring.is_null()
            || *(*label).valuestring.offset(0isize) as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int
            || strlen((*label).valuestring) >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>()
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid tag label\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*tag).label as *mut ::core::ffi::c_char,
            (*label).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            0 as size_t,
        );
    } else if creating {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Tag label is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut color: *mut cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"color\0".as_ptr() as *const ::core::ffi::c_char);
    if !color.is_null() {
        if cJSON_IsString(color) == 0
            || (*color).valuestring.is_null()
            || strlen((*color).valuestring) >= ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
            || !valid_color((*color).valuestring)
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"color must be empty or a #RRGGBB value\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*tag).color as *mut ::core::ffi::c_char,
            (*color).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
    }
    let mut description: *mut cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"description\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !description.is_null() {
        if cJSON_IsString(description) == 0
            || (*description).valuestring.is_null()
            || strlen((*description).valuestring)
                >= ::core::mem::size_of::<[::core::ffi::c_char; 512]>()
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid tag description\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*tag).description as *mut ::core::ffi::c_char,
            (*description).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    return r#true != 0;
}
unsafe extern "C" fn extract_tag_uuid(
    mut req: *const http_request_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut uuid_size: size_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut path_value: [::core::ffi::c_char; 512] = [0; 512];
    if http_request_extract_path_param(
        req,
        b"/api/camera-tags/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut path_value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid camera tag path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut slash: *mut ::core::ffi::c_char = strchr(
        &raw mut path_value as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if !slash.is_null() {
        *slash = '\0' as ::core::ffi::c_char;
    }
    if !lightnvr_uuid_is_valid(&raw mut path_value as *mut ::core::ffi::c_char)
        || strlen(&raw mut path_value as *mut ::core::ffi::c_char) >= uuid_size
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid camera tag UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        uuid,
        &raw mut path_value as *mut ::core::ffi::c_char,
        uuid_size,
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn extract_camera_uuid(
    mut req: *const http_request_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut uuid_size: size_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut path_value: [::core::ffi::c_char; 512] = [0; 512];
    if http_request_extract_path_param(
        req,
        b"/api/cameras/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut path_value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid camera path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut slash: *mut ::core::ffi::c_char = strchr(
        &raw mut path_value as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if !slash.is_null() {
        *slash = '\0' as ::core::ffi::c_char;
    }
    if !lightnvr_uuid_is_valid(&raw mut path_value as *mut ::core::ffi::c_char)
        || strlen(&raw mut path_value as *mut ::core::ffi::c_char) >= uuid_size
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid camera UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        uuid,
        &raw mut path_value as *mut ::core::ffi::c_char,
        uuid_size,
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn load_visible_cameras(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut cameras: *mut *mut fleet_camera_t,
    mut count: *mut ::core::ffi::c_int,
    mut all_fleet: *mut bool,
) -> bool {
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
    memset(
        &raw mut user as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<user_t>(),
    );
    *cameras = ::core::ptr::null_mut::<fleet_camera_t>();
    *count = 0 as ::core::ffi::c_int;
    *all_fleet = r#false != 0;
    if httpd_check_action_access(req, &raw mut user) == 0 {
        http_response_set_json_error(
            res,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
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
    *all_fleet = authorization_evaluate(
        &raw mut user,
        authorization_action_t::AUTHZ_LIVE_VIEW,
        ::core::ptr::null::<fleet_camera_t>(),
        &raw mut evaluation,
    ) == 0 as ::core::ffi::c_int
        && evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0;
    if db_fleet_camera_load(cameras, count) != 0 as ::core::ffi::c_int
        || authorization_filter_cameras(
            &raw mut user,
            authorization_action_t::AUTHZ_LIVE_VIEW,
            *cameras,
            count,
        ) != 0 as ::core::ffi::c_int
    {
        free(*cameras as *mut ::core::ffi::c_void);
        *cameras = ::core::ptr::null_mut::<fleet_camera_t>();
        *count = 0 as ::core::ffi::c_int;
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Authorization policy evaluation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn visible_tag_camera_count(
    mut cameras: *const fleet_camera_t,
    mut camera_count: ::core::ffi::c_int,
    mut tag_uuid: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < camera_count {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < (*cameras.offset(i as isize)).tag_count {
            if strcmp(
                &raw const (*(&raw const (*cameras.offset(i as isize)).tags
                    as *const fleet_camera_tag_t)
                    .offset(j as isize))
                .uuid as *const ::core::ffi::c_char,
                tag_uuid,
            ) == 0 as ::core::ffi::c_int
            {
                count += 1;
                break;
            } else {
                j += 1;
            }
        }
        i += 1;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_camera_tags(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut all_fleet: bool = r#false != 0;
    if !load_visible_cameras(
        req,
        res,
        &raw mut cameras,
        &raw mut camera_count,
        &raw mut all_fleet,
    ) {
        return;
    }
    let mut total: ::core::ffi::c_int = db_camera_tag_count();
    if total < 0 as ::core::ffi::c_int {
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to count camera tags\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut tags: *mut camera_tag_t = (if total > 0 as ::core::ffi::c_int {
        calloc(total as size_t, ::core::mem::size_of::<camera_tag_t>())
    } else {
        NULL
    }) as *mut camera_tag_t;
    if total > 0 as ::core::ffi::c_int && tags.is_null() {
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int = if total > 0 as ::core::ffi::c_int {
        db_camera_tag_list(tags, total)
    } else {
        0 as ::core::ffi::c_int
    };
    if count < 0 as ::core::ffi::c_int {
        free(cameras as *mut ::core::ffi::c_void);
        free(tags as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to list camera tags\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || items.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(cameras as *mut ::core::ffi::c_void);
        free(tags as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddItemToObject(
        root,
        b"tags\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    let mut visible_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut scoped_count: ::core::ffi::c_int = if all_fleet as ::core::ffi::c_int != 0 {
            (*tags.offset(i as isize)).camera_count
        } else {
            visible_tag_camera_count(
                cameras,
                camera_count,
                &raw mut (*tags.offset(i as isize)).uuid as *mut ::core::ffi::c_char,
            )
        };
        if !(!all_fleet && scoped_count == 0 as ::core::ffi::c_int) {
            let mut filtered: camera_tag_t = *tags.offset(i as isize);
            filtered.camera_count = scoped_count;
            let mut item: *mut cJSON = tag_to_json(&raw mut filtered);
            if item.is_null() {
                cJSON_Delete(root);
                free(tags as *mut ::core::ffi::c_void);
                free(cameras as *mut ::core::ffi::c_void);
                http_response_set_json_error(
                    res,
                    500 as ::core::ffi::c_int,
                    b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            cJSON_AddItemToArray(items, item);
            visible_count += 1;
        }
        i += 1;
    }
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        visible_count as ::core::ffi::c_double,
    );
    free(tags as *mut ::core::ffi::c_void);
    free(cameras as *mut ::core::ffi::c_void);
    let mut json: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if json.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json);
    free(json as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_camera_tag(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    if body.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON request body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut tag: camera_tag_t = camera_tag_t {
        uuid: [0; 37],
        label: [0; 256],
        color: [0; 16],
        description: [0; 512],
        camera_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    memset(
        &raw mut tag as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<camera_tag_t>(),
    );
    if !apply_tag_fields(body, &raw mut tag, r#true != 0, res) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    let mut result: db_camera_tag_result_t = db_camera_tag_create(&raw mut tag);
    if result.0 != db_camera_tag_result_t::DB_CAMERA_TAG_OK.0 {
        set_db_error(res, result);
        return;
    }
    set_tag_json(res, 201 as ::core::ffi::c_int, &raw mut tag);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_camera_tag(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_tag_uuid(
        req,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) {
        return;
    }
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut all_fleet: bool = r#false != 0;
    if !load_visible_cameras(
        req,
        res,
        &raw mut cameras,
        &raw mut camera_count,
        &raw mut all_fleet,
    ) {
        return;
    }
    let mut scoped_count: ::core::ffi::c_int = visible_tag_camera_count(
        cameras,
        camera_count,
        &raw mut uuid as *mut ::core::ffi::c_char,
    );
    free(cameras as *mut ::core::ffi::c_void);
    if !all_fleet && scoped_count == 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut tag: camera_tag_t = camera_tag_t {
        uuid: [0; 37],
        label: [0; 256],
        color: [0; 16],
        description: [0; 512],
        camera_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_camera_tag_result_t =
        db_camera_tag_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut tag);
    if result.0 != db_camera_tag_result_t::DB_CAMERA_TAG_OK.0 {
        set_db_error(res, result);
        return;
    }
    if !all_fleet {
        tag.camera_count = scoped_count;
    }
    set_tag_json(res, 200 as ::core::ffi::c_int, &raw mut tag);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_camera_tag(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_tag_uuid(
        req,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) {
        return;
    }
    let mut tag: camera_tag_t = camera_tag_t {
        uuid: [0; 37],
        label: [0; 256],
        color: [0; 16],
        description: [0; 512],
        camera_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_camera_tag_result_t =
        db_camera_tag_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut tag);
    if result.0 != db_camera_tag_result_t::DB_CAMERA_TAG_OK.0 {
        set_db_error(res, result);
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    if body.is_null() {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON request body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !apply_tag_fields(body, &raw mut tag, r#false != 0, res) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    result = db_camera_tag_update(&raw mut tag);
    if result.0 != db_camera_tag_result_t::DB_CAMERA_TAG_OK.0 {
        set_db_error(res, result);
        return;
    }
    set_tag_json(res, 200 as ::core::ffi::c_int, &raw mut tag);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_camera_tag(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_tag_uuid(
        req,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) {
        return;
    }
    let mut result: db_camera_tag_result_t =
        db_camera_tag_delete(&raw mut uuid as *mut ::core::ffi::c_char);
    if result.0 != db_camera_tag_result_t::DB_CAMERA_TAG_OK.0 {
        set_db_error(res, result);
        return;
    }
    http_response_set_json(
        res,
        200 as ::core::ffi::c_int,
        b"{\"success\":true}\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_camera_tag_merge(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    let mut source_uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_tag_uuid(
        req,
        &raw mut source_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut target: *mut cJSON = if !body.is_null() {
        cJSON_GetObjectItemCaseSensitive(
            body,
            b"target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if body.is_null()
        || cJSON_IsObject(body) == 0
        || cJSON_IsString(target) == 0
        || !lightnvr_uuid_is_valid((*target).valuestring)
    {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"A valid target_uuid is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut target_uuid: [::core::ffi::c_char; 37] = [0; 37];
    safe_strcpy(
        &raw mut target_uuid as *mut ::core::ffi::c_char,
        (*target).valuestring,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    cJSON_Delete(body);
    let mut result: db_camera_tag_result_t = db_camera_tag_merge(
        &raw mut source_uuid as *mut ::core::ffi::c_char,
        &raw mut target_uuid as *mut ::core::ffi::c_char,
    );
    if result.0 != db_camera_tag_result_t::DB_CAMERA_TAG_OK.0 {
        set_db_error(res, result);
        return;
    }
    let mut response: *mut cJSON = cJSON_CreateObject();
    if response.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddBoolToObject(
        response,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    cJSON_AddStringToObject(
        response,
        b"source_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut source_uuid as *mut ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        response,
        b"target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut target_uuid as *mut ::core::ffi::c_char,
    );
    let mut json: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);
    if json.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json);
    free(json as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn set_camera_assignments_json(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut camera_uuid: *const ::core::ffi::c_char,
) {
    let mut tags: [camera_tag_t; 64] = [camera_tag_t {
        uuid: [0; 37],
        label: [0; 256],
        color: [0; 16],
        description: [0; 512],
        camera_count: 0,
        created_at: 0,
        updated_at: 0,
    }; 64];
    let mut count: ::core::ffi::c_int = db_camera_tag_list_for_camera(
        camera_uuid,
        &raw mut tags as *mut camera_tag_t,
        CAMERA_TAG_MAX_ASSIGNMENTS,
    );
    if count == -2 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Camera not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if count < 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to list camera tags\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut all_fleet: bool = r#false != 0;
    if !load_visible_cameras(
        req,
        res,
        &raw mut cameras,
        &raw mut camera_count,
        &raw mut all_fleet,
    ) {
        return;
    }
    if !all_fleet {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < count {
            tags[i as usize].camera_count = visible_tag_camera_count(
                cameras,
                camera_count,
                &raw mut (*(&raw mut tags as *mut camera_tag_t).offset(i as isize)).uuid
                    as *mut ::core::ffi::c_char,
            );
            i += 1;
        }
    }
    free(cameras as *mut ::core::ffi::c_void);
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
        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        camera_uuid,
    );
    cJSON_AddItemToObject(
        root,
        b"tags\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        count as ::core::ffi::c_double,
    );
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < count {
        let mut item: *mut cJSON =
            tag_to_json((&raw mut tags as *mut camera_tag_t).offset(i_0 as isize));
        if item.is_null() {
            cJSON_Delete(root);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddItemToArray(items, item);
        i_0 += 1;
    }
    let mut json: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if json.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, json);
    free(json as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_camera_tag_assignments(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut camera_uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_camera_uuid(
        req,
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) {
        return;
    }
    let mut c2rust_lvalue: user_t = user_t {
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
    let mut c2rust_lvalue_0: fleet_camera_t = fleet_camera_t {
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
    let mut c2rust_lvalue_1: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
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
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut c2rust_lvalue,
        &raw mut c2rust_lvalue_0,
        &raw mut c2rust_lvalue_1,
    ) == 0
    {
        return;
    }
    set_camera_assignments_json(req, res, &raw mut camera_uuid as *mut ::core::ffi::c_char);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_camera_tag_assignments(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut camera_uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_camera_uuid(
        req,
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) {
        return;
    }
    let mut c2rust_lvalue: user_t = user_t {
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
    let mut c2rust_lvalue_0: fleet_camera_t = fleet_camera_t {
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
    let mut c2rust_lvalue_1: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
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
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut c2rust_lvalue,
        &raw mut c2rust_lvalue_0,
        &raw mut c2rust_lvalue_1,
    ) == 0
    {
        return;
    }
    let mut c2rust_lvalue_2: user_t = user_t {
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
    let mut c2rust_lvalue_3: fleet_camera_t = fleet_camera_t {
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
    let mut c2rust_lvalue_4: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
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
        authorization_action_t::AUTHZ_LIVE_VIEW,
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut c2rust_lvalue_2,
        &raw mut c2rust_lvalue_3,
        &raw mut c2rust_lvalue_4,
    ) == 0
    {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut items: *mut cJSON = if !body.is_null() {
        cJSON_GetObjectItemCaseSensitive(
            body,
            b"tag_uuids\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if body.is_null() || cJSON_IsObject(body) == 0 || cJSON_IsArray(items) == 0 {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"tag_uuids must be an array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int = cJSON_GetArraySize(items);
    if count < 0 as ::core::ffi::c_int || count > CAMERA_TAG_MAX_ASSIGNMENTS {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Too many camera tags\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut tag_storage: [[::core::ffi::c_char; 37]; 64] = [[0; 37]; 64];
    let mut tag_uuids: [*const ::core::ffi::c_char; 64] =
        [::core::ptr::null::<::core::ffi::c_char>(); 64];
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut item: *mut cJSON = cJSON_GetArrayItem(items, i);
        if cJSON_IsString(item) == 0 || !lightnvr_uuid_is_valid((*item).valuestring) {
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"tag_uuids contains an invalid UUID\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        safe_strcpy(
            &raw mut *(&raw mut tag_storage as *mut [::core::ffi::c_char; 37]).offset(i as isize)
                as *mut ::core::ffi::c_char,
            (*item).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
        tag_uuids[i as usize] = &raw mut *(&raw mut tag_storage as *mut [::core::ffi::c_char; 37])
            .offset(i as isize) as *mut ::core::ffi::c_char;
        i += 1;
    }
    cJSON_Delete(body);
    let mut result: db_camera_tag_result_t = db_camera_tag_set_for_camera(
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        &raw mut tag_uuids as *mut *const ::core::ffi::c_char,
        count,
    );
    if result.0 != db_camera_tag_result_t::DB_CAMERA_TAG_OK.0 {
        set_db_error(res, result);
        return;
    }
    set_camera_assignments_json(req, res, &raw mut camera_uuid as *mut ::core::ffi::c_char);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
