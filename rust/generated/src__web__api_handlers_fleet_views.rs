unsafe extern "C" {
    pub type fleet_selector;
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
    unsafe fn cJSON_AddArrayToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    unsafe fn fleet_selector_parse(
        json: *const cJSON,
        error: *mut ::core::ffi::c_char,
        error_size: size_t,
    ) -> *mut fleet_selector_t;
    unsafe fn fleet_selector_free(selector: *mut fleet_selector_t);
    unsafe fn db_fleet_saved_view_list_visible(
        owner_user_id: int64_t,
        views: *mut fleet_saved_view_t,
        max_count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn db_fleet_saved_view_get_visible(
        owner_user_id: int64_t,
        uuid: *const ::core::ffi::c_char,
        view: *mut fleet_saved_view_t,
    ) -> db_fleet_saved_view_result_t;
    unsafe fn db_fleet_saved_view_create(
        view: *mut fleet_saved_view_t,
    ) -> db_fleet_saved_view_result_t;
    unsafe fn db_fleet_saved_view_update(
        view: *mut fleet_saved_view_t,
        expected_revision: int64_t,
    ) -> db_fleet_saved_view_result_t;
    unsafe fn db_fleet_saved_view_delete(
        owner_user_id: int64_t,
        uuid: *const ::core::ffi::c_char,
        expected_revision: int64_t,
    ) -> db_fleet_saved_view_result_t;
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
pub type cJSON_bool = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fleet_saved_view_t {
    pub uuid: [::core::ffi::c_char; 37],
    pub owner_user_id: int64_t,
    pub name: [::core::ffi::c_char; 128],
    pub is_shared: bool,
    pub selector_json: [::core::ffi::c_char; 4096],
    pub search: [::core::ffi::c_char; 256],
    pub collection_uuid: [::core::ffi::c_char; 37],
    pub columns_json: [::core::ffi::c_char; 1024],
    pub sort_by: [::core::ffi::c_char; 32],
    pub sort_order: [::core::ffi::c_char; 8],
    pub revision: int64_t,
    pub created_at: int64_t,
    pub updated_at: int64_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct db_fleet_saved_view_result_t(pub ::core::ffi::c_int);
impl db_fleet_saved_view_result_t {
    pub const DB_FLEET_SAVED_VIEW_OK: Self = Self(0);
    pub const DB_FLEET_SAVED_VIEW_NOT_FOUND: Self = Self(-1);
    pub const DB_FLEET_SAVED_VIEW_INVALID: Self = Self(-2);
    pub const DB_FLEET_SAVED_VIEW_CONFLICT: Self = Self(-3);
    pub const DB_FLEET_SAVED_VIEW_STALE: Self = Self(-4);
    pub const DB_FLEET_SAVED_VIEW_LIMIT: Self = Self(-5);
    pub const DB_FLEET_SAVED_VIEW_ERROR: Self = Self(-6);
}
pub type fleet_selector_t = fleet_selector;
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const CAMERA_UUID_STRING_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const FLEET_SAVED_VIEW_MAX_VISIBLE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
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
unsafe extern "C" fn demo_read_only(mut user: *const user_t) -> bool {
    return !user.is_null()
        && strcmp(
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
unsafe extern "C" fn encode_json(
    mut value: *const cJSON,
    mut destination: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> bool {
    let mut encoded: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(value);
    if encoded.is_null() || strlen(encoded) >= size {
        free(encoded as *mut ::core::ffi::c_void);
        return r#false != 0;
    }
    safe_strcpy(destination, encoded, size, 0 as size_t);
    free(encoded as *mut ::core::ffi::c_void);
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
unsafe extern "C" fn parse_view(
    mut body: *const cJSON,
    mut owner_user_id: int64_t,
    mut view: *mut fleet_saved_view_t,
    mut res: *mut http_response_t,
) -> bool {
    memset(
        view as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<fleet_saved_view_t>(),
    );
    (*view).owner_user_id = owner_user_id;
    safe_strcpy(
        &raw mut (*view).sort_by as *mut ::core::ffi::c_char,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*view).sort_order as *mut ::core::ffi::c_char,
        b"asc\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
        0 as size_t,
    );
    safe_strcpy(
        &raw mut (*view).columns_json as *mut ::core::ffi::c_char,
        b"[\"camera\",\"health\",\"location\",\"tags\",\"recording\",\"actions\"]\0".as_ptr()
            as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        0 as size_t,
    );
    if cJSON_IsObject(body) == 0
        || !copy_string(
            body,
            b"name\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*view).name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
            r#true != 0,
        )
        || !copy_string(
            body,
            b"search\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*view).search as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
            r#false != 0,
        )
        || !copy_string(
            body,
            b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*view).collection_uuid as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
            r#false != 0,
        )
        || (*view).collection_uuid[0usize] as ::core::ffi::c_int != 0
            && !lightnvr_uuid_is_valid(&raw mut (*view).collection_uuid as *mut ::core::ffi::c_char)
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid saved view fields\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
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
    (*view).is_shared = !shared.is_null() && cJSON_IsTrue(shared) != 0;
    let mut selector: *const cJSON = cJSON_GetObjectItemCaseSensitive(
        body,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut selector_error: [::core::ffi::c_char; 256] = [
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
    let mut parsed: *mut fleet_selector_t = if cJSON_IsObject(selector) != 0 {
        fleet_selector_parse(
            selector,
            &raw mut selector_error as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        )
    } else {
        ::core::ptr::null_mut::<fleet_selector_t>()
    };
    if parsed.is_null()
        || !encode_json(
            selector,
            &raw mut (*view).selector_json as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        )
    {
        fleet_selector_free(parsed);
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            if selector_error[0usize] as ::core::ffi::c_int != 0 {
                &raw mut selector_error as *mut ::core::ffi::c_char
            } else {
                b"Invalid selector\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return r#false != 0;
    }
    fleet_selector_free(parsed);
    let mut columns: *const cJSON =
        cJSON_GetObjectItemCaseSensitive(body, b"columns\0".as_ptr() as *const ::core::ffi::c_char);
    if !columns.is_null()
        && (cJSON_IsArray(columns) == 0
            || !encode_json(
                columns,
                &raw mut (*view).columns_json as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            ))
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid columns\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !cJSON_GetObjectItemCaseSensitive(body, b"sort_by\0".as_ptr() as *const ::core::ffi::c_char)
        .is_null()
        && !copy_string(
            body,
            b"sort_by\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*view).sort_by as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            r#true != 0,
        )
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid sort_by\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    if !cJSON_GetObjectItemCaseSensitive(
        body,
        b"sort_order\0".as_ptr() as *const ::core::ffi::c_char,
    )
    .is_null()
        && !copy_string(
            body,
            b"sort_order\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*view).sort_order as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 8]>(),
            r#true != 0,
        )
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid sort_order\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
unsafe extern "C" fn view_json(
    mut view: *const fleet_saved_view_t,
    mut current_user_id: int64_t,
) -> *mut cJSON {
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut selector: *mut cJSON =
        cJSON_Parse(&raw const (*view).selector_json as *const ::core::ffi::c_char);
    let mut columns: *mut cJSON =
        cJSON_Parse(&raw const (*view).columns_json as *const ::core::ffi::c_char);
    if root.is_null() || cJSON_IsObject(selector) == 0 || cJSON_IsArray(columns) == 0 {
        cJSON_Delete(root);
        cJSON_Delete(selector);
        cJSON_Delete(columns);
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        root,
        b"uuid\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*view).uuid as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"name\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*view).name as *const ::core::ffi::c_char,
    );
    cJSON_AddBoolToObject(
        root,
        b"is_shared\0".as_ptr() as *const ::core::ffi::c_char,
        (*view).is_shared as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        root,
        b"owned\0".as_ptr() as *const ::core::ffi::c_char,
        ((*view).owner_user_id == current_user_id) as ::core::ffi::c_int,
    );
    cJSON_AddItemToObject(
        root,
        b"selector\0".as_ptr() as *const ::core::ffi::c_char,
        selector,
    );
    cJSON_AddStringToObject(
        root,
        b"search\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*view).search as *const ::core::ffi::c_char,
    );
    if (*view).collection_uuid[0usize] != 0 {
        cJSON_AddStringToObject(
            root,
            b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const (*view).collection_uuid as *const ::core::ffi::c_char,
        );
    } else {
        cJSON_AddNullToObject(
            root,
            b"collection_uuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    cJSON_AddItemToObject(
        root,
        b"columns\0".as_ptr() as *const ::core::ffi::c_char,
        columns,
    );
    cJSON_AddStringToObject(
        root,
        b"sort_by\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*view).sort_by as *const ::core::ffi::c_char,
    );
    cJSON_AddStringToObject(
        root,
        b"sort_order\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const (*view).sort_order as *const ::core::ffi::c_char,
    );
    cJSON_AddNumberToObject(
        root,
        b"revision\0".as_ptr() as *const ::core::ffi::c_char,
        (*view).revision as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"created_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*view).created_at as ::core::ffi::c_double,
    );
    cJSON_AddNumberToObject(
        root,
        b"updated_at\0".as_ptr() as *const ::core::ffi::c_char,
        (*view).updated_at as ::core::ffi::c_double,
    );
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
    mut result: db_fleet_saved_view_result_t,
) {
    match result {
        db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_NOT_FOUND => {
            http_response_set_json_error(
                res,
                404 as ::core::ffi::c_int,
                b"Saved view not found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_INVALID => {
            http_response_set_json_error(
                res,
                400 as ::core::ffi::c_int,
                b"Invalid saved view\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_CONFLICT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"A view with this name already exists\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_STALE => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Saved view changed; reload and retry\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_LIMIT => {
            http_response_set_json_error(
                res,
                409 as ::core::ffi::c_int,
                b"Saved view limit reached\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _ => {
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Saved view operation failed\0".as_ptr() as *const ::core::ffi::c_char,
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
        b"/api/fleet/views/\0".as_ptr() as *const ::core::ffi::c_char,
        uuid,
        CAMERA_UUID_STRING_SIZE as size_t,
    ) != 0 as ::core::ffi::c_int
        || !lightnvr_uuid_is_valid(uuid)
    {
        http_response_set_json_error(
            res,
            400 as ::core::ffi::c_int,
            b"Invalid saved view UUID\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return r#false != 0;
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_fleet_saved_views(
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
    let mut views: *mut fleet_saved_view_t = calloc(
        FLEET_SAVED_VIEW_MAX_VISIBLE as size_t,
        ::core::mem::size_of::<fleet_saved_view_t>(),
    ) as *mut fleet_saved_view_t;
    if views.is_null() {
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut count: ::core::ffi::c_int =
        db_fleet_saved_view_list_visible(user.id, views, FLEET_SAVED_VIEW_MAX_VISIBLE);
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut items: *mut cJSON = if !root.is_null() {
        cJSON_AddArrayToObject(root, b"views\0".as_ptr() as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if count < 0 as ::core::ffi::c_int || root.is_null() || items.is_null() {
        free(views as *mut ::core::ffi::c_void);
        cJSON_Delete(root);
        http_response_set_json_error(
            res,
            500 as ::core::ffi::c_int,
            b"Failed to list saved views\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while index < count {
        let mut item: *mut cJSON = view_json(views.offset(index as isize), user.id);
        if item.is_null() {
            free(views as *mut ::core::ffi::c_void);
            cJSON_Delete(root);
            http_response_set_json_error(
                res,
                500 as ::core::ffi::c_int,
                b"Failed to create saved view response\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        cJSON_AddItemToArray(items, item);
        index += 1;
    }
    free(views as *mut ::core::ffi::c_void);
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
    send_json(res, 200 as ::core::ffi::c_int, root);
    cJSON_Delete(root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_post_fleet_saved_view(
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
    if demo_read_only(&raw mut user) {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Demo mode is read-only\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut view: fleet_saved_view_t = fleet_saved_view_t {
        uuid: [0; 37],
        owner_user_id: 0,
        name: [0; 128],
        is_shared: false,
        selector_json: [0; 4096],
        search: [0; 256],
        collection_uuid: [0; 37],
        columns_json: [0; 1024],
        sort_by: [0; 32],
        sort_order: [0; 8],
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut valid: bool = parse_view(body, user.id, &raw mut view, res);
    cJSON_Delete(body);
    if !valid {
        return;
    }
    if view.is_shared as ::core::ffi::c_int != 0 && user.role.0 != user_role_t::USER_ROLE_ADMIN.0 {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Only administrators can share views\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut result: db_fleet_saved_view_result_t = db_fleet_saved_view_create(&raw mut view);
    if result.0 != db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_OK.0 {
        set_db_error(res, result);
        return;
    }
    let mut root: *mut cJSON = view_json(&raw mut view, user.id);
    send_json(res, 201 as ::core::ffi::c_int, root);
    cJSON_Delete(root);
}
unsafe extern "C" fn load_visible_view(
    mut user: *const user_t,
    mut uuid: *const ::core::ffi::c_char,
    mut view: *mut fleet_saved_view_t,
    mut res: *mut http_response_t,
) -> bool {
    let mut result: db_fleet_saved_view_result_t =
        db_fleet_saved_view_get_visible((*user).id, uuid, view);
    if result.0 != db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_OK.0 {
        set_db_error(res, result);
        return r#false != 0;
    }
    return r#true != 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_fleet_saved_view(
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
    let mut view: fleet_saved_view_t = fleet_saved_view_t {
        uuid: [0; 37],
        owner_user_id: 0,
        name: [0; 128],
        is_shared: false,
        selector_json: [0; 4096],
        search: [0; 256],
        collection_uuid: [0; 37],
        columns_json: [0; 1024],
        sort_by: [0; 32],
        sort_order: [0; 8],
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    if !authenticate(req, res, &raw mut user)
        || !extract_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res)
        || !load_visible_view(
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            &raw mut view,
            res,
        )
    {
        return;
    }
    let mut root: *mut cJSON = view_json(&raw mut view, user.id);
    send_json(res, 200 as ::core::ffi::c_int, root);
    cJSON_Delete(root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_fleet_saved_view(
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
    let mut existing: fleet_saved_view_t = fleet_saved_view_t {
        uuid: [0; 37],
        owner_user_id: 0,
        name: [0; 128],
        is_shared: false,
        selector_json: [0; 4096],
        search: [0; 256],
        collection_uuid: [0; 37],
        columns_json: [0; 1024],
        sort_by: [0; 32],
        sort_order: [0; 8],
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    if demo_read_only(&raw mut user) {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Demo mode is read-only\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !extract_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res)
        || !load_visible_view(
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            &raw mut existing,
            res,
        )
    {
        return;
    }
    if existing.owner_user_id != user.id {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Only the owner can edit this view\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(req);
    let mut view: fleet_saved_view_t = fleet_saved_view_t {
        uuid: [0; 37],
        owner_user_id: 0,
        name: [0; 128],
        is_shared: false,
        selector_json: [0; 4096],
        search: [0; 256],
        collection_uuid: [0; 37],
        columns_json: [0; 1024],
        sort_by: [0; 32],
        sort_order: [0; 8],
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    let mut revision: int64_t = 0 as int64_t;
    let mut valid: bool = parse_view(body, user.id, &raw mut view, res) as ::core::ffi::c_int != 0
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
    if view.is_shared as ::core::ffi::c_int != 0 && user.role.0 != user_role_t::USER_ROLE_ADMIN.0 {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Only administrators can share views\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_strcpy(
        &raw mut view.uuid as *mut ::core::ffi::c_char,
        &raw mut uuid as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 37]>(),
        0 as size_t,
    );
    let mut result: db_fleet_saved_view_result_t =
        db_fleet_saved_view_update(&raw mut view, revision);
    if result.0 != db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_OK.0 {
        set_db_error(res, result);
        return;
    }
    let mut root: *mut cJSON = view_json(&raw mut view, user.id);
    send_json(res, 200 as ::core::ffi::c_int, root);
    cJSON_Delete(root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_delete_fleet_saved_view(
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
    let mut view: fleet_saved_view_t = fleet_saved_view_t {
        uuid: [0; 37],
        owner_user_id: 0,
        name: [0; 128],
        is_shared: false,
        selector_json: [0; 4096],
        search: [0; 256],
        collection_uuid: [0; 37],
        columns_json: [0; 1024],
        sort_by: [0; 32],
        sort_order: [0; 8],
        revision: 0,
        created_at: 0,
        updated_at: 0,
    };
    if !authenticate(req, res, &raw mut user) {
        return;
    }
    if demo_read_only(&raw mut user) {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Demo mode is read-only\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !extract_uuid(req, &raw mut uuid as *mut ::core::ffi::c_char, res)
        || !load_visible_view(
            &raw mut user,
            &raw mut uuid as *mut ::core::ffi::c_char,
            &raw mut view,
            res,
        )
    {
        return;
    }
    if view.owner_user_id != user.id {
        http_response_set_json_error(
            res,
            403 as ::core::ffi::c_int,
            b"Only the owner can delete this view\0".as_ptr() as *const ::core::ffi::c_char,
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
    let mut result: db_fleet_saved_view_result_t =
        db_fleet_saved_view_delete(user.id, &raw mut uuid as *mut ::core::ffi::c_char, revision);
    if result.0 != db_fleet_saved_view_result_t::DB_FLEET_SAVED_VIEW_OK.0 {
        set_db_error(res, result);
        return;
    }
    http_response_set_json(
        res,
        200 as ::core::ffi::c_int,
        b"{\"success\":true}\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
