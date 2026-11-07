unsafe extern "C" {
    pub type fleet_selector;
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
    unsafe fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    unsafe fn fleet_selector_parse(
        json: *const cJSON,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> *mut fleet_selector_t;
    unsafe fn fleet_selector_free(selector: *mut fleet_selector_t);
    unsafe fn fleet_selector_matches(
        selector: *const fleet_selector_t,
        camera: *const fleet_camera_t,
        explanation: *mut fleet_selector_explanation_t,
    ) -> bool;
    unsafe fn db_camera_collection_count() -> ::core::ffi::c_int;
    unsafe fn db_camera_collection_list(
        collections: *mut camera_collection_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_camera_collection_get(
        uuid: *const ::core::ffi::c_char,
        collection: *mut camera_collection_t,
    ) -> db_camera_collection_result_t;
    unsafe fn db_camera_collection_create(
        collection: *mut camera_collection_t,
    ) -> db_camera_collection_result_t;
    unsafe fn db_camera_collection_update(
        collection: *mut camera_collection_t,
    ) -> db_camera_collection_result_t;
    unsafe fn db_camera_collection_delete(
        uuid: *const ::core::ffi::c_char,
    ) -> db_camera_collection_result_t;
    unsafe fn db_camera_collection_list_members(
        collection_uuid: *const ::core::ffi::c_char,
        camera_uuids: *mut [::core::ffi::c_char; 37],
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_camera_collection_set_members(
        collection_uuid: *const ::core::ffi::c_char,
        camera_uuids: *const *const ::core::ffi::c_char,
        camera_count: ::core::ffi::c_int,
    ) -> db_camera_collection_result_t;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fleet_camera_enrich_runtime_health(
        cameras: *mut fleet_camera_t,
        count: ::core::ffi::c_int,
    );
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
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
    unsafe fn authorization_filter_visible_cameras(
        user: *const user_t,
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
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_selector_explanation_t {
    pub clauses: [[::core::ffi::c_char; 160]; 64],
    pub clause_count: ::core::ffi::c_int,
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
pub struct camera_collection_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub description: [::core::ffi::c_char; 512],
    pub collection_type: [::core::ffi::c_char; 16],
    pub selector_json: [::core::ffi::c_char; 8192],
    pub is_shared: bool,
    pub owner_user_id: int64_t,
    pub member_count: ::core::ffi::c_int,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_camera_collection_result_t(pub ::core::ffi::c_int);
impl db_camera_collection_result_t {
    pub const DB_CAMERA_COLLECTION_OK: Self = Self(0);
    pub const DB_CAMERA_COLLECTION_NOT_FOUND: Self = Self(-1);
    pub const DB_CAMERA_COLLECTION_CONFLICT: Self = Self(-2);
    pub const DB_CAMERA_COLLECTION_INVALID: Self = Self(-3);
    pub const DB_CAMERA_COLLECTION_ERROR: Self = Self(-4);
    pub const DB_CAMERA_COLLECTION_WRONG_TYPE: Self = Self(-5);
    pub const DB_CAMERA_COLLECTION_LIMIT: Self = Self(-6);
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
    pub const AUTHZ_ACTION_COUNT: Self = Self(19);
}
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const CAMERA_COLLECTION_MAX_MEMBERS: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const COLLECTION_PREVIEW_MAX: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
unsafe extern "C" fn valid_uuid(mut value: *const ::core::ffi::c_char) -> bool {
    if value.is_null()
        || strlen(value) != (CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int) as size_t
    {
        return r#false != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < CAMERA_UUID_STRING_SIZE - 1 as ::core::ffi::c_int {
        let mut c: ::core::ffi::c_uchar = *value.offset(i as isize) as ::core::ffi::c_uchar;
        if i == 8 as ::core::ffi::c_int
            || i == 13 as ::core::ffi::c_int
            || i == 18 as ::core::ffi::c_int
            || i == 23 as ::core::ffi::c_int
        {
            if c as ::core::ffi::c_int != '-' as ::core::ffi::c_int {
                return r#false != 0;
            }
        } else if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
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
unsafe extern "C" fn principal_can_manage_collections(mut user: *const user_t) -> bool {
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
    return authorization_evaluate(
        user,
        authorization_action_t::AUTHZ_CAMERA_CONFIGURE,
        ::core::ptr::null::<fleet_camera_t>(),
        &raw mut evaluation,
    ) == 0 as ::core::ffi::c_int
        && evaluation.decision.0 == authorization_decision_t::AUTHZ_DECISION_ALLOW.0;
}
unsafe extern "C" fn owns_collection(
    mut user: *const user_t,
    mut collection: *const camera_collection_t,
) -> bool {
    return (*collection).owner_user_id > 0 as int64_t && (*collection).owner_user_id == (*user).id;
}
unsafe extern "C" fn can_view_collection(
    mut user: *const user_t,
    mut collection: *const camera_collection_t,
    mut can_manage: bool,
) -> bool {
    return can_manage as ::core::ffi::c_int != 0
        || (*collection).is_shared as ::core::ffi::c_int != 0
        || (*collection).owner_user_id > 0 as int64_t && (*collection).owner_user_id == (*user).id;
}
unsafe extern "C" fn extract_collection_uuid(
    mut req: *const http_request_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut uuid_size: size_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut value: [::core::ffi::c_char; 512] = [0; 512];
    if http_request_extract_path_param(
        req,
        b"/api/camera-collections/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid collection path\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut slash: *mut ::core::ffi::c_char = strchr(
        &raw mut value as *mut ::core::ffi::c_char,
        '/' as ::core::ffi::c_int,
    );
    if !slash.is_null() {
        *slash = '\0' as ::core::ffi::c_char;
    }
    if !valid_uuid(&raw mut value as *mut ::core::ffi::c_char)
        || strlen(&raw mut value as *mut ::core::ffi::c_char) >= uuid_size
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid collection UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        uuid,
        &raw mut value as *mut ::core::ffi::c_char,
        uuid_size,
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn set_db_error(
    mut res: *mut http_response_t,
    mut result: db_camera_collection_result_t,
) {
    match result {
        db_camera_collection_result_t::DB_CAMERA_COLLECTION_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Camera or collection not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_camera_collection_result_t::DB_CAMERA_COLLECTION_CONFLICT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Collection name conflicts or the collection is used by an access policy\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_camera_collection_result_t::DB_CAMERA_COLLECTION_WRONG_TYPE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Only static collections have explicit members\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_camera_collection_result_t::DB_CAMERA_COLLECTION_LIMIT => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Collection member limit exceeded\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_camera_collection_result_t::DB_CAMERA_COLLECTION_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid camera collection request\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Camera collection operation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn load_authorized_fleet(
    mut user: *const user_t,
    mut cameras: *mut *mut fleet_camera_t,
    mut count: *mut ::core::ffi::c_int,
) -> bool {
    if db_fleet_camera_load(cameras, count) != 0 as ::core::ffi::c_int {
        return r#false != 0;
    }
    fleet_camera_enrich_runtime_health(*cameras, *count);
    if authorization_filter_visible_cameras(user, *cameras, count) != 0 as ::core::ffi::c_int {
        free(*cameras as *mut ::core::ffi::c_void);
        *cameras = ::core::ptr::null_mut::<fleet_camera_t>();
        *count = 0 as ::core::ffi::c_int;
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn uuid_in_members(
    mut uuid: *const ::core::ffi::c_char,
    mut members: *mut [::core::ffi::c_char; 37],
    mut member_count: ::core::ffi::c_int,
) -> bool {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < member_count {
        if strcasecmp(
            uuid,
            &raw mut *members.offset(i as isize) as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        i += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn collection_matches(
    mut collection: *const camera_collection_t,
    mut cameras: *mut fleet_camera_t,
    mut camera_count: ::core::ffi::c_int,
    mut matched_out: *mut *mut *mut fleet_camera_t,
) -> ::core::ffi::c_int {
    let mut matched: *mut *mut fleet_camera_t = (if camera_count > 0 as ::core::ffi::c_int {
        calloc(
            camera_count as size_t,
            ::core::mem::size_of::<*mut fleet_camera_t>(),
        )
    } else {
        NULL
    }) as *mut *mut fleet_camera_t;
    if camera_count > 0 as ::core::ffi::c_int && matched.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let mut matched_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if strcmp(
        &raw const (*collection).collection_type as *const ::core::ffi::c_char,
        b"static\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut member_capacity: ::core::ffi::c_int = (*collection).member_count;
        let mut members: *mut [::core::ffi::c_char; 37] =
            (if member_capacity > 0 as ::core::ffi::c_int {
                calloc(
                    member_capacity as size_t,
                    ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                )
            } else {
                NULL
            }) as *mut [::core::ffi::c_char; 37];
        if member_capacity > 0 as ::core::ffi::c_int && members.is_null() {
            free(matched as *mut ::core::ffi::c_void);
            return -1 as ::core::ffi::c_int;
        }
        let mut member_count: ::core::ffi::c_int = if member_capacity > 0 as ::core::ffi::c_int {
            db_camera_collection_list_members(
                &raw const (*collection).uuid as *const ::core::ffi::c_char,
                members,
                member_capacity,
            )
        } else {
            0 as ::core::ffi::c_int
        };
        if member_count < 0 as ::core::ffi::c_int {
            free(members as *mut ::core::ffi::c_void);
            free(matched as *mut ::core::ffi::c_void);
            return -1 as ::core::ffi::c_int;
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < camera_count {
            if uuid_in_members(
                &raw mut (*cameras.offset(i as isize)).camera_uuid as *mut ::core::ffi::c_char,
                members,
                member_count,
            ) {
                let c2rust_fresh0 = matched_count;
                matched_count += 1;
                *matched.offset(c2rust_fresh0 as isize) = cameras.offset(i as isize);
            }
            i += 1;
        }
        free(members as *mut ::core::ffi::c_void);
    } else {
        let mut selector_json: *mut cJSON =
            cJSON_Parse(&raw const (*collection).selector_json as *const ::core::ffi::c_char);
        let mut error: [::core::ffi::c_char; 256] = [
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
        let mut selector: *mut fleet_selector_t = fleet_selector_parse(
            selector_json,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        );
        cJSON_Delete(selector_json);
        if selector.is_null() {
            free(matched as *mut ::core::ffi::c_void);
            return -1 as ::core::ffi::c_int;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < camera_count {
            if fleet_selector_matches(
                selector,
                cameras.offset(i_0 as isize),
                ::core::ptr::null_mut::<fleet_selector_explanation_t>(),
            ) {
                let c2rust_fresh1 = matched_count;
                matched_count += 1;
                *matched.offset(c2rust_fresh1 as isize) = cameras.offset(i_0 as isize);
            }
            i_0 += 1;
        }
        fleet_selector_free(selector);
    }
    *matched_out = matched;
    return matched_count;
}
unsafe extern "C" fn collection_to_json(
    mut collection: *const camera_collection_t,
    mut member_count: ::core::ffi::c_int,
    mut effective_count: ::core::ffi::c_int,
    mut include_selector: bool,
) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_CreateObject();
    if object.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*collection).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*collection).name as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"description\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*collection).description as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*collection).collection_type as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        object,
        b"shared\0".as_ptr() as *const ::core::ffi::c_char,
        (*collection).is_shared as cJSON_bool,
    );
    if (*collection).owner_user_id > 0 as int64_t {
        cJSON_AddNumberToObject(
            object,
            b"owner_user_id\0".as_ptr() as *const ::core::ffi::c_char,
            (*collection).owner_user_id as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(
            object,
            b"owner_user_id\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddNumberToObject(
        object,
        b"member_count\0".as_ptr() as *const ::core::ffi::c_char,
        member_count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"effective_count\0".as_ptr() as *const ::core::ffi::c_char,
        effective_count as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*collection).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*collection).updated_at as ::core::ffi::c_double,
    );
    if strcmp(
        &raw const (*collection).collection_type as *const ::core::ffi::c_char,
        b"smart\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && include_selector as ::core::ffi::c_int != 0
    {
        let mut selector: *mut cJSON =
            cJSON_Parse(&raw const (*collection).selector_json as *const ::core::ffi::c_char);
        if selector.is_null() {
            cJSON_Delete(object);
            return ::core::ptr::null_mut::<cJSON>();
        }
        cJSON_AddItemToObject(
            object,
            b"selector\0".as_ptr() as *const ::core::ffi::c_char,
            selector,
        );
    } else {
        cJSON_AddNullToObject(object, b"selector\0".as_ptr() as *const ::core::ffi::c_char);
    }
    cJSON_AddBoolToObject(
        object,
        b"selector_redacted\0".as_ptr() as *const ::core::ffi::c_char,
        (strcmp(
            &raw const (*collection).collection_type as *const ::core::ffi::c_char,
            b"smart\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && !include_selector) as ::core::ffi::c_int,
    );
    return object;
}
unsafe extern "C" fn apply_fields(
    mut body: *mut cJSON,
    mut collection: *mut camera_collection_t,
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
    let mut name: *mut cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"name\0".as_ptr() as *const ::core::ffi::c_char);
    if !name.is_null() {
        if cJSON_IsString(name) == 0
            || (*name).valuestring.is_null()
            || *(*name).valuestring.offset(0isize) as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int
            || strlen((*name).valuestring) >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid collection name\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*collection).name as *mut ::core::ffi::c_char,
            (*name).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
    } else if creating {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Collection name is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
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
                b"Invalid collection description\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*collection).description as *mut ::core::ffi::c_char,
            (*description).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
            0 as size_t,
        );
    }
    let mut r#type: *mut cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"type\0".as_ptr() as *const ::core::ffi::c_char);
    if !r#type.is_null() {
        if cJSON_IsString(r#type) == 0
            || (*r#type).valuestring.is_null()
            || strcmp(
                (*r#type).valuestring,
                b"static\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
                && strcmp(
                    (*r#type).valuestring,
                    b"smart\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"type must be static or smart\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*collection).collection_type as *mut ::core::ffi::c_char,
            (*r#type).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>(),
            0 as size_t,
        );
    } else if creating {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Collection type is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut shared: *mut cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"shared\0".as_ptr() as *const ::core::ffi::c_char);
    if !shared.is_null() {
        if cJSON_IsBool(shared) == 0 {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"shared must be boolean\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*collection).is_shared = cJSON_IsTrue(shared) != 0;
    }
    let mut selector: *mut cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !selector.is_null() {
        let mut error: [::core::ffi::c_char; 256] = [
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
        let mut parsed: *mut fleet_selector_t = fleet_selector_parse(
            selector,
            &raw mut error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        );
        if parsed.is_null() {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                if error[0usize] as ::core::ffi::c_int != 0 {
                    &raw mut error as *mut ::core::ffi::c_char
                } else {
                    b"Invalid collection selector\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            return r#false != 0;
        }
        fleet_selector_free(parsed);
        let mut serialized: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(selector);
        if serialized.is_null()
            || strlen(serialized) >= ::core::mem::size_of::<[::core::ffi::c_char; 8192]>()
        {
            free(serialized as *mut ::core::ffi::c_void);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Collection selector is too large\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*collection).selector_json as *mut ::core::ffi::c_char,
            serialized,
            ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
            0 as size_t,
        );
        free(serialized as *mut ::core::ffi::c_void);
    }
    if strcmp(
        &raw mut (*collection).collection_type as *mut ::core::ffi::c_char,
        b"smart\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && (*collection).selector_json[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Smart collections require selector\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if strcmp(
        &raw mut (*collection).collection_type as *mut ::core::ffi::c_char,
        b"static\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        (*collection).selector_json[0usize] = '\0' as ::core::ffi::c_char;
    }
    return r#true != 0;
}
unsafe extern "C" fn set_collection_response(
    mut res: *mut http_response_t,
    mut status: ::core::ffi::c_int,
    mut collection: *const camera_collection_t,
    mut user: *const user_t,
    mut reject_empty_scoped: bool,
) {
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !load_authorized_fleet(user, &raw mut cameras, &raw mut camera_count) {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load fleet cameras\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut matched: *mut *mut fleet_camera_t = ::core::ptr::null_mut::<*mut fleet_camera_t>();
    let mut effective_count: ::core::ffi::c_int =
        collection_matches(collection, cameras, camera_count, &raw mut matched);
    free(matched as *mut ::core::ffi::c_void);
    free(cameras as *mut ::core::ffi::c_void);
    if effective_count < 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to evaluate collection\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut can_manage: bool = principal_can_manage_collections(user);
    let mut owner: bool = owns_collection(user, collection);
    if reject_empty_scoped as ::core::ffi::c_int != 0
        && effective_count == 0 as ::core::ffi::c_int
        && !can_manage
        && !owner
    {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Collection not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut include_selector: bool =
        can_manage as ::core::ffi::c_int != 0 || owner as ::core::ffi::c_int != 0;
    let mut object: *mut cJSON = collection_to_json(
        collection,
        effective_count,
        effective_count,
        include_selector,
    );
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
            b"Failed to serialize collection\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, status, json);
    free(json as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_camera_collections(
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
    let mut total: ::core::ffi::c_int = db_camera_collection_count();
    if total < 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to count collections\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut collections: *mut camera_collection_t = (if total > 0 as ::core::ffi::c_int {
        calloc(
            total as size_t,
            ::core::mem::size_of::<camera_collection_t>(),
        )
    } else {
        NULL
    }) as *mut camera_collection_t;
    if total > 0 as ::core::ffi::c_int && collections.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int = if total > 0 as ::core::ffi::c_int {
        db_camera_collection_list(collections, total)
    } else {
        0 as ::core::ffi::c_int
    };
    if count < 0 as ::core::ffi::c_int {
        free(collections as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to list collections\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !load_authorized_fleet(&raw mut user, &raw mut cameras, &raw mut camera_count) {
        free(collections as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load fleet cameras\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut can_manage: bool = principal_can_manage_collections(&raw mut user);
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || items.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(cameras as *mut ::core::ffi::c_void);
        free(collections as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddItemToObject(
        root,
        b"collections\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    let mut visible_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        if can_view_collection(&raw mut user, collections.offset(i as isize), can_manage) {
            let mut matched: *mut *mut fleet_camera_t =
                ::core::ptr::null_mut::<*mut fleet_camera_t>();
            let mut effective_count: ::core::ffi::c_int = collection_matches(
                collections.offset(i as isize),
                cameras,
                camera_count,
                &raw mut matched,
            );
            free(matched as *mut ::core::ffi::c_void);
            if effective_count < 0 as ::core::ffi::c_int {
                cJSON_Delete(root);
                free(cameras as *mut ::core::ffi::c_void);
                free(collections as *mut ::core::ffi::c_void);
                http_response_set_json_error(
                    res,
                    500 as ::core::ffi::c_int,
                    b"Failed to evaluate collection\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            let mut owner: bool = owns_collection(&raw mut user, collections.offset(i as isize));
            if !(effective_count == 0 as ::core::ffi::c_int && !can_manage && !owner) {
                let mut include_selector: bool =
                    can_manage as ::core::ffi::c_int != 0 || owner as ::core::ffi::c_int != 0;
                let mut item: *mut cJSON = collection_to_json(
                    collections.offset(i as isize),
                    effective_count,
                    effective_count,
                    include_selector,
                );
                if item.is_null() {
                    cJSON_Delete(root);
                    free(cameras as *mut ::core::ffi::c_void);
                    free(collections as *mut ::core::ffi::c_void);
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
        }
        i += 1;
    }
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        visible_count as ::core::ffi::c_double,
    );
    free(cameras as *mut ::core::ffi::c_void);
    free(collections as *mut ::core::ffi::c_void);
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
pub unsafe extern "C" fn handle_post_camera_collection(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
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
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut collection: camera_collection_t = camera_collection_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        collection_type: [0; 16],
        selector_json: [0; 8192],
        is_shared: false,
        owner_user_id: 0,
        member_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    memset(
        &raw mut collection as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<camera_collection_t>(),
    );
    collection.is_shared = r#true != 0;
    collection.owner_user_id = user.id;
    if !apply_fields(body, &raw mut collection, r#true != 0, res) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    let mut result: db_camera_collection_result_t =
        db_camera_collection_create(&raw mut collection);
    if result.0 != db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK.0 {
        set_db_error(res, result);
        return;
    }
    set_collection_response(
        res,
        201 as ::core::ffi::c_int,
        &raw mut collection,
        &raw mut user,
        r#false != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_camera_collection(
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
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_collection_uuid(
        req,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) {
        return;
    }
    let mut collection: camera_collection_t = camera_collection_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        collection_type: [0; 16],
        selector_json: [0; 8192],
        is_shared: false,
        owner_user_id: 0,
        member_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_camera_collection_result_t = db_camera_collection_get(
        &raw mut uuid as *mut ::core::ffi::c_char,
        &raw mut collection,
    );
    if result.0 != db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK.0 {
        set_db_error(res, result);
        return;
    }
    let mut can_manage: bool = principal_can_manage_collections(&raw mut user);
    if !can_view_collection(&raw mut user, &raw mut collection, can_manage) {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Collection not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    set_collection_response(
        res,
        200 as ::core::ffi::c_int,
        &raw mut collection,
        &raw mut user,
        r#true != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_camera_collection(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
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
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_collection_uuid(
        req,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) {
        return;
    }
    let mut collection: camera_collection_t = camera_collection_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        collection_type: [0; 16],
        selector_json: [0; 8192],
        is_shared: false,
        owner_user_id: 0,
        member_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_camera_collection_result_t = db_camera_collection_get(
        &raw mut uuid as *mut ::core::ffi::c_char,
        &raw mut collection,
    );
    if result.0 != db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK.0 {
        set_db_error(res, result);
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    if !apply_fields(body, &raw mut collection, r#false != 0, res) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);
    result = db_camera_collection_update(&raw mut collection);
    if result.0 != db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK.0 {
        set_db_error(res, result);
        return;
    }
    set_collection_response(
        res,
        200 as ::core::ffi::c_int,
        &raw mut collection,
        &raw mut user,
        r#false != 0,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_camera_collection(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_collection_uuid(
        req,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) {
        return;
    }
    let mut result: db_camera_collection_result_t =
        db_camera_collection_delete(&raw mut uuid as *mut ::core::ffi::c_char);
    if result.0 != db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK.0 {
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
pub unsafe extern "C" fn handle_get_camera_collection_members(
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
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_collection_uuid(
        req,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) {
        return;
    }
    let mut collection: camera_collection_t = camera_collection_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        collection_type: [0; 16],
        selector_json: [0; 8192],
        is_shared: false,
        owner_user_id: 0,
        member_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_camera_collection_result_t = db_camera_collection_get(
        &raw mut uuid as *mut ::core::ffi::c_char,
        &raw mut collection,
    );
    if result.0 != db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK.0 {
        set_db_error(res, result);
        return;
    }
    let mut can_manage: bool = principal_can_manage_collections(&raw mut user);
    if !can_view_collection(&raw mut user, &raw mut collection, can_manage) {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Collection not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if strcmp(
        &raw mut collection.collection_type as *mut ::core::ffi::c_char,
        b"static\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        set_db_error(
            res,
            db_camera_collection_result_t::DB_CAMERA_COLLECTION_WRONG_TYPE,
        );
        return;
    }
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !load_authorized_fleet(&raw mut user, &raw mut cameras, &raw mut camera_count) {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load fleet cameras\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut matched: *mut *mut fleet_camera_t = ::core::ptr::null_mut::<*mut fleet_camera_t>();
    let mut matched_count: ::core::ffi::c_int =
        collection_matches(&raw mut collection, cameras, camera_count, &raw mut matched);
    if matched_count < 0 as ::core::ffi::c_int {
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to list members\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if matched_count == 0 as ::core::ffi::c_int
        && !can_manage
        && !owns_collection(&raw mut user, &raw mut collection)
    {
        free(matched as *mut ::core::ffi::c_void);
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Collection not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || items.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(matched as *mut ::core::ffi::c_void);
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddStringToObject(
        root,
        b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut uuid as *mut ::core::ffi::c_char,
    );
    cJSON_AddItemToObject(
        root,
        b"camera_uuids\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        matched_count as ::core::ffi::c_double,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < matched_count {
        cJSON_AddItemToArray(
            items,
            cJSON_CreateString(
                &raw mut (**matched.offset(i as isize)).camera_uuid as *mut ::core::ffi::c_char,
            ),
        );
        i += 1;
    }
    free(matched as *mut ::core::ffi::c_void);
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
pub unsafe extern "C" fn handle_put_camera_collection_members(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    if httpd_authorize_global_action(req, res, authorization_action_t::AUTHZ_CAMERA_CONFIGURE) == 0
    {
        return;
    }
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_collection_uuid(
        req,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) {
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut items: *mut cJSON = if !body.is_null() {
        cJSON_GetObjectItemCaseSensitive(
            body,
            b"camera_uuids\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if cJSON_IsObject(body) == 0 || cJSON_IsArray(items) == 0 {
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"camera_uuids must be an array\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int = cJSON_GetArraySize(items);
    if count < 0 as ::core::ffi::c_int || count > CAMERA_COLLECTION_MAX_MEMBERS {
        cJSON_Delete(body);
        set_db_error(
            res,
            db_camera_collection_result_t::DB_CAMERA_COLLECTION_LIMIT,
        );
        return;
    }
    let mut storage: *mut [::core::ffi::c_char; 37] = (if count > 0 as ::core::ffi::c_int {
        calloc(
            count as size_t,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        )
    } else {
        NULL
    }) as *mut [::core::ffi::c_char; 37];
    let mut camera_uuids: *mut *const ::core::ffi::c_char = (if count > 0 as ::core::ffi::c_int {
        calloc(
            count as size_t,
            ::core::mem::size_of::<*const ::core::ffi::c_char>(),
        )
    } else {
        NULL
    })
        as *mut *const ::core::ffi::c_char;
    if count > 0 as ::core::ffi::c_int && (storage.is_null() || camera_uuids.is_null()) {
        free(storage as *mut ::core::ffi::c_void);
        free(camera_uuids as *mut ::core::ffi::c_void);
        cJSON_Delete(body);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut item: *mut cJSON = cJSON_GetArrayItem(items, i);
        if cJSON_IsString(item) == 0 || !valid_uuid((*item).valuestring) {
            free(storage as *mut ::core::ffi::c_void);
            free(camera_uuids as *mut ::core::ffi::c_void);
            cJSON_Delete(body);
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"camera_uuids contains invalid UUID\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        safe_strcpy(
            &raw mut *storage.offset(i as isize) as *mut ::core::ffi::c_char,
            (*item).valuestring,
            CAMERA_UUID_STRING_SIZE as size_t,
            0 as size_t,
        );
        *camera_uuids.offset(i as isize) =
            &raw mut *storage.offset(i as isize) as *mut ::core::ffi::c_char;
        i += 1;
    }
    cJSON_Delete(body);
    let mut result: db_camera_collection_result_t = db_camera_collection_set_members(
        &raw mut uuid as *mut ::core::ffi::c_char,
        camera_uuids,
        count,
    );
    free(storage as *mut ::core::ffi::c_void);
    free(camera_uuids as *mut ::core::ffi::c_void);
    if result.0 != db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK.0 {
        set_db_error(res, result);
        return;
    }
    handle_get_camera_collection_members(req, res);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_camera_collection_preview(
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
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !extract_collection_uuid(
        req,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        res,
    ) {
        return;
    }
    let mut collection: camera_collection_t = camera_collection_t {
        uuid: [0; 37],
        name: [0; 128],
        description: [0; 512],
        collection_type: [0; 16],
        selector_json: [0; 8192],
        is_shared: false,
        owner_user_id: 0,
        member_count: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut result: db_camera_collection_result_t = db_camera_collection_get(
        &raw mut uuid as *mut ::core::ffi::c_char,
        &raw mut collection,
    );
    if result.0 != db_camera_collection_result_t::DB_CAMERA_COLLECTION_OK.0 {
        set_db_error(res, result);
        return;
    }
    let mut can_manage: bool = principal_can_manage_collections(&raw mut user);
    if !can_view_collection(&raw mut user, &raw mut collection, can_manage) {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Collection not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !load_authorized_fleet(&raw mut user, &raw mut cameras, &raw mut camera_count) {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to load fleet cameras\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut matched: *mut *mut fleet_camera_t = ::core::ptr::null_mut::<*mut fleet_camera_t>();
    let mut matched_count: ::core::ffi::c_int =
        collection_matches(&raw mut collection, cameras, camera_count, &raw mut matched);
    if matched_count < 0 as ::core::ffi::c_int {
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to evaluate collection\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if matched_count == 0 as ::core::ffi::c_int
        && !can_manage
        && !owns_collection(&raw mut user, &raw mut collection)
    {
        free(matched as *mut ::core::ffi::c_void);
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Collection not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut sample: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || sample.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(sample);
        free(matched as *mut ::core::ffi::c_void);
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddStringToObject(
        root,
        b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut uuid as *mut ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        root,
        b"matched_count\0".as_ptr() as *const ::core::ffi::c_char,
        matched_count as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"sample\0".as_ptr() as *const ::core::ffi::c_char,
        sample,
    );
    let mut sample_count: ::core::ffi::c_int = if matched_count < COLLECTION_PREVIEW_MAX {
        matched_count
    } else {
        COLLECTION_PREVIEW_MAX
    };
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < sample_count {
        let mut item: *mut cJSON = cJSON_CreateObject();
        cJSON_AddStringToObject(
            item,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (**matched.offset(i as isize)).camera_uuid as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            item,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (**matched.offset(i as isize)).name as *mut ::core::ffi::c_char,
        );
        cJSON_AddStringToObject(
            item,
            b"location_path\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (**matched.offset(i as isize)).location_path as *mut ::core::ffi::c_char,
        );
        cJSON_AddItemToArray(sample, item);
        i += 1;
    }
    free(matched as *mut ::core::ffi::c_void);
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
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
