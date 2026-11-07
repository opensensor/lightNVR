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
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsNull(item: *const cJSON) -> cJSON_bool;
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
    unsafe fn cJSON_Duplicate(item: *const cJSON, recurse: cJSON_bool) -> *mut cJSON;
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
    unsafe fn cJSON_AddArrayToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn authorization_filter_cameras(
        user: *const user_t,
        action: authorization_action_t,
        cameras: *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_live_layout_list_visible(
        owner_user_id: int64_t,
        layouts: *mut live_saved_layout_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_live_layout_get_visible(
        owner_user_id: int64_t,
        uuid: *const ::core::ffi::c_char,
        layout: *mut live_saved_layout_t,
    ) -> db_live_layout_result_t;
    unsafe fn db_live_layout_create(layout: *mut live_saved_layout_t) -> db_live_layout_result_t;
    unsafe fn db_live_layout_update(
        layout: *mut live_saved_layout_t,
        expected_revision: int64_t,
    ) -> db_live_layout_result_t;
    unsafe fn db_live_layout_delete(
        owner_user_id: int64_t,
        uuid: *const ::core::ffi::c_char,
        expected_revision: int64_t,
    ) -> db_live_layout_result_t;
    unsafe fn safe_strcpy(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        dst_size: size_t,
        src_size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
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
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct live_saved_layout_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub owner_user_id: int64_t,
    pub name: [::core::ffi::c_char; 128],
    pub is_shared: bool,
    pub location_uuid: [::core::ffi::c_char; 37],
    pub availability: [::core::ffi::c_char; 24],
    pub columns: ::core::ffi::c_int,
    pub rows: ::core::ffi::c_int,
    pub camera_slots_json: [::core::ffi::c_char; 8192],
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
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
    pub const AUTHZ_ACTION_COUNT: Self = Self(19);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_live_layout_result_t(pub ::core::ffi::c_int);
impl db_live_layout_result_t {
    pub const DB_LIVE_LAYOUT_OK: Self = Self(0);
    pub const DB_LIVE_LAYOUT_NOT_FOUND: Self = Self(-1);
    pub const DB_LIVE_LAYOUT_INVALID: Self = Self(-2);
    pub const DB_LIVE_LAYOUT_CONFLICT: Self = Self(-3);
    pub const DB_LIVE_LAYOUT_STALE: Self = Self(-4);
    pub const DB_LIVE_LAYOUT_LIMIT: Self = Self(-5);
    pub const DB_LIVE_LAYOUT_ERROR: Self = Self(-6);
}
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const LIVE_LAYOUT_MAX_VISIBLE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn read_only_identity(mut user: *const user_t) -> bool {
    return (*user).authenticated_via_scoped_token as ::core::ffi::c_int != 0
        || strcmp(
            &raw const (*user).authentication_method as *const ::core::ffi::c_char,
            b"demo\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn copy_string(
    mut body: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut destination: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut required: bool,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, key);
    if item.is_null() {
        if required {
            return r#false != 0;
        }
        *destination.offset(0isize) = '\0' as ::core::ffi::c_char;
        return r#true != 0;
    }
    if cJSON_IsString(item) == 0
        || (*item).valuestring.is_null()
        || required as ::core::ffi::c_int != 0
            && *(*item).valuestring.offset(0isize) as ::core::ffi::c_int
                == '\0' as ::core::ffi::c_int
        || strlen((*item).valuestring) >= size
    {
        return r#false != 0;
    }
    safe_strcpy(destination, (*item).valuestring, size, 0 as size_t);
    return r#true != 0;
}
unsafe extern "C" fn parse_integer(
    mut body: *const cJSON,
    mut key: *const ::core::ffi::c_char,
    mut minimum: ::core::ffi::c_int,
    mut maximum: ::core::ffi::c_int,
    mut value: *mut ::core::ffi::c_int,
) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(body, key);
    if cJSON_IsNumber(item) == 0
        || (*item).valuedouble.is_finite() as ::core::ffi::c_int == 0
        || floor((*item).valuedouble) != (*item).valuedouble
        || (*item).valueint < minimum
        || (*item).valueint > maximum
    {
        return r#false != 0;
    }
    *value = (*item).valueint;
    return r#true != 0;
}
unsafe extern "C" fn parse_revision(mut body: *const cJSON, mut revision: *mut int64_t) -> bool {
    let mut item: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if cJSON_IsNumber(item) == 0
        || (*item).valuedouble.is_finite() as ::core::ffi::c_int == 0
        || floor((*item).valuedouble) != (*item).valuedouble
        || (*item).valuedouble < 1 as ::core::ffi::c_int as ::core::ffi::c_double
        || (*item).valuedouble > INT64_MAX as ::core::ffi::c_double
    {
        return r#false != 0;
    }
    *revision = (*item).valuedouble as int64_t;
    return r#true != 0;
}
unsafe extern "C" fn camera_allowed(
    mut camera_uuid: *const ::core::ffi::c_char,
    mut cameras: *const fleet_camera_t,
    mut camera_count: ::core::ffi::c_int,
) -> bool {
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < camera_count {
        if strcmp(
            &raw const (*cameras.offset(index as isize)).camera_uuid as *const ::core::ffi::c_char,
            camera_uuid,
        ) == 0 as ::core::ffi::c_int
        {
            return r#true != 0;
        }
        index += 1;
    }
    return r#false != 0;
}
unsafe extern "C" fn load_authorized_cameras(
    mut user: *const user_t,
    mut cameras: *mut *mut fleet_camera_t,
    mut camera_count: *mut ::core::ffi::c_int,
) -> bool {
    *cameras = ::core::ptr::null_mut::<fleet_camera_t>();
    *camera_count = 0 as ::core::ffi::c_int;
    return db_fleet_camera_load(cameras, camera_count) == 0 as ::core::ffi::c_int
        && authorization_filter_cameras(
            user,
            authorization_action_t::AUTHZ_LIVE_VIEW,
            *cameras,
            camera_count,
        ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn authorized_slots(
    mut cameras: *const fleet_camera_t,
    mut camera_count: ::core::ffi::c_int,
    mut encoded: *const ::core::ffi::c_char,
    mut all_allowed: *mut bool,
) -> *mut cJSON {
    let mut source: *mut cJSON = cJSON_Parse(encoded);
    let mut result: *mut cJSON = cJSON_CreateArray();
    *all_allowed = r#true != 0;
    if cJSON_IsArray(source) == 0 || result.is_null() {
        cJSON_Delete(source);
        cJSON_Delete(result);
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut slot: *const cJSON = ::core::ptr::null::<cJSON>();
    slot = if !source.is_null() {
        (*source).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !slot.is_null() {
        let mut camera: *const cJSON = cJSON_GetObjectItemCaseSensitive(
            slot,
            b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if cJSON_IsString(camera) == 0
            || !camera_allowed((*camera).valuestring, cameras, camera_count)
        {
            *all_allowed = r#false != 0;
        } else {
            let mut copy: *mut cJSON = cJSON_Duplicate(slot, r#true);
            if copy.is_null() {
                cJSON_Delete(source);
                cJSON_Delete(result);
                return ::core::ptr::null_mut::<cJSON>();
            }
            cJSON_AddItemToArray(result, copy);
        }
        slot = (*slot).next;
    }
    cJSON_Delete(source);
    return result;
}
unsafe extern "C" fn parse_layout(
    mut body: *const cJSON,
    mut user: *const user_t,
    mut layout: *mut live_saved_layout_t,
    mut res: *mut http_response_t,
) -> bool {
    memset(
        layout as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<live_saved_layout_t>(),
    );
    (*layout).owner_user_id = (*user).id;
    if cJSON_IsObject(body) == 0
        || !copy_string(
            body,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*layout).name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            r#true != 0,
        )
        || !copy_string(
            body,
            b"availability\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*layout).availability as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 24]>(),
            r#true != 0,
        )
        || !parse_integer(
            body,
            b"columns\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            9 as ::core::ffi::c_int,
            &raw mut (*layout).columns,
        )
        || !parse_integer(
            body,
            b"rows\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            9 as ::core::ffi::c_int,
            &raw mut (*layout).rows,
        )
        || (*layout).columns * (*layout).rows > 36 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid live layout fields\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut location: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"location_uuid\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !location.is_null() && cJSON_IsNull(location) == 0 {
        if cJSON_IsString(location) == 0
            || (*location).valuestring.is_null()
            || !lightnvr_uuid_is_valid((*location).valuestring)
        {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid location_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return r#false != 0;
        }
        safe_strcpy(
            &raw mut (*layout).location_uuid as *mut ::core::ffi::c_char,
            (*location).valuestring,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            0 as size_t,
        );
    }
    let mut shared: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"is_shared\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !shared.is_null() && cJSON_IsBool(shared) == 0 {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"is_shared must be boolean\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    (*layout).is_shared = !shared.is_null() && cJSON_IsTrue(shared) != 0;
    let mut slots: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"camera_slots\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut encoded: *mut ::core::ffi::c_char = if cJSON_IsArray(slots) != 0 {
        cJSON_PrintUnformatted(slots)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    if encoded.is_null() || strlen(encoded) >= ::core::mem::size_of::<[::core::ffi::c_char; 8192]>()
    {
        free(encoded as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid camera_slots\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut all_allowed: bool = r#false != 0;
    let mut filtered: *mut cJSON =
        if load_authorized_cameras(user, &raw mut cameras, &raw mut camera_count)
            as ::core::ffi::c_int
            != 0
        {
            authorized_slots(cameras, camera_count, encoded, &raw mut all_allowed)
        } else {
            ::core::ptr::null_mut::<cJSON>()
        };
    free(cameras as *mut ::core::ffi::c_void);
    if filtered.is_null()
        || !all_allowed
        || cJSON_GetArraySize(filtered) != cJSON_GetArraySize(slots)
    {
        cJSON_Delete(filtered);
        free(encoded as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Layout contains an unavailable camera\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    cJSON_Delete(filtered);
    safe_strcpy(
        &raw mut (*layout).camera_slots_json as *mut ::core::ffi::c_char,
        encoded,
        ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        0 as size_t,
    );
    free(encoded as *mut ::core::ffi::c_void);
    return r#true != 0;
}
unsafe extern "C" fn layout_json_with_inventory(
    mut layout: *const live_saved_layout_t,
    mut user: *const user_t,
    mut cameras: *const fleet_camera_t,
    mut camera_count: ::core::ffi::c_int,
) -> *mut cJSON {
    let mut all_allowed: bool = r#false != 0;
    let mut slots: *mut cJSON = authorized_slots(
        cameras,
        camera_count,
        &raw const (*layout).camera_slots_json as *const ::core::ffi::c_char,
        &raw mut all_allowed,
    );
    let mut root: *mut cJSON = cJSON_CreateObject();
    if root.is_null() || slots.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(slots);
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        root,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*layout).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*layout).name as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        root,
        b"is_shared\0".as_ptr() as *const ::core::ffi::c_char,
        (*layout).is_shared as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        root,
        b"owned\0".as_ptr() as *const ::core::ffi::c_char,
        ((*layout).owner_user_id == (*user).id) as ::core::ffi::c_int,
    );
    if (*layout).location_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            root,
            b"location_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*layout).location_uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            root,
            b"location_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddStringToObject(
        root,
        b"availability\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*layout).availability as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        root,
        b"columns\0".as_ptr() as *const ::core::ffi::c_char,
        (*layout).columns as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"rows\0".as_ptr() as *const ::core::ffi::c_char,
        (*layout).rows as ::core::ffi::c_double,
    );
    cJSON_AddItemToObject(
        root,
        b"camera_slots\0".as_ptr() as *const ::core::ffi::c_char,
        slots,
    );
    cJSON_AddBoolToObject(
        root,
        b"partially_authorized\0".as_ptr() as *const ::core::ffi::c_char,
        !all_allowed as ::core::ffi::c_int,
    );
    cJSON_AddNumberToObject(
        root,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        (*layout).revision as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*layout).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*layout).updated_at as ::core::ffi::c_double,
    );
    return root;
}
unsafe extern "C" fn layout_json(
    mut layout: *const live_saved_layout_t,
    mut user: *const user_t,
) -> *mut cJSON {
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !load_authorized_cameras(user, &raw mut cameras, &raw mut camera_count) {
        free(cameras as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<cJSON>();
    }
    let mut root: *mut cJSON = layout_json_with_inventory(layout, user, cameras, camera_count);
    free(cameras as *mut ::core::ffi::c_void);
    return root;
}
unsafe extern "C" fn send_json(
    mut res: *mut http_response_t,
    mut status: ::core::ffi::c_int,
    mut json: *mut cJSON,
) {
    let mut encoded: *mut ::core::ffi::c_char = if !json.is_null() {
        cJSON_PrintUnformatted(json)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    if encoded.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(res, status, encoded);
    free(encoded as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn set_db_error(
    mut res: *mut http_response_t,
    mut result: db_live_layout_result_t,
) {
    match result {
        db_live_layout_result_t::DB_LIVE_LAYOUT_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Live layout not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_live_layout_result_t::DB_LIVE_LAYOUT_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid live layout\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_live_layout_result_t::DB_LIVE_LAYOUT_CONFLICT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"A live layout with this name already exists\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        db_live_layout_result_t::DB_LIVE_LAYOUT_STALE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Live layout changed; reload and retry\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_live_layout_result_t::DB_LIVE_LAYOUT_LIMIT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Live layout limit reached\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Live layout operation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn extract_uuid(
    mut req: *const http_request_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut res: *mut http_response_t,
) -> bool {
    if http_request_extract_path_param(
        req,
        b"/api/live/layouts/\0".as_ptr() as *const ::core::ffi::c_char,
        uuid,
        CAMERA_UUID_STRING_SIZE as size_t,
    ) != 0 as ::core::ffi::c_int
        || !lightnvr_uuid_is_valid(uuid)
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid live layout UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_live_layouts(
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
    let mut layouts: *mut live_saved_layout_t = calloc(
        LIVE_LAYOUT_MAX_VISIBLE as size_t,
        ::core::mem::size_of::<live_saved_layout_t>(),
    ) as *mut live_saved_layout_t;
    if layouts.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int =
        db_live_layout_list_visible(user.id, layouts, LIVE_LAYOUT_MAX_VISIBLE);
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = if !root.is_null() {
        cJSON_AddArrayToObject(root, b"layouts\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if count < 0 as ::core::ffi::c_int
        || root.is_null()
        || items.is_null()
        || !load_authorized_cameras(&raw mut user, &raw mut cameras, &raw mut camera_count)
    {
        free(layouts as *mut ::core::ffi::c_void);
        free(cameras as *mut ::core::ffi::c_void);
        cJSON_Delete(root);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to list live layouts\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut item: *mut cJSON = layout_json_with_inventory(
            layouts.offset(index as isize),
            &raw mut user,
            cameras,
            camera_count,
        );
        if item.is_null() {
            free(layouts as *mut ::core::ffi::c_void);
            free(cameras as *mut ::core::ffi::c_void);
            cJSON_Delete(root);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create layout response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddItemToArray(items, item);
        index += 1;
    }
    free(layouts as *mut ::core::ffi::c_void);
    free(cameras as *mut ::core::ffi::c_void);
    cJSON_AddNumberToObject(
        root,
        b"count\0".as_ptr() as *const ::core::ffi::c_char,
        count as ::core::ffi::c_double,
    );
    cJSON_AddBoolToObject(
        root,
        b"can_share\0".as_ptr() as *const ::core::ffi::c_char,
        (user.role.0 == user_role_t::USER_ROLE_ADMIN.0) as ::core::ffi::c_int,
    );
    cJSON_AddBoolToObject(
        root,
        b"can_modify\0".as_ptr() as *const ::core::ffi::c_char,
        !read_only_identity(&raw mut user) as ::core::ffi::c_int,
    );
    send_json(res, 200 as ::core::ffi::c_int, root);
    cJSON_Delete(root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_live_layout(
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
    if read_only_identity(&raw mut user) {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Live layouts require an interactive user\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut layout: live_saved_layout_t = live_saved_layout_t {
        uuid: [0; 37],
        owner_user_id: 0,
        name: [0; 128],
        is_shared: false,
        location_uuid: [0; 37],
        availability: [0; 24],
        columns: 0,
        rows: 0,
        camera_slots_json: [0; 8192],
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut valid: bool = parse_layout(body, &raw mut user, &raw mut layout, res);
    cJSON_Delete(body);
    if !valid {
        return;
    }
    if layout.is_shared as ::core::ffi::c_int != 0 && user.role.0 != user_role_t::USER_ROLE_ADMIN.0
    {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Only administrators can share layouts\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut result: db_live_layout_result_t = db_live_layout_create(&raw mut layout);
    if result.0 != db_live_layout_result_t::DB_LIVE_LAYOUT_OK.0 {
        set_db_error(res, result);
        return;
    }
    let mut root: *mut cJSON = layout_json(&raw mut layout, &raw mut user);
    send_json(res, 201 as ::core::ffi::c_int, root);
    cJSON_Delete(root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_live_layout(
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
    let mut uuid: [::core::ffi::c_char; 37] = [
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
    ];
    let mut existing: live_saved_layout_t = live_saved_layout_t {
        uuid: [0; 37],
        owner_user_id: 0,
        name: [0; 128],
        is_shared: false,
        location_uuid: [0; 37],
        availability: [0; 24],
        columns: 0,
        rows: 0,
        camera_slots_json: [0; 8192],
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    if read_only_identity(&raw mut user) {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Live layouts require an interactive user\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !extract_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res) {
        return;
    }
    let mut result: db_live_layout_result_t = db_live_layout_get_visible(
        user.id,
        &raw mut uuid as *mut ::core::ffi::c_char,
        &raw mut existing,
    );
    if result.0 != db_live_layout_result_t::DB_LIVE_LAYOUT_OK.0 {
        set_db_error(res, result);
        return;
    }
    if existing.owner_user_id != user.id {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Only the owner can edit this layout\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut layout: live_saved_layout_t = live_saved_layout_t {
        uuid: [0; 37],
        owner_user_id: 0,
        name: [0; 128],
        is_shared: false,
        location_uuid: [0; 37],
        availability: [0; 24],
        columns: 0,
        rows: 0,
        camera_slots_json: [0; 8192],
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut revision: int64_t = 0 as int64_t;
    let mut valid: bool =
        parse_layout(body, &raw mut user, &raw mut layout, res) as ::core::ffi::c_int != 0
            && parse_revision(body, &raw mut revision) as ::core::ffi::c_int != 0;
    cJSON_Delete(body);
    if !valid {
        if (*res).status_code == 0 as ::core::ffi::c_int {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"revision is required\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    if layout.is_shared as ::core::ffi::c_int != 0 && user.role.0 != user_role_t::USER_ROLE_ADMIN.0
    {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Only administrators can share layouts\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_strcpy(
        &raw mut layout.uuid as *mut ::core::ffi::c_char,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    result = db_live_layout_update(&raw mut layout, revision);
    if result.0 != db_live_layout_result_t::DB_LIVE_LAYOUT_OK.0 {
        set_db_error(res, result);
        return;
    }
    let mut root: *mut cJSON = layout_json(&raw mut layout, &raw mut user);
    send_json(res, 200 as ::core::ffi::c_int, root);
    cJSON_Delete(root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_live_layout(
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
    let mut uuid: [::core::ffi::c_char; 37] = [
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
    ];
    let mut existing: live_saved_layout_t = live_saved_layout_t {
        uuid: [0; 37],
        owner_user_id: 0,
        name: [0; 128],
        is_shared: false,
        location_uuid: [0; 37],
        availability: [0; 24],
        columns: 0,
        rows: 0,
        camera_slots_json: [0; 8192],
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    if read_only_identity(&raw mut user) {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Live layouts require an interactive user\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !extract_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res) {
        return;
    }
    let mut result: db_live_layout_result_t = db_live_layout_get_visible(
        user.id,
        &raw mut uuid as *mut ::core::ffi::c_char,
        &raw mut existing,
    );
    if result.0 != db_live_layout_result_t::DB_LIVE_LAYOUT_OK.0 {
        set_db_error(res, result);
        return;
    }
    if existing.owner_user_id != user.id {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Only the owner can delete this layout\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut revision: int64_t = 0 as int64_t;
    let mut valid: bool = cJSON_IsObject(body) != 0
        && parse_revision(body, &raw mut revision) as ::core::ffi::c_int != 0;
    cJSON_Delete(body);
    if !valid {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"revision is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    result = db_live_layout_delete(user.id, &raw mut uuid as *mut ::core::ffi::c_char, revision);
    if result.0 != db_live_layout_result_t::DB_LIVE_LAYOUT_OK.0 {
        set_db_error(res, result);
        return;
    }
    http_response_set_json(
        res,
        200 as ::core::ffi::c_int,
        b"{\"success\":true}\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
