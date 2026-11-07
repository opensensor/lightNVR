unsafe extern "C" {
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
    unsafe fn cJSON_GetObjectItemCaseSensitive(
        object: *const cJSON,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    unsafe fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
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
    unsafe fn cJSON_AddStringToObject(
        object: *mut cJSON,
        name: *const ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut cJSON;
    unsafe fn db_workspace_preferences_defaults(preferences: *mut workspace_preferences_t);
    unsafe fn db_workspace_preferences_get(
        owner_user_id: int64_t,
        preferences: *mut workspace_preferences_t,
    ) -> ::core::ffi::c_int;
    unsafe fn db_workspace_preferences_replace(
        owner_user_id: int64_t,
        preferences: *const workspace_preferences_t,
    ) -> ::core::ffi::c_int;
    unsafe fn httpd_parse_json_body(req: *const http_request_t) -> *mut cJSON;
    unsafe fn httpd_check_action_access(
        req: *const http_request_t,
        user: *mut user_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
pub struct workspace_preferences_t {
    pub live_navigator_visible: bool,
    pub investigation_visible: bool,
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
pub const WORKSPACE_KEY_LIVE_NAVIGATOR: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"live.navigator\0") };
pub const WORKSPACE_KEY_INVESTIGATION: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"investigation\0") };
unsafe extern "C" fn set_json_response(mut response: *mut http_response_t, mut root: *mut cJSON) {
    let mut body: *mut ::core::ffi::c_char = cJSON_PrintUnformatted(root);
    if body.is_null() {
        http_response_set_json_error(
            response,
            500 as ::core::ffi::c_int,
            b"Failed to serialize workspaces\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    http_response_set_json(response, 200 as ::core::ffi::c_int, body);
    free(body as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn workspace_json(
    mut key: *const ::core::ffi::c_char,
    mut label: *const ::core::ffi::c_char,
    mut description: *const ::core::ffi::c_char,
    mut visible: bool,
    mut configurable: bool,
) -> *mut cJSON {
    let mut workspace: *mut cJSON = cJSON_CreateObject();
    if workspace.is_null() {
        return ::core::ptr::null_mut::<cJSON>();
    }
    cJSON_AddStringToObject(
        workspace,
        b"key\0".as_ptr() as *const ::core::ffi::c_char,
        key,
    );
    cJSON_AddStringToObject(
        workspace,
        b"label\0".as_ptr() as *const ::core::ffi::c_char,
        label,
    );
    cJSON_AddStringToObject(
        workspace,
        b"description\0".as_ptr() as *const ::core::ffi::c_char,
        description,
    );
    cJSON_AddBoolToObject(
        workspace,
        b"visible\0".as_ptr() as *const ::core::ffi::c_char,
        visible as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        workspace,
        b"configurable\0".as_ptr() as *const ::core::ffi::c_char,
        configurable as cJSON_bool,
    );
    cJSON_AddBoolToObject(
        workspace,
        b"authorization_unchanged\0".as_ptr() as *const ::core::ffi::c_char,
        r#true,
    );
    return workspace;
}
unsafe extern "C" fn respond(
    mut user: *const user_t,
    mut preferences: *const workspace_preferences_t,
    mut response: *mut http_response_t,
) {
    let mut root: *mut cJSON = cJSON_CreateObject();
    let mut workspaces: *mut cJSON = cJSON_CreateArray();
    let mut configurable: bool = strcmp(
        &raw const (*user).authentication_method as *const ::core::ffi::c_char,
        b"demo\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
        && !(*user).authenticated_via_scoped_token;
    let mut live: *mut cJSON = workspace_json(
        WORKSPACE_KEY_LIVE_NAVIGATOR.as_ptr(),
        b"Enhanced Live Navigator\0".as_ptr() as *const ::core::ffi::c_char,
        b"Location tree, availability filters, saved layouts, and recent events.\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*preferences).live_navigator_visible,
        configurable,
    );
    let mut investigation: *mut cJSON = workspace_json(
        WORKSPACE_KEY_INVESTIGATION.as_ptr(),
        b"Investigation\0".as_ptr() as *const ::core::ffi::c_char,
        b"Search, correlate, protect, and export recorded evidence.\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*preferences).investigation_visible,
        configurable,
    );
    if root.is_null() || workspaces.is_null() || live.is_null() || investigation.is_null() {
        cJSON_Delete(root);
        cJSON_Delete(workspaces);
        cJSON_Delete(live);
        cJSON_Delete(investigation);
        http_response_set_json_error(
            response,
            500 as ::core::ffi::c_int,
            b"Failed to create workspace response\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    cJSON_AddItemToArray(workspaces, live);
    cJSON_AddItemToArray(workspaces, investigation);
    cJSON_AddItemToObject(
        root,
        b"workspaces\0".as_ptr() as *const ::core::ffi::c_char,
        workspaces,
    );
    cJSON_AddBoolToObject(
        root,
        b"configurable\0".as_ptr() as *const ::core::ffi::c_char,
        configurable as cJSON_bool,
    );
    cJSON_AddStringToObject(
        root,
        b"scope\0".as_ptr() as *const ::core::ffi::c_char,
        if (*user).id > 0 as int64_t {
            b"user\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"installation\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    set_json_response(response, root);
    cJSON_Delete(root);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_get_ui_workspaces(
    mut request: *const http_request_t,
    mut response: *mut http_response_t,
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
    if httpd_check_action_access(request, &raw mut user) == 0 {
        http_response_set_json_error(
            response,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut preferences: workspace_preferences_t = workspace_preferences_t {
        live_navigator_visible: false,
        investigation_visible: false,
    };
    if strcmp(
        &raw mut user.authentication_method as *mut ::core::ffi::c_char,
        b"demo\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        db_workspace_preferences_defaults(&raw mut preferences);
    } else if db_workspace_preferences_get(user.id, &raw mut preferences) != 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            response,
            500 as ::core::ffi::c_int,
            b"Failed to load workspace preferences\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    respond(&raw mut user, &raw mut preferences, response);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_put_ui_workspaces(
    mut request: *const http_request_t,
    mut response: *mut http_response_t,
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
    if httpd_check_action_access(request, &raw mut user) == 0 {
        http_response_set_json_error(
            response,
            401 as ::core::ffi::c_int,
            b"Unauthorized\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if user.authenticated_via_scoped_token as ::core::ffi::c_int != 0
        || strcmp(
            &raw mut user.authentication_method as *mut ::core::ffi::c_char,
            b"demo\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        http_response_set_json_error(
            response,
            403 as ::core::ffi::c_int,
            b"Workspace preferences require an interactive user\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut body: *mut cJSON = httpd_parse_json_body(request);
    let mut workspaces: *const cJSON = if !body.is_null() {
        cJSON_GetObjectItemCaseSensitive(
            body,
            b"workspaces\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    if cJSON_IsObject(workspaces) == 0 {
        cJSON_Delete(body);
        http_response_set_json_error(
            response,
            400 as ::core::ffi::c_int,
            b"workspaces must be an object\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut preferences: workspace_preferences_t = workspace_preferences_t {
        live_navigator_visible: false,
        investigation_visible: false,
    };
    if db_workspace_preferences_get(user.id, &raw mut preferences) != 0 as ::core::ffi::c_int {
        cJSON_Delete(body);
        http_response_set_json_error(
            response,
            500 as ::core::ffi::c_int,
            b"Failed to load workspace preferences\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut changed: bool = r#false != 0;
    let mut item: *const cJSON = ::core::ptr::null::<cJSON>();
    item = if !workspaces.is_null() {
        (*workspaces).child
    } else {
        ::core::ptr::null_mut::<cJSON>()
    };
    while !item.is_null() {
        if (*item).string.is_null() || cJSON_IsBool(item) == 0 {
            cJSON_Delete(body);
            http_response_set_json_error(
                response,
                400 as ::core::ffi::c_int,
                b"Workspace values must be booleans\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        if strcmp((*item).string, WORKSPACE_KEY_LIVE_NAVIGATOR.as_ptr()) == 0 as ::core::ffi::c_int
        {
            preferences.live_navigator_visible = cJSON_IsTrue(item) != 0;
        } else if strcmp((*item).string, WORKSPACE_KEY_INVESTIGATION.as_ptr())
            == 0 as ::core::ffi::c_int
        {
            preferences.investigation_visible = cJSON_IsTrue(item) != 0;
        } else {
            cJSON_Delete(body);
            http_response_set_json_error(
                response,
                400 as ::core::ffi::c_int,
                b"Unknown workspace key\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        changed = r#true != 0;
        item = (*item).next;
    }
    cJSON_Delete(body);
    if !changed {
        http_response_set_json_error(
            response,
            400 as ::core::ffi::c_int,
            b"At least one workspace is required\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if db_workspace_preferences_replace(user.id, &raw mut preferences) != 0 as ::core::ffi::c_int {
        http_response_set_json_error(
            response,
            500 as ::core::ffi::c_int,
            b"Failed to save workspace preferences\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    respond(&raw mut user, &raw mut preferences, response);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
