unsafe extern "C" {
    unsafe fn http_request_get_body_str(
        req: *const http_request_t,
        buf: *mut ::core::ffi::c_char,
        buf_len: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn http_request_extract_path_param(
        req: *const http_request_t,
        prefix: *const ::core::ffi::c_char,
        param_buf: *mut ::core::ffi::c_char,
        buf_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn http_response_add_header(
        res: *mut http_response_t,
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
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
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
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
    unsafe fn cJSON_AddNullToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
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
    unsafe fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn db_lpr_reads_search(
        query: *const lpr_read_query_t,
        reads: *mut lpr_read_t,
        capacity: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_lpr_read_delete(uuid: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn db_lpr_read_get_camera_uuid(
        uuid: *const ::core::ffi::c_char,
        camera_uuid: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn lpr_canonicalize_plate(
        plate: *const ::core::ffi::c_char,
        canonical: *mut ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lpr_crypto_fingerprint(
        data: *const ::core::ffi::c_void,
        data_len: size_t,
        output: *mut uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type int64_t = i64;
pub type uint8_t = u8;
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
pub struct lpr_read_query_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub start_at_ms: int64_t,
    pub end_at_ms: int64_t,
    pub match_mode: lpr_match_mode_t,
    pub plate_query: [::core::ffi::c_char; 64],
    pub limit: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct lpr_match_mode_t(pub ::core::ffi::c_uint);
impl lpr_match_mode_t {
    pub const LPR_MATCH_NONE: Self = Self(0);
    pub const LPR_MATCH_EXACT: Self = Self(1);
    pub const LPR_MATCH_PARTIAL: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lpr_read_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub read: lpr_read_input_t,
    pub received_at_ms: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lpr_read_input_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub stream_name: [::core::ffi::c_char; 256],
    pub observed_at_ms: int64_t,
    pub source: [::core::ffi::c_char; 32],
    pub vendor_topic: [::core::ffi::c_char; 256],
    pub plate: [::core::ffi::c_char; 64],
    pub has_confidence: bool,
    pub confidence: ::core::ffi::c_float,
    pub country: [::core::ffi::c_char; 64],
    pub region: [::core::ffi::c_char; 64],
    pub plate_type: [::core::ffi::c_char; 64],
    pub direction: [::core::ffi::c_char; 64],
    pub lane: [::core::ffi::c_char; 64],
    pub vehicle_type: [::core::ffi::c_char; 64],
    pub vehicle_color: [::core::ffi::c_char; 64],
    pub object_id: [::core::ffi::c_char; 96],
    pub correlation_id: [::core::ffi::c_char; 96],
    pub has_bounding_box: bool,
    pub bbox_left: ::core::ffi::c_float,
    pub bbox_top: ::core::ffi::c_float,
    pub bbox_right: ::core::ffi::c_float,
    pub bbox_bottom: ::core::ffi::c_float,
    pub recording_id: uint64_t,
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
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const LPR_CRYPTO_HMAC_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LPR_SEARCH_MAX_RANGE_MS: ::core::ffi::c_long = 366 as ::core::ffi::c_long
    * 24 as ::core::ffi::c_long
    * 60 as ::core::ffi::c_long
    * 60 as ::core::ffi::c_long
    * 1000 as ::core::ffi::c_long;
pub const LPR_SEARCH_RESULT_MAX: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const LPR_EXPORT_RESULT_MAX: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
unsafe extern "C" fn secure_zero(mut data: *mut ::core::ffi::c_void, mut size: size_t) {
    let mut bytes: *mut ::core::ffi::c_uchar = data as *mut ::core::ffi::c_uchar;
    while !bytes.is_null() && {
        let c2rust_fresh0 = size;
        size = size.wrapping_sub(1);
        c2rust_fresh0 > 0 as size_t
    } {
        let c2rust_fresh1 = bytes;
        bytes = bytes.offset(1);
        ::core::ptr::write_volatile(c2rust_fresh1, 0 as ::core::ffi::c_uchar);
    }
}
unsafe extern "C" fn delete_query_body(mut body: *mut cJSON) {
    let mut plate: *mut cJSON = if !body.is_null() {
        cJSON_GetObjectItemCaseSensitive(body, b"plate\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if cJSON_IsString(plate) != 0 && !(*plate).valuestring.is_null() {
        secure_zero(
            (*plate).valuestring as *mut ::core::ffi::c_void,
            strlen((*plate).valuestring),
        );
    }
    cJSON_Delete(body);
}
unsafe extern "C" fn wipe_response_plates(mut root: *mut cJSON) {
    let mut items: *mut cJSON = if !root.is_null() {
        cJSON_GetObjectItemCaseSensitive(root, b"reads\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if cJSON_IsArray(items) == 0 {
        return;
    }
    let mut item: *mut cJSON = (*items).child;
    while !item.is_null() {
        let mut plate: *mut cJSON = cJSON_GetObjectItemCaseSensitive(
            item,
            b"plate\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if cJSON_IsString(plate) != 0 && !(*plate).valuestring.is_null() {
            secure_zero(
                (*plate).valuestring as *mut ::core::ffi::c_void,
                strlen((*plate).valuestring),
            );
        }
        item = (*item).next as *mut cJSON;
    }
}
unsafe extern "C" fn json_int64(
    mut body: *const cJSON,
    mut name: *const ::core::ffi::c_char,
    mut value: *mut int64_t,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, name);
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
unsafe extern "C" fn digest_hex(mut digest: *const uint8_t, mut output: *mut ::core::ffi::c_char) {
    static mut hex: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"0123456789abcdef\0")
    };
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < LPR_CRYPTO_HMAC_SIZE {
        *output.offset((i * 2 as ::core::ffi::c_int) as isize) =
            hex[(*digest.offset(i as isize) as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                as usize];
        *output.offset((i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) = hex
            [(*digest.offset(i as isize) as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                as usize];
        i += 1;
    }
    *output.offset(64isize) = '\0' as ::core::ffi::c_char;
}
unsafe extern "C" fn read_json(mut read: *const lpr_read_t) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_CreateObject();
    if object.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"id\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*read).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"type\0".as_ptr() as *const ::core::ffi::c_char,
        b"license_plate\0".as_ptr() as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*read).read.camera_uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"stream_name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*read).read.stream_name as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        object,
        b"observed_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*read).read.observed_at_ms as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        object,
        b"received_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*read).received_at_ms as ::core::ffi::c_double,
    );
    cJSON_AddStringToObject(
        object,
        b"source\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*read).read.source as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        object,
        b"plate\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*read).read.plate as *const ::core::ffi::c_char,
    );
    if (*read).read.has_confidence {
        cJSON_AddNumberToObject(
            object,
            b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
            (*read).read.confidence as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(
            object,
            b"confidence\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut attributes: *mut cJSON = cJSON_AddObjectToObject(
        object,
        b"attributes\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if (*read).read.country[0usize] != 0 {
        cJSON_AddStringToObject(
            attributes,
            b"country\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*read).read.country as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            attributes,
            b"country\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*read).read.region[0usize] != 0 {
        cJSON_AddStringToObject(
            attributes,
            b"region\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*read).read.region as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            attributes,
            b"region\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*read).read.plate_type[0usize] != 0 {
        cJSON_AddStringToObject(
            attributes,
            b"plate_type\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*read).read.plate_type as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            attributes,
            b"plate_type\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*read).read.direction[0usize] != 0 {
        cJSON_AddStringToObject(
            attributes,
            b"direction\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*read).read.direction as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            attributes,
            b"direction\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*read).read.lane[0usize] != 0 {
        cJSON_AddStringToObject(
            attributes,
            b"lane\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*read).read.lane as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(attributes, b"lane\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if (*read).read.vehicle_type[0usize] != 0 {
        cJSON_AddStringToObject(
            attributes,
            b"vehicle_type\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*read).read.vehicle_type as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            attributes,
            b"vehicle_type\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*read).read.vehicle_color[0usize] != 0 {
        cJSON_AddStringToObject(
            attributes,
            b"vehicle_color\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*read).read.vehicle_color as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            attributes,
            b"vehicle_color\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*read).read.object_id[0usize] != 0 {
        cJSON_AddStringToObject(
            attributes,
            b"object_id\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*read).read.object_id as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            attributes,
            b"object_id\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*read).read.correlation_id[0usize] != 0 {
        cJSON_AddStringToObject(
            attributes,
            b"correlation_id\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*read).read.correlation_id as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            attributes,
            b"correlation_id\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddStringToObject(
        attributes,
        b"vendor_topic\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*read).read.vendor_topic as *const ::core::ffi::c_char,
    );
    if (*read).read.recording_id != 0 {
        cJSON_AddNumberToObject(
            object,
            b"recording_id\0".as_ptr() as *const ::core::ffi::c_char,
            (*read).read.recording_id as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(
            object,
            b"recording_id\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if (*read).read.has_bounding_box {
        let mut bbox: *mut cJSON = cJSON_AddObjectToObject(
            attributes,
            b"bounding_box\0".as_ptr() as *const ::core::ffi::c_char,
        );
        cJSON_AddNumberToObject(
            bbox,
            b"left\0".as_ptr() as *const ::core::ffi::c_char,
            (*read).read.bbox_left as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            bbox,
            b"top\0".as_ptr() as *const ::core::ffi::c_char,
            (*read).read.bbox_top as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            bbox,
            b"right\0".as_ptr() as *const ::core::ffi::c_char,
            (*read).read.bbox_right as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            bbox,
            b"bottom\0".as_ptr() as *const ::core::ffi::c_char,
            (*read).read.bbox_bottom as ::core::ffi::c_double,
        );
    } else {
        cJSON_AddNullToObject(
            attributes,
            b"bounding_box\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return object;
}
unsafe extern "C" fn audit_query(
    mut req: *const http_request_t,
    mut user: *const user_t,
    mut camera: *const fleet_camera_t,
    mut query: *const lpr_read_query_t,
    mut fingerprint: *const ::core::ffi::c_char,
    mut result_count: ::core::ffi::c_int,
    mut outcome: *const ::core::ffi::c_char,
    mut exporting: bool,
) {
    let mut context: *mut cJSON = cJSON_CreateObject();
    if !context.is_null() {
        let mut mode: *const ::core::ffi::c_char =
            if (*query).match_mode.0 == lpr_match_mode_t::LPR_MATCH_EXACT.0 {
                b"exact\0".as_ptr() as *const ::core::ffi::c_char
            } else if (*query).match_mode.0 == lpr_match_mode_t::LPR_MATCH_PARTIAL.0 {
                b"partial\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"none\0".as_ptr() as *const ::core::ffi::c_char
            };
        cJSON_AddStringToObject(
            context,
            b"match_mode\0".as_ptr() as *const ::core::ffi::c_char,
            mode,
        );
        cJSON_AddNumberToObject(
            context,
            b"start_at\0".as_ptr() as *const ::core::ffi::c_char,
            (*query).start_at_ms as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            context,
            b"end_at\0".as_ptr() as *const ::core::ffi::c_char,
            (*query).end_at_ms as ::core::ffi::c_double,
        );
        cJSON_AddNumberToObject(
            context,
            b"result_count\0".as_ptr() as *const ::core::ffi::c_char,
            result_count as ::core::ffi::c_double,
        );
        if !fingerprint.is_null() && *fingerprint.offset(0isize) as ::core::ffi::c_int != 0 {
            cJSON_AddStringToObject(
                context,
                b"query_fingerprint\0".as_ptr() as *const ::core::ffi::c_char,
                fingerprint,
            );
        }
    }
    audit_log_operation(
        req,
        user,
        if exporting as ::core::ffi::c_int != 0 {
            b"lpr.export\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"lpr.search\0".as_ptr() as *const ::core::ffi::c_char
        },
        b"camera\0".as_ptr() as *const ::core::ffi::c_char,
        if !camera.is_null() {
            &raw const (*camera).camera_uuid as *const ::core::ffi::c_char
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        },
        if exporting as ::core::ffi::c_int != 0 {
            b"export\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"search\0".as_ptr() as *const ::core::ffi::c_char
        },
        outcome,
        context,
    );
    cJSON_Delete(context);
}
unsafe extern "C" fn handle_lpr_query(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut exporting: bool,
) {
    let mut body_text: [::core::ffi::c_char; 4096] = [0; 4096];
    if req.is_null()
        || res.is_null()
        || http_request_get_body_str(
            req,
            &raw mut body_text as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid JSON body\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = cJSON_Parse(&raw mut body_text as *mut ::core::ffi::c_char);
    secure_zero(
        &raw mut body_text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
    );
    let mut camera_item: *const cJSON = if !body.is_null() {
        cJSON_GetObjectItemCaseSensitive(
            body,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if cJSON_IsObject(body) == 0
        || cJSON_IsString(camera_item) == 0
        || !lightnvr_uuid_is_valid((*camera_item).valuestring)
    {
        delete_query_body(body);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"camera_uuid is required\0".as_ptr() as *const ::core::ffi::c_char,
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
    if exporting as ::core::ffi::c_int != 0
        && httpd_authorize_camera_identity_action_with_context(
            req,
            res,
            authorization_action_t::AUTHZ_LPR_EXPORT,
            (*camera_item).valuestring,
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw mut user,
            &raw mut camera,
            &raw mut evaluation,
        ) == 0
    {
        delete_query_body(body);
        return;
    }
    if httpd_authorize_camera_identity_action_with_context(
        req,
        res,
        authorization_action_t::AUTHZ_LPR_SEARCH,
        (*camera_item).valuestring,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut user,
        &raw mut camera,
        &raw mut evaluation,
    ) == 0
    {
        delete_query_body(body);
        return;
    }
    let mut read_user: user_t = user_t {
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
    let mut read_camera: fleet_camera_t = fleet_camera_t {
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
    let mut read_evaluation: authorization_evaluation_t = authorization_evaluation_t {
        decision: authorization_decision_t::AUTHZ_DECISION_DENY,
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
        authorization_action_t::AUTHZ_LPR_READ,
        (*camera_item).valuestring,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut read_user,
        &raw mut read_camera,
        &raw mut read_evaluation,
    ) == 0
    {
        delete_query_body(body);
        return;
    }
    let mut query: lpr_read_query_t = lpr_read_query_t {
        camera_uuid: [0; 37],
        start_at_ms: 0,
        end_at_ms: 0,
        match_mode: lpr_match_mode_t::LPR_MATCH_NONE,
        plate_query: [0; 64],
        limit: 0,
    };
    memset(
        &raw mut query as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<lpr_read_query_t>(),
    );
    snprintf(
        &raw mut query.camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*camera_item).valuestring,
    );
    let mut mode: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"match\0".as_ptr() as *const ::core::ffi::c_char);
    let mut plate: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"plate\0".as_ptr() as *const ::core::ffi::c_char);
    let mut limit: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"limit\0".as_ptr() as *const ::core::ffi::c_char);
    let mut maximum_results: ::core::ffi::c_int = if exporting as ::core::ffi::c_int != 0 {
        LPR_EXPORT_RESULT_MAX
    } else {
        LPR_SEARCH_RESULT_MAX
    };
    query.limit = if !limit.is_null() && cJSON_IsNumber(limit) != 0 {
        (*limit).valueint
    } else if exporting as ::core::ffi::c_int != 0 {
        1000 as ::core::ffi::c_int
    } else {
        50 as ::core::ffi::c_int
    };
    let mut valid: bool = json_int64(
        body,
        b"start_at\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut query.start_at_ms,
    ) as ::core::ffi::c_int
        != 0
        && json_int64(
            body,
            b"end_at\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut query.end_at_ms,
        ) as ::core::ffi::c_int
            != 0
        && query.end_at_ms >= query.start_at_ms
        && query.end_at_ms - query.start_at_ms <= LPR_SEARCH_MAX_RANGE_MS as int64_t
        && query.limit >= 1 as ::core::ffi::c_int
        && query.limit <= maximum_results;
    if mode.is_null() {
        query.match_mode = lpr_match_mode_t::LPR_MATCH_NONE;
    } else if cJSON_IsString(mode) != 0
        && strcmp(
            (*mode).valuestring,
            b"exact\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        query.match_mode = lpr_match_mode_t::LPR_MATCH_EXACT;
    } else if cJSON_IsString(mode) != 0
        && strcmp(
            (*mode).valuestring,
            b"partial\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        query.match_mode = lpr_match_mode_t::LPR_MATCH_PARTIAL;
    } else {
        valid = r#false != 0;
    }
    if query.match_mode.0 != lpr_match_mode_t::LPR_MATCH_NONE.0 {
        if cJSON_IsString(plate) == 0
            || (*plate).valuestring.is_null()
            || strlen((*plate).valuestring) >= ::core::mem::size_of::<[::core::ffi::c_char; 64]>()
        {
            valid = r#false != 0;
        } else {
            snprintf(
                &raw mut query.plate_query as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                (*plate).valuestring,
            );
        }
    }
    let mut query_fingerprint: [::core::ffi::c_char; 65] = [
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
    ];
    if valid as ::core::ffi::c_int != 0 && query.match_mode.0 != lpr_match_mode_t::LPR_MATCH_NONE.0
    {
        let mut canonical: [::core::ffi::c_char; 64] = [0; 64];
        let mut digest: [uint8_t; 32] = [0; 32];
        let mut canonical_length: ::core::ffi::c_int = lpr_canonicalize_plate(
            &raw mut query.plate_query as *mut ::core::ffi::c_char,
            &raw mut canonical as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        );
        if canonical_length < 0 as ::core::ffi::c_int
            || query.match_mode.0 == lpr_match_mode_t::LPR_MATCH_PARTIAL.0
                && canonical_length < 3 as ::core::ffi::c_int
            || lpr_crypto_fingerprint(
                &raw mut canonical as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                strlen(&raw mut canonical as *mut ::core::ffi::c_char),
                &raw mut digest as *mut uint8_t,
            ) != 0 as ::core::ffi::c_int
        {
            valid = r#false != 0;
        } else {
            digest_hex(
                &raw mut digest as *mut uint8_t,
                &raw mut query_fingerprint as *mut ::core::ffi::c_char,
            );
        }
        memset(
            &raw mut canonical as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>(),
        );
    }
    delete_query_body(body);
    if !valid {
        audit_query(
            req,
            &raw mut user,
            &raw mut camera,
            &raw mut query,
            &raw mut query_fingerprint as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            exporting,
        );
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid LPR search\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut reads: *mut lpr_read_t =
        calloc(query.limit as size_t, ::core::mem::size_of::<lpr_read_t>()) as *mut lpr_read_t;
    if reads.is_null() {
        audit_query(
            req,
            &raw mut user,
            &raw mut camera,
            &raw mut query,
            &raw mut query_fingerprint as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            exporting,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int =
        db_lpr_reads_search(&raw mut query, reads, query.limit as size_t);
    if count < 0 as ::core::ffi::c_int {
        secure_zero(
            reads as *mut ::core::ffi::c_void,
            (query.limit as size_t).wrapping_mul(::core::mem::size_of::<lpr_read_t>()),
        );
        free(reads as *mut ::core::ffi::c_void);
        audit_query(
            req,
            &raw mut user,
            &raw mut camera,
            &raw mut query,
            &raw mut query_fingerprint as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            exporting,
        );
        http_response_set_json_error(
            res,
            503 as ::core::ffi::c_int,
            b"Protected LPR storage unavailable\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = cJSON_CreateArray();
    if root.is_null() || items.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(items);
        secure_zero(
            reads as *mut ::core::ffi::c_void,
            (query.limit as size_t).wrapping_mul(::core::mem::size_of::<lpr_read_t>()),
        );
        free(reads as *mut ::core::ffi::c_void);
        audit_query(
            req,
            &raw mut user,
            &raw mut camera,
            &raw mut query,
            &raw mut query_fingerprint as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            exporting,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddItemToObject(
        root,
        b"reads\0".as_ptr() as *const ::core::ffi::c_char,
        items,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut item: *mut cJSON = read_json(reads.offset(i as isize));
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
    let mut encoded: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    wipe_response_plates(root);
    cJSON_Delete(root);
    secure_zero(
        reads as *mut ::core::ffi::c_void,
        (query.limit as size_t).wrapping_mul(::core::mem::size_of::<lpr_read_t>()),
    );
    free(reads as *mut ::core::ffi::c_void);
    if encoded.is_null() {
        audit_query(
            req,
            &raw mut user,
            &raw mut camera,
            &raw mut query,
            &raw mut query_fingerprint as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            b"failure\0".as_ptr() as *const ::core::ffi::c_char,
            exporting,
        );
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to serialize response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, 200 as ::core::ffi::c_int, encoded);
    if exporting {
        http_response_add_header(
            res,
            b"Content-Disposition\0".as_ptr() as *const ::core::ffi::c_char,
            b"attachment; filename=\"lightnvr-lpr-export.json\"\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    secure_zero(encoded as *mut ::core::ffi::c_void, strlen(encoded));
    free(encoded as *mut ::core::ffi::c_void);
    audit_query(
        req,
        &raw mut user,
        &raw mut camera,
        &raw mut query,
        &raw mut query_fingerprint as *mut ::core::ffi::c_char,
        count,
        b"success\0".as_ptr() as *const ::core::ffi::c_char,
        exporting,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_lpr_search(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    handle_lpr_query(req, res, r#false != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_lpr_export(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    handle_lpr_query(req, res, r#true != 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_lpr_read(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
) {
    let mut read_uuid: [::core::ffi::c_char; 37] = [0; 37];
    if req.is_null()
        || res.is_null()
        || http_request_extract_path_param(
            req,
            b"/api/lpr/reads/\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut read_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        ) != 0 as ::core::ffi::c_int
        || !lightnvr_uuid_is_valid(&raw mut read_uuid as *mut ::core::ffi::c_char)
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid LPR read ID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut camera_uuid: [::core::ffi::c_char; 37] = [0; 37];
    if db_lpr_read_get_camera_uuid(
        &raw mut read_uuid as *mut ::core::ffi::c_char,
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"LPR read not found\0".as_ptr() as *const ::core::ffi::c_char,
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
    if httpd_authorize_camera_identity_action_with_context(
        req,
        res,
        authorization_action_t::AUTHZ_LPR_DELETE,
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut user,
        &raw mut camera,
        &raw mut evaluation,
    ) == 0
    {
        return;
    }
    let mut result: ::core::ffi::c_int =
        db_lpr_read_delete(&raw mut read_uuid as *mut ::core::ffi::c_char);
    let mut context: *mut cJSON = cJSON_CreateObject();
    if !context.is_null() {
        cJSON_AddStringToObject(
            context,
            b"read_id\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut read_uuid as *mut ::core::ffi::c_char,
        );
    }
    audit_log_operation(
        req,
        &raw mut user,
        b"lpr.delete\0".as_ptr() as *const ::core::ffi::c_char,
        b"camera\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        b"delete\0".as_ptr() as *const ::core::ffi::c_char,
        if result == 0 as ::core::ffi::c_int {
            b"success\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"failure\0".as_ptr() as *const ::core::ffi::c_char
        },
        context,
    );
    cJSON_Delete(context);
    if result != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"LPR read not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(
        res,
        200 as ::core::ffi::c_int,
        b"{\"deleted\":true}\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
