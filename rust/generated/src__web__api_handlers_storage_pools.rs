unsafe extern "C" {
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
    unsafe fn cJSON_GetArraySize(array: *const cJSON) -> ::core::ffi::c_int;
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
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn db_storage_pool_validate(
        pool: *mut storage_pool_t,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> db_storage_pool_result_t;
    unsafe fn db_storage_pool_count() -> ::core::ffi::c_int;
    unsafe fn db_storage_pool_list(
        pools: *mut storage_pool_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_storage_pool_get(
        uuid: *const ::core::ffi::c_char,
        pool: *mut storage_pool_t,
    ) -> db_storage_pool_result_t;
    unsafe fn db_storage_pool_create(pool: *mut storage_pool_t) -> db_storage_pool_result_t;
    unsafe fn db_storage_pool_update(
        pool: *mut storage_pool_t,
        expected_revision: int64_t,
    ) -> db_storage_pool_result_t;
    unsafe fn db_storage_pool_delete(
        uuid: *const ::core::ffi::c_char,
        expected_revision: int64_t,
    ) -> db_storage_pool_result_t;
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
    unsafe fn httpd_authorize_action(
        req: *const http_request_t,
        res: *mut http_response_t,
        action: authorization_action_t,
        camera: *const fleet_camera_t,
        user: *mut user_t,
        evaluation: *mut authorization_evaluation_t,
    ) -> ::core::ffi::c_int;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    unsafe fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_pool_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub name: [::core::ffi::c_char; 128],
    pub strategy: [::core::ffi::c_char; 24],
    pub enabled: bool,
    pub allocation_cursor: ::core::ffi::c_int,
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
    pub member_count: ::core::ffi::c_int,
    pub members: [storage_pool_member_t; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct storage_pool_member_t {
    pub target_uuid: [::core::ffi::c_char; 37],
    pub position: ::core::ffi::c_int,
    pub weight: ::core::ffi::c_int,
}
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_storage_pool_result_t(pub ::core::ffi::c_int);
impl db_storage_pool_result_t {
    pub const DB_STORAGE_POOL_OK: Self = Self(0);
    pub const DB_STORAGE_POOL_NOT_FOUND: Self = Self(-1);
    pub const DB_STORAGE_POOL_CONFLICT: Self = Self(-2);
    pub const DB_STORAGE_POOL_INVALID: Self = Self(-3);
    pub const DB_STORAGE_POOL_STALE: Self = Self(-4);
    pub const DB_STORAGE_POOL_LIMIT: Self = Self(-5);
    pub const DB_STORAGE_POOL_IN_USE: Self = Self(-6);
    pub const DB_STORAGE_POOL_UNAVAILABLE: Self = Self(-7);
    pub const DB_STORAGE_POOL_ERROR: Self = Self(-8);
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
pub const LIGHTNVR_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const STORAGE_TARGET_MAX_COUNT: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const STORAGE_POOL_MAX_COUNT: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const STORAGE_POOL_MEMBER_MAX: ::core::ffi::c_int = STORAGE_TARGET_MAX_COUNT;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
unsafe extern "C" fn authorize_storage(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut user: *mut user_t,
) -> bool {
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
    return httpd_authorize_action(
        req,
        res,
        authorization_action_t::AUTHZ_STORAGE_CONFIGURE,
        ::core::ptr::null::<fleet_camera_t>(),
        user,
        &raw mut evaluation,
    ) != 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn audit_pool(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut uuid: *const ::core::ffi::c_char,
    mut operation: *const ::core::ffi::c_char,
    mut outcome: *const ::core::ffi::c_char,
) {
    audit_log_operation(
        req,
        user,
        b"storage.configure\0".as_ptr() as *const ::core::ffi::c_char,
        b"storage_pool\0".as_ptr() as *const ::core::ffi::c_char,
        uuid,
        operation,
        outcome,
        ::core::ptr::null::<cJSON>(),
    );
}
unsafe extern "C" fn pool_to_json(mut pool: *const storage_pool_t) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_CreateObject();
    let mut members: *mut cJSON = cJSON_CreateArray();
    if object.is_null() || members.is_null() {
        cJSON_Delete(object);
        cJSON_Delete(members);
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*pool).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*pool).name as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"strategy\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*pool).strategy as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        object,
        b"enabled\0".as_ptr() as *const ::core::ffi::c_char,
        (*pool).enabled as cJSON_bool,
    );
    cJSON_AddNumberToObject(
        object,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        (*pool).revision as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*pool).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*pool).updated_at as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        object,
        b"members\0".as_ptr() as *const ::core::ffi::c_char,
        members,
    );
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < (*pool).member_count {
        let mut member: *mut cJSON = cJSON_CreateObject();
        if member.is_null() {
            cJSON_Delete(object);
            return ::core::ptr::null_mut::<cJSON>();
        }
        cJSON_AddStringToObject(
            member,
            b"target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*(&raw const (*pool).members as *const storage_pool_member_t)
                .offset(index as isize))
            .target_uuid as *const ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            member,
            b"position\0".as_ptr() as *const ::core::ffi::c_char,
            index as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            member,
            b"weight\0".as_ptr() as *const ::core::ffi::c_char,
            (*pool).members[index as usize].weight as ::core::ffi::c_double,
        );
        cJSON_AddItemToArray(members, member);
        index += 1;
    }
    return object;
}
unsafe extern "C" fn send_json(
    mut res: *mut http_response_t,
    mut status: ::core::ffi::c_int,
    mut object: *mut cJSON,
) -> bool {
    let mut body: *mut ::core::ffi::c_char = if !object.is_null() {
        cJSON_PrintUnformatted(object)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    cJSON_Delete(object);
    if body.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize storage pool response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    http_response_set_json(res, status, body);
    free(body as *mut ::core::ffi::c_void);
    return r#true != 0;
}
unsafe extern "C" fn set_db_error(
    mut res: *mut http_response_t,
    mut result: db_storage_pool_result_t,
    mut validation_error: *const ::core::ffi::c_char,
) {
    match result {
        db_storage_pool_result_t::DB_STORAGE_POOL_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Storage pool not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_storage_pool_result_t::DB_STORAGE_POOL_CONFLICT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"A storage pool already uses that name\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_storage_pool_result_t::DB_STORAGE_POOL_STALE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Storage pool was changed by another administrator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_storage_pool_result_t::DB_STORAGE_POOL_IN_USE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Storage pool is used by a policy\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_storage_pool_result_t::DB_STORAGE_POOL_LIMIT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Storage pool limit reached\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_storage_pool_result_t::DB_STORAGE_POOL_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                if !validation_error.is_null()
                    && *validation_error.offset(0isize) as ::core::ffi::c_int != 0
                {
                    validation_error
                } else {
                    b"Invalid storage pool\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Storage pool operation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn extract_uuid(
    mut req: *const http_request_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut res: *mut http_response_t,
) -> bool {
    let mut value: [::core::ffi::c_char; 512] = [0; 512];
    if http_request_extract_path_param(
        req,
        b"/api/storage-pools/\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>(),
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid storage pool path\0".as_ptr() as *const ::core::ffi::c_char,
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
    if !lightnvr_uuid_is_valid(&raw mut value as *mut ::core::ffi::c_char) {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid storage pool UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    safe_strcpy(
        uuid,
        &raw mut value as *mut ::core::ffi::c_char,
        LIGHTNVR_UUID_STRING_SIZE as size_t,
        0 as size_t,
    );
    return r#true != 0;
}
unsafe extern "C" fn apply_body(
    mut body: *const cJSON,
    mut pool: *mut storage_pool_t,
    mut create: bool,
    mut revision: *mut int64_t,
    mut error: *mut ::core::ffi::c_char,
    mut error_size: size_t,
    mut res: *mut http_response_t,
) -> bool {
    if cJSON_IsObject(body) == 0 {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Request body must be a JSON object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut name: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"name\0".as_ptr() as *const ::core::ffi::c_char);
    let mut strategy: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"strategy\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut enabled: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"enabled\0".as_ptr() as *const ::core::ffi::c_char);
    let mut members: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"members\0".as_ptr() as *const ::core::ffi::c_char);
    if (create as ::core::ffi::c_int != 0 || !name.is_null())
        && (cJSON_IsString(name) == 0
            || (*name).valuestring.is_null()
            || strnlen(
                (*name).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            ) >= ::core::mem::size_of::<[::core::ffi::c_char; 128]>())
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"name must be a valid string\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !name.is_null() {
        safe_strcpy(
            &raw mut (*pool).name as *mut ::core::ffi::c_char,
            (*name).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            0 as size_t,
        );
    }
    if !strategy.is_null() {
        if cJSON_IsString(strategy) == 0
            || (*strategy).valuestring.is_null()
            || strnlen(
                (*strategy).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
            ) >= ::core::mem::size_of::<[::core::ffi::c_char; 24]>()
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"strategy must be a valid string\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*pool).strategy as *mut ::core::ffi::c_char,
            (*strategy).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
            0 as size_t,
        );
    }
    if !enabled.is_null() {
        if cJSON_IsBool(enabled) == 0 {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"enabled must be boolean\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*pool).enabled = cJSON_IsTrue(enabled) != 0;
    }
    if create as ::core::ffi::c_int != 0 || !members.is_null() {
        if cJSON_IsArray(members) == 0
            || cJSON_GetArraySize(members) < 1 as ::core::ffi::c_int
            || cJSON_GetArraySize(members) > STORAGE_POOL_MEMBER_MAX
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"members must be a non-empty array\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        (*pool).member_count = 0 as ::core::ffi::c_int;
        let mut item: *const cJSON = ::core::ptr::null::<cJSON>();
        item = if !members.is_null() {
            (*members).child
        } else {
            ::core::ptr::null_mut::<cJSON>()
        };
        while !item.is_null() {
            let mut target_uuid: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                item,
                b"target_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut weight: *const cJSON = cJSON_GetObjectItemCaseSensitive(
                item,
                b"weight\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if cJSON_IsObject(item) == 0
                || cJSON_IsString(target_uuid) == 0
                || (*target_uuid).valuestring.is_null()
                || !lightnvr_uuid_is_valid((*target_uuid).valuestring)
                || !weight.is_null()
                    && (cJSON_IsNumber(weight) == 0
                        || floor((*weight).valuedouble) != (*weight).valuedouble
                        || (*weight).valueint < 1 as ::core::ffi::c_int
                        || (*weight).valueint > 1000 as ::core::ffi::c_int)
            {
                http_response_set_json_error(
                    res,
                    400 as ::core::ffi::c_int,
                    b"Invalid pool member\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return r#false != 0;
            }
            let c2rust_fresh0 = (*pool).member_count;
            (*pool).member_count += 1;
            let mut member: *mut storage_pool_member_t = (&raw mut (*pool).members
                as *mut storage_pool_member_t)
                .offset(c2rust_fresh0 as isize);
            safe_strcpy(
                &raw mut (*member).target_uuid as *mut ::core::ffi::c_char,
                (*target_uuid).valuestring,
                ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
                0 as size_t,
            );
            (*member).position = (*pool).member_count - 1 as ::core::ffi::c_int;
            (*member).weight = if !weight.is_null() {
                (*weight).valueint
            } else {
                1 as ::core::ffi::c_int
            };
            item = (*item).next;
        }
    }
    if !create {
        let mut revision_item: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            body,
            b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if cJSON_IsNumber(revision_item) == 0
            || (*revision_item).valuedouble < 1 as ::core::ffi::c_int as ::core::ffi::c_double
            || floor((*revision_item).valuedouble) != (*revision_item).valuedouble
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"revision must be a positive integer\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        *revision = (*revision_item).valuedouble as int64_t;
    }
    let mut validated: db_storage_pool_result_t = db_storage_pool_validate(pool, error, error_size);
    if validated.0 != db_storage_pool_result_t::DB_STORAGE_POOL_OK.0 {
        set_db_error(res, validated, error);
        return r#false != 0;
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_storage_pools(
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
    if !authorize_storage(req, res, &raw mut user) {
        return;
    }
    let mut total: ::core::ffi::c_int = db_storage_pool_count();
    let mut pools: *mut storage_pool_t =
        (if total > 0 as ::core::ffi::c_int && total <= STORAGE_POOL_MAX_COUNT {
            calloc(total as size_t, ::core::mem::size_of::<storage_pool_t>())
        } else {
            NULL
        }) as *mut storage_pool_t;
    let mut count: ::core::ffi::c_int = if total < 0 as ::core::ffi::c_int
        || total > STORAGE_POOL_MAX_COUNT
        || total > 0 as ::core::ffi::c_int && pools.is_null()
    {
        -1 as ::core::ffi::c_int
    } else if total > 0 as ::core::ffi::c_int {
        db_storage_pool_list(pools, total)
    } else {
        0 as ::core::ffi::c_int
    };
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if count < 0 as ::core::ffi::c_int || root.is_null() || items.is_null() {
        free(pools as *mut ::core::ffi::c_void);
        cJSON_Delete(root);
        cJSON_Delete(items);
        set_db_error(
            res,
            db_storage_pool_result_t::DB_STORAGE_POOL_ERROR,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return;
    }
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        count as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"pools\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut item: *mut cJSON = pool_to_json(pools.offset(index as isize));
        if item.is_null() {
            free(pools as *mut ::core::ffi::c_void);
            cJSON_Delete(root);
            set_db_error(
                res,
                db_storage_pool_result_t::DB_STORAGE_POOL_ERROR,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            return;
        }
        cJSON_AddItemToArray(items, item);
        index += 1;
    }
    free(pools as *mut ::core::ffi::c_void);
    send_json(res, 200 as ::core::ffi::c_int, root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_storage_pool(
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
    if !authorize_storage(req, res, &raw mut user) {
        return;
    }
    let mut pool: storage_pool_t = storage_pool_t {
        uuid: [0; 37],
        name: [0; 128],
        strategy: [0; 24],
        enabled: false,
        allocation_cursor: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
        member_count: 0,
        members: [storage_pool_member_t {
            target_uuid: [0; 37],
            position: 0,
            weight: 0,
        }; 128],
    };
    memset(
        &raw mut pool as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<storage_pool_t>(),
    );
    pool.enabled = r#true != 0;
    safe_strcpy(
        &raw mut pool.strategy as *mut ::core::ffi::c_char,
        b"most_free\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
        0 as size_t,
    );
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut revision: int64_t = 0 as int64_t;
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
    if !apply_body(
        body,
        &raw mut pool,
        r#true != 0,
        &raw mut revision,
        &raw mut error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        res,
    ) {
        cJSON_Delete(body);
        audit_pool(
            req,
            &raw mut user,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"pool_create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_Delete(body);
    let mut result: db_storage_pool_result_t = db_storage_pool_create(&raw mut pool);
    if result.0 != db_storage_pool_result_t::DB_STORAGE_POOL_OK.0 {
        set_db_error(res, result, &raw mut error as *mut ::core::ffi::c_char);
        audit_pool(
            req,
            &raw mut user,
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"pool_create\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_pool(
        req,
        &raw mut user,
        &raw mut pool.uuid as *mut ::core::ffi::c_char,
        b"pool_create\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
    );
    send_json(res, 201 as ::core::ffi::c_int, pool_to_json(&raw mut pool));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_storage_pool(
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
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !authorize_storage(req, res, &raw mut user)
        || !extract_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res)
    {
        return;
    }
    let mut pool: storage_pool_t = storage_pool_t {
        uuid: [0; 37],
        name: [0; 128],
        strategy: [0; 24],
        enabled: false,
        allocation_cursor: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
        member_count: 0,
        members: [storage_pool_member_t {
            target_uuid: [0; 37],
            position: 0,
            weight: 0,
        }; 128],
    };
    let mut result: db_storage_pool_result_t =
        db_storage_pool_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut pool);
    if result.0 != db_storage_pool_result_t::DB_STORAGE_POOL_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        return;
    }
    send_json(res, 200 as ::core::ffi::c_int, pool_to_json(&raw mut pool));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_storage_pool(
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
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !authorize_storage(req, res, &raw mut user)
        || !extract_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res)
    {
        return;
    }
    let mut pool: storage_pool_t = storage_pool_t {
        uuid: [0; 37],
        name: [0; 128],
        strategy: [0; 24],
        enabled: false,
        allocation_cursor: 0,
        revision: 0,
        created_at: 0,
        updated_at: 0,
        member_count: 0,
        members: [storage_pool_member_t {
            target_uuid: [0; 37],
            position: 0,
            weight: 0,
        }; 128],
    };
    let mut result: db_storage_pool_result_t =
        db_storage_pool_get(&raw mut uuid as *mut ::core::ffi::c_char, &raw mut pool);
    if result.0 != db_storage_pool_result_t::DB_STORAGE_POOL_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut revision: int64_t = 0 as int64_t;
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
    if !apply_body(
        body,
        &raw mut pool,
        r#false != 0,
        &raw mut revision,
        &raw mut error as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        res,
    ) {
        cJSON_Delete(body);
        audit_pool(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"pool_update\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_Delete(body);
    result = db_storage_pool_update(&raw mut pool, revision);
    if result.0 != db_storage_pool_result_t::DB_STORAGE_POOL_OK.0 {
        set_db_error(res, result, &raw mut error as *mut ::core::ffi::c_char);
        audit_pool(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"pool_update\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_pool(
        req,
        &raw mut user,
        &raw mut uuid as *mut ::core::ffi::c_char,
        b"pool_update\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
    );
    send_json(res, 200 as ::core::ffi::c_int, pool_to_json(&raw mut pool));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_storage_pool(
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
    let mut uuid: [::core::ffi::c_char; 37] = [0; 37];
    if !authorize_storage(req, res, &raw mut user)
        || !extract_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res)
    {
        return;
    }
    let mut value: [::core::ffi::c_char; 64] = [0; 64];
    if http_request_get_query_param(
        req,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut value as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
    ) < 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"revision query parameter is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut revision: ::core::ffi::c_longlong = strtoll(
        &raw mut value as *mut ::core::ffi::c_char,
        &raw mut end,
        10 as ::core::ffi::c_int,
    );
    if *__errno_location() == ERANGE
        || end.is_null()
        || *end as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
        || revision < 1 as ::core::ffi::c_longlong
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"revision must be a positive integer\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut result: db_storage_pool_result_t = db_storage_pool_delete(
        &raw mut uuid as *mut ::core::ffi::c_char,
        revision as int64_t,
    );
    if result.0 != db_storage_pool_result_t::DB_STORAGE_POOL_OK.0 {
        set_db_error(res, result, ::core::ptr::null::<::core::ffi::c_char>());
        audit_pool(
            req,
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            b"pool_delete\0".as_ptr() as *const ::core::ffi::c_char,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    audit_pool(
        req,
        &raw mut user,
        &raw mut uuid as *mut ::core::ffi::c_char,
        b"pool_delete\0".as_ptr() as *const ::core::ffi::c_char,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
    );
    http_response_set_json(
        res,
        200 as ::core::ffi::c_int,
        b"{\"success\":true}\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
