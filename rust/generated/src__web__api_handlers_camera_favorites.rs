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
    unsafe fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut ::core::ffi::c_char;
    unsafe fn cJSON_Delete(item: *mut cJSON);
    unsafe fn cJSON_CreateObject() -> *mut cJSON;
    unsafe fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
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
    unsafe fn authorization_filter_cameras(
        user: *const user_t,
        action: authorization_action_t,
        cameras: *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_camera_favorites_list(
        user_id: int64_t,
        favorites: *mut camera_favorite_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_camera_favorite_add(
        user_id: int64_t,
        camera_uuid: *const ::core::ffi::c_char,
        favorite: *mut camera_favorite_t,
    ) -> db_camera_favorite_result_t;
    unsafe fn db_camera_favorite_remove(
        user_id: int64_t,
        camera_uuid: *const ::core::ffi::c_char,
        removed: *mut bool,
    ) -> db_camera_favorite_result_t;
    unsafe fn db_fleet_camera_load(
        cameras: *mut *mut fleet_camera_t,
        count: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn lightnvr_uuid_is_valid(value: *const ::core::ffi::c_char) -> bool;
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
pub struct camera_favorite_t {
    pub camera_uuid: [::core::ffi::c_char; 37],
    pub created_at: int64_t,
}
pub type cJSON_bool = ::core::ffi::c_int;
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
pub struct db_camera_favorite_result_t(pub ::core::ffi::c_int);
impl db_camera_favorite_result_t {
    pub const DB_CAMERA_FAVORITE_OK: Self = Self(0);
    pub const DB_CAMERA_FAVORITE_NOT_FOUND: Self = Self(-1);
    pub const DB_CAMERA_FAVORITE_INVALID: Self = Self(-2);
    pub const DB_CAMERA_FAVORITE_ERROR: Self = Self(-3);
}
pub const MAX_STREAMS: ::core::ffi::c_int = LIGHTNVR_MAX_STREAMS;
pub const CAMERA_FAVORITES_MAX_LIST: ::core::ffi::c_int = MAX_STREAMS;
pub const FAVORITES_PATH_PREFIX: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"/api/camera-favorites/\0")
};
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
unsafe extern "C" fn require_interactive_user(
    mut user: *const user_t,
    mut res: *mut http_response_t,
) -> bool {
    if !read_only_identity(user) {
        return r#true != 0;
    }
    http_response_set_json_error(
        res,
        403 as ::core::ffi::c_int,
        b"Camera favorites require an interactive user\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return r#false != 0;
}
unsafe extern "C" fn extract_camera_uuid(
    mut req: *const http_request_t,
    mut res: *mut http_response_t,
    mut uuid: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> bool {
    if http_request_extract_path_param(req, FAVORITES_PATH_PREFIX.as_ptr(), uuid, size)
        != 0 as ::core::ffi::c_int
        || !lightnvr_uuid_is_valid(uuid)
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid camera UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
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
unsafe extern "C" fn favorite_json(mut favorite: *const camera_favorite_t) -> *mut cJSON {
    let mut object: *mut cJSON = cJSON_CreateObject();
    if object.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        object,
        b"camera_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*favorite).camera_uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        object,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*favorite).created_at as ::core::ffi::c_double,
    );
    return object;
}
unsafe extern "C" fn send_json(
    mut res: *mut http_response_t,
    mut status: ::core::ffi::c_int,
    mut root: *mut cJSON,
) {
    let mut encoded: *mut ::core::ffi::c_char = if !root.is_null() {
        cJSON_PrintUnformatted(root)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
    cJSON_Delete(root);
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
unsafe extern "C" fn send_db_error(
    mut res: *mut http_response_t,
    mut result: db_camera_favorite_result_t,
) {
    match result {
        db_camera_favorite_result_t::DB_CAMERA_FAVORITE_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid camera UUID\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_camera_favorite_result_t::DB_CAMERA_FAVORITE_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Camera not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to update camera favorites\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_camera_favorites(
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
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = if !root.is_null() {
        cJSON_AddArrayToObject(root, b"favorites\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if root.is_null() || items.is_null() {
        cJSON_Delete(root);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to create response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut can_modify: bool = !read_only_identity(&raw mut user);
    cJSON_AddBoolToObject(
        root,
        b"can_modify\0".as_ptr() as *const ::core::ffi::c_char,
        can_modify as cJSON_bool,
    );
    if can_modify {
        let mut favorites: *mut camera_favorite_t = calloc(
            CAMERA_FAVORITES_MAX_LIST as size_t,
            ::core::mem::size_of::<camera_favorite_t>(),
        ) as *mut camera_favorite_t;
        let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
        let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut count: ::core::ffi::c_int = if !favorites.is_null() {
            db_camera_favorites_list(user.id, favorites, CAMERA_FAVORITES_MAX_LIST)
        } else {
            -1 as ::core::ffi::c_int
        };
        let mut cameras_ok: bool = count >= 0 as ::core::ffi::c_int
            && load_authorized_cameras(&raw mut user, &raw mut cameras, &raw mut camera_count)
                as ::core::ffi::c_int
                != 0;
        if count < 0 as ::core::ffi::c_int || !cameras_ok {
            free(favorites as *mut ::core::ffi::c_void);
            free(cameras as *mut ::core::ffi::c_void);
            cJSON_Delete(root);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to load camera favorites\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while index < count {
            if camera_allowed(
                &raw mut (*favorites.offset(index as isize)).camera_uuid
                    as *mut ::core::ffi::c_char,
                cameras,
                camera_count,
            ) {
                let mut item: *mut cJSON = favorite_json(favorites.offset(index as isize));
                if !item.is_null() {
                    cJSON_AddItemToArray(items, item);
                }
            }
            index += 1;
        }
        free(favorites as *mut ::core::ffi::c_void);
        free(cameras as *mut ::core::ffi::c_void);
    }
    send_json(res, 200 as ::core::ffi::c_int, root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_camera_favorite(
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
    let mut camera_uuid: [::core::ffi::c_char; 37] = [
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
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    if !require_interactive_user(&raw mut user, res) {
        return;
    }
    if !extract_camera_uuid(
        req,
        res,
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
    ) {
        return;
    }
    let mut cameras: *mut fleet_camera_t = ::core::ptr::null_mut::<fleet_camera_t>();
    let mut camera_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !load_authorized_cameras(&raw mut user, &raw mut cameras, &raw mut camera_count) {
        free(cameras as *mut ::core::ffi::c_void);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to check camera access\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut allowed: bool = camera_allowed(
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        cameras,
        camera_count,
    );
    free(cameras as *mut ::core::ffi::c_void);
    if !allowed {
        http_response_set_json_error(
            res,
            404 as ::core::ffi::c_int,
            b"Camera not found\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut favorite: camera_favorite_t = camera_favorite_t {
        camera_uuid: [0; 37],
        created_at: 0,
    };
    let mut result: db_camera_favorite_result_t = db_camera_favorite_add(
        user.id,
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        &raw mut favorite,
    );
    if result.0 != db_camera_favorite_result_t::DB_CAMERA_FAVORITE_OK.0 {
        send_db_error(res, result);
        return;
    }
    send_json(
        res,
        200 as ::core::ffi::c_int,
        favorite_json(&raw mut favorite),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_camera_favorite(
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
    let mut camera_uuid: [::core::ffi::c_char; 37] = [
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
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    if !require_interactive_user(&raw mut user, res) {
        return;
    }
    if !extract_camera_uuid(
        req,
        res,
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
    ) {
        return;
    }
    let mut removed: bool = r#false != 0;
    let mut result: db_camera_favorite_result_t = db_camera_favorite_remove(
        user.id,
        &raw mut camera_uuid as *mut ::core::ffi::c_char,
        &raw mut removed,
    );
    if result.0 != db_camera_favorite_result_t::DB_CAMERA_FAVORITE_OK.0 {
        send_db_error(res, result);
        return;
    }
    let mut root: *mut cJSON = cJSON_CreateObject();
    if !root.is_null() {
        cJSON_AddBoolToObject(
            root,
            b"success\0".as_ptr() as *const ::core::ffi::c_char,
            r#true,
        );
        cJSON_AddBoolToObject(
            root,
            b"removed\0".as_ptr() as *const ::core::ffi::c_char,
            removed as cJSON_bool,
        );
    }
    send_json(res, 200 as ::core::ffi::c_int, root);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIGHTNVR_MAX_STREAMS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
